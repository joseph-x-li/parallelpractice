struct NDArray<T> {
  data: Vec<T>, // Vec stores length data
  shape: (usize, usize, usize, usize),
}

impl<T> NDArray<T> {
  fn ndim(&self) -> usize {
    
  }

}
