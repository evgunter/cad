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
//! A mate is a **leaf**: its `a`/`b` are `SitedRef`s — an
//! instance-qualified stable name plus the OPERAND node it is read at
//! — and neither half is a DAG edge (the shipped D3 carve-out, which
//! `Declare` established, extended to the node half by A12's reading
//! rule). What A12 adds on top is the *reading* edge: the MEMBER
//! instance each reference's OPERAND resolves through, walking down
//! to the minting instance past any number of transforms and at most
//! one pattern level (A11's member vocabulary) — RECOMPUTED from the
//! recipe at need ([`reading_edges`]) and never stored beside it. The
//! partitions
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
    ClusterMaintenance, MateRole, Member, SolvedPoses, clusters, gauge_of, member_of,
    reading_edges, relative_freedom_components, solve_document,
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
    /// **The datum's own extent wherever it has one** (ERROR-DESIGN
    /// E3's amendment, ratified at revision E12), and the metre ONLY
    /// where it has none.
    ///
    /// The amendment's complaint was that `max(extent, 1 m)` made the
    /// constant the operative lever for every model smaller than a
    /// metre: a 10 mm datum's tilt was priced across a metre it does not
    /// span, and the separation never entered. That half is fixed — a
    /// datum with any authored length is levered by ITS OWN extent, at
    /// whatever scale the author works, and no absolute constant
    /// participates.
    ///
    /// **The other half cannot be taken here, and the reason is the
    /// data this type carries.** `eval::measure`'s sibling arm has no
    /// floor at all because its operands are FACES and a validated face
    /// has positive extent by construction. A mate's operands are the
    /// mated PARTS, which also have extent — but an [`Alignment`]
    /// carries only the authored DATUM, and a datum authored at the
    /// origin with no length (`Coaxial` on two frames at their parts'
    /// origins is the common spelling) has none. Levering that at zero
    /// would price every tilt at zero and read every pair as parallel:
    /// an answer, in the direction that reports rather than refuses.
    /// MEASURED, removing the floor without the parts' extent: twelve
    /// rows of `asm_r2a_mate_solve` turn into refusals, every one of
    /// them a document a user may legitimately author.
    ///
    /// So the metre survives exactly where D4 ¶4 put it — as the
    /// session box's own order of magnitude, for a datum that names no
    /// scale at all — and it is named as that rather than as a lever.
    /// The full amendment needs the mated parts' extent to reach this
    /// door, which is issue `mate-lever-needs-the-parts-extent`.
    ///
    /// # The gap between "no scale" and "a usable scale"
    ///
    /// The first shipped form of this function was
    /// `if extent > 0.0 { extent } else { 1.0 }`, and a reviewer took it
    /// apart in one line: a datum at the origin is levered at 1 m, and a
    /// datum ONE NANOMETRE from the origin is levered at 1e-9 m. The
    /// second is the failure the paragraph above warns about — a lever
    /// that prices every tilt at ~zero and reads every pair as parallel
    /// — sitting a nanometre away from the case the metre is there to
    /// cover. A bit-exact test against `0.0` was choosing between two
    /// answers nine orders apart.
    ///
    /// Three cases now, and the middle one is a REFUSAL rather than a
    /// number:
    ///
    /// * **no scale named at all** ([`Self::names_a_scale`]) — the
    ///   session box's [`SESSION_SCALE`]. This is D4 ¶4's arm and it is
    ///   the case `Coaxial` on two origin frames lands in, which is a
    ///   spelling users author constantly.
    /// * **a scale named, at or above [`MIN_LEVER_ARM`]** — that scale.
    /// * **a scale named BELOW it** — [`LeverRefusal::DatumTooSmall`].
    ///   The author named a length, so the metre is not theirs to
    ///   borrow; and the length they named cannot decide anything,
    ///   because a lever of `L` makes the smallest decidable tilt
    ///   `ε/L`, which at `L = 1 nm` and ε = 1e-9 is a whole radian. A
    ///   verdict there is not wrong, it is vacuous, and reporting a
    ///   vacuous parallel is the direction this kernel refuses.
    ///
    /// The branch is on WHAT WAS AUTHORED, not on a computed magnitude
    /// against zero: `names_a_scale` asks the recipe whether a length or
    /// a non-origin coordinate was written down.
    ///
    /// # Errors
    ///
    /// [`LeverRefusal::DatumTooSmall`] — see above.
    ///
    /// [`arm`]: crate::eval::measure
    pub fn lever_arm(&self) -> Result<f64, LeverRefusal> {
        if !self.names_a_scale() {
            return Ok(SESSION_SCALE);
        }
        let extent = self.authored_extent();
        if extent < MIN_LEVER_ARM {
            return Err(LeverRefusal::DatumTooSmall {
                extent,
                floor: MIN_LEVER_ARM,
            });
        }
        Ok(extent)
    }

    /// The largest length this alignment names — its authored lengths
    /// and its frames' distances from the origin.
    fn authored_extent(&self) -> f64 {
        let norm = |v: [f64; 3]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        self.primitive
            .authored_lengths()
            .into_iter()
            .flatten()
            .fold(
                norm(self.a.origin).max(norm(self.b.origin)),
                |lever, length| lever.max(length.abs()),
            )
    }

    /// **Whether this alignment names a length scale at all** — a
    /// question about the RECIPE, asked of what was authored rather than
    /// of a computed magnitude against zero.
    ///
    /// A primitive with an authored length names one. A frame placed
    /// away from its part's origin names one. `Coaxial` on two frames
    /// both at the origin names none, and that is not a degenerate case
    /// — it is how an axis-to-axis mate is ordinarily written.
    fn names_a_scale(&self) -> bool {
        self.primitive
            .authored_lengths()
            .into_iter()
            .flatten()
            .any(|l| l != 0.0)
            || self.a.origin != [0.0; 3]
            || self.b.origin != [0.0; 3]
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

/// **The order of magnitude a datum that names NO scale is read at** —
/// D4 ¶4's session box, not a lever.
///
/// One metre, and it is a statement about the working envelope of a CAD
/// session rather than about any part: a mate written as two coincident
/// origin frames says where things meet and nothing about how big they
/// are, so the only honest scale left is the session's own.
pub const SESSION_SCALE: f64 = 1.0;

/// **The smallest datum extent a parallelism verdict can be levered
/// over** — one micron.
///
/// Below it the verdict is vacuous rather than wrong: a lever of `L`
/// makes the smallest decidable tilt `ε / L`, so at 1 nm and the default
/// ε the smallest tilt this door could call non-parallel is about a
/// radian, and everything under that reads parallel. A micron is three
/// decades above the default ε and below any datum offset a drawing
/// means, so nothing authorable falls in the gap without deserving to.
///
/// It is NOT an ε and it is not compared against a margin: it is a
/// precondition on the ARM, checked before any predicate runs. No
/// decision is made here.
pub const MIN_LEVER_ARM: f64 = 1.0e-6;

/// Why a lever arm could not be formed ([`Alignment::lever_arm`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeverRefusal {
    /// The datum names a length scale, and the scale it names is too
    /// small to decide a tilt over ([`MIN_LEVER_ARM`]).
    DatumTooSmall {
        /// The extent the datum names.
        extent: f64,
        /// The floor it is under.
        floor: f64,
    },
}

impl core::fmt::Display for LeverRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DatumTooSmall { extent, floor } => write!(
                f,
                "this mate's datum names a scale of {extent:e} m, under the {floor:e} m floor a \
                 parallelism verdict can be levered over: at that arm the smallest tilt the \
                 predicate could call non-parallel is about eps/{extent:e} radians, so every \
                 tilt would read parallel. Author the datum at the scale the parts actually \
                 have, or place the frames where the feature is"
            ),
        }
    }
}

/// A typed mate refusal (D9: fail loud, never a guess). Every arm names
/// its subject — the mate, the pair, the predicate, the residual, or
/// the two documents a mispaired read named.
#[derive(Debug, Clone, PartialEq)]
pub enum MateFault {
    /// The solve is a solve of ANOTHER document (DI3): one of the two
    /// arms whose subject is not a mate at all — `Band`, whose subject
    /// is the constructor, is the other. Raised by
    /// [`crate::SolvedPoses::placement`], never recorded against a
    /// node — a solve records no fault about a document it never read,
    /// which is why `viewer::tree` blames no row for it.
    PosesOfAnotherDocument {
        /// The document whose placement was asked for.
        expected: crate::ident::DocumentId,
        /// The document the solve is of.
        found: crate::ident::DocumentId,
    },
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
        /// The mate already folded. **Equal to `added` when one mate
        /// contradicts ITSELF**: a mate whose own datum and rider admit
        /// no common pose dies by the same rule, and naming it on both
        /// sides is how that shape reaches this variant.
        held: RecipeNodeId,
        /// The mate whose intersection died against it.
        added: RecipeNodeId,
        /// The predicate that decided against them.
        predicate: &'static str,
        /// The measured clash, in metres: the margin that should have
        /// been zero and was not, or — for the predicate that decides
        /// the empty intersection STRUCTURALLY — no measurement at all.
        /// Which of those it is, is settled by `predicate`, never by
        /// reading this number.
        clash: f64,
        /// The lever, when the predicate measured a ROLL and an arm
        /// carried it to `clash`: `(radians, arm in metres)`, in that
        /// order, whose product is the deviation. `None` when the
        /// predicate measured its margin without a lever.
        ///
        /// The arm is the solve's own scale surrogate
        /// ([`Alignment::lever_arm`]) — the larger of the two frame
        /// origins' distances and the authored lengths, floored at one
        /// metre — and NOT a contact feature, so a message that names
        /// it is naming that scale and nothing in the model.
        lever: Option<(f64, f64)>,
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
    /// A mate's reference does not resolve to a live MEMBER — the
    /// walk from its operand down to its name's head found no live
    /// instantiate node, reached through transforms and at most one
    /// pattern level at a derivable pose (A11's member vocabulary) —
    /// N5's dangling reference. It contributes no reading edge; the
    /// solve refuses typed rather than pretending the mate is absent.
    DanglingHead {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side dangles.
        side: MateSide,
        /// **The node at which the reference resolves to no member**:
        /// where the walk stopped, which is a stranded operand when
        /// the operand is the broken half and the first node outside
        /// the vocabulary otherwise. Not in general the reference's
        /// own head, which is often live and fine.
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
    /// The mate's datum names a length scale too small to lever a
    /// parallelism verdict over ([`Alignment::lever_arm`]).
    Unleverable {
        /// The mate.
        mate: RecipeNodeId,
        /// Why.
        refusal: LeverRefusal,
    },
}

/// The predicate that decides the EMPTY intersection. It is the one
/// name in the membership vocabulary that reports no measurement — the
/// empty set holds nothing and no margin decides that — so it is a
/// shared constant rather than a literal at each site, and every door
/// that needs to know reads THIS rather than inspecting a margin.
pub(crate) const MATE_MEMBER_EMPTY: &str = "mate_member_empty";

impl core::fmt::Display for MateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PosesOfAnotherDocument { expected, found } => write!(
                f,
                "the solve is of document {found}, not of document {expected}"
            ),
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
                lever,
            } => {
                // One mate named on BOTH sides is a mate contradicting
                // itself, and "mates 6 and 6" reads as an indexing
                // fault rather than as the shape the payload states.
                if held == added {
                    write!(
                        f,
                        "mate {} contradicts itself — the constraints it declares admit no \
                         common pose",
                        held.0
                    )?;
                } else {
                    write!(f, "mates {} and {} cannot both hold", held.0, added.0)?;
                }
                write!(f, ": predicate `{predicate}` ")?;
                // WHETHER there is a measurement to report is the
                // predicate's fact, settled where the refusal is
                // raised. The margin's value never stands in for it:
                // reading a sentinel back out of a number makes every
                // other non-finite margin claim to be this case.
                if *predicate == MATE_MEMBER_EMPTY {
                    return write!(
                        f,
                        "found the cosets meet in the empty set — a structural refusal, with no \
                         margin to measure"
                    );
                }
                // A levered clash IS the product of its two halves, so
                // the sentence prints the product it computes here.
                // Stating a stored figure beside the halves would
                // assert an identity nothing enforces.
                if let Some((radians, arm)) = lever {
                    return write!(
                        f,
                        "measured a roll of {radians} rad on a {arm} m arm, a deviation of {} m \
                         where the cosets would have had to meet",
                        radians * arm
                    );
                }
                // Every other margin is a length measured outright —
                // and a length that is not finite is not one.
                if clash.is_finite() {
                    write!(
                        f,
                        "measured a clash of {clash} m where the cosets would have had to meet"
                    )
                } else {
                    write!(
                        f,
                        "measured a clash that is not a finite length ({clash}) where the cosets \
                         would have had to meet"
                    )
                }
            }
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
            Self::Unleverable { mate, refusal } => {
                write!(f, "mate {}: {refusal}", mate.0)
            }
        }
    }
}

impl core::error::Error for MateFault {}
