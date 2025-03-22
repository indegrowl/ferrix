use calcis::prelude::*;

#[derive(Default)]
pub struct Transform2D {
    pub translation: Vec2<f32>,
    pub rotation: f32,
}

impl Transform2D {
    pub fn new(translation: Vec2<f32>, rotation: f32) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn identity() -> Self {
        Self {
            translation: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        }
    }
}
