#![feature(clamp)]

mod filter;
mod generator;
mod position;

fn main() {
    test_generate();
    println!("Hello, world!");
}

fn test_generate() {
    use self::filter::Filter;
    use self::generator::Generator;
    use self::position::Position;

    fn rgb(r: u8, g: u8, b: u8) -> image::Rgb<u8> {
        image::Rgb::from([r, g, b])
    }
    fn hex(hex: u32) -> image::Rgb<u8> {
        rgb(
            ((hex & 0xff0000) >> 16) as u8,
            ((hex & 0x00ff00) >> 8) as u8,
            (hex & 0x0000ff) as u8,
        )
    }

    let barrel_scale = 60;
    // let generator = self::generator::RandomGenerator {
    //     block_size: Position::new(10, 10),
    //     image_size: Position::new(1600 + barrel_scale * 2, 900 + barrel_scale * 2),
    //     padding: Position::new(200 + barrel_scale, 150 + barrel_scale),
    //     distribution: 0.30,
    //     block_color: hex(0xc0c0c0),
    //     // block_color: hex(0xff3300),
    //     background_color: hex(0x333333),
    // };
    // // let generator = self::generator::FileGenerator::new("poly.jpg");

    // let blur_filter = self::filter::BlurFilter::new(1.0);
    // let shift_filter = self::filter::MarchFilter::new(0.0325, 80, hex(0x333333));
    // let abberate_filter = self::filter::AbberateFilter::new(2, 0, -3);
    // let scan_filter = self::filter::ScanFilter::new(160 * 2, 0x10 / 2);
    // let noise_filter = self::filter::NoiseFilter::new(0.0325, 0x33, 0x99);
    // let vignette_filter = self::filter::VignetteFilter::new(-0.9, 0.3);
    // let barrel_filter = self::filter::BarrelFilter::new(barrel_scale as f32, hex(0x333333));
    // let crop_filter =
    //     self::filter::CropFilter::new(barrel_scale, barrel_scale, barrel_scale, barrel_scale);

    // let generator = self::generator::RandomGenerator {
    //     block_size: Position::new(10 * 2, 10 * 2),
    //     image_size: Position::new(3440 + barrel_scale * 2, 1440 + barrel_scale * 2),
    //     padding: Position::new(200 + barrel_scale, 150 + barrel_scale),
    //     distribution: 0.30,
    //     block_color: hex(0xc0c0c0),
    //     // block_color: hex(0xff3300),
    //     background_color: hex(0x333333),
    // };
    let generator = self::generator::MazeGenerator {
        cell_size: Position::new(14, 14),
        wall_size: Position::new(6, 6),
        image_size: Position::new(3440 + barrel_scale * 2, 1440 + barrel_scale * 2),
        padding: Position::new(200 - 6 + barrel_scale, 150 - 6 + barrel_scale),
        block_color: hex(0xc0c0c0),
        // block_color: hex(0xff3300),
        background_color: hex(0x333333),
        direction_weights: {
            let mut weights = std::collections::HashMap::new();
            weights.insert(self::generator::maze::Direction::West, 30);
            weights.insert(self::generator::maze::Direction::East, 30);
            weights
        },
        default_weight: 10,
    };
    // let generator = self::generator::FileGenerator::new("poly.jpg");

    let blur_filter = self::filter::BlurFilter::new(1.0);
    let shift_filter = self::filter::MarchFilter::new(0.0125, 80, hex(0x333333));
    let abberate_filter = self::filter::AbberateFilter::new(2, 0, -3);
    let scan_filter = self::filter::ScanFilter::new(160 * 3, 0x10 / 2);
    let noise_filter = self::filter::NoiseFilter::new(0.0325, 0x33, 0x99);
    let vignette_filter = self::filter::VignetteFilter::new(-0.9, 0.3);
    let barrel_filter = self::filter::BarrelFilter::new(barrel_scale as f32, hex(0x333333));
    let crop_filter =
        self::filter::CropFilter::new(barrel_scale, barrel_scale, barrel_scale, barrel_scale);

    let mut image = time("generate", || generator.generate());
    time("filter.shift", || shift_filter.filter(&mut image));
    time("filter.noise", || noise_filter.filter(&mut image));
    time("filter.blur", || blur_filter.filter(&mut image));
    time("filter.abberate", || abberate_filter.filter(&mut image));
    time("filter.scan", || scan_filter.filter(&mut image));
    time("filter.barrel", || barrel_filter.filter(&mut image));
    time("filter.vignette", || vignette_filter.filter(&mut image));
    time("filter.crop", || crop_filter.filter(&mut image));
    time("image.save", || {
        image.save_with_format("test.png", image::ImageFormat::Png)
    })
    .expect("could not save image");
}

fn time<R, F: FnOnce() -> R>(name: &str, block: F) -> R {
    let start = std::time::Instant::now();
    let v = block();
    eprintln!("time[{:?}] ... {}ms", name, start.elapsed().as_millis());
    v
}
