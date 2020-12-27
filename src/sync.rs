//! Mutable sync global variables --- may be used across threads.

use std::sync::{Mutex, MutexGuard};
use std::ops::{Deref, DerefMut};
use once_cell::sync::OnceCell;

type MutexGuardSync<T> = MutexGuard<'static, T>;

/// Global "guard" type --- used to get a mutable
/// reference to a global.
pub struct GlobalGuard<T: 'static>(MutexGuardSync<T>);

impl<T: 'static> Deref for GlobalGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: 'static> DerefMut for GlobalGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
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
    pub fn fetch(&'static self) -> GlobalGuard<T> {
        let guard = self.0
            .get()
            .expect("global uninitialized")
            .lock()
            .expect("global lock poisoned");
        GlobalGuard(guard)
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
