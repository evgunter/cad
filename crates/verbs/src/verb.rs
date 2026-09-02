//! The verb enum itself and its fieldless projection.

use topo::{BooleanDeclarations, BooleanOp, EdgeKey};

/// **One kernel operation, with its parameters as data.**
///
/// Scalars sit at `T`; entity references are arena keys, resolved by
/// whoever built the value. The OPERANDs are not here — they are
/// borrowed at the run doors ([`Verb::run`], [`Verb::run_pair`]),
/// because a body is not a parameter of the operation, it is the thing
/// operated on, and putting it in the payload would make every
/// declaration own a clone of it. How many bodies a verb takes is
/// declared data ([`VerbKind::arity`]), and each arity has its own
/// door, so the payload never smuggles an operand count either.
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
    /// A regularized boolean over two operand bodies.
    ///
    /// The payload is the boolean door's own parameter list with the
    /// operands and the run witnesses removed: which regularized op,
    /// and the declared coincidence intents in the kernel's lowered
    /// form — arena keys into the two operands, resolved by whoever
    /// built the value (the recipe layer's name resolution never
    /// enters this crate). The candidate-sweep strategy is NOT here:
    /// it is a property of the run, not of the operation (both
    /// strategies produce bit-identical results), so it comes in at
    /// [`Verb::run_pair`] beside the tolerance witness.
    Boolean {
        /// The regularized set operation.
        op: BooleanOp,
        /// Declared coincidence intents, in operand arena keys.
        declare: BooleanDeclarations,
    },
}

/// **The verb vocabulary with the scalar and reference payload
/// dropped** — the closed set of operation names, addressable where no
/// [`Verb`] value exists yet.
///
/// It exists because the commitments V2 hangs off the vocabulary are
/// not all reachable from a built verb. A content key, in particular,
/// is computed from a document node BEFORE its selection has resolved
/// to arena keys or its slot to a scalar, so there is nothing to match
/// on but the name — and the name is what the tag is a function of.
///
/// **The boolean rows carry the op**, and that is part of the NAME,
/// not payload leaking in: union, intersect and subtract are three
/// kernel operations (the kernel's own production doors are `union`,
/// `intersect` and `subtract`) that share one payload shape, and every
/// commitment keyed on this vocabulary — the content tags first —
/// has always kept the three apart. What is dropped here is the
/// scalars and the entity references, the halves of a payload no
/// commitment is a function of.
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
    /// [`Verb::Boolean`] running the named regularized op.
    Boolean(BooleanOp),
}

/// **How many operand bodies a verb takes** — the declared arity of
/// design V1 ("the declaration states operand arity and kind"), as
/// data.
///
/// Each arity has its own run door with the operand count in its
/// signature ([`Verb::run`], [`Verb::run_pair`]); this enum is what the
/// doors' typed mismatch refusal ([`crate::VerbError::Arity`]) speaks,
/// and what a test can assert the doors against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arity {
    /// One operand body.
    One,
    /// Two operand bodies.
    Two,
}

impl VerbKind {
    /// Every verb in the vocabulary, for censuses that must be total
    /// over it.
    pub const ALL: &'static [Self] = &[
        Self::Fillet,
        Self::Chamfer,
        Self::Boolean(BooleanOp::Union),
        Self::Boolean(BooleanOp::Intersect),
        Self::Boolean(BooleanOp::Subtract),
    ];

    /// The verb's declared operand arity: which run door answers it.
    #[must_use]
    pub fn arity(self) -> Arity {
        match self {
            Self::Fillet | Self::Chamfer => Arity::One,
            Self::Boolean(_) => Arity::Two,
        }
    }
}

impl<T> Verb<T> {
    /// Which verb this is, scalar and reference payload dropped.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        match self {
            Self::Fillet { .. } => VerbKind::Fillet,
            Self::Chamfer { .. } => VerbKind::Chamfer,
            Self::Boolean { op, .. } => VerbKind::Boolean(*op),
        }
    }
}

#[cfg(test)]
mod all_census {
    use topo::BooleanOp;

    use super::VerbKind;

    /// **[`VerbKind::ALL`] is the WHOLE vocabulary**, pinned against a
    /// compile-time visit rather than reviewed.
    ///
    /// The precedent this list cites (`profile::Verb::ALL`) is
    /// macro-generated and cannot drift; this one is hand-written, so it
    /// needs the guard the macro would otherwise have been. The match
    /// below is EXHAUSTIVE — over the vocabulary AND over the boolean's
    /// op, since each op is its own row — so a variant added to either
    /// enum makes this file fail to compile until it is visited here,
    /// and every arm names the same total, so visiting it means writing
    /// the new count, which then reds until `ALL` has grown too.
    ///
    /// The no-repeats half is what makes the count a census: with every
    /// entry distinct, a `len` equal to the number of rows means `ALL`
    /// holds each of them exactly once.
    #[test]
    fn all_is_the_whole_vocabulary() {
        let rows = match VerbKind::Fillet {
            VerbKind::Fillet => 5,
            VerbKind::Chamfer => 5,
            VerbKind::Boolean(op) => match op {
                BooleanOp::Union => 5,
                BooleanOp::Intersect => 5,
                BooleanOp::Subtract => 5,
            },
        };
        for (i, kind) in VerbKind::ALL.iter().enumerate() {
            assert!(
                !VerbKind::ALL[..i].contains(kind),
                "{kind:?} appears twice in VerbKind::ALL"
            );
        }
        assert_eq!(
            VerbKind::ALL.len(),
            rows,
            "VerbKind::ALL has drifted from the vocabulary — it has {rows} rows"
        );
    }
}
