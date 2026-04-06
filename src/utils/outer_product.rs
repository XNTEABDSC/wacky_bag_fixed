use std::{array, ops::{Add, Mul, Sub}};

use nalgebra::{ArrayStorage, Matrix, SMatrix, SVector, Scalar};
use num_traits::Zero;




pub fn outer_product<Num,const R: usize,const C: usize>(a: &SVector<Num, R>, b: &SVector<Num, C>) -> SMatrix<<Num::Output as Sub>::Output, R, C> 
	where Num:Mul<Output : Sub<Output :Scalar>>+Scalar
{
	let m = SMatrix::from_fn(|i,j|a[i].clone() * b[j].clone() - a[j].clone() * b[i].clone());
    m
}