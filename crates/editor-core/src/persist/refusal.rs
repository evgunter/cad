//! How a structured refusal leaves a `Deserialize` impl.
//!
//! `Deserialize` hands an impl exactly one error type and it is the
//! FORMAT's: everything a rebuild refusal knows has to fit through
//! `serde::de::Error::custom`, which takes prose. That is why an
//! ill-dimensioned expression in a save file used to reach a caller as
//! a sentence inside [`super::PersistError::Unreadable`] — the
//! [`DimensionError`] itself had nowhere to go.
//!
//! This slot is where it goes. The refusing impl records the typed
//! value here on its way out, and [`super::parse_body`] reads it back
//! beside serde_json's own classification, so the door can answer with
//! [`super::PersistError::Dimension`] carrying the refusal rather than
//! a description of it.
//!
//! # What makes reading it back sound
//!
//! The slot belongs to ONE `parse_body` call: that function empties it
//! before the parse and takes it after, on both the success and the
//! failure path, so nothing a parse leaves behind can be read as the
//! next parse's. It is thread-local, so two threads loading at once
//! never see each other's.
//!
//! **First refusal wins.** Every impl in [`super::wire`] propagates the
//! first error it meets (`?` in `rebuild`, `?` in the derived visitors
//! above it), so the refusal that reaches the caller is the first one
//! raised — which is the one this slot keeps. The premise is that no
//! type between an expression and the file body asks serde to TRY an
//! alternative: `#[serde(untagged)]` and `#[serde(other)]` appear
//! nowhere in this crate, and a derived externally-tagged enum commits
//! to its variant on the tag rather than by attempting arms. A
//! declaration that changed this would let a refusal be recorded and
//! then discarded, and the door would name a refusal that did not
//! decide the parse.
//!
//! The reading back is narrow for that reason: a recorded refusal is
//! adopted only when serde_json classifies the failure as
//! [`serde_json::error::Category::Data`] — the class a rebuild refusal
//! raises — and a parse that succeeds discards whatever is in the slot
//! rather than reporting it.

use std::cell::RefCell;

use crate::expr::DimensionError;

thread_local! {
    /// The refusal the current parse's first rebuild raised, if any.
    static SLOT: RefCell<Option<DimensionError>> = const { RefCell::new(None) };
}

/// Records a rebuild refusal on its way out through `Error::custom`.
///
/// Keeps the FIRST of a parse, which is the one that propagates.
pub(super) fn record(err: &DimensionError) {
    SLOT.with_borrow_mut(|slot| {
        if slot.is_none() {
            *slot = Some(err.clone());
        }
    });
}

/// Empties the slot, answering what it held.
pub(super) fn take() -> Option<DimensionError> {
    SLOT.with_borrow_mut(Option::take)
}
