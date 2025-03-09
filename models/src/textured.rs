use bevy::prelude::Component;

#[derive(Component)]
pub struct Textured {
    pub file: String,
}

impl Default for Textured {
    fn default() -> Self {
        Textured {
            file: "missing_texture.png".to_string(),
        }
    }
}
