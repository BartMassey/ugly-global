//! Mutable sync global variables --- may be used across threads.

use std::cell::RefCell;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

/// Global type.
pub type Global<T> = OnceCell<Mutex<RefCell<T>>>;

/// Global "guard" type --- used to get a mutable
/// reference to a global.
pub type GlobalGuard<T> = MutexGuard<'static, RefCell<T>>;

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
    ($($x:ident : $t:ty ;)*) => {
        $(static $x: $crate::sync::Global<$t> =
            once_cell::sync::OnceCell::new();)*
    };
}

/// Lock a global and acquire the object used to access it. See
/// `fetch!()` for the macro normally used here.
///
/// # Panics
///
/// Will panic if the global has not yet been initialized.
/// Will panic if the underlying mutex gets poisoned (should
/// not happen).
pub fn fetch<T>(x: &'static Global<T>) -> GlobalGuard<T> {
    x.get()
        .expect("global uninitialized")
        .lock()
        .expect("global mutex failed")
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
        let mut $y = $crate::sync::fetch(&$x);
        let $y = $y.get_mut();
    };
}

/// Initialize a global reference to contain an initial
/// value.  See `init!()` for the macro normally used here.
///
/// # Panics
///
/// Will panic on initialization failure; for example on an attempt
/// to reinitialize a variable.
pub fn init<T>(x: &'static Global<T>, v: T) {
    if x.set(Mutex::new(RefCell::new(v))).is_err() {
        panic!("initialization failed");
    }
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
        $crate::sync::init(&$x, $v)
    };
}
