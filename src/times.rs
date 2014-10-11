use std::{fmt, mem};
use std::fmt::Show;
use std::num::{Zero};
use meta::{HasValue, Unit};
generate_unit!{
  enum Times<i64> {
    Nanosecond  = 1,
    Millisecond = 1000000,
    Second      = 1000000000,
    Minute      = 60000000000,
    Hour        = 3600000000000
  }
  trait Time {
    fn ns(self);
    fn ms(self);
    fn s(self);
    fn minute(self);
    fn h(self);
  }
  struct TimeStruct;
  match symbol {
    Nanosecond  => "ns",
    Millisecond => "ms",
    Second      => "s",
    Minute      => "min",
    Hour        => "h"
  }
}
