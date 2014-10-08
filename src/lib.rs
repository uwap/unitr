#![crate_name = "unitr"]
#![crate_type="lib"]
#![feature(macro_rules, globs)]

//! A library that implements basic units and unit conversions.
//!
//!

pub use self::distances::*;
pub use self::times::*;
pub use self::meta::{HasValue, Unit};
pub use self::velocities::*;

pub mod meta;
pub mod distances;
pub mod times;
pub mod velocities;

#[test]
fn test_distance() {
  assert_eq!(1000u32.m().val(), 1000u32)
  assert_eq!(1000u32.m().km().val(), 1u32)
  assert_eq!(1000u32.m().km(), 1u32.km())
  assert_eq!(1000u32.m().km().val(), 1u32)
  assert_eq!(1u32.m().km().val(), 100u32.cm().km().val())
  assert_eq!(1u32.km().mm().cm().m(), 1000u32.m().mm().km().dm().m())
  assert!(1u32.km() != 2u32.km())
  assert_eq!(1f64.km().mm().km().val(), 1f64)
  assert_eq!(100f64.m() + 5f64.m(), 105f64.m())
  assert_eq!(100f32.m(), 0.1f32.km())
  assert_eq!(1f64.km() + 100f64.m(), 1.1f64.km())
  assert_eq!(1f64.km() - 100f64.m(), 0.9f64.km())
}

#[test]
fn test_time() {
  assert_eq!(1u32.h().minute(), 60u32.minute())
  assert_eq!(1000u32.ms(), 1u32.s().ms())
  assert_eq!(1000f64.ms().h().val(), 1f64.s().minute().h().val())
  assert_eq!(1f32.s() + 100f32.ms(), 1.1f32.s())
}

#[test]
fn test_velocity() {
  assert_eq!((10f64.m() / 2f64.s()).val(), 5f64)
  assert_eq!((100f64.km()) / 100f64.s(), 1f64.km_s())
  assert_eq!((100f64.m() / 1f64.s()), (0.1f64.km() / 1f64.s()).m_s())
  assert_eq!(100f64.m() / 2f64.s(), 50f64.m_s())
  assert!(100f64.m() / 1f64.s() != 50f64.m() / 1f64.s())
}
