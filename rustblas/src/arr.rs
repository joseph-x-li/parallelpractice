// use rand::distributions::{Distribution, Standard};
use num::{self, Num};
use std::{
  alloc::{alloc_zeroed, dealloc, Layout, LayoutError},
  fmt::Display,
};

// pub struct Array1D<T, const D0: usize> {
//   data: *mut T,
//   strides: (usize,), // not in bytes, but in indicies
// }

#[derive(Debug)]
pub struct Array2D<T, const D1: usize, const D0: usize>
where
T: Num + Display + Copy,
{
  buf: *mut T,
  strides: (usize, usize), // not in bytes, but in indicies
}

impl<T, const D1: usize, const D0: usize> Array2D<T, D1, D0>
where
  T: Num + Display + Copy,
{
  pub fn zeros() -> Result<Self, LayoutError> {
    Ok(Array2D {
      buf: unsafe { alloc_zeroed(Layout::array::<i32>(D0 * D1)?) as *mut T },
      strides: (D1, 1),
    })
  }

  // Transpose
  pub fn transpose(self) -> Array2D<T, D0, D1> {
    let out = Array2D {
      buf: self.buf,
      strides: (self.strides.1, self.strides.0),
    };
    std::mem::forget(self);
    out
  }
}

impl<T, const D1: usize, const D0: usize> Display for Array2D<T, D1, D0>
where
  T: Num + Display + Copy,
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
        let item = unsafe { *self.buf.add(bufidx) };
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

impl<T, const D1: usize, const D0: usize> Drop for Array2D<T, D1, D0>
where
  T: Num + Display + Copy,
{
  fn drop(&mut self) {
    unsafe {
      dealloc(self.buf as *mut u8, Layout::array::<i32>(D0 * D1).unwrap());
    }
  }
}
