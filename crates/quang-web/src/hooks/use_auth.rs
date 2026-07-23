//! use_auth hook — reactive auth state accessor.

use dioxus::prelude::*;

use crate::auth::UserSession;

/// Hook to access the current auth session reactively.
/// Returns a Signal to the Option<UserSession> provided by the app root.
pub fn use_auth() -> Signal<Option<UserSession>> {
    use_context::<Signal<Option<UserSession>>>()
}
