use crate::{
    consts, crypto, storage,
    utils::{self, ResponseExt},
    Error, Result,
};
use pob::SerdePathOfBuilding;
use serde::Serialize;
use worker::{Env, Headers, Method, Request, Response};

#[derive(Serialize)]
struct Upload {
    id: String,
}

pub async fn try_handle(req: &mut Request, env: &Env) -> Result<Option<Response>> {
    // TODO: use a sycamore router for this?
    if req.path() == "/api/v1/paste/" && req.method() == Method::Post {
        return handle_upload(req, env, true).await.map(Some);
    }
    if req.path() == "/pob/" && req.method() == Method::Post {
        return handle_upload(req, env, false).await.map(Some);
    }
    if let Some(id) = is_download_url(&req.path(), req) {
        return handle_download(env, id).await.map(Some);
    }

    Ok(None)
}

fn is_download_url<'a>(path: &'a str, req: &Request) -> Option<&'a str> {
    if req.method() != Method::Get {
        return None;
    }

    is_pob_download_url(path).or_else(|| is_raw_download_url(path))
}

fn is_raw_download_url(path: &str) -> Option<&str> {
    path.trim_start_matches('/')
        .split_once('/')
        .filter(|(_, raw)| *raw == "raw")
        .map(|(id, _)| id)
}

fn is_pob_download_url(path: &str) -> Option<&str> {
    path.rsplit_once('/')
        .filter(|(path, _)| *path == "/pob")
        .map(|(_, id)| id)
}

async fn handle_download(env: &Env, id: &str) -> Result<Response> {
    let path = utils::to_path(id)?;

    let response = storage::get(env, &path)
        .await?
        .ok_or_else(|| Error::NotFound("paste", id.to_owned()))?;

    response
        .with_headers(Headers::new())
        .with_content_type("text/plain")?
        .cache_for(31536000)
}

async fn handle_upload(req: &mut Request, env: &Env, return_json: bool) -> Result<Response> {
    let mut data = req.bytes().await?;

    if data.len() > consts::MAX_UPLOAD_SIZE {
        return Err(Error::BadRequest("Paste too large".to_owned()));
    }

    let s = std::str::from_utf8(&data)
        .map_err(|_| "invalid content".to_owned())
        .map_err(Error::BadRequest)?;

    // Generic 401, probably just actually bad data
    let s = pob::decompress(s).map_err(|e| Error::BadRequest(e.to_string()))?;
    // More specific error for a separate Sentry categoy
    let _ =
        SerdePathOfBuilding::from_xml(&s).map_err(move |e| Error::InvalidPoB(e.to_string(), s))?;

    let sha1 = crypto::sha1(&mut data).await?;
    let id = utils::hash_to_short_id(&sha1, 9)?;
    let filename = utils::to_path(&id)?;

    log::debug!("--> uploading paste '{}' to '{}'", id, filename);
    storage::put(env, &filename, &sha1, &mut data).await?;
    log::debug!("<-- paste uploaded");

    if !return_json {
        // for pob API
        return Ok(Response::ok(id)?);
    }

    // Weird garbage data bug, but this seems to make it better
    let response = serde_json::to_string(&Upload { id })?;
    let mut response = Response::from_bytes(response.into_bytes())?;
    response
        .headers_mut()
        .set("Content-Type", "application/json")?;
    Ok(response)
}
