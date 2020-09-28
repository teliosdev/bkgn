use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use rand_distr::Normal;

pub struct MarchFilter<SeD: Distribution<bool>, ShD: Distribution<f64>> {
    pub selection_distribution: SeD,
    pub shift_distribution: ShD,
    pub max_shift: u32,
    pub keep_adjustment: Option<Box<dyn Fn(f64) -> f64>>,
    pub background_color: image::Rgb<u8>,
}

impl MarchFilter<Bernoulli, Normal<f64>> {
    pub fn new(selection: f64, max_shift: u32, background_color: image::Rgb<u8>) -> Self {
        MarchFilter {
            selection_distribution: Bernoulli::new(selection)
                .expect("selection distribution should be between 0 and 1"),
            shift_distribution: Normal::new(0.0, 0.25).expect("???"),
            max_shift,
            keep_adjustment: Some(Box::new(|f| f * 0.95)),
            background_color,
        }
    }
}

impl<SeD: Distribution<bool>, ShD: Distribution<f64>> super::Filter for MarchFilter<SeD, ShD> {
    fn filter(&self, image: &mut image::RgbImage) {
        let mut rng = rand::thread_rng();
        let (_, height) = image.dimensions();

        let mut keep = 0.0;

        for y in 0..height {
            let shift = if !rng.sample(&self.selection_distribution) {
                keep
            } else {
                let sample = rng.sample(&self.shift_distribution);
                (sample + keep).clamp(-1.0, 1.0)
            };

            if let Some(adjustment) = self.keep_adjustment.as_ref() {
                keep = adjustment(shift);
            }

            if shift == 0.0 {
                continue;
            }

            super::shift_row(
                image,
                y,
                (self.max_shift as f64 * shift.abs()) as u32,
                shift.is_sign_positive(),
                |from, to| *to = from,
                |to| *to = self.background_color,
            );

            // for x in 0..width {
            //     if shift >= 0.0 {
            //         let from_pixel_x = x + (self.max_shift as f64 * shift) as u32;
            //         let pixel = get(image, from_pixel_x, y)
            //             .cloned()
            //             .unwrap_or(self.background_color);
            //         image.put_pixel(x, y, pixel);
            //     } else {
            //         let nx = width - 1 - x;
            //         let nshift = (self.max_shift as f64 * shift.abs() as f64) as u32;
            //         let pixel = if nshift > nx {
            //             self.background_color
            //         } else {
            //             *image.get_pixel(nx - nshift, y)
            //         };
            //         image.put_pixel(nx, y, pixel);
            //     }
            // }
        }
    }
}
