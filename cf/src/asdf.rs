#[derive(Debug)]
struct Asdf<'a> {
  a: &'a mut i32,
}

fn modify(ruct: &Asdf) {
  // *ruct.a = 7;
}

fn main() {
  let mut b = 6;
  let ruct = &Asdf{ a: &mut b };
  let buct = Asdf{ a: &mut b };
  let cuct = Asdf{ a: &mut b };
  let duct = Asdf{ a: &mut b };
  let c = &mut b;
  println!("{}", c);
  println!("{:?}", ruct);
  println!("{:?}", buct);
  println!("{:?}", cuct);
  println!("{:?}", duct);
}