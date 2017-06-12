#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate image;
extern crate imageproc;
extern crate nalgebra;
extern crate alga;

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