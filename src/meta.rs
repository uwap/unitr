#![macro_escape]

#[stable]
pub trait HasValue<T> {
  fn val(self) -> T;
}
#[unstable]
pub trait Unit {
  fn symbol(&self) -> &str;
}

macro_rules! impl_has_value_for_primitives(
  ($($t:ty),+) => (
    $(
      impl HasValue<$t> for $t {
        #[inline(always)]
        fn val(self) -> $t {
          self
        }
      }
      impl Unit for $t {
        #[inline(always)]
        fn symbol(&self) -> &str {
          ""
        }
      }
    )+
  )
)

macro_rules! gen_unit_structs_with_args_derive(
  ($r:ident, $($T:ident > $t:ident $fac:expr),+) => (
    gen_unit_structs!($($T),+)
    $(
      for_types!($r $T of $t $fac)
    )+
  )
)

macro_rules! for_primitives(
  ($r:ident, $ri:ident, $p:tt) => (
    $r!($p, $ri, uint, u8, u16, u32, u64, int, i8, i16, i32, i64, f32, f64)
  );
  ($r:ident, $p:tt) => (
    $r!($p, uint, u8, u16, u32, u64, int, i8, i16, i32, i64, f32, f64)
  );
  ($r:ident) => (
    $r!(uint, u8, u16, u32, u64, int, i8, i16, i32, i64, f32, f64)
  )
)
macro_rules! for_single_primitives(
  ($r:ident, $p:tt) => (
    for_primitives!(for_single_primitives_inner, $r, $p)
  )
)
macro_rules! for_single_primitives_inner(
  ($r:ident, $p:tt, $($t:ty),+) => (
    $(
      $r!($p, $t)
    )+
  )
)

for_primitives!(impl_has_value_for_primitives)
