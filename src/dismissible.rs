//! Dismissible stated scope guard.
//!
//! For a more common and simple situation, where there are only two states,
//! and the default state action is to revert, the other is do nothing, we can
//! use [`DismissibleScopeGuard`].

use crate::ScopeGuard;

/// Dismissible stated scope guard
pub type DismissibleScopeGuard<T, F> = ScopeGuard<T, bool, F>;

/// Create a new dismissible stated scope guard, the `callback` will always
/// be called when dropped unless [`dismiss`][DismissibleScopeGuard::dismiss] is called.
/// As a result, we don't need to check the state in the passed `callback`.
pub fn new_dismissible<T, F: FnOnce(T)>(
    value: T,
    callback: F,
) -> DismissibleScopeGuard<T, impl FnOnce(T, &bool)> {
    ScopeGuard::new(value, true, |value, state| {
        if *state {
            callback(value)
        }
    })
}

impl<T, F> DismissibleScopeGuard<T, F>
where
    F: FnOnce(T, &bool),
{
    /// Dismiss the scope guard callback. After this function, the callback
    /// will not be called when [`DismissibleScopeGuard`] is dropped.
    pub fn dismiss(&mut self) {
        self.set_state(false);
    }
}
