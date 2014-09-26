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

macro_rules! gen_unit_structs_with_args_derive(
  ($r:ident, $($T:ident > $t:ident $fac:expr),+) => (
    gen_unit_structs!($($T),+)
    $(
      for_types!($r $T of $t $fac)
    )+
  )
)

macro_rules! for_types(
  ($r:ident) => (
    $r!(f64, f32, i64, i32, u64, u32, int, uint)
  )
)

for_types!(impl_has_value_for_primitives)
