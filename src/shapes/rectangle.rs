use crate::{
    bounding_box::{AABB, BoundingBox},
    transform2d::Transform2D,
};
use calcis::prelude::*;

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub transform: Transform2D,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, transform: Transform2D) -> Self {
        Self {
            width,
            height,
            transform,
        }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}

impl AABB for Rectangle {
    fn bounding_box(&self) -> crate::bounding_box::BoundingBox {
        let top = self.transform.translation.y + self.height / 2.0;
        let bottom = self.transform.translation.y - self.height / 2.0;

        let left = self.transform.translation.x + self.width / 2.0;
        let right = self.transform.translation.x - self.width / 2.0;

        let top_left = Vec2::new(left, top)
            .rotate_around(&self.transform.translation, self.transform.rotation);
        let top_right = Vec2::new(right, top)
            .rotate_around(&self.transform.translation, self.transform.rotation);
        let bottom_left = Vec2::new(left, bottom)
            .rotate_around(&self.transform.translation, self.transform.rotation);
        let bottom_right = Vec2::new(right, bottom)
            .rotate_around(&self.transform.translation, self.transform.rotation);

        let max_x = [top_left.x, top_right.x, bottom_left.x, bottom_right.x]
            .into_iter()
            .fold(f32::MIN, f32::max);
        let min_x = [top_left.x, top_right.x, bottom_left.x, bottom_right.x]
            .into_iter()
            .fold(f32::MAX, f32::min);

        let max_y = [top_left.y, top_right.y, bottom_left.y, bottom_right.y]
            .into_iter()
            .fold(f32::MIN, f32::max);
        let min_y = [top_left.y, top_right.y, bottom_left.y, bottom_right.y]
            .into_iter()
            .fold(f32::MAX, f32::min);

        BoundingBox {
            max: Vec2::new(max_x, max_y),
            min: Vec2::new(min_x, min_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a Transform2D with given translation and rotation.
    fn create_transform(x: f32, y: f32, rotation: f32) -> Transform2D {
        Transform2D {
            translation: Vec2::new(x, y),
            rotation,
        }
    }

    #[test]
    fn test_area() {
        let transform = create_transform(0.0, 0.0, 0.0);
        let rectangle = Rectangle::new(4.0, 2.0, transform);
        assert_eq!(rectangle.area(), 8.0);
    }

    #[test]
    fn test_perimeter() {
        let transform = create_transform(0.0, 0.0, 0.0);
        let rectangle = Rectangle::new(4.0, 2.0, transform);
        assert_eq!(rectangle.perimeter(), 12.0);
    }

    #[test]
    fn test_bounding_box_no_rotation() {
        // With no rotation, the bounding box should be a rectangle aligned with the axes.
        let transform = create_transform(0.0, 0.0, 0.0);
        let rectangle = Rectangle::new(4.0, 2.0, transform);
        let bbox = rectangle.bounding_box();

        // Given the implementation:
        // left  = translation.x + width/2 = 0 + 2 = 2
        // right = translation.x - width/2 = 0 - 2 = -2
        // top   = translation.y + height/2 = 0 + 1 = 1
        // bottom= translation.y - height/2 = 0 - 1 = -1
        // After rotation of 0 the points remain unchanged.
        // Thus, bbox.min should be (-2, -1) and bbox.max should be (2, 1).
        assert_eq!(bbox.min, Vec2::new(-2.0, -1.0));
        assert_eq!(bbox.max, Vec2::new(2.0, 1.0));
    }

    #[test]
    fn test_bounding_box_with_rotation() {
        // Test with a non-zero rotation.
        // For simplicity, use 90 degrees counter-clockwise rotation (pi/2 radians),
        // so that coordinates swap: (x, y) -> (-y, x)
        let transform = create_transform(0.0, 0.0, std::f32::consts::FRAC_PI_2);
        let rectangle = Rectangle::new(4.0, 2.0, transform);
        let bbox = rectangle.bounding_box();

        // Calculate expected corner points manually.
        // Without rotation, points are:
        // top_left: (2, 1) -> after rotation: (-1, 2)
        // top_right: (-2, 1) -> after rotation: (-1, -2)
        // bottom_left: (2, -1) -> after rotation: (1, 2)
        // bottom_right: (-2, -1) -> after rotation: (1, -2)
        // Bounding box then is min = (-1, -2), max = (1, 2)
        assert_eq!(bbox.min, Vec2::new(-1.0, -2.0));
        assert_eq!(bbox.max, Vec2::new(1.0, 2.0));
    }
}
