//! **Mates** — the A3 declaration node and its constructive solve
//! (ASSEMBLY-DESIGN A3/A11/A12; ASM-R2a spec D-1…D-5).
//!
//! A mate states one relation between two instances: which frames
//! coincide, with which axis senses, at which clocking. That statement
//! is BOTH the placement constraint and the contact declaration — one
//! node kind, no second vocabulary to keep synced (A3).
//!
//! # What a mate is, structurally
//!
//! A mate is a **leaf**: its `a`/`b` are instance-qualified stable
//! names, and name references are not DAG edges (the shipped D3
//! carve-out, which `Declare` established). What A12 adds on top is
//! the *reading* edge: the MEMBER instance each name's head resolves
//! through — the instantiate node itself, or a pattern's input
//! instance for an `Instance(i)`-qualified head (A11's member
//! vocabulary) — RECOMPUTED from the recipe at need
//! ([`reading_edges`]) and never stored beside it. The partitions
//! divide on that distinction — A9's relative-freedom components and
//! A11's placement clusters run over consuming ∪ reading edges, while
//! A10's coverage, ancestor-freedom, maintenance and product gather
//! run over consuming edges only. A mate is therefore an ordinary
//! non-body root: an isolated sink under consuming edges, listed like
//! any other, denoting no body, ignored by the gather.
//!
//! # What a mate says, geometrically
//!
//! [`Alignment`] carries two mate frames — one per side, in that
//! instance's OWN part coordinates — plus the primitive relating
//! them, the axis sense, and the clocking rider. Nothing here reads
//! geometry: the frames are authored data, so the whole solve is a
//! decided-predicate computation over the recipe (A11's "no geometry
//! inspection, no numerics beyond decided predicates").
//!
//! Each primitive pins the pair's relative pose to a COSET of an
//! SE(3) subgroup, and multiple mates on one pair fold by exact coset
//! intersection — [`coset`], whose table is the spec's binding one.
//!
//! # v1's admitted classes
//!
//! `class` is the KERNEL [`ContactClass`] (M9-1), re-exported rather
//! than re-minted, so a mate's declaration is already the currency the
//! boolean wrapper's records speak.
//!
//! **Which classes v1 admits is [`class_admission`], not a sentence
//! here.** A class passes TWO doors — the solve, which folds cosets,
//! and the assembly gate's mint, which needs a kernel record type to
//! carry the declaration at rest — and the two do not admit the same
//! set. The table is one function both doors read, so a class can
//! never be admitted at one and refused at the other without saying
//! so: `Rest` clears both, `Tangent` solves and then refuses typed at
//! the mint door (an at-rest contact has no witness edge for its
//! `CurveContact` — [`crate::AssemblyError::NoAtRestRecord`]), and
//! every later class — `Fit { gap }` when it lands — refuses at the
//! solve door, because a declared clearance changes what "coincide"
//! means and this unit solves coincidence only.

use crate::node::RecipeNodeId;
use geom_core::Tol;
use geom_core::linalg::frame::FrameError;
use geom_core::linalg::{Affine3, Point3, Vec3};
use geom_core::predicate::{BandError, Indeterminate};

pub mod coset;
pub mod solve;

pub use coset::{Coset, Subgroup};
pub use solve::{
    ClusterMaintenance, MateRole, SolvedPoses, clusters, gauge_of, reading_edges,
    relative_freedom_components, solve_document,
};

/// The kernel's contact vocabulary, re-exported (M9-1 PR-1: one enum,
/// defined lowest). A mate's class IS a contact declaration.
pub use topo::ContactClass;

/// Which side of a mate a diagnostic is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MateSide {
    /// The `a` reference.
    A,
    /// The `b` reference.
    B,
}

impl MateSide {
    /// The side's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }
}

/// One side's **mate frame**, in that instance's own part coordinates:
/// an origin, the primary axis (a planar rest's normal, a coaxial
/// mate's axis), and the clocking reference that fixes roll.
///
/// Authored data, not geometry read back: the solve is structural plus
/// decided predicates over exactly these numbers (A11). The frame is
/// built through [`geom_core::linalg::frame::point_at`] — U4B's frame
/// family, reused rather than reinvented — so a degenerate axis or a
/// reference on the axis line refuses through that ladder's typed
/// voice instead of silently producing a rank-deficient basis.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MateFrame {
    /// The frame's origin (part coordinates).
    pub origin: [f64; 3],
    /// The primary axis: a rest plane's normal, a coaxial axis. Need
    /// not be unit; only its direction is read.
    pub axis: [f64; 3],
    /// The clocking reference, fixing roll about `axis`. Need not be
    /// perpendicular to the axis; only its perpendicular part is read.
    pub reference: [f64; 3],
}

impl MateFrame {
    /// The rigid placement this frame denotes: local +Z is `axis`,
    /// local origin is `origin`, roll fixed by `reference` (U4B's
    /// `point_at` convention, verbatim).
    ///
    /// # Errors
    ///
    /// [`FrameError`] when the axis has no definite direction or the
    /// reference has no definite perpendicular offset from it.
    pub fn placement(&self, tol: Tol) -> Result<Affine3<f64>, FrameError> {
        let eye = Point3::new(self.origin[0], self.origin[1], self.origin[2]);
        let axis = Vec3::new(self.axis[0], self.axis[1], self.axis[2]);
        let reference = Vec3::new(self.reference[0], self.reference[1], self.reference[2]);
        geom_core::linalg::frame::point_at(eye, eye + axis, reference, tol)
    }
}

/// Which way the two sides' axes point at each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AxisSense {
    /// The axes point the same way (a shaft into a through-hole).
    Aligned,
    /// The axes point at each other (a rest: two outward normals
    /// meeting). This is what kills every π-flip ambiguity — the
    /// senses are authored, never inferred.
    Opposed,
}

/// The **mate primitive**: which coset of SE(3) this mate pins the
/// pair's relative pose to (A11 rule 1; the spec's coset table).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatePrimitive {
    /// The two mate frames coincide outright — residual trivial.
    FrameCoincidence,
    /// The two axes coincide as a LINE — residual cylindrical (rotation
    /// about + translation along it).
    Coaxial,
    /// `b`'s plane rests on `a`'s plane, displaced by `offset` along
    /// `a`'s axis — residual planar. A zero offset is the flush rest;
    /// a nonzero one is an authored standoff, not a `Fit`.
    PlanarRest {
        /// The signed standoff along `a`'s axis, in metres.
        offset: f64,
    },
    /// Clocking with NO carrying primitive. Representable precisely so
    /// it can be REFUSED: the table lacks the entry by design (a bare
    /// angular relation pins no coset of the shapes the table closes
    /// over), and an unrepresentable refusal is an untestable one.
    Clocking,
}

impl MatePrimitive {
    /// **Every LENGTH this primitive authors**, in metres — the one
    /// home for "what does a primitive carry that has a scale", read
    /// by the THREE doors that must account for one:
    ///
    /// - [`Alignment::lever_arm`], where a length left out LOOSENS the
    ///   mate's angular threshold (a smaller lever admits a bigger
    ///   angle for the same induced gap);
    /// - [`Alignment::is_finite`], where one left unchecked lets a
    ///   non-finite datum past the edit door;
    /// - the evaluation's content key (`eval`'s `feed_alignment`),
    ///   where one left unhashed makes two documents differing ONLY in
    ///   that length share a memo entry.
    ///
    /// All three are the unsound direction, and none of them has a row
    /// that goes red.
    ///
    /// The match is EXHAUSTIVE and the array is as wide as the widest
    /// variant, so a primitive that grows a length cannot arrive here
    /// unnoticed. **What that buys is a forced VISIT, not a correct
    /// answer** — `[None]` still compiles for a variant that does
    /// carry one, and nothing here can tell. What it does guarantee is
    /// that the answer is given ONCE, so the three readers cannot
    /// disagree about it; three hand-kept lists disagreeing is the
    /// state this replaced.
    ///
    /// The width lives in this list rather than in the type: a bare
    /// `Option<f64>` would say "at most one" in every reader's
    /// signature, and a two-length variant would then move all three.
    /// `None` means this variant carries fewer lengths than the widest
    /// does — never a zero standing in for a length that is not there.
    pub(crate) fn authored_lengths(self) -> [Option<f64>; 1] {
        match self {
            Self::PlanarRest { offset } => [Some(offset)],
            // Pure pose relations: their whole datum is the two mate
            // frames, whose scale is the origins the lever arm folds
            // in already.
            Self::FrameCoincidence | Self::Coaxial | Self::Clocking => [None],
        }
    }

    /// The primitive's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::FrameCoincidence => "frame-coincidence",
            Self::Coaxial => "coaxial",
            Self::PlanarRest { .. } => "planar-rest",
            Self::Clocking => "clocking",
        }
    }
}

/// The A3 alignment datum: which frames coincide, the axis senses, and
/// the clocking rider.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alignment {
    /// The `a` side's mate frame, in `a`'s part coordinates.
    pub a: MateFrame,
    /// The `b` side's mate frame, in `b`'s part coordinates.
    pub b: MateFrame,
    /// Which coset this mate pins.
    pub primitive: MatePrimitive,
    /// Which way the axes point at each other.
    pub sense: AxisSense,
    /// The clocking rider: the signed angle (radians) from `a`'s
    /// reference to `b`'s about the shared axis. A RIDER, never a
    /// primitive — on [`MatePrimitive::Coaxial`] it cuts the residual
    /// to prismatic along the axis; on
    /// [`MatePrimitive::FrameCoincidence`] it is redundant-or-
    /// contradictory and gets decided; on a planar rest the table has
    /// no entry and the solve refuses typed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clocking: Option<f64>,
}

impl Alignment {
    /// The **lever arm** this mate's angular decisions turn on: the
    /// largest distance in its own authored data over which an angular
    /// error accumulates into a gap (D4 ¶1 — an angle only means
    /// something through the displacement it induces at a length
    /// scale, and the scale is named at the call site).
    ///
    /// The unit-metre floor is what keeps a mate authored AT the origin
    /// from claiming an arbitrarily tight angular threshold: with no
    /// length in the datum there is no lever, and a metre is the
    /// session box's own order of magnitude (D4 ¶4).
    pub fn lever_arm(&self) -> f64 {
        let norm = |v: [f64; 3]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        self.primitive
            .authored_lengths()
            .into_iter()
            .flatten()
            .fold(
                norm(self.a.origin).max(norm(self.b.origin)).max(1.0),
                |lever, length| lever.max(length.abs()),
            )
    }

    /// Whether every authored coordinate is finite — the edit door's
    /// admission test, the placement registry's rule applied one level
    /// out (a non-finite alignment could never decide anything).
    pub fn is_finite(&self) -> bool {
        let finite = |v: &[f64; 3]| v.iter().all(|x| x.is_finite());
        let frame = |f: &MateFrame| finite(&f.origin) && finite(&f.axis) && finite(&f.reference);
        frame(&self.a)
            && frame(&self.b)
            && self.clocking.is_none_or(f64::is_finite)
            && self
                .primitive
                .authored_lengths()
                .into_iter()
                .flatten()
                .all(f64::is_finite)
    }
}

/// **The A11 rule-4 recourse**, verbatim: what an author does about an
/// under-determined tree mate.
pub const UNDER_RECOURSE: &str = "add the complementary mate, or delete the mate if free relative \
                                  motion was intended";

/// **The v1 class restriction, named.** `Fit` is specified and not
/// built; a mate cannot declare a designed clearance until the kernel
/// variant lands with its first consumer, and AQ6 is where the
/// cross-document detail is still open.
pub const CLASS_DEFERRAL: &str = "v1 mates SOLVE Rest and Tangent and ASSEMBLE Rest alone; the \
                                  cross-document detail of a designed clearance is undischarged";

/// **How far a contact class gets in v1** — the whole class policy as
/// a value, read by both doors that enforce it.
///
/// The two doors want different things of a class: the solve needs a
/// coset the alignment table can fold, the assembly gate's mint needs
/// a KERNEL RECORD TYPE that can carry the declaration at rest. A
/// class can satisfy the first and not the second, so the admitted
/// sets differ — and the gap is stated here, once, rather than
/// asserted separately by each door's own match (which is how they
/// drift apart, and how a door comes to advertise what it cannot
/// execute).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassAdmission {
    /// Both doors: the solve folds it, and [`crate::assemble`] mints
    /// it into the product's record set as the kernel's own record
    /// type. Whether the census then CERTIFIES that record is the
    /// census's verdict, not this table's.
    Mints,
    /// The solve door only. No kernel record carries this class at
    /// rest, so [`crate::assemble`] refuses
    /// [`crate::AssemblyError::NoAtRestRecord`] naming the mate — a
    /// solved placement that cannot be verified at rest, never a
    /// record minted with an invented witness.
    NoAtRestRecord {
        /// Why THIS class has none, in its own terms. Carried here so
        /// the mint door's message is the table's, never one class's
        /// reason rendered confidently over another's refusal.
        why: &'static str,
    },
    /// Neither: outside v1's vocabulary, so the solve door refuses
    /// [`MateFault::ClassNotAdmitted`]. Reaching the mint door means a
    /// mate of this class was live, which the solve door does not
    /// permit — so the mint refuses it too, with the deferral, rather
    /// than assuming the chain held.
    NotAdmitted,
}

impl ClassAdmission {
    /// Why the assembly gate carries nothing at rest for this class.
    ///
    /// Total, so the mint door never has to choose a sentence: a class
    /// that mints has no such reason and says so.
    pub fn no_record_reason(self) -> &'static str {
        match self {
            Self::Mints => "the class mints at rest",
            Self::NoAtRestRecord { why } => why,
            Self::NotAdmitted => CLASS_DEFERRAL,
        }
    }
}

/// The class table itself ([`ClassAdmission`]).
///
/// INVARIANT: every door that ENFORCES the class policy reads it
/// here, so the admitted sets cannot drift apart.
/// `ContactClass` is `#[non_exhaustive]`, so a class the kernel grows
/// arrives in the wildcard arm as [`ClassAdmission::NotAdmitted`] —
/// deferred by default, admitted only by an edit HERE that both doors
/// then obey.
pub fn class_admission(class: ContactClass) -> ClassAdmission {
    match class {
        // Face granularity: a rest between two placed faces IS a
        // `PatchContact` (M9-1).
        ContactClass::Rest => ClassAdmission::Mints,
        ContactClass::Tangent => ClassAdmission::NoAtRestRecord {
            why: "a tangency's record is a `CurveContact` keyed by the witness EDGE whose \
                  carrier is the contact locus, and an assembly at rest has none — nothing \
                  zipped the instances together, which is what \"at rest, not a boolean\" means",
        },
        _ => ClassAdmission::NotAdmitted,
    }
}

/// A typed mate refusal (D9: fail loud, never a guess). Every arm names
/// its subject — the mate, the pair, the predicate, or the residual.
#[derive(Debug, Clone, PartialEq)]
pub enum MateFault {
    /// A mate frame's authored data has no definite frame.
    Frame {
        /// The mate whose datum refused.
        mate: RecipeNodeId,
        /// Which side.
        side: MateSide,
        /// The frame ladder's own refusal.
        error: FrameError,
    },
    /// A class outside v1's admitted vocabulary — `Fit { gap }` today,
    /// any later addition tomorrow. The message carries the kernel's
    /// own deferral sentence VERBATIM ([`topo::FIT_DEFERRAL`]) rather
    /// than a paraphrase of it.
    ClassNotAdmitted {
        /// The mate that declared it.
        mate: RecipeNodeId,
    },
    /// The coset table has no entry for this combination, and the
    /// refuse-any-missing-pair rule applies.
    TableLacks {
        /// The mate whose datum has no entry.
        mate: RecipeNodeId,
        /// What was asked for, in the table's own words.
        what: &'static str,
    },
    /// A case-split predicate landed in the ambiguity band — the typed
    /// escalation, never a silent pick.
    Indeterminate {
        /// The mate being folded when it fired.
        mate: RecipeNodeId,
        /// The predicate's diagnostics (it names itself).
        diag: Box<Indeterminate>,
    },
    /// The run's tolerance could not yield a band.
    Band {
        /// The band constructor's refusal.
        error: BandError,
    },
    /// The pair's mates intersect to the EMPTY coset (A11 rule 1's
    /// CONTRADICTORY): names both mates, the predicate that failed,
    /// and the measured clash.
    Contradictory {
        /// The mate already folded.
        held: RecipeNodeId,
        /// The mate whose intersection died against it.
        added: RecipeNodeId,
        /// The predicate that decided against them.
        predicate: &'static str,
        /// The measured clash, in metres (the margin that should have
        /// been zero and was not).
        clash: f64,
    },
    /// A tree mate left a positive-dimensional residual (A11 rule 4's
    /// UNDER): names the pair, the residual subgroup, and its
    /// parameters.
    Under {
        /// The tree mate that failed to determine.
        mate: RecipeNodeId,
        /// The instance the tree was extending from.
        parent: RecipeNodeId,
        /// The instance it failed to place.
        child: RecipeNodeId,
        /// What survived the fold.
        residual: Subgroup,
    },
    /// A mate's name head does not resolve to a live MEMBER — a live
    /// instantiate node, or a pattern-placed instance (the `Pattern`
    /// node with its `Instance(i)` qualifier at a derivable pose;
    /// A11's member vocabulary) — N5's dangling reference. It
    /// contributes no reading edge; the solve refuses typed rather
    /// than pretending the mate is absent.
    DanglingHead {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side dangles.
        side: MateSide,
        /// The head the name claims.
        head: RecipeNodeId,
    },
    /// A mate names ONE instance on both sides. A pair is two
    /// instances; a self-mate constrains nothing and is a recipe
    /// mistake, refused rather than folded into a tautology.
    SelfMate {
        /// The mate.
        mate: RecipeNodeId,
        /// The instance it names twice.
        instance: RecipeNodeId,
    },
}

impl core::fmt::Display for MateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Frame { mate, side, error } => write!(
                f,
                "mate {}'s {} frame has no definite placement: {error}",
                mate.0,
                side.name()
            ),
            Self::ClassNotAdmitted { mate } => write!(
                f,
                "mate {}'s contact class is not admitted in v1 — {} ({CLASS_DEFERRAL})",
                mate.0,
                topo::FIT_DEFERRAL
            ),
            Self::TableLacks { mate, what } => write!(
                f,
                "mate {}: the coset table has no entry for {what} — the table refuses every pair \
                 it lacks rather than inventing one",
                mate.0
            ),
            Self::Indeterminate { mate, diag } => write!(
                f,
                "mate {}: a case split could not be decided — {}",
                mate.0,
                diag.payload()
            ),
            Self::Band { error } => write!(f, "the mate solve could not build a band: {error}"),
            Self::Contradictory {
                held,
                added,
                predicate,
                clash,
            } => write!(
                f,
                "mates {} and {} cannot both hold: predicate `{predicate}` measured a clash of \
                 {clash} m where their cosets would have had to meet",
                held.0, added.0
            ),
            Self::Under {
                mate,
                parent,
                child,
                residual,
            } => write!(
                f,
                "mate {} does not determine instance {} from instance {}: {} survives — \
                 {UNDER_RECOURSE}",
                mate.0,
                child.0,
                parent.0,
                residual.describe()
            ),
            Self::DanglingHead { mate, side, head } => write!(
                f,
                "mate {}'s {} reference resolves through node {}, which does not resolve to a \
                 live member (an instance, or a pattern-placed instance) — rebind it",
                mate.0,
                side.name(),
                head.0
            ),
            Self::SelfMate { mate, instance } => write!(
                f,
                "mate {} names one member on both sides (it stands on instance {}); a mate \
                 relates a PAIR",
                mate.0, instance.0
            ),
        }
    }
}

impl core::error::Error for MateFault {}
