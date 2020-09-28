mod abberate;
mod barrel;
mod blur;
mod crop;
mod march;
mod noise;
mod scan;
mod vignette;

pub use self::abberate::AbberateFilter;
pub use self::barrel::BarrelFilter;
pub use self::blur::BlurFilter;
pub use self::crop::CropFilter;
pub use self::march::MarchFilter;
pub use self::noise::NoiseFilter;
pub use self::scan::ScanFilter;
pub use self::vignette::VignetteFilter;

pub trait Filter {
    fn filter(&self, image: &mut image::RgbImage);
}

// direction = false -> to the left, direction = true -> to the right
fn shift_row<F: FnMut(image::Rgb<u8>, &mut image::Rgb<u8>), D: FnMut(&mut image::Rgb<u8>)>(
    image: &mut image::RgbImage,
    y: u32,
    by: u32,
    direction: bool,
    mut action: F,
    mut default: D,
) {
    let width = image.dimensions().0;
    for x in 0..(image.dimensions().0) {
        let (nx, offset) = if direction {
            let nx = width - 1 - x;
            if nx <= by {
                (nx, None)
            } else {
                (nx, Some(nx - by))
            }
        } else {
            if x + by >= width {
                (x, None)
            } else {
                (x, Some(x + by))
            }
        };

        match offset {
            Some(off) => action(*image.get_pixel(off, y), image.get_pixel_mut(nx, y)),
            None => default(image.get_pixel_mut(nx, y)),
        }
    }
}
