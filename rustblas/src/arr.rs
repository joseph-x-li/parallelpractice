use rand::distributions::{Distribution, Standard};

struct NDArray<T> {
  data: Vec<T>, // Vec stores length data
  shape: (usize, usize), // must be 2D for now
  ndim: usize,
  strides: (usize, usize), // not in bytes, but in indicies
}

impl<T> NDArray<T> {
  fn new() -> Self {
    NDArray {
      data: Vec::new(),
      shape: (0, 0),
      ndim: 2,
      strides: (0, 0),
    }
  }

  fn random(shape: (usize, usize)) -> Self {
    use rand;
    let empty_data = Vec::new();
    let rng = rand::thread_rng::<Distribution<T>>();
    let data = empty_data.resize_with(
      shape.0 * shape.1,
      || rand::random::<T>())
    ;
    let mut arr = NDArray {
      data: data,
      shape: shape,
      ndim: 2,
      strides: (1, shape.0),
    };

    arr.

    arr
  }

  fn transpose(&self) {
    self.strides = (self.strides.1, self.strides.0);
    self.shape = (self.shape.1, self.shape.0);
  }

}
