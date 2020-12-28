//! Mutable global variables for Rust. No unsafe code in
//! this crate. Really slow and gross, but better than
//! nothing.
//!
//! Currently only thread-safe sync global variables are
//! provided.  Future enhancements may include thread_local
//! variables, if the `std` ones are ever upgraded to have a
//! guard type instead of just being usable in a closure.

use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

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
        $(static $x: $crate::Global<$t> =
            $crate::Global::new();)*
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

/// Global type.
pub struct Global<T>(OnceCell<Mutex<T>>);

impl<T: 'static> Global<T> {
    /// Global `OnceCell` function --- used to get a new
    /// `OnceCell` with `once_cell` in scope.
    pub const fn new() -> Self {
        Global(OnceCell::new())
    }

    /// Lock a global and acquire the object used to access it. See
    /// `fetch!()` for the macro normally used here.
    ///
    /// # Panics
    ///
    /// Will panic if the global has not yet been initialized.
    /// Will panic if the underlying mutex gets poisoned (should
    /// not happen).
    pub fn fetch(&self) -> MutexGuard<'_, T> {
        self.0
            .get()
            .expect("global uninitialized")
            .lock()
            .expect("global lock poisoned")
    }

    /// Initialize a global reference to contain an initial
    /// value.  See `init!()` for the macro normally used here.
    ///
    /// # Panics
    ///
    /// Will panic on initialization failure; for example on an attempt
    /// to reinitialize a variable.
    pub fn init(&self, v: T) {
        if self.0.set(Mutex::new(v)).is_err() {
            panic!("initialization failed");
        }
    }
}
