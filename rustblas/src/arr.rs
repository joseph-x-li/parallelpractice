use rand::distributions::{Distribution, Standard};
use std::fmt::{self, Debug, Display};

pub struct NDArray<T> {
  data: Vec<T>,          // Vec stores length data
  shape: (usize, usize), // must be 2D for now
  ndim: usize,
  strides: (usize, usize), // not in bytes, but in indicies
}

impl<T> NDArray<T>
where
  Standard: Distribution<T>,
{
  pub fn new() -> Self {
    NDArray {
      data: Vec::new(),
      shape: (0, 0),
      ndim: 2,
      strides: (0, 0),
    }
  }

  pub fn random(shape: (usize, usize)) -> Self {
    let mut empty_data = Vec::new();
    empty_data.resize_with(shape.0 * shape.1, || rand::random::<T>());

    NDArray {
      data: empty_data,
      shape,
      ndim: 2,
      strides: (1, shape.0),
    }
  }

  pub fn transpose(&mut self) {
    self.strides = (self.strides.1, self.strides.0);
    self.shape = (self.shape.1, self.shape.0);
  }
}

impl<T> Iter for NDArray<T{
  type Item = T;

  fn next(&self) -> Option<Self::Item> {
    self.data.pop()
  }
}

impl<T> Display for NDArray<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[")?;
    let (st0, st1) = self.strides;
    let (rows, cols) = self.shape;
    for r in 0..rows {
      for c in 0..cols {
        write!(f, "{:?} ", self.data[r * st0 + c * st1])?;
      }
      if r 
      write!(f, "\n")?;
    }
    write!(f, "]")
  }
}
