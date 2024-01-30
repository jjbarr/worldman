use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;

pub type Notes = Vec<String>;

pub type Attributes = BTreeMap<String, String>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct World {
    #[serde(default)]
    pub npcs: Vec<Character>,
    #[serde(default)]
    pub pcs: Vec<Character>,
    #[serde(default)]
    pub places: Vec<Place>,
    #[serde(default)]
    pub notes: Notes,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Character {
    pub name: String,
    pub player: Option<String>,
    pub race: Option<String>,
    pub level: Option<i32>,
    pub class: Option<String>,
    pub stats: Option<BTreeMap<String, String>>,
    pub skills: Option<Vec<String>>,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub attributes: Attributes,
    #[serde(default)]
    pub inventory: Vec<String>,
    #[serde(default)]
    pub notes: Notes,
}

impl Character {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Place {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub objects: Vec<String>,
    #[serde(default)]
    pub attributes: Attributes,
    #[serde(default)]
    pub notes: Notes,
}

impl Place {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}
