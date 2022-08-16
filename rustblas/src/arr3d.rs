use num::{Num, Zero};
use std::fmt::Display;

#[derive(Debug)]
pub struct Array3D<T, const D2: usize, const D1: usize, const D0: usize>
where
  T: Display + Copy + Num,
{
  buf: Box<[T]>,
  strides: (usize, usize, usize),
}

impl<T, const D2: usize, const D1: usize, const D0: usize> Array3D<T, D2, D1, D0>
where
  T: Display + Copy + Num,
{
  pub fn zeros() -> Self {
    Array3D {
      buf: vec![Zero::zero(); D2 * D1 * D0].into_boxed_slice(),
      strides: (1, D0, D1),
    }
  }

  pub fn new_with<F: Fn(usize) -> T>(f: F) -> Self {
    Array3D {
      buf: (0..(D2 * D1 * D0))
        .map(|idx| f(idx))
        .collect::<Vec<T>>()
        .into_boxed_slice(),
      strides: (1, D0, D1),
    }
  }
}
