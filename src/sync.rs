//! Mutable sync global variables --- may be used across threads.

use std::sync::{Mutex, MutexGuard};
use once_cell::sync::OnceCell;

// Type of guard usable by synchronous programs.
type Guard<T> = MutexGuard<'static, T>;

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
    pub fn fetch(&'static self) -> Guard<T> {
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
