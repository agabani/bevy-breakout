#[derive(Debug, Clone, Copy)]
pub(crate) struct Spec {
    pub(crate) columns: u32,
    pub(crate) rows: u32,

    pub(crate) border_bottom: f32,
    pub(crate) border_left: f32,
    pub(crate) border_right: f32,
    pub(crate) border_top: f32,

    pub(crate) padding_x: f32,
    pub(crate) padding_y: f32,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Transform {
    pub(crate) height: f32,
    pub(crate) width: f32,

    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Spec {
    #[allow(clippy::cast_precision_loss, unused)]
    pub(crate) fn transform(self, column: u32, row: u32) -> Transform {
        let total_height = self.border_top - self.border_bottom;
        let total_width = self.border_right - self.border_left;

        let border_height = total_height / self.rows as f32;
        let border_width = total_width / self.columns as f32;

        let height = border_height - 2.0 * self.padding_y;
        let width = border_width - 2.0 * self.padding_x;

        let x = column as f32 * border_width + self.border_left + self.padding_x + width / 2.0;
        let y = row as f32 * border_height + self.border_bottom + self.padding_y + height / 2.0;

        Transform {
            height,
            width,
            x,
            y,
        }
    }
}
