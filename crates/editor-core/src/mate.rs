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
//! A mate is a **leaf**: its `a`/`b` are [`crate::SitedFace`]s — an
//! instance-qualified FACE name plus the OPERAND node it is read at,
//! the kind fixed by the type because a mate is a face-pair contact
//! — and neither half is a DAG edge (the shipped D3 carve-out, which
//! `Declare` established, extended to the node half by A12's reading
//! rule). What A12 adds on top is the *reading* edge: the MEMBER
//! instance each reference's OPERAND resolves through, walking down
//! to the minting instance past any number of transforms, `Part`
//! instance selections and pattern levels (A11's member vocabulary,
//! [`member`]) — RECOMPUTED from the
//! recipe at need ([`reading_edges`]) and never stored beside it. The
//! partitions
//! divide on that distinction — A9's relative-freedom components and
//! A11's placement groups run over consuming ∪ reading edges, while
//! A10's coverage, ancestor-freedom, maintenance and product gather
//! run over consuming edges only. A mate is therefore an ordinary
//! non-body root: an isolated sink under consuming edges, listed like
//! any other, denoting no body, ignored by the gather.
//!
//! # What a mate says, geometrically
//!
//! [`Alignment`] carries two mate frames — one per side, in that
//! instance's OWN part coordinates — plus the primitive relating
//! them, the axis sense, and the clocking rider. A frame is either
//! three authored vectors or the name of a FACE of the part, whose
//! canonical pose the part's own evaluation answers at solve time
//! ([`MateFrame`]); the solve's inputs are the document plus its
//! mated parts' evaluations, and the solve ALGORITHM is a
//! decided-predicate computation over the resolved frames (A11's
//! "coset intersection over decided predicates, no numeric fitting",
//! with the two reads `ASSEMBLY.md` A11 rule 5 states — the lever,
//! [`reach`], and a `FromFace` frame's pose, [`MateReach::face_pose`]).
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
//! `CurveContact` — [`crate::MintRefusal::NoAtRestRecord`]), and
//! every later class — `Fit { gap }` when it lands — refuses at the
//! solve door, because a declared clearance changes what "coincide"
//! means and this unit solves coincidence only.

use crate::eval::NodeRefusal;
use crate::node::RecipeNodeId;
use geom_core::Tol;
use geom_core::linalg::frame::FrameError;
use geom_core::linalg::{Affine3, OrthoFrame, Point3, UnitVec3, Vec3};
use geom_core::predicate::{BandError, Indeterminate, Margin};

pub mod coset;
pub mod member;
pub mod reach;
pub mod solve;

pub use coset::{Coset, Subgroup};
pub use member::{Member, member_of};
pub use reach::{
    FacePoseRefusal, MateReach, ReachRefusal, RefusingReach, SurfaceKind, body_reach, part_reach,
};
pub(crate) use solve::solve_with_env;
pub use solve::{
    MateRole, PoseRefusal, SolvedPoses, Space, UNPLACED_RECOURSE, Unplaced, gauge_chain, groups,
    places, reading_edges, relative_freedom_components, root_of, solve_document,
};

/// The kernel's contact vocabulary, re-exported (M9-1 PR-1: one enum,
/// defined lowest). A mate's class IS a contact declaration.
pub use topo::ContactClass;

/// Which side of a mate a diagnostic is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// **Three authored vectors** in the part's own coordinates: an
/// origin, the primary axis (a planar rest's normal, a coaxial mate's
/// axis), and the clocking reference that fixes roll — the
/// [`MateFrame::Authored`] arm, and what a [`MateFrame::FromFace`] arm
/// RESOLVES to once the face's pose is read, so both arms meet the
/// same frame witness ([`AuthoredFrame::frame`]). The type therefore
/// holds a resolved face's numbers as well as an author's: the name
/// says where the numbers come from on the wire, not in the solve.
///
/// The frame is built through [`geom_core::linalg::frame::point_at`]
/// — U4B's frame family, reused rather than reinvented — so a
/// degenerate axis or a reference on the axis line refuses through
/// that ladder's typed voice instead of silently producing a
/// rank-deficient basis.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoredFrame {
    /// The frame's origin (part coordinates).
    pub origin: [f64; 3],
    /// The primary axis: a rest plane's normal, a coaxial axis. Need
    /// not be unit; only its direction is read.
    pub axis: [f64; 3],
    /// The clocking reference, fixing roll about `axis`. Need not be
    /// perpendicular to the axis; only its perpendicular part is read.
    pub reference: [f64; 3],
}

impl AuthoredFrame {
    /// **The frame this datum denotes, as the witness the ladder
    /// decided**: local +Z is `axis`, local origin is `origin`, roll
    /// fixed by `reference` (U4B's `point_at` convention, verbatim),
    /// through `point_at_frame` — so a reader that levers a sine or a
    /// cosine off the axis takes it as [`OrthoFrame::w`], a
    /// [`UnitVec3`], holding the fact by type. [`Self::placement`] is
    /// this frame's `to_affine`, and [`Self::axis`] its `w`; one
    /// construction, read once per side by the solve.
    ///
    /// # Errors
    ///
    /// [`FrameError`] when the axis has no definite direction, when
    /// the reference has no definite perpendicular offset from it, or
    /// — asked before either sign — when the axis's length or that
    /// perpendicular offset is not a finite NUMBER
    /// (`FrameError::NonFiniteLength`, at `Aim` and `RollReference`
    /// respectively). This is `point_at`'s own list; the three cases
    /// arrive here unchanged.
    pub fn frame(&self, tol: Tol) -> Result<OrthoFrame<f64>, FrameError> {
        let eye = Point3::from_array(self.origin);
        let axis = Vec3::from_array(self.axis);
        let reference = Vec3::from_array(self.reference);
        geom_core::linalg::frame::point_at_frame(eye, eye + axis, reference, tol)
    }

    /// The rigid placement this frame denotes: [`Self::frame`]'s
    /// affine, bit for bit.
    ///
    /// # Errors
    ///
    /// [`Self::frame`]'s.
    pub fn placement(&self, tol: Tol) -> Result<Affine3<f64>, FrameError> {
        Ok(self.frame(tol)?.to_affine())
    }

    /// The axis this frame aims along, as the witness [`Self::frame`]
    /// decided: its `w`, which is [`Self::placement`]'s third column.
    ///
    /// # Errors
    ///
    /// [`Self::frame`]'s.
    pub fn axis(&self, tol: Tol) -> Result<UnitVec3<f64>, FrameError> {
        Ok(self.frame(tol)?.w())
    }

    /// Whether every coordinate is a finite number.
    fn is_finite(&self) -> bool {
        [self.origin, self.axis, self.reference]
            .iter()
            .all(|v| v.iter().all(|x| x.is_finite()))
    }

    /// The origin's distance from the part's origin — this frame's
    /// term of the lever ([`Alignment::lever_arm`]).
    fn origin_norm(&self) -> f64 {
        let [x, y, z] = self.origin;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }
}

/// **A face of the part, resolved at evaluation** — the
/// [`MateFrame::FromFace`] arm. The face name is the STATE; the frame
/// is derived from the face's canonical pose every time the solve
/// reads it, so an edit to the part that moves the face moves the
/// mate with it and nothing is stored twice.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaceFrame {
    /// The face, by its PART-LOCAL stable name — a row of the part's
    /// own product table, the spelling the part's document authored,
    /// never the qualified spelling a head carries in the assembling
    /// document (a head's `InPart` wrapper names the instance; this
    /// name is read INSIDE the part, where no instance exists). A
    /// face by type, as a head is: a name of another kind is refused
    /// where the name is made ([`crate::FaceName::new`]) and at the
    /// wire, so the solve never meets one.
    ///
    /// The face is the whole of the arm: its roll reference is the
    /// carrier's own in-frame direction (`Pose::u_ref`), which every
    /// carrier the readback answers fixes, so there is nothing for an
    /// author to add and no key for one on the wire.
    pub face: crate::FaceName,
}

/// One side's **mate frame**, in that instance's own part coordinates.
///
/// Two arms, closed. [`Self::Authored`] is three vectors the author
/// wrote. [`Self::FromFace`] names a face of the part and takes that
/// face's CANONICAL POSE — read off its surface parameters exactly,
/// no tolerance (`topo::readback::face_pose`), through the mated
/// part's own evaluation in the part's own coordinates
/// ([`MateReach::face_pose`]) — as the side's frame: the pose's
/// origin, its axis, and the carrier's own in-frame reference
/// direction as the roll reference. Both arms then meet the same
/// witness ladder ([`AuthoredFrame::frame`]), so a resolved face
/// refuses a degenerate axis or a reference on the axis line exactly
/// as authored vectors do.
///
/// **A face frame's roll is the carrier's.** The roll reference is
/// the carrier's `u_ref` and nothing else, so a face frame cannot
/// turn a mate's roll about its axis: a side that needs a roll of its
/// own takes authored vectors.
///
/// **The pose's orientation sense is NOT folded into the axis.** The
/// resolved axis is the CHART's direction, as the readback documents
/// it (`Pose::axis`), whether the face's outward normal is `+axis` or
/// `-axis`; which way the two sides point at each other is the mate's
/// own [`AxisSense`], authored beside the frames, and folding the bit
/// in would make one datum answer two questions. An analytic carrier
/// — plane, cylinder, cone, sphere, torus — resolves; a face with no
/// canonical frame (a NURBS or approximating carrier) refuses typed
/// at the mate, the side, the instance and the part
/// ([`MateFault::FaceUnresolved`]) and keeps taking authored vectors.
///
/// **A face frame resolves at the nominal value only.** The pose is
/// read off the part's evaluated product and crosses to the solve as
/// `f64`; on an analysis lane — the `Dual64` passes of
/// `stackup::sensitivities`, the `Interval` leaf of a certified
/// `clearance` — the product's coordinates pin no single number, and
/// the side refuses [`FacePoseRefusal::Unpinned`] rather than read the
/// nominal and drop the pose's own sensitivity to the parameters. So
/// those two doors refuse an assembly that holds a face frame, where
/// the same mate authored as vectors still solves on every lane.
///
/// On the wire the arm is externally tagged — `{"Authored": {…}}` or
/// `{"FromFace": {…}}` — and each inner struct is closed over its own
/// keys: the tag decides the arm before a field is read, a stray key on
/// either arm refuses, and a frame with no tag refuses.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MateFrame {
    /// Three authored vectors in the part's coordinates.
    Authored(AuthoredFrame),
    /// A face of the part, resolved at evaluation.
    FromFace(FaceFrame),
}

impl MateFrame {
    /// Three authored vectors: `origin`, the primary `axis`, and the
    /// clocking `reference` ([`AuthoredFrame`]'s fields, in its order).
    pub fn authored(origin: [f64; 3], axis: [f64; 3], reference: [f64; 3]) -> Self {
        Self::Authored(AuthoredFrame {
            origin,
            axis,
            reference,
        })
    }

    /// A face of the part, by its part-local name ([`FaceFrame`]).
    pub fn from_face(face: crate::FaceName) -> Self {
        Self::FromFace(FaceFrame { face })
    }

    /// The authored vectors, where this frame is [`Self::Authored`];
    /// `None` for a face, which has none until the solve resolves it.
    pub fn authored_vectors(&self) -> Option<&AuthoredFrame> {
        match self {
            Self::Authored(frame) => Some(frame),
            Self::FromFace(_) => None,
        }
    }

    /// The face this frame names, where it is [`Self::FromFace`].
    pub fn face(&self) -> Option<&FaceFrame> {
        match self {
            Self::Authored(_) => None,
            Self::FromFace(face) => Some(face),
        }
    }

    /// Whether every authored coordinate is finite — the vectors of an
    /// authored frame; a face frame authors no number (the face's pose
    /// is the part's, read at the solve).
    fn is_finite(&self) -> bool {
        match self {
            Self::Authored(frame) => frame.is_finite(),
            Self::FromFace(_) => true,
        }
    }
}

/// Which way the two sides' axes point at each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
#[serde(rename_all = "snake_case", deny_unknown_fields)]
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
    /// Refused at the insert door and at every solve that reads the
    /// datum ([`table_gap`]), not re-decided on replay.
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// contradictory and gets decided over the mate's own lever — at
    /// the insert door and at every solve that reads the datum, not
    /// re-decided on replay; on a planar rest the table has no entry
    /// ([`table_gap`]) and the same doors refuse typed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clocking: Option<f64>,
}

impl Alignment {
    /// **The datum's own contribution to the lever** this mate's angular
    /// decisions turn on: both mate frames' distances from their parts'
    /// origins, plus every length the primitive authors, all summed.
    ///
    /// The lever itself is formed in the solve (`mate/solve.rs`), as
    ///
    /// ```text
    /// L  =  (R_a + ‖a.origin‖)  +  (R_b + ‖b.origin‖)  +  Σ |authored lengths|
    /// ```
    ///
    /// where each `R` is the mated part's own reach from its origin
    /// ([`MateReach`]) — an UPPER bound on the extent of the two parts
    /// together from the datum, with no floor and no constant
    /// (ERROR-DESIGN E3's amendment, ratified at revision E12, shipped
    /// whole at this site). This function is the part of that sum the
    /// datum answers over its two RESOLVED frames — `a` and `b` are
    /// the sides' frames as the solve reads them, the authored vectors
    /// or the face's pose ([`MateFrame`]), so the term is formed once
    /// per mate, after resolution, beside the reach read: `R +
    /// ‖origin‖` bounds a part's reach from its mate frame by the
    /// triangle inequality, and the authored lengths (a planar rest's
    /// offset) are the separation the datum names between the two
    /// frames once mated. Every term is an upper bound and none is
    /// dropped, because over-refusal is the safe direction: a lever
    /// larger than the truth prices a tilt higher and refuses sooner.
    ///
    /// Pure, and never a refusal: a datum authored at the origin with
    /// no length contributes nothing, and that is not a degenerate
    /// case — `Coaxial` on two origin frames is how an axis-to-axis
    /// mate is ordinarily written. With a real part on each side the
    /// lever is never zero, so no floor guards this door.
    ///
    /// No floor stands under this term: the parts' own reach is the
    /// scale, at whatever size the author works, and a lever of `L`
    /// makes the smallest decidable tilt `ε / L`.
    pub fn lever_arm(&self, a: &AuthoredFrame, b: &AuthoredFrame) -> f64 {
        self.primitive
            .authored_lengths()
            .into_iter()
            .flatten()
            .fold(a.origin_norm() + b.origin_norm(), |lever, length| {
                lever + length.abs()
            })
    }

    /// Whether every authored coordinate is finite — the edit door's
    /// admission test, as a placement's (a non-finite alignment could
    /// never decide anything). A
    /// `FromFace` side authors no number; its face's pose is the
    /// part's, read at the solve, and is not a number this door can
    /// see.
    pub fn is_finite(&self) -> bool {
        self.a.is_finite()
            && self.b.is_finite()
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

/// **The recourse a [`MateFault::Contradictory`] ends on**: what an
/// author does about two declarations that admit no common pose.
///
/// One sentence for both shapes the arm renders — a PAIR of mates and
/// a mate contradicting itself through its own rider — because the
/// repair is the same in both and a sentence naming "one of the two"
/// is false of the second. The rung is the solve door: cosets are
/// intersected exactly, so nothing here is a tolerance to widen and no
/// pose is averaged out of the two.
pub const CONTRADICTORY_RECOURSE: &str = "delete the mate that was not meant, or re-author the \
                                          datum that is wrong; the solve intersects cosets \
                                          exactly and never averages two declarations";

/// **The v1 class restriction, named.** `Fit` is specified and not
/// built; a mate cannot declare a designed clearance until the kernel
/// variant lands with its first consumer, and AQ6 is where the
/// cross-document detail is still open.
pub const CLASS_DEFERRAL: &str = "v1 mates SOLVE Rest and Tangent and ASSEMBLE Rest alone; the \
                                  cross-document detail of a designed clearance is undischarged";

/// **The recourse a [`crate::MintRefusal::NoAtRestRecord`] ends on**:
/// what an author does about a class that solves and has no record to
/// be verified by at rest.
///
/// The arm already quotes the table's reason for THIS class; this is
/// the repair, and it names the rung the way [`CLASS_DEFERRAL`] does —
/// `Rest` is the one class v1 carries all the way to the gate, and a
/// curved contact verified at rest is outside v1 rather than a
/// tolerance away.
pub const NO_AT_REST_RECORD_RECOURSE: &str = "declare the contact as a Rest, the one class v1 \
                                              mints and verifies at rest; a curved contact \
                                              verified at rest is outside v1 and is not built";

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
    /// rest, so [`crate::assemble`] refuses with a
    /// [`crate::MintRefusal::NoAtRestRecord`] row naming the mate — a
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

/// **The coset table's static gaps** — the primitive-and-rider pairs
/// the table has no row for, refused on the datum alone with no
/// geometry read and no lever formed: a clocking rider on a planar
/// rest, and a standalone clocking with no carrying primitive. The
/// sentence is the table's own, and is what
/// [`MateFault::TableLacks`]'s `what` carries.
///
/// INVARIANT: every door that refuses a static gap reads it here —
/// the coset table itself (`mate/solve.rs`'s `mate_coset`, at the
/// arm each gap falls in, so a frame refusal still precedes it) and
/// the viewer's mate tool, which refuses before any geometry — so no
/// second match over the pair can drift from the table. `None` for
/// every pair the table has a row for, decided or not: a rider on a
/// coincidence is DECIDED over a lever, which is not a gap.
pub fn table_gap(primitive: MatePrimitive, clocking: Option<f64>) -> Option<&'static str> {
    match (primitive, clocking) {
        (MatePrimitive::PlanarRest { .. }, Some(_)) => Some("a clocking rider on a planar rest"),
        (MatePrimitive::Clocking, _) => Some("a standalone clocking with no carrying mate"),
        (
            MatePrimitive::FrameCoincidence
            | MatePrimitive::Coaxial
            | MatePrimitive::PlanarRest { .. },
            _,
        ) => None,
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

/// Why a lever could not be formed for a mate: one of its parts' reach
/// ([`MateReach::reach`]) is not in hand, the member stands on no
/// instance, or the lever the two parts form is out of the format's
/// range.
#[derive(Debug, Clone, PartialEq)]
pub enum LeverRefusal {
    /// **The reach's own refusal**, named against the instance the
    /// solve was asking for and the part it stands on: the reach's
    /// vocabulary has its one home in [`ReachRefusal`], and the lever
    /// adds only its subject.
    Reach {
        /// The instance.
        instance: RecipeNodeId,
        /// Its part.
        part: crate::ident::DocRef,
        /// Why the part's reach is not in hand.
        refusal: ReachRefusal,
    },
    /// The member stands on a node that is not a live instantiate
    /// node, so there is no part whose reach could be asked. The
    /// member walk ends only on a live `InstantiatePart` (A11 rule 5),
    /// so no door reaches this arm: it names the node rather than
    /// assume the walk's rule.
    NotAnInstance {
        /// The node.
        node: RecipeNodeId,
    },
    /// Both parts' reach is in hand, but the lever they form with the
    /// datum's own terms is a length the format cannot decide a tilt
    /// over ([`coset::Arm::of`]): each half is finite where it is
    /// read, and their sum, or its square, overflows.
    OutOfRange {
        /// The two parts' reach, summed.
        parts: f64,
        /// The datum's own terms ([`Alignment::lever_arm`]).
        datum: f64,
    },
}

// The part's own refusal is numbered in the part, so it is said by its
// own `Display`; the instance and the node are this document's.
impl crate::spoken::Say for LeverRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Reach {
                instance, refusal, ..
            } => write!(f, "{}'s part {refusal}", by.node_as(*instance, "instance")),
            Self::NotAnInstance { node } => write!(
                f,
                "{} is not a live instantiate node, so it has no part whose extent \
                 could lever a verdict. {}",
                by.node(*node),
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::OutOfRange { parts, datum } => {
                // A half that is not finite has no metre figure to print.
                let length = |f: &mut core::fmt::Formatter<'_>, x: f64, of: &str| {
                    if x.is_finite() {
                        write!(f, "{x} m of {of}")
                    } else {
                        write!(f, "a length of {of} that is not finite")
                    }
                };
                f.write_str("its lever, ")?;
                length(f, *parts, "its parts' reach")?;
                f.write_str(" plus ")?;
                length(f, *datum, "its datum")?;
                write!(
                    f,
                    ", is too long for a tilt to be decided over it. Recourse: {}",
                    geom_core::RANGE_RECOURSE
                )
            }
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for LeverRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// Why a `FromFace` frame could not be resolved to a pose: what the
/// mated part's own evaluation answered about the named face
/// ([`MateReach::face_pose`]), named against the instance the solve
/// was reading, the part it stands on and the face the frame named —
/// the subject a [`LeverRefusal`] adds to a reach refusal, and a face.
#[derive(Debug, Clone, PartialEq)]
pub enum FaceRefusal {
    /// **The reach's own refusal of the face's pose**, named against
    /// the instance, its part and the face the frame named: the
    /// reach's vocabulary has its one home in [`FacePoseRefusal`].
    Reach {
        /// The instance.
        instance: RecipeNodeId,
        /// Its part.
        part: crate::ident::DocRef,
        /// The face the frame named.
        face: crate::FaceName,
        /// Why the face's pose is not in hand.
        refusal: FacePoseRefusal,
    },
    /// The member stands on a node that is not a live instantiate
    /// node, so there is no part whose face could be asked — the same
    /// fact as [`LeverRefusal::NotAnInstance`], met by the reader of a
    /// frame, and no door reaches it either.
    NotAnInstance {
        /// The node.
        node: RecipeNodeId,
    },
}

impl FaceRefusal {
    /// The face the refusal is about, where it names one.
    pub fn face(&self) -> Option<&crate::FaceName> {
        match self {
            Self::Reach { face, .. } => Some(face),
            Self::NotAnInstance { .. } => None,
        }
    }
}

// The face is the part's name and the reach's refusal the part's, both
// numbered in the part, so each is said by its own `Display`; the
// instance and the node are this document's.
impl crate::spoken::Say for FaceRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Reach {
                instance,
                face,
                refusal,
                ..
            } => write!(
                f,
                "{}'s part answers none for the {face}: {refusal}",
                by.node_as(*instance, "instance")
            ),
            Self::NotAnInstance { node } => write!(
                f,
                "{} is not a live instantiate node, so it has no part whose face could \
                 be read. {}",
                by.node(*node),
                geom_core::KERNEL_DEFECT_ENDING
            ),
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for FaceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// **What a levered clash measured, and the arm that carried it to a
/// length.** The product is the deviation; the halves are what the
/// sentence prints, because a stored product beside them would assert
/// an identity nothing enforces ([`Lever::deviation`] computes it).
///
/// A closed enum, and deliberately not a value beside a unit string:
/// the two arms are the two kinds of number the solve levers, and a
/// third kind is a new arm with its own sentence, never a free string
/// a site could set to a length.
///
/// The arm is the one the predicate was decided over, and the two
/// kinds are decided over different ones: a `Roll` is decided in the
/// mate's own coset over that mate's lever, a `Residual` in the fold
/// over the largest lever of the mates folded so far — so a pair can
/// print one arm on a roll and a larger one on a residual, each the
/// truth of its own decision.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lever {
    /// An authored roll, in radians (the clocking rider's
    /// `mate_clocking_redundant`).
    Roll {
        /// The roll, in radians.
        radians: f64,
        /// The arm, in metres.
        arm: f64,
    },
    /// A dimensionless residual — a sine, a cosine, a Frobenius
    /// departure from the identity, a reachability defect — named by
    /// the predicate that measured it.
    Residual {
        /// The residual, a pure number.
        value: f64,
        /// The arm, in metres.
        arm: f64,
    },
}

impl Lever {
    /// The arm, in metres, whichever kind of number it levered.
    pub fn arm(self) -> f64 {
        match self {
            Self::Roll { arm, .. } | Self::Residual { arm, .. } => arm,
        }
    }

    /// **The margin this lever decides**: the pure number levered by
    /// the arm through [`Margin::levered`] — the ONE home of that
    /// multiplication for every levered predicate in the solve, so
    /// the number a refusal quotes and the number the funnel decided
    /// are one value.
    pub fn margin(self) -> Margin<f64> {
        match self {
            Self::Roll { radians, arm } => Margin::levered(radians, arm),
            Self::Residual { value, arm } => Margin::levered(value, arm),
        }
    }

    /// The deviation the lever measured, in metres: [`Self::margin`]'s
    /// value, computed here and nowhere stored.
    pub fn deviation(self) -> f64 {
        self.margin().value()
    }
}

/// **What a contradictory refusal measured** — closed over the three
/// ways a membership predicate refuses, so the refusal carries the
/// measurement and nothing that could disagree with it: no metre
/// figure stored beside the halves it is the product of, no sentinel
/// read back out of a number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Clash {
    /// The predicate decided the empty intersection STRUCTURALLY
    /// (`mate_member_empty`): the empty set holds nothing and no
    /// margin decides that, so there is no measurement at all.
    Structural,
    /// A length the predicate measured outright.
    Length {
        /// The margin that should have been zero and was not, in
        /// metres.
        metres: f64,
    },
    /// A pure number the predicate levered by an arm; the deviation
    /// is the product ([`Lever::deviation`]).
    Levered(Lever),
}

impl Clash {
    /// The deviation in metres, when the predicate measured one: a
    /// length verbatim, a lever's product, and `None` for the
    /// structural refusal.
    pub fn deviation(self) -> Option<f64> {
        match self {
            Self::Structural => None,
            Self::Length { metres } => Some(metres),
            Self::Levered(lever) => Some(lever.deviation()),
        }
    }
}

/// A typed mate refusal (D9: fail loud, never a guess). Every arm names
/// its subject — the mate, the pair, the predicate, the residual, or
/// the two documents a mispaired read named.
///
/// **Which mate an arm is about is read per arm by its two consumers,
/// `viewer::tree::blamed_mates` and `pncad_py::MateFaultPayload`, and
/// the two arms that name no mate are not the same case**: [`Self::Band`]
/// names no mate and reaches EVERY row of the document —
/// [`solve_document`] records the one fault against every `Node::Mate`
/// and every `Node::InstantiatePart` before it reads a mate — while
/// [`Self::PosesOfAnotherDocument`] names no mate and reaches NO row,
/// being raised by [`SolvedPoses::placement`] and never inserted in a
/// fault map. There is deliberately no `subject()` here: an
/// `Option<RecipeNodeId>` answers `None` for both and erases that
/// asymmetry, which is the one fact both consumers exist to carry.
#[derive(Debug, Clone, PartialEq)]
pub enum MateFault {
    /// The solve is a solve of ANOTHER document (DI3). Names no mate
    /// and reaches no row — the enum's own doc states which of the two
    /// mate-less arms reaches what.
    PosesOfAnotherDocument {
        /// The document whose placement was asked for.
        expected: crate::ident::DocumentId,
        /// The document the solve is of.
        found: crate::ident::DocumentId,
    },
    /// A side's frame — its authored vectors, or the pose a `FromFace`
    /// frame resolved to — has no definite frame on the witness ladder.
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
        /// What it measured: a length outright, a pure number levered
        /// by an arm, or nothing — the structural refusal.
        ///
        /// A lever's arm is the mate's lever as the solve forms it —
        /// the two mated parts' own extent from the datum plus the
        /// datum's own terms, `ASSEMBLY.md` A11 rule 5's qualifier
        /// ([`Alignment::lever_arm`] states the sum; [`MateReach`] is
        /// the parts' half) — and NOT a contact feature, so a message
        /// that names it is naming the parts' scale and nothing in the
        /// model.
        clash: Clash,
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
    /// **A mate's reference names no member of A11's vocabulary** —
    /// N5's dangling reference. It contributes no reading edge; the
    /// insert door and every solve refuse typed rather than pretending
    /// the mate is absent — at insert, where the head resolves to no
    /// member as authored, and at evaluation, where a later edit
    /// stranded it.
    ///
    /// Exactly two causes, and both are about a copy or a node that
    /// does not exist:
    ///
    /// - the WALK from the reference's operand down to its name's
    ///   head stopped — a stranded operand, or a node that places no
    ///   body of its own ([`crate::member_of`]);
    /// - the copy the name says is at or beyond the pattern's
    ///   evaluated count, so the named copy is not there.
    ///
    /// A placer that exists and whose pose merely could not be
    /// DERIVED is [`MateFault::PlacerRefused`], which carries the
    /// evaluation's own refusal. So this arm is about a node the
    /// vocabulary does not reach or a copy that is not there — never
    /// about arithmetic. It is still reported at a node that may be
    /// perfectly live: a pattern the reference's name does not
    /// qualify `Instance(i)` stops the walk, and the walk stops AT
    /// that pattern.
    DanglingHead {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side dangles.
        side: MateSide,
        /// **The node at which the reference resolves to no member**:
        /// where the walk stopped, which is a stranded operand when
        /// the operand is the broken half and the first node outside
        /// the vocabulary otherwise; or the pattern whose count the
        /// named copy is past. Not in general the reference's own
        /// head, which is often live and fine.
        head: RecipeNodeId,
    },
    /// **A placer on the reference's chain refused to derive its
    /// pose**, in the evaluation layer's own words.
    ///
    /// The member exists and the walk reached it; what did not exist
    /// is the static offset a pattern copy or a transform on the way
    /// contributes. Every such refusal is one the evaluation layer
    /// already types — a slot that does not evaluate, a direction of
    /// no definite length, an explicit-rule pattern, an axis operand
    /// that is not an axis datum — so it is carried here UNALTERED
    /// rather than relabelled as a dangling head.
    ///
    /// The sentence names the placer and never renders `error`, which
    /// is the placer's refusal with its own recourse. Whether the fault
    /// CARRIES it, as a line of its own under the mate's
    /// ([`crate::NodeErrorKind::carried_chain`]), is `placer_row`'s:
    ///
    /// - **Raised while a group's fold derives an offset**, at a
    ///   placer on the reference's chain: the fault reaches the
    ///   instance under that placer, so the placer is POISONED, never
    ///   evaluates and never states its own cause. This fault is the
    ///   only place that cause appears, and it carries it
    ///   ([`PlacerRow::Silent`]).
    /// - **Raised where the solve reads one mate's references**, or at
    ///   a node read off the chain (an axis datum, a transform on the
    ///   way to it): the fault reaches the mate alone, and the placer
    ///   evaluates and fails in its own right with the same refusal.
    ///   Its own row states it, and the mate's sentence points there
    ///   ([`PlacerRow::States`]).
    PlacerRefused {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side's reference the placer is on.
        side: MateSide,
        /// **The node whose evaluation raised the refusal.** It lies
        /// on the reference's derivation: a pattern or a transform on
        /// the chain, a `Part` on it whose own index does not evaluate or
        /// selects outside its value,
        /// or a node one of those reads to derive its map — a circular
        /// rule's axis DATUM, whose slot that does not evaluate is
        /// reported under the datum's id, or a TRANSFORM on the way to
        /// that datum, whose own operand refusal is reported under the
        /// transform's — because that is the node an author goes and
        /// fixes, and the node the evaluation itself fails.
        placer: RecipeNodeId,
        /// The evaluation layer's own typed refusal for it, unchanged.
        error: NodeRefusal,
        /// Whether the placer's own row states `error`.
        placer_row: PlacerRow,
    },
    /// **A `Node::Part` selects a copy the reference's NAME does not
    /// name.** The name is the authority on which copy a mate speaks
    /// about; a `Part` standing above a pattern in the walk (with
    /// nothing but transforms between) says which body of that
    /// pattern's value the body below it is. A document where those
    /// disagree would be PLACED by the name and GATHERED by the
    /// `Part` — two different bodies for one declaration — so the
    /// insert door and the solve refuse rather than choosing, and
    /// report both indices in the `Part`'s index space.
    ///
    /// Raised per REFERENCE, where the solve reads each one, so it
    /// reaches a declaring mate as surely as a tree edge's.
    PartSelectsAnotherCopy {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side.
        side: MateSide,
        /// The `Part` node whose index expression disagrees.
        part: RecipeNodeId,
        /// The copy the reference's name names, as the body index
        /// the `Part` selects by: the copy index itself over a
        /// pattern of one body, the flat `j·M + i` over a nested one.
        named: u32,
        /// What the `Part`'s index expression evaluates to at the
        /// document's parameter bindings: a copy the value has, since
        /// an index outside it is the `Part`'s own refusal
        /// ([`MateFault::PlacerRefused`]), judged first. `i64`, the
        /// type the evaluated Count has.
        selected: i64,
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
    /// The mate's lever could not be formed: one of its parts' reach
    /// ([`MateReach`]) is not in hand, so no verdict over the parts'
    /// extent can be levered ([`Alignment::lever_arm`] for the sum).
    Unleverable {
        /// The mate.
        mate: RecipeNodeId,
        /// Why — boxed, as [`Self::FaceUnresolved`]'s is: the refusal
        /// carries the instance, the part and the reach's own refusal,
        /// and the fault's every other arm stays the size it is.
        refusal: Box<LeverRefusal>,
    },
    /// **A checked offset disagrees with the solve** (A11 (2)): a
    /// member of a placed group that is not its root carries an
    /// offset, which states where it sits, and the solve places it
    /// elsewhere. The statement is verified, never trusted and never
    /// ignored, so the instance is faulted and the rest of its group
    /// stands. Recourse: [`OFFSET_RECOURSE`].
    OffsetDisagrees {
        /// The instance whose offset disagrees.
        instance: RecipeNodeId,
        /// Its group's root, whose offset places the group.
        root: RecipeNodeId,
        /// The predicate that measured the disagreement.
        predicate: &'static str,
        /// What it measured.
        clash: Clash,
    },
    /// **A checked offset could not be checked** (A11 (2)): the
    /// statement is neither confirmed nor refuted, so the instance is
    /// faulted rather than trusted.
    OffsetUnchecked {
        /// The instance whose offset could not be checked.
        instance: RecipeNodeId,
        /// Why.
        cause: Box<OffsetCheck>,
    },
    /// **A side's `FromFace` frame did not resolve to a pose**: the
    /// mated part's own evaluation answered no pose for the face the
    /// frame names ([`MateReach::face_pose`]), in the resolver's or
    /// the readback's own voice — the part not in hand, a name the
    /// part's table lacks or ties, a carrier with no canonical frame, a
    /// product on an analysis lane ([`FaceRefusal`]). Raised
    /// where the solve reads the side's frame, before the coset table
    /// and before any lever is formed; the insert door raises it for
    /// a mate being inserted, and the solve at every evaluation for a
    /// state a part edit brings a mate to (a face that vanished or
    /// changed carrier) — a load never refuses it, since replay
    /// declines the read (`solve::admit_mate`).
    FaceUnresolved {
        /// The mate.
        mate: RecipeNodeId,
        /// Which side's frame.
        side: MateSide,
        /// Why — boxed, as an escalation's diagnostics are: the
        /// refusal names the part, the face and the readback's own
        /// arm, and the fault's every other arm stays the size it is.
        refusal: Box<FaceRefusal>,
    },
}

/// The recourse a disagreeing checked offset names.
pub const OFFSET_RECOURSE: &str = "clear the offset, or change the mate";

/// **Why a checked offset could not be checked**
/// ([`MateFault::OffsetUnchecked`]).
#[derive(Debug, Clone, PartialEq)]
pub enum OffsetCheck {
    /// A placement the check reads did not evaluate at the document's
    /// own parameters: the offset itself, the root's, or a gauge on
    /// the chain.
    Placement {
        /// The instance or gauge whose placement refused.
        node: RecipeNodeId,
        /// The evaluation layer's own refusal, unchanged.
        error: NodeRefusal,
    },
    /// The member's part reach — the lever the check is decided over —
    /// is not in hand.
    Unleverable(LeverRefusal),
    /// The check landed in the ambiguity band.
    Indeterminate(Box<Indeterminate>),
    /// The solve gives the member no pose: `mate`, which welds its
    /// group, refused, so the group's spanning tree does not reach it.
    Unreached {
        /// The group's first mate the solve refused.
        mate: RecipeNodeId,
    },
}

impl crate::spoken::Say for OffsetCheck {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Placement { node, .. } => {
                write!(f, "the placement at {} does not evaluate", by.node(*node))
            }
            Self::Unleverable(refusal) => write!(f, "{}", crate::spoken::Said(refusal, by)),
            Self::Indeterminate(diag) => {
                write!(f, "the check could not be decided — {}", diag.payload())
            }
            Self::Unreached { mate } => write!(
                f,
                "the solve gives it no pose, because {}, which welds its group, refused",
                by.node_as(*mate, "mate")
            ),
        }
    }
}

/// The cause where no document is at hand: each node by its tag.
impl core::fmt::Display for OffsetCheck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

/// The measurement a refusal quotes, as the tail of its sentence:
/// what was measured where `there` would have had to hold.
fn write_clash(f: &mut core::fmt::Formatter<'_>, clash: Clash, there: &str) -> core::fmt::Result {
    match clash {
        Clash::Structural => write!(
            f,
            "found the cosets meet in the empty set — a structural refusal, with no margin to \
             measure"
        ),
        Clash::Length { metres } if metres.is_finite() => {
            write!(f, "measured a clash of {metres} m where {there}")
        }
        // A length that is not finite is not one.
        Clash::Length { metres } => write!(
            f,
            "measured a clash that is not a finite length ({metres}) where {there}"
        ),
        Clash::Levered(lever @ Lever::Roll { radians, arm }) => write!(
            f,
            "measured a roll of {radians} rad on a {arm} m arm, a deviation of {} m where {there}",
            lever.deviation()
        ),
        Clash::Levered(lever @ Lever::Residual { value, arm }) => write!(
            f,
            "measured a dimensionless residual of {value} on a {arm} m arm, a deviation of {} m \
             where {there}",
            lever.deviation()
        ),
    }
}

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for MateFault {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::PosesOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

impl MateFault {
    /// **The refusal this fault carries, when it carries one**: the
    /// node that raised it and the refusal, which the sentence points
    /// at and never renders ([`crate::NodeErrorKind::carried_chain`]).
    /// A placer whose own row states its refusal is pointed at and not
    /// carried ([`PlacerRow`]).
    ///
    /// Exhaustive, so an arm that comes to carry another node's
    /// refusal says so here rather than being drawn as though it did
    /// not.
    #[must_use]
    pub fn carried(&self) -> Option<(RecipeNodeId, &NodeRefusal)> {
        match self {
            Self::PlacerRefused {
                placer,
                error,
                placer_row: PlacerRow::Silent,
                ..
            } => Some((*placer, error)),
            Self::PlacerRefused {
                placer_row: PlacerRow::States,
                ..
            }
            | Self::PosesOfAnotherDocument { .. }
            | Self::Frame { .. }
            | Self::ClassNotAdmitted { .. }
            | Self::TableLacks { .. }
            | Self::Indeterminate { .. }
            | Self::Band { .. }
            | Self::Contradictory { .. }
            | Self::Under { .. }
            | Self::DanglingHead { .. }
            | Self::PartSelectsAnotherCopy { .. }
            | Self::SelfMate { .. }
            | Self::Unleverable { .. }
            | Self::FaceUnresolved { .. }
            | Self::OffsetDisagrees { .. } => None,
            Self::OffsetUnchecked { cause, .. } => match &**cause {
                OffsetCheck::Placement { node, error } => Some((*node, error)),
                OffsetCheck::Unleverable(_)
                | OffsetCheck::Indeterminate(_)
                | OffsetCheck::Unreached { .. } => None,
            },
        }
    }

    /// **Every refusal this fault carries, outermost first**, each with
    /// the document its node is in — the mate's own at the first level
    /// ([`crate::NodeErrorKind::carried_chain`]).
    pub fn carried_chain(&self) -> crate::CarriedChain<'_> {
        crate::CarriedChain::from_first(self.carried(), crate::CarriedIn::ThisDocument)
    }
}

/// **Whether a [`MateFault::PlacerRefused`]'s placer states the
/// refusal on its own row**, which decides whether the fault carries
/// it. Read at the site that raises the fault, which is the one place
/// that knows whether the placer is poisoned by it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlacerRow {
    /// The placer evaluates and fails with this refusal in its own
    /// right: its row states it, and the mate points there.
    States,
    /// The placer's row cannot state it — the placer is poisoned
    /// through the instance this fault reached, or is no node of the
    /// document — so the mate carries it.
    Silent,
}

/// The predicate that decides the EMPTY intersection. It is the one
/// name in the membership vocabulary that reports no measurement — the
/// empty set holds nothing and no margin decides that — so it is a
/// shared constant rather than a literal at each site, and every door
/// that needs to know reads THIS rather than inspecting a margin.
pub(crate) const MATE_MEMBER_EMPTY: &str = "mate_member_empty";

/// **A predicate the solve refuses a contradiction on: its funnel name
/// and what refusing it found, in words, declared together.** Every
/// site that decides one passes [`Refuted::name`], and the sentence
/// says [`Refuted::found`] in its place, so a predicate cannot be
/// decided under a name the sentence has no words for — both are arms
/// of one exhaustive match. The structural empty intersection
/// ([`MATE_MEMBER_EMPTY`]) is not here: no margin decides it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refuted {
    /// A coset admitting no rotation: the candidate's rotation is the
    /// identity.
    RotationIdentity,
    /// A coset admitting no motion: the candidate's translation is zero.
    TranslationZero,
    /// A prismatic coset: the translation runs along its direction.
    TranslationAlong,
    /// A coset about an axis: the rotation fixes that axis.
    AxisFixed,
    /// A planar coset: the translation stays in the plane.
    TranslationInPlane,
    /// A cylindrical coset: the axis point stays on the axis.
    PointOnAxis,
    /// A revolute coset: the axis point stays put.
    PointFixed,
    /// Two one-axis rotation constraints: one rotation reaches both.
    TwoAxisReachable,
    /// A clocking rider on a coincidence: the rider is redundant.
    ClockingRedundant,
}

impl Refuted {
    /// Every predicate, once.
    pub const ALL: [Self; 9] = [
        Self::RotationIdentity,
        Self::TranslationZero,
        Self::TranslationAlong,
        Self::AxisFixed,
        Self::TranslationInPlane,
        Self::PointOnAxis,
        Self::PointFixed,
        Self::TwoAxisReachable,
        Self::ClockingRedundant,
    ];

    /// The funnel's name for it — routing, which a refusal carries in
    /// its payload and never in its sentence.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::RotationIdentity => "mate_member_rotation_identity",
            Self::TranslationZero => "mate_member_translation_zero",
            Self::TranslationAlong => "mate_member_translation_along",
            Self::AxisFixed => "mate_member_axis_fixed",
            Self::TranslationInPlane => "mate_member_translation_in_plane",
            Self::PointOnAxis => "mate_member_point_on_axis",
            Self::PointFixed => "mate_member_point_fixed",
            Self::TwoAxisReachable => "mate_rotation_two_axis_reachable",
            Self::ClockingRedundant => "mate_clocking_redundant",
        }
    }

    /// What refusing it found, as a clause of the contradiction's
    /// sentence.
    #[must_use]
    pub const fn found(self) -> &'static str {
        match self {
            Self::RotationIdentity => "the relative rotation is not the identity",
            Self::TranslationZero => "the relative translation is not zero",
            Self::TranslationAlong => "the translation leaves the shared direction",
            Self::AxisFixed => "the rotation moves the shared axis",
            Self::TranslationInPlane => "the translation leaves the shared plane",
            Self::PointOnAxis => "the axis point leaves the shared axis",
            Self::PointFixed => "the shared point moves",
            Self::TwoAxisReachable => "no one rotation aligns both axes",
            Self::ClockingRedundant => {
                "the clocking disagrees with the roll the coincidence already pins"
            }
        }
    }

    /// The predicate a refusal's payload names, when it is one of these.
    #[must_use]
    pub fn of(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.name() == name)
    }
}

// Every node the fault names is in the mate's own document; a part's
// face and refusal are numbered in the part and said by their own
// `Display`. The mate is said by what it is, so a frame that has already
// named it ([`crate::spoken::Speaker::about`]) reads `this mate`.
impl crate::spoken::Say for MateFault {
    #[allow(clippy::too_many_lines)] // one arm per variant, each short
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        use crate::spoken::Said;
        let mate_ = |id: RecipeNodeId| by.node_as(id, "mate");
        let instance_ = |id: RecipeNodeId| by.node_as(id, "instance");
        match self {
            // The two ids ride the payload; the sentence names the roles.
            Self::PosesOfAnotherDocument { .. } => f.write_str(
                "a placement was asked of a solve of another document than the one it was \
                 asked for, so no frame is read. Recourse: solve the document whose placement \
                 is asked for, and read it off that solve",
            ),
            Self::Frame { mate, side, error } => write!(
                f,
                "{}'s {} frame has no definite placement: {error}",
                mate_(*mate),
                side.name()
            ),
            Self::ClassNotAdmitted { mate } => write!(
                f,
                "{}'s contact class is not admitted in v1 — {}. Recourse: delete the \
                 mate, and insert it again declaring a Rest",
                mate_(*mate),
                topo::FIT_DEFERRAL
            ),
            Self::TableLacks { mate, what } => write!(
                f,
                "{}: the coset table has no entry for {what}, and refuses rather than \
                 invent one. Recourse: delete the mate, and insert it again as a coaxial mate \
                 carrying the clocking",
                mate_(*mate)
            ),
            Self::Indeterminate { mate, diag } => write!(
                f,
                "{}: a case split could not be decided — {}. Recourse: {}",
                mate_(*mate),
                diag.payload(),
                geom_core::NO_DECLARATION_RECOURSE
            ),
            Self::Band { error } => write!(f, "the mate solve could not build a band: {error}"),
            Self::Contradictory {
                held,
                added,
                predicate,
                clash,
            } => {
                // One mate named on BOTH sides is a mate contradicting
                // itself, and "mate 6 and mate 6" reads as an indexing
                // fault rather than as the shape the payload states.
                if held == added {
                    write!(
                        f,
                        "{} contradicts itself — the constraints it declares admit no \
                         common pose",
                        mate_(*held)
                    )?;
                } else {
                    write!(f, "{} and {} cannot both hold", mate_(*held), mate_(*added))?;
                }
                // The predicate's name is routing and rides the
                // payload; the sentence says what it found in words.
                f.write_str(": ")?;
                if let Some(refuted) = Refuted::of(predicate) {
                    write!(f, "{} — ", refuted.found())?;
                }
                f.write_str("the solve ")?;
                // WHETHER there is a measurement to report, and of
                // which kind, is the predicate's fact and the type
                // carries it: a levered clash prints the product of
                // the two halves it shows, computed there, so the
                // sentence cannot assert an identity the payload
                // failed to keep.
                write_clash(f, *clash, "the cosets would have had to meet")?;
                // The repair is the same whichever measurement the
                // predicate had to report, so it is stated once.
                write!(f, ". Recourse: {CONTRADICTORY_RECOURSE}")
            }
            Self::Under {
                mate,
                parent,
                child,
                residual,
            } => write!(
                f,
                "{} does not determine {} from {}: {} survives. \
                 Recourse: {UNDER_RECOURSE}",
                mate_(*mate),
                instance_(*child),
                instance_(*parent),
                residual.describe()
            ),
            Self::DanglingHead { mate, side, head } => write!(
                f,
                "{}'s {} reference resolves through {}, which does not resolve to a \
                 live member (an instance, or a pattern-placed instance). Recourse: rebind the \
                 reference, or delete the mate",
                mate_(*mate),
                side.name(),
                by.node(*head)
            ),
            // `error` is the placer's own refusal, drawn on a line of its
            // own — the placer's row, or the mate's carried line — so
            // this sentence names the placer and points.
            Self::PlacerRefused {
                mate, side, placer, ..
            } => write!(
                f,
                "{}'s {} reference has no derived pose: {p}, on its derivation, \
                 refuses. Recourse: repair {p}",
                mate_(*mate),
                side.name(),
                p = by.node(*placer)
            ),
            Self::PartSelectsAnotherCopy {
                mate,
                side,
                part,
                named,
                selected,
            } => write!(
                f,
                "{}'s {} reference names copy {named}; the {p} above it selects \
                 copy {selected}, and a document may not place one copy and gather another. \
                 Recourse: set {p}'s index to copy {named}, or rebind the reference \
                 to copy {selected}",
                mate_(*mate),
                side.name(),
                p = by.node_as(*part, "part node")
            ),
            Self::SelfMate { mate, instance } => write!(
                f,
                "{} names one member on both sides (it stands on {}); a mate \
                 relates a PAIR. Recourse: rebind one side to another member, or delete the \
                 mate",
                mate_(*mate),
                instance_(*instance)
            ),
            Self::Unleverable { mate, refusal } => {
                write!(f, "{}: {}", mate_(*mate), Said(&**refusal, by))
            }
            Self::OffsetDisagrees {
                instance,
                root,
                predicate,
                clash,
            } => {
                write!(
                    f,
                    "{}'s offset disagrees with where its mates place it relative to \
                     its group's root, {} — predicate `{predicate}` ",
                    instance_(*instance),
                    instance_(*root)
                )?;
                write_clash(
                    f,
                    *clash,
                    "the offset and the solve would have had to agree",
                )?;
                write!(f, ". {}", crate::sentence::Recourse(OFFSET_RECOURSE))
            }
            Self::OffsetUnchecked { instance, cause } => {
                write!(
                    f,
                    "{}'s offset could not be checked against where its mates place \
                     it: {}",
                    instance_(*instance),
                    Said(&**cause, by)
                )?;
                // A lever refusal ends on its own recourse.
                match &**cause {
                    OffsetCheck::Placement { node, .. } => write!(
                        f,
                        ". {}",
                        crate::sentence::Recourse(format_args!(
                            "repair {}, or clear the offset",
                            by.node(*node)
                        ))
                    ),
                    OffsetCheck::Unleverable(_) => Ok(()),
                    OffsetCheck::Unreached { mate } => write!(
                        f,
                        ". {}",
                        crate::sentence::Recourse(format_args!(
                            "repair {}, or clear the offset",
                            mate_(*mate)
                        ))
                    ),
                    OffsetCheck::Indeterminate(_) => {
                        write!(f, ". {}", crate::sentence::Recourse("clear the offset"))
                    }
                }
            }
            Self::FaceUnresolved {
                mate,
                side,
                refusal,
            } => write!(
                f,
                "{}'s {} frame names a face that did not resolve to a pose: {}",
                mate_(*mate),
                side.name(),
                Said(&**refusal, by)
            ),
        }
    }
}

/// The fault where no document is at hand: each node by its tag.
impl core::fmt::Display for MateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl MateFault {
    /// **The fault as the frame holding the mate's document says it**:
    /// each node as `doc` holds it now ([`crate::Doc::spoken`]). A fault
    /// is memoized with the solve and the evaluation, so it holds ids,
    /// never a label.
    #[must_use]
    pub fn spoken<P>(&self, doc: &crate::doc::Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

impl core::error::Error for MateFault {}
