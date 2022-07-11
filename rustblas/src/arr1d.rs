use crate::arr2d;
use const_guards::guard;
use num::{self, Num};
use std::{fmt::Display, ops::Index};

pub struct ArrView1DMut<'a, T, const D0: usize>
where
  T: Num + Display + Copy,
{
  buf: &'a mut T,
  strides: (usize,),
}

impl<'a, T, const D0: usize> Index<usize> for ArrView1DMut<'a, T, D0> 
where
  T: Num + Display + Copy, {
  type Output = T;
  fn index(&self, idx: usize) -> &Self::Output {
    let idx = idx * self.strides.0;
    let buf: *const T = self.buf as *const T;
    unsafe { &*buf.add(idx) }
  }
}

struct ArrView1dIter<'a, T>
where
  T: Num + Display + Copy,
{
  buf: &'a mut T,
  stride: usize,
  pos: usize,
  len: usize,
}

impl<'a, T> Iterator for ArrView1dIter<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    if (pos < len) {
      Some()
    }
  }
}
