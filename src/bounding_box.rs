use calcis::prelude::*;

pub struct BoundingBox {
    pub min: Vec2<f32>,
    pub max: Vec2<f32>,
}

pub trait AABB {
    fn bounding_box(&self) -> BoundingBox;
}
