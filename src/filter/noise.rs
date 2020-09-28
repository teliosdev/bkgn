use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct NoiseFilter<D: Distribution<u8>> {
    pub rate: f64,
    pub luminescence_distribution: D,
}

impl NoiseFilter<Uniform<u8>> {
    pub fn new(rate: f64, low: u8, high: u8) -> Self {
        NoiseFilter {
            rate,
            luminescence_distribution: Uniform::new_inclusive(low, high),
        }
    }
}

impl<D: Distribution<u8>> super::Filter for NoiseFilter<D> {
    fn filter(&self, image: &mut image::RgbImage) {
        let dist =
            Bernoulli::new(self.rate).expect("correct distribution should be between 0.0 and 1.0");
        let mut rng = SmallRng::from_rng(&mut rand::thread_rng()).unwrap();

        for pixel in image.pixels_mut() {
            if rng.sample(&dist) {
                let value = rng.sample(&self.luminescence_distribution);
                *pixel = image::Rgb([value, value, value]);
            }
        }
    }
}
