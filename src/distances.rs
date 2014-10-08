use meta::{HasValue, Unit};
use std::{fmt, mem};
use std::fmt::Show;
use std::num::{Zero};

#[unstable="Likely to change its name"]
pub trait Distance<T> : HasValue<T> + Unit {
  fn mm(self)  -> DistanceStruct<T>;
  fn cm(self)  -> DistanceStruct<T>;
  fn dm(self)  -> DistanceStruct<T>;
  fn m (self)  -> DistanceStruct<T>;
  fn km(self)  -> DistanceStruct<T>;
}

#[deriving(Show, PartialEq, PartialOrd, Clone)]
enum Distances {
  Millimeter  = 1,
  Centimeter  = 1000,
  Decimeter   = 10000,
  Meter       = 100000,
  Kilometer   = 100000000
}
impl <T> HasValue<T> for Distances where T: Primitive {
  #[inline]
  fn val(self) -> T {
    NumCast::from::<i32>(unsafe { mem::transmute(self) }).unwrap()
  }
}
impl Unit for Distances {
  fn symbol(&self) -> &str {
    match *self {
      Millimeter => "mm",
      Centimeter => "cm",
      Decimeter  => "dm",
      Meter      => "m",
      Kilometer  => "km"
    }
  }
}
impl Distances {
  #[inline]
  fn fac<T>(from: T, to: T) -> f64 where T: HasValue<f64> {
    from.val() / to.val()
  }
}

#[experimental]
#[deriving(Clone)]
pub struct DistanceStruct<T> {
  _ty: Distances,
  _val: T
}

impl <T> DistanceStruct<T> where T: Primitive {
  #[inline]
  fn new(kind: Distances, val: T) -> DistanceStruct<T> {
    DistanceStruct{_ty: kind, _val: val}
  }
  fn convert<U>(self, to: Distances) -> DistanceStruct<U> where U: Primitive {
    let self_val_f64 : f64 = NumCast::from(self._val).unwrap();
    DistanceStruct::new(to,
      NumCast::from::<f64>(
        self_val_f64 * Distances::fac(self._ty, to)).unwrap())
  }
}
impl <T> HasValue<T> for DistanceStruct<T> {
  #[inline]
  fn val(self) -> T {
    self._val
  }
}

macro_rules! impl_distance_for_primitives(($($T:ty),+) => ($(
impl Distance<$T> for $T {
  #[inline]
  fn mm(self) -> DistanceStruct<$T> {
    DistanceStruct::new(Millimeter, self)
  }
  #[inline]
  fn cm(self) -> DistanceStruct<$T> {
    DistanceStruct::new(Centimeter, self)
  }
  #[inline]
  fn dm(self) -> DistanceStruct<$T> {
    DistanceStruct::new(Decimeter, self)
  }
  #[inline]
  fn m (self) -> DistanceStruct<$T> {
    DistanceStruct::new(Meter, self)
  }
  #[inline]
  fn km(self) -> DistanceStruct<$T> {
    DistanceStruct::new(Kilometer, self)
  }
}

impl Distance<$T> for DistanceStruct<$T> {
  #[inline]
  fn mm(self) -> DistanceStruct<$T> {
    self.convert(Millimeter)
  }
  #[inline]
  fn cm(self) -> DistanceStruct<$T> {
    self.convert(Centimeter)
  }
  #[inline]
  fn dm(self) -> DistanceStruct<$T> {
    self.convert(Decimeter)
  }
  #[inline]
  fn m (self) -> DistanceStruct<$T> {
    self.convert(Meter)
  }
  #[inline]
  fn km(self) -> DistanceStruct<$T> {
    self.convert(Kilometer)
  }
}

impl ToPrimitive for DistanceStruct<$T> {
  #[inline]
  fn to_i64(&self) -> Option<i64> {
    NumCast::from(self.val())
  }
  #[inline]
  fn to_u64(&self) -> Option<u64> {
    NumCast::from(self.val())
  }
}

impl Unit for DistanceStruct<$T> {
  #[inline]
  fn symbol(&self) -> &str {
    self._ty.symbol()
  }
}

// Implement number traits
impl Zero for DistanceStruct<$T> {
  fn zero() -> DistanceStruct<$T> {
    DistanceStruct::new(Meter, Zero::zero())
  }
  fn is_zero(&self) -> bool {
    self.val().is_zero()
  }
}
impl Add<DistanceStruct<$T>, DistanceStruct<$T>> for DistanceStruct<$T> {
  fn add(&self, rhs: &DistanceStruct<$T>) -> DistanceStruct<$T> {
    DistanceStruct::new(self._ty, self.val() + rhs.convert::<$T>(self._ty).val())
  }
}
impl Sub<DistanceStruct<$T>, DistanceStruct<$T>> for DistanceStruct<$T> {
  fn sub(&self, rhs: &DistanceStruct<$T>) -> DistanceStruct<$T> {
    DistanceStruct::new(self._ty, self.val() - rhs.convert::<$T>(self._ty).val())
  }
}
//impl Mul<DistanceStruct<$T>, Surface<$T>>

impl Show for DistanceStruct<$T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.val(), self.symbol())
  }
}
impl PartialEq for DistanceStruct<$T> {
  fn eq(&self, other: &DistanceStruct<$T>) -> bool {
    self.val() == other.convert::<$T>(self._ty).val()
  }
}
impl PartialOrd for DistanceStruct<$T> {
  fn partial_cmp(&self, other: &DistanceStruct<$T>) -> Option<Ordering> {
    self.val().partial_cmp(&other.convert::<$T>(self._ty).val())
  }
}

)+)) for_primitives!(impl_distance_for_primitives)
