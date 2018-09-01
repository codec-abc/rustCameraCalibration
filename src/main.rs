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

type Point2D = MatrixMN<f64, U2, U1>;
type Point2DT = MatrixMN<f64, U1, U2>;

type Point3D = MatrixMN<f64, U3, U1>;
type Point3DT = MatrixMN<f64, U1, U3>;

type Point4D = MatrixMN<f64, U4, U1>;
type Point4DT = MatrixMN<f64, U1, U4>;

fn main() {
    build_normalization_matrix(640.0, 480.0);
}

struct PairPoint2D {
    image_point : Point2D,
    model_point : Point2D
}

struct PairPoint3D {
    image_point : Point3D,
    model_point : Point3D
}

struct ImageData2D {
    width : f64,
    height : f64,
    pair_points : Vec<PairPoint2D>
}

struct ImageData3D {
    width : f64,
    height : f64,
    pair_points : Vec<PairPoint3D>
}

fn matrix_index(i : u32, j : u32, rows : usize, columns : usize) -> usize {
    let i_ = i as usize;
    let j_ = j as usize;
    (j_ * rows + i_)
}

fn matrix_index2<N : Scalar, R : Dim, C : Dim, S : nalgebra::storage::Storage<N, R, C>>(i : u32, j : u32, matrix : &Matrix<N, R, C, S>) -> usize {
    let rows = matrix.nrows();
    let columns = matrix.ncols();
    let i_ = i as usize;
    let j_ = j as usize;
    (j_ * rows + i_)
}

fn build_normalization_matrix(w : f64, h : f64) -> F64Matrix3x3 {
    let mut n = Matrix::identity_generic(U3 , U3);
    let mut index = matrix_index2(0u32, 0u32, &n);
    n[index] = 2.0 / w;
    index = matrix_index2(1u32, 1u32, &n);
    n[index] = 2.0 / h;
    index = matrix_index2(0u32, 2u32, &n);
    n[index] = -1.0;
    index = matrix_index2(1u32, 2u32, &n);
    n[index] = -1.0;
    n
}

fn point_2d_to_point_3d(point : &Point2D) -> Point3D {
    Point3D::new(point[0], point[1], 0.0)
}

fn point_3d_to_point_4d(point : &Point2D) -> Point4D {
    Point4D::new(point[0], point[1], point[2], 0.0)
}

fn normalize_point(point : &Point2D , normalization_matrix : &F64Matrix3x3) -> Point3D {
    let point_3d = point_2d_to_point_3d(point);
    normalization_matrix * point_3d
}

fn normalize_data_set(data_set : & mut ImageData2D) -> ImageData3D {
    let normalization_matrix = build_normalization_matrix(data_set.width, data_set.height);
    let mut normalized_points : Vec<PairPoint3D> = vec!();
    for ref p in data_set.pair_points.iter() {
        let new_model_point = point_2d_to_point_3d(&p.model_point);
        let new_image_point = normalize_point(&p.image_point, &normalization_matrix);
        let new_pair = PairPoint3D {
            model_point : new_model_point,
            image_point : new_image_point
        };
        normalized_points.push(new_pair);
    }
    ImageData3D {
        width : data_set.width,
        height : data_set.height,
        pair_points : normalized_points
    }
}
