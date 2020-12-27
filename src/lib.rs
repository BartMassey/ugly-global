//! Mutable global variables for Rust. No unsafe code in
//! this crate. Really slow and gross, but better than
//! nothing.
//!
//! Currently only sync and unsync global variables are
//! provided.  Future enhancements may include thread_local
//! variables.

pub mod sync;

/// Declare mutable global variables. Use uppercase
/// variable names to avoid compiler warnings.
///
/// # Examples
///
/// ```
/// use ugly_global::*;
/// struct S { x: usize, y: usize };
/// global_vars! {
///     X: S;
/// }
/// ```
#[macro_export]
macro_rules! global_vars {
    ($module:ident, $($x:ident : $t:ty ;)*) => {
        $(static $x: $crate::$module::Global<$t> =
            $crate::$module::Global::new();)*
    };
}

/// Declare a local identifier containing a mutable reference
/// to a global variable. The reference will be statically invalid
/// at the end of the scope in which `fetch!()` is invoked.
///
/// # Panics
///
/// See `fetch()`.
///
/// # Examples
///
/// ```
/// use ugly_global::*;
/// struct S { u: usize };
/// global_vars! {
///     X: S;
/// }
///
/// fn f() {
///     fetch!(s = X);
///     s.u += 1;
/// }
///
/// fn main() {
///     init!(X = S { u: 0 });
/// }
/// ```
#[macro_export]
macro_rules! fetch {
    ($y:ident = $x:ident) => {
        let mut $y = $x.fetch();
    };
}

/// Initialize a global variable. Must be called before
/// first access.
///
/// # Panics
///
/// See `init()`.
///
/// # Examples
///
/// ```
/// use ugly_global::*;
/// struct S { u: usize };
/// global_vars! {
///     X: S;
/// }
///
/// fn f() {
///     fetch!(s = X);
///     s.u += 1;
/// }
///
/// fn main() {
///     init!(X = S { u: 0 });
/// }
/// ```
#[macro_export]
macro_rules! init {
    ($x:ident = $v:expr) => {
        $x.init($v)
    };
}
