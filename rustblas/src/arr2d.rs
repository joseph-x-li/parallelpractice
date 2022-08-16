use crate::arr1d::ArrView1D;
use crate::asserts::Assert;
use num::{Num, Zero};
use std::{
  fmt::Display,
  ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Array2D<T, const D1: usize, const D0: usize>
where
  T: Display + Copy + Num,
{
  buf: Box<[T]>,
  strides: (usize, usize), // not in bytes, but in indicies
}

impl<T, const D1: usize, const D0: usize> Array2D<T, D1, D0>
where
  T: Display + Copy + Num,
{
  pub fn zeros() -> Self {
    Array2D {
      // buf: unsafe { &mut *(alloc_zeroed(Layout::array::<T>(D0 * D1)?) as *mut T) },
      buf: vec![Zero::zero(); D1 * D0].into_boxed_slice(),
      strides: (1, D0), // Stride across D0, Stride across D1; Tuple stored in reverse
    }
  }

  pub fn new_with<F: Fn(usize) -> T>(f: F) -> Self {
    Array2D {
      buf: (0..(D1 * D0))
        .map(|idx| f(idx))
        .collect::<Vec<T>>()
        .into_boxed_slice(),
      strides: (1, D0), // Stride across D0, Stride across D1; Tuple stored in reverse
    }
  }

  // Transpose
  pub fn transpose(self) -> Array2D<T, D0, D1> {
    Array2D {
      buf: self.buf,
      strides: (self.strides.1, self.strides.0),
    }
  }

  /// Shape = (D1, D0), take a slice of shape (1, D0).
  /// returned view has shape (D0,)
  pub fn row<'a>(&'a self, idx: usize) -> ArrView1D<'a, T, D0> {
    assert!(idx < D1);
    ArrView1D {
      buf: &self.buf,
      offset: self.strides.1 * idx,
      strides: self.strides.0,
    }
  }

  /// Shape = (D1, D0), take a slice of shape (D1, 1).
  /// returned view has shape (D1,)
  pub fn col<'a>(&'a self, idx: usize) -> ArrView1D<'a, T, D1> {
    assert!(idx < D0);
    ArrView1D {
      buf: &self.buf,
      offset: self.strides.0 * idx,
      strides: self.strides.1,
    }
  }
  /// Shape = (D1, D0), take a slice of shape (D1, 1).
  /// returned view has shape (D1,)
  // #[guard(C < D0)];
  pub fn col_safe<'a, const C: usize>(&'a self) -> ArrView1D<'a, T, D1> {
    ArrView1D {
      buf: &self.buf,
      offset: self.strides.0 * C,
      strides: self.strides.1,
    }
  }

  pub fn as_view<'a>(&'a self) -> ArrView2D<'a, T, D1, D0> {
    ArrView2D {
      buf: &self.buf,
      offset: (0, 0),
      strides: self.strides,
    }
  }
}

// #[guard(D1 * D0 == N1 * N0)]
// pub fn reshape2D<T, const D1: usize, const D0: usize, const N1: usize, const N0: usize>(
//   arr: Array2D<T, D1, D0>,
// ) -> Array2D<T, N1, N0>
// where
//   T: Display + Copy,
// {
//   // Move pointer if in row major
//   let out = if arr.strides.1 > arr.strides.0 {
//     std::mem::forget(arr);
//     Array2D {
//       buf: arr.buf,
//       strides: (1, N0),
//     }
//   } else {
//     // Perform memcopy
//   };
//   out
// }

// impl<T, const D1: usize, const D0: usize> Iterator for Array2D<T, D1, D0> {
//   Item = T;

// }

impl<T, const D1: usize, const D0: usize> Index<(usize, usize)> for Array2D<T, D1, D0>
where
  T: Display + Copy + Num,
{
  type Output = T;
  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let total_offset = self.strides.0 * index.1 + self.strides.1 * index.0;
    &self.buf[total_offset]
  }
}

impl<T, const D1: usize, const D0: usize> IndexMut<(usize, usize)> for Array2D<T, D1, D0>
where
  T: Display + Copy + Num,
{
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let total_offset = self.strides.0 * index.1 + self.strides.1 * index.0;
    &mut self.buf[total_offset]
  }
}

impl<T, const D1: usize, const D0: usize> Display for Array2D<T, D1, D0>
where
  T: Display + Copy + Num,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for idx1 in 0..D1 {
      if idx1 == 0 {
        write!(f, "[")?;
      } else {
        write!(f, " [")?;
      }
      for idx0 in 0..D0 {
        let bufidx = idx1 * self.strides.1 + idx0 * self.strides.0;
        let item = self.buf[bufidx];
        write!(f, "{}", item)?;
        if idx0 < D0 - 1 {
          write!(f, ", ")?
        }
      }
      if idx1 < D1 - 1 {
        writeln!(f, "],")?
      } else {
        write!(f, "]")?
      }
    }
    write!(f, "]")
  }
}

/// Desired functionality
///
#[derive(Debug)]
pub struct ArrView2D<'a, T, const D1: usize, const D0: usize>
where
  T: Display + Copy + Num,
{
  pub(crate) buf: &'a Box<[T]>,
  pub(crate) offset: (usize, usize), // Skip these many elements
  pub(crate) strides: (usize, usize),
}

impl<T, const D1: usize, const D0: usize> Index<(usize, usize)> for ArrView2D<'_, T, D1, D0>
where
  T: Display + Copy + Num,
{
  type Output = T;
  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let total_offset =
      (self.offset.0 + index.1) * self.strides.0 + (self.offset.1 + index.0) * self.strides.1;
    &self.buf[total_offset]
  }
}

pub fn matmul<T, const D1: usize, const D0: usize, const E1: usize, const E0: usize>(
  a: ArrView2D<T, D1, D0>,
  b: ArrView2D<T, E1, E0>,
) -> Array2D<T, D1, E0>
where
  T: Display + Copy + Num,
{
  Assert::<D0, E1>::EQ;
  let mut out = Array2D::<T, D1, E0>::zeros();
  for r in 0..D1 {
    for c in 0..E0 {
      let mut total: T = Zero::zero();
      for i in 0..D0 {
        total = total + (a[(r, i)] * b[(i, c)]);
      }
      out[(r, c)] = total;
    }
  }
  out
}
