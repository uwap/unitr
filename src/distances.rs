use meta::HasValue;

#[stable]
pub trait Distance<T> : HasValue<T> {
  fn mm(&self)  -> Distances<T>;
  fn cm(&self)  -> Distances<T>;
  fn dm(&self)  -> Distances<T>;
  fn m (&self)  -> Distances<T>;
  fn km(&self)  -> Distances<T>;
}

#[deriving(Show, PartialEq, PartialOrd, Clone)]
pub enum Distances<T> {
  Kilometer(T),
  Meter(T),
  Decimeter(T),
  Centimeter(T),
  Millimeter(T)
}

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
