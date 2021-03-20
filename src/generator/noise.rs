use rand::Rng;

use crate::position::{Position, Region};

pub struct NoiseGenerator<F: noise::NoiseFn<[f64; 2]>> {
    f: F,
    pub image_scale: [f64; 2],
    pub value_scale: f64,
    pub background_color: image::Rgb<u8>,
    pub block_color: image::Rgb<u8>,
    pub image_size: Position,
}

pub struct DownSample<F: noise::NoiseFn<[f64; 4]>>(F, f64, f64);

impl<F> noise::NoiseFn<[f64; 2]> for DownSample<F>
where
    F: noise::NoiseFn<[f64; 4]>,
{
    fn get(&self, v: [f64; 2]) -> f64 {
        let v = [v[0], v[1], self.1, self.2];
        (self.0).get(v)
    }
}

impl NoiseGenerator<noise::Perlin> {
    pub fn new(
        image_size: Position,
        background_color: image::Rgb<u8>,
        block_color: image::Rgb<u8>,
        image_scale: f64,
        value_scale: f64,
    ) -> Self {
        use noise::Seedable;
        use rand::distributions::Open01;
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen();
        let f = noise::Perlin::new();
        let f = f.set_seed(seed);


        NoiseGenerator {
            f,//: DownSample(f, dbg!(rng.sample(Open01)), dbg!(rng.sample(Open01))),
            image_scale: [image_scale, image_scale],
            background_color,
            block_color,
            image_size,
            value_scale,
        }
    }
}

impl<F> super::Generator<image::Rgb<u8>> for NoiseGenerator<F>
where
    F: noise::NoiseFn<[f64; 2]>,
{
    fn generate(&self) -> image::RgbImage {
        image::ImageBuffer::from_fn(self.image_size.y(), self.image_size.x(), |x, y| {
            let point = [
                ((x as f64) / (self.image_size.x() as f64)) * self.image_scale[0],
                ((y as f64) / (self.image_size.x() as f64)) * self.image_scale[1],
            ];
            // let value = (self.f.get(point) * self.value_scale).clamp(0.0, 1.0);
            let value = self.f.get(point);
            let value = (value + 1.0) / 2.0;
            let value = value * self.value_scale;
            let value = value.clamp(0.0, 1.0);

            imageproc::pixelops::interpolate(self.background_color, self.block_color, value as f32)
        })
    }
}
