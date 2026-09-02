//! The verb enum itself and its fieldless projection.

use topo::EdgeKey;

/// **One kernel operation, with its parameters as data.**
///
/// Scalars sit at `T`; entity references are arena keys, resolved by
/// whoever built the value. The OPERAND is not here — it is borrowed at
/// [`Verb::run`], because a body is not a parameter of the operation,
/// it is the thing operated on, and putting it in the payload would
/// make every declaration own a clone of it.
///
/// Closed, with no wildcard arm anywhere that matches on it (D3), so a
/// variant added here breaks every commitment site at compile time
/// rather than silently defaulting.
///
/// **`PartialEq` but not `Eq`**, deliberately: `T` is a lane scalar and
/// no scalar this workspace runs on is `Eq` — `f64` is not, `Dual` is
/// not, and an interval's equality is not a total one either. A derived
/// `Eq` bound would therefore be inert on every instantiation that
/// exists, which is a promise no caller could ever cash. Comparison of
/// verbs is structural and partial, exactly like comparison of the
/// scalars inside them.
#[derive(Clone, Debug, PartialEq)]
pub enum Verb<T> {
    /// Constant-radius rolling-ball fillets on a set of the operand's
    /// edges.
    Fillet {
        /// The operand edges to blend.
        edges: Vec<EdgeKey>,
        /// The rolling ball's radius.
        radius: T,
    },
    /// Equal-setback flat chamfers on a set of the operand's edges.
    Chamfer {
        /// The operand edges to chamfer.
        edges: Vec<EdgeKey>,
        /// The setback measured along each support from the edge.
        distance: T,
    },
}

/// **The verb vocabulary with the payload dropped** — the same closed
/// set of names, addressable where no `Verb` value exists yet.
///
/// It exists because the commitments V2 hangs off the vocabulary are
/// not all reachable from a built verb. A content key, in particular,
/// is computed from a document node BEFORE its selection has resolved
/// to arena keys or its slot to a scalar, so there is nothing to match
/// on but the name — and the name is what the tag is a function of.
///
/// This is a projection, not a twin: [`Verb::kind`] is the one place
/// the mapping is written, it is exhaustive, and a new [`Verb`] variant
/// therefore cannot compile until it has a `VerbKind` and every match
/// over `VerbKind` has been visited.
///
/// `sweep::blend::BlendKind` is a different thing that looks like this
/// one: it is the label a blend REFUSAL carries, enumerating the two
/// blend doors, and it lives in `sweep` because a `sweep` refusal
/// carries it. It cannot serve here — the verb vocabulary grows past
/// the blend pair into ops `sweep` must not name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerbKind {
    /// [`Verb::Fillet`].
    Fillet,
    /// [`Verb::Chamfer`].
    Chamfer,
}

impl VerbKind {
    /// Every verb in the vocabulary, for censuses that must be total
    /// over it.
    pub const ALL: &'static [Self] = &[Self::Fillet, Self::Chamfer];
}

impl<T> Verb<T> {
    /// Which verb this is, payload dropped.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        match self {
            Self::Fillet { .. } => VerbKind::Fillet,
            Self::Chamfer { .. } => VerbKind::Chamfer,
        }
    }
}

#[cfg(test)]
mod all_census {
    use super::VerbKind;

    /// **[`VerbKind::ALL`] is the WHOLE vocabulary**, pinned against a
    /// compile-time visit rather than reviewed.
    ///
    /// The precedent this list cites (`profile::Verb::ALL`) is
    /// macro-generated and cannot drift; this one is hand-written, so it
    /// needs the guard the macro would otherwise have been. The match
    /// below is EXHAUSTIVE, so a variant added to the vocabulary makes
    /// this file fail to compile until it is visited here — and every
    /// arm names the same total, so visiting it means writing the new
    /// count, which then reds until `ALL` has grown too.
    ///
    /// The no-repeats half is what makes the count a census: with every
    /// entry distinct, a `len` equal to the number of variants means
    /// `ALL` holds each of them exactly once.
    #[test]
    fn all_is_the_whole_vocabulary() {
        let variants = match VerbKind::Fillet {
            VerbKind::Fillet => 2,
            VerbKind::Chamfer => 2,
        };
        for (i, kind) in VerbKind::ALL.iter().enumerate() {
            assert!(
                !VerbKind::ALL[..i].contains(kind),
                "{kind:?} appears twice in VerbKind::ALL"
            );
        }
        assert_eq!(
            VerbKind::ALL.len(),
            variants,
            "VerbKind::ALL has drifted from the vocabulary — the enum has {variants} variants"
        );
    }
}
