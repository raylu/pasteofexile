use crate::serde::utils::u8_or_nil;
use serde::de;
use serde::Deserialize;
use serde_with::{rust::StringWithSeparator, CommaSeparator};

#[derive(Debug, Deserialize)]
pub(crate) struct PathOfBuilding {
    #[serde(rename = "Build")]
    pub build: Build,

    #[serde(rename = "Skills")]
    pub skills: Skills,

    #[serde(rename = "Tree")]
    pub tree: Tree,

    #[serde(default, rename = "Items")]
    pub items: Items,

    #[serde(default, rename = "Notes")]
    pub notes: String,

    #[serde(default, rename = "Config")]
    pub config: Config,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Build {
    pub level: u8,
    pub class_name: String,
    pub ascend_class_name: String,
    #[serde(default, rename = "PlayerStat")]
    pub player_stats: Vec<BuildStat>,
    #[serde(default, rename = "MinionStat")]
    pub minion_stats: Vec<BuildStat>,
    pub main_socket_group: u8,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BuildStat {
    #[serde(rename = "stat")]
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Skills {
    #[serde(default, rename = "$value")]
    pub skills: Vec<Skill>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Skill {
    #[serde(default, deserialize_with = "u8_or_nil")]
    pub main_active_skill: u8,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub slot: Option<String>,
    #[serde(default, rename = "$value")]
    pub gems: Vec<Gem>,
}

impl Skill {
    pub fn active_gems(&self) -> impl Iterator<Item = &Gem> {
        self.gems.iter().filter(|gem| gem.is_active())
    }

    pub fn support_gems(&self) -> impl Iterator<Item = &Gem> {
        self.gems.iter().filter(|gem| gem.is_support())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Gem {
    #[serde(rename = "nameSpec")]
    pub name: String,
    pub skill_id: Option<String>,
    pub gem_id: Option<String>,
    #[serde(default)]
    pub level: u8,
    #[serde(default)]
    pub quality: u8,
}

impl Gem {
    pub fn is_support(&self) -> bool {
        if let Some(gem_id) = &self.gem_id {
            return gem_id.starts_with("Metadata/Items/Gems/Support");
        }
        if let Some(skill_id) = &self.skill_id {
            return skill_id.starts_with("Support");
        }
        self.name.contains("Support")
    }

    pub fn is_active(&self) -> bool {
        !self.is_support()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Tree {
    pub active_spec: u8,
    #[serde(rename = "Spec")]
    pub specs: Vec<Spec>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Spec {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, with = "StringWithSeparator::<CommaSeparator>")]
    pub nodes: Vec<u32>,
    #[serde(default, rename = "URL")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Items {
    #[serde(default, rename = "Item")]
    pub items: Vec<Item>,
    #[serde(default, rename = "Slot")]
    pub slots: Vec<Slot>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Item {
    pub id: u16,
    // this might be parsable with serde_as into a `(String, Vec<()>)`
    #[serde(rename = "$value")]
    pub content: ItemContent,
}

#[derive(Default, Debug)]
pub(crate) struct ItemContent {
    pub content: String,
}

impl<'de> de::Deserialize<'de> for ItemContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ItemContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expected pob item")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                // first element is the item content
                let content = seq.next_element::<String>()?.unwrap_or_else(String::new);
                // following elements are mod ranges, ignore them for now
                while seq.next_element::<()>()?.is_some() {}

                Ok(ItemContent { content })
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Slot {
    pub item_id: u16,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    #[serde(default, rename = "Input")]
    pub input: Vec<Input>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Input {
    pub name: String,
    pub string: Option<String>,
    pub boolean: Option<bool>,
    pub number: Option<f32>,
}
