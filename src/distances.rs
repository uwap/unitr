use meta::HasValue;

#[stable]
pub trait Distance<T> : HasValue<T> {
  fn mm(&self)  -> Millimeter <T>;
  fn cm(&self)  -> Centimeter <T>;
  fn dm(&self)  -> Decimeter  <T>;
  fn m (&self)  -> Meter      <T>;
  fn km(&self)  -> Kilometer  <T>;
}

macro_rules! gen_distance_structs(
  ($($id:ident $t:ty $fac:expr),+) => (
    $(
      impl Distance<$t> for $id<$t> {
        fn mm(&self) -> Millimeter<$t> {
          Millimeter(self.val() * $fac)
        }
        fn cm(&self) -> Centimeter<$t> {
          Centimeter((self.mm().val() as f64 / 1000f64) as $t)
        }
        fn dm(&self) -> Decimeter<$t> {
          Decimeter((self.mm().val() as f64 / 10000f64) as $t)
        }
        fn m(&self) -> Meter<$t> {
          Meter((self.mm().val() as f64 / 100000f64) as $t)
        }
        fn km(&self) -> Kilometer<$t> {
          Kilometer((self.mm().val() as f64 / 100000000f64) as $t)
        }
      }
      impl HasValue<$t> for $id<$t> {
        fn val(&self) -> $t {
          match *self { $id(v) => v }
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
    )+
  )
)

for_types!(impl_distance_for_primitives)
gen_unit_structs_with_args!(gen_distance_structs, Millimeter 1f64, Centimeter 1000f64, Decimeter 10000f64, Meter 100000f64, Kilometer 100000000f64)
