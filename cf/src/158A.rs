use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Problem {
  n: i32,
  k: i32,
  a: Vec<i32>,
}

#[inline(always)]
fn read_inputs() -> Problem {
  let cin: io::Stdin = io::stdin();
  // use std::fs;
  // let fin: fs::File = fs::File::open("./tc.txt").unwrap();
  let mut buf = String::new();
  let mut scanner = io::BufReader::new(cin);
  scanner.read_line(&mut buf).ok();

  // n, k
  let mut nk_iter = buf.trim_end().split(" ").map(|x| x.parse::<i32>().unwrap());
  let n = nk_iter.next().unwrap();
  let k = nk_iter.next().unwrap();

  buf.clear();

  // a
  scanner.read_line(&mut buf).ok();
  let a = buf
    .trim_end()
    .split(" ")
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

  Problem { n, k, a }
}

fn main() {
  let prob = read_inputs();
  let kth_score = prob.a[(prob.k - 1) as usize];
  let ans = prob
    .a
    .iter()
    .map(|item| ((item > &0) && (item >= &kth_score)) as i32)
    .reduce(|acc, item| acc + item)
    .unwrap();
  println!("{}", ans);
}
