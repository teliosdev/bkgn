use std::{cell::RefCell, num::NonZeroUsize};

use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use rand_distr::Normal;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    Rgb,
    Yiq,
}

pub struct AbberateFilter {
    pub channel_shifts: [i32; 3],
    pub color_space: ColorSpace,
}

impl AbberateFilter {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        AbberateFilter {
            channel_shifts: [r, g, b],
            color_space: ColorSpace::Yiq,
        }
    }
}

impl super::Filter for AbberateFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        let height = image.dimensions().1;
        for y in 0..height {
            for (i, shift) in self.channel_shifts.iter().cloned().enumerate() {
                let direction = shift.is_positive();
                let norm = shift.abs() as u32;

                super::shift_row(
                    image,
                    y,
                    norm,
                    direction,
                    |from, to| shift_color(i, self.color_space, from, to),
                    |_to| {},
                )
            }
        }
    }
}

fn shift_color(channel: usize, space: ColorSpace, from: image::Rgb<u8>, to: &mut image::Rgb<u8>) {
    match space {
        ColorSpace::Rgb => {
            to.0[channel] = from.0[channel];
        }
        ColorSpace::Yiq => {
            let from_yiq = to_yiq(from.0);
            let mut to_yiq = to_yiq(to.0);
            to_yiq[channel] = from_yiq[channel];
            *to = image::Rgb(to_rgb(to_yiq));
        }
    }
}

pub struct NullFilter<D: Distribution<f32>> {
    pub null_start_chance: Bernoulli,
    pub null_distribution: D,
    pub null_component: Option<usize>,
    pub color_space: ColorSpace,
}

impl NullFilter<Normal<f32>> {
    pub fn new(
        selection: f64,
        average_null: f32,
        null_component: impl Into<Option<usize>>,
    ) -> Self {
        NullFilter {
            null_start_chance: Bernoulli::new(selection)
                .expect("selection distribution should be between 0 and 1"),
            null_distribution: Normal::new(average_null, average_null / 2.0).expect("???"),
            null_component: null_component.into(),
            color_space: ColorSpace::Rgb,
        }
    }
}

impl<D: Distribution<f32>> super::Filter for NullFilter<D> {
    fn filter(&self, image: &mut image::RgbImage) {
        let mut rng = rand::thread_rng();
        let mut current_null: Option<NonZeroUsize> = None;
        let mut current_component: usize =
            self.null_component.unwrap_or_else(|| rng.gen_range(0, 3));
        for y in 0..image.dimensions().1 {
            for x in 0..image.dimensions().0 {
                if let Some(current) = current_null.take() {
                    let current = current.get() - 1;
                    current_null = NonZeroUsize::new(current);
                    let pixel =
                        null_pixel(image.get_pixel(x, y), current_component, self.color_space);
                    image.put_pixel(x, y, pixel);
                } else {
                    if rng.sample(&self.null_start_chance) {
                        let stretch = rng.sample(&self.null_distribution).max(1.0) as usize;
                        current_null = NonZeroUsize::new(stretch);
                        current_component =
                            self.null_component.unwrap_or_else(|| rng.gen_range(0, 3));
                    }
                }
            }
        }
    }
}

fn null_pixel(pixel: &image::Rgb<u8>, component: usize, color_space: ColorSpace) -> image::Rgb<u8> {
    match color_space {
        ColorSpace::Rgb => {
            let mut array = pixel.0;
            array[component] = 0;
            image::Rgb::from(array)
        }
        ColorSpace::Yiq => {
            let mut array = to_yiq(pixel.0);
            array[component] = 0.0;
            image::Rgb::from(to_rgb(array))
        }
    }
}

fn to_yiq(rgb: [u8; 3]) -> [f64; 3] {
    let r = rgb[0] as f64 / 255.0;
    let g = rgb[1] as f64 / 255.0;
    let b = rgb[2] as f64 / 255.0;

    // using FCC YIQ, because why not
    let y = r * 0.30 + g * 0.59 + b * 0.11;
    let i = (b - y) * -0.27 + (r - y) * 0.74;
    let q = (b - y) * 0.41 + (r - y) * 0.48;

    [y, i, q]
}

fn to_rgb(yiq: [f64; 3]) -> [u8; 3] {
    let rf = yiq[0] + 0.9469 * yiq[1] + 0.6236 * yiq[2];
    let gf = yiq[0] + -0.2748 * yiq[1] + -0.6357 * yiq[2];
    let bf = yiq[0] + -1.1 * yiq[1] + 1.7 * yiq[2];

    [(rf * 255.0) as u8, (gf * 255.0) as u8, (bf * 255.0) as u8]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hex(v: u32) -> [u8; 3] {
        [
            ((v & 0xff0000) >> 16) as u8,
            ((v & 0x00ff00) >> 8) as u8,
            (v & 0x0000ff) as u8,
        ]
    }

    #[test]
    fn test_yiq_conversion() {
        let color = hex(0xe0e0e0);
        assert_eq!(color, to_rgb(to_yiq(color)));
    }

    #[test]
    fn test_yiq_corruption() {
        let a = hex(0xeaebec);
        let b = hex(0x333333);
        let mut a_yiq = to_yiq(a);
        let b_yiq = to_yiq(b);
        a_yiq[1] = b_yiq[1];
        let result = to_rgb(a_yiq);
        assert_ne!(a, result);
    }
}
