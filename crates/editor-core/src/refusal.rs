//! **A refusal shared into a `Clone + Eq` vocabulary.**
//!
//! The evaluation's refusals ([`crate::NodeErrorKind`]) and the gather's
//! ([`crate::ProductError`]) carry kernel refusals UNALTERED (D2), and
//! those kernel types have neither `Clone` nor equality of their own,
//! while the document-layer error enums that carry them on have both.
//! [`Refusal`] is how they cross: it shares the refusal rather than
//! copying it, so the payload reaching a reader is the very value the
//! door raised, its node ids bare for the frame that hands it out to
//! say, and nothing is stringified on the way.

use std::sync::Arc;

/// **A refusal, shared**: a clone is a pointer copy, and the value is
/// the one its door raised ([`crate::NodeRefusal`],
/// [`crate::ProductRefusal`]).
#[derive(Debug)]
pub struct Refusal<E>(Arc<E>);

impl<E> Refusal<E> {
    /// The refusal, as its door typed it.
    #[must_use]
    pub fn get(&self) -> &E {
        &self.0
    }
}

impl<E> Clone for Refusal<E> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<E> From<E> for Refusal<E> {
    fn from(error: E) -> Self {
        Self(Arc::new(error))
    }
}

/// **Equality is over the refusal's `Debug` structure**, which is the
/// derived one on the refusal and on every payload it carries, so two
/// refusals compare equal exactly when they are the same variant
/// carrying the same fields.
///
/// It is written rather than derived because the kernel error types a
/// refusal carries unaltered do not implement `PartialEq`, and inventing
/// equality for them here would be this layer deciding something the
/// kernel owns. Two float differences follow from comparing renderings
/// rather than values, and both are the ones a diagnostic wants: `NaN`
/// payloads compare EQUAL to themselves, and `0.0` and `-0.0` compare
/// DIFFERENT.
///
/// It is an equivalence, so the refusal is `Eq`: the relation is
/// equality of two strings, and the pointer test short-cuts only pairs
/// whose strings are the same.
impl<E: core::fmt::Debug> PartialEq for Refusal<E> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
            || format!("{:?}", self.get()) == format!("{:?}", other.get())
    }
}

impl<E: core::fmt::Debug> Eq for Refusal<E> {}

impl<E: core::fmt::Display> core::fmt::Display for Refusal<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E: crate::spoken::Say> crate::spoken::Say for Refusal<E> {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        self.0.say(f, by)
    }
}
