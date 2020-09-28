use crate::position::Position;

pub struct VignetteFilter {
    pub distance_offset: f64,
    pub scale: f64,
}

impl VignetteFilter {
    pub fn new(distance_offset: f64, scale: f64) -> Self {
        VignetteFilter {
            distance_offset,
            scale,
        }
    }
}

impl super::Filter for VignetteFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        let center = Position::from(image.dimensions()) / 2;

        for y in 0..image.dimensions().1 {
            for x in 0..image.dimensions().0 {
                let distance = distance_from_edge(center, Position::new(x, y));
                let value = (((distance + self.distance_offset) * self.scale) * 255.0)
                    .clamp(0.0, 255.0) as u8;
                let mutate = |v: u8| v.saturating_sub(value);
                let pixel = *image.get_pixel(x, y);
                let new = image::Rgb([mutate(pixel.0[0]), mutate(pixel.0[1]), mutate(pixel.0[2])]);
                image.put_pixel(x, y, new);
            }
        }

        // for (x, y, pixel) in image.enumerate_pixels_mut() {
        //     let distance = distance_from_edge(center, Position::new(x, y));
        //     let value =
        //         ((distance + self.distance_offset) * (self.scale as f64)).clamp(0.0, 255.0) as u8;
        //     pixel
        //         .0
        //         .iter_mut()
        //         .for_each(|v| *v = (*v + value).clamp(0, 255));
        // }
    }
}

fn distance_from_edge(center: Position, position: Position) -> f64 {
    let x = (center.x() as f64 - position.x() as f64) / center.x() as f64;
    let y = (center.y() as f64 - position.y() as f64) / center.y() as f64;
    let d = (x.powi(2) + y.powi(2)).sqrt();

    d
}
