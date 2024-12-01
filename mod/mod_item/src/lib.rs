use std::{fs::File, ops::Deref, sync::Arc};

use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

pub struct ModItemPlugin;

#[derive(Resource)]
pub struct ItemRegistry(pub HashMap<ItemType, Arc<Item>>);

impl Plugin for ModItemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_items());
    }
}

fn load_items() -> ItemRegistry {
    let items: Items =
        serde_xml_rs::from_reader(File::open("./assets/data/items.xml").unwrap()).unwrap();
    debug!("loaded items: {:?}", items);

    ItemRegistry(
        items
            .items
            .into_iter()
            .map(|item| (item.id, Arc::new(item)))
            .collect(),
    )
}

impl Deref for ItemRegistry {
    type Target = HashMap<ItemType, Arc<Item>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Shovel,
    Almanac,
    Key,
    Taco,
    WateringCan,
}

#[derive(Debug, Deserialize)]
struct Items {
    #[serde(rename = "Item")]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: ItemType,
    #[serde(rename = "Texture")]
    pub texture: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
}
