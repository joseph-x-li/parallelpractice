use anyhow::Result;
use rustblas::arr::Array2D;

fn main() -> Result<()> {
  let x: Array2D<i32, 2, 3> = Array2D::zeros()?;
  println!("{:?}", x);
  println!("{}", x);
  println!("{:?}", x);
  let x = x.transpose();
  println!("{:?}", x);
  println!("{}", x);
  Ok(())
}
