//! A crate for abstracting over interior mutable containers.

#![cfg_attr(not(feature = "std"), no_std)]

mod lib {
    #[cfg(not(feature = "std"))]
    pub use core::*;
    #[cfg(feature = "std")]
    pub use std::*;
}

use lib::cell::RefCell;
use lib::ops::{Deref, DerefMut};

/// A trait for obtaining an immutable or mutable reference on types that allow interior mutability.
pub trait InteriorMut<T: ?Sized> {
    /// The immutable reference type.
    type Ref<'a>: Deref<Target = T>
    where
        Self: 'a;

    /// The mutable reference type.
    type RefMut<'a>: DerefMut<Target = T>
    where
        Self: 'a;

    /// The error type for immutable borrows.
    type Error<'a>
    where
        Self: 'a;

    /// The error type for mutable borrows.
    type ErrorMut<'a>
    where
        Self: 'a;

    /// Immutably borrows the internal value from an immutable reference.
    fn borrow_int(&self) -> Result<Self::Ref<'_>, Self::Error<'_>>;

    /// Mutably borrows the internal value from an immutable reference.
    fn borrow_int_mut(&self) -> Result<Self::RefMut<'_>, Self::ErrorMut<'_>>;
}

impl<T: ?Sized> InteriorMut<T> for RefCell<T> {
    type Ref<'a> = lib::cell::Ref<'a, T> where T: 'a;
    type RefMut<'a> = lib::cell::RefMut<'a, T> where T: 'a;
    type Error<'a> = lib::cell::BorrowError where T: 'a;
    type ErrorMut<'a> = lib::cell::BorrowMutError where T: 'a;

    fn borrow_int(&self) -> Result<Self::Ref<'_>, Self::Error<'_>> {
        RefCell::try_borrow(self)
    }

    fn borrow_int_mut(&self) -> Result<Self::RefMut<'_>, Self::ErrorMut<'_>> {
        RefCell::try_borrow_mut(self)
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized> InteriorMut<T> for std::sync::Mutex<T> {
    type Ref<'a> = std::sync::MutexGuard<'a, T> where T: 'a;
    type RefMut<'a> = std::sync::MutexGuard<'a, T> where T: 'a;
    type Error<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a, T>> where T: 'a;
    type ErrorMut<'a> = std::sync::PoisonError<std::sync::MutexGuard<'a, T>> where T: 'a;

    fn borrow_int(&self) -> Result<Self::Ref<'_>, Self::Error<'_>> {
        self.lock()
    }

    fn borrow_int_mut(&self) -> Result<Self::RefMut<'_>, Self::ErrorMut<'_>> {
        self.lock()
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized> InteriorMut<T> for std::sync::RwLock<T> {
    type Ref<'a> = std::sync::RwLockReadGuard<'a, T> where T: 'a;
    type RefMut<'a> = std::sync::RwLockWriteGuard<'a, T> where T: 'a;
    type Error<'a> = std::sync::PoisonError<std::sync::RwLockReadGuard<'a, T>> where T: 'a;
    type ErrorMut<'a> = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>> where T: 'a;

    fn borrow_int(&self) -> Result<Self::Ref<'_>, Self::Error<'_>> {
        self.read()
    }

    fn borrow_int_mut(&self) -> Result<Self::RefMut<'_>, Self::ErrorMut<'_>> {
        self.write()
    }
}
