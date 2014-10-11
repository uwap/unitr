#![macro_escape]
macro_rules! generate_unit((
// The generate_unit macro generates the struct, trait and enum
// of a unit and implements basic methods for it.

// below is the pattern to match the macro
enum $_enum:ident<$_enum_size:ty> {
  $($_enum_ident:ident = $_enum_val:expr),+
}
trait $_trait:ident {
  $(fn $_trait_ident:ident(self);)+
}
struct $_struct:ident;
match symbol {
  $($_enum_match_ident:ident => $_symbol_name:expr),+
}
// The variables annoted are as following:
// $_enum is the name of the enum, containing
// at least one $_enum_ident which is assigned to an $_enum_val.
// For example for Distances, $_enum is `Distances`,
// $_enum_ident is `[Millimeter, Centimeter, ...]` and
// $_enum_val is `[1, 1000, 10000, ...]`.
// The trait ($_trait) contains methods ($_trait_ident) in the following flavor:
// `.mm()`, `.cm()`, `.dm()`, ...
// The struct is the struct returned by those functions.
// In the end there is the match symbol statement,
// which describes how the symbols of the $_enum_idents are.
// For example:
// Millimeter => "mm"
) => (

// Below are general definitions assign for any T.
pub trait $_trait<T> : HasValue<T> + Unit {$(
  fn $_trait_ident(self) -> $_struct<T>;
)+}
#[deriving(Clone)]
pub enum $_enum {$(
  $_enum_ident = $_enum_val,
)+}
impl <T> HasValue<T> for $_enum where T: Primitive {
  #[inline]
  fn val(self) -> T {
    NumCast::from::<$_enum_size>(unsafe { mem::transmute(self) }).unwrap()
  }
}
impl Unit for $_enum {
  fn symbol(&self) -> &str {
    match *self {
      $($_enum_match_ident => $_symbol_name),+
    }
  }
}

#[deriving(Clone)]
pub struct $_struct<T> {
  _ty: $_enum,
  _val: T
}

impl <T> $_struct<T> where T: Primitive {
  #[inline]
  pub fn new(kind: $_enum, val: T) -> $_struct<T> {
    $_struct{_ty: kind, _val: val}
  }
  fn convert<U>(self, to: $_enum) -> $_struct<U> where U: Primitive {
    #[inline(always)]
    fn fac<T>(from: T, to: T) -> f64 where T: HasValue<f64> {
      from.val() / to.val()
    }
    let self_val_f64 : f64 = NumCast::from(self._val).unwrap();
    $_struct::new(to,
      NumCast::from::<f64>(
        self_val_f64 * (fac(self._ty, to))).unwrap())
  }
}
impl <T> HasValue<T> for $_struct<T> {
  #[inline]
  fn val(self) -> T {
    self._val
  }
}

// Beside methods for any T, we need methods for any primitive.
// Sadly this doesn't work with using the primitive trait quite well,
// since any T could also be primitive and that could cause multiple impls.
// Therefor we have to generate it for any certain primitive $T.
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, i8)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, u8)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, i16)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, u16)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, i32)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, u32)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, f32)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, i64)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, u64)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, f64)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, int)
generate_unit!($_enum, $_enum_size, {$($_enum_ident = $_enum_val),+}, $_trait {$($_trait_ident),+}, $_struct, uint)
); ($_enum:ident, $_enum_size:ty, {$($_enum_ident:ident = $_enum_val:expr),+}, $_trait:ident {$($_trait_ident:ident),+}, $_struct:ident, $T:ty) => (
// Anything below here is code generated for any primitive $T.

impl $_trait<$T> for $T {
  $(
  #[inline]
  fn $_trait_ident(self) -> $_struct<$T> {
    $_struct::new($_enum_ident, self)
  }
  )+
}

impl $_trait<$T> for $_struct<$T> {
  $(
  #[inline]
  fn $_trait_ident(self) -> $_struct<$T> {
    self.convert($_enum_ident)
  }
  )+
}

impl ToPrimitive for $_struct<$T> {
  #[inline]
  fn to_i64(&self) -> Option<i64> {
    NumCast::from(self.val())
  }
  #[inline]
  fn to_u64(&self) -> Option<u64> {
    NumCast::from(self.val())
  }
}

impl Unit for $_struct<$T> {
  #[inline]
  fn symbol(&self) -> &str {
    self._ty.symbol()
  }
}

// Implement number traits
impl Zero for $_struct<$T> {
  fn zero() -> $_struct<$T> {
    $_struct::new(unsafe { mem::transmute(1 as $_enum_size) }, Zero::zero())
  }
  fn is_zero(&self) -> bool {
    self.val().is_zero()
  }
}
impl Add<$_struct<$T>, $_struct<$T>> for $_struct<$T> {
  fn add(&self, rhs: &$_struct<$T>) -> $_struct<$T> {
    $_struct::new(self._ty, self.val() + rhs.convert::<$T>(self._ty).val())
  }
}
impl Sub<$_struct<$T>, $_struct<$T>> for $_struct<$T> {
  fn sub(&self, rhs: &$_struct<$T>) -> $_struct<$T> {
    $_struct::new(self._ty, self.val() - rhs.convert::<$T>(self._ty).val())
  }
}

impl Show for $_struct<$T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.val(), self.symbol())
  }
}
impl PartialEq for $_struct<$T> {
  fn eq(&self, other: &$_struct<$T>) -> bool {
    self.val() == other.convert::<$T>(self._ty).val()
  }
}
impl PartialOrd for $_struct<$T> {
  fn partial_cmp(&self, other: &$_struct<$T>) -> Option<Ordering> {
    self.val().partial_cmp(&other.convert::<$T>(self._ty).val())
  }
}
))
