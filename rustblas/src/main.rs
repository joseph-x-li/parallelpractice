use anyhow::Result;
use rustblas::arr2d::{self, Array2D};

fn main() -> Result<()> {
  let x: Array2D<i32, 4, 3> = Array2D::new_with(|x| x as i32);
  println!("{}", x);
  let x = x.transpose();
  println!("{}", x);
  // let y = arr2d::reshape::<i32, 3, 2, 6, 1>(x);
  let y = x.col(2);
  let z = x.row(1);
  println!("{}", z);
  println!("{}", y);
  Ok(())
}
