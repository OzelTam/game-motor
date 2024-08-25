
use {
    sfml::{
        audio::{Sound, SoundBuffer},
        graphics::{Color, RcTexture, RenderTarget, RenderWindow, Sprite, Texture},
        window::{Event, Key, Style},
        SfBox, SfResource,
    },
    std::{collections::HashMap, hash::Hash},
};

#[derive(Debug)]
pub struct ResourceHolder<Resource: SfResource, Identifier: Hash + Eq> {
    resource_map: HashMap<Identifier, SfBox<Resource>>,
}

impl<Resource: SfResource + ResLoad, Identifier: Hash + Eq> ResourceHolder<Resource, Identifier> {
    pub fn load(&mut self, identifier: Identifier, filename: &str) {
        let res = Resource::load(filename);
        self.resource_map.insert(identifier, res);
    }
    pub fn get(&self, id: Identifier) -> &Resource {
        &self.resource_map[&id]
    }
}

trait ResLoad: SfResource {
    fn load(filename: &str) -> SfBox<Self>;
}

impl ResLoad for Texture {
    fn load(filename: &str) -> SfBox<Self> {
        Self::from_file(filename).unwrap()
    }
}

impl ResLoad for SoundBuffer {
    fn load(filename: &str) -> SfBox<Self> {
        Self::from_file(filename).unwrap()
    }
}




impl<Resource: SfResource, Identifier: Hash + Eq> Default for ResourceHolder<Resource, Identifier> {
    fn default() -> Self {
        Self {
            resource_map: HashMap::default(),
        }
    }
}
