// Function that 
use std::io;


fn get_int(buf: &mut String) -> i32 {
  io::stdin().read_line(buf).ok();
  let res = buf.trim().parse().unwrap();
  buf.clear();
  res
}

fn get_ints(buf: &mut String) -> Vec<i32> {
  io::stdin().read_line(buf).ok();
  let res = buf.trim().split(" ").map(|x| {x.parse::<i32>().unwrap()}).collect();
  buf.clear();
  res
}

fn main() {
  let mut buf = String::new();
  let n = get_int(&mut buf);

  'reset: for _ in 0..n {
    let m = get_int(&mut buf);
    let nums = get_ints(&mut buf);
    let total = nums.iter().sum::<i32>();
    let avg = total / m;
    
    if avg * m != total {
      println!("NO");
      continue
    }

    for x in nums {
      if x == avg {
        println!("YES");
        continue 'reset
      }
    }
    println!("NO");
  }
}
