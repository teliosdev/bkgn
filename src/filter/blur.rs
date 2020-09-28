pub struct BlurFilter {
    pub sigma: f32,
}

impl BlurFilter {
    pub fn new(sigma: f32) -> Self {
        BlurFilter { sigma }
    }
}

impl super::Filter for BlurFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        // let mut temp = image::RgbImage::from_raw(0, 0, vec![]).unwrap();

        *image = image::imageops::blur(image, self.sigma)
    }
}
