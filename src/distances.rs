use meta::HasValue;
use std::mem;

#[unstable="Likely to change its name"]
pub trait Distance<T> : HasValue<T> {
  fn mm(self)  -> DistanceStruct<T>;
  fn cm(self)  -> DistanceStruct<T>;
  fn dm(self)  -> DistanceStruct<T>;
  fn m (self)  -> DistanceStruct<T>;
  fn km(self)  -> DistanceStruct<T>;
}

#[deriving(Show, PartialEq, PartialOrd)]
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
impl Distances {
  #[inline]
  fn fac<T>(from: T, to: T) -> f32 where T: HasValue<f32> {
    from.val() / to.val()
  }
}

#[experimental]
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
    let self_val_f32 : f32 = NumCast::from(self._val).unwrap();
    DistanceStruct::new(to,
      NumCast::from::<f32>(
        self_val_f32 * Distances::fac(self._ty, to)).unwrap())
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

impl HasValue<$T> for DistanceStruct<$T> {
  #[inline]
  fn val(self) -> $T {
    self._val
  }
}

)+)) for_primitives!(impl_distance_for_primitives)

/*
macro_rules! distance_overload_operator(
  ($op:ident, $_self:ident, $other:ident) => (
    match *$_self {
      Millimeter(_) => Millimeter($_self.val().$op(&$other.mm().val())),
      Centimeter(_) => Millimeter($_self.mm().val().$op(&$other.mm().val())).cm(),
      Decimeter(_) => Millimeter($_self.mm().val().$op(&$other.mm().val())).dm(),
      Meter(_) => Millimeter($_self.mm().val().$op(&$other.mm().val())).m(),
      Kilometer(_) => Millimeter($_self.mm().val().$op(&$other.mm().val())).km(),
    }
  )
)

macro_rules! impl_distance_for_primitives(
  ($($t:ty),+) => (
    $(
      // primitive impl
      impl Distance<$t> for $t {
        fn mm(&self) -> Distances<$t> {
          Millimeter(*self)
        }
        fn cm(&self) -> Distances<$t> {
          Centimeter(*self)
        }
        fn dm(&self) -> Distances<$t> {
          Decimeter(*self)
        }
        fn m(&self) -> Distances<$t> {
          Meter(*self)
        }
        fn km(&self) -> Distances<$t> {
          Kilometer(*self)
        }
      }
      // unit impl
      impl Distance<$t> for Distances<$t> {
        fn mm(&self) -> Distances<$t> {
          match *self {
            Kilometer(v) => Millimeter((100000000f64 * v as f64) as $t),
            Meter(v) => Millimeter((100000f64 * v as f64) as $t),
            Decimeter(v) => Millimeter((10000f64 * v as f64) as $t),
            Centimeter(v) => Millimeter((1000f64 * v as f64) as $t),
            Millimeter(v) => Millimeter((1f64 * v as f64) as $t)
          }
        }
        fn cm(&self) -> Distances<$t> {
          Centimeter((self.mm().val() as f64 / 1000f64) as $t)
        }
        fn dm(&self) -> Distances<$t> {
          Decimeter((self.mm().val() as f64 / 10000f64) as $t)
        }
        fn m(&self) -> Distances<$t> {
          Meter((self.mm().val() as f64 / 100000f64) as $t)
        }
        fn km(&self) -> Distances<$t> {
          Kilometer((self.mm().val() as f64 / 100000000f64) as $t)
        }
      }
      impl HasValue<$t> for Distances<$t> {
        fn val(&self) -> $t {
          match *self {
            Kilometer(v) => v,
            Meter(v) => v,
            Decimeter(v) => v,
            Centimeter(v) => v,
            Millimeter(v) => v
          }
        }
      }
      // operator overloading
      impl Add<Distances<$t>, Distances<$t>> for Distances<$t> {
        fn add(&self, other: &Distances<$t>) -> Distances<$t> {
          distance_overload_operator!(add, self, other)
        }
      }
      impl Sub<Distances<$t>, Distances<$t>> for Distances<$t> {
        fn sub(&self, other: &Distances<$t>) -> Distances<$t> {
          distance_overload_operator!(sub, self, other)
        }
      }
      /*impl Div<Times<$t>, Velocity<$t>> for Distances<$t> {
        fn div(&self, other: &Times<$t>) -> Velocity<$t> {
          Velocity((*self).clone(), (*other).clone())
        }
      }*/
    )+
  )
)

for_types!(impl_distance_for_primitives)
*/
