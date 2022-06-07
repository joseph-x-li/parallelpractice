use std::io;

fn read_int(buf: &mut String) -> i32 {
  io::stdin().read_line(buf).ok();
  let res = buf.trim().parse::<i32>().unwrap();
  buf.clear();
  res
}

fn read_ints(buf: &mut String) -> Vec<i32> {
  io::stdin().read_line(buf).ok();
  let res = buf.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
  buf.clear();
  res
}

fn main() {
  let mut buf = String::new();
  let t = read_int(&mut buf);
  for _ in 0..t {
    let _ = read_int(&mut buf);
    let aes = read_ints(&mut buf);
    let mut num_odd = 0;
    let mut num_even = 0;
    for a in aes.iter() {
      if a % 2 == 1 {
        num_odd += 1;
      } else {
        num_even += 1;
      }
    }
    if num_odd == 0 {
      let fastest = aes.into_iter().map(|mut a| {
        let mut place_counter = 0;
        while a % 2 == 0 {
          place_counter += 1;
          a >>= 1;
        }
        place_counter
      }).reduce(i32::min).unwrap();
      println!("{}", num_even - 1 + fastest);
    } else {
      println!("{}", num_even);
    }


  }
}