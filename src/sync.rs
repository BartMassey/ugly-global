//! Mutable sync global variables --- may be used across threads.

use std::cell::RefCell;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

/// Global type.
pub struct Global<T>(OnceCell<Mutex<RefCell<T>>>);

/// Global "guard" type --- used to get a mutable
/// reference to a global.
pub type GlobalGuard<T> = MutexGuard<'static, RefCell<T>>;

impl<T> Global<T> {
    /// Global `OnceCell` function --- used to get a new
    /// `OnceCell` with `once_cell` in scope.
    pub const fn new() -> Self {
        Global(once_cell::sync::OnceCell::new())
    }

    /// Lock a global and acquire the object used to access it. See
    /// `fetch!()` for the macro normally used here.
    ///
    /// # Panics
    ///
    /// Will panic if the global has not yet been initialized.
    /// Will panic if the underlying mutex gets poisoned (should
    /// not happen).
    pub fn fetch(&'static self) -> GlobalGuard<T> {
        self.0
            .get()
            .expect("global uninitialized")
            .lock()
            .expect("global mutex failed")
    }


    /// Initialize a global reference to contain an initial
    /// value.  See `init!()` for the macro normally used here.
    ///
    /// # Panics
    ///
    /// Will panic on initialization failure; for example on an attempt
    /// to reinitialize a variable.
    pub fn init(&'static self, v: T) {
        if self.0.set(Mutex::new(RefCell::new(v))).is_err() {
            panic!("initialization failed");
        }
    }
}
