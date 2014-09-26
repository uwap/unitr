#![macro_escape]

#[stable]
pub trait HasValue<T> {
  fn val(&self) -> T;
}

macro_rules! impl_has_value_for_primitives(
  ($($t:ty),+) => (
    $(
      impl HasValue<$t> for $t {
        fn val(&self) -> $t {
          *self
        }
      }
    )+
  )
)

macro_rules! gen_unit_structs(
  ($($T:ident),+) => (
    $(
      #[deriving(Show, PartialEq, PartialOrd)]
      pub struct $T<T>(T);
    )+
  )
)

macro_rules! gen_unit_structs_with_args(
  ($r:ident, $($T:ident $fac:expr),+) => (
    gen_unit_structs!($($T),+)
    $(
      for_types!($r $T $fac)
    )+
  )
)

macro_rules! for_types(
  ($r:ident) => (
    $r!(f64, f32, i64, i32, u64, u32, int, uint)
  );
  ($r:ident $T:ident $fac:expr) => (
    for_types_expand!($r $T $fac
      f64, f32, i64, i32, u64, u32, int, uint
      )
  )
)
macro_rules! for_types_expand(
  ($r:ident $T:ident $fac:expr $($t:ty),+) => (
    $(
      $r!($T $t $fac as $t)
    )+
  )
)

for_types!(impl_has_value_for_primitives)
