mod file;
pub mod maze;
mod noise;
mod random;
mod bars;
pub mod stripe;

pub use self::file::FileGenerator;
pub use self::maze::MazeGenerator;
pub use self::noise::NoiseGenerator;
pub use self::random::RandomGenerator;
pub use self::bars::BarGenerator;
pub use self::stripe::StripeGenerator;

pub trait Generator<P: image::Pixel> {
    fn generate(&self) -> image::ImageBuffer<P, Vec<P::Subpixel>>;
}
