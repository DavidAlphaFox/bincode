pub trait Sealed {}
/// https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds
/// fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
/// y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
/// let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
/// }
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax
impl<'a, T> Sealed for &'a mut T where T: Sealed {}
