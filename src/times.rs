use meta::HasValue;

#[stable]
pub trait Time<T> : HasValue<T> {
  fn ms (&self)   -> Millisecond  <T>;
  fn s  (&self)   -> Second       <T>;
  fn min(&self)   -> Minute       <T>;
  fn h  (&self)   -> Hour         <T>;
}

macro_rules! gen_time_structs(
  ($($id:ident $t:ty $fac:expr),+) => (
    $(
      impl Time<$t> for $id<$t> {
        fn ms(&self) -> Millisecond<$t> {
          Millisecond(self.val() * $fac)
        }
        fn s(&self) -> Second<$t> {
          Second((self.ms().val() as f64 / 1000f64) as $t)
        }
        fn min(&self) -> Minute<$t> {
          Minute((self.ms().val() as f64 / 60000f64) as $t)
        }
        fn h(&self) -> Hour<$t> {
          Hour((self.ms().val() as f64 / 3600000f64) as $t)
        }
      }
      impl HasValue<$t> for $id<$t> {
        fn val(&self) -> $t {
          match *self { $id(v) => v }
        }
      }
    )+
  )
)

macro_rules! impl_time_for_primitives(
  ($($t:ty),+) => (
    $(
      impl Time<$t> for $t {
        fn ms(&self) -> Millisecond<$t> {
          Millisecond(*self)
        }
        fn s(&self) -> Second<$t> {
          Second(*self)
        }
        fn min(&self) -> Minute<$t> {
          Minute(*self)
        }
        fn h(&self) -> Hour<$t> {
          Hour(*self)
        }
      }
    )+
  )
)

for_types!(impl_time_for_primitives)
gen_unit_structs_with_args!(gen_time_structs, Millisecond 1f64, Second 1000f64, Minute 60000f64, Hour 3600000f64)
