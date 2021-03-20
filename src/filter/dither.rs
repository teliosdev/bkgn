pub struct DitherFilter {
    matrix: Vec<Vec<f32>>,
    white: image::Rgb<u8>,
    black: image::Rgb<u8>,
}

impl DitherFilter {
    pub fn new(levels: u16, white: image::Rgb<u8>, black: image::Rgb<u8>) -> Self {
        DitherFilter {
            matrix: dbg!(generate_matrix(levels)),
            white, black
        }
    }
}

type Matrix = Vec<Vec<f32>>;

// size is a power of 2.
fn generate_matrix(size: u16) -> Matrix {
    // let mut out = Vec::with_capacity(levels as usize);
    // out.resize_with(levels as usize, || Vec::with_capacity(levels as usize));
    // for i in 0..2 {
    //     let row = &mut out[i as usize];
    //     for j in 0..2 {
    //         let value = interleave(i ^ j, i).reverse_bits();
    //         row.push(value as f32 );
    //     }
    // }

    // out

    let mut matrix = base_matrix();

    for i in 0..size {
        let mut set = Vec::with_capacity(4);
        multiply_matrix(&mut matrix, 4.0);
        set.push(matrix.clone());
        set.push({
            let mut base = matrix.clone();
            add_matrix(&mut base, 2.0);
            base
        });
        set.push({
            let mut base = matrix.clone();
            add_matrix(&mut base, 3.0);
            base
        });
        set.push({
            let mut base = matrix.clone();
            add_matrix(&mut base, 1.0);
            base
        });

        matrix = combine_matricies(set);
        // divide_matrix(&mut matrix, (2.0 * ((i + 1) as f32)).powi(2));
    }
    divide_matrix(&mut matrix, (2.0f32).powf(2.0 * (size as f32) + 2.0));

    add_matrix(&mut matrix, 0.25);
    matrix
}

fn multiply_matrix(matrix: &mut Matrix, value: f32) {
    for row in matrix.iter_mut() {
        for item in row.iter_mut() {
            *item = *item * value;
        }
    }
}

fn divide_matrix(matrix: &mut Matrix, value: f32) {
    for row in matrix.iter_mut() {
        for item in row.iter_mut() {
            *item = *item / value;
        }
    }
}

fn add_matrix(matrix: &mut Matrix, value: f32) {
    for row in matrix.iter_mut() {
        for item in row.iter_mut() {
            *item = *item + value;
        }
    }
}

fn combine_matricies(set: Vec<Matrix>) -> Matrix {
    assert_eq!(set.len(), 4);

    let rows = set[0].len();
    let columns = set[0][0].len();
    assert_eq!(rows, columns);

    let mut matrix = Vec::with_capacity(rows * 2);
    matrix.resize_with(rows * 2, || {
        let mut row = Vec::with_capacity(columns * 2);
        row.resize_with(columns * 2, || 0.0f32);
        row
    });

    eprintln!("rows={:?}, columns={:?}", rows, columns);

    for x in 0..(rows * 2) {
        for y in 0..(columns * 2) {
            let m = (x / rows) * 2+ (y / columns);
            eprintln!("matrix[x={:?}][y={:?}] = set[m={:?}][i={:?}][j={:?}]", x, y, m, x % rows, y % columns);
            let v = set[m][x % rows][y % columns];
            eprintln!("v = {:?}", v);
            matrix[x][y] = v;
        }
    }

    matrix
}

fn base_matrix() -> Matrix {
    [[0f32, 2f32], [3f32, 1f32]].iter()
        .map(|v| v.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn interleave(a: u16, b: u16) -> u32 {
    fn interleave_zeros(input: u16) -> u32 {
        let mut word = input as u32;
        word = (word ^ (word << 16)) & 0x0000ffff;
        word = (word ^ (word << 8 )) & 0x00ff00ff;
        word = (word ^ (word << 4 )) & 0x0f0f0f0f;
        word = (word ^ (word << 2 )) & 0x33333333;
        word = (word ^ (word << 1 )) & 0x55555555;
        word
    }

    interleave_zeros(a) | (interleave_zeros(b) << 1)
}

impl super::Filter for DitherFilter {
    fn filter(&self, image: &mut image::RgbImage) {
        // let mut temp = image::RgbImage::from_raw(0, 0, vec![]).unwrap();

        let (width, height) = image.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let pixel = mutate(x, y, pixel, &self.matrix, &self.white, &self.black);
                image.put_pixel(x, y, pixel);
            }
        }
    }
}

fn mutate(x: u32, y: u32, pixel: &image::Rgb<u8>, matrix: &Vec<Vec<f32>>, white: &image::Rgb<u8>, black: &image::Rgb<u8>) -> image::Rgb<u8> {
    let row = &matrix[x as usize % matrix.len()];
    let value = row[y as usize % matrix.len()];
    let brightness = pixel_brightness(pixel);

    // if brightness > value {
    //     *white
    // } else {
    //     *black
    // }

    new_colors(pixel, brightness > value)


    // if (brightness + (rand::random::<f32>() - 0.5) / 2.0) > 0.5 {
    //     *white
    // } else {
    //     *black
    // }
}

fn new_colors(pixel: &image::Rgb<u8>, upper: bool) -> image::Rgb<u8> {
    const BANDS: f32 = 15.0;
    fn band(v: u8) -> f32 {
        ((v as f32 / 255.0) * BANDS).floor()
    }

    fn deband(v: f32) -> u8 {
        ((v.clamp(0.0, BANDS) / BANDS) * 255.0) as u8
    }

    let r = band(pixel.0[0]);
    let g = band(pixel.0[1]);
    let b = band(pixel.0[2]);

    if upper {
        image::Rgb::from([deband(r + 1.0), deband(g + 1.0), deband(b + 1.0)])
    } else {
        image::Rgb::from([deband(r - 1.0), deband(g - 1.0), deband(b - 1.0)])
    }
}

fn pixel_brightness(pixel: &image::Rgb<u8>) -> f32 {
    let r = pixel.0[0] as f32 / 255.0;
    let g = pixel.0[1] as f32 / 255.0;
    let b = pixel.0[2] as f32 / 255.0;
    (r * 0.30 + g * 0.59 + b * 0.11).clamp(0.0, 1.0)
}
