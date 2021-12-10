#![allow(dead_code)]
use bitmesh_server;
use bitmesh_probes;
use image::*;
use mnist::*;
use ndarray::prelude::*;
use std::{rc::*, thread, time, u32};

struct Bit {
    value: f32,
}

impl Bit {
    fn test_bit(&self) -> bool {
        self.value > 0.5
    }
    fn set_bit(&mut self) {
        self.value = 1.0;
    }
    fn clr_bit(&mut self) {
        self.value = 0.0;
    }
}

struct CellConv {
    stride: u32,
    width: u32,
}

enum TemporalFunction {
    LastUpTime,   // RC(x) filter
    LastDownTime, // RC(1-x) filter
}

enum CellConnection {
    Full,
    Random(f32),
    Stride(u32),
    Convoluate(Vec<CellConnection>),
}

struct CellLink {
    from: Rc<Cell>,
    connection: CellConnection,
}

struct Cell {
    shape: Vec<u32>,
    links: Vec<CellLink>,
}

struct Mesh {
    cells: Vec<Rc<Cell>>,
}

impl Mesh {
    fn new() -> Mesh {
        Mesh { cells: vec![] }
    }
    fn create_cell(&mut self, shape: Vec<u32>) -> Rc<Cell> {
        let cell = Rc::new(Cell {
            shape: shape,
            links: vec![],
        });
        self.cells.push(cell.clone());
        cell
    }
    fn connect_cells(&mut self, from: &Rc<Cell>, to: &mut Rc<Cell>, connection: CellConnection) {
        Rc::get_mut(to).unwrap().links.push(CellLink {
            from: from.clone(),
            connection: connection,
        });
    }
}
trait BitScalarView {}

fn test() {
    let mut m = Mesh::new();
    let c0 = m.create_cell(vec![2]);
    let mut c1 = m.create_cell(vec![1]);
    m.connect_cells(&c0, &mut c1, CellConnection::Full);
}
fn main() {
    bitmesh_server::run_services();

    let Mnist { trn_img, .. } = MnistBuilder::new()
        .base_path(
            std::env::current_dir()
                .unwrap()
                .join("data")
                .to_str()
                .unwrap(),
        )
        .label_format_digit()
        .training_set_length(50_000)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();

    // Can use an Array2 or Array3 here (Array3 for visualization)
    let train_data = Array3::from_shape_vec((50_000, 28, 28), trn_img.clone())
        .expect("Error converting images to Array3 struct");
    //println!("{:#.1?}\n", train_data.slice(s![image_num, .., ..]));

    loop {
        let mut img = RgbaImage::new(28, 28);
        let idx = rand::random::<u32>() % 50_000;
        for i in 0..28u32 {
            for j in 0..28u32 {
                let v = *train_data
                    .get([idx as usize, i as usize, j as usize])
                    .unwrap();
                img.put_pixel(j, i, Rgba([v, v, v, 255]));
            }
        }

        bitmesh_probes::write(&img);
        thread::sleep(time::Duration::from_millis(100))
    }
}
