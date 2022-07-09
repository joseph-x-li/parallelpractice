use rustblas::arr::NDArray;
fn main() {
  let mut x = NDArray::<i32>::random((5, 5));
  println!("{}", x);
  x.transpose();
  println!("{}", x);
}
