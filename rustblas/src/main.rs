use anyhow::Result;
use rustblas::arr2d::{self, Array2D};

fn main() -> Result<()> {
  let x: Array2D<i32, 3, 4> = Array2D::new_with(|x| x as i32);
  // let xt = Array2D::<i32, 4, 3>::new_with(|x| x as i32).transpose();
  let xxt = arr2d::matmul(x.as_view(), x.as_view());
  println!("{}", xxt);
  Ok(())
}
