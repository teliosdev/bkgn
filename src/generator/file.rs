pub struct FileGenerator {
    path: std::path::PathBuf,
}

impl FileGenerator {
    pub fn new<A: AsRef<std::path::Path>>(path: A) -> Self {
        FileGenerator {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl super::Generator<image::Rgb<u8>> for FileGenerator {
    fn generate(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        image::open(&self.path).expect("invalid image").to_rgb()
    }
}
