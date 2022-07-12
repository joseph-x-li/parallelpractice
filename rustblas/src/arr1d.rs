use num::Num;
// use const_guards::guard;
use std::{fmt::Display, ops::Index};

pub struct ArrView1D<'a, T, const D0: usize>
where
  T: Display + Copy + Num,
{
  pub (crate) buf: &'a Box<[T]>,
  pub (crate) offset: usize, // Skip these many elements
  pub (crate) strides: usize,
}

impl<T, const D0: usize> ArrView1D<'_, T, D0> 
where
  T: Display + Copy + Num {
    pub fn col(&self, idx: usize) -> &T {
      &self.buf[self.offset + idx * self.strides]
    }
}

impl<T, const D0: usize> Index<usize> for ArrView1D<'_, T, D0>
where
  T: Display + Copy + Num,
{
  type Output = T;
  fn index(&self, idx: usize) -> &Self::Output {
    self.col(idx)
  }
}

impl<T, const D0: usize> Display for ArrView1D<'_, T, D0> 
where
  T: Display + Copy + Num,
  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "[")?;
      for idx0 in 0..D0 {
        write!(f, "{}", self.col(idx0))?;
        if idx0 < D0 - 1 {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
  }
}

struct ArrView1DIter<'a, T>
where
  T: Display + Copy,
{
  buf: &'a mut T,
  stride: usize,
  pos: usize,
  len: usize,
}

// impl<'a, T> Iterator for ArrView1DIter<'a, T> {
//   type Item = &'a mut T;

//   fn next(&mut self) -> Option<Self::Item> {
//     if (pos < len) {
//       Some()
//     }
//   }
// }
