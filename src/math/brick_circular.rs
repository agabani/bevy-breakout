use std::f32::consts::PI;

use bevy::prelude::*;

use crate::physics::MIN_Z;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BrickCircular {
    pub(crate) columns: u32,
    pub(crate) rows: u32,

    pub(crate) radius_max: f32,
    pub(crate) radius_min: f32,

    pub(crate) offset_x: f32,
    pub(crate) offset_y: f32,

    pub(crate) padding_x: f32,
    pub(crate) padding_y: f32,
}

impl BrickCircular {
    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn transform(self, column: u32, row: u32) -> Transform {
        let total_height = self.radius_max - self.radius_min;
        let height = total_height / self.rows as f32;

        let radius = self.radius_min + (row as f32) * height;

        let total_width = 2.0 * PI * radius;
        let width = total_width / self.columns as f32;

        let rotation = (2.0 * PI / self.columns as f32) * (column as f32);

        let (sin_x, cos_x) = f32::sin_cos(-rotation);

        let x = radius * sin_x;
        let y = radius * cos_x;

        Transform {
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::new(
                width - 2.0 * self.padding_x,
                height - 2.0 * self.padding_y,
                MIN_Z,
            ),
            translation: Vec3::new(x + self.offset_x, y + self.offset_y, 0.0),
        }
    }
}
