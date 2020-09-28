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
            color_space: ColorSpace::Rgb,
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
