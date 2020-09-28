pub struct ScanFilter {
    pub lines: u32,
    pub vary: i16,
}

impl ScanFilter {
    pub fn new(lines: u32, vary: i16) -> Self {
        ScanFilter { lines, vary }
    }
}

impl super::Filter for ScanFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        let (width, height) = image.dimensions();
        let pixels_per_line = (height as f64) / (self.lines as f64);

        for y in 0..height {
            let is_even = ((y as f64 / pixels_per_line as f64).floor() as u32) % 2 == 0;
            let progress = (y as f64 % pixels_per_line) / pixels_per_line;
            let progress = (-(progress - 0.5).abs()) + 0.5;
            let progress = if is_even { progress } else { progress * -1.0 };
            let by = (progress * (self.vary as f64)) as i16;

            for x in 0..width {
                image.put_pixel(x, y, brighten(*image.get_pixel(x, y), by));
            }
        }
    }
}

fn brighten(pixel: image::Rgb<u8>, by: i16) -> image::Rgb<u8> {
    fn normal_mutate(value: u8, by: i16) -> u8 {
        (by + (value as i16)).clamp(0, 255) as u8
    }

    image::Rgb([
        normal_mutate(pixel.0[0], by),
        normal_mutate(pixel.0[1], by),
        normal_mutate(pixel.0[2], by),
    ])
}
