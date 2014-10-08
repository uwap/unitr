use meta::{Unit, HasValue};
use std::fmt;
use std::fmt::Show;
use times::*;
use distances::*;

// TODO: Implement add, sub and mul traits
// TODO: Implement Unit for Velocity

#[deriving(Clone)]
pub struct Velocity<T>(DistanceStruct<T>, TimeStruct<T>);

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
      #[inline]
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
      impl Div<TimeStruct<$t>, Velocity<$t>> for DistanceStruct<$t> {
        #[inline]
        fn div(&self, time: &TimeStruct<$t>) -> Velocity<$t> {
          Velocity(*self, *time)
        }
      }
    )+
  )
)

macro_rules! impl_velocity_trait_for_velocity_part(
  ($t:ty, $({$m:ident, $l:ident, $r:ident}),+) => (
    $(
      #[inline]
      fn $m(&self) -> Velocity<$t> {
        match *self {
          Velocity(d, t) => Velocity(d.$l(), t.$r())
        }
      }
    )+
  )
)

macro_rules! impl_velocity_trait_for_velocity(
  ($($t:ty),+) => (
    $(
      impl VelocityTrait<$t> for Velocity<$t> {
        for_velocities!(impl_velocity_trait_for_velocity_part, $t)
      }
      impl HasValue<$t> for Velocity<$t> {
        #[inline]
        fn val(self) -> $t {
          match self {
            Velocity(d, t) => d.val() / t.val()
          }
        }
      }
      impl PartialEq for Velocity<$t> {
        #[inline]
        fn eq(&self, other: &Velocity<$t>) -> bool {
          self.val() == other.val()
        }
      }
      impl PartialOrd for Velocity<$t> {
        #[inline]
        fn partial_cmp(&self, other: &Velocity<$t>) -> Option<Ordering> {
          (&self.val()).partial_cmp(&other.val())
        }
      }
      impl Show for Velocity<$t> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "{}{}/{}", self.val(),
            match *self { Velocity(ref d, _) => d.symbol() },
            match *self { Velocity(_, ref t) => t.symbol() })
        }
      }
    )+
  )
)

for_velocities!(generate_velocity_trait)
for_primitives!(impl_trait_for_primitives)
for_primitives!(impl_velocity_trait_for_velocity)
for_primitives!(add_distance_division)
