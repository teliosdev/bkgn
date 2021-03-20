use crate::position::Position;

pub struct StripeGenerator {
    pub image_size: Position,
    pub stripes: Vec<Stripe>,
    pub stripe_shift: f64,
    pub top_offset: u32,
    pub background_color: image::Rgb<u8>,

    pub error_color: image::Rgb<u8>,
}

pub struct Stripe {
    pub width: u32,
    pub color: image::Rgb<u8>,
    pub padding_bottom: u32,
}

impl Stripe {
    pub fn new(width: u32, color: image::Rgb<u8>, padding_bottom: u32) -> Self {
        Stripe {
            width,
            color,
            padding_bottom,
        }
    }
}

impl super::Generator<image::Rgb<u8>> for StripeGenerator {
    fn generate(&self) -> image::RgbImage {
        let mut tree = std::collections::BTreeMap::new();
        tree.insert(0, self.background_color);
        let mut offset = self.top_offset;
        for stripe in self.stripes.iter() {
            tree.insert(offset, stripe.color);
            tree.insert(offset + stripe.width, self.background_color);
            offset += stripe.width + stripe.padding_bottom;
        }

        let pick = |y| tree.range(0..=y).rev().next().map(|(_, p)| *p).unwrap();

        image::ImageBuffer::from_fn(self.image_size.x(), self.image_size.y(), |x, y| {
            let progress = x as f64 / (self.image_size.x() as f64);
            let modifier = self.stripe_shift * progress;
            let corrected_y = y + modifier as u32;
            let until_next = (modifier % 1.0) as f32;

            let current = pick(corrected_y);
            let next = pick(corrected_y + 1);

            let actual = imageproc::pixelops::interpolate(current, next, 1.0 - until_next);
            actual
        })
    }
}
