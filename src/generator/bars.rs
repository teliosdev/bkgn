use crate::position::{Position, Region};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct BarGenerator {
    /// The size of the image, in pixels.
    pub image_size: Position,
    /// The bars present in the image.
    pub bars: Vec<(u32, u32, image::Rgb<u8>)>,

    /// The initial offset of the bars into the image.
    pub initial_offset: u32,
    /// The angel the bars are placed at.
    pub angle: f64,

    /// Whether or not the bars are vertical.
    pub vertical: bool,

    pub background_color: image::Rgb<u8>,
}

impl BarGenerator {
    pub fn push_bar(&mut self, plus: u32, size: u32, color: image::Rgb<u8>) {
        let last = self.bars.last().map(|(o, s, _)| *o + *s).unwrap_or(0);
        self.bars.push((last + plus, size, color));
    }

    fn find_bar(&self, offset: u32) -> Option<image::Rgb<u8>> {
        self.bars.iter().rfind(|(o, s, _)| {
            offset > *o && offset < (*o + *s)
        }).map(|(_, _, v)| *v)
    }
}

impl super::Generator<image::Rgb<u8>> for BarGenerator {
    fn generate(&self) -> image::RgbImage {
        fn angle_adjustment(other: u32, max: u32, angle: f64) -> u32 {
            ((angle.sin() * (other as f64))) as u32
        }
        let parallel: fn(u32, u32) -> u32 = if self.vertical { |x, _y| x } else { |_x, y| y };
        let perpendicular: fn(u32, u32) -> u32 = if self.vertical { |_x, y| y } else { |x, _y| x };

        image::ImageBuffer::from_fn(self.image_size.x(), self.image_size.y(), |x, y| {
            let current_offset = parallel(x, y);
            let adjustment = angle_adjustment(perpendicular(x, y), parallel(self.image_size.x(), self.image_size.y()), self.angle);

            // dbg!((parallel(x, y), perpendicular(x, y), adjustment));
            let current_offset = current_offset.saturating_add(adjustment);
            if current_offset < self.initial_offset {
                self.background_color
            } else {
                let current_offset = current_offset.saturating_sub(self.initial_offset);//.saturating_sub(adjustment);

                self.find_bar(current_offset).unwrap_or(self.background_color)
            }
        })
    }
}
