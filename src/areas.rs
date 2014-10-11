use meta::{HasValue, Unit};
use std::{fmt, mem};
use std::fmt::Show;
use std::num::{Zero};
generate_unit!{
  enum Areas<i64> {
    SquareMillimeter  = 1,
    SquareCentimeter  = 1000000,
    SquareDecimeter   = 100000000,
    SquareMeter       = 10000000000,
    SquareKilometer   = 10000000000000000
  }
  trait Area {
    fn mm2(self);
    fn cm2(self);
    fn dm2(self);
    fn m2 (self);
    fn km2(self);
  }
  struct AreaStruct;
  match symbol {
    SquareMillimeter => "mm²",
    SquareCentimeter => "cm²",
    SquareDecimeter  => "dm²",
    SquareMeter      => "m²",
    SquareKilometer  => "km²"
  }
}
