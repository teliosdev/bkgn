mod file;
pub mod maze;
mod random;
mod bars;

pub use self::file::FileGenerator;
pub use self::maze::MazeGenerator;
pub use self::random::RandomGenerator;
pub use self::bars::BarGenerator;

pub trait Generator<P: image::Pixel> {
    fn generate(&self) -> image::ImageBuffer<P, Vec<P::Subpixel>>;
}
