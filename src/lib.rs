#![crate_name = "unitr"]
#![crate_type="lib"]
#![feature(macro_rules, globs)]

//! A library that implements basic units and unit conversions.
//!
//!

pub use self::distances::*;
pub use self::times::*;
pub use self::meta::HasValue;

pub mod meta;
pub mod distances;
pub mod times;

/*pub trait Velocity<T> : HasValue<T> {
  fn mm_ms  (&self) -> MillimetersPerMillsecond <T>;
  fn mm_s   (&self) -> MillimetersPerSecond     <T>;
  fn mm_min (&self) -> MillimetersPerMinute     <T>;
  fn mm_h   (&self) -> MillimetersPerHour       <T>;
}*/

#[test]
fn test_distance() {
  assert_eq!(1000u32.m().val(), 1000u32)
  assert_eq!(1000u32.m().km().val(), 1u32)
  assert_eq!(1000u32.m().km(), 1u32.km())
  assert_eq!(1000u32.m().km().val(), 1u32)
  assert_eq!(1u32.m().km().val(), 100u32.cm().km().val())
  assert_eq!(1u32.km().mm().cm().m(), 1000u32.m().mm().km().dm().m())
  assert!(1u32.km() != 2u32.km())
}

#[test]
fn test_time() {
  assert_eq!(1u32.h().min(), 60u32.min())
  assert_eq!(1000u32.ms(), 1u32.s().ms())
  assert_eq!(1000f64.ms().h().val(), 1f64.s().min().h().val())
}
