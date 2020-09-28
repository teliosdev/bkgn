pub struct BarrelFilter {
    background_color: image::Rgb<u8>,
    scale: f32,
}

impl BarrelFilter {
    pub fn new(scale: f32, background_color: image::Rgb<u8>) -> Self {
        BarrelFilter {
            scale,
            background_color,
        }
    }
}

impl super::Filter for BarrelFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        let center_x = image.dimensions().0 as f32 / 2.0;
        let center_y = image.dimensions().1 as f32 / 2.0;
        *image = imageproc::geometric_transformations::warp_with(
            image,
            |x, y| {
                // y -> height, x -> width

                fn center_curve(center: f32, pos: f32) -> f32 {
                    let adjusted = (pos / center) - 1.0; // should be 0..1..2
                    -1.0 * adjusted.powi(2) + 1.0 // should be between 0 and 1, 1 being when at the center
                }

                let adjust = (1.0 - center_curve(center_x, x))
                    * (1.0 - center_curve(center_y, y))
                    * ((y - center_y).signum());
                // eprintln!("curve({:?}, {:?})={:?}", y, adjust, y + self.scale * adjust);
                (x, y + self.scale * adjust)
            },
            imageproc::geometric_transformations::Interpolation::Bilinear,
            self.background_color,
        )
    }
}
