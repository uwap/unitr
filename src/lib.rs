#![crate_name = "unitr"]
#![crate_type="lib"]
#![feature(macro_rules)]

//! A library that implements basic units and unit conversions.
//!
//!

trait HasValue<T> {
  fn val(&self) -> T;
}

#[stable]
trait Distance<T> : HasValue<T> {
  fn mm(&self)  -> Millimeter <T>;
  fn cm(&self)  -> Centimeter <T>;
  fn dm(&self)  -> Decimeter  <T>;
  fn m(&self)   -> Meter      <T>;
  fn km(&self)  -> Kilometer  <T>;
}

#[stable]
trait Time<T> : HasValue<T> {
  fn ms(&self)  -> Millisecond <T>;
  fn s(&self)   -> Second     <T>;
  fn min(&self) -> Minute     <T>;
  fn h(&self)   -> Hour       <T>;
}

macro_rules! gen_unit_struct(
  ($($T:ident),+) => (
    $(
      #[deriving(Show, PartialEq, PartialOrd)]
      pub struct $T<T>(T);
    )+
  )
)

macro_rules! impl_distance_struct(
  ($T:ident of $t:ty * $fac:expr) => (
    impl Distance<$t> for $T<$t> {
      fn mm(&self) -> Millimeter<$t> {
        match *self {
          $T(v) => Millimeter((v as f64 * $fac) as $t)
        }
      }
      fn cm(&self) -> Centimeter<$t> {
        match *self {
          $T(v) => Centimeter((v as f64 * ($fac / 10f64)) as $t)
        }
      }
      fn dm(&self) -> Decimeter<$t> {
        match *self {
          $T(v) => Decimeter((v as f64 * ($fac / 100f64)) as $t)
        }
      }
      fn m(&self) -> Meter<$t> {
          match *self {
            $T(v) => Meter((v as f64 * ($fac / 1000f64)) as $t)
          }
      }
      fn km(&self) -> Kilometer<$t> {
        match *self {
          $T(v) => Kilometer((v as f64 * ($fac / 1000000f64)) as $t)
        }
      }
    }
    impl HasValue<$t> for $T<$t> {
      fn val(&self) -> $t {
        match *self {
          $T(v) => v
        }
      }
    }
  )
)

macro_rules! impl_primitives(
    ($($t:ty),+) => (
      $(
        impl_distance_for_primitives!($t)
        impl_time_for_primitives!($t)
        impl HasValue<$t> for $t {
          fn val(&self) -> $t {
            *self
          }
        }
      )+
    )
)

macro_rules! impl_distance_for_primitives(
  ($($t:ty),+) => (
    $(
      impl Distance<$t> for $t {
        fn mm(&self) -> Millimeter<$t> {
          Millimeter(*self)
        }
        fn cm(&self) -> Centimeter<$t> {
          Centimeter(*self)
        }
        fn dm(&self) -> Decimeter<$t> {
          Decimeter(*self)
        }
        fn m(&self) -> Meter<$t> {
          Meter(*self)
        }
        fn km(&self) -> Kilometer<$t> {
          Kilometer(*self)
        }
      }
      impl_distance_struct!(Kilometer of $t * 1000000f64)
      impl_distance_struct!(Meter of $t * 1000f64)
      impl_distance_struct!(Decimeter of $t * 100f64)
      impl_distance_struct!(Centimeter of $t * 10f64)
      impl_distance_struct!(Millimeter of $t * 1f64)
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
      impl Time<$t> for Millisecond<$t> {
        fn ms(&self) -> Millisecond<$t> {
          Millisecond(self.val())
        }
        fn s(&self) -> Second<$t> {
          Second((self.val() as f64 / 1000f64) as $t)
        }
        fn min(&self) -> Minute<$t> {
          Minute((self.val() as f64 / 60000f64) as $t)
        }
        fn h(&self) -> Hour<$t> {
          Hour((self.val() as f64 / 3600000f64) as $t)
        }
      }
      impl HasValue<$t> for Millisecond<$t> {
        fn val(&self) -> $t {
          match *self {
            Millisecond(v) => v
          }
        }
      }
      impl Time<$t> for Second<$t> {
        fn ms(&self) -> Millisecond<$t> {
          Millisecond((self.val() as f64 * 1000f64) as $t)
        }
        fn s(&self) -> Second<$t> {
          Second(self.val())
        }
        fn min(&self) -> Minute<$t> {
          Minute((self.val() as f64 / 60f64) as $t)
        }
        fn h(&self) -> Hour<$t> {
          Hour((self.val() as f64 / 3600f64) as $t)
        }
      }
      impl HasValue<$t> for Second<$t> {
        fn val(&self) -> $t {
          match *self {
            Second(v) => v
          }
        }
      }
      impl Time<$t> for Minute<$t> {
        fn ms(&self) -> Millisecond<$t> {
          Millisecond((self.val() as f64 * 60000f64) as $t)
        }
        fn s(&self) -> Second<$t> {
          Second((self.val() as f64 * 60f64) as $t)
        }
        fn min(&self) -> Minute<$t> {
          Minute(self.val())
        }
        fn h(&self) -> Hour<$t> {
          Hour((self.val() as f64 / 60f64) as $t)
        }
      }
      impl HasValue<$t> for Minute<$t> {
        fn val(&self) -> $t {
          match *self {
            Minute(v) => v
          }
        }
      }
      impl Time<$t> for Hour<$t> {
        fn ms(&self) -> Millisecond<$t> {
          Millisecond((self.val() as f64 * 3600000f64) as $t)
        }
        fn s(&self) -> Second<$t> {
          Second((self.val() as f64 * 3600f64) as $t)
        }
        fn min(&self) -> Minute<$t> {
          Minute((self.val() as f64 * 60f64) as $t)
        }
        fn h(&self) -> Hour<$t> {
          Hour(self.val())
        }
      }
      impl HasValue<$t> for Hour<$t> {
        fn val(&self) -> $t {
          match *self {
            Hour(v) => v
          }
        }
      }
    )+
  )
)

gen_unit_struct!(Millimeter, Centimeter, Decimeter, Meter, Kilometer)
gen_unit_struct!(Hour, Minute, Second, Millisecond)

// we are not going to implement units < 32 bit, because of possible conversion failures e.g. 1u8.km().mm() == 256u8
impl_primitives!(i32, i64)
impl_primitives!(u32, u64)
impl_primitives!(f32, f64, int, uint)

#[test]
fn test_distance() {
  assert_eq!(1000u32.m().val(), 1000u32)
  assert_eq!(1000u32.m().km().val(), 1u32)
  assert_eq!(1000u32.m().km(), 1u32.km())
  assert_eq!(1000u32.m().km().val(), 1u32)
  assert_eq!(1u32.m().km().val(), 100u32.cm().km().val())
  assert_eq!(1u32.km().mm().cm().m(), 1000u32.m().mm().km().dm().m())
  assert!(1u32.km() != 2u32.km())
}

#[test]
fn test_time() {
  assert_eq!(1u32.h().min(), 60u32.min())
  assert_eq!(1000u32.ms(), 1u32.s().ms())
  assert_eq!(1000f64.ms().h().val(), 1f64.s().min().h().val())
}
