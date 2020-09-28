pub struct CropFilter {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

impl CropFilter {
    pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        CropFilter {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl super::Filter for CropFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        let width = image.dimensions().0 - self.left - self.right;
        let height = image.dimensions().1 - self.top - self.bottom;
        *image = image::imageops::crop_imm(image, self.left, self.top, width, height).to_image();
    }
}
