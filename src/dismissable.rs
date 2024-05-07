//! For a more common and simple situation, where there are only two states,
//! and the default state action is to revert, the other is do nothing, we can
//! use [`DismissableScopeGuard`].

use crate::ScopeGuard;

/// Dismissable stated scope guard
pub type DismissableScopeGuard<F> = ScopeGuard<(), bool, F>;

/// Create a new dismissable stated scope guard, the `callback` will always
/// be called when dropped unless [`dismiss`][DismissableScopeGuard::dismiss] is called
pub fn new_dismissable<F: FnOnce()>(callback: F) -> DismissableScopeGuard<impl FnOnce((), &bool)> {
    ScopeGuard::new((), true, |_, state| {
        if *state {
            callback()
        }
    })
}

impl<F> DismissableScopeGuard<F>
where
    F: FnOnce((), &bool),
{
    /// Dismiss the scope guard callback. After this function, the callback
    /// will not be called when [`DismissableScopeGuard`] is dropped.
    pub fn dismiss(&mut self) {
        self.set_state(false);
    }
}
