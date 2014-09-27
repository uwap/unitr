use meta::HasValue;
use times::*;
use distances::*;

#[deriving(Show, PartialEq, PartialOrd, Clone)]
pub struct Velocity<T>(Distances<T>, Times<T>);

macro_rules! for_velocities(
  ($r:ident) => (
    $r!({mm_ms, mm, ms}, {mm_s, mm, s}, {mm_min, mm, minute}, {mm_h, mm, h},
      {cm_ms, cm, ms}, {cm_s, cm, s}, {cm_min, cm, minute}, {cm_h, cm, h},
      {dm_ms, dm, ms}, {dm_s, dm, s}, {dm_min, dm, minute}, {dm_h, dm, h},
      {m_ms, m, ms}, {m_s, m, s}, {m_min, m, minute}, {m_h, m, h},
      {km_ms, km, ms}, {km_s, km, s}, {km_min, km, minute}, {km_h, km, h})
  );
  ($r:ident, $t:ty) => (
    $r!($t,
      {mm_ms, mm, ms}, {mm_s, mm, s}, {mm_min, mm, minute}, {mm_h, mm, h},
      {cm_ms, cm, ms}, {cm_s, cm, s}, {cm_min, cm, minute}, {cm_h, cm, h},
      {dm_ms, dm, ms}, {dm_s, dm, s}, {dm_min, dm, minute}, {dm_h, dm, h},
      {m_ms, m, ms}, {m_s, m, s}, {m_min, m, minute}, {m_h, m, h},
      {km_ms, km, ms}, {km_s, km, s}, {km_min, km, minute}, {km_h, km, h})
  )
)

macro_rules! generate_velocity_trait(
  ($({$i:ident, $l:ident, $r:ident}),+) => (
    pub trait VelocityTrait<T> : HasValue<T> {
      $(
        fn $i(&self) -> Velocity<T>;
      )+
    }
  )
)

macro_rules! impl_trait_for_primitives_part(
  ($t:ty, $({$m:ident, $l:ident, $r:ident}),+) => (
    $(
      fn $m(&self) -> Velocity<$t> {
        Velocity(self.$l(), (1f64 as $t).$r())
      }
    )+
  )
)

macro_rules! impl_trait_for_primitives(
  ($($t:ty),+) => (
    $(
      impl VelocityTrait<$t> for $t {
        for_velocities!(impl_trait_for_primitives_part, $t)
      }
    )+
  )
)

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

for_velocities!(generate_velocity_trait)
for_types!(impl_trait_for_primitives)
for_types!(impl_velocity_hasvalue)
for_types!(add_distance_division)
