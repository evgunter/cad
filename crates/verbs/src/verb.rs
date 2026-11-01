//! **The kernel's verbs, one payload type per run door, and the closed
//! vocabulary they project onto** ([`VerbKind`]).
//!
//! Each payload holds one operation's parameters as data. Scalars sit
//! at `T`; entity references are arena keys, resolved by whoever built
//! the value. The OPERANDs are not here — they are borrowed at the run
//! doors ([`Blend::run`], [`Boolean::run_pair`], [`Members::run_members`],
//! [`Sweep::run_profile`], [`Split::run_split`], [`Shell::run_shell`]), because an operand is
//! not a parameter of the operation, it is the thing operated on, and
//! putting it in the payload would make every declaration own a clone
//! of it. That holds for the sweeps' validated PROFILE exactly as it
//! holds for a body: it is borrowed at the door and never stored.
//!
//! **Each door is a method of its own payload type**, so a verb cannot
//! be handed to a door that does not answer it: there is no mismatch to
//! refuse. What an operand IS, and how many, is the door's signature,
//! so the payload never smuggles an operand count either.
//!
//! **Closed, through [`VerbKind`]**: every payload type's `kind` is an
//! exhaustive match onto it, and every commitment keyed on the
//! vocabulary matches `VerbKind` with no wildcard arm (D3), so a verb
//! added to it breaks every commitment site at compile time rather than
//! silently defaulting.
//!
//! **`T: Real`**: the revolve's axis is a point and a direction in the
//! sketch plane, and `geom-core`'s vectors are defined for a lane
//! scalar only. Every lane this workspace runs is one, so the bound
//! narrows nothing a caller could have wanted.
//!
//! **No equality of any kind**, and the reason is the payload rather
//! than a preference. A verb's payload holds the operation's own
//! values, and once one of those is GEOMETRY — the revolve's axis, a
//! point and a direction in sketch coordinates — the declaration
//! inherits `geom-core`'s stance on comparing geometry: `Point2` and
//! `Vec2` carry no `PartialEq` at all, deliberately, because deciding
//! whether two coordinates are the same is what the classification
//! band is for and never what `==` is for. Synthesising one here
//! (component-wise, at `T: PartialEq`) would put exactly that
//! comparison in the kernel's verb vocabulary, so these types do
//! without: nothing in the tree compares verbs, and the day something
//! must, what it wants is a decided predicate at a band, not a derive.

use core::fmt;

use geom_core::Real;
use sweep::{Revolution, RevolveAxis};
use topo::{BooleanInputs, BooleanOp, EdgeKey, FaceKey, MemberInputs, NaryOp, SplitPlane};

/// **The blend pair** — the verbs [`Blend::run`] answers.
#[derive(Clone, Debug)]
pub enum Blend<T: Real> {
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

/// **A regularized boolean over two operand bodies** — the verb
/// [`Boolean::run_pair`] answers.
///
/// The payload is the boolean door's own parameter list with the
/// operands and the run witnesses removed: which regularized op, and
/// its inputs ([`BooleanInputs`]) — arena keys into the two operands,
/// resolved by whoever built the value (the recipe layer's name
/// resolution never enters this crate). The candidate-sweep strategy is
/// NOT here: it is a property of the run, not of the operation (both
/// strategies produce bit-identical results), so it comes in at
/// [`Boolean::run_pair`] beside the tolerance witness.
#[derive(Clone, Debug)]
pub struct Boolean {
    /// The regularized set operation.
    pub op: BooleanOp,
    /// The operands' carried records, in operand arena keys, boxed so
    /// the verb stays one small value whatever they hold.
    pub inputs: Box<BooleanInputs>,
}

/// **A union or an intersect over a list of member bodies** — the verb
/// [`Members::run_members`] answers: the kernel's n-ary operation
/// (`topo::nary_boolean`), whose judgements and fold steps are the
/// boolean's own.
///
/// The payload is that door's parameter list with the member bodies
/// and the run witnesses removed: which operation, and what each member
/// carries ([`MemberInputs`]: its records and its rank), in list order.
/// Its kind is the boolean's ([`VerbKind::Boolean`]): a union of a list
/// is the union verb, answered at a door of its own.
#[derive(Clone, Debug)]
pub struct Members {
    /// The operation.
    pub op: NaryOp,
    /// Each member's records and rank, in list order.
    pub members: Vec<MemberInputs>,
    /// The member pairs certified apart, which are not judged.
    pub apart: topo::Apart,
}

/// **The two sweeps of a profile** — the verbs [`Sweep::run_profile`]
/// answers.
#[derive(Clone, Debug)]
pub enum Sweep<T: Real> {
    /// Extrudes the operand profile along its sketch normal.
    ///
    /// The payload is the door's parameter list minus the operand:
    /// the depth and the side it goes toward. The door's second extrusion form —
    /// an explicit world VECTOR — is deliberately absent: no recipe
    /// spells one, so a variant for it would be an arm every
    /// commitment on this vocabulary (content tag, wire spelling,
    /// Python constructor, viewer label) had to name and no document
    /// could ever reach.
    Extrude {
        /// The depth, a size (`sweep::Extrusion::Distance`).
        distance: T,
        /// Which side of the profile plane the depth goes toward.
        side: sweep::ExtrudeSide,
    },
    /// Revolves the operand profile about an axis written in its own
    /// sketch plane.
    ///
    /// Both fields are the kernel door's own values. The `Revolution`
    /// in particular is CLASSIFIED, not raw: whether an authored angle
    /// is the exact full turn or a partial one is a decided predicate
    /// at the document layer's own funnel site, and its escalation is
    /// a document-layer refusal — so what reaches the payload is the
    /// classified value, exactly as the boolean's inputs reach it
    /// already resolved to arena keys.
    Revolve {
        /// The axis of revolution, in sketch-plane coordinates.
        axis: RevolveAxis<T>,
        /// How far to revolve.
        revolution: Revolution<T>,
    },
}

/// **Parts the operand body by a plane into its two sides** — the verb
/// [`Split::run_split`] answers.
///
/// The plane is a kernel VALUE in the payload — a point and a unit
/// normal — exactly as the boolean carries its inputs already
/// resolved: whatever document object the plane was read off (a datum
/// node, upstairs) is the recipe layer's business, and what reaches
/// this crate is the split door's own `SplitPlane`. The body is the one
/// operand and stays out of the payload like every other. What sets
/// this verb apart is not what it takes but what it gives back — two
/// sides, each a body or the typed empty — which is why it answers its
/// own door.
#[derive(Clone, Debug)]
pub struct Split<T: Real> {
    /// The parting plane; the side its normal points to is ABOVE.
    pub plane: SplitPlane<T>,
}

/// **Hollows the operand body to a wall of the given thickness**,
/// optionally opening designated faces into rims — the verb
/// [`Shell::run_shell`] answers.
///
/// **One verb, not two.** The kernel spells the sealed hollow and the
/// opened one as two functions (`topo::shell`, `topo::shell_open`), and
/// the second's own contract says an empty designation IS the first —
/// so a shell with an empty `open` is `shell`, and a second verb would
/// be a name for a value of this one's payload. That is the boolean's
/// rule read the other way: three kernel doors that differ in WHAT THEY
/// DO are three names, while one door reached with an empty list is one
/// name.
///
/// The designation is arena keys, resolved by whoever built the value,
/// exactly as the blends' edges are.
#[derive(Clone, Debug)]
pub struct Shell<T: Real> {
    /// The wall thickness in metres — a magnitude; each face's inward
    /// direction is its own orientation's.
    pub thickness: T,
    /// The operand faces to open into rims. Empty is the sealed hollow.
    pub open: Vec<FaceKey>,
}

/// **The kernel's verb vocabulary with the scalar and reference payload
/// dropped** — the closed set of operation names, addressable where no
/// verb value exists yet.
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
/// This is a projection, not a twin: each payload type's `kind` is
/// where the mapping is written, each is exhaustive, and a new verb therefore cannot compile until it has a
/// `VerbKind` and every match over `VerbKind` has been visited.
///
/// `sweep::blend::BlendKind` is a different thing that looks like this
/// one: it is the label a blend REFUSAL carries, enumerating the two
/// blend doors, and it lives in `sweep` because a `sweep` refusal
/// carries it. It cannot serve here — the kernel's verb vocabulary grows past
/// the blend pair into ops `sweep` must not name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerbKind {
    /// [`Blend::Fillet`].
    Fillet,
    /// [`Blend::Chamfer`].
    Chamfer,
    /// [`Sweep::Extrude`].
    Extrude,
    /// [`Sweep::Revolve`].
    Revolve,
    /// [`Boolean`] running the named regularized op.
    Boolean(BooleanOp),
    /// [`Split`].
    Split,
    /// [`Shell`]. It shipped kernel-first, ahead of its document
    /// node, and every commitment keyed on this vocabulary said what it
    /// meant for a name no document could reach rather than skipping
    /// it (`editor-core`'s content tag is an `Option` for that reason);
    /// `Node::Shell` builds it now.
    Shell,
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
}

/// The verb's own name, which is the word a sentence about a verb
/// writes.
///
/// Beside [`VerbKind::ALL`] because it is the same census read for a
/// different purpose: `ALL` says what the vocabulary holds, this says
/// what each member is called. The match is exhaustive with no
/// wildcard — over the vocabulary AND over the boolean's op, since
/// each op is its own verb — so a verb the enum gains has no word
/// until someone writes one, and the words cannot fall behind the
/// vocabulary the way a lookup table could. What they CAN do is
/// collide, and `ALL` is what guards that
/// (`the_vocabulary_says_each_verb_by_one_unshared_name`).
///
/// A fieldless verb's word is its variant identifier: these are the
/// doors' own names and the identifier is what a caller who reached
/// one is told. **The boolean rows are where that stops being
/// `Debug`**: the kernel's production doors there are `union`,
/// `intersect` and `subtract`, three verbs sharing one payload shape,
/// so each says its op — `Boolean(Union)` is the enum's coordinate for
/// the verb and names a door that does not exist.
impl fmt::Display for VerbKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.noun())
    }
}

impl VerbKind {
    /// The verb's word, the one its `Display` writes, for a sentence
    /// that says it as a node's kind (`Subtract 1669`).
    #[must_use]
    pub const fn noun(self) -> &'static str {
        match self {
            Self::Fillet => "Fillet",
            Self::Chamfer => "Chamfer",
            Self::Extrude => "Extrude",
            Self::Revolve => "Revolve",
            Self::Boolean(BooleanOp::Union) => "Union",
            Self::Boolean(BooleanOp::Intersect) => "Intersect",
            Self::Boolean(BooleanOp::Subtract) => "Subtract",
            Self::Split => "Split",
            Self::Shell => "Shell",
        }
    }
}

impl<T: Real> Blend<T> {
    /// Which blend this is.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        match self {
            Self::Fillet { .. } => VerbKind::Fillet,
            Self::Chamfer { .. } => VerbKind::Chamfer,
        }
    }
}

impl Boolean {
    /// Which boolean this is: its op is its name.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        VerbKind::Boolean(self.op)
    }
}

impl Members {
    /// Which boolean this is: its operation's op is its name.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        VerbKind::Boolean(self.op.op())
    }
}

impl<T: Real> Sweep<T> {
    /// Which sweep this is.
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        match self {
            Self::Extrude { .. } => VerbKind::Extrude,
            Self::Revolve { .. } => VerbKind::Revolve,
        }
    }
}

impl<T: Real> Split<T> {
    /// [`VerbKind::Split`].
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        VerbKind::Split
    }
}

impl<T: Real> Shell<T> {
    /// [`VerbKind::Shell`].
    #[must_use]
    pub fn kind(&self) -> VerbKind {
        VerbKind::Shell
    }
}

#[cfg(test)]
mod all_census {
    use topo::BooleanOp;

    use super::VerbKind;

    /// **[`VerbKind::ALL`] is the WHOLE vocabulary**, pinned against a
    /// compile-time visit rather than reviewed.
    ///
    /// The precedent this list cites, `profile::Verb::ALL`, is the
    /// SKETCH program's `Verb` — and is macro-generated, so it cannot drift; this one is
    /// hand-written, so it needs the guard the macro would otherwise
    /// have been. The match
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

    /// **Every verb says itself by one word no other verb says**,
    /// anchored on [`VerbKind::ALL`] so a verb the vocabulary gains
    /// arrives pinned.
    ///
    /// The `Display` arms are an exhaustive match, so they cannot fall
    /// BEHIND the vocabulary — a verb without a word does not compile.
    /// What a hand-written word can still do is collide with another
    /// row's (a literal copied onto a new row), and then the refusal
    /// naming it cannot say which verb it refused. `ALL` is the census
    /// that sees the collision.
    ///
    /// The second half is what tells the word apart from `Debug` at
    /// run time. A fieldless verb's word IS its variant identifier, by
    /// intent, so for those rows the two renderings agree and this
    /// file could not tell which one ran. The boolean rows can:
    /// `VerbKind::Boolean(BooleanOp::Union)` debugs as `Boolean(Union)`
    /// — the enum's coordinate for the verb, naming a `Boolean` door
    /// the kernel does not have — and says `Union`, the production
    /// door's own name. A `Display` deleted, forwarded to `Debug`, or
    /// written to spell the payload reds here.
    #[test]
    fn the_vocabulary_says_each_verb_by_one_unshared_name() {
        let mut said: Vec<(String, VerbKind)> = Vec::new();
        for kind in VerbKind::ALL {
            let word = kind.to_string();
            let shared = said.iter().find(|(w, _)| *w == word).map(|(_, o)| *o);
            assert!(
                shared.is_none(),
                "{kind:?} shares its word \"{word}\" with {shared:?} — a refusal naming \
                 that word cannot say which verb it refused"
            );
            said.push((word.clone(), *kind));
            let carries_an_op = match kind {
                VerbKind::Fillet
                | VerbKind::Chamfer
                | VerbKind::Extrude
                | VerbKind::Revolve
                | VerbKind::Split
                | VerbKind::Shell => false,
                VerbKind::Boolean(_) => true,
            };
            if carries_an_op {
                assert!(
                    !word.contains('('),
                    "{kind:?} says \"{word}\", which spells the enum's coordinate for the \
                     verb; the word is the production door's own name (union, intersect, \
                     subtract)"
                );
            } else {
                assert_eq!(
                    word,
                    format!("{kind:?}"),
                    "{kind:?} says \"{word}\"; a fieldless verb's word is the door's own \
                     name, which is the identifier"
                );
            }
        }
    }
}
