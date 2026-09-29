//! **A refusal's two renderings, and its recourse label.**
//!
//! A refusal drawn under its own name opens with its stage word
//! (`product:`, `persist:`, `workspace:`) and may label its parts. A
//! carrier that names the stage itself (a part whose reference did not
//! resolve, a store's "refused to load") draws the same refusal as a
//! bare sentence. [`Staged`] is that pair: a type renders its arms
//! once, at a [`Labels`], and [`Staged::sentence`] is the stripped
//! rendering.

/// Whether a rendering keeps a refusal's own labels: its stage word,
/// and whatever labels its arms carry (a finding's `root N output M`
/// subject, a forwarded load's stage word).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Labels {
    /// The refusal as its own door draws it.
    Kept,
    /// The refusal as a carrier that names the stage draws it.
    Stripped,
}

/// A refusal that renders at either [`Labels`]. Its `Display` is
/// `Labelled(self, Labels::Kept)`.
pub trait Staged {
    /// The stage word the refusal opens with under its own name.
    const STAGE: &'static str;

    /// Writes the refusal's sentence at `labels`, after the stage word
    /// when they are kept.
    ///
    /// # Errors
    ///
    /// The formatter's.
    fn fmt_labelled(&self, f: &mut core::fmt::Formatter<'_>, labels: Labels) -> core::fmt::Result;

    /// The refusal as a carrier that names the stage renders it.
    fn sentence(&self) -> Labelled<'_, Self> {
        Labelled(self, Labels::Stripped)
    }
}

/// A [`Staged`] refusal rendered at a [`Labels`].
pub struct Labelled<'a, E: ?Sized>(pub &'a E, pub Labels);

impl<E: Staged + ?Sized> core::fmt::Display for Labelled<'_, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.1 == Labels::Kept {
            write!(f, "{}: ", E::STAGE)?;
        }
        self.0.fmt_labelled(f, self.1)
    }
}

/// **The recourse, labelled**: `Recourse: {action}`, the one spelling
/// of the label a refusal's way through opens with.
pub struct Recourse<A>(pub A);

impl<A: core::fmt::Display> core::fmt::Display for Recourse<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Recourse: {}", self.0)
    }
}

/// What an API door that was given no part resolver says to do: every
/// door that evaluates, solves, edits or splits over parts takes one.
/// The viewer states its own (it resolves through the saved file's
/// directory).
pub const PASS_A_RESOLVER: &str = "pass a resolver over the store that holds the part";
