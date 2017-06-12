#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate image;
extern crate imageproc;
extern crate nalgebra;
extern crate alga;

// use std::env;
// use std::path::Path;

// use image::{open, ImageBuffer};

// use imageproc::hog::*;

use nalgebra::core::*;
use nalgebra::core::dimension::*;
use alga::general::Inverse;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::io::BufReader;
use std::io::BufRead;

type DynamicF64VecMatrix = Matrix<f64, Dynamic, Dynamic, MatrixVec<f64, Dynamic, Dynamic>>;

type HeapF64VecMatrix4x4 = Matrix<f64, U4, U4, MatrixVec<f64, U4, U4>>;
type F64Matrix4x4 = Matrix4<f64>;

type HeapF64VecMatrix3x3 = Matrix<f64, U3, U3, MatrixVec<f64, U3, U3>>;
type F64Matrix3x3 = Matrix3<f64>;

const  TOL: f64 = 0.000_000_01;

fn main() {
    let rows = Dynamic::new(4);
    let columns = Dynamic::new(4);
    let a_matrix : DynamicF64VecMatrix = Matrix::identity_generic(rows , columns);
    let other_matrix : DynamicF64VecMatrix = Matrix::identity_generic(rows , columns);

    let mut c = a_matrix * other_matrix;

    let mut total_index = 0;
    for mxx in c.iter_mut() {
        let i = total_index % 4;
        let j = (total_index / 4) % 4;
        total_index = total_index + 1;
        if j == 0 {
            *mxx = 8f64;
        }
        println!("{}", mxx);
    }

    println!("");

    let w = c.column(0);
    for mxx in w.iter() {
        println!("{}", mxx);
    }

    println!("{}" , c);
}

struct OkResult {
}

struct Vec2r {
    x : f64,
    y : f64,
}

impl fmt::Display for Vec2r {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn write_pattern_results
(
    patterns : &Vec<Vec<Vec2r>>,
    im_w : u32, 
    im_h : u32, 
    path : &str
) -> Result<OkResult, String> {

    let f = File::create(path);

    match f {
        Err(a) =>  {
            return Err(format!("{}", a));
        }
        Ok(mut f) => 
        {
            let msg = format!("{} {} \n", im_w, im_h);
            let result = f.write_all(msg.as_bytes());
            if result.is_err()
            {
                return Err(format!("{}", result.unwrap_err()));
            }
            for x in patterns {
                for y in x {
                    let result = f.write_all(format!("{}", y).as_bytes());
                    if result.is_err()
                    {
                        return Err(format!("{}", result.unwrap_err()));
                    }
                }
                let result = f.write_all("\n".as_bytes());
                if result.is_err()
                {
                    return Err(format!("{}", result.unwrap_err()));
                }
            }
        }
    }

    Ok(OkResult {})
}

struct PatternReadResult {
    patterns : Vec<Vec<Vec2r>>,
    im_w : u32,
    im_h : u32,
}

fn read_pattern_results(path : & str) -> Result<PatternReadResult, String> {
	let mut patterns : Vec<Vec<Vec2r>> = vec![];
    let im_w : u32;
    let im_h : u32;

    let f_ = File::open(path);

    if f_.is_err() {
        return Err("Cannot open file".to_owned());
    }

    let f = f_.unwrap();
    let file = BufReader::new(&f);
    let mut lines = file.lines();
    let line = lines.next();
    if line.is_some() {
        let line_ = line.unwrap();
        if line_.is_ok() {
            let line = line_.unwrap();
            let l : Vec<&str> = line.split(' ').collect();
            if l.len() == 2 {
                let w = l[0].parse::<u32>();
                let h = l[1].parse::<u32>();
                if w.is_ok() && h.is_ok() {
                    im_w = w.unwrap();
                    im_h = h.unwrap();
                } else {
                    return Err("Width and height cannot be parsed".to_owned());
                }
            } else {
                return Err("First line has not a single space".to_owned());
            }
        } else {
            return Err("Cannot read line".to_owned());
        }
    } else {
        return Err("There is no line in file".to_owned());
    }

    for line_ in lines {
        if line_.is_ok() {
            let line = line_.unwrap();
            let l : Vec<&str> = line.split(',').collect();
            let mut p : Vec<Vec2r> = vec!();
            for v in l {
                let w : Vec<&str>  = v.split(' ').collect();
                if w.len() == 2 {
                    let x = w[0].parse::<f64>();
                    let y = w[1].parse::<f64>();
                    if x.is_ok() && y.is_ok() {
                        let vp = Vec2r {
                            x : x.unwrap(),
                            y : y.unwrap()
                        };
                        p.push(vp);
                    }
                }
            }
            patterns.push(p);
        }
    }

    Ok (PatternReadResult {
        patterns : patterns,
        im_w : im_w,
        im_h : im_h
    })

}

// fn create_hog_image(input: &Path, signed: bool) {

//     // Load a image::DynamicImage and convert it to a image::GrayImage
//     let image = open(input)
//         .expect(&format!("Could not load image at {:?}", input))
//         .to_luma();

//     // We're not going to do anything interesting with the block sizes here - they're
//     // only relevant when combining and normalising per-cell histograms, and in this
//     // example we're just going to compute and visualise the histograms. However, to
//     // create a HogSpec from these HogOptions we still require that the image is evenly
//     // divisible into blocks.
//     let opts = HogOptions {
//         orientations: 8,
//         signed: signed,
//         cell_side: 5,
//         block_side: 2,
//         block_stride: 1
//     };

//     let (width, height) = image.dimensions();
//     assert!(width >= 10 && height >= 10, "input file must have width and height both >= 10");

//     // Crop image to a suitable size
//     let (cropped_width, cropped_height) = (10 * (width / 10), 10 * (height / 10));
//     let mut cropped = ImageBuffer::new(cropped_width, cropped_height);
//     for y in 0..cropped_height {
//         for x in 0..cropped_width {
//             cropped.put_pixel(x, y, *image.get_pixel(x, y));
//         }
//     }

//     cropped.save(input.with_file_name("cropped.png")).unwrap();

//     let spec = HogSpec::from_options(cropped_width, cropped_height, opts).unwrap();
//     let mut hist = cell_histograms(&cropped, spec);

//     let star_side = 20;
//     let hog = render_hist_grid(star_side, &hist.view_mut(), signed);

//     let output_name = if signed { "hog_signed.png" } else { "hog_unsigned.png" };
//     hog.save(input.with_file_name(output_name)).unwrap();
// }

// fn main() {

//     let arg = if env::args().count() == 2 {
//             env::args().nth(1).unwrap()
//         } else {
//             panic!("Please enter an input file")
//         };
//     let path = Path::new(&arg);

//     create_hog_image(path, true);
//     create_hog_image(path, false);
// }