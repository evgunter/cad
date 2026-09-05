//! The verb enum itself and its fieldless projection.

use geom_core::Real;
use sweep::{Revolution, RevolveAxis};
use topo::{BooleanDeclarations, BooleanOp, EdgeKey, FaceKey, SplitPlane};

/// **One kernel operation, with its parameters as data.**
///
/// Scalars sit at `T`; entity references are arena keys, resolved by
/// whoever built the value. The OPERANDs are not here — they are
/// borrowed at the run doors ([`Verb::run`], [`Verb::run_pair`],
/// [`Verb::run_profile`], [`Verb::run_split`], [`Verb::run_shell`]),
/// because an operand is
/// not a parameter of
/// the operation, it is the thing operated on, and putting it in the
/// payload would make every declaration own a clone of it. That holds
/// for the sweeps' validated PROFILE exactly as it holds for a body:
/// it is borrowed at the door and never stored. What an operand IS,
/// and how many, is declared data ([`VerbKind::arity`]), and each
/// shape has its own door, so the payload never smuggles an operand
/// count either.
///
/// Closed, with no wildcard arm anywhere that matches on it (D3), so a
/// variant added here breaks every commitment site at compile time
/// rather than silently defaulting.
///
/// **`T: Real`**, which the declaration did not need while every
/// payload was scalars and keys: the revolve's axis is a point and a
/// direction in the sketch plane, and `geom-core`'s vectors are
/// defined for a lane scalar only. Every lane this workspace runs is
/// one, so the bound narrows nothing a caller could have wanted.
///
/// **No equality of any kind**, and the reason is the payload rather
/// than a preference. A verb's payload holds the operation's own
/// values, and once one of those is GEOMETRY — the revolve's axis, a
/// point and a direction in sketch coordinates — the declaration
/// inherits `geom-core`'s stance on comparing geometry: `Point2` and
/// `Vec2` carry no `PartialEq` at all, deliberately, because deciding
/// whether two coordinates are the same is what the classification
/// band is for and never what `==` is for. Synthesising one here
/// (component-wise, at `T: PartialEq`) would put exactly that
/// comparison in the kernel's verb vocabulary, so this type does
/// without: nothing in the tree compares verbs, and the day something
/// must, what it wants is a decided predicate at a band, not a derive.
#[derive(Clone, Debug)]
pub enum Verb<T: Real> {
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
    /// Extrudes the operand profile along its sketch normal.
    ///
    /// The payload is the door's parameter list minus the operand:
    /// the signed distance alone. The door's second extrusion form —
    /// an explicit world VECTOR — is deliberately absent: no recipe
    /// spells one, so a variant for it would be an arm every
    /// commitment on this vocabulary (content tag, wire spelling,
    /// Python constructor, viewer label) had to name and no document
    /// could ever reach.
    Extrude {
        /// The signed distance along the profile plane's normal.
        distance: T,
    },
    /// Revolves the operand profile about an axis written in its own
    /// sketch plane.
    ///
    /// Both fields are the kernel door's own values. The `Revolution`
    /// in particular is CLASSIFIED, not raw: whether an authored angle
    /// is the exact full turn or a partial one is a decided predicate
    /// at the document layer's own funnel site, and its escalation is
    /// a document-layer refusal — so what reaches the payload is the
    /// classified value, exactly as the boolean's `declare` reaches it
    /// already resolved to arena keys.
    Revolve {
        /// The axis of revolution, in sketch-plane coordinates.
        axis: RevolveAxis<T>,
        /// How far to revolve.
        revolution: Revolution<T>,
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
    /// Parts the operand body by a plane into its two sides.
    ///
    /// The plane is a kernel VALUE in the payload — a point and a unit
    /// normal — exactly as the boolean carries its declarations
    /// already resolved: whatever document object the plane was read
    /// off (a datum node, upstairs) is the recipe layer's business, and
    /// what reaches this crate is the split door's own `SplitPlane`.
    /// The body is the one operand and stays out of the payload like
    /// every other. What sets this verb apart is not what it takes but
    /// what it gives back — two sides, each a body or the typed empty —
    /// which is why it answers its own door ([`Arity::Split`]).
    Split {
        /// The parting plane; the side its normal points to is ABOVE.
        plane: SplitPlane<T>,
    },
    /// Hollows the operand body to a wall of the given thickness,
    /// optionally opening designated faces into rims.
    ///
    /// **One variant, not two.** The kernel spells the sealed hollow
    /// and the opened one as two functions (`topo::shell`,
    /// `topo::shell_open`), and the second's own contract says an empty
    /// designation IS the first — so a `Shell` arm with an empty `open`
    /// is `shell`, and a second variant would be a name for a value of
    /// this one's payload. That is the boolean's rule read the other
    /// way: three kernel doors that differ in WHAT THEY DO are three
    /// names, while one door reached with an empty list is one name.
    ///
    /// The designation is arena keys, resolved by whoever built the
    /// value, exactly as the blends' edges are.
    Shell {
        /// The wall thickness in metres — a magnitude; each face's
        /// inward direction is its own orientation's.
        thickness: T,
        /// The operand faces to open into rims. Empty is the sealed
        /// hollow.
        open: Vec<FaceKey>,
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
    /// [`Verb::Extrude`].
    Extrude,
    /// [`Verb::Revolve`].
    Revolve,
    /// [`Verb::Boolean`] running the named regularized op.
    Boolean(BooleanOp),
    /// [`Verb::Split`].
    Split,
    /// [`Verb::Shell`].
    ///
    /// **A kernel-only verb**: the document layer has no `Node` that
    /// builds one, so every commitment keyed on this vocabulary has to
    /// say what it means for a name no document can reach yet, rather
    /// than skip it (`editor-core`'s content tag is the first —
    /// `verb_content_tag` answers `None` here and the tag censuses read
    /// that as closed data).
    Shell,
}

/// **The run doors, as data** — one row per door, and every verb names
/// the row that answers it ([`VerbKind::arity`]).
///
/// A door is its signature at BOTH ends: the operand it takes and the
/// out-type it hands back. Two doors that take the same operand and
/// hand back different things are two rows — `One` and `Split` both
/// take one body — so a row is never, on its own, a claim about an
/// operand count.
///
/// **And at a third place: the SCALAR the door can run at.** `One` and
/// `Shell` agree at both ends — one body in, one body and a record
/// out — and are still two doors, because the shell's op door demands
/// certification rights (`Decide + PropsQuadLane + CertifiedBounds`)
/// that the blend doors do not, and no `Dual` scalar has them. A
/// `Verb<Dual<f64>>` can be handed to `One` and cannot be handed to
/// `Shell`; that is not a run-time refusal to be spoken by this enum
/// but a signature the caller either satisfies or does not compile
/// against, and a door with its own bound is a door. The rows are what the doors' typed mismatch refusal
/// ([`crate::VerbError::Arity`]) speaks, and what `tests/run_door.rs`
/// asserts the doors against: [`Arity::ALL`] and the door matrix there
/// must name the same set, so a row without a door, or a door without
/// a row, reds. The name dates from when the rows were operand counts
/// and is kept because it crosses the document layer's refusal
/// payload; read it as the door vocabulary and nothing narrower.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arity {
    /// [`Verb::run`]: one operand body in, one result body and its
    /// record out ([`crate::VerbOut`]).
    One,
    /// [`Verb::run_pair`]: two operand bodies in, a result body with
    /// its record or the typed empty out ([`crate::PairOut`]).
    Two,
    /// [`Verb::run_profile`]: one validated profile in — borrowed at
    /// the door, never a body, never in the payload — and the door's
    /// own bundle out, as the record ([`crate::VerbRecord`]).
    Profile,
    /// [`Verb::run_split`]: one operand body in, exactly `One`'s, and
    /// TWO sides out under one record ([`crate::SplitOut`]). The door
    /// is its own because its out-type is.
    Split,
    /// [`Verb::run_shell`]: one operand body in and one out with its
    /// record, exactly [`Arity::One`]'s two ends — and a separate row
    /// because the third half of a door's signature differs (see this
    /// enum's docs on the scalar).
    Shell,
}

impl Arity {
    /// Every door row, for censuses that must be total over the
    /// doors.
    pub const ALL: &'static [Self] = &[
        Self::One,
        Self::Two,
        Self::Profile,
        Self::Split,
        Self::Shell,
    ];
}

impl VerbKind {
    /// Every verb in the vocabulary, for censuses that must be total
    /// over it.
    pub const ALL: &'static [Self] = &[
        Self::Fillet,
        Self::Chamfer,
        Self::Extrude,
        Self::Revolve,
        Self::Boolean(BooleanOp::Union),
        Self::Boolean(BooleanOp::Intersect),
        Self::Boolean(BooleanOp::Subtract),
        Self::Split,
        Self::Shell,
    ];

    /// The verb's declared door: which run door answers it.
    #[must_use]
    pub fn arity(self) -> Arity {
        match self {
            Self::Fillet | Self::Chamfer => Arity::One,
            Self::Boolean(_) => Arity::Two,
            Self::Extrude | Self::Revolve => Arity::Profile,
            Self::Split => Arity::Split,
            Self::Shell => Arity::Shell,
        }
    }
}

impl<T: Real> Verb<T> {
    /// Which verb this is, scalar and reference payload dropped.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        match self {
            Self::Fillet { .. } => VerbKind::Fillet,
            Self::Chamfer { .. } => VerbKind::Chamfer,
            Self::Extrude { .. } => VerbKind::Extrude,
            Self::Revolve { .. } => VerbKind::Revolve,
            Self::Boolean { op, .. } => VerbKind::Boolean(*op),
            Self::Split { .. } => VerbKind::Split,
            Self::Shell { .. } => VerbKind::Shell,
        }
    }
}

#[cfg(test)]
mod all_census {
    use topo::BooleanOp;

    use super::{Arity, VerbKind};

    /// **[`Arity::ALL`] is every door row**, by the same construction
    /// as the vocabulary's census: the match is exhaustive, so a row
    /// added to the enum fails this file to compile until it is
    /// visited here, and visiting it means writing the new count,
    /// which reds until `ALL` has grown too. The other half — that
    /// every row in `ALL` has a DOOR — is `tests/run_door.rs`'s, where
    /// the doors are.
    #[test]
    fn all_is_every_door_row() {
        let rows = match Arity::One {
            Arity::One => 5,
            Arity::Two => 5,
            Arity::Profile => 5,
            Arity::Split => 5,
            Arity::Shell => 5,
        };
        for (i, row) in Arity::ALL.iter().enumerate() {
            assert!(
                !Arity::ALL[..i].contains(row),
                "{row:?} appears twice in Arity::ALL"
            );
        }
        assert_eq!(
            Arity::ALL.len(),
            rows,
            "Arity::ALL has drifted from the door rows — it holds {} rows, the enum has {rows}",
            Arity::ALL.len()
        );
    }

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
            VerbKind::Fillet => 9,
            VerbKind::Chamfer => 9,
            VerbKind::Extrude => 9,
            VerbKind::Revolve => 9,
            VerbKind::Boolean(op) => match op {
                BooleanOp::Union => 9,
                BooleanOp::Intersect => 9,
                BooleanOp::Subtract => 9,
            },
            VerbKind::Split => 9,
            VerbKind::Shell => 9,
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
            "VerbKind::ALL has drifted from the vocabulary — it holds {} rows, the vocabulary has {rows}",
            VerbKind::ALL.len()
        );
    }
}
