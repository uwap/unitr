use meta::HasValue;

#[stable]
pub trait Time<T> : HasValue<T> {
  fn ms (&self)   -> Times<T>;
  fn s  (&self)   -> Times<T>;
  fn min(&self)   -> Times<T>;
  fn h  (&self)   -> Times<T>;
}

#[deriving(Show, PartialEq, PartialOrd)]
pub enum Times<T> {
  Millisecond(T),
  Second(T),
  Minute(T),
  Hour(T)
}

macro_rules! impl_time_for_primitives(
  ($($t:ty),+) => (
    $(
      impl Time<$t> for $t {
        fn ms(&self) -> Times<$t> {
          Millisecond(*self)
        }
        fn s(&self) -> Times<$t> {
          Second(*self)
        }
        fn min(&self) -> Times<$t> {
          Minute(*self)
        }
        fn h(&self) -> Times<$t> {
          Hour(*self)
        }
      }
      impl Time<$t> for Times<$t> {
        fn ms(&self) -> Times<$t> {
          Millisecond(match *self {
            Millisecond(v) => v,
            Second(v) => (v as f64 * 1000f64) as $t,
            Minute(v) => (v as f64 * 60000f64) as $t,
            Hour(v) => (v as f64 * 3600000f64) as $t,
          })
        }
        fn s(&self) -> Times<$t> {
          Second((self.ms().val() as f64 / 1000f64) as $t)
        }
        fn min(&self) -> Times<$t> {
          Minute((self.ms().val() as f64 / 60000f64) as $t)
        }
        fn h(&self) -> Times<$t> {
          Hour((self.ms().val() as f64 / 3600000f64) as $t)
        }
      }
      impl HasValue<$t> for Times<$t> {
        fn val(&self) -> $t {
          match *self {
            Millisecond(v) => v,
            Second(v) => v,
            Minute(v) => v,
            Hour(v) => v,
          }
        }
      }
    )+
  )
)

for_types!(impl_time_for_primitives)
