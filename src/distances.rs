use areas::AreaStruct;
use std::{fmt, mem};
use std::fmt::Show;
use std::num::{Zero};
use meta::{HasValue, Unit};
generate_unit!{
  enum Distances<i32> {
    Millimeter  = 1,
    Centimeter  = 1000,
    Decimeter   = 10000,
    Meter       = 100000,
    Kilometer   = 100000000
  }
  trait Distance {
    fn mm(self);
    fn cm(self);
    fn dm(self);
    fn m (self);
    fn km(self);
  }
  struct DistanceStruct;
  match symbol {
    Millimeter => "mm",
    Centimeter => "cm",
    Decimeter  => "dm",
    Meter      => "m",
    Kilometer  => "km"
  }
}
macro_rules! impl_distance_for_primitives(($($T:ident),+) => ($(
impl Mul<DistanceStruct<$T>, AreaStruct<$T>> for DistanceStruct<$T> {
  fn mul(&self, rhs: &DistanceStruct<$T>) -> AreaStruct<$T> {
    let ty : i64 = NumCast::from::<i32>(self._ty.val()).unwrap();
    AreaStruct::new(
      unsafe { mem::transmute(ty*ty) },
      self.val() * rhs.convert::<$T>(self._ty).val()
    )
  }
})+)) for_primitives!(impl_distance_for_primitives)
