use crate::pob;
use ::pob::SerdePathOfBuilding;
use std::borrow::Cow;

const TITLE_INDEX: &str = "POB B.in - Share your Path of Exile build";
const TITLE_NOT_FOUND: &str = "POB B.in - Not Found";
const TITLE_SERVER_ERROR: &str = "POB B.in - Server Error";
const DESCRIPTION: &str = "POB B.in is a website to share your Path of Building builds online";
const DEFAULT_COLOR: &str = "#0ea5e9";

#[derive(Debug)]
pub struct Meta {
    pub(crate) title: Cow<'static, str>,
    pub(crate) description: Cow<'static, str>,
    pub(crate) image: Cow<'static, str>,
    pub(crate) color: &'static str,
}

impl Meta {
    pub(crate) fn index() -> Self {
        Self {
            title: TITLE_INDEX.into(),
            description: DESCRIPTION.into(),
            image: "".into(),
            color: DEFAULT_COLOR,
        }
    }

    pub(crate) fn not_found() -> Self {
        Self {
            title: TITLE_NOT_FOUND.into(),
            description: DESCRIPTION.into(),
            image: "".into(),
            color: DEFAULT_COLOR,
        }
    }

    pub(crate) fn server_error() -> Self {
        Self {
            title: TITLE_SERVER_ERROR.into(),
            description: DESCRIPTION.into(),
            image: "".into(),
            color: DEFAULT_COLOR,
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self::index()
    }
}

pub(crate) fn get_paste_summary(pob: &SerdePathOfBuilding) -> Vec<String> {
    let core_stats = pob::summary::core_stats(pob);
    let defense = pob::summary::defense(pob);
    let offense = pob::summary::offense(pob);
    let config = pob::summary::config(pob);

    vec![core_stats, defense, offense, config]
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter_map(|stat| stat.render_to_string())
                .collect::<Vec<_>>()
        })
        .map(|line| line.join("\u{318d}"))
        .map(|line| format!("\u{27A4} {}", line))
        .collect()
}

pub(crate) fn get_color(ascendancy_name: &str) -> &'static str {
    match ascendancy_name {
        "Slayer" => "#96afc8",
        "Gladiator" => "#96afc8",
        "Champion" => "#96afc8",
        "Juggernaut" => "#af5a32",
        "Berserker" => "#af5a32",
        "Chieftain" => "#af5a32",
        "Raider" => "#7cb376",
        "Deadeye" => "#7cb376",
        "Pathfinder" => "#7cb376",
        "Assassin" => "#72818d",
        "Trickster" => "#72818d",
        "Saboteur" => "#72818d",
        "Inquisitor" => "#cfbd8a",
        "Hierophant" => "#cfbd8a",
        "Guardian" => "#cfbd8a",
        "Occultist" => "#9ac3c9",
        "Elementalist" => "#9ac3c9",
        "Necromancer" => "#9ac3c9",
        "Ascendant" => "#cccccc",
        _ => DEFAULT_COLOR,
    }
}
