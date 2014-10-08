use meta::{HasValue, Unit};
use std::{fmt, mem};
use std::fmt::Show;
use std::num::{Zero};

#[stable]
pub trait Time<T> : HasValue<T> {
  fn ns(self)     -> TimeStruct<T>;
  fn ms(self)     -> TimeStruct<T>;
  fn s(self)      -> TimeStruct<T>;
  fn minute(self) -> TimeStruct<T>;
  fn h(self)      -> TimeStruct<T>;
}

#[deriving(Show, PartialEq, PartialOrd, Clone)]
enum Times {
  Nanosecond  = 1,
  Millisecond = 1000000,
  Second      = 1000000000,
  Minute      = 60000000000,
  Hour        = 3600000000000
}
impl Unit for Times {
  fn symbol(&self) -> &str {
    match *self {
      Nanosecond  => "ns",
      Millisecond => "ms",
      Second      => "s",
      Minute      => "min",
      Hour        => "h"
    }
  }
}
impl <T> HasValue<T> for Times where T: Primitive {
  #[inline]
  fn val(self) -> T {
    NumCast::from::<i64>(unsafe { mem::transmute(self) }).unwrap()
  }
}
// FIXME: Reduce redundandency with Distances::fac
impl Times {
  #[inline]
  fn fac<T>(from: T, to: T) -> f64 where T: HasValue<f64> {
    from.val() / to.val()
  }
}

#[deriving(Clone)]
pub struct TimeStruct<T> {
  _ty: Times,
  _val: T
}
impl <T> TimeStruct<T> where T: Primitive {
  #[inline]
  fn new(ty: Times, val: T) -> TimeStruct<T> {
    TimeStruct { _ty: ty, _val: val }
  }
  fn convert<U>(self, to: Times) -> TimeStruct<U> where U: Primitive {
    let self_val_f64 : f64 = NumCast::from(self._val).unwrap();
    TimeStruct::new(to,
      NumCast::from::<f64>(
        self_val_f64 * Times::fac(self._ty, to)).unwrap())
  }
}
impl <T> HasValue<T> for TimeStruct<T> {
  #[inline]
  fn val(self) -> T {
    self._val
  }
}

macro_rules! impl_time_for_primitives(($($T:ty),+) => ($(
impl Time<$T> for $T {
  #[inline]
  fn ns(self) -> TimeStruct<$T> {
    TimeStruct::new(Nanosecond, self)
  }
  #[inline]
  fn ms(self) -> TimeStruct<$T> {
    TimeStruct::new(Millisecond, self)
  }
  #[inline]
  fn s(self) -> TimeStruct<$T> {
    TimeStruct::new(Second, self)
  }
  #[inline]
  fn minute(self) -> TimeStruct<$T> {
    TimeStruct::new(Minute, self)
  }
  #[inline]
  fn h(self) -> TimeStruct<$T> {
    TimeStruct::new(Hour, self)
  }
}

impl Time<$T> for TimeStruct<$T> {
  #[inline]
  fn ns(self) -> TimeStruct<$T> {
    self.convert(Nanosecond)
  }
  #[inline]
  fn ms(self) -> TimeStruct<$T> {
    self.convert(Millisecond)
  }
  #[inline]
  fn s(self) -> TimeStruct<$T> {
    self.convert(Second)
  }
  #[inline]
  fn minute(self) -> TimeStruct<$T> {
    self.convert(Minute)
  }
  #[inline]
  fn h(self) -> TimeStruct<$T> {
    self.convert(Hour)
  }
}

impl ToPrimitive for TimeStruct<$T> {
  #[inline]
  fn to_i64(&self) -> Option<i64> {
    NumCast::from(self.val())
  }
  #[inline]
  fn to_u64(&self) -> Option<u64> {
    NumCast::from(self.val())
  }
}

impl Unit for TimeStruct<$T> {
  #[inline]
  fn symbol(&self) -> &str {
    self._ty.symbol()
  }
}

// Implement number traits
impl Zero for TimeStruct<$T> {
  fn zero() -> TimeStruct<$T> {
    TimeStruct::new(Second, Zero::zero())
  }
  fn is_zero(&self) -> bool {
    self.val().is_zero()
  }
}
impl Add<TimeStruct<$T>, TimeStruct<$T>> for TimeStruct<$T> {
  fn add(&self, rhs: &TimeStruct<$T>) -> TimeStruct<$T> {
    TimeStruct::new(self._ty, self.val() + rhs.convert::<$T>(self._ty).val())
  }
}
impl Sub<TimeStruct<$T>, TimeStruct<$T>> for TimeStruct<$T> {
  fn sub(&self, rhs: &TimeStruct<$T>) -> TimeStruct<$T> {
    TimeStruct::new(self._ty, self.val() - rhs.convert::<$T>(self._ty).val())
  }
}
//impl Mul<DistanceStruct<$T>, Surface<$T>>

impl Show for TimeStruct<$T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.val(), self.symbol())
  }
}
impl PartialEq for TimeStruct<$T> {
  fn eq(&self, other: &TimeStruct<$T>) -> bool {
    self.val() == other.convert::<$T>(self._ty).val()
  }
}
impl PartialOrd for TimeStruct<$T> {
  fn partial_cmp(&self, other: &TimeStruct<$T>) -> Option<Ordering> {
    self.val().partial_cmp(&other.convert::<$T>(self._ty).val())
  }
}
)+)) for_primitives!(impl_time_for_primitives)
