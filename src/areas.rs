use meta::{HasValue, Unit};
use std::{fmt, mem};
use std::fmt::Show;
use std::num::{Zero};

#[unstable="Likely to change its name"]
pub trait Area<T> : HasValue<T> + Unit {
  fn mm2(self)  -> AreaStruct<T>;
  fn cm2(self)  -> AreaStruct<T>;
  fn dm2(self)  -> AreaStruct<T>;
  fn m2 (self)  -> AreaStruct<T>;
  fn km2(self)  -> AreaStruct<T>;
}

#[deriving(Show, PartialEq, PartialOrd, Clone)]
pub enum Areas {
  SquareMillimeter  = 1,
  SquareCentimeter  = 1000000,
  SquareDecimeter   = 100000000,
  SquareMeter       = 10000000000,
  SquareKilometer   = 10000000000000000
}
impl <T> HasValue<T> for Areas where T: Primitive {
  #[inline]
  fn val(self) -> T {
    NumCast::from::<i64>(unsafe { mem::transmute(self) }).unwrap()
  }
}
impl Unit for Areas {
  fn symbol(&self) -> &str {
    match *self {
      SquareMillimeter => "mm²",
      SquareCentimeter => "cm²",
      SquareDecimeter  => "dm²",
      SquareMeter      => "m²",
      SquareKilometer  => "km²"
    }
  }
}
// FIXME: Reduce redundancy with Areas::fac
impl Areas {
  #[inline]
  fn fac<T>(from: T, to: T) -> f64 where T: HasValue<f64> {
    from.val() / to.val()
  }
}

#[experimental]
#[deriving(Clone)]
pub struct AreaStruct<T> {
  _ty: Areas,
  _val: T
}

impl <T> AreaStruct<T> where T: Primitive {
  #[inline]
  pub fn new(kind: Areas, val: T) -> AreaStruct<T> {
    AreaStruct{_ty: kind, _val: val}
  }
  fn convert<U>(self, to: Areas) -> AreaStruct<U> where U: Primitive {
    let self_val_f64 : f64 = NumCast::from(self._val).unwrap();
    AreaStruct::new(to,
      NumCast::from::<f64>(
        self_val_f64 * Areas::fac(self._ty, to)).unwrap())
  }
}
impl <T> HasValue<T> for AreaStruct<T> {
  #[inline]
  fn val(self) -> T {
    self._val
  }
}

macro_rules! impl_area_for_primitives(($($T:ty),+) => ($(
impl Area<$T> for $T {
  #[inline]
  fn mm2(self) -> AreaStruct<$T> {
    AreaStruct::new(SquareMillimeter, self)
  }
  #[inline]
  fn cm2(self) -> AreaStruct<$T> {
    AreaStruct::new(SquareCentimeter, self)
  }
  #[inline]
  fn dm2(self) -> AreaStruct<$T> {
    AreaStruct::new(SquareDecimeter, self)
  }
  #[inline]
  fn m2 (self) -> AreaStruct<$T> {
    AreaStruct::new(SquareMeter, self)
  }
  #[inline]
  fn km2(self) -> AreaStruct<$T> {
    AreaStruct::new(SquareKilometer, self)
  }
}

impl Area<$T> for AreaStruct<$T> {
  #[inline]
  fn mm2(self) -> AreaStruct<$T> {
    self.convert(SquareMillimeter)
  }
  #[inline]
  fn cm2(self) -> AreaStruct<$T> {
    self.convert(SquareCentimeter)
  }
  #[inline]
  fn dm2(self) -> AreaStruct<$T> {
    self.convert(SquareDecimeter)
  }
  #[inline]
  fn m2 (self) -> AreaStruct<$T> {
    self.convert(SquareMeter)
  }
  #[inline]
  fn km2(self) -> AreaStruct<$T> {
    self.convert(SquareKilometer)
  }
}

impl ToPrimitive for AreaStruct<$T> {
  #[inline]
  fn to_i64(&self) -> Option<i64> {
    NumCast::from(self.val())
  }
  #[inline]
  fn to_u64(&self) -> Option<u64> {
    NumCast::from(self.val())
  }
}

impl Unit for AreaStruct<$T> {
  #[inline]
  fn symbol(&self) -> &str {
    self._ty.symbol()
  }
}

// Implement number traits
impl Zero for AreaStruct<$T> {
  fn zero() -> AreaStruct<$T> {
    AreaStruct::new(SquareMeter, Zero::zero())
  }
  fn is_zero(&self) -> bool {
    self.val().is_zero()
  }
}
impl Add<AreaStruct<$T>, AreaStruct<$T>> for AreaStruct<$T> {
  fn add(&self, rhs: &AreaStruct<$T>) -> AreaStruct<$T> {
    AreaStruct::new(self._ty, self.val() + rhs.convert::<$T>(self._ty).val())
  }
}
impl Sub<AreaStruct<$T>, AreaStruct<$T>> for AreaStruct<$T> {
  fn sub(&self, rhs: &AreaStruct<$T>) -> AreaStruct<$T> {
    AreaStruct::new(self._ty, self.val() - rhs.convert::<$T>(self._ty).val())
  }
}
//impl Mul<AreaStruct<$T>, Surface<$T>>

impl Show for AreaStruct<$T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.val(), self.symbol())
  }
}
impl PartialEq for AreaStruct<$T> {
  fn eq(&self, other: &AreaStruct<$T>) -> bool {
    self.val() == other.convert::<$T>(self._ty).val()
  }
}
impl PartialOrd for AreaStruct<$T> {
  fn partial_cmp(&self, other: &AreaStruct<$T>) -> Option<Ordering> {
    self.val().partial_cmp(&other.convert::<$T>(self._ty).val())
  }
}

)+)) for_primitives!(impl_area_for_primitives)
