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
    #[cfg(not(feature = "std"))]
    type Error<'a>: core::fmt::Debug + core::fmt::Display
    where
        Self: 'a;
    #[cfg(feature = "std")]
    type Error<'a>: std::error::Error
    where
        Self: 'a;

    /// The error type for mutable borrows.
    #[cfg(not(feature = "std"))]
    type ErrorMut<'a>: core::fmt::Debug + core::fmt::Display
    where
        Self: 'a;
    #[cfg(feature = "std")]
    type ErrorMut<'a>: std::error::Error
    where
        Self: 'a;

    /// Immutably borrows the internal value from an immutable reference.
    fn borrow_int(&self) -> Result<Self::Ref<'_>, Self::Error<'_>>;

    /// Mutably borrows the internal value from an immutable reference.
    fn borrow_int_mut(&self) -> Result<Self::RefMut<'_>, Self::ErrorMut<'_>>;
}

/// A reference that can be downgraded to a weak variant.
/// If a type is only referenced via weak references, it will be dropped.
pub trait StrongReference<T: ?Sized>: InteriorMut<T> {
    type Weak: WeakReference<T>;

    /// Return the weak variant of the reference type.
    fn downgrade(&self) -> Self::Weak;
}

/// The weak variant of a reference.
/// If a type is only referenced via weak references, it will be dropped.
pub trait WeakReference<T: ?Sized> {
    type Strong: InteriorMut<T>;

    /// Return the strong variant of the reference type, if it has not be dropped already.
    fn upgrade(&self) -> Option<Self::Strong>;
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

#[cfg(feature = "std")]
impl<T: ?Sized, I: InteriorMut<T> + ?Sized> InteriorMut<T> for std::rc::Rc<I> {
    type Ref<'a> = I::Ref<'a>
    where
        Self: 'a, I: 'a;

    type RefMut<'a>=I::RefMut<'a>
    where
        Self: 'a, I: 'a;

    type Error<'a>=I::Error<'a>
    where
        Self: 'a, I: 'a;

    type ErrorMut<'a>=I::ErrorMut<'a>
    where
        Self: 'a, I: 'a;

    fn borrow_int(&self) -> Result<Self::Ref<'_>, Self::Error<'_>> {
        self.deref().borrow_int()
    }

    fn borrow_int_mut(&self) -> Result<Self::RefMut<'_>, Self::ErrorMut<'_>> {
        self.deref().borrow_int_mut()
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized, I: InteriorMut<T> + ?Sized> StrongReference<T> for std::rc::Rc<I> {
    type Weak = std::rc::Weak<I>;

    fn downgrade(&self) -> Self::Weak {
        std::rc::Rc::downgrade(self)
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized, I: InteriorMut<T> + ?Sized> WeakReference<T> for std::rc::Weak<I> {
    type Strong = std::rc::Rc<I>;

    fn upgrade(&self) -> Option<Self::Strong> {
        std::rc::Weak::upgrade(self)
    }
}
