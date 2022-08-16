pub struct Assert<const L: usize, const R: usize>;
impl<const L: usize, const R: usize> Assert<L, R> {
  pub const GREATER_EQ: usize = L - R;
  pub const LESS_EQ: usize = R - L;
  pub const NOT_EQ: isize = 0 / (R as isize - L as isize);
  pub const EQ: usize = (R - L) + (L - R);
  pub const GREATER: usize = L - R - 1;
  pub const LESS: usize = R - L - 1;
}
