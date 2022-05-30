use std::io;

fn get_int(buf: &mut String) -> i32 {
  io::stdin().read_line(buf).ok();
  let res = buf.trim().parse().unwrap();
  buf.clear();
  res
}

fn main() {
  let mut buf = String::new();
  let n = get_int(&mut buf);
  for _ in 0..n {
    buf.clear();
    io::stdin().read_line(&mut buf).ok();
    let tmp = buf.trim();
    let len = tmp.len();
    if len > 10 {
      let mut tmpiter = tmp.chars();
      let first_letter = tmpiter.next().unwrap();
      let last_letter = tmpiter.last().unwrap();
      println!("{}{}{}", first_letter, len - 2, last_letter);
    } else {
      println!("{}", tmp);
    }
  }
}