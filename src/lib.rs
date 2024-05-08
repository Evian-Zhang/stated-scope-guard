#![doc = include_str!("../README.md")]
#![no_std]

pub mod dismissible;

use core::ops::{Deref, DerefMut};

/// Stated scope guard
pub struct ScopeGuard<T, S, F>
where
    F: FnOnce(T, &S),
{
    /// Inner value
    ///
    /// Use `Option` here since the [`drop`][Drop::drop] method only supports `&mut self`,
    /// while we need to take ownership in `drop`.
    value: Option<T>,
    /// State
    state: S,
    /// Callback function. It takes current value and state as parameter, and is expected to work as
    /// dealing `value` differently according to `state`.
    ///
    /// This function will be called when [`ScopeGuard`] is dropped.
    ///
    /// Use `Option` here since the [`drop`][Drop::drop] method only supports `&mut self`,
    /// while we need to take ownership in `drop`.
    callback: Option<F>,
}

impl<T, S, F> ScopeGuard<T, S, F>
where
    F: FnOnce(T, &S),
{
    /// Create a new stated scope guard.
    ///
    /// The `value` passed into it can be derefed by [`Deref`] and [`DerefMut`] trait.
    pub fn new(value: T, state: S, callback: F) -> Self {
        Self {
            value: Some(value),
            state,
            callback: Some(callback),
        }
    }

    /// Set current state to `state`
    pub fn set_state(&mut self, state: S) {
        self.state = state;
    }
}

impl<T, S, F> Deref for ScopeGuard<T, S, F>
where
    F: FnOnce(T, &S),
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `value` is always `Some` until dropped
        unsafe { self.value.as_ref().unwrap_unchecked() }
    }
}

impl<T, S, F> DerefMut for ScopeGuard<T, S, F>
where
    F: FnOnce(T, &S),
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `value` is always `Some` until dropped
        unsafe { self.value.as_mut().unwrap_unchecked() }
    }
}

impl<T, S, F> Drop for ScopeGuard<T, S, F>
where
    F: FnOnce(T, &S),
{
    /// When dropping, the `callback` will be called with current `value`
    /// and `state` as parameter.
    fn drop(&mut self) {
        // SAFETY: `value` is always `Some` until dropped
        let value = unsafe { self.value.take().unwrap_unchecked() };
        // SAFETY: `callback` is always `Some` until dropped
        let callback = unsafe { self.callback.take().unwrap_unchecked() };
        callback(value, &self.state);
    }
}
