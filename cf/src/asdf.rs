#[derive(Debug)]
struct Asdf {
  a: *mut i32,
  // a: &'a mut [i32],
}

impl Asdf {
  fn getbsdf<'a>(&'a self) -> Bsdf<'a> {
    Bsdf { a: &self.a }
  }
}

#[derive(Debug)]
struct Bsdf<'a> {
  a: &'a *mut i32,
  // a: &'a mut [i32],
}

fn modify<'a>() -> () {
  let a = Asdf { a: 8 as *mut i32 };
  let b = a.getbsdf();
  // println!("{:?}", ptr);
  // println!("{:?}", asdf);
  ()
}

fn main() {
  modify();
}
