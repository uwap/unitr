#![feature(macro_rules)]

trait Distance<T> {
  fn mm(&self)  -> Millimeter <T>;
  fn cm(&self)  -> Centimeter <T>;
  fn dm(&self)  -> Decimeter  <T>;
  fn m(&self)   -> Meter      <T>;
  fn km(&self)  -> Kilometer  <T>;
  fn val(&self) -> T;
}

macro_rules! gen_unit_struct(
  ($($T:ident),+) => (
    $(
      #[deriving(Show, PartialEq, PartialOrd)]
      struct $T<T>(T);
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
      fn val(&self) -> $t {
        match *self {
          $T(v) => v
        }
      }
    }
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
        fn val(&self) -> $t {
          *self
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

gen_unit_struct!(Millimeter, Centimeter, Decimeter, Meter, Kilometer)

// we are not going to implement units < 32 bit, because of possible conversion failures e.g. 1u8.km().mm() == 256u8
impl_distance_for_primitives!(i32, i64)
impl_distance_for_primitives!(u32, u64)
impl_distance_for_primitives!(f32, f64, int, uint)

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
