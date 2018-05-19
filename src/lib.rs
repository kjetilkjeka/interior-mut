#![cfg_attr(not(feature="std"), no_std)]

mod lib {
    #[cfg(feature="std")]
    pub use std::*;
    #[cfg(not(feature="std"))]
    pub use core::*;
}

use lib::ops::DerefMut;
use lib::cell::RefCell;

/// A trait for obtaining a mutable reference on types that allow interior mutability.
pub trait InteriorMut<'a, T> {

    /// The reference type
    type RefMut: DerefMut<Target=T> + 'a;

    /// The error type
    type Error;

    /// Mutably borrows the internal value from an immutable reference.
    fn borrow_int_mut(&'a self) -> Result<Self::RefMut, Self::Error>;
}

impl<'a, T: 'a> InteriorMut<'a, T> for RefCell<T> {
    type RefMut = lib::cell::RefMut<'a, T>;
    type Error = lib::cell::BorrowMutError;

    fn borrow_int_mut(&'a self) -> Result<Self::RefMut, Self::Error> {
        RefCell::try_borrow_mut(self)
    }
}

#[cfg(feature="std")]
impl<'a, T: 'a> InteriorMut<'a, T> for std::sync::Mutex<T> {
    type RefMut = std::sync::MutexGuard<'a, T>;
    type Error = std::sync::PoisonError<std::sync::MutexGuard<'a, T>>;

    fn borrow_int_mut(&'a self) -> std::sync::LockResult<std::sync::MutexGuard<'a, T>> {
        self.lock()
    }
}

#[cfg(feature="std")]
impl<'a, T: 'a> InteriorMut<'a, T> for std::sync::RwLock<T> {
    type RefMut = std::sync::RwLockWriteGuard<'a, T>;
    type Error = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>>;

    fn borrow_int_mut(&'a self) -> std::sync::LockResult<std::sync::RwLockWriteGuard<'a, T>> {
        self.write()
    }
}