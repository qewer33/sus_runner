use std::collections::HashMap;

use macroquad::prelude::*;

#[derive(Clone)]
pub struct AssetManager {
    pub textures: HashMap<String, Texture2D>,
    pub fonts: HashMap<String, Font>,
}
impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            textures: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    pub async fn add_texture(
        &mut self,
        texture_name: &str,
        texture_path: &str,
    ) -> Option<&mut Self> {
        match load_texture(texture_path).await {
            Ok(texture) => self.textures.insert(texture_name.into(), texture),
            Err(_) => None,
        };

        Some(self)
    }

    pub fn get_texture(&self, texture_name: &str) -> Option<&Texture2D> {
        self.textures.get(texture_name.into())
    }

    pub async fn add_font(&mut self, font_name: &str, font_path: &str) -> Option<&mut Self> {
        match load_ttf_font(font_path).await {
            Ok(font) => self.fonts.insert(font_name.into(), font),
            Err(_) => None,
        };

        Some(self)
    }

    pub fn get_font(&self, font_name: &str) -> Option<&Font> {
        self.fonts.get(font_name.into())
    }
}
