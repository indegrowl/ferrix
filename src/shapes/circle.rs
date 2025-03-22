use crate::{
    bounding_box::{AABB, BoundingBox},
    transform2d::Transform2D,
};
use calcis::prelude::*;

pub struct Circle {
    pub radius: f32,
    pub transform: Transform2D,
}

impl Circle {
    pub fn new(radius: f32, transform: Transform2D) -> Self {
        Self { radius, transform }
    }

    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    pub fn circumference(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }
}

impl AABB for Circle {
    fn bounding_box(&self) -> crate::bounding_box::BoundingBox {
        let min_x = self.transform.translation.x - self.radius;
        let min_y = self.transform.translation.y - self.radius;

        let max_x = self.transform.translation.x + self.radius;
        let max_y = self.transform.translation.y + self.radius;

        BoundingBox {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounding_box::{AABB, BoundingBox};

    #[test]
    fn test_area() {
        // Assume Transform2D::default() is available.
        let circle = Circle::new(3.0, Transform2D::default());
        let expected_area = std::f32::consts::PI * 3.0 * 3.0;
        let actual_area = circle.area();
        assert!(
            (expected_area - actual_area).abs() < 1e-5,
            "Area calculation is incorrect"
        );
    }

    #[test]
    fn test_circumference() {
        let circle = Circle::new(3.0, Transform2D::identity());
        let expected_circumference = 2.0 * std::f32::consts::PI * 3.0;
        let actual_circumference = circle.circumference();
        assert!(
            (expected_circumference - actual_circumference).abs() < 1e-5,
            "Circumference calculation is incorrect"
        );
    }

    #[test]
    fn test_bounding_box() {
        // Create a transform with a specified translation.
        let translation = Vec2::new(3.0, 4.0);
        // Assuming Transform2D implements Default for other fields.
        let transform = Transform2D {
            translation,
            rotation: 0.0,
        };
        let circle = Circle::new(5.0, transform);
        let bbox = circle.bounding_box();
        let expected_bbox = BoundingBox {
            min: Vec2::new(3.0 - 5.0, 4.0 - 5.0),
            max: Vec2::new(3.0 + 5.0, 4.0 + 5.0),
        };
        assert_eq!(
            bbox.min, expected_bbox.min,
            "Bounding box min point is incorrect"
        );
        assert_eq!(
            bbox.max, expected_bbox.max,
            "Bounding box max point is incorrect"
        );
    }
}
