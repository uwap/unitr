use meta::HasValue;
use times::*;
use distances::*;

#[deriving(Show, PartialEq, PartialOrd, Clone)]
pub struct Velocity<T>(Distances<T>, Times<T>);

macro_rules! add_distance_division(
  ($($t:ty),+) => (
    $(
      impl Div<Times<$t>, Velocity<$t>> for Distances<$t> {
        fn div(&self, time: &Times<$t>) -> Velocity<$t> {
          Velocity(*self, *time)
        }
      }
    )+
  )
)

macro_rules! impl_velocity_hasvalue(
  ($($t:ty),+) => (
    $(
      impl HasValue<$t> for Velocity<$t> {
        fn val(&self) -> $t {
          match *self {
            Velocity(d, t) => d.val() / t.val()
          }
        }
      }
    )+
  )
)

for_types!(impl_velocity_hasvalue)
for_types!(add_distance_division)
