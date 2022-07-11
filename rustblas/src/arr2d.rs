use num::{self, Num};
// use static_assertions::const_assert_eq;
use const_guards::guard;
use std::{
  alloc::{alloc, alloc_zeroed, dealloc, Layout, LayoutError},
  fmt::Display,
  mem::ManuallyDrop,
  ops::Index,
};

#[derive(Debug)]
pub struct Array2D<'a, T, const D1: usize, const D0: usize>
where
  T: Num + Display + Copy,
{
  buf: &'a mut T,
  strides: (usize, usize), // not in bytes, but in indicies
}

// #[guard(D1 * D0 > 0)]
impl<'a, T, const D1: usize, const D0: usize> Array2D<'a, T, D1, D0>
where
  T: Num + Display + Copy,
{
  pub fn zeros() -> Result<Self, LayoutError> {
    Ok(Array2D {
      buf: unsafe { &mut *(alloc_zeroed(Layout::array::<i32>(D0 * D1)?) as *mut T) },
      strides: (1, D0), // Stride across D0, Stride across D1; Tuple stored in reverse
    })
  }

  pub fn new_with<F: Fn(usize) -> T>(f: F) -> Result<Self, LayoutError> {
    let out = Array2D {
      buf: unsafe { &mut *(alloc(Layout::array::<i32>(D0 * D1)?) as *mut T) },
      strides: (1, D0), // Stride across D0, Stride across D1; Tuple stored in reverse
    };
    for idx in 0..(D1 * D0) {
      unsafe {
        let item = (out.buf as *mut T).add(idx);
        *item = f(idx);
      };
    }
    Ok(out)
  }

  // Transpose
  pub fn transpose(self) -> Array2D<'a, T, D0, D1> {
    let mut arr = ManuallyDrop::new(self);
    let out = Array2D {
      buf: arr.buf,
      strides: (arr.strides.1, arr.strides.0),
    };
    std::mem::forget(self);
    out
  }
}

#[guard(D1 * D0 == N1 * N0)]
pub fn reshape<T, const D1: usize, const D0: usize, const N1: usize, const N0: usize>(
  arr: Array2D<T, D1, D0>,
) -> Array2D<T, N1, N0>
where
  T: Num + Display + Copy,
{
  // Move pointer if in row major
  let out = if arr.strides.1 > arr.strides.0 {
    std::mem::forget(arr);
    Array2D {
      buf: arr.buf,
      strides: (1, N0),
    }
  } else {
    // Perform memcopy
  };
  out
}

// impl<T, const D1: usize, const D0: usize> Iterator for Array2D<T, D1, D0> {
//   Item = T;

// }

impl<T, const D1: usize, const D0: usize> Display for Array2D<'_, T, D1, D0>
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

impl<T, const D1: usize, const D0: usize> Drop for Array2D<'_, T, D1, D0>
where
  T: Num + Display + Copy,
{
  fn drop(&mut self) {
    unsafe {
      dealloc(self.buf as *mut u8, Layout::array::<i32>(D0 * D1).unwrap());
    }
  }
}
