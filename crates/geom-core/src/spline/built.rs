//! **Who built a refused input**: a spline-layer refusal's `Display`
//! ends on a recourse addressed to whoever supplied the spline or asked
//! for the knot edit. Where that is the kernel itself (or a file it
//! adopted), the recourse is not the user's, so the holder keeps the
//! refusal as a [`Built`], whose `Display` is the refused [`Fact`]
//! alone and leaves the ending to the holder's door.

/// **What a refusal states without its recourse**: the sentence a
/// [`Built`] renders, and the prefix of the refusal's own `Display`,
/// which follows it with `. Recourse: ` and the recourse its caller can
/// act on.
pub trait Fact {
    /// The refused fact, with no recourse.
    fn fact(&self) -> impl core::fmt::Display + '_;
}

/// **A refusal of an input the kernel built**, not the reader of the
/// holder: set at the `map_err` of the kernel's own build, where who
/// built the input is known. Its `Display` is [`Fact::fact`] alone, so
/// a holder that writes it cannot pass the caller-facing recourse on;
/// the holder ends it at its own door.
#[derive(Clone, Debug, PartialEq)]
pub struct Built<E>(E);

impl<E> Built<E> {
    /// The refusal of an input the kernel built.
    pub const fn new(refusal: E) -> Self {
        Self(refusal)
    }

    /// The refusal, for matching on its arms.
    pub const fn refusal(&self) -> &E {
        &self.0
    }
}

impl<E: Fact> core::fmt::Display for Built<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fact().fmt(f)
    }
}

impl<E: Fact + core::fmt::Debug> core::error::Error for Built<E> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spline::{KnotAlgebraError, KnotVectorIssue, SplineError};

    /// A kernel-built refusal renders its fact, which its caller-facing
    /// text carries before the recourse, and no recourse of its own.
    #[test]
    fn a_built_refusal_renders_its_fact_and_no_recourse() {
        let spline = SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::DegreeZero,
        };
        let knot = KnotAlgebraError::Structure(spline.clone());
        let texts = [
            (spline.to_string(), Built::new(spline).to_string()),
            (knot.to_string(), Built::new(knot).to_string()),
        ];
        for (caller, built) in &texts {
            assert!(
                caller.contains("Recourse:"),
                "the caller's text has one: {caller}"
            );
            assert!(
                !built.contains("Recourse"),
                "the built text has none: {built}"
            );
            assert!(
                caller.contains(built.as_str()),
                "the fact is the caller's: {built} | {caller}"
            );
        }
    }
}
