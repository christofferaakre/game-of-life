use image::{Rgb, RgbImage};

#[derive(Debug, Clone)]
pub struct Cell {
    neighbour_indices: Vec<(usize, usize)>,
    alive: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            alive: false,
            neighbour_indices: vec![],
        }
    }
    pub fn neighbour_count(&self, cells: &Vec<Vec<Cell>>) -> u32 {
        let mut neighbour_count = 0;
        for &position in &self.neighbour_indices {
            neighbour_count += cells[position.0][position.1].alive as u32;
        }

        neighbour_count
    }
}

#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
}

const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);

pub enum HorizontalEdge {
    Left,
    Right,
    NoEdge,
}

pub enum VerticalEdge {
    Top,
    Bottom,
    NoEdge,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Cell::new())
            }
            cells.push(row);
        }
        let mut grid = Self {
            width,
            height,
            cells,
        };

        grid.compute_neighbour_indices();
        grid
    }

    pub fn advance(&mut self) {
        let old_cells = self.cells.clone();
        for row in self.cells.iter_mut() {
            for cell in row {
                match cell.neighbour_count(&old_cells) {
                    0..=1 => {
                        cell.alive = false;
                    }
                    3 => {
                        cell.alive = true;
                    }
                    4.. => {
                        cell.alive = false;
                    }
                    _ => {}
                };
            }
        }
    }

    pub fn compute_neighbour_indices(&mut self) {
        for (x, row) in self.cells.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                let mut x_indices = vec![x];
                let mut y_indices = vec![y];

                if x != 0 {
                    x_indices.push(x - 1);
                }

                if x != self.width as usize - 1 {
                    x_indices.push(x + 1);
                }

                if y != 0 {
                    y_indices.push(y - 1);
                }

                if y != self.height as usize - 1 {
                    y_indices.push(y + 1);
                }

                for neighbour_x in x_indices {
                    for &neighbour_y in &y_indices {
                        if neighbour_x != x || neighbour_y != y {
                            cell.neighbour_indices.push((neighbour_x, neighbour_y));
                        }
                    }
                }
            }
        }
    }

    pub fn load_object(&mut self, object: &Object, offset: (usize, usize)) {
        for (x, y) in &object.pixels {
            self.cells
                .get_mut(*x + offset.0)
                .unwrap()
                .get_mut(*y + offset.1)
                .unwrap()
                .alive = true;
        }
    }

    pub fn draw(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                let color = match cell.alive {
                    true => GREEN,
                    false => RED,
                };

                image.put_pixel(x as u32, y as u32, color);
            }
        }

        image
    }
}

const WIDTH: u32 = 16;
const HEIGHT: u32 = 16;

const TARGET_WIDTH: u32 = 512;
const TARGET_HEIGHT: u32 = 512;

pub struct Object {
    pub pixels: Vec<(usize, usize)>,
}

impl Object {
    pub fn from_file(filename: &str) -> Self {
        let file_contents = std::fs::read_to_string(filename).expect("Failed to read file");
        Self::from_string(file_contents)
    }

    fn from_string(buffer: String) -> Object {
        let mut pixels = vec![];
        let buffer = buffer.replace("(", "");
        let buffer = buffer.replace(")", "");
        let coords = buffer.split_whitespace();
        for coord in coords {
            let mut coord_vals = coord.split(",");
            let x: usize = coord_vals
                .next()
                .expect("Failed to get next value in comma split")
                .parse()
                .expect("Failed to parse coordinate to usize");
            let y: usize = coord_vals
                .next()
                .expect("Failed to get next value in comma split")
                .parse()
                .expect("Failed to parse coordinate to usize");
            pixels.push((x, y));
        }
        Self { pixels }
    }
}

fn main() {
    let mut grid = Grid::new(16, 16);

    let glider = Object::from_file("data/glider.life");
    grid.load_object(&glider, (0, 0));

    for generation in 0..10 {
        println!("Generation {}", generation);
        let filename = format!("out/generation-{}.png", generation);

        let image = grid.draw();
        let image = image::imageops::resize(
            &image,
            TARGET_WIDTH,
            TARGET_HEIGHT,
            image::imageops::FilterType::Nearest,
        );
        grid.advance();
        image.save(filename).expect("Failed to save file");
    }

    // std::process::Command::new("sxiv")
    //     .arg(filename)
    //     .output()
    //     .expect("Failed to execute sxiv");
}
