use crate::position::{Position, Region};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct MazeGenerator {
    /// The size of the box, in pixels.  This will be colored in the
    /// block color when picked.
    pub cell_size: Position,
    pub wall_size: Position,
    /// The size of the image, in pixels.
    pub image_size: Position,
    /// The padding on all sides of the image.  The `x` component is
    /// for both the left and right, and the `y` component is for the
    /// top and bottom.
    pub padding: Position,

    pub background_color: image::Rgb<u8>,
    pub block_color: image::Rgb<u8>,

    pub direction_weights: HashMap<Direction, u32>,
    pub default_weight: u32,
}

impl super::Generator<image::Rgb<u8>> for MazeGenerator {
    fn generate(&self) -> image::RgbImage {
        let block_size = self.cell_size + self.wall_size;
        let center_adjustment: Position =
            (self.image_size - self.padding * 2 - self.wall_size) % block_size;
        let inner_region = Region::new(
            self.padding + center_adjustment / 2,
            self.image_size - self.padding - center_adjustment / 2,
        )
        .expect("top left should be greater than bottom right?");

        let blocks_count: Position = inner_region.dimensions() / block_size;

        let grid = generate_maze(
            |d| {
                self.direction_weights
                    .get(&d)
                    .cloned()
                    .unwrap_or(self.default_weight)
            },
            blocks_count.x() as usize,
            blocks_count.y() as usize,
        );

        image::ImageBuffer::from_fn(self.image_size.x(), self.image_size.y(), |x, y| {
            let pos = Position::new(x, y);
            if !inner_region.contains(pos) {
                return self.background_color;
            }

            let adjusted: Position = (pos - self.padding - center_adjustment / 2) / block_size;
            let inside: Position = (pos - self.padding - center_adjustment / 2) % block_size;

            if let Some(row) = grid.get(adjusted.y() as usize) {
                if let Some(cell) = row.get(adjusted.x() as usize) {
                    if !cell.north && inside.y() < self.wall_size.y() {
                        self.block_color
                    } else if !cell.west && inside.x() < self.wall_size.x() {
                        self.block_color
                    } else if inside.x() < self.wall_size.x()
                        && inside.y() < self.wall_size.y()
                        && (!grid[adjusted.y() as usize - 1][adjusted.x() as usize].west
                            || !grid[adjusted.y() as usize][adjusted.x() as usize - 1].north)
                    {
                        self.block_color
                    } else {
                        self.background_color
                    }
                } else if let Some(left) = row.get(adjusted.x() as usize - 1) {
                    if !left.east {
                        self.block_color
                    } else {
                        self.background_color
                    }
                } else {
                    self.block_color
                }
            } else if let Some(above) = grid[adjusted.y() as usize - 1].get(adjusted.x() as usize) {
                if !above.south {
                    self.block_color
                } else {
                    self.background_color
                }
            } else {
                self.block_color
            }
        })
    }
}

pub fn generate_maze<F: Fn(Direction) -> u32>(
    weight: F,
    width: usize,
    height: usize,
) -> Vec<Vec<Cell>> {
    eprintln!("maze({:?}, {:?})", width, height);
    let mut grid = (0..height)
        .map(|_| (0..width).map(|_| Cell::default()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // return grid;
    let mut stack = vec![];
    stack.push((0usize, 0usize));

    while let Some((x, y)) = stack.last() {
        grid[*y][*x].visit = true;
        let possible = Direction::all()
            .iter()
            .flat_map(|d| d.vector(*x, *y, width, height))
            .filter(|v| !grid[v.y()][v.x()].visit)
            .collect::<Vec<_>>();

        if possible.is_empty() {
            stack.pop();
            continue;
        }

        let vector = possible
            .choose_weighted(&mut rand::thread_rng(), |d| weight(d.direction()))
            .expect("could not choose new direction");

        vector.direction().mark(&mut grid[*y][*x]);
        vector
            .direction()
            .reverse()
            .mark(&mut grid[vector.y()][vector.x()]);
        stack.push((vector.x(), vector.y()));
    }

    grid[0][0].north = true;
    grid[height - 1][width - 1].south = true;

    grid
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    visit: bool,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            visit: false,
            north: false,
            east: false,
            south: false,
            west: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct DirectionVector(Direction, usize, usize);

impl DirectionVector {
    fn direction(self) -> Direction {
        self.0
    }
    fn x(self) -> usize {
        self.1
    }
    fn y(self) -> usize {
        self.2
    }
}

impl Direction {
    fn vector(self, x: usize, y: usize, width: usize, height: usize) -> Option<DirectionVector> {
        match self {
            Direction::North if y > 0 => Some(DirectionVector(self, x, y - 1)),
            Direction::East if x + 1 < width => Some(DirectionVector(self, x + 1, y)),
            Direction::South if y + 1 < height => Some(DirectionVector(self, x, y + 1)),
            Direction::West if x > 0 => Some(DirectionVector(self, x - 1, y)),
            _ => None,
        }
    }

    fn reverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn mark(self, cell: &mut Cell) {
        match self {
            Direction::North => cell.north = true,
            Direction::East => cell.east = true,
            Direction::South => cell.south = true,
            Direction::West => cell.west = true,
        }
    }

    fn all() -> &'static [Direction] {
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

static GRID_ALPHABET: &[char] = &[
    '@', '╡', '╥', '╗', '╞', '═', '╔', '╦', '╨', '╝', '║', '╣', '╚', '╩', '╠', '╬',
];

pub fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            let v = GRID_ALPHABET[((cell.north as usize) << 3)
                | ((cell.east as usize) << 2)
                | ((cell.south as usize) << 1)
                | (cell.west as usize)];
            eprint!("{}", v);
        }
        eprint!("\n");
    }
}
