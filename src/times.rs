use meta::HasValue;

#[stable]
pub trait Time<T> : HasValue<T> {
  fn ms (&self)   -> Times<T>;
  fn s  (&self)   -> Times<T>;
  fn minute(&self)   -> Times<T>;
  fn h  (&self)   -> Times<T>;
}

#[deriving(Show, PartialEq, PartialOrd, Clone)]
pub enum Times<T> {
  Millisecond(T),
  Second(T),
  Minute(T),
  Hour(T)
}

macro_rules! time_overload_operator(
  ($op:ident, $_self:ident, $other:ident) => (
    match *$_self {
      Millisecond(_) => Millisecond($_self.val().$op(&$other.ms().val())),
      Second(_) => Millisecond($_self.ms().val().$op(&$other.ms().val())).s(),
      Minute(_) => Millisecond($_self.ms().val().$op(&$other.ms().val())).minute(),
      Hour(_) => Millisecond($_self.ms().val().$op(&$other.ms().val())).h(),
    }
  )
)

macro_rules! impl_time_for_primitives(
  ($($t:ty),+) => (
    $(
      // impl primitive
      impl Time<$t> for $t {
        fn ms(&self) -> Times<$t> {
          Millisecond(*self)
        }
        fn s(&self) -> Times<$t> {
          Second(*self)
        }
        fn minute(&self) -> Times<$t> {
          Minute(*self)
        }
        fn h(&self) -> Times<$t> {
          Hour(*self)
        }
      }
      // impl enum
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
        fn minute(&self) -> Times<$t> {
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
      // operator overloading
      impl Add<Times<$t>, Times<$t>> for Times<$t> {
        fn add(&self, other: &Times<$t>) -> Times<$t> {
          time_overload_operator!(add, self, other)
        }
      }
      impl Sub<Times<$t>, Times<$t>> for Times<$t> {
        fn sub(&self, other: &Times<$t>) -> Times<$t> {
          time_overload_operator!(sub, self, other)
        }
      }
    )+
  )
)

for_types!(impl_time_for_primitives)
