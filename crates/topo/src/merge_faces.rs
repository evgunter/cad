//! `merge_coplanar_faces` — explicit opt-in maximal-faces normalization
//! (M3 PR 1, fork F7): merge maximal runs of adjacent faces whose
//! planes are **structurally or declaredly** the same, killing the
//! shared edges and re-homing rings.
//!
//! Ch. 15's booleans require maximal-faced operands (no two adjacent
//! coplanar faces), and the seam zip *manufactures* coplanar pairs by
//! construction — so F7 ratified a fail-loud precondition on the
//! boolean side plus this **public, explicit** normalization op (the
//! M2 no-automatic-face-merging ratification: merging is never
//! silent; boolean outputs run this op as a documented final stage of
//! their own contract).
//!
//! **Coincidence discipline (the F6/round-8 ladder; N6 retirement,
//! M4 PR 5)**: two adjacent faces merge iff their surfaces are the
//! *same key* (structural), the *same [`crate::GeomSource`]*
//! (declared — shared recipe source, syntactic identity), or a
//! *declared face pair* of the call
//! ([`Body::merge_coplanar_faces_declared`] — recipe intent, verified
//! not trusted). The M3-era bit-identical-description rung is RETIRED:
//! a pair that is merely **numerically or bitwise** value-equal — same
//! plane, independent sources — stays unmerged **by design** (the
//! ladder's ratified rung (b): coincidence is never inferred from
//! values; the boolean's `NonMaximalFaces` gate agrees — the ladder is
//! consistent end to end). Since M5 PR 9 (C12.5) the hard rungs are
//! **kind-agnostic**: same-key and same-source CURVED neighbors merge
//! through the same never-numeric ladder (the cosurface
//! generalization — the boolean zip's cylinder-wall re-merge is the
//! named consumer); only the per-call declared-PAIR rung stays planar
//! (its verification predicate is `oriented_plane_eq`; the curved
//! counterpart's verification is the contact census's — CONTACT-DESIGN
//! C2's decision procedure and C4's per-class tables — not a
//! merge-local predicate). A face on the `mvfs` seed's placeholder
//! surface is a THIRD kind ([`MergeKind::Placeholder`]): the door
//! takes a census of them ([`MergeCoplanarOutcome::placeholders`]) and
//! glues none.
//!
//! Serves the ch. 15 boolean pipeline's operand precondition and
//! output stage (M3 PRs 4–5).

use std::collections::BTreeMap;

use geom::SurfaceKind;
use geom::{NetState, Surface};
use geom_brep::recourse::{Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite, Unsized};
use geom_core::{Band, BandError, Decide, Decided, Indeterminate, Tol};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::{
    PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, PlaneRung, oriented_plane_eq,
};
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, LoopKey, VertexKey};
use crate::euler::EulerOpError;
use crate::face_normal::plane_outward_normal;
use crate::geometry::SurfaceKey;
use crate::loop_winding::{LoopWinding, TornLoop, WINDING_PREDICATE};
use crate::readback::DanglingRef;
use crate::validate::{ValidationError, validate_closed};

/// One merged run: the surviving face and what was consumed into it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergedGroup {
    /// The surviving face: the group's first face in face-arena order
    /// that lies in no other member's hole (`outermost_survivor`) —
    /// the arena-first face, unless that face plugs a hole of another
    /// member.
    pub kept: FaceKey,
    /// The absorbed faces (dead keys), in kill order.
    pub absorbed: Vec<FaceKey>,
    /// The killed shared edges (dead keys), in kill order.
    pub killed_edges: Vec<EdgeKey>,
    /// Rings minted by intra-face shared-edge kills (`kemr` — a merged
    /// run that surrounds a hole grows a genuine ring), in mint order.
    pub rings_made: Vec<LoopKey>,
    /// Vertices deleted by the dangling-seam pruning, in kill order:
    /// each the free end of a shared edge the glue left dangling inside
    /// the merged face (the argument that this changes no region is
    /// stated once, at the pruning in `merge_group`). A contact record
    /// citing one is consumed and drops.
    ///
    /// Recorded rather than left implicit because this is the one
    /// thing the op destroys that no other field names: `absorbed`
    /// carries the dead faces and `killed_edges` the dead edges, and
    /// without this a caller reconciling the Euler delta would find a
    /// `v −1` with nothing accounting for it.
    pub killed_vertices: Vec<VertexKey>,
}

/// One record, two subjects, told apart by `reason`: a curved merge
/// GROUP that was NOT glued (its shape is outside the merge's Euler
/// inventory — loud in the record, never a silent drop, never a
/// partial commit), or a declared surface PAIR the door has no rung
/// for — a legal declaration on a non-planar carrier
/// ([`MergeCoplanarError::DeclaredCarrierUnsupported`]). A consumer
/// that walks `faces` treats both alike: faces the door left as they
/// were.
///
/// **The scope statements below are the KERNEL's, not the type's.**
/// Both fields are public and there is no private constructor, so
/// anything outside this crate can build a `SkippedMerge` holding any
/// [`MergeCoplanarError`] at all. What the kernel guarantees is about
/// the values IT produces — the ones reached through
/// [`MergeCoplanarOutcome::skipped`]; a record an external caller
/// mints carries no such promise and none is claimed for it.
#[derive(Clone, Debug, PartialEq)]
pub struct SkippedMerge {
    /// The faces the record is about: a group's faces in group
    /// order, or every live face on either surface of a declared
    /// pair in face-arena order. Never empty for a kernel-minted
    /// record.
    pub faces: Vec<FaceKey>,
    /// The inventory refusal that stopped the glue, carried whole:
    /// the same [`MergeCoplanarError`] vocabulary the door refuses
    /// with, so a caller matches this exactly as it would match an
    /// `Err` ([`Body::merge_coplanar_faces_declared`] states which
    /// group gets which regime). Two variants are scope-specific and
    /// tell the reader which gate spoke:
    /// [`MergeCoplanarError::GroupNotClosed`] reaches a caller only
    /// here, and [`MergeCoplanarError::ResultNotClosed`] — the whole
    /// run's after-gate — only as an `Err`.
    pub reason: MergeCoplanarError,
}

/// The outcome of one [`Body::merge_coplanar_faces`] call.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MergeCoplanarOutcome {
    /// The merged runs, in group order (first face's arena order).
    pub groups: Vec<MergedGroup>,
    /// Curved groups left unmerged as outside the inventory, with the
    /// refusal that stopped each; a planar group never lands here (it
    /// refuses the call). Non-empty needs no declaration: a curved run
    /// that would close its chart's full period is recorded here
    /// through either entry point. Declared pairs on a
    /// non-planar carrier are recorded here too, ahead of the group
    /// records, and survive a call that found nothing to merge.
    pub skipped: Vec<SkippedMerge>,
    /// Faces whose surface is the placeholder
    /// ([`MergeKind::Placeholder`]), in face-arena order — named so a
    /// caller holding a body still under construction can see why
    /// nothing happened to them, instead of reading `Ok` as "nothing
    /// to merge". None is a merge candidate; the rule is
    /// [`Body::merge_coplanar_faces_declared`]'s *The placeholder is
    /// a third kind, not a curved one*.
    pub placeholders: Vec<FaceKey>,
}

/// One half-edge resolved to the facts the merge's scans read through
/// it, taken in a single walk (see [`Body::edge_halves`]).
#[derive(Clone, Copy, Debug)]
struct HalfEdgeFacts {
    /// The parent loop.
    r#loop: LoopKey,
    /// The parent loop's face.
    face: FaceKey,
    /// The half-edge's start vertex.
    start: VertexKey,
}

/// The door's one kind question, asked of every live face
/// ([`Body::kind_census`]).
struct KindCensus<'a, T: geom_core::Real> {
    /// Each live face's kind.
    kinds: SecondaryMap<FaceKey, MergeKind>,
    /// Each live face's record and the surface it resolved to, which
    /// the adjacency test reads rather than looking the surface up.
    described: Described<'a, T>,
    /// The faces on a placeholder, in face-arena order.
    placeholders: Vec<FaceKey>,
}

/// [`KindCensus::described`]'s map.
type Described<'a, T> = SecondaryMap<FaceKey, (&'a crate::entity::Face, &'a Surface<T>)>;

/// What becomes of one group's INVENTORY refusal — the door's two
/// failure regimes, named so the boundary between them is a value
/// rather than a condition spelled inline.
///
/// Both regimes raise the SAME [`MergeCoplanarError`], which is why
/// nothing here duplicates that enum. Which regime a group runs under
/// is a property of the group: whether its surface is curved.
///
/// **The regime governs inventory failures and nothing else.** An
/// arena fault says nothing about the group, so it refuses the call
/// under BOTH regimes and never becomes a skip record. The split is
/// over inventory refusals; the escape is a class of refusal, not a
/// third regime. What puts a refusal in that class is stated once, in
/// [`Body::merge_coplanar_faces_declared`]'s *Two failure regimes*
/// section, and applied by
/// [`MergeCoplanarError::is_arena_fault`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GroupRegime {
    /// An inventory refusal is the CALL's refusal: nothing commits
    /// and the body is untouched. Planar runs, structural and declared
    /// alike: a planar group left unglued is two coplanar neighbours,
    /// which no boolean accepts as an operand, so a boolean whose
    /// output stage cannot glue one refuses its own step rather than
    /// ship a body the next boolean refuses.
    RefusesTheCall,
    /// An inventory refusal is recorded in
    /// [`MergeCoplanarOutcome::skipped`] and the remaining groups
    /// commit. Curved runs: a full-period closure keeps the operands'
    /// cut-carrying canonical form, which the boolean's maximal-faces
    /// gate accepts. The group is staged on its own clone and adopted
    /// only after its own tier-2 gate, so a recorded skip is never a
    /// partial commit.
    RecordsASkip,
}

impl GroupRegime {
    /// Whether one group's refusal is RECORDED (`true`) or refuses
    /// the whole call (`false`).
    ///
    /// This is the whole of the regime split, in one place with one
    /// production call site, so the rule and the code that applies it
    /// cannot drift: a refusal is recorded only under
    /// [`GroupRegime::RecordsASkip`] and only when it is an inventory
    /// refusal rather than an arena fault
    /// ([`MergeCoplanarError::is_arena_fault`], the class stated in
    /// [`Body::merge_coplanar_faces_declared`]).
    fn records(self, reason: &MergeCoplanarError) -> bool {
        self == Self::RecordsASkip && !reason.is_arena_fault()
    }
}

/// The surface kind the merge asks of a face — three answers, because
/// the two regimes are written for two kinds and the `mvfs` seed's
/// surface is neither. What the door does with each kind is stated
/// once, in [`Body::merge_coplanar_faces_declared`]'s *The placeholder
/// is a third kind, not a curved one*.
///
/// Which nets are the placeholder is [`NetState`]'s answer, read
/// through [`geom::NurbsSurface::net_state`] and never re-derived
/// here. A poisoned net has NO kind at this door: it is described
/// geometry that cannot evaluate, and [`NetState::Poisoned`]'s docs
/// require every consumer's described arm to FAIL on it rather than
/// hand it the placeholder's benign answer. The merge's curved arm
/// evaluates nothing, so it could not fail on its own — it would
/// commit surgery over a description tier 3 refuses at rest — and
/// the door refuses the face instead
/// ([`MergeCoplanarError::PoisonedSurfaceDescription`]).
///
/// Ordered so a refusal naming two members of different kinds names
/// them the same way whichever was the group's seed
/// ([`MergeCoplanarError::GroupKindSplit`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MergeKind {
    /// [`Surface::Plane`].
    Plane,
    /// Any described surface that is not a plane — the analytic
    /// kinds, a described NURBS, a fitted stand-in.
    Curved,
    /// [`NetState::Placeholder`]: no description yet.
    Placeholder,
}

/// A net in [`NetState::Poisoned`], which has no [`MergeKind`]
/// ([`MergeKind`]'s docs say why).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PoisonedNet;

impl MergeKind {
    /// The kind of one surface value. The plane question is
    /// [`geom::Surface::kind`]'s — the crate's one carrier-kind read —
    /// and the net question is [`NetState`]'s, matched exhaustively so
    /// no state is answered by a default.
    fn of<T: geom_core::Real>(surface: &Surface<T>) -> Result<Self, PoisonedNet> {
        match surface {
            Surface::Nurbs(net) => match net.net_state() {
                NetState::Placeholder => Ok(Self::Placeholder),
                NetState::Poisoned => Err(PoisonedNet),
                NetState::Described => Ok(Self::Curved),
            },
            s if s.kind() == SurfaceKind::Plane => Ok(Self::Plane),
            _ => Ok(Self::Curved),
        }
    }

    /// The kind's name, for a rendered refusal: one adjective each.
    fn name(self) -> &'static str {
        match self {
            Self::Plane => "planar",
            Self::Curved => "curved",
            Self::Placeholder => "placeholder",
        }
    }
}

/// What the door does with one group, decided from its members'
/// kinds — every member's, because the hard rungs glue on identity
/// and never ask the kind ([`Body::group_contract`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GroupContract {
    /// Described faces of one kind: the surgery runs, its inventory
    /// refusals placed by `regime`, and `kind` — [`MergeKind::Plane`]
    /// or [`MergeKind::Curved`], never the placeholder — tells it what
    /// a same-face duplicate on the survivor means: a ring on a plane,
    /// a closed period on a chart.
    Runs {
        regime: GroupRegime,
        kind: MergeKind,
    },
    /// Every member is a placeholder: no surgery, no skip
    /// ([`Body::merge_coplanar_faces_declared`], *The placeholder is a
    /// third kind, not a curved one*).
    SetAside,
}

/// A refused [`Body::merge_coplanar_faces`] call (closed enum, D3
/// style). **Returned as an `Err`, the body is untouched on every
/// variant** — the op stages its work on a clone and commits only a
/// tier-2-valid result. The same values also ride
/// [`SkippedMerge::reason`], where they are not refusals of the call
/// and carry no such promise: the call succeeded and the groups that
/// merged are in the body.
#[derive(Clone, Debug, PartialEq)]
pub enum MergeCoplanarError {
    /// The input is not a tier-2 closed solid — normalization is
    /// defined on at-rest bodies ("tier-valid before").
    InputNotClosed {
        /// The tier-1/2 failures.
        errors: Vec<ValidationError>,
    },
    /// The WHOLE run's merged result failed tier 2 ("tier-valid
    /// after") — the configuration is outside what this op can safely
    /// merge (e.g. a kill sequence that would strand scaffolding);
    /// refused whole, nothing commits. Its scope is the run: every
    /// group that merged is in the abandoned body. One group's own
    /// trial failing is the separate [`MergeCoplanarError::GroupNotClosed`],
    /// so a consumer holding either always knows which gate spoke.
    ResultNotClosed {
        /// The tier-1/2 failures of the abandoned attempt.
        errors: Vec<ValidationError>,
    },
    /// ONE group's staged trial failed tier 2, so that group is not
    /// adopted. Scope is the single group, named by the record that
    /// carries this ([`SkippedMerge::faces`]); the run continues and
    /// `work` is exactly as it was before the trial. The kernel
    /// raises it only under [`GroupRegime::RecordsASkip`] — the
    /// refusing regime runs no sub-stage gate — so it never appears
    /// as an `Err` of the door. That is a fact about where the kernel
    /// constructs it, not something the type enforces.
    GroupNotClosed {
        /// The tier-1/2 failures of the abandoned trial.
        errors: Vec<ValidationError>,
    },
    /// A merge group's members are not all of one surface KIND
    /// ([`MergeKind`]), so the run is neither a planar run nor a
    /// curved one and there is no regime to give it.
    ///
    /// The hard rungs glue on *source* identity, not on kind, and a
    /// source can join descriptions nothing compared — the graft copies
    /// origin rows verbatim, and [`Body::set_surface_source`] checks
    /// agreement only where debug assertions are compiled in — so a
    /// body can declare a plane and a cylinder to be one recipe
    /// surface, or a plane and a placeholder that describes nothing
    /// yet. Deciding the group's kind off one member would let arena
    /// order pick its contract; this refuses instead. The two members
    /// are named in [`MergeKind`] order, so the same pair reads the
    /// same way whichever face seeded the group.
    GroupKindSplit {
        /// A member.
        face: FaceKey,
        /// Its kind.
        kind: MergeKind,
        /// A member of another kind.
        other: FaceKey,
        /// That member's kind.
        other_kind: MergeKind,
    },
    /// A face's surface is a `Nurbs` net in [`NetState::Poisoned`]:
    /// described geometry that cannot evaluate. Tier 3 refuses such a
    /// body at rest ([`ValidationError::PoisonedSurfaceDescription`]);
    /// this door's entry gate is tier 2, so it refuses the face itself,
    /// before any group forms — its curved arm would otherwise commit
    /// surgery over a description nothing certified ([`MergeKind`]).
    /// Distinct from the placeholder, which is set aside and named,
    /// never refused.
    PoisonedSurfaceDescription {
        /// The face whose surface net carries the poison.
        face: FaceKey,
    },
    /// A shared edge's two halves lie in **different loops of one
    /// face** after absorption (a ring-adjacent merge shape) — outside
    /// the M2+PR-7 inventory this op handles; refused rather than
    /// guessed at (the kev/ring bookkeeping for it arrives with the
    /// pipeline that produces it, if any does).
    UnsupportedConfiguration {
        /// The edge the op cannot safely kill.
        edge: EdgeKey,
    },
    /// A CURVED cosurface run would close its chart's full period
    /// (M5 PR 9, C12.5): killing the last shared edge leaves either a
    /// ring on a curved face or a full-wrap seam-form loop — shapes
    /// the exact-B-rep props cannot integrate yet, so the run refuses
    /// here and the driver records a loud skip (the operands'
    /// cut-carrying canonical form stays; sub-period re-merges — the
    /// through-cut case — commit normally). FLIP NOTE: the du_of_rims
    /// per-level-sum repair may already make the kept-cut seam form
    /// integrable; re-evaluating this skip belongs to M5 PR 11 (the
    /// tessellation/props unit), which owns the curved-face
    /// quadrature story.
    PeriodClosure {
        /// The shared edge whose kill would close the period.
        edge: EdgeKey,
    },
    /// An internal Euler step refused — surfaced typed (unreachable on
    /// tier-2 input in the supported inventory; never a panic, D9).
    /// Also the spelling for a dangling reference the surgery's own
    /// plan steps observe: [`crate::DanglingRef`] maps into
    /// [`EulerOpError::StaleKey`] / [`EulerOpError::StaleGeometry`],
    /// the state is the one the operators name and the caller's
    /// recourse is identical, so it is not given a second name here.
    /// Those two are arena faults and refuse the call under both
    /// failure regimes, as do the other operator refusals the class
    /// covers — which is wider than a dangling reference and is
    /// stated in [`Body::merge_coplanar_faces_declared`].
    Op {
        /// The refusing operator's error.
        error: EulerOpError,
    },
    /// A declared surface pair references a key that does not
    /// resolve, or names two surfaces of DIFFERENT kinds — a torn
    /// argument, refused up front. A pair on one non-planar kind is
    /// NOT this: it is a legal declaration recorded as
    /// [`MergeCoplanarError::DeclaredCarrierUnsupported`].
    InvalidDeclaration {
        /// The offending surface key.
        surface: SurfaceKey,
        /// What was wrong.
        what: &'static str,
    },
    /// A declared face pair's planes are DEFINITELY distinct — the
    /// declaration contradicts the geometry; refused loudly, never
    /// glued (M4 PR 5; `plane_eq` rung 2's verification direction).
    DeclarationContradicted {
        /// The fact that contradicted the declaration: `PlanesNotParallel`
        /// or `PlanesApart`, the two the declared plane rung raises. The
        /// field keeps the rung's own type (`PlaneEqError` is
        /// `CarrierEqError`), which a narrower one would convert from
        /// fallibly at the one raise site.
        fact: crate::boolean::Contradiction,
    },
    /// A declared face pair meets with OPPOSITE orientations at a
    /// shared edge — no valid closed solid merges such a pair; the
    /// declaration cannot be honored here.
    DeclaredOppositeOrientation {
        /// The pair's first face (arena order at the meeting edge).
        f1: FaceKey,
        /// The second face.
        f2: FaceKey,
    },
    /// A declared surface pair lies on one NON-PLANAR carrier kind.
    /// The declaration is legal — the boolean's declarable inventory
    /// admits it and it served the consuming op's classification —
    /// but this door's declared-pair rung is planar and has no arm
    /// for the kind, so the pair is left unmerged and RECORDED. Scope
    /// is the pair: the kernel constructs this only as a
    /// [`SkippedMerge::reason`], never as an `Err` of the door, and
    /// it is an inventory statement, not an arena fault. Like
    /// [`MergeCoplanarError::GroupNotClosed`], that is a fact about
    /// where the kernel constructs it, not something the type
    /// enforces. The record is keyed off the DECLARATION: it states
    /// what this door did with the caller's argument, not what the
    /// geometry admits — a pair whose surviving faces are a slit's
    /// two sides is recorded the same as one a curved arm would glue.
    /// A pair with no live face on either surface is not recorded at
    /// all, so a record's `faces` is never empty.
    DeclaredCarrierUnsupported {
        /// The declared surface pair, as the caller passed it. Both
        /// keys resolved when the door read them, but either may be
        /// gone from the body it returns: a surface held only by an
        /// edge curve goes with that curve when the edge is
        /// re-described, by this door's kept boundaries or by a caller
        /// afterwards. A consumer walks the record's `faces`, not the
        /// pair, for what is live.
        pair: (SurfaceKey, SurfaceKey),
        /// The carrier kind both surfaces share.
        kind: SurfaceKind,
    },
    /// After absorbing a group, the survivor's loops admit no unique
    /// positively-wound outline, so the outer/ring roles cannot be
    /// assigned; refused rather than guessed. The intra-face `kemr`
    /// designates its ring provisionally and the winding resolves it.
    /// A zero winding that leaves no outline is not this variant: it
    /// is the winding decision's own band-decided refusal,
    /// [`MergeCoplanarError::Escalated`] with
    /// [`MergeDecision::LoopWinding`] and the winding's margin.
    MergedFaceRoleAmbiguous {
        /// The merged survivor face.
        face: FaceKey,
        /// Why no loop is the unique outline.
        verdict: OutlineVerdict,
    },
    /// A decision of the merge could not be taken at this tolerance —
    /// typed, never guessed.
    Escalated {
        /// The decision, which the refusal's ending follows from.
        decision: MergeDecision,
        /// Its diagnostics: the margin in band, or the one a rung
        /// decided at zero where zero does not pass.
        diag: Indeterminate,
    },
    /// The run's tolerance cannot form a valid band (needed only when
    /// declared pairs are present).
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// The staged result's pcurve RE-MINT refused (M6-3: an input that
    /// carried stored caches re-mints them on the staged clone before
    /// commit — the `topo::pcurves` module docs' rule for ops that
    /// mutate minted bodies; the merged loops' one-branch walks are
    /// derived fresh, never stitched from the absorbed fragments'
    /// rows). The body is untouched, exactly as on every other
    /// variant.
    Pcurve {
        /// The mint pass's typed refusal.
        source: crate::pcurves::PcurveMintError,
    },
    /// A kept face's boundary edge could not be re-described against
    /// the two faces the merge left it between.
    KeptBoundaryUndescribed {
        /// The kept face.
        face: FaceKey,
        /// Its boundary edge.
        edge: EdgeKey,
        /// What could not be done.
        failure: EdgeDescribeFailure,
    },
    /// Whether the two faces at a kept face's boundary edge cross or
    /// meet smoothly could not be decided at this tolerance, so the edge
    /// has no honest description: refused, never guessed.
    KeptBoundaryUndecided {
        /// The kept face.
        face: FaceKey,
        /// Its boundary edge.
        edge: EdgeKey,
        /// The reading that could not decide.
        reading: DihedralReading,
        /// Its diagnostics.
        diag: Indeterminate,
    },
}

/// What the edge describer could not do at one edge
/// ([`MergeCoplanarError::KeptBoundaryUndescribed`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeDescribeFailure {
    /// The edge carries no certified curve to re-describe, or its
    /// halves, faces, vertices or surfaces do not resolve.
    NotWalkable,
    /// The two faces' intersection does not certify on the edge's
    /// carrier.
    Intersection,
    /// Their tangency does not certify on the edge's carrier.
    Tangency,
    /// The carrier has no conventional description in a chart.
    NoConventionalLane,
    /// The arc's image in its face's chart does not certify.
    Arc,
    /// The line's image in its face's chart does not certify.
    Line,
}

impl EdgeDescribeFailure {
    /// The failure, as a clause with no colon or dash of its own.
    const fn clause(self) -> &'static str {
        match self {
            Self::NotWalkable => {
                "it carries no certified curve, or its faces, vertices or surfaces do not resolve"
            }
            Self::Intersection => "its two faces' intersection does not certify on its curve",
            Self::Tangency => "its two faces' tangency does not certify on its curve",
            Self::NoConventionalLane => {
                "its curve has no description in a face's chart for its kind"
            }
            Self::Arc => "its arc does not certify as an image in its face's chart",
            Self::Line => "its line does not certify as an image in its face's chart",
        }
    }
}

/// The reading of the two faces at an edge that could not decide
/// ([`MergeCoplanarError::KeptBoundaryUndecided`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DihedralReading {
    /// The first-order dihedral: the angle between the faces, metered
    /// over the edge, at the rung that could not decide.
    Lever(geom_brep::LeverRung),
    /// The second-order bend of two faces that meet smoothly.
    Bend,
}

/// Why the edge describer refused one edge, before a door words it:
/// the merge door as [`MergeCoplanarError::KeptBoundaryUndescribed`] /
/// [`MergeCoplanarError::KeptBoundaryUndecided`], the boolean in its
/// own vocabulary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum DescribeRefusal {
    /// The edge could not be described.
    Failed {
        edge: EdgeKey,
        failure: EdgeDescribeFailure,
    },
    /// A reading of its two faces could not decide.
    Undecided {
        edge: EdgeKey,
        reading: DihedralReading,
        diag: Indeterminate,
    },
}

impl DescribeRefusal {
    pub(crate) const fn failed(edge: EdgeKey, failure: EdgeDescribeFailure) -> Self {
        Self::Failed { edge, failure }
    }

    pub(crate) const fn undecided(
        edge: EdgeKey,
        reading: DihedralReading,
        diag: Indeterminate,
    ) -> Self {
        Self::Undecided {
            edge,
            reading,
            diag,
        }
    }

    const fn edge(self) -> EdgeKey {
        match self {
            Self::Failed { edge, .. } | Self::Undecided { edge, .. } => edge,
        }
    }
}

impl MergeCoplanarError {
    /// The door's refusal for `refusal` at one of `boundary`'s edges,
    /// naming the kept face it was walked from.
    pub(crate) fn of_kept_boundary(
        boundary: &[(FaceKey, EdgeKey)],
        refusal: DescribeRefusal,
    ) -> Self {
        let Some(&(face, _)) = boundary.iter().find(|(_, e)| *e == refusal.edge()) else {
            unreachable!(
                "merge_coplanar_faces: the describer refused edge {:?}, which is not on the \
                 worklist it was handed — it describes only the edges it is given",
                refusal.edge()
            )
        };
        match refusal {
            DescribeRefusal::Failed { edge, failure } => Self::KeptBoundaryUndescribed {
                face,
                edge,
                failure,
            },
            DescribeRefusal::Undecided {
                edge,
                reading,
                diag,
            } => Self::KeptBoundaryUndecided {
                face,
                edge,
                reading,
                diag,
            },
        }
    }

    /// The describer's refusal this one words, for a caller that words
    /// it in its own vocabulary; `None` for every other refusal.
    pub(crate) const fn kept_boundary(&self) -> Option<DescribeRefusal> {
        match *self {
            Self::KeptBoundaryUndescribed { edge, failure, .. } => {
                Some(DescribeRefusal::failed(edge, failure))
            }
            Self::KeptBoundaryUndecided {
                edge,
                reading,
                diag,
                ..
            } => Some(DescribeRefusal::undecided(edge, reading, diag)),
            _ => None,
        }
    }

    /// The refusal of the plane identity verifying a declared pair.
    pub(crate) fn of_declared_refusal(refusal: PlaneEqError) -> Self {
        match refusal {
            PlaneEqError::Contradicted { fact, .. } => Self::DeclarationContradicted { fact },
            PlaneEqError::Escalated { rung, diag } => Self::Escalated {
                decision: MergeDecision::DeclaredPlanes(rung),
                diag,
            },
            PlaneEqError::Unsettled { diag } => Self::Escalated {
                decision: MergeDecision::DeclaredReach,
                diag,
            },
            // Unreachable with `declared: true`; refuse loudly anyway.
            PlaneEqError::Undeclared { diag, .. } => Self::Escalated {
                decision: MergeDecision::DeclaredOffset,
                diag,
            },
        }
    }
}

/// What the merge does with the declared plane rung's `verdict` on the
/// pair `(f1, f2)` meeting at a shared edge: it glues a pair that faces
/// the same way and refuses every other verdict.
pub(crate) fn declared_pair_verdict(
    verdict: Result<PlaneRelation, PlaneEqError>,
    f1: FaceKey,
    f2: FaceKey,
) -> Result<bool, MergeCoplanarError> {
    match verdict {
        Ok(PlaneRelation::SameOriented) => Ok(true),
        Ok(PlaneRelation::SameOpposite) => {
            Err(MergeCoplanarError::DeclaredOppositeOrientation { f1, f2 })
        }
        // Unreachable through the declared rung; kept typed.
        Ok(PlaneRelation::Distinct) => Ok(false),
        Err(refusal) => Err(MergeCoplanarError::of_declared_refusal(refusal)),
    }
}

/// Why a merged face has no unique outline
/// ([`MergeCoplanarError::MergedFaceRoleAmbiguous`]), each loop's
/// winding read about the face's outward normal.
///
/// `SeveralPositive` and `AllNegative` are the ROLE decision's
/// verdicts, not the winding's: every loop's winding was decided, and
/// it is the count of outlines among them that is wrong. A winding the
/// band cannot decide, or decides zero, is the winding decision's own
/// refusal ([`MergeDecision::LoopWinding`]), never one of these.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutlineVerdict {
    /// Several loops wind positively: each bounds a region of its own,
    /// and one face bounds one connected region.
    SeveralPositive {
        /// The positively-wound loops, outline slot first.
        loops: Vec<LoopKey>,
    },
    /// No loop winds positively or zero, and this one rides a carrier
    /// the kernel does not wind (a NURBS or spiric edge): it may be the
    /// outline, and which loop is cannot be read until that winding is
    /// built.
    UnsupportedWinding {
        /// The first such loop, outline slot first.
        r#loop: LoopKey,
    },
    /// Every loop winds negatively or is a lone vertex (which bounds
    /// no area): none bounds the face, so the face's sense disagrees
    /// with its boundary.
    AllNegative,
}

/// Which decision [`MergeCoplanarError::Escalated`] could not take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MergeDecision {
    /// A rung of the plane identity verifying a pair the caller
    /// declared (`plane_eq`'s declared rung). The declaration is
    /// already there, so no ending offers one.
    DeclaredPlanes(PlaneRung),
    /// Whether a declared pair's parallel planes lie apart. The declared
    /// rung contradicts a definite offset and bridges an in-band one, so
    /// it never escalates this; kept typed for the refusal that would
    /// be a kernel defect.
    DeclaredOffset,
    /// Whether a declared pair's planes stay within the band across the
    /// extent the rung reads them over: their displacement's upper
    /// bound stands past the band and its lower bound does not
    /// (`carrier_eq::CarrierEqError::Unsettled`).
    DeclaredReach,
    /// Which way a loop of the merged face winds about its normal,
    /// which decides the outline among its loops.
    LoopWinding,
}

impl MergeDecision {
    /// What the decision decides, as a clause with no colon or dash of
    /// its own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::DeclaredPlanes(PlaneRung::Orientation) => {
                "whether the two declared faces face the same way across the edge they share"
            }
            Self::DeclaredPlanes(rung @ (PlaneRung::Parallel | PlaneRung::Norm)) => rung.subject(),
            Self::DeclaredOffset => "whether the two declared planes lie apart",
            Self::DeclaredReach => {
                "whether the two declared planes stay within the tolerance of \
                                    one another across the faces"
            }
            Self::LoopWinding => "which way a loop of the merged face winds about its normal",
        }
    }

    /// The one ending an escalation of this decision carries, at the
    /// merge that built the geometry.
    fn ending(self, diag: &Indeterminate) -> String {
        let arm = RefusedArm::Undecided(diag);
        match self {
            Self::DeclaredPlanes(PlaneRung::Orientation) => {
                DECLARED_ORIENTATION.recourse(arm, Reading::Build)
            }
            // The declared rung bridges in-band parallelism, so what
            // escalates here is a norm the rung could not read, as at every
            // Boolean door (`SelfCheck::Normals`).
            Self::DeclaredPlanes(PlaneRung::Parallel | PlaneRung::Norm) | Self::DeclaredOffset => {
                Unsized::Defect.recourse(arm, Reading::Build)
            }
            // The displacement is a bound over a ball enclosing the
            // faces, not a reading of them.
            Self::DeclaredReach => Unsized::LastResort.recourse(arm, Reading::Build),
            Self::LoopWinding => LOOP_WINDING.recourse(arm, Reading::Build),
        }
    }
}

/// The merge's orientation decision on a declared pair meeting at a
/// shared edge. Only a pair that faces the same way glues, so it passes
/// on a positive margin, and [`MergeCoplanarError::DeclaredOppositeOrientation`]
/// is its sign-certain arm. The margin is the outward normals' cosine
/// levered at the radius of a ball enclosing both faces, so `|cos| ≈ 1`
/// wherever the planes stand near parallel, and what an undecided
/// margin measures is that reach: the lever names both moves a refusal
/// may need.
const DECLARED_ORIENTATION: SizedDecision = SizedDecision {
    lever: "turn one of the two faces so both clearly face the same way, on faces that clearly \
            span a length",
    size: "span of the two faces",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The winding decision: its margin is the loop's signed area over its
/// length. A positive loop is an outline, a negative one a hole, and a
/// zero one neither, so it passes on either nonzero sign. Its zero arm
/// refuses where the zero winding leaves the merged face no outline
/// ([`Body::merged_outline_ring`]), with the margin it was decided on.
const LOOP_WINDING: SizedDecision = SizedDecision {
    lever: "reshape the merged faces so each loop of the result clearly encloses an area",
    size: "area a loop encloses over its length",
    passes: SizedPass::NonZero,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

impl core::fmt::Display for MergeCoplanarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InputNotClosed { errors } => {
                write!(
                    f,
                    "merge_coplanar_faces: input is not tier-2 ({} errors)",
                    errors.len()
                )
            }
            Self::ResultNotClosed { errors } => write!(
                f,
                "merge_coplanar_faces: merged result failed tier 2 ({} errors); refused",
                errors.len()
            ),
            Self::GroupNotClosed { errors } => write!(
                f,
                "merge_coplanar_faces: this group's staged merge failed tier 2 ({} errors); \
                 the group is left unmerged and the run continues",
                errors.len()
            ),
            Self::GroupKindSplit {
                face,
                kind,
                other,
                other_kind,
            } => {
                write!(
                    f,
                    "merge_coplanar_faces: group members {face:?} ({}) and {other:?} ({}) are \
                     one group but not one surface kind — the run is neither planar nor \
                     curved; re-check the shared surface source that joined them",
                    kind.name(),
                    other_kind.name()
                )?;
                if [*kind, *other_kind].contains(&MergeKind::Placeholder) {
                    write!(
                        f,
                        " (a placeholder describes no locus and is never one recipe surface \
                         with a described face)"
                    )?;
                }
                Ok(())
            }
            Self::PoisonedSurfaceDescription { face } => write!(
                f,
                "merge_coplanar_faces: face {face:?}'s surface is a Nurbs net carrying poison \
                 in some channel — described geometry that cannot evaluate, which tier 3 \
                 refuses at rest; attach a real description (Body::set_face_surface) before \
                 merging"
            ),
            Self::UnsupportedConfiguration { edge } => write!(
                f,
                "merge_coplanar_faces: shared edge {edge:?} spans two loops of \
                 one face — unsupported configuration, refused"
            ),
            Self::PeriodClosure { edge } => write!(
                f,
                "merge_coplanar_faces: killing shared edge {edge:?} would close the \
                 curved cosurface run's full chart period — outside the merge's \
                 inventory: sub-period re-merges commit, full closures stay in their \
                 cut-carrying canonical form and are recorded as a loud skip"
            ),
            Self::Op { error } => write!(f, "merge_coplanar_faces: {error}"),
            Self::InvalidDeclaration { surface, what } => write!(
                f,
                "merge_coplanar_faces: invalid declared pair at surface {surface:?}: {what}"
            ),
            Self::DeclarationContradicted { fact } => write!(
                f,
                "a declared coincidence contradicts the geometry: {}, and the merge never \
                 glues a lie. {}",
                fact.fact(),
                crate::contact::CONTRADICTION_RECOURSE
            ),
            Self::DeclaredOppositeOrientation { .. } => write!(
                f,
                "the two declared faces face opposite ways across the edge they share. {}",
                DECLARED_ORIENTATION.recourse(RefusedArm::SignCertain, Reading::Build)
            ),
            Self::DeclaredCarrierUnsupported { pair, kind } => write!(
                f,
                "merge_coplanar_faces: declared pair {pair:?} lies on a {kind} carrier — \
                 the declaration is legal and served the op, but this door's declared-pair \
                 rung is planar and has no {kind} arm; the pair is left unmerged and recorded",
                kind = kind.name()
            ),
            Self::MergedFaceRoleAmbiguous { face, verdict } => match verdict {
                OutlineVerdict::SeveralPositive { loops } => write!(
                    f,
                    "{} loops of the merged face {face:?} each wind counterclockwise about its \
                     outward normal ({loops:?}), so each bounds a region of its own and one \
                     face cannot bound them all. Recourse: reshape the merged faces so their \
                     union is one connected region",
                    loops.len()
                ),
                OutlineVerdict::UnsupportedWinding { r#loop } => write!(
                    f,
                    "no loop of the merged face {face:?} winds counterclockwise about its \
                     outward normal, and loop {loop:?} rides a NURBS or spiric edge, whose \
                     winding the kernel does not read, so which loop is the outline is not \
                     known. Recourse: bound the merged faces with line, \
                     circle or ellipse edges, whose winding the kernel reads"
                ),
                OutlineVerdict::AllNegative => write!(
                    f,
                    "every loop of the merged face {face:?} winds clockwise about its outward \
                     normal or is a lone vertex, so none bounds the face; its sense disagrees \
                     with its boundary. Recourse: orient the faces being merged so each outline \
                     winds counterclockwise about its outward normal, as tier 3 requires at rest"
                ),
            },
            Self::Escalated { decision, diag } => write!(
                f,
                "{} is undecided: {}. {}",
                decision.subject(),
                diag.payload(),
                decision.ending(diag)
            ),
            Self::Band { error } => write!(f, "merge_coplanar_faces: {error}"),
            Self::Pcurve { source } => write!(
                f,
                "merge_coplanar_faces: the staged result's pcurve re-mint refused \
                 ({source}) — the body is untouched"
            ),
            Self::KeptBoundaryUndescribed {
                face,
                edge,
                failure,
            } => write!(
                f,
                "merge_coplanar_faces: kept face {face:?}'s boundary edge {edge:?} cannot be \
                 re-described against the two faces it now lies between: {}. The body is \
                 untouched",
                failure.clause()
            ),
            Self::KeptBoundaryUndecided {
                face,
                edge,
                reading,
                diag,
            } => write!(
                f,
                "merge_coplanar_faces: re-describing kept face {face:?}'s boundary edge \
                 {edge:?}, {} is undecided: {}. Recourse: move the geometry so the faces at \
                 that edge clearly cross or are clearly smooth",
                match reading {
                    DihedralReading::Lever(geom_brep::LeverRung::Arm) => {
                        "whether the edge is long enough to measure the faces' angle over"
                    }
                    DihedralReading::Lever(geom_brep::LeverRung::Reading) => {
                        "whether the faces at it cross or meet smoothly"
                    }
                    DihedralReading::Bend => "whether the faces at it bend apart",
                },
                diag.payload()
            ),
        }
    }
}

impl std::error::Error for MergeCoplanarError {}

impl From<EulerOpError> for MergeCoplanarError {
    fn from(error: EulerOpError) -> Self {
        Self::Op { error }
    }
}

/// The crate's dangling-reference vocabulary reaches this door
/// through the operator layer's own mapping, so a failed lookup here
/// is spelled once and named the same way it is everywhere else.
impl From<DanglingRef> for MergeCoplanarError {
    fn from(what: DanglingRef) -> Self {
        Self::Op {
            error: EulerOpError::from(what),
        }
    }
}

// ---- Test-only tear points -------------------------------------
//
// `merge_group` RE-CHECKS each fact it relies on immediately before
// the operator call that could contradict it (the `debug_assert!`
// rows in the surgery), so a refusal of a contradicted variant on a
// production build means the arena moved in the instruction window
// between the proof and the call. Nothing outside a test build can
// reach that window — the door's entry gate refuses a torn INPUT
// before any group is staged — so a test arms one of these points,
// which sit AFTER the re-check and immediately before the call, and
// mutate exactly the fact the re-check just proved. That placement is
// the point: the tear models the window the re-check cannot cover.

/// One falsifiable fact of `merge_group`'s surgery, named where it is
/// established, re-checked and torn.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TearPoint {
    /// Before `ring_move`: makes the ring its own face's outer loop.
    RingBecomesItsFacesOuter,
    /// Before `ring_move`: bends the ring's face onto a foreign shell.
    RingsFaceLeavesTheShell,
    /// Before `kef`: re-parents the dying half onto its mate's loop.
    DyingHalfJoinsItsMatesLoop,
    /// Before `kef`: puts a ring back onto the drained face.
    DrainedFaceRegainsARing,
    /// Before `kev`: collapses the strut edge onto one vertex.
    StrutBecomesASelfLoop,
    /// Before `kemr`: re-parents the duplicate's minus half elsewhere.
    DuplicateHalvesPartCompany,
    /// Before the role pass: drops the survivor's surface. Not a
    /// re-checked fact: a key the role pass looks up after the surgery
    /// has mutated the arena, torn where no input can tear it.
    SurvivorLosesItsSurface,
    /// Before the role pass: drops the point of the survivor's
    /// outline's first vertex (a lookup tear, as above).
    OutlineLosesAPoint,
}

#[cfg(test)]
thread_local! {
    static ARMED_TEAR: std::cell::Cell<Option<TearPoint>> = const { std::cell::Cell::new(None) };
}

/// Arms one tear point for the guard's lifetime and disarms on drop,
/// so a panic inside the door — an assertion in the row itself, or a
/// `debug_assert!` in the surgery — cannot leave it armed for the
/// next row on this thread.
#[cfg(test)]
pub(crate) struct ArmedTear;

#[cfg(test)]
impl ArmedTear {
    fn at(point: TearPoint) -> Self {
        ARMED_TEAR.with(|c| c.set(Some(point)));
        Self
    }
}

#[cfg(test)]
impl Drop for ArmedTear {
    fn drop(&mut self) {
        ARMED_TEAR.with(|c| c.set(None));
    }
}

#[cfg(test)]
fn armed_tear() -> Option<TearPoint> {
    ARMED_TEAR.with(std::cell::Cell::get)
}

/// The tear points offered immediately before `ring_move`.
#[cfg(test)]
fn tear_before_ring_move<T: geom_core::Real>(body: &mut Body<T>, ring: LoopKey) {
    let Some(point) = armed_tear() else { return };
    let Some(face_key) = body.get_loop(ring).map(|l| l.face) else {
        return;
    };
    let Some(face) = body.faces.get_mut(face_key) else {
        return;
    };
    match point {
        TearPoint::RingBecomesItsFacesOuter => face.outer = ring,
        TearPoint::RingsFaceLeavesTheShell => face.shell = crate::entity::ShellKey::default(),
        _ => {}
    }
}

/// The tear points offered immediately before `kef`.
#[cfg(test)]
fn tear_before_kef<T: geom_core::Real>(
    body: &mut Body<T>,
    dying_he: crate::entity::HalfEdgeKey,
    edge: EdgeKey,
    dying_face: FaceKey,
) {
    match armed_tear() {
        Some(TearPoint::DyingHalfJoinsItsMatesLoop) => {
            let Some(e) = body.get_edge(edge).cloned() else {
                return;
            };
            let mate = if e.he_plus == dying_he {
                e.he_minus
            } else {
                e.he_plus
            };
            let Some(mates_loop) = body.get_half_edge(mate).map(|h| h.parent_loop) else {
                return;
            };
            if let Some(h) = body.half_edges.get_mut(dying_he) {
                h.parent_loop = mates_loop;
            }
        }
        Some(TearPoint::DrainedFaceRegainsARing) => {
            let Some(outer) = body.get_face(dying_face).map(|f| f.outer) else {
                return;
            };
            if let Some(face) = body.faces.get_mut(dying_face) {
                face.rings.push(outer);
            }
        }
        _ => {}
    }
}

/// The tear point offered immediately before `kev`.
#[cfg(test)]
fn tear_before_kev<T: geom_core::Real>(
    body: &mut Body<T>,
    from_rim: crate::entity::HalfEdgeKey,
    edge: EdgeKey,
) {
    if armed_tear() != Some(TearPoint::StrutBecomesASelfLoop) {
        return;
    }
    let Some(e) = body.get_edge(edge).cloned() else {
        return;
    };
    let mate = if e.he_plus == from_rim {
        e.he_minus
    } else {
        e.he_plus
    };
    let Some(start) = body.get_half_edge(from_rim).map(|h| h.start) else {
        return;
    };
    if let Some(h) = body.half_edges.get_mut(mate) {
        h.start = start;
    }
}

/// The tear point offered immediately before `kemr`.
#[cfg(test)]
fn tear_before_kemr<T: geom_core::Real>(body: &mut Body<T>, he_minus: crate::entity::HalfEdgeKey) {
    if armed_tear() != Some(TearPoint::DuplicateHalvesPartCompany) {
        return;
    }
    if let Some(h) = body.half_edges.get_mut(he_minus) {
        h.parent_loop = LoopKey::default();
    }
}

/// The lookup tears offered immediately before the role pass.
#[cfg(test)]
fn tear_before_role_pass<T: geom_core::Real>(body: &mut Body<T>, survivor: &crate::entity::Face) {
    match armed_tear() {
        Some(TearPoint::SurvivorLosesItsSurface) => {
            body.surfaces.remove(survivor.surface);
        }
        Some(TearPoint::OutlineLosesAPoint) => {
            let point = body
                .get_loop(survivor.outer)
                .and_then(|l| match l.boundary {
                    crate::entity::LoopBoundary::Cycle { first } => body.get_half_edge(first),
                    crate::entity::LoopBoundary::Empty { .. } => None,
                })
                .and_then(|h| body.get_vertex(h.start))
                .map(|v| v.point);
            if let Some(point) = point {
                body.points.remove(point);
            }
        }
        _ => {}
    }
}

/// A fact `merge_group` establishes, **re-checks immediately before**
/// the operator call that could contradict it, and therefore owns.
///
/// A fact the door merely read earlier is worth nothing here: the
/// door's own mutations run in between, and a fact the surgery
/// invalidates itself is not contradicted by an operator reporting
/// it. `kef`'s [`EulerOpError::SameFace`] is the case that taught it —
/// the absorption's `ring_move` drain re-homes the dying loop onto
/// the survivor, so on a nested (membrane-in-a-ring) group the two
/// halves legitimately end up in one face and `SameFace` is an
/// INVENTORY refusal, not a contradiction. Each fact below is
/// re-derived from the arena at the call, so a refusal naming it can
/// only be a tear.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EstablishedFact {
    /// The ring being re-homed is not its own face's outer loop
    /// (tier 1 keeps `outer` out of `rings`, and the ring came out of
    /// `rings`).
    RingIsNotItsFacesOuter,
    /// The ring's face and the survivor lie in one shell (they were
    /// found across one edge, and tier 1's edge-adjacency coherence
    /// keeps an edge's two faces in one shell).
    RingAndSurvivorShareAShell,
    /// The absorbed face is ring-free: the drain above re-homed every
    /// ring it had onto the survivor.
    AbsorbedFaceIsRingFree,
    /// The dying half-edge and its mate are in different loops.
    DyingHalvesAreInDifferentLoops,
    /// The strut edge's two halves start at distinct vertices: the
    /// valence-one answer `strut_tip` gives immediately before the
    /// call is that fact's one spelling, so no second re-check
    /// restates it.
    StrutHalvesHaveDistinctEnds,
    /// The duplicate edge's two halves share a loop, which the
    /// intra-face pass verified before choosing `kemr`.
    DuplicateHalvesShareALoop,
}

impl EstablishedFact {
    /// The sentence the re-check proves, for its assertion message.
    fn what(self) -> &'static str {
        match self {
            Self::RingIsNotItsFacesOuter => {
                "the ring being re-homed is not its own face's outer loop"
            }
            Self::RingAndSurvivorShareAShell => "the ring's face and the survivor lie in one shell",
            Self::AbsorbedFaceIsRingFree => "the drain leaves the absorbed face ring-free",
            Self::DyingHalvesAreInDifferentLoops => {
                "the dying half-edge and its mate are in different loops"
            }
            Self::StrutHalvesHaveDistinctEnds => {
                "the strut edge's two halves start at distinct vertices"
            }
            Self::DuplicateHalvesShareALoop => "the duplicate edge's two halves share a loop",
        }
    }
}

/// Where one [`EulerOpError`] falls **at this door**.
///
/// The arena-fault rule ([`Body::merge_coplanar_faces_declared`]) has
/// two halves with two owners, and this enum is the door's half. *Is
/// the variant torn by its own line?* belongs to the operator layer
/// ([`EulerOpError::reports_tier1_corruption`]) and
/// [`OpPlacement::TheEnumsVerdict`] delegates it. *Does the refusal
/// contradict a fact this door RE-CHECKS at the call?* no other
/// caller of the operator can answer, so it is enumerated here, one
/// arm per [`EstablishedFact`].
///
/// # Which site raises which
///
/// The sites in `merge_group` that can return an [`EulerOpError`],
/// and the whole of what they raise. `R` marks a refusal that is
/// reachable on a tier-1-valid body — the regime's to place — and
/// `C` one this door contradicts. The survivor's kind is not a site:
/// the contract reads it, of every member, before the surgery is
/// handed the group ([`Body::group_contract`]), and the surgery's own
/// re-check of what it was told is a `debug_assert!`, not a lookup
/// that can return.
///
/// | site | can return |
/// | --- | --- |
/// | `edge_halves` | `StaleKey` |
/// | `get_face` (the dying face) | `StaleKey` |
/// | `strut_tip` | `OrbitBroken` |
/// | `merged_outline_ring` (the survivor's surface) | `StaleGeometry` |
/// | `loop_winding`, through `merged_outline_ring` | `StaleKey`, `StaleGeometry`, `UnclaimedHalfEdge`, `LoopCycleBroken` |
/// | `ring_move_minting` | `StaleKey`, `RingIsOuter` (C), `CrossShell` (C), `LoopCycleBroken`; its site mint's `StaleGeometry`, `PcurveMint` (`Corrupt` alone: a moved loop is left as found on a spline chart) and `Certification` (a `tol` that forms no band) |
/// | `kef_minting` | `StaleKey`, `UnclaimedHalfEdge`, `NotSameEdge`, `LoopCycleBroken`, `LoopNotCycle`, `OrbitBroken`, `EmptyAnchorsCollide`, `KillLeavesDangling`, `SameLoop` (C), `SameFace` (**R**), `FaceHasRings` (C); its site mint's, as `ring_move_minting`'s |
/// | `kev` | `StaleKey`, `UnclaimedHalfEdge`, `NotSameEdge`, `LoopNotCycle`, `OrbitBroken`, `LoopCycleBroken`, `EmptyAnchorsCollide`, `KillLeavesDangling`, `SelfLoopEdge` (C); not its fan-merge refusals, which need a fan that neither kill's far vertex has: `strut_tip`'s valence-one tip, and the lone vertex the `mekr_chord` bridge ends at |
/// | `mekr_chord` (a lone vertex's ring) | `StaleKey`, `StaleGeometry`, `LoopNotCycle`, `LoopNotEmpty`, `LoopCycleBroken`, `KillLeavesDangling`, `SameLoop`, `NotSameFace`, `RingIsOuter`, `Certification` |
/// | `kemr` | `StaleKey`, `NotSameEdge`, `UnclaimedHalfEdge`, `LoopNotCycle`, `LoopCycleBroken`, `OrbitBroken`, `EmptyAnchorsCollide`, `KillLeavesDangling`, `NotSameLoop` (C) |
///
/// The `mekr_chord` row and the `kev` after it run only on a planar
/// survivor, which refuses the call whatever it raises, so where those
/// refusals fall changes no outcome; the row's variants take the arms
/// below as they stand.
///
/// The variants the table does not name take the enum's verdict like
/// any other variant this door does not contradict. Outside
/// `merge_group` the door raises `StaleKey` and `StaleGeometry` itself,
/// through [`crate::DanglingRef`] before any group reaches the
/// surgery: the kind census ([`Body::merge_kind`]) on a face whose
/// surface key does not resolve, and the adjacency test
/// ([`Body::planes_declared_equal`]) on a face, vertex or point it
/// cannot resolve. The rest belong to operators this door does not
/// call: the attachment and split gates (`set_edge_curve`,
/// `split_edge`), the make-side sites (`mev`, `mef`), `kvfs`,
/// `kfmrh`'s cross-solid form, `movefac`'s ownership proof, the
/// shell-move door and the null-face door. That is not a third arm: an
/// arm the door cannot reach cannot be pinned, and a classification
/// nothing can distinguish is documentation, which is what this table
/// is. No count of the remainder is stated here; the match below is
/// the census.
///
/// The match producing this is exhaustive on purpose, like the enum's
/// own: a new [`EulerOpError`] variant does not compile until someone
/// places it here as well as on the operator layer's line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpPlacement {
    /// The operator layer's verdict, taken as it stands. The door
    /// keeps no copy of that list and adds nothing to it.
    TheEnumsVerdict,
    /// The refusal contradicts a fact this door re-checks immediately
    /// before the call it came back from, so it reports the arena
    /// whatever the variant means elsewhere.
    Contradicts(EstablishedFact),
}

impl OpPlacement {
    /// Places one operator refusal against the facts `merge_group`
    /// re-checks at its calls (the table in [`OpPlacement`]).
    fn of(error: &EulerOpError) -> Self {
        use EstablishedFact as F;
        use EulerOpError as E;
        match error {
            // ---- This door's own half: contradicts a re-checked
            // fact. Each arm's re-check is the `debug_assert!` that
            // carries the same `EstablishedFact` in the surgery, or,
            // for the strut, `strut_tip`'s answer just before `kev`. ----
            E::RingIsOuter { .. } => Self::Contradicts(F::RingIsNotItsFacesOuter),
            E::CrossShell { .. } => Self::Contradicts(F::RingAndSurvivorShareAShell),
            E::FaceHasRings { .. } => Self::Contradicts(F::AbsorbedFaceIsRingFree),
            E::SameLoop { .. } => Self::Contradicts(F::DyingHalvesAreInDifferentLoops),
            E::SelfLoopEdge { .. } => Self::Contradicts(F::StrutHalvesHaveDistinctEnds),
            E::NotSameLoop { .. } => Self::Contradicts(F::DuplicateHalvesShareALoop),
            // ---- The enum's half. `SameFace` is here and not above:
            // the absorption's own drain re-homes the dying loop onto
            // the survivor, so a nested group meets it on a body that
            // was never torn and the regime places it. ----
            E::SameFace { .. }
            | E::StaleKey { .. }
            | E::StaleGeometry { .. }
            | E::LoopCycleBroken { .. }
            | E::LoopNotCycle { .. }
            | E::NotSameEdge { .. }
            | E::UnclaimedHalfEdge { .. }
            | E::OrbitBroken { .. }
            | E::EmptyAnchorsCollide { .. }
            | E::KillLeavesDangling { .. }
            | E::NotOwned { .. }
            | E::Certification { .. }
            | E::RebasedCarrier { .. }
            | E::NurbsLaneUnsupported { .. }
            | E::RebasedNullEdge { .. }
            | E::MergeRebasesCarriers { .. }
            | E::NotMergedMember { .. }
            | E::DuplicateRedescription { .. }
            | E::DescriptionNotAdjacent { .. }
            | E::RechartStrandsDescriptions { .. }
            | E::RechartUnvouched { .. }
            | E::RechartUndescribed { .. }
            | E::RechartFalsifies { .. }
            | E::RechartOffBoundary { .. }
            | E::RechartBoundaryEscalated { .. }
            | E::FaceMovedTwice { .. }
            | E::FanStartMismatch { .. }
            | E::FanOrbitBroken { .. }
            | E::LoopNotEmpty { .. }
            | E::NotSameFace { .. }
            | E::NullPairForeignLoop { .. }
            | E::SolidNotSingleShell { .. }
            | E::ShellNotSingleFace { .. }
            | E::NullScaffoldCurve { .. }
            | E::SplitParamNotInterior { .. }
            | E::SplitParamEscalated { .. }
            | E::PcurveSplit { .. }
            | E::PcurveMint { .. }
            | E::CrossSolid { .. }
            | E::NoShellsNamed
            | E::ShellRepeated { .. }
            | E::ShellsAcrossSolids { .. }
            | E::SolidWouldEmpty { .. }
            | E::SenseContradictsChart { .. } => Self::TheEnumsVerdict,
        }
    }
}

impl MergeCoplanarError {
    /// Whether this refusal reports a torn ARENA rather than a fact
    /// about the group's mergeability.
    ///
    /// The rule is stated once, in [`GroupRegime`]'s header, and this
    /// applies it: a refusal escapes when the variant is torn by the
    /// operator layer's own line
    /// ([`EulerOpError::reports_tier1_corruption`]), **or** when it
    /// contradicts a fact this door re-checks at the call
    /// ([`OpPlacement`], [`EstablishedFact`]). The second half is
    /// enumerated here because it is this door's own question; the
    /// first is delegated because it is not.
    fn is_arena_fault(&self) -> bool {
        match self {
            Self::Op { error } => match OpPlacement::of(error) {
                OpPlacement::Contradicts(_) => true,
                OpPlacement::TheEnumsVerdict => error.reports_tier1_corruption(),
            },
            _ => false,
        }
    }
}

/// A non-empty declared-pair context: the surface equivalence plus
/// the band its verification decisions run in.
struct DeclaredCtx {
    eq: DeclaredSurfaceEq,
    band: Band,
}

/// The declared surface-key equivalence (M4 PR 5): union-find classes
/// over the declared face pairs' surface keys. Fragments of a face
/// inherit its surface key (`FaceSurface::Inherit`), so surface-level
/// equivalence covers every fragment of a declared pair without
/// key-chasing.
#[derive(Debug, Default)]
struct DeclaredSurfaceEq {
    parent: BTreeMap<SurfaceKey, SurfaceKey>,
}

impl DeclaredSurfaceEq {
    fn find(&self, mut k: SurfaceKey) -> SurfaceKey {
        while let Some(&p) = self.parent.get(&k) {
            if p == k {
                break;
            }
            k = p;
        }
        k
    }

    fn union(&mut self, a: SurfaceKey, b: SurfaceKey) {
        let (ra, rb) = (self.find(a), self.find(b));
        self.parent.entry(ra).or_insert(ra);
        self.parent.entry(rb).or_insert(rb);
        if ra != rb {
            self.parent.insert(rb, ra);
        }
    }

    fn same(&self, a: SurfaceKey, b: SurfaceKey) -> bool {
        if self.parent.is_empty() {
            return false;
        }
        self.find(a) == self.find(b)
    }

    fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

impl<T: Decide> Body<T> {
    /// Merges every maximal run of adjacent same-plane faces (module
    /// docs: structural or declared coincidence only), killing shared
    /// edges (`kef`; intra-face duplicates via `kemr`, whose new ring
    /// takes the plus half's side — a **provisional designation, not
    /// truth**: which loop is "the" ring is a containment question this
    /// op does not ask, so a region-sensitive consumer must re-home the
    /// ring via containment — PR 2+ machinery, `ring_move` the
    /// mechanism. Until then the convention is simply not detected
    /// wrong) and re-homing absorbed faces' rings onto the survivor.
    ///
    /// **Dangling seam edges are pruned (`kev`).** An intra-face
    /// duplicate is not always a ring. A shared edge left with a free
    /// end once the faces are joined — the second leg of a seam that
    /// bends at a corner, a spoke of a junction where several absorbed
    /// faces met, the last edge of a doubled cycle — is deleted with
    /// its free end, at any angle and repeatedly along a chain; only a
    /// doubled edge with no free end separates a ring, which `kemr`
    /// mints. The decision is topological and reads no coordinate; the
    /// deleted vertices are recorded in
    /// [`MergedGroup::killed_vertices`], whose docs say where the
    /// region argument lives.
    ///
    /// **The kept faces' boundaries are re-described.** An absorbed
    /// face's boundary edges end on its survivor still described
    /// against the absorbed face's surface. Before it returns, the door
    /// describes every boundary edge of each kept face again from the
    /// two faces it now lies between — definitely transverse ⇒
    /// `Intersection`, definitely smooth ⇒ the must-carry rule, or the
    /// conventional description where the old one no longer cites those
    /// faces — certified in `Band::linear(tol)`, so no such edge comes
    /// back with tier 3's `DescriptionNotAdjacent`. An edge it cannot
    /// describe refuses ([`MergeCoplanarError::KeptBoundaryUndescribed`],
    /// [`MergeCoplanarError::KeptBoundaryUndecided`]).
    ///
    /// **Atomic and deterministic (D9)**: the op stages on a clone —
    /// on any refusal `self` is untouched; on success the staged body
    /// replaces `self` wholesale. All scans are arena-order; the
    /// surviving face of each group is its first face in face-arena
    /// order that lies in no other member's hole
    /// ([`MergedGroup::kept`]); edges die in edge-arena order. Composite Euler delta per
    /// group: `f −(n−1)`, `e −k`, plus `r +m` for intra-face `kemr`
    /// kills, and `v −1` for each pruning `kev` (which is what
    /// keeps χ conserved when a ring is NOT minted: `kemr` trades an
    /// edge for a ring, `kev` trades an edge for a vertex), plus
    /// `v −1, r −1` for each lone vertex deleted with its ring (`mekr`
    /// then `kev`, the edge they mint and kill leaving no trace). Each step
    /// is an Euler operator, so tier 1 holds throughout and χ is
    /// conserved at every step.
    ///
    /// A body with nothing to merge returns `Ok` with an empty outcome
    /// and is untouched (deterministic no-op).
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces(
        &mut self,
        tol: Tol,
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError>
    where
        T: crate::props::AtRestPolicy,
    {
        self.merge_coplanar_faces_declared(&[], tol)
    }

    /// [`Body::merge_coplanar_faces`] with declared coincident
    /// SURFACE pairs (M4 PR 5, F5): each pair's surfaces are declared
    /// to describe one carrier by recipe intent. A PLANAR pair's
    /// surfaces become equivalent for the adjacency test (fragments
    /// inherit surface keys, so every fragment of a declared face is
    /// covered), verified at each meeting edge through `plane_eq`'s
    /// declared rung (contradiction refuses typed). Same-source
    /// surfaces (N6) glue with zero declarations — the retired bit
    /// rung's replacement.
    ///
    /// A declared pair whose surfaces never meet at an edge licenses
    /// nothing and is a no-op (the equivalence is consulted only
    /// across shared edges); a pair whose keys do not resolve, or
    /// whose surfaces are of two kinds, is a typed refusal. A pair on
    /// one NON-PLANAR kind is a legal declaration this door has no
    /// rung for: it is recorded in [`MergeCoplanarOutcome::skipped`]
    /// as [`MergeCoplanarError::DeclaredCarrierUnsupported`] (the
    /// declaration served the calling op, and the curved run it
    /// leaves is a legal operand) and never refused, even when the
    /// call has nothing else to merge.
    ///
    /// # Two failure regimes, one refusal vocabulary
    ///
    /// A group's **inventory** refusal — *this group cannot be
    /// merged* — either refuses the call or is recorded as a
    /// [`SkippedMerge`] while the remaining groups commit, and which
    /// of the two is a property of the GROUP: curved runs record,
    /// planar runs refuse, whether their adjacency was structural or
    /// declared. A planar group left unglued is two coplanar
    /// neighbours, which the boolean's maximal-faces gate refuses as
    /// an operand and no declaration can cover, so the boolean that
    /// licensed the merge refuses its own step instead of shipping
    /// that body; a curved run's cut form is an operand that gate
    /// accepts. Both raise
    /// the same [`MergeCoplanarError`] and a recorded one is carried
    /// whole in [`SkippedMerge::reason`], so the diagnosis a caller
    /// can make does not depend on which side of the boundary a group
    /// fell.
    ///
    /// **A refusal that reports a torn ARENA is not an inventory
    /// refusal and never becomes a record**: it says nothing about
    /// the group, so it refuses the call under both regimes rather
    /// than returning `Ok` from a door that has just observed a
    /// kernel bug. **This is the rule's one statement in this
    /// module** — everything else that applies it points here.
    ///
    /// Two things put a refusal in that class and this door asks
    /// both:
    ///
    /// 1. The **operator layer's own line**
    ///    ([`EulerOpError::reports_tier1_corruption`]): a dangling
    ///    reference, and the walks and bijections that cannot fail on
    ///    a tier-1-valid body. A property of the variant, asked and
    ///    never copied here.
    /// 2. A refusal that **contradicts a fact the surgery re-checks
    ///    at the call** it came back from. `kef` reporting that the
    ///    dying face still has rings, on a face whose rings the
    ///    surgery re-homed and re-checked a statement earlier,
    ///    reports the arena and not the group's mergeability,
    ///    whatever that variant means at another door.
    ///
    /// **Re-checked, not merely established.** A fact the door read
    /// before its own next mutation says nothing about the call:
    /// `kef`'s [`EulerOpError::SameFace`] is the case that settles
    /// it. The absorption's ring drain re-homes the dying loop onto
    /// the survivor, so on a nested group — a membrane face covering
    /// a ring of its neighbour — the two halves legitimately end up
    /// in one face, and `SameFace` there is an INVENTORY refusal on a
    /// body that was never torn. Only the facts the surgery
    /// re-derives from the arena immediately before the call are the
    /// door's to contradict.
    ///
    /// [`MergeCoplanarError::GroupKindSplit`] also refuses under both
    /// regimes, but for a different reason and with a cost worth
    /// stating: it is raised while the regime is being COMPUTED, so
    /// there is no regime yet to record it under. A group that
    /// straddles two surface kinds therefore refuses even where a
    /// curved group of the same shape would have been recorded. That
    /// is the honest outcome of having no contract to give such a
    /// group, not a decision to refuse licensed work.
    ///
    /// # The placeholder is a third kind, not a curved one
    ///
    /// A face whose surface is the `mvfs` seed's placeholder
    /// ([`MergeKind::Placeholder`]) describes no locus, so it is
    /// neither a planar run nor a curved one: the door names every
    /// such face in [`MergeCoplanarOutcome::placeholders`], a run of
    /// them on one key is SET ASIDE — no surgery, no group, no skip —
    /// and a group that joins one to a described face through a
    /// shared surface source refuses [`MergeCoplanarError::GroupKindSplit`]
    /// like any other straddle. That is the only rung that can join
    /// them: the structural rung reads one key as one surface, so a
    /// placeholder shares a key only with placeholders, and a declared
    /// pair must name planes. A refusal rather than a quiet set-aside
    /// because the source stamp is the CALLER's claim that the two
    /// are one recipe surface, and a placeholder cannot be one with a
    /// described face; setting it aside would have the door pick
    /// which half of that contradiction to believe.
    ///
    /// The census is taken before the adjacency scan, so the record is
    /// complete whether or not anything merges. A POISONED net is not
    /// a placeholder and is not set aside: it refuses the call
    /// ([`MergeCoplanarError::PoisonedSurfaceDescription`]) before any
    /// group forms, for the reason [`MergeKind`] gives.
    ///
    /// The recording side is bounded the same way the refusing side
    /// is: each such group is staged on its own clone behind its own
    /// tier-2 gate, so a recorded skip leaves the run exactly as it
    /// was — never a partial commit, and the unglued curved adjacency
    /// persists in the cut-carrying form the operands already carried.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces_declared(
        &mut self,
        declared: &[(SurfaceKey, SurfaceKey)],
        tol: Tol,
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError>
    where
        T: crate::props::AtRestPolicy,
    {
        // ---- Gate: tier-valid before. ----
        if let Err(errors) = validate_closed(self) {
            return Err(MergeCoplanarError::InputNotClosed { errors });
        }
        // ---- Declared pairs: validate, then class each by carrier kind. ----
        //
        // A key that does not resolve, or a pair of two kinds, is a
        // torn argument and refuses. A planar pair joins the surface
        // equivalence. A pair on one non-planar kind is a LEGAL
        // declaration this door has no rung for: it is declined here
        // and recorded below, never refused — the declaration served
        // the calling op, and an inventory limit of this door is not
        // the caller's error.
        let kind_of = |k: SurfaceKey| -> Result<SurfaceKind, MergeCoplanarError> {
            self.get_surface(k).map(geom::Surface::kind).ok_or(
                MergeCoplanarError::InvalidDeclaration {
                    surface: k,
                    what: "declared surface key does not resolve",
                },
            )
        };
        let mut eq = DeclaredSurfaceEq::default();
        let mut declined: Vec<((SurfaceKey, SurfaceKey), SurfaceKind)> = Vec::new();
        let mut declined_seen: std::collections::BTreeSet<(SurfaceKey, SurfaceKey)> =
            std::collections::BTreeSet::new();
        for &(k1, k2) in declared {
            let (kind1, kind2) = (kind_of(k1)?, kind_of(k2)?);
            if kind1 != kind2 {
                // Named: the second key, whose kind disagrees with the first's.
                return Err(MergeCoplanarError::InvalidDeclaration {
                    surface: k2,
                    what: "declared surfaces are not one kind",
                });
            }
            if kind1 == SurfaceKind::Plane {
                eq.union(k1, k2);
            } else if declined_seen.insert((k1, k2)) {
                // One record per declared pair: a caller that lowers
                // several face pairs to one surface pair hands the
                // door copies, and a copy names nothing new.
                declined.push(((k1, k2), kind1));
            }
        }
        // The declined pairs' records name every live face on either
        // declared surface, read off the body the caller RECEIVES —
        // `self` when nothing merges, the staged result otherwise —
        // so a recorded face is never a dead key (a same-key run on
        // one of the pair's surfaces can COMMIT beside the declined
        // pair, absorbing a face the pre-surgery body still had;
        // `curved_mergedoor::record_beside_a_committing_curved_run_names_only_live_faces`
        // pins it). A pair with NO live face on either surface — a
        // surface kept alive by an edge description after every face
        // left it — gets no record: the declaration served nothing at
        // this door, and a record naming nothing would be the silent
        // shape (`curved_mergedoor::pair_with_no_live_faces_mints_no_record`).
        let declined_records = |body: &Self| -> Vec<SkippedMerge> {
            declined
                .iter()
                .filter_map(|&(pair, kind)| {
                    let faces: Vec<FaceKey> = body
                        .faces()
                        .filter(|(_, face)| face.surface == pair.0 || face.surface == pair.1)
                        .map(|(key, _)| key)
                        .collect();
                    (!faces.is_empty()).then_some(SkippedMerge {
                        faces,
                        reason: MergeCoplanarError::DeclaredCarrierUnsupported { pair, kind },
                    })
                })
                .collect()
        };
        let declared_ctx = if eq.is_empty() {
            None
        } else {
            Some(DeclaredCtx {
                eq,
                band: Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?,
            })
        };
        // ---- The kind census (read-only, face-arena order). ----
        //
        // Every face's kind, asked once: the contract reads it for
        // each group's members, and the placeholder record is its
        // by-product. Taken before the adjacency scan so the record is
        // complete whether or not anything merges, and so a poisoned
        // net refuses before any group forms.
        let KindCensus {
            kinds,
            described,
            placeholders,
        } = self.kind_census()?;
        let mut outcome = MergeCoplanarOutcome {
            placeholders,
            ..MergeCoplanarOutcome::default()
        };
        // ---- Mergeable adjacency (read-only, edge-arena order). ----
        let mut neighbors: BTreeMap<FaceKey, Vec<FaceKey>> = BTreeMap::new();
        let mut any = false;
        for (_, edge) in self.edges() {
            let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
            let (fp, fm) = (hp.face, hm.face);
            if fp != fm && self.planes_declared_equal(hp, hm, &described, declared_ctx.as_ref())? {
                neighbors.entry(fp).or_default().push(fm);
                neighbors.entry(fm).or_default().push(fp);
                any = true;
            }
        }
        if !any {
            // Nothing to merge: the declined declared pairs still
            // ship (they are a statement about the caller's argument,
            // not about any merge), on the outcome whose placeholder
            // census the initializer already took.
            outcome.skipped = declined_records(self);
            return Ok(outcome);
        }
        // ---- Group labeling (face-arena order seeds, DFS worklist). ----
        //
        // A group is (seed, rest) rather than one list: the seed is
        // the survivor and every later step needs it, so carrying it
        // separately is what stops an empty group from being
        // spellable at all.
        let mut label: SecondaryMap<FaceKey, usize> = SecondaryMap::new();
        let mut groups: Vec<(FaceKey, Vec<FaceKey>)> = Vec::new();
        for (face_key, _) in self.faces() {
            if !neighbors.contains_key(&face_key) || label.contains_key(face_key) {
                continue;
            }
            let id = groups.len();
            let mut rest = Vec::new();
            label.insert(face_key, id);
            let mut pending = vec![face_key];
            while let Some(next) = pending.pop() {
                let Some(adjacent) = neighbors.get(&next) else {
                    unreachable!(
                        "merge_coplanar_faces: face {next:?} was queued from the adjacency map, \
                         and every face that map lists is one of its keys (each mergeable edge \
                         enters both of its faces)"
                    )
                };
                for &n in adjacent {
                    if !label.contains_key(n) {
                        label.insert(n, id);
                        rest.push(n);
                        pending.push(n);
                    }
                }
            }
            groups.push((face_key, rest));
        }
        // ---- Staged surgery on a clone. ----
        //
        // One refusal vocabulary; [`GroupRegime`] places the INVENTORY
        // refusals and an arena fault escapes it under both regimes.
        // The recording arm additionally clones a trial and runs its
        // own tier-2 gate, which is the price of letting the rest of
        // the run commit. A placeholder run has no regime and no
        // surgery: it is set aside here, its faces already named.
        let mut work = self.clone();
        for (seed, members) in groups {
            let (rep, rest) = work.outermost_survivor(seed, members)?;
            let (regime, kind) = match Self::group_contract(rep, &rest, &kinds)? {
                GroupContract::Runs { regime, kind } => (regime, kind),
                GroupContract::SetAside => continue,
            };
            match regime {
                GroupRegime::RefusesTheCall => {
                    // One surgery scope per group: the ring surgery
                    // inside `merge_group` is this door's, and the
                    // tier-2 gate below is what certifies its result.
                    let mut surgery = work.begin_surgery();
                    let group = surgery.merge_group(rep, &rest, kind, tol)?;
                    surgery.sweep_and_close();
                    outcome.groups.push(group);
                }
                GroupRegime::RecordsASkip => {
                    let mut trial = work.clone();
                    // The sub-stage's own tier-2 gate: the group is
                    // adopted only if its trial validates, so a
                    // recorded skip leaves `work` exactly as it was.
                    let staged = {
                        // The sweep is the SUCCESS path's. A refusal
                        // here is the trial's own — the group is
                        // recorded as skipped and the trial thrown
                        // away — and the state a refusal leaves
                        // behind was never this door's to certify.
                        let mut surgery = trial.begin_surgery();
                        match surgery.merge_group(rep, &rest, kind, tol) {
                            Ok(group) => {
                                surgery.sweep_and_close();
                                Ok(group)
                            }
                            Err(error) => Err(error),
                        }
                    }
                    .and_then(|group| match validate_closed(&trial) {
                        Ok(()) => Ok(group),
                        Err(errors) => Err(MergeCoplanarError::GroupNotClosed { errors }),
                    });
                    match staged {
                        Ok(group) => {
                            work = trial;
                            outcome.groups.push(group);
                        }
                        Err(reason) => {
                            // An arena fault is not this group's
                            // failure to be recorded; it reports the
                            // body, so it refuses here too.
                            if !GroupRegime::RecordsASkip.records(&reason) {
                                return Err(reason);
                            }
                            outcome.skipped.push(SkippedMerge {
                                faces: core::iter::once(rep).chain(rest).collect(),
                                reason,
                            });
                        }
                    }
                }
            }
        }
        // ---- Gate: tier-valid after. ----
        if let Err(errors) = validate_closed(&work) {
            return Err(MergeCoplanarError::ResultNotClosed { errors });
        }
        // The records the caller receives. Neither the re-description
        // nor the pcurve re-mint below moves a face or its surface, so
        // they are read once, here.
        let declined = declined_records(&work);
        // ---- The kept faces' boundaries, re-described. ----
        //
        // An absorbed face's boundary edges now lie on its survivor,
        // and their descriptions still name the absorbed face's
        // surface: each is described again from the two faces it lies
        // between. After the gate, so a torn result reports its tier-2
        // errors, never a walk the describer could not make.
        if !outcome.groups.is_empty() {
            let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
            let Some(boundary) =
                crate::boolean::boundary_edges(&work, outcome.groups.iter().map(|g| g.kept))
            else {
                unreachable!(
                    "merge_coplanar_faces: a kept face's loop does not walk, on a result that \
                     passed tier 1 and tier 2 a statement ago"
                )
            };
            // The describer reads the records for the scaffold a
            // recorded skip leaves between its faces.
            let records: Vec<SkippedMerge> =
                declined.iter().chain(&outcome.skipped).cloned().collect();
            let mut surgery = work.begin_surgery();
            crate::boolean::describe_edges(
                &mut surgery,
                boundary.iter().map(|&(_, edge)| edge),
                &records,
                band,
                tol,
            )
            .map_err(|refusal| MergeCoplanarError::of_kept_boundary(&boundary, refusal))?;
            surgery.sweep_and_close();
            // The describer's postcondition: every edge it described is
            // coherent with its two faces (tier 3's naming check, exact).
            #[cfg(debug_assertions)]
            for &(face, edge) in &boundary {
                debug_assert!(
                    matches!(work.stored_description_adjacent(edge), Ok(true)),
                    "merge_coplanar_faces: kept face {face:?}'s boundary edge {edge:?} was \
                     described and is not coherent with its sides {:?}",
                    crate::readback::edge_sides(&work, edge)
                );
            }
        }
        // A body that carried stored pcurve caches RE-MINTS them on
        // the staged result before commit (the `topo::pcurves` module
        // docs' rule for ops that mutate minted bodies; a body at rest
        // carries them on every face whose chart mints): the merge
        // rebuilds face loops, and two absorbed fragments' walks were
        // branch-anchored independently — the merged loop's one-branch
        // walk must be derived fresh, never stitched from the
        // fragments' rows. Still on the staged clone, so a mint
        // refusal keeps the untouched-on-error contract. A row the mint
        // has no route to (a `Fitted` row on a class the closed-form
        // lane does not cover) is carried across, re-certified, not
        // dropped (`pcurves::carry_rows`).
        if !self.pcurves.is_empty() {
            crate::pcurves::mint_pcurves(&mut work, tol)
                .map_err(|source| MergeCoplanarError::Pcurve { source })?;
        }
        let mut skipped = declined;
        skipped.append(&mut outcome.skipped);
        outcome.skipped = skipped;
        self.adopt(work);
        Ok(outcome)
    }

    /// The other half of `edge`, resolved — the re-checks' shared
    /// lookup. `None` when a key does not resolve or `edge` does not
    /// claim `he` ([`crate::entity::Edge::claim`]), which the re-checks
    /// read as "nothing to prove here": that is the operator's own
    /// refusal to make (`StaleKey`, `UnclaimedHalfEdge`), and it is
    /// torn by the enum's line either way.
    fn edge_mate(
        &self,
        he: crate::entity::HalfEdgeKey,
        edge: EdgeKey,
    ) -> Option<&crate::entity::HalfEdge> {
        self.get_half_edge(self.get_edge(edge)?.claim(he)?.mate)
    }

    /// The [`MergeKind`] of one face's surface, announcing both
    /// lookups.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError::Op`], through the crate's dangling-
    /// reference vocabulary;
    /// [`MergeCoplanarError::PoisonedSurfaceDescription`] for a net
    /// that has no kind here.
    fn merge_kind(&self, face: FaceKey) -> Result<MergeKind, MergeCoplanarError> {
        let record = self
            .get_face(face)
            .ok_or(DanglingRef::Entity(EntityId::Face(face)))?;
        Ok(self.described_kind(face, record)?.1)
    }

    /// Face `face`'s surface (`record` is its resolved record) and
    /// the surface's [`MergeKind`]: [`Body::merge_kind`]'s errors past
    /// the face lookup.
    fn described_kind(
        &self,
        face: FaceKey,
        record: &crate::entity::Face,
    ) -> Result<(&Surface<T>, MergeKind), MergeCoplanarError> {
        let described = self
            .get_surface(record.surface)
            .ok_or(DanglingRef::Geometry(GeomRef::Surface(record.surface)))?;
        let kind = MergeKind::of(described)
            .map_err(|PoisonedNet| MergeCoplanarError::PoisonedSurfaceDescription { face })?;
        Ok((described, kind))
    }

    /// Every live face's [`MergeKind`], in one pass — the one place
    /// the door asks the kind question of the arena — with the
    /// surface each face resolved to and the placeholder faces that
    /// pass met, in face-arena order.
    ///
    /// # Errors
    ///
    /// [`Body::merge_kind`]'s, for the first face (arena order) that
    /// raises one.
    fn kind_census(&self) -> Result<KindCensus<'_, T>, MergeCoplanarError> {
        let mut census = KindCensus {
            kinds: SecondaryMap::new(),
            described: SecondaryMap::new(),
            placeholders: Vec::new(),
        };
        for (face_key, record) in self.faces() {
            let (described, kind) = self.described_kind(face_key, record)?;
            if kind == MergeKind::Placeholder {
                census.placeholders.push(face_key);
            }
            census.kinds.insert(face_key, kind);
            census.described.insert(face_key, (record, described));
        }
        Ok(census)
    }

    /// **Does `toward` dangle alone at its start vertex** — is that
    /// vertex the free end of a shared edge the glue left dangling,
    /// the condition that licenses `kev` over `kemr` on a planar
    /// survivor's duplicate?
    ///
    /// A BROKEN orbit is ANNOUNCED, not read as "no tip". `kev` and
    /// `kemr` are different operators with different Euler deltas, so
    /// a torn arena answering this question silently chooses which
    /// surgery runs and which delta the group reports.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::OrbitBroken`], naming the half-edge whose
    /// start vertex's orbit failed to close.
    fn strut_tip(&self, toward: crate::entity::HalfEdgeKey) -> Result<bool, EulerOpError> {
        let orbit = self
            .vertex_orbit(toward)
            .ok_or(EulerOpError::OrbitBroken { he: toward })?;
        Ok(orbit.len() == 1)
    }

    /// The group's survivor: its first member in face-arena order that
    /// lies in no other member's HOLE, with the rest of the members.
    ///
    /// A member lies in another's hole when that member's ring borders
    /// it — a coplanar face plugging a hole of a group face. The
    /// absorption keeps the survivor and kills every other member with
    /// `kef` across a shared edge, after re-homing the dying face's
    /// rings onto the survivor; a survivor inside the dying face's ring
    /// would receive that very ring, leave both halves of the shared
    /// edge on itself, and `kef` could not kill anything (`SameFace`).
    /// The outermost member has no such ring around it, so every other
    /// member dies into it. `seed`, the arena-first member, is the
    /// survivor whenever it is not nested, which is every group without
    /// a plug; a group whose every member is nested (not a planar region
    /// a merge can reach) keeps `seed`, and the absorption refuses as
    /// before.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError::Op`] carrying an unresolved reference, or
    /// a ring half-edge its own edge does not claim
    /// ([`EulerOpError::UnclaimedHalfEdge`]): the scan runs on the
    /// staged body after earlier groups' surgery, which the entry
    /// gate's proof does not reach.
    fn outermost_survivor(
        &self,
        seed: FaceKey,
        members: Vec<FaceKey>,
    ) -> Result<(FaceKey, Vec<FaceKey>), MergeCoplanarError> {
        let in_group = |f: FaceKey| f == seed || members.contains(&f);
        let mut nested: std::collections::BTreeSet<FaceKey> = std::collections::BTreeSet::new();
        for f in core::iter::once(seed).chain(members.iter().copied()) {
            let face = self
                .get_face(f)
                .ok_or(DanglingRef::Entity(EntityId::Face(f)))?;
            for &ring in &face.rings {
                let first = match self
                    .get_loop(ring)
                    .ok_or(DanglingRef::Entity(EntityId::Loop(ring)))?
                    .boundary
                {
                    crate::entity::LoopBoundary::Cycle { first } => first,
                    // A lone-vertex ring borders no face.
                    crate::entity::LoopBoundary::Empty { vertex: _lone } => continue,
                };
                let cycle = self
                    .loop_cycle(first)
                    .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring })?;
                for he in cycle {
                    let edge = self
                        .get_half_edge(he)
                        .ok_or(DanglingRef::Entity(EntityId::HalfEdge(he)))?
                        .edge;
                    let e = self
                        .get_edge(edge)
                        .ok_or(DanglingRef::Entity(EntityId::Edge(edge)))?;
                    let mate = e
                        .claim(he)
                        .ok_or(EulerOpError::UnclaimedHalfEdge { he, edge })?
                        .mate;
                    let (_, facts) = self.edge_halves(he, mate)?;
                    if facts.face != f && in_group(facts.face) {
                        nested.insert(facts.face);
                    }
                }
            }
        }
        if !nested.contains(&seed) {
            return Ok((seed, members));
        }
        let survivor = self
            .faces()
            .map(|(k, _)| k)
            .find(|&k| in_group(k) && !nested.contains(&k));
        Ok(match survivor {
            Some(kept) => {
                let rest = core::iter::once(seed)
                    .chain(members)
                    .filter(|&f| f != kept)
                    .collect();
                (kept, rest)
            }
            None => (seed, members),
        })
    }

    /// One group's [`GroupContract`]: whether it runs, and under which
    /// failure regime its INVENTORY refusals fall.
    ///
    /// A curved group records a skip: its refusal is a statement
    /// about the merge's inventory rather than about the body, and its
    /// unglued adjacency is a legal output the operands already
    /// carried — the boolean's maximal-faces gate accepts a curved
    /// run's cut form. A planar group refuses the call, however its
    /// adjacency was licensed: two unglued coplanar neighbours are a
    /// body no boolean accepts as an operand, and a declared pair
    /// cannot make them one, so a planar group the merge cannot glue
    /// is the caller's refusal, never a record. A placeholder run is
    /// set aside ([`GroupContract::SetAside`]).
    ///
    /// **The kind question is asked of EVERY member.** The hard rungs
    /// glue on surface-key or surface-SOURCE identity, and neither
    /// tests the surface's kind, so a group can straddle two kinds;
    /// answering off one member would let arena order decide which
    /// contract the group is handed. A straddling group refuses
    /// ([`MergeCoplanarError::GroupKindSplit`]), a placeholder among
    /// described faces included.
    ///
    /// The kinds are read from the census ([`Body::kind_census`]), not
    /// from the arena: the census asked every face once, and a member
    /// it does not hold is a face the arena did not have when the
    /// door started.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError::GroupKindSplit`] where the members
    /// disagree, or [`MergeCoplanarError::Op`] carrying an unresolved
    /// reference. The kind lookups are ANNOUNCED rather than read as
    /// "not curved": they decide which contract the group is handed,
    /// and a failed lookup silently spelled "planar" would move a
    /// group between regimes on a torn arena.
    fn group_contract(
        rep: FaceKey,
        rest: &[FaceKey],
        kinds: &SecondaryMap<FaceKey, MergeKind>,
    ) -> Result<GroupContract, MergeCoplanarError> {
        let kind_of = |f: FaceKey| {
            kinds
                .get(f)
                .copied()
                .ok_or(DanglingRef::Entity(EntityId::Face(f)))
        };
        let rep_kind = kind_of(rep)?;
        for &f in rest {
            let kind = kind_of(f)?;
            if kind != rep_kind {
                let ((face, kind), (other, other_kind)) = if rep_kind < kind {
                    ((rep, rep_kind), (f, kind))
                } else {
                    ((f, kind), (rep, rep_kind))
                };
                return Err(MergeCoplanarError::GroupKindSplit {
                    face,
                    kind,
                    other,
                    other_kind,
                });
            }
        }
        Ok(match rep_kind {
            MergeKind::Placeholder => GroupContract::SetAside,
            MergeKind::Plane => GroupContract::Runs {
                regime: GroupRegime::RefusesTheCall,
                kind: MergeKind::Plane,
            },
            MergeKind::Curved => GroupContract::Runs {
                regime: GroupRegime::RecordsASkip,
                kind: MergeKind::Curved,
            },
        })
    }

    /// One resolved half of an edge — every fact the merge's scans
    /// read through a half-edge, taken in the one walk that proves
    /// the keys, so no later step looks a proven key up again.
    fn half_edge_facts(
        &self,
        he: crate::entity::HalfEdgeKey,
    ) -> Result<HalfEdgeFacts, EulerOpError> {
        let hd = self.get_half_edge(he).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he),
        })?;
        let parent = hd.parent_loop;
        let face = self
            .get_loop(parent)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Loop(parent),
            })?
            .face;
        Ok(HalfEdgeFacts {
            r#loop: parent,
            face,
            start: hd.start,
        })
    }

    /// An edge's two halves, resolved.
    ///
    /// Every link this walks is one tier 1 requires to resolve, so a
    /// refusal names a torn arena and never an ordinary shape. It is
    /// returned rather than folded into "this edge is not interesting"
    /// because the two are indistinguishable to the scans that call
    /// this, and treating a torn link as an uninteresting edge drops a
    /// mergeable adjacency or a shared seam without a word.
    ///
    /// It carries the parent loop and start vertex beside the face
    /// because the scans need those too: returning them from the walk
    /// that proved the keys is what leaves the later steps with
    /// nothing to look up and therefore nothing to discard.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`], naming the link that did not
    /// resolve.
    fn edge_halves(
        &self,
        he_plus: crate::entity::HalfEdgeKey,
        he_minus: crate::entity::HalfEdgeKey,
    ) -> Result<(HalfEdgeFacts, HalfEdgeFacts), EulerOpError> {
        Ok((
            self.half_edge_facts(he_plus)?,
            self.half_edge_facts(he_minus)?,
        ))
    }

    /// The F6 ladder's merge test: the recipe declared the two faces'
    /// surfaces one ([`crate::source::surface_declaration`] — same key
    /// or same [`crate::GeomSource`], zero numerics) and the faces share
    /// a `sense`, or the pair's planes are declared-equivalent by this
    /// call's face pairs (verified through `plane_eq`'s declared rung
    /// at the meeting edge; contradiction refuses).
    ///
    /// The M3-era rung — bit-identical nine-scalar descriptions — is
    /// RETIRED from production: equal bits without shared source stay
    /// unglued (the ladder's ratified rung (b)). The bit comparison
    /// survives as the debug assertion that same-source records agree
    /// with the bits. *No banded comparison certifies coincidence
    /// here by design* — the declared-pair verification only checks
    /// the declaration is not a lie; the INTENT does the gluing.
    ///
    /// The declared hard rungs merge any kind; the declared-pair rung
    /// is planar.
    ///
    /// **Shared sense is a precondition of every rung** (S10). Two
    /// faces on one surface whose `sense` bits differ have OPPOSITE
    /// outward normals: they are the two sides of a slit, not one
    /// region cut in two, and gluing them would mint a face that is
    /// its own reverse. The hard rungs therefore stop firing on such a
    /// pair — they answer "same SURFACE", which is no longer the same
    /// question as "same FACE geometry". They fall through to the
    /// declared rung, where the verified `oriented_plane_eq` verdict
    /// on the two OUTWARD normals is `SameOpposite` and the existing
    /// [`MergeCoplanarError::DeclaredOppositeOrientation`] refusal
    /// fires — a declaration that such a pair is mergeable is exactly
    /// the lie that variant was minted to refuse. An UNDECLARED
    /// opposite-sense pair is not refused, it is simply not a merge
    /// candidate: a slit is legal geometry, and this op has no
    /// standing to reject a body for containing one.
    ///
    /// `plus` and `minus` are the shared edge's two halves as the
    /// adjacency scan resolved them: their faces are the pair, and
    /// their start vertices are the edge's two ends.
    ///
    /// `described` is the kind census's record of every live face and
    /// its surface. "Not a plane" is an answer (`false`: the
    /// declared-pair rung is planar); a face or end that does not
    /// resolve is not.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError::Op`] naming a face the census did not
    /// meet live, or an end vertex or point that does not resolve.
    /// Otherwise the declared rung's refusals
    /// ([`declared_pair_verdict`]).
    fn planes_declared_equal(
        &self,
        plus: HalfEdgeFacts,
        minus: HalfEdgeFacts,
        described: &Described<'_, T>,
        declared: Option<&DeclaredCtx>,
    ) -> Result<bool, MergeCoplanarError> {
        let (f1, f2) = (plus.face, minus.face);
        let resolved = |f: FaceKey| {
            described
                .get(f)
                .copied()
                .ok_or(DanglingRef::Entity(EntityId::Face(f)))
        };
        let ((face1, s1), (face2, s2)) = (resolved(f1)?, resolved(f2)?);
        let (k1, k2) = (face1.surface, face2.surface);
        // The shared-sense precondition (fn docs): a differing bit
        // makes the two outward normals opposite, so neither hard rung
        // — both of which certify the SURFACE, not the face — may
        // conclude the faces are one region. Falling through leaves
        // the declared rung to refuse loudly if the pair was declared.
        let same_sense = face1.sense == face2.sense;
        // The hard rungs are the declared-identity predicate
        // (`crate::source`'s module docs): kind-agnostic, never numeric.
        let declaration = crate::source::surface_declaration(self, k1, self, k2);
        if same_sense && declaration.one_surface() {
            // Asserted where the grouping's kind split will not refuse
            // the pair typed, and only where the scalar HAS a bit
            // channel: a scalar with no channel (`Dual`, `Sym`) offers
            // no evidence, and `None` there is not disagreement.
            #[cfg(debug_assertions)]
            if declaration == crate::source::SurfaceDeclaration::SameSource
                && matches!(
                    (MergeKind::of(s1), MergeKind::of(s2)),
                    (Ok(a), Ok(b)) if a == b
                )
                && let Some(agree) = crate::source::surface_bits_witness(s1, s2)
            {
                debug_assert!(
                    agree,
                    "same-source theorem violated: same-source surface descriptions disagree \
                     bitwise (kernel bug: a source survived a geometric rewrite)"
                );
            }
            return Ok(true);
        }
        // The declared-PAIR rung stays planar (its verification is
        // `oriented_plane_eq`; the curved-pair verification predicate
        // is the contact census's — CONTACT-DESIGN C2/C4 — not minted
        // here).
        let (
            Surface::Plane {
                origin: o1,
                normal: n1,
                ..
            },
            Surface::Plane {
                origin: o2,
                normal: n2,
                ..
            },
        ) = (s1.clone(), s2.clone())
        else {
            return Ok(false);
        };
        // Declared face pairs (this call's recipe intent), verified.
        if let Some(ctx) = declared
            && ctx.eq.same(k1, k2)
        {
            let band = ctx.band;
            let id = PlaneIdentity {
                s1: None,
                s2: None,
                declared: true,
            };
            // Outward normals, not chart normals (S10): `PlaneDesc`'s
            // contract, and the reason the SameOpposite arm below can
            // stand as the shared-sense refusal — an opposite-sense
            // pair on one plane lands there by construction.
            let p1 = PlaneDesc {
                origin: o1,
                normal: plane_outward_normal(face1, n1).vec(),
            };
            let p2 = PlaneDesc {
                origin: o2,
                normal: plane_outward_normal(face2, n2).vec(),
            };
            // The pair is read over a ball enclosing both faces, and
            // their vertices, the points known to be consumed: the
            // glue holds at every point of both faces, and a face
            // standing definitely off the other's plane contradicts the
            // declaration. A boundary key that does not resolve is
            // announced by name; a face whose extent does not read
            // otherwise has no reach to settle, which the reach
            // decision names.
            let (on1, on2) = (self.boundary_points(f1)?, self.boundary_points(f2)?);
            let reach =
                crate::boolean::rest::pair_extent(self, f1, self, f2, band).map_err(|_| {
                    MergeCoplanarError::Escalated {
                        decision: MergeDecision::DeclaredReach,
                        diag: Indeterminate {
                            margin: geom_core::MarginDiag::INVALID,
                            band,
                            predicate: Some("merge_declared_extent"),
                            terminal_sliver: false,
                        },
                    }
                })?;
            let extent = crate::boolean::ConsumedExtent {
                reach: reach.reach,
                on: [&on1, &on2],
            };
            return declared_pair_verdict(oriented_plane_eq(&p1, &p2, id, &extent, band), f1, f2);
        }
        Ok(false)
    }

    /// The face's boundary vertex positions, outer loop then rings
    /// (an empty loop contributes its lone vertex), each key that does
    /// not resolve named: the points the declared rung knows lie on
    /// the face.
    fn boundary_points(&self, face: FaceKey) -> Result<Vec<geom_core::Point3<T>>, DanglingRef> {
        let f = self
            .get_face(face)
            .ok_or(DanglingRef::Entity(EntityId::Face(face)))?;
        let point = |v: VertexKey| {
            let key = self
                .get_vertex(v)
                .ok_or(DanglingRef::Entity(EntityId::Vertex(v)))?
                .point;
            self.get_point(key)
                .copied()
                .ok_or(DanglingRef::Geometry(GeomRef::Point(key)))
        };
        let mut out = Vec::new();
        for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let l = self
                .get_loop(lk)
                .ok_or(DanglingRef::Entity(EntityId::Loop(lk)))?;
            match l.boundary {
                crate::entity::LoopBoundary::Empty { vertex } => out.push(point(vertex)?),
                crate::entity::LoopBoundary::Cycle { first } => {
                    let cycle = self
                        .loop_cycle(first)
                        .ok_or(DanglingRef::Entity(EntityId::HalfEdge(first)))?;
                    for he in cycle {
                        let start = self
                            .get_half_edge(he)
                            .ok_or(DanglingRef::Entity(EntityId::HalfEdge(he)))?
                            .start;
                        out.push(point(start)?);
                    }
                }
            }
        }
        Ok(out)
    }

    /// Merges one group into `rep`, its survivor (see the
    /// public op's docs for order and refusals). Runs on the staged
    /// clone.
    ///
    /// The survivor is a parameter rather than `members[0]` so that a
    /// group with no survivor cannot be spelled: every caller has the
    /// seed in hand, and a bare index here would be a panic path for
    /// a state the labeling never produces.
    ///
    /// `kind` is the group's kind as the contract decided it of every
    /// member ([`GroupContract::Runs`]): [`MergeKind::Plane`] or
    /// [`MergeKind::Curved`], never the placeholder, which is set
    /// aside before the surgery. It says what a same-face duplicate on
    /// the survivor means after the absorption. The surgery is told
    /// rather than reading the survivor's surface again, so the one
    /// kind question has one site — and it re-checks what it was told
    /// against the survivor's surface before it mutates anything,
    /// because a curved run told "planar" MERGES where the truthful
    /// call refuses `PeriodClosure`, and nothing downstream of the
    /// surgery would notice.
    fn merge_group(
        &mut self,
        rep: FaceKey,
        rest: &[FaceKey],
        kind: MergeKind,
        tol: Tol,
    ) -> Result<MergedGroup, MergeCoplanarError>
    where
        T: crate::props::AtRestPolicy,
    {
        debug_assert!(
            kind != MergeKind::Placeholder && self.merge_kind(rep).ok().is_none_or(|k| k == kind),
            "merge_group: the survivor's kind is the contract's ({kind:?})"
        );
        let mut group = MergedGroup {
            kept: rep,
            absorbed: Vec::new(),
            killed_edges: Vec::new(),
            rings_made: Vec::new(),
            killed_vertices: Vec::new(),
        };
        let in_group = |f: FaceKey| f == rep || rest.contains(&f);
        // Absorption: repeatedly kill the first (edge-arena order)
        // edge shared between rep and another group member.
        loop {
            let mut found = None;
            for (edge_key, edge) in self.edges() {
                let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
                if hp.face == rep && hm.face != rep && in_group(hm.face) {
                    found = Some((edge_key, edge.he_minus, hm.face));
                    break;
                }
                if hm.face == rep && hp.face != rep && in_group(hp.face) {
                    found = Some((edge_key, edge.he_plus, hp.face));
                    break;
                }
            }
            let Some((edge_key, dying_he, other)) = found else {
                break;
            };
            // Re-home the dying face's rings onto the survivor, then
            // kill the shared edge and the face together (kef).
            //
            // The lookup is ANNOUNCED rather than defaulted to an
            // empty ring list, because "this face has no rings" and
            // "this face is gone" are different answers and the
            // default gave them one spelling. It is a typed refusal
            // and not an `unreachable!`: `other` arrives from a
            // loop's back-pointer and no check in this call proves it
            // live. `kef` below re-derives the same key and refuses
            // on it too, so the ANSWER here was never reachable — the
            // announcement is what makes the two agree at the site
            // that reads the rings.
            let dying = self
                .get_face(other)
                .ok_or(DanglingRef::Entity(EntityId::Face(other)))?;
            for ring in dying.rings.clone() {
                // The two facts `ring_move` could contradict, RE-READ
                // from the arena here rather than carried down from
                // the scan: the drain's own earlier iterations have
                // mutated ring lists and loop back-pointers in
                // between, and a fact from before a mutation proves
                // nothing about this call.
                debug_assert!(
                    self.get_loop(ring)
                        .and_then(|l| self.get_face(l.face))
                        .is_none_or(|f| f.outer != ring),
                    "merge_group: {}",
                    EstablishedFact::RingIsNotItsFacesOuter.what()
                );
                debug_assert!(
                    self.get_loop(ring)
                        .and_then(|l| self.get_face(l.face))
                        .zip(self.get_face(rep))
                        .is_none_or(|(from, to)| from.shell == to.shell),
                    "merge_group: {}",
                    EstablishedFact::RingAndSurvivorShareAShell.what()
                );
                #[cfg(test)]
                tear_before_ring_move(self, ring);
                self.ring_move_minting(ring, rep, tol)?;
            }
            // The two facts `kef` could contradict. The drain above is
            // exactly the mutation that invalidates a fact read before
            // it — it is why `SameFace` is NOT one of these — so both
            // are re-derived here, after it.
            debug_assert!(
                self.get_half_edge(dying_he)
                    .zip(self.edge_mate(dying_he, edge_key))
                    .is_none_or(|(dying, mate)| dying.parent_loop != mate.parent_loop),
                "merge_group: {}",
                EstablishedFact::DyingHalvesAreInDifferentLoops.what()
            );
            debug_assert!(
                self.get_face(other).is_none_or(|f| f.rings.is_empty()),
                "merge_group: {}",
                EstablishedFact::AbsorbedFaceIsRingFree.what()
            );
            #[cfg(test)]
            tear_before_kef(self, dying_he, edge_key, other);
            self.kef_minting(dying_he, tol)?;
            group.absorbed.push(other);
            group.killed_edges.push(edge_key);
        }
        // Intra-face duplicates: edges now occurring twice within the
        // survivor's loops. On a CURVED survivor (C12.5, M5 PR 9) a
        // same-face duplicate means the cosurface run CLOSED THE FULL
        // PERIOD — a shape outside the merge's inventory (neither the
        // ring form nor the kept-cut seam form is integrable by the
        // exact-B-rep props yet), refused typed here; the driver
        // records it as a LOUD skip, so sub-period re-merges (the C12.5
        // through-cut case) proceed and full closures stay unmerged
        // exactly as the operands arrived.
        //
        // On a PLANAR survivor every duplicate is one of two things,
        // told apart by topology alone:
        //
        // - **A dangling seam edge** — one end has no other edge (a
        //   valence-one vertex). It encloses no area: deleting it and
        //   its free end leaves the merged face's region exactly as it
        //   was, whatever the edge's direction or the angle the seam
        //   turned there. `kev` deletes both. A shared chain of `k`
        //   edges loses its `k − 1` interior junctions this way, one
        //   free end at a time, and a junction where several absorbed
        //   faces met goes with the last of its spokes. An edge with
        //   BOTH ends free is a ring holding nothing but itself (the
        //   last edge of a doubled cycle, say the rim of a hole a
        //   coplanar face plugged exactly): both ends go, the second
        //   with the ring, which by then is a lone vertex and encloses
        //   nothing either. Every vertex deleted here is the free end
        //   of a seam edge the glue left dangling, none is on the
        //   merged face's boundary, none is fused into another, and
        //   each is recorded in `killed_vertices`.
        // - **A doubled edge with no free end** separates a genuine
        //   ring from the outline: `kemr` mints it. Dangling edges are
        //   pruned first, and again after every ring, so `kemr` only
        //   ever sees an edge whose two ends both still bound
        //   something.
        loop {
            let mut duplicates = Vec::new();
            for (edge_key, edge) in self.edges() {
                let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
                if hp.face == rep && hm.face == rep {
                    duplicates.push((edge_key, (edge.he_plus, hp), (edge.he_minus, hm)));
                }
            }
            let Some(&(first_key, _, _)) = duplicates.first() else {
                break;
            };
            if kind == MergeKind::Curved {
                return Err(MergeCoplanarError::PeriodClosure { edge: first_key });
            }
            // The first dangling edge in edge-arena order, with the
            // half that runs TOWARD its free end: `kev` takes that
            // half's edge and its end vertex. `vertex_orbit` walks the
            // halves STARTING at its argument's start vertex, so each
            // candidate asks about the far end of the other half.
            //
            // A BROKEN orbit is announced, not read as "not dangling":
            // `kev` and `kemr` are different operators with different
            // Euler deltas, so letting a torn arena answer this
            // question silently chooses which surgery runs.
            let mut strut = None;
            'scan: for &(edge_key, (he_plus, hp), (he_minus, hm)) in &duplicates {
                for (toward_free, from_free, near, free) in [
                    (he_plus, he_minus, hp.start, hm.start),
                    (he_minus, he_plus, hm.start, hp.start),
                ] {
                    if self.strut_tip(from_free)? {
                        strut = Some((edge_key, toward_free, near, free, hp.r#loop));
                        break 'scan;
                    }
                }
            }
            if let Some((edge_key, toward_free, near, free, ring)) = strut {
                // BOTH ends free: the edge is its own loop, a ring of
                // the survivor holding nothing but itself — the last
                // edge of a doubled cycle the pruning has taken the
                // rest of. `kev` leaves that ring as a lone vertex, and
                // `mekr` then `kev` delete the vertex with its ring.
                // The ring is never the survivor's outline (an outline
                // bounds the face's area); if the arena says it is,
                // the shape is refused before any of the three calls.
                let other = self.strut_tip(toward_free)?.then_some(near);
                let outline = self
                    .get_face(rep)
                    .ok_or(DanglingRef::Entity(EntityId::Face(rep)))?
                    .outer;
                if other.is_some() && ring == outline {
                    return Err(MergeCoplanarError::UnsupportedConfiguration { edge: edge_key });
                }
                #[cfg(test)]
                tear_before_kev(self, toward_free, edge_key);
                self.kev(toward_free)?;
                group.killed_edges.push(edge_key);
                group.killed_vertices.push(free);
                if let Some(lone) = other {
                    let target = match self
                        .get_loop(outline)
                        .ok_or(DanglingRef::Entity(EntityId::Loop(outline)))?
                        .boundary
                    {
                        crate::entity::LoopBoundary::Cycle { first } => first,
                        crate::entity::LoopBoundary::Empty { .. } => {
                            return Err(MergeCoplanarError::Op {
                                error: EulerOpError::LoopNotCycle { r#loop: outline },
                            });
                        }
                    };
                    let bridge =
                        self.mekr_chord(crate::MekrSite::EmptyRing { target, ring }, tol)?;
                    self.kev(bridge.he_plus)?;
                    group.killed_vertices.push(lone);
                }
                continue;
            }
            let (edge_key, (he_plus, hp), (he_minus, hm)) = duplicates[0];
            if hp.r#loop != hm.r#loop {
                return Err(MergeCoplanarError::UnsupportedConfiguration { edge: edge_key });
            }
            // The fact `kemr` could contradict, re-read at the call.
            debug_assert!(
                self.get_half_edge(he_plus)
                    .zip(self.get_half_edge(he_minus))
                    .is_none_or(|(a, b)| a.parent_loop == b.parent_loop),
                "merge_group: {}",
                EstablishedFact::DuplicateHalvesShareALoop.what()
            );
            #[cfg(test)]
            tear_before_kemr(self, he_minus);
            let result = self.kemr(he_plus, he_minus)?;
            group.killed_edges.push(edge_key);
            group.rings_made.push(result.ring);
        }
        // Role normalization: the intra-face `kemr` above designates its ring PROVISIONALLY
        // (the plus half's side — the module docs' documented
        // containment question). A group absorbed across TWO disjoint
        // shared runs closes a genuine hole, and the provisional side
        // can put the OUTLINE in the ring slot and the hole in the
        // outer slot — every downstream volume gate is role-invariant,
        // but tessellation is not (the silent-corrupt-export class).
        // Resolve by winding: the outline is the unique cycle winding
        // POSITIVELY around the face's outward normal (the same
        // Newell-functional predicate the boolean join's ring lane
        // decides on); swap it into the outer slot if the kemr put it
        // elsewhere; no unique positive cycle refuses with the verdict
        // that left none ([`Body::merged_outline_ring`]).
        if !group.rings_made.is_empty() {
            // The survivor is resolved HERE, where its liveness is
            // proven in this call: `rings_made` is non-empty only
            // because a `kemr` returned, and `kemr` requires the
            // duplicate's loop's face — `rep`, the face both halves
            // were found on through that loop — to be live
            // (`require_key` on the loop's `face`) and kills no face.
            // The role pass takes the resolved face record; the
            // surface, loop and walk lookups it makes read the arena
            // the surgery has just mutated, and announce what they
            // cannot resolve.
            let Some(survivor) = self.get_face(rep) else {
                unreachable!(
                    "merge_group: `rep` was required live by the `kemr` that minted the last \
                     ring, and `kemr` kills no face"
                )
            };
            let survivor = survivor.clone();
            #[cfg(test)]
            tear_before_role_pass(self, &survivor);
            if let Some(i) = self.merged_outline_ring(rep, &survivor, tol)? {
                let Some(fm) = self.faces.get_mut(rep) else {
                    unreachable!(
                        "merge_group: `rep` resolved a few lines above and the winding \
                         pass between is read-only"
                    )
                };
                fm.outer = survivor.rings[i];
                fm.rings[i] = survivor.outer;
            }
        }
        Ok(group)
    }

    /// [`Body::planar_loop_winding_decided`], in this door's error
    /// vocabulary: a torn walk names the key that did not resolve, the
    /// half-edge its edge does not claim, or the loop whose walk did
    /// not close, and an in-band margin escalates. The decided sign
    /// keeps its margin, which a zero winding's refusal quotes.
    /// `normal` is the face's OUTWARD normal.
    fn loop_winding(
        &self,
        l: LoopKey,
        normal: geom_core::Vec3<T>,
        band: Band,
    ) -> Result<LoopWinding<Decided>, MergeCoplanarError> {
        let winding =
            self.planar_loop_winding_decided(l, normal, band)
                .map_err(|torn| match torn {
                    TornLoop::Dangling(what) => MergeCoplanarError::from(what),
                    TornLoop::Unclaimed { he, edge } => {
                        MergeCoplanarError::from(EulerOpError::UnclaimedHalfEdge { he, edge })
                    }
                    TornLoop::Unclosed => {
                        MergeCoplanarError::from(EulerOpError::LoopCycleBroken { r#loop: l })
                    }
                })?;
        match winding {
            LoopWinding::Empty => Ok(LoopWinding::Empty),
            LoopWinding::Unsupported => Ok(LoopWinding::Unsupported),
            LoopWinding::Wound(Ok(decided)) => Ok(LoopWinding::Wound(decided)),
            LoopWinding::Wound(Err(diag)) => Err(MergeCoplanarError::Escalated {
                decision: MergeDecision::LoopWinding,
                diag,
            }),
        }
    }

    /// Which of the merged survivor's rings is its outline, decided by
    /// winding (doc at the call site): the unique positively-wound
    /// cycle is the outer loop. `Some(i)` means ring `i` must swap into
    /// the outer slot; `None` means the roles are already correct, or
    /// the survivor is not a plane, which has no winding question here
    /// (a curved run refuses `PeriodClosure` before any `kemr`, so the
    /// survivor is a plane at every call).
    ///
    /// Takes the survivor's resolved face record: the caller holds its
    /// liveness proof. `face` is the refusal's payload only.
    ///
    /// # Errors
    ///
    /// When no loop is the unique outline, the verdict that left none,
    /// in this order:
    ///
    /// - several positive windings:
    ///   [`MergeCoplanarError::MergedFaceRoleAmbiguous`]
    ///   ([`OutlineVerdict::SeveralPositive`]);
    /// - no positive winding and a zero one:
    ///   [`MergeCoplanarError::Escalated`] ([`MergeDecision::LoopWinding`],
    ///   the zero winding's own margin), the winding decision's
    ///   band-decided refusal, named before a loop the kernel does not
    ///   wind, which counts as not positive;
    /// - no positive or zero winding and a loop the kernel does not
    ///   wind: [`OutlineVerdict::UnsupportedWinding`];
    /// - otherwise every loop is negative or a lone vertex:
    ///   [`OutlineVerdict::AllNegative`].
    ///
    /// A surface or walk key that does not resolve, or a half-edge its
    /// edge does not claim, is [`MergeCoplanarError::Op`] naming it:
    /// this pass reads the arena after the surgery has mutated it, so
    /// the entry gate's proof does not reach it.
    fn merged_outline_ring(
        &self,
        face: FaceKey,
        survivor: &crate::entity::Face,
        tol: Tol,
    ) -> Result<Option<usize>, MergeCoplanarError> {
        let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
        let described = self
            .get_surface(survivor.surface)
            .ok_or(DanglingRef::Geometry(GeomRef::Surface(survivor.surface)))?;
        // The face's OUTWARD normal (S10): "positively wound" means
        // CCW seen from OUTSIDE the material, so on a reversed face
        // the chart normal names the opposite convention and every
        // role assignment below would come out inverted.
        let Surface::Plane { normal, .. } = described else {
            return Ok(None);
        };
        let normal = plane_outward_normal(survivor, *normal).vec();
        // Every loop's verdict, outline slot first (`None`). A negative
        // winding and an empty loop are not positive and name nothing.
        let mut positive: Vec<(Option<usize>, LoopKey)> = Vec::new();
        let mut zero = None;
        let mut unsupported = None;
        let slots = core::iter::once((None, survivor.outer)).chain(
            survivor
                .rings
                .iter()
                .enumerate()
                .map(|(i, &r)| (Some(i), r)),
        );
        for (slot, l) in slots {
            match self.loop_winding(l, normal, band)? {
                LoopWinding::Wound(Decided {
                    sign: geom_core::Sign::Positive,
                    ..
                }) => positive.push((slot, l)),
                LoopWinding::Wound(Decided {
                    sign: geom_core::Sign::Zero,
                    margin,
                }) => {
                    zero.get_or_insert(margin);
                }
                LoopWinding::Wound(Decided {
                    sign: geom_core::Sign::Negative,
                    ..
                })
                | LoopWinding::Empty => {}
                LoopWinding::Unsupported => {
                    unsupported.get_or_insert(l);
                }
            }
        }
        let verdict = match (&positive[..], zero, unsupported) {
            (&[(slot, _)], _, _) => return Ok(slot),
            ([_, _, ..], _, _) => OutlineVerdict::SeveralPositive {
                loops: positive.iter().map(|&(_, l)| l).collect(),
            },
            ([], Some(margin), _) => {
                return Err(MergeCoplanarError::Escalated {
                    decision: MergeDecision::LoopWinding,
                    diag: Indeterminate {
                        margin,
                        band,
                        predicate: Some(WINDING_PREDICATE),
                        terminal_sliver: false,
                    },
                });
            }
            ([], None, Some(r#loop)) => OutlineVerdict::UnsupportedWinding { r#loop },
            ([], None, None) => OutlineVerdict::AllNegative,
        };
        Err(MergeCoplanarError::MergedFaceRoleAmbiguous { face, verdict })
    }
}

#[cfg(test)]
#[path = "merge_faces_kept_rows.rs"]
pub(crate) mod kept_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::test_support_fixtures::{declined_cube, plant_ring_face};

    /// A shared edge of the ops cube, as the pair of faces meeting
    /// there — addressed exactly as the absorption scan addresses it,
    /// through the loops' `face` back-pointers.
    fn adjacent_pair(body: &Body<f64>) -> (FaceKey, FaceKey) {
        body.edges()
            .find_map(|(_, e)| {
                let (hp, hm) = body.edge_halves(e.he_plus, e.he_minus).ok()?;
                (hp.face != hm.face).then_some((hp.face, hm.face))
            })
            .expect("a cube has adjacent faces")
    }

    /// **A dangling absorbed-face key is refused typed, naming the
    /// face** — the contract the absorption owes, pinned end to end.
    ///
    /// It does NOT isolate the ring lookup, and cannot: `kef`'s own
    /// plan phase re-derives the same key
    /// (`loops[half_edges[dying_he].parent_loop].face`) and refuses on
    /// it with the same value, before any mutation. The two sites are
    /// indistinguishable from outside by construction — which is why
    /// the earlier `unwrap_or_default()` there could never actually
    /// drop a ring. This row pins the answer; it is not evidence about
    /// which of the two produced it.
    #[test]
    fn a_dangling_absorbed_face_key_is_refused_typed_and_names_the_face() {
        let tol = Tol::witness();
        let mut body = structural_planar_cube(tol);
        let (rep, other) = adjacent_pair(&body);
        body.faces
            .remove(other)
            .expect("the pair's second face is live before the tear");

        assert_eq!(
            body.merge_group(rep, &[other], MergeKind::Plane, tol),
            Err(MergeCoplanarError::Op {
                error: EulerOpError::StaleKey {
                    key: EntityId::Face(other),
                },
            }),
        );
    }

    /// The control: with the arena intact the same call absorbs, so
    /// the row above pins the tear and not the fixture.
    #[test]
    fn absorption_of_the_same_pair_runs_on_an_intact_arena() {
        let tol = Tol::witness();
        let mut body = structural_planar_cube(tol);
        let (rep, other) = adjacent_pair(&body);

        let group = body
            .merge_group(rep, &[other], MergeKind::Plane, tol)
            .expect("an intact adjacent pair absorbs");
        assert_eq!(group.kept, rep);
        assert_eq!(group.absorbed, vec![other]);
    }

    // ---- Fixtures whose REGIME is a property of the fixture ----
    //
    // A group's regime must be asserted, never inherited from an
    // accident of the fixture: `declined_cube`'s faces sit on the `mvfs`
    // NURBS placeholder, which has no regime at all — the door sets
    // the whole cube aside and no surgery runs. These two build the
    // regime deliberately, out of planes.

    /// The unit cube with the `mvfs` placeholder overwritten IN PLACE
    /// by a real plane, so every face shares one plane key: the
    /// structural rung groups the whole cube and, being planar and
    /// undeclared, it runs under [`GroupRegime::RefusesTheCall`].
    fn structural_planar_cube(tol: Tol) -> Body<f64> {
        let mut body = declined_cube::<f64>(tol).body;
        describe_shared_key(&mut body);
        body
    }

    /// One plane description, used for every face of the planar
    /// fixtures: the merge's rungs never compare coordinates, so one
    /// description on distinct keys is exactly the declared rung's
    /// subject.
    fn flat_plane() -> Surface<f64> {
        Surface::Plane {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// Gives every face of `body` its OWN plane key carrying one
    /// description, and returns the declared pairs that glue them:
    /// planar and licensed only by the declaration.
    fn declare_planes_pairwise(body: &mut Body<f64>) -> Vec<(SurfaceKey, SurfaceKey)> {
        describe_shared_key(body);
        let key = body.faces().next().expect("faces").1.surface;
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let mut keys = vec![key];
        for f in &faces[1..] {
            let plane = body.add_surface(flat_plane());
            body.faces.get_mut(*f).expect("live").surface = plane;
            keys.push(plane);
        }
        keys[1..].iter().map(|&k| (keys[0], k)).collect()
    }

    /// The declared planar cube: a group licensed by declarations
    /// alone, with no curved face anywhere.
    fn declared_planar_cube(tol: Tol) -> (Body<f64>, Vec<(SurfaceKey, SurfaceKey)>) {
        let mut body = declined_cube::<f64>(tol).body;
        let declared = declare_planes_pairwise(&mut body);
        (body, declared)
    }

    /// The contract of the group holding every face of `body`, asked
    /// the way the door asks it.
    fn contract_of(body: &Body<f64>) -> GroupContract {
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let kinds = body
            .kind_census()
            .expect("the fixture's faces resolve")
            .kinds;
        Body::<f64>::group_contract(faces[0], &faces[1..], &kinds)
            .expect("the census holds every face")
    }

    /// A planar run's contract: the refusing regime, whatever
    /// licensed the adjacency.
    const PLANAR: GroupContract = GroupContract::Runs {
        regime: GroupRegime::RefusesTheCall,
        kind: MergeKind::Plane,
    };

    /// A curved run's contract: the recording regime.
    const CURVED: GroupContract = GroupContract::Runs {
        regime: GroupRegime::RecordsASkip,
        kind: MergeKind::Curved,
    };

    /// Re-describes the fixture's one shared placeholder key as the
    /// unit cylinder, in place: one curved same-key run over the whole
    /// body, the recording regime's fixture.
    fn describe_shared_key_curved(body: &mut Body<f64>) {
        let key = body.faces().next().expect("faces").1.surface;
        *body
            .surfaces
            .get_mut(key)
            .expect("the placeholder resolves") = unit_cylinder();
    }

    /// Re-describes the fixture's one shared placeholder key as a
    /// real plane, in place — the key is overwritten rather than
    /// replaced because a fresh key would orphan the placeholder and
    /// the entry gate refuses an orphaned surface.
    fn describe_shared_key(body: &mut Body<f64>) {
        let key = body.faces().next().expect("faces").1.surface;
        *body
            .surfaces
            .get_mut(key)
            .expect("the placeholder resolves") = flat_plane();
    }

    /// Runs the public door with one tear point armed, and returns
    /// the operator refusal it escaped with. The guard disarms on
    /// drop, so a failing assertion cannot leave the point armed.
    fn escaped_refusal(
        point: TearPoint,
        body: &mut Body<f64>,
        declared: &[(SurfaceKey, SurfaceKey)],
        tol: Tol,
    ) -> EulerOpError {
        let outcome = {
            let _armed = ArmedTear::at(point);
            body.merge_coplanar_faces_declared(declared, tol)
        };
        match outcome {
            Err(MergeCoplanarError::Op { error }) => error,
            other => panic!("{point:?}: expected the refusal to escape as Err(Op), got {other:?}"),
        }
    }

    /// **Every planar run refuses and every curved run records** —
    /// the regime is the kind's, whatever licensed the adjacency, so
    /// a declared planar group the merge cannot glue refuses the
    /// call (and the boolean step that ran it) exactly as a
    /// structural one does. Every row below states which fixture
    /// carries which regime instead of inheriting one.
    #[test]
    fn planar_runs_refuse_and_curved_runs_record() {
        let tol = Tol::witness();
        assert_eq!(contract_of(&structural_planar_cube(tol)), PLANAR);
        let (declared_body, _) = declared_planar_cube(tol);
        assert_eq!(contract_of(&declared_body), PLANAR);
        let mut curved = curved_same_key_cube(tol);
        assert_eq!(contract_of(&curved), CURVED);
        // ...and the recording fixture reaches the surgery untorn: its
        // refusal comes back as a record, not an `Err`.
        let outcome = curved
            .merge_coplanar_faces(tol)
            .expect("a curved run's inventory refusal is recorded");
        let [skipped] = &outcome.skipped[..] else {
            panic!("one recorded skip: {:?}", outcome.skipped)
        };
        assert!(
            matches!(skipped.reason, MergeCoplanarError::PeriodClosure { .. }),
            "{:?}",
            skipped.reason
        );
    }

    /// The fixture a tear point needs: the two `ring_move` tears need
    /// a group whose absorption has a ring to re-home, and `kemr`
    /// needs a merged face with a genuine hole — a doubled edge with
    /// no free end — which a bare cube has not (its seven doubled
    /// edges form a tree, and the pruning takes them all).
    fn fixture_for(point: TearPoint, tol: Tol) -> Body<f64> {
        match point {
            TearPoint::RingBecomesItsFacesOuter | TearPoint::RingsFaceLeavesTheShell => {
                crate::fixtures::ops_holed_box(tol).body
            }
            TearPoint::DuplicateHalvesPartCompany => split_ringed_top(tol),
            _ => declined_cube::<f64>(tol).body,
        }
    }

    /// The half-edge of `face`'s outer loop that starts at `(x, y, z)`.
    fn outer_he_at(body: &Body<f64>, face: FaceKey, at: [f64; 3]) -> crate::entity::HalfEdgeKey {
        let outer = body.get_face(face).expect("live").outer;
        loop_he_at(body, outer, at)
    }

    /// The half-edge of `l` that starts at `(x, y, z)`.
    fn loop_he_at(body: &Body<f64>, l: LoopKey, at: [f64; 3]) -> crate::entity::HalfEdgeKey {
        let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(l).expect("live").boundary
        else {
            panic!("the loop is a cycle")
        };
        body.loop_cycle(first)
            .expect("the cycle closes")
            .into_iter()
            .find(|&he| {
                let v = body.get_half_edge(he).expect("live").start;
                let p = body
                    .get_point(body.get_vertex(v).expect("live").point)
                    .expect("live");
                p.to_array() == at
            })
            .expect("a half-edge starts there")
    }

    /// **Two U-shaped faces around a hole**, the only same-key pair
    /// of the body: the membrane cube's ringed top, bridged to its
    /// ring from the corner (0, 0) and cut again from the corner
    /// (1, 1) to the ring's far corner, so its two halves share two
    /// straight seams, each running from the outline to the hole.
    /// Every other face — the membrane included — is on a plane key
    /// of its own. Merging the halves `kef`s one seam and leaves the
    /// other doubled with no free end, which is `kemr`'s case.
    fn split_ringed_top(tol: Tol) -> Body<f64> {
        let (mut body, top, _membrane) = cube_with_membrane(tol);
        describe_shared_key(&mut body);
        let ring = body.get_face(top).expect("live").rings[0];
        body.mekr_chord(
            crate::MekrSite::Cycles {
                target: outer_he_at(&body, top, [0.0, 0.0, 1.0]),
                ring: loop_he_at(&body, ring, [0.25, 0.25, 1.0]),
            },
            tol,
        )
        .expect("the bridge joins the ring to the outline");
        let half = body
            .mef_chord(
                crate::MefSite::Chords {
                    he1: outer_he_at(&body, top, [1.0, 1.0, 1.0]),
                    he2: outer_he_at(&body, top, [0.75, 0.75, 1.0]),
                },
                tol,
            )
            .expect("the second cut splits the top in two")
            .face;
        let others: Vec<FaceKey> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|&k| k != top && k != half)
            .collect();
        for f in others {
            body.set_face_surface(
                f,
                crate::euler::FaceSurface::New {
                    surface: flat_plane(),
                    sense: true,
                },
            )
            .expect("a live face takes a surface");
        }
        assert_eq!(
            validate_closed(&body),
            Ok(()),
            "the fixture is tier-2 legal"
        );
        body
    }

    /// The split ringed top merges back into one face carrying the
    /// hole as a ring: `kef` on one seam, `kemr` on the other, nothing
    /// pruned.
    #[test]
    fn a_seam_with_no_free_end_mints_a_ring() {
        let tol = Tol::witness();
        let mut body = split_ringed_top(tol);
        let outcome = body.merge_coplanar_faces(tol).expect("the halves merge");
        let [group] = &outcome.groups[..] else {
            panic!("one group: {:?}", outcome.groups)
        };
        assert_eq!(group.absorbed.len(), 1);
        assert_eq!(group.rings_made.len(), 1, "{group:?}");
        assert!(group.killed_vertices.is_empty(), "{group:?}");
        assert_eq!(body.get_face(group.kept).expect("live").rings.len(), 1);
        assert_eq!(validate_closed(&body), Ok(()));
    }

    /// **Every key the adjacency test reads that does not resolve is
    /// announced by name** — not `false`, which drops a mergeable
    /// adjacency without a word, and not an extent read over a boundary
    /// the declared rung could not walk. A face's surface is
    /// resolved by the kind census, so its tear refuses there; the
    /// adjacency test announces a face the census did not meet and an
    /// end vertex or point. Asked of the census and the helper
    /// directly: the door's entry gate refuses each of these tears
    /// first (`the_entry_gate_refuses_every_tear_the_adjacency_scan_reads`).
    /// The first row is the control: the intact pair is declared one
    /// plane, so each torn row pins its tear and not the fixture.
    #[test]
    fn a_torn_adjacency_lookup_is_announced_naming_its_key() {
        let tol = Tol::witness();
        type Tear = fn(&mut Body<f64>, HalfEdgeFacts, HalfEdgeFacts) -> Option<DanglingRef>;
        let rows: [(&str, Tear); 5] = [
            ("nothing", |_, _, _| None),
            ("a face", |b, _, hm| {
                b.faces.remove(hm.face);
                Some(DanglingRef::Entity(EntityId::Face(hm.face)))
            }),
            ("a face's surface", |b, _, hm| {
                let k = b.get_face(hm.face).expect("live").surface;
                b.surfaces.remove(k);
                Some(DanglingRef::Geometry(GeomRef::Surface(k)))
            }),
            ("an end vertex", |b, hp, _| {
                b.vertices.remove(hp.start);
                Some(DanglingRef::Entity(EntityId::Vertex(hp.start)))
            }),
            ("an end point", |b, _, hm| {
                let p = b.get_vertex(hm.start).expect("live").point;
                b.points.remove(p);
                Some(DanglingRef::Geometry(GeomRef::Point(p)))
            }),
        ];
        for (what, tear) in rows {
            let (mut body, declared) = declared_planar_cube(tol);
            let mut eq = DeclaredSurfaceEq::default();
            for &(a, b) in &declared {
                eq.union(a, b);
            }
            let ctx = DeclaredCtx {
                eq,
                band: Band::linear(tol).expect("the witness tolerance forms a band"),
            };
            let edge = body.edges().next().expect("an edge").1;
            let (hp, hm) = body
                .edge_halves(edge.he_plus, edge.he_minus)
                .expect("the fixture resolves");
            let want = match tear(&mut body, hp, hm) {
                None => Ok(true),
                Some(key) => Err(MergeCoplanarError::from(key)),
            };
            let got = body.kind_census().and_then(|census| {
                body.planes_declared_equal(hp, hm, &census.described, Some(&ctx))
            });
            assert_eq!(got, want, "tearing {what}");
        }
    }

    /// **The entry gate refuses every tear the adjacency scan's
    /// helpers read** — the measurement behind their docs' claim that
    /// no input reaches a failed lookup there. Each row tears a
    /// declared planar cube the way one of `planes_declared_equal`'s
    /// lookups would fail (a face a loop points at, that face's
    /// surface, a vertex a half-edge starts at, that vertex's point)
    /// and drives the public door: tier 2 refuses the input, and the
    /// body comes back exactly as it went in.
    #[test]
    fn the_entry_gate_refuses_every_tear_the_adjacency_scan_reads() {
        let tol = Tol::witness();
        type Tear = fn(&mut Body<f64>);
        let tears: [(&str, Tear); 4] = [
            ("a face", |b| {
                let (_, f) = adjacent_pair(b);
                b.faces.remove(f);
            }),
            ("a face's surface", |b| {
                let (f, _) = adjacent_pair(b);
                let k = b.get_face(f).expect("live").surface;
                b.surfaces.remove(k);
            }),
            ("a vertex", |b| {
                let v = b.vertices().next().expect("a vertex").0;
                b.vertices.remove(v);
            }),
            ("a vertex's point", |b| {
                let p = b.vertices().next().expect("a vertex").1.point;
                b.points.remove(p);
            }),
        ];
        for (what, tear) in tears {
            let (mut body, declared) = declared_planar_cube(tol);
            tear(&mut body);
            let before = format!("{body:?}");
            let got = body.merge_coplanar_faces_declared(&declared, tol);
            assert!(
                matches!(got, Err(MergeCoplanarError::InputNotClosed { .. })),
                "tearing {what}: {got:?}"
            );
            assert_eq!(
                format!("{body:?}"),
                before,
                "tearing {what}: the refusal leaves the body as it was"
            );
        }
    }

    /// Runs the public door on the split ringed top — whose merge
    /// mints a ring and so reaches the role pass — with one lookup
    /// tear armed there, checks the body comes back as it went in (the
    /// door stages on a clone), and returns the key the refusal names.
    fn role_pass_tear(point: TearPoint) -> (Body<f64>, GeomRef) {
        let tol = Tol::witness();
        let mut body = split_ringed_top(tol);
        let before = format!("{body:?}");
        let got = {
            let _armed = ArmedTear::at(point);
            body.merge_coplanar_faces(tol)
        };
        assert_eq!(
            format!("{body:?}"),
            before,
            "{point:?}: the refusal leaves the body as it was"
        );
        match got {
            Err(MergeCoplanarError::Op {
                error: EulerOpError::StaleGeometry { key },
            }) => (body, key),
            other => panic!("{point:?}: expected the torn key announced, got {other:?}"),
        }
    }

    /// **The survivor's surface, torn under the role pass, refuses the
    /// call naming that surface** — not read as "not a plane", which
    /// would leave the roles as the `kemr` put them. The role pass
    /// reads the arena after the surgery has mutated it, so the entry
    /// gate's proof does not reach it.
    #[test]
    fn a_torn_survivor_surface_in_the_role_pass_refuses_naming_it() {
        let (body, key) = role_pass_tear(TearPoint::SurvivorLosesItsSurface);
        let GeomRef::Surface(k) = key else {
            panic!("named {key:?}, not the surface")
        };
        assert_eq!(
            body.faces().filter(|(_, f)| f.surface == k).count(),
            2,
            "names the surface the merged pair shares"
        );
    }

    /// **A point torn under the winding walk refuses naming the
    /// point**, not the loop the walk started from.
    #[test]
    fn a_torn_outline_point_in_the_role_pass_refuses_naming_it() {
        let (body, key) = role_pass_tear(TearPoint::OutlineLosesAPoint);
        let GeomRef::Point(p) = key else {
            panic!("named {key:?}, not the point")
        };
        assert!(
            body.vertices().any(|(_, v)| v.point == p),
            "names a vertex's point"
        );
    }

    /// The facts whose tear reaches its operator under the RECORDING
    /// regime, each with the [`EstablishedFact`] the re-check before
    /// that call proves.
    ///
    /// Only the absorption's four: a curved survivor refuses
    /// `PeriodClosure` at its first same-face duplicate, before either
    /// intra-face operator runs, so `kev` and `kemr` are reached only
    /// by planar runs — which refuse the call whatever they raise.
    const RECORDING_FACTS: [(TearPoint, EstablishedFact); 4] = [
        (
            TearPoint::RingBecomesItsFacesOuter,
            EstablishedFact::RingIsNotItsFacesOuter,
        ),
        (
            TearPoint::RingsFaceLeavesTheShell,
            EstablishedFact::RingAndSurvivorShareAShell,
        ),
        (
            TearPoint::DyingHalfJoinsItsMatesLoop,
            EstablishedFact::DyingHalvesAreInDifferentLoops,
        ),
        (
            TearPoint::DrainedFaceRegainsARing,
            EstablishedFact::AbsorbedFaceIsRingFree,
        ),
    ];

    /// Every fact the surgery re-checks, each reached by a planar run
    /// under the REFUSING regime.
    const EVERY_FACT: [(TearPoint, EstablishedFact); 6] = [
        RECORDING_FACTS[0],
        RECORDING_FACTS[1],
        RECORDING_FACTS[2],
        RECORDING_FACTS[3],
        (
            TearPoint::StrutBecomesASelfLoop,
            EstablishedFact::StrutHalvesHaveDistinctEnds,
        ),
        (
            TearPoint::DuplicateHalvesPartCompany,
            EstablishedFact::DuplicateHalvesShareALoop,
        ),
    ];

    /// **A refusal that contradicts a re-checked fact escapes under
    /// the RECORDING regime.** One row-body per [`EstablishedFact`]
    /// the surgery re-checks and a curved run can reach, driven
    /// through the PUBLIC door with the matching tear point armed.
    ///
    /// Each tear sits immediately after the `debug_assert!` that
    /// proves the fact and immediately before the operator call, so
    /// it exercises exactly the window the re-check cannot cover —
    /// which is the whole of what "contradicts a fact this door
    /// re-checks" claims. A torn INPUT cannot reach any of these
    /// arms: the entry gate refuses one before a group is staged.
    ///
    /// Reds against asking the enum alone: every variant here answers
    /// `false` to [`EulerOpError::reports_tier1_corruption`], so
    /// under [`GroupRegime::RecordsASkip`] the door returned `Ok`
    /// with the corruption filed as an inventory skip.
    #[test]
    fn every_contradicted_fact_escapes_the_recording_regime() {
        let tol = Tol::witness();
        for (point, want) in RECORDING_FACTS {
            let mut body = fixture_for(point, tol);
            describe_shared_key_curved(&mut body);
            assert_eq!(contract_of(&body), CURVED, "{point:?}");
            let error = escaped_refusal(point, &mut body, &[], tol);
            assert!(
                !error.reports_tier1_corruption(),
                "{error} is the enum's already; this row would prove nothing"
            );
            assert_eq!(
                OpPlacement::of(&error),
                OpPlacement::Contradicts(want),
                "{point:?} produced {error}"
            );
            assert!(MergeCoplanarError::Op { error }.is_arena_fault());
        }
    }

    /// Every tear under [`GroupRegime::RefusesTheCall`], where the
    /// escape changes no outcome and the row's value is that each
    /// operator is reached and its refusal still names the operator's
    /// own variant — `kev` on the cube's pruned seam tree, `kemr` on
    /// the holed box's ring.
    #[test]
    fn every_contradicted_fact_refuses_the_refusing_regime() {
        let tol = Tol::witness();
        for (point, want) in EVERY_FACT {
            let mut body = fixture_for(point, tol);
            describe_shared_key(&mut body);
            assert_eq!(contract_of(&body), PLANAR, "{point:?}");
            let error = escaped_refusal(point, &mut body, &[], tol);
            assert_eq!(
                OpPlacement::of(&error),
                OpPlacement::Contradicts(want),
                "{point:?} produced {error}"
            );
        }
    }

    // ---- The nested (membrane) group: `SameFace` is INVENTORY ----
    //
    // The reviewer's falsification, adopted. The absorption's own
    // `ring_move` drain re-homes the dying loop onto the survivor, so
    // a fact the scan read before it proves nothing at the `kef`
    // after it — which is why `SameFace` is placed by the enum and
    // not contradicted here.

    /// The unit cube whose TOP face carries a square inner ring, with
    /// a coplanar membrane face covering the opening — the holed
    /// box's construction stopped one step before the tube is grown.
    /// Built entirely by Euler operators, so it is a tier-1/tier-2
    /// valid body. Its faces stay on the `mvfs` placeholder key; a row
    /// that runs the surgery on it describes that key first.
    ///
    /// The shared rim edges have their top-face half in the top
    /// face's RING and their membrane half in the membrane's outer
    /// loop, which is the nesting the drain re-homes.
    fn cube_with_membrane(tol: Tol) -> (Body<f64>, FaceKey, FaceKey) {
        let crate::test_support_fixtures::CubeOps {
            mut body,
            seed,
            mefs,
            ..
        } = declined_cube::<f64>(tol);
        let membrane = plant_ring_face(
            &mut body,
            mefs[1].he_plus,
            &[
                geom_core::Point3::new(0.25, 0.25, 1.0),
                geom_core::Point3::new(0.75, 0.25, 1.0),
                geom_core::Point3::new(0.75, 0.75, 1.0),
                geom_core::Point3::new(0.25, 0.75, 1.0),
            ],
            tol,
        )
        .membrane;
        (body, seed.face, membrane.face)
    }

    /// The membrane fixture is a valid closed body with the nesting
    /// the row below depends on.
    #[test]
    fn the_membrane_fixture_is_valid_and_nested() {
        let tol = Tol::witness();
        let (body, top, membrane) = cube_with_membrane(tol);
        assert_eq!(crate::validate::validate(&body), Ok(()));
        assert_eq!(validate_closed(&body), Ok(()));
        assert_eq!(body.get_face(top).expect("live").rings.len(), 1);
        assert!(body.get_face(membrane).expect("live").rings.is_empty());
        assert_eq!(
            body.get_face(top).expect("live").surface,
            body.get_face(membrane).expect("live").surface
        );
    }

    /// **A ring half-edge its own edge does not claim refuses the
    /// survivor search**, naming both, rather than reading the edge's
    /// plus half as its mate and asking about the wrong face.
    #[test]
    fn an_unclaimed_ring_half_edge_refuses_the_survivor_search() {
        let tol = Tol::witness();
        let (mut body, top, membrane) = cube_with_membrane(tol);
        let ring = body.get_face(top).expect("live").rings[0];
        let crate::entity::LoopBoundary::Cycle { first } =
            body.get_loop(ring).expect("live").boundary
        else {
            panic!("the ring is a cycle")
        };
        let own = body.get_half_edge(first).expect("live").edge;
        let other = body
            .edges()
            .map(|(k, _)| k)
            .find(|&k| k != own)
            .expect("another edge");
        body.half_edges.get_mut(first).expect("live").edge = other;
        assert_eq!(
            body.outermost_survivor(top, vec![membrane]),
            Err(MergeCoplanarError::Op {
                error: EulerOpError::UnclaimedHalfEdge {
                    he: first,
                    edge: other,
                },
            })
        );
    }

    /// **`kef` reports [`EulerOpError::SameFace`] on a body that was
    /// never torn.** With the survivor the face INSIDE the ring, the
    /// absorption's own drain re-homes the dying half-edge's parent
    /// loop onto the survivor, and the `kef` that follows reads one
    /// face on both sides.
    ///
    /// This is why the fact the scan reads is not the fact the call
    /// can be held to: `SameFace` takes the enum's verdict and stays
    /// an inventory refusal.
    #[test]
    fn kef_reports_same_face_on_an_untorn_nested_group() {
        let tol = Tol::witness();
        let (mut body, top, membrane) = cube_with_membrane(tol);
        describe_shared_key(&mut body);
        assert_eq!(
            body.merge_group(membrane, &[top], MergeKind::Plane, tol),
            Err(MergeCoplanarError::Op {
                error: EulerOpError::SameFace { face: membrane },
            }),
        );
        assert!(
            !MergeCoplanarError::Op {
                error: EulerOpError::SameFace { face: membrane },
            }
            .is_arena_fault(),
            "an inventory refusal on an untorn body must not escape the regime"
        );
    }

    /// **A face plugging a hole of a coplanar neighbour merges into
    /// it, whichever of the two is arena-first.** The nested fixture
    /// with the inner face (the membrane) arena-FIRST, licensed by
    /// declared pairs alone. Kept as the survivor, the membrane would
    /// receive the top face's ring in the absorption's drain and leave
    /// `kef` one face on both sides (`SameFace`,
    /// `kef_reports_same_face_on_an_untorn_nested_group`); the door
    /// keeps the OUTER face instead, absorbs the membrane across its
    /// rim, and the hole's rim — a doubled cycle — is cut off as a ring
    /// and pruned away. One face, no ring, tier 2 green.
    #[test]
    fn a_face_plugging_a_neighbours_hole_merges_into_it() {
        let tol = Tol::witness();
        let (mut body, membrane) = cube_with_arena_first_membrane(tol);
        let first = body.faces().map(|(k, _)| k).next().expect("faces");
        assert_eq!(first, membrane, "the plug is arena-first");
        // Every face on its own plane key; ONE pair declared: the plug
        // against the face whose ring it fills.
        let host = body
            .faces()
            .find(|(_, f)| !f.rings.is_empty())
            .map(|(k, _)| k)
            .expect("a face carries the ring");
        let mut declared = declare_planes_pairwise(&mut body);
        let host_key = surface_of(&body, host);
        declared.retain(|&(_, k)| k == host_key);
        assert_eq!(declared.len(), 1);
        assert_eq!(validate_closed(&body), Ok(()));
        let outcome = body
            .merge_coplanar_faces_declared(&declared, tol)
            .expect("the plug merges into the face it plugs");
        let [group] = &outcome.groups[..] else {
            panic!("one group: {:?}", outcome.groups)
        };
        assert_eq!(group.kept, host, "the outer face survives");
        assert_eq!(group.absorbed, vec![membrane]);
        assert!(group.absorbed.contains(&membrane), "{group:?}");
        assert!(outcome.skipped.is_empty(), "{:?}", outcome.skipped);
        assert!(
            body.get_face(group.kept).expect("live").rings.is_empty(),
            "the hole is gone"
        );
        assert_eq!(validate_closed(&body), Ok(()));
    }

    /// The membrane fixture with the inner face arena-first: a `kef`
    /// before the rim is grown frees the seed face's slot, which the
    /// membrane's `add_face` then reuses.
    fn cube_with_arena_first_membrane(tol: Tol) -> (Body<f64>, FaceKey) {
        let cube = declined_cube::<f64>(tol);
        let mut body = cube.body;
        let seed_face = cube.seed.face;
        let victim = body
            .edges()
            .find_map(|(_, e)| {
                let (hp, hm) = body.edge_halves(e.he_plus, e.he_minus).ok()?;
                if hp.face == seed_face && hm.face != seed_face {
                    Some(e.he_plus)
                } else if hm.face == seed_face && hp.face != seed_face {
                    Some(e.he_minus)
                } else {
                    None
                }
            })
            .expect("the seed face has a neighbour");
        body.kef(victim).expect("the seed face is absorbed");
        let host = body
            .faces()
            .map(|(k, _)| k)
            .find(|&k| {
                let outer = body.get_face(k).expect("live").outer;
                body.get_loop(outer).expect("live").face == k && k != cube.mefs[0].face
            })
            .expect("a live face to host the ring");
        let host_he = {
            let outer = body.get_face(host).expect("live").outer;
            let crate::entity::LoopBoundary::Cycle { first } =
                body.get_loop(outer).expect("live").boundary
            else {
                panic!("an outer loop is a cycle")
            };
            first
        };
        let membrane = plant_ring_face(
            &mut body,
            host_he,
            &[
                geom_core::Point3::new(0.25, 0.25, 1.0),
                geom_core::Point3::new(0.75, 0.25, 1.0),
                geom_core::Point3::new(0.75, 0.75, 1.0),
                geom_core::Point3::new(0.25, 0.75, 1.0),
            ],
            tol,
        )
        .membrane;
        (body, membrane.face)
    }

    /// **The door's placement is exhaustive, and no arm of it
    /// contradicts the enum.**
    ///
    /// Over the crate's shared sample array
    /// ([`crate::euler::every_euler_op_error_once`], which carries
    /// the coverage and discriminant-order assertions), so a variant
    /// added without a placement fails by name. The direction pinned
    /// is the one the door owes the operator layer: a variant the
    /// door CONTRADICTS is one the enum answers `false` for —
    /// otherwise the arm is dead and the door's own question has no
    /// content — and every contradicted variant escapes while the
    /// delegated ones answer exactly as the enum does.
    #[test]
    fn the_doors_placement_is_exhaustive_and_agrees_with_the_enum() {
        for error in crate::euler::every_euler_op_error_once() {
            let escapes = MergeCoplanarError::Op {
                error: error.clone(),
            }
            .is_arena_fault();
            match OpPlacement::of(&error) {
                OpPlacement::Contradicts(fact) => {
                    assert!(
                        !error.reports_tier1_corruption(),
                        "{error} is the enum's already; the door's arm would be dead"
                    );
                    assert!(!fact.what().is_empty());
                    assert!(escapes, "{error}");
                }
                OpPlacement::TheEnumsVerdict => {
                    assert_eq!(escapes, error.reports_tier1_corruption(), "{error}");
                }
            }
        }
    }

    /// **The regime split, wired.** `records` is the whole of it and
    /// has one production call site, so this reds if the arena-fault
    /// conjunct is deleted from it — which pinning the classifier
    /// alone did not.
    ///
    /// No INPUT can pin this behaviourally — the door's tier-2 entry
    /// gate refuses a torn body before any group is staged, so no
    /// valid input reaches a corruption refusal under either regime.
    /// The behavioural pins go through the door's own tear points
    /// instead ([`TearPoint`],
    /// `every_contradicted_fact_escapes_the_recording_regime`); this
    /// row states that the regime applies the rule, over a variant
    /// the operator layer's own line places.
    #[test]
    fn only_inventory_refusals_are_ever_recorded() {
        let arena = MergeCoplanarError::Op {
            error: EulerOpError::OrbitBroken {
                he: crate::entity::HalfEdgeKey::default(),
            },
        };
        let inventory = MergeCoplanarError::GroupNotClosed { errors: Vec::new() };
        assert!(
            GroupRegime::RecordsASkip.records(&inventory),
            "the recording regime records an inventory refusal"
        );
        assert!(
            !GroupRegime::RecordsASkip.records(&arena),
            "a torn arena refuses the call even where the group would record"
        );
        assert!(
            !GroupRegime::RefusesTheCall.records(&inventory),
            "the refusing regime records nothing"
        );
        assert!(!GroupRegime::RefusesTheCall.records(&arena));
    }

    /// **The corruption class is the operator layer's, and it is
    /// nine variants wide, not two — and the door's is wider still.**
    /// The door's docs promise that a refusal reporting a torn arena
    /// never becomes a record; this pins the membership that promise
    /// needs, at the sample the merge can actually raise, in both of
    /// its halves.
    ///
    /// The second half is what the enum cannot answer: `FaceHasRings`
    /// and `SameFace` are legal facts about an operation and the enum
    /// says so, here and below — but a `kef` at THIS door raises them
    /// only against a fact the absorption established a few lines
    /// earlier, so they report the arena and escape. The two
    /// assertions on each are the two questions, and they no longer
    /// have one answer.
    #[test]
    fn the_arena_fault_class_is_the_operator_layers_tier_one_row() {
        let he = crate::entity::HalfEdgeKey::default();
        let torn = [
            EulerOpError::StaleKey {
                key: EntityId::Face(FaceKey::default()),
            },
            EulerOpError::StaleGeometry {
                key: GeomRef::Surface(SurfaceKey::default()),
            },
            EulerOpError::OrbitBroken { he },
            EulerOpError::LoopCycleBroken {
                r#loop: LoopKey::default(),
            },
            EulerOpError::UnclaimedHalfEdge {
                he,
                edge: EdgeKey::default(),
            },
        ];
        for error in torn {
            assert!(
                error.reports_tier1_corruption(),
                "{error} is tier-1-invalid input by its own docs"
            );
            assert!(MergeCoplanarError::Op { error }.is_arena_fault());
        }
        // Facts about the operation elsewhere, legal to meet on a
        // valid body — and at this door, refusals that contradict a
        // fact the absorption established before the call.
        let contradicted = [
            EulerOpError::FaceHasRings {
                face: FaceKey::default(),
            },
            EulerOpError::SameLoop {
                r#loop: LoopKey::default(),
            },
            EulerOpError::NotSameLoop { he1: he, he2: he },
            EulerOpError::SelfLoopEdge {
                edge: EdgeKey::default(),
                vertex: VertexKey::default(),
            },
            EulerOpError::RingIsOuter {
                r#loop: LoopKey::default(),
            },
            EulerOpError::CrossShell {
                f1: FaceKey::default(),
                f2: FaceKey::default(),
            },
        ];
        for error in contradicted {
            assert!(
                !error.reports_tier1_corruption(),
                "{error} is a legal fact about the operation, and the enum's line is \
                 unchanged by this door's question"
            );
            assert!(
                MergeCoplanarError::Op {
                    error: error.clone()
                }
                .is_arena_fault(),
                "{error} contradicts a fact merge_group established, so it escapes"
            );
            assert!(matches!(
                OpPlacement::of(&error),
                OpPlacement::Contradicts(_)
            ));
        }
        // `SameFace` is on neither list. The enum answers `false`
        // for it and this door agrees, because the absorption's own
        // drain can re-home the dying loop onto the survivor and
        // leave `kef` reading one face on both sides — legally, on a
        // body that was never torn
        // (`kef_reports_same_face_on_an_untorn_nested_group`).
        assert!(
            !MergeCoplanarError::Op {
                error: EulerOpError::SameFace {
                    face: FaceKey::default()
                }
            }
            .is_arena_fault()
        );
        // A merge-local refusal is never an arena fault.
        assert!(!MergeCoplanarError::GroupNotClosed { errors: Vec::new() }.is_arena_fault());
        assert!(
            !MergeCoplanarError::PeriodClosure {
                edge: EdgeKey::default()
            }
            .is_arena_fault()
        );
    }

    /// **`edge_halves` announces a torn link; it does not skip the
    /// edge.** Reds against the `else { continue }` it replaced: with
    /// the discard, the poisoned edge is passed over, the absorption
    /// finds nothing, and the group merges nothing while reporting
    /// `Ok`.
    #[test]
    fn a_torn_parent_loop_link_refuses_rather_than_skipping_the_edge() {
        let tol = Tol::witness();
        let mut body = structural_planar_cube(tol);
        let (rep, other, he_plus) = body
            .edges()
            .find_map(|(_, e)| {
                let (hp, hm) = body.edge_halves(e.he_plus, e.he_minus).ok()?;
                (hp.face != hm.face).then_some((hp.face, hm.face, e.he_plus))
            })
            .expect("a cube has adjacent faces");
        body.half_edges
            .get_mut(he_plus)
            .expect("the shared edge's plus half is live")
            .parent_loop = LoopKey::default();

        assert_eq!(
            body.merge_group(rep, &[other], MergeKind::Plane, tol),
            Err(MergeCoplanarError::Op {
                error: EulerOpError::StaleKey {
                    key: EntityId::Loop(LoopKey::default()),
                },
            }),
            "a torn parent-loop link is announced, never passed over"
        );
    }

    /// **A broken orbit is announced, not read as "no tip".** Reds
    /// against the `is_some_and` it replaced, which answered `false`
    /// — routing the pruning from `kev` to `kemr` and changing the
    /// group's Euler delta on a torn arena.
    ///
    /// It pins `strut_tip` rather than the surgery: reaching the tip
    /// search with a broken orbit needs the arena torn BETWEEN the
    /// absorption and the free-end test, and the surgery
    /// offers no tear point there — its points sit at the operator
    /// calls, which is where a contradicted fact is decided.
    /// `strut_tip` is that decision and has one production call site.
    #[test]
    fn a_broken_vertex_orbit_refuses_rather_than_answering_no_tip() {
        let tol = Tol::witness();
        let mut body = declined_cube::<f64>(tol).body;
        let he = body
            .edges()
            .map(|(_, e)| e.he_plus)
            .next()
            .expect("a cube has edges");
        assert_eq!(
            body.strut_tip(he),
            Ok(false),
            "an intact cube's half-edge has a two-member orbit"
        );

        // Tear the edge back-pointer so the orbit's `mate` step
        // cannot resolve; the half-edge itself stays live.
        body.half_edges
            .get_mut(he)
            .expect("the half-edge is live")
            .edge = EdgeKey::default();
        assert_eq!(
            body.strut_tip(he),
            Err(EulerOpError::OrbitBroken { he }),
            "a broken orbit is a refusal, not a `false`"
        );
    }

    /// **The kind question is asked of every member.** A group whose
    /// members are not all of one surface kind has no regime, and the
    /// refusal names both sides rather than letting arena order pick
    /// the contract.
    ///
    /// The mixed group is built at the arena, because the door that
    /// mints one — the same-source rung gluing a plane to a cylinder
    /// on a stamped `GeomSource` — needs a curved body this crate has
    /// no fixture for; `sweep`'s cosurface suite carries that half.
    #[test]
    fn a_group_that_straddles_two_surface_kinds_has_no_regime() {
        let tol = Tol::witness();
        let mut body = declined_cube::<f64>(tol).body;
        let (rep, other) = adjacent_pair(&body);
        // Both surfaces are set here: the fixture's faces carry the
        // mvfs placeholder, and the row is about KINDS, not about
        // which kind a fixture happens to leave behind.
        let plane = body.add_surface(Surface::Plane {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        });
        let cylinder = body.add_surface(Surface::Cylinder {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        });
        body.faces
            .get_mut(rep)
            .expect("the pair's first face is live")
            .surface = plane;
        body.faces
            .get_mut(other)
            .expect("the pair's second face is live")
            .surface = cylinder;

        let split = Err(MergeCoplanarError::GroupKindSplit {
            face: rep,
            kind: MergeKind::Plane,
            other,
            other_kind: MergeKind::Curved,
        });
        let contract = |body: &Body<f64>, a: FaceKey, b: FaceKey| {
            let kinds = body
                .kind_census()
                .expect("the fixture's faces resolve")
                .kinds;
            Body::<f64>::group_contract(a, &[b], &kinds)
        };
        assert_eq!(contract(&body, rep, other), split);
        // Both orders answer, and both name the same two faces the
        // same way.
        assert_eq!(contract(&body, other, rep), split);

        // The placeholder is the third kind, and a group holding one
        // beside a described face of EITHER kind has no contract.
        let placeholder = body.add_surface(Surface::nurbs_placeholder());
        body.faces
            .get_mut(other)
            .expect("the pair's second face is live")
            .surface = placeholder;
        let split = Err(MergeCoplanarError::GroupKindSplit {
            face: rep,
            kind: MergeKind::Plane,
            other,
            other_kind: MergeKind::Placeholder,
        });
        assert_eq!(contract(&body, rep, other), split);
        assert_eq!(contract(&body, other, rep), split);
        body.faces
            .get_mut(rep)
            .expect("the pair's first face is live")
            .surface = cylinder;
        let split = Err(MergeCoplanarError::GroupKindSplit {
            face: rep,
            kind: MergeKind::Curved,
            other,
            other_kind: MergeKind::Placeholder,
        });
        assert_eq!(contract(&body, rep, other), split);
        assert_eq!(contract(&body, other, rep), split);
    }

    /// **A run of placeholders is set aside, not run.** The contract
    /// of [`crate::test_support_fixtures::declined_cube`]'s six faces on their one placeholder key is
    /// [`GroupContract::SetAside`] — no regime, because a placeholder
    /// is neither of the two kinds the regimes are written for — and
    /// the door names the faces.
    #[test]
    fn a_placeholder_run_has_no_regime_and_is_set_aside() {
        let tol = Tol::witness();
        let body = declined_cube::<f64>(tol).body;
        assert_eq!(contract_of(&body), GroupContract::SetAside);
    }

    /// The unit cylinder about `z`.
    fn unit_cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    /// A bilinear net over the unit square with every `x` poisoned:
    /// [`NetState::Poisoned`], described geometry that cannot
    /// evaluate, and not the placeholder.
    fn poisoned_net() -> Surface<f64> {
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
            .expect("a clamped linear knot vector");
        let net = geom::NurbsSurface::new(
            kv.clone(),
            kv,
            (0..4)
                .map(|i| geom_core::Point3::new(f64::NAN, f64::from(i % 2), f64::from(i / 2)))
                .collect(),
            vec![1.0; 4],
        )
        .expect("four control points on a 2x2 net");
        assert_eq!(net.net_state(), NetState::Poisoned);
        Surface::Nurbs(std::sync::Arc::new(net))
    }

    /// **The answers of [`MergeKind::of`] at the values that decide
    /// them** — a plane, an analytic curved surface, the placeholder,
    /// and the arm that could be got wrong either way: a POISONED net
    /// has no kind, and through [`Body::merge_kind`] that is the typed
    /// refusal naming the face — never the placeholder's benign answer
    /// and never the curved arm.
    #[test]
    fn merge_kind_answers_each_surface_state() {
        let tol = Tol::witness();
        assert_eq!(MergeKind::of(&flat_plane()), Ok(MergeKind::Plane));
        assert_eq!(MergeKind::of(&unit_cylinder()), Ok(MergeKind::Curved));
        assert_eq!(
            MergeKind::of(&Surface::<f64>::nurbs_placeholder()),
            Ok(MergeKind::Placeholder)
        );
        assert_eq!(MergeKind::of(&poisoned_net()), Err(PoisonedNet));
        let mut body = declined_cube::<f64>(tol).body;
        let face = body.faces().next().expect("a cube has faces").0;
        body.set_face_surface(
            face,
            crate::euler::FaceSurface::New {
                surface: poisoned_net(),
                sense: true,
            },
        )
        .expect("a live face takes a surface");
        assert_eq!(
            body.merge_kind(face),
            Err(MergeCoplanarError::PoisonedSurfaceDescription { face })
        );
        assert!(
            MergeKind::Plane < MergeKind::Curved && MergeKind::Curved < MergeKind::Placeholder,
            "the refusal's naming order"
        );
    }

    /// **A poisoned net refuses the call before any surgery.** Two
    /// adjacent faces of the cube share one key carrying a net in
    /// [`NetState::Poisoned`]; the other four are placeholders. The
    /// door refuses [`MergeCoplanarError::PoisonedSurfaceDescription`]
    /// naming the arena-first poisoned face and leaves the body
    /// byte-identical — the placeholders are not set aside into an
    /// `Ok`, because the call never gets past its census.
    ///
    /// Reds against the described arm: the pair groups as a curved
    /// same-key run, `kef` kills their shared edge on the trial, the
    /// tier-2 gate passes (it reads no surface), and the door returns
    /// `Ok` with the group committed — surgery over a description
    /// tier 3 refuses at rest.
    #[test]
    fn a_poisoned_net_refuses_before_any_surgery() {
        let tol = Tol::witness();
        let mut body = declined_cube::<f64>(tol).body;
        let (first, second) = adjacent_pair(&body);
        body.set_face_surface(
            first,
            crate::euler::FaceSurface::New {
                surface: poisoned_net(),
                sense: true,
            },
        )
        .expect("a live face takes a surface");
        body.set_face_surface(
            second,
            crate::euler::FaceSurface::Shared {
                key: surface_of(&body, first),
                sense: true,
            },
        )
        .expect("a live face takes a shared key");
        let named = body
            .faces()
            .map(|(k, _)| k)
            .find(|&k| k == first || k == second)
            .expect("the pair is live");
        let before = crate::fixtures::deep_snapshot(&body);
        assert_eq!(
            body.merge_coplanar_faces(tol),
            Err(MergeCoplanarError::PoisonedSurfaceDescription { face: named })
        );
        assert_eq!(crate::fixtures::deep_snapshot(&body), before);
    }

    /// [`crate::test_support_fixtures::declined_cube`] with its one shared key re-described as a
    /// cylinder: one curved same-key run over the whole cube.
    fn curved_same_key_cube(tol: Tol) -> Body<f64> {
        let mut body = declined_cube::<f64>(tol).body;
        describe_shared_key_curved(&mut body);
        body
    }

    /// **The surgery re-checks the kind it is told — the truthful
    /// call.** A curved same-key cube told `Curved` refuses
    /// `PeriodClosure`: the absorption closes the cylinder's full
    /// period on the survivor.
    #[test]
    fn a_curved_run_told_its_own_kind_refuses_the_period_closure() {
        let tol = Tol::witness();
        let mut body = curved_same_key_cube(tol);
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        assert!(matches!(
            body.merge_group(faces[0], &faces[1..], MergeKind::Curved, tol),
            Err(MergeCoplanarError::PeriodClosure { .. })
        ));
    }

    /// **Told "planar", the same cube is caught before any mutation.**
    /// Reds against a surgery that believes its caller: the curved
    /// cube then MERGES — five faces absorbed, seven rings minted —
    /// where the truthful call refuses, and nothing downstream of the
    /// surgery notices.
    #[test]
    #[should_panic(expected = "merge_group: the survivor's kind is the contract's")]
    fn a_curved_run_told_planar_is_caught_before_any_surgery() {
        let tol = Tol::witness();
        let mut body = curved_same_key_cube(tol);
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let _ = body.merge_group(faces[0], &faces[1..], MergeKind::Plane, tol);
    }

    /// **Told "curved", a planar nested group is caught the same
    /// way.** On this fixture the lie would change no outcome — the
    /// absorption runs first and refuses `SameFace` — and the re-check
    /// does not depend on that.
    #[test]
    #[should_panic(expected = "merge_group: the survivor's kind is the contract's")]
    fn a_planar_group_told_curved_is_caught_before_any_surgery() {
        let tol = Tol::witness();
        let (mut body, top, membrane) = cube_with_membrane(tol);
        describe_shared_key(&mut body);
        let _ = body.merge_group(membrane, &[top], MergeKind::Curved, tol);
    }

    /// **An `Ok` carries a recorded skip and the placeholder census
    /// together.** The curved same-key cube with one face put back on
    /// a placeholder: the five curved faces record their
    /// `PeriodClosure` skip, the placeholder is named, and neither
    /// record swallows the other.
    #[test]
    fn an_ok_carries_a_recorded_skip_beside_the_placeholder_census() {
        let tol = Tol::witness();
        let mut body = curved_same_key_cube(tol);
        let side = body
            .faces()
            .map(|(k, _)| k)
            .last()
            .expect("a cube has faces");
        body.set_face_surface(
            side,
            crate::euler::FaceSurface::New {
                surface: Surface::nurbs_placeholder(),
                sense: true,
            },
        )
        .expect("a live face takes a surface");
        let outcome = body
            .merge_coplanar_faces(tol)
            .expect("a recorded skip beside a set-aside face is not a refusal");
        assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
        let [skipped] = &outcome.skipped[..] else {
            panic!("one recorded skip: {:?}", outcome.skipped)
        };
        assert!(
            matches!(skipped.reason, MergeCoplanarError::PeriodClosure { .. }),
            "{:?}",
            skipped.reason
        );
        assert_eq!(skipped.faces.len(), 5, "{:?}", skipped.faces);
        assert!(!skipped.faces.contains(&side), "{:?}", skipped.faces);
        assert_eq!(outcome.placeholders, vec![side]);
    }

    /// [`crate::test_support_fixtures::declined_cube`] with its arena-first face re-described as a real
    /// plane on its OWN key, the other five still on the shared
    /// placeholder.
    fn cube_with_one_described_face(tol: Tol) -> (Body<f64>, FaceKey) {
        let mut body = declined_cube::<f64>(tol).body;
        let face = body.faces().next().expect("a cube has faces").0;
        body.set_face_surface(
            face,
            crate::euler::FaceSurface::New {
                surface: flat_plane(),
                sense: true,
            },
        )
        .expect("a live face takes a surface");
        (body, face)
    }

    /// The surface key of one face.
    fn surface_of(body: &Body<f64>, face: FaceKey) -> SurfaceKey {
        body.get_face(face).expect("live").surface
    }

    /// **A described face beside placeholder neighbours is left
    /// alone, and the neighbours are set aside.** Nothing joins the
    /// plane to the placeholders — a different key, no source, no
    /// declaration — so the door returns `Ok` with no group and no
    /// skip, names exactly the five placeholders in arena order, and
    /// leaves the body byte-identical. The mixed case's quiet side:
    /// nothing here claimed the two kinds were one surface.
    #[test]
    fn a_described_face_beside_placeholders_is_untouched_and_they_are_named() {
        let tol = Tol::witness();
        let (mut body, plane) = cube_with_one_described_face(tol);
        let expected: Vec<FaceKey> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|&k| k != plane)
            .collect();
        assert_eq!(expected.len(), 5);
        let before = crate::fixtures::deep_snapshot(&body);
        let outcome = body
            .merge_coplanar_faces(tol)
            .expect("a described face with nothing to glue is not a refusal");
        assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
        assert!(outcome.skipped.is_empty(), "{:?}", outcome.skipped);
        assert_eq!(outcome.placeholders, expected);
        assert_eq!(crate::fixtures::deep_snapshot(&body), before);
    }

    /// **A source stamp joining a placeholder to a described face
    /// refuses typed, naming both and their kinds.** The same-source
    /// rung is the one rung that can join the two — it glues on
    /// provenance and never asks the kind — and the stamp is the
    /// caller's claim that they are one recipe surface, which a
    /// placeholder cannot be with anything described. The door
    /// refuses rather than choosing which half of the claim to
    /// believe, and the body is untouched. The mixed case's loud
    /// side, on the cube. The stamp sits on the KEY: stamping the
    /// plane's key and the placeholder key joins the plane to all five
    /// placeholders, edge-neighbours or not, and the face picked below
    /// is only the handle for that key.
    ///
    /// Reds against a quiet set-aside: the plane would be left alone,
    /// the five placeholders named, and the call would return `Ok`
    /// over a stamp the door had silently overruled.
    #[test]
    fn a_source_stamp_joining_a_placeholder_to_a_plane_refuses_typed() {
        let tol = Tol::witness();
        let (mut body, plane) = cube_with_one_described_face(tol);
        let on_placeholder_key = body
            .faces()
            .map(|(k, _)| k)
            .find(|&k| k != plane)
            .expect("a face on the placeholder key");
        let source = crate::GeomSource::minted(7, 0);
        for face in [plane, on_placeholder_key] {
            let key = surface_of(&body, face);
            forge_source(&mut body, key, &source);
        }
        let before = crate::fixtures::deep_snapshot(&body);
        let Err(MergeCoplanarError::GroupKindSplit {
            face,
            kind,
            other,
            other_kind,
        }) = body.merge_coplanar_faces(tol)
        else {
            panic!("a source-joined placeholder refuses")
        };
        assert_eq!((face, kind), (plane, MergeKind::Plane));
        assert_eq!(other_kind, MergeKind::Placeholder);
        assert_ne!(other, plane);
        assert_eq!(
            body.get_face(other)
                .expect("the named member is live")
                .surface,
            surface_of(&body, on_placeholder_key),
            "the other member is on the placeholder key"
        );
        assert_eq!(crate::fixtures::deep_snapshot(&body), before);
    }

    /// Writes a recipe origin on `key` with no agreement check — the
    /// way the graft carries one in. The stamp door's assertion refuses
    /// a pair of unequal descriptions under one source.
    fn forge_source(body: &mut Body<f64>, key: crate::SurfaceKey, source: &crate::GeomSource) {
        body.surface_origins
            .insert(key, crate::GeomOrigin::Recipe(source.clone()));
    }

    /// The two-face digon pillow — two vertices, two chord edges —
    /// with its split face on a real plane and its seed face left on
    /// the placeholder, each on its own key: a placeholder cap.
    fn pillow_with_a_placeholder_cap(tol: Tol) -> (Body<f64>, FaceKey, FaceKey) {
        let mut body = Body::<f64>::new();
        let seed = body
            .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
            .expect("mvfs has no preconditions");
        let seg = body
            .mev_line(
                crate::euler::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                geom_core::Point3::new(1.0, 0.0, 0.0),
                tol,
            )
            .expect("the first edge grows");
        let split = body
            .mef_chord(
                crate::euler::MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                tol,
            )
            .expect("the chord closes a second face");
        body.set_face_surface(
            split.face,
            crate::euler::FaceSurface::New {
                surface: flat_plane(),
                sense: true,
            },
        )
        .expect("a live face takes a surface");
        (body, seed.face, split.face)
    }

    /// **The pillow with a placeholder cap, both ways.** Unjoined, the
    /// cap is named and the pillow is untouched; joined by one source
    /// stamp, the pair refuses with the cap named as the placeholder
    /// — and the seed face being arena-first does not put it first in
    /// the refusal, which names the pair in kind order.
    #[test]
    fn a_pillow_with_a_placeholder_cap_is_set_aside_unjoined_and_refused_joined() {
        let tol = Tol::witness();
        let (mut body, cap, plane) = pillow_with_a_placeholder_cap(tol);
        assert_eq!(validate_closed(&body), Ok(()));
        let before = crate::fixtures::deep_snapshot(&body);
        let outcome = body
            .merge_coplanar_faces(tol)
            .expect("an unjoined cap is not a refusal");
        assert!(outcome.groups.is_empty() && outcome.skipped.is_empty());
        assert_eq!(outcome.placeholders, vec![cap]);
        assert_eq!(crate::fixtures::deep_snapshot(&body), before);

        let source = crate::GeomSource::minted(11, 0);
        for face in [cap, plane] {
            let key = surface_of(&body, face);
            forge_source(&mut body, key, &source);
        }
        let before = crate::fixtures::deep_snapshot(&body);
        assert_eq!(
            body.merge_coplanar_faces(tol),
            Err(MergeCoplanarError::GroupKindSplit {
                face: plane,
                kind: MergeKind::Plane,
                other: cap,
                other_kind: MergeKind::Placeholder,
            })
        );
        assert_eq!(crate::fixtures::deep_snapshot(&body), before);
    }

    /// The crate's one executable pin on this door's rendered
    /// refusals: `DESIGN.md`'s D4 ¶2(ii) names them as the in-repo
    /// precedent for message-level recourse, so the recourse sentence
    /// is asserted rather than left to a reader.
    #[test]
    fn the_refusals_render_their_recourse() {
        let rendered = |e: &MergeCoplanarError| e.to_string();
        assert!(
            rendered(&MergeCoplanarError::ResultNotClosed { errors: Vec::new() })
                .contains("refused"),
            "the whole run's after-gate says the call is refused"
        );
        let group = rendered(&MergeCoplanarError::GroupNotClosed { errors: Vec::new() });
        assert!(
            group.contains("unmerged") && group.contains("run continues"),
            "a group's own gate says the run survives it: {group}"
        );
        let contradicted = rendered(&MergeCoplanarError::DeclarationContradicted {
            fact: crate::boolean::Contradiction::PlanesApart,
        });
        assert!(
            contradicted.contains("the declared planes are parallel but apart")
                && contradicted.ends_with(
                    "Recourse: correct or remove the declaration, or move the geometry so it holds"
                ),
            "the declared-pair contradiction names its fact and ends on its recourse: \
             {contradicted}"
        );
        let split = |other_kind| {
            rendered(&MergeCoplanarError::GroupKindSplit {
                face: FaceKey::default(),
                kind: MergeKind::Plane,
                other: FaceKey::default(),
                other_kind,
            })
        };
        let with_placeholder = split(MergeKind::Placeholder);
        assert!(
            with_placeholder.contains("(planar)")
                && with_placeholder.contains("(placeholder)")
                && with_placeholder.contains("re-check the shared surface source")
                && with_placeholder.contains("describes no locus"),
            "a split on a placeholder names both kinds and the placeholder's recourse: \
             {with_placeholder}"
        );
        let two_described = split(MergeKind::Curved);
        assert!(
            two_described.contains("(curved)")
                && two_described.contains("re-check the shared surface source")
                && !two_described.contains("describes no locus"),
            "a split between two described kinds carries no placeholder recourse: \
             {two_described}"
        );
        let poisoned = rendered(&MergeCoplanarError::PoisonedSurfaceDescription {
            face: FaceKey::default(),
        });
        assert!(
            poisoned.contains("poison") && poisoned.contains("Body::set_face_surface"),
            "the poisoned refusal names its recourse: {poisoned}"
        );
    }

    /// **The placeholder cube forms no merge group.** Every face of
    /// `declined_cube` carries the `mvfs` seed's surface on one key, and
    /// the structural rung reads one key as one surface — but that
    /// surface describes no locus, so there is nothing to glue: the
    /// door returns `Ok` with no group and no skip, names the six
    /// faces as placeholders, and leaves the body byte-identical.
    ///
    /// Reds against reading the placeholder as curved: the whole cube
    /// then groups as one curved run, the absorption kills five edges
    /// on the trial clone, the survivor's duplicates read as a period
    /// closure, and the door returns `Ok` with a `PeriodClosure` skip
    /// over six faces that were never a cosurface run.
    #[test]
    fn the_placeholder_cube_forms_no_group_and_its_faces_are_named() {
        let tol = Tol::witness();
        let mut body = declined_cube::<f64>(tol).body;
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let before = crate::fixtures::deep_snapshot(&body);
        let outcome = body
            .merge_coplanar_faces(tol)
            .expect("nothing to glue is not a refusal");
        assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
        assert!(
            outcome.skipped.is_empty(),
            "a placeholder run is not a curved run: {:?}",
            outcome.skipped
        );
        assert_eq!(outcome.placeholders, faces);
        assert_eq!(crate::fixtures::deep_snapshot(&body), before);
    }
}

/// **The winding arm's own rows** (`loop_winding`): the carriers the
/// functional answers about, decided directly rather than through the
/// merge that consumes them.
///
/// The fixtures are built through the euler doors as chord polygons and
/// then RETYPED onto their real carriers — the same two-step
/// `tier3_tests` uses — because the arm is about what a cycle's edges
/// carry, not about how the face was minted.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod winding_arm_tests {
    use super::*;
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use crate::loop_winding::RunClosing::{self, Straight};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
    use geom_core::{Point3, Sign, Vec3};

    /// The plane every fixture here lives on, and the outward normal
    /// every winding is asked about.
    fn plane() -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    /// A triangle `a → b → d` on the z = 0 plane, as ONE face's outer
    /// loop, with the `a → b` edge handed back so a row can retype it.
    /// The loop's stored order is `a → b → d`, so `a → b` is traversed
    /// by that edge's `he_plus`.
    struct Tri {
        body: Body<f64>,
        r#loop: LoopKey,
        surface: geom_brep::SurfaceKey,
        ab: crate::entity::EdgeKey,
    }

    fn tri(a: Point3<f64>, b: Point3<f64>, d: Point3<f64>, tol: Tol) -> Tri {
        let mut body = Body::<f64>::new();
        let (r#loop, surface, ab) = tri_into(&mut body, [a, b, d], tol);
        Tri {
            body,
            r#loop,
            surface,
            ab,
        }
    }

    /// [`tri`]'s triangle, added to `body` as a shell of its own:
    /// the `a → b → d` loop, its surface and the `a → b` edge.
    fn tri_into(
        body: &mut Body<f64>,
        [a, b, d]: [Point3<f64>; 3],
        tol: Tol,
    ) -> (LoopKey, geom_brep::SurfaceKey, crate::entity::EdgeKey) {
        let seed = body.mvfs(a, true).unwrap();
        let surface = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: plane(),
                    sense: true,
                },
            )
            .unwrap();
        let e_ab = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                b,
                tol,
            )
            .unwrap();
        let e_bd = body
            .mev_line(
                MevSite::Fan {
                    he1: e_ab.he_minus,
                    he2: e_ab.he_minus,
                },
                d,
                tol,
            )
            .unwrap();
        let new = body
            .mef_chord(
                MefSite::Chords {
                    he1: e_bd.he_minus,
                    he2: e_ab.he_plus,
                },
                tol,
            )
            .unwrap();
        body.set_face_surface(
            new.face,
            FaceSurface::New {
                surface: plane(),
                sense: true,
            },
        )
        .unwrap();
        let r#loop = body.get_face(seed.face).unwrap().outer;
        let ab = body_edge(body, e_ab.he_plus);
        (r#loop, surface, ab)
    }

    fn body_edge(body: &Body<f64>, he: crate::HalfEdgeKey) -> crate::entity::EdgeKey {
        body.get_half_edge(he).unwrap().edge
    }

    /// A winding's sign, its margin dropped.
    fn signed(
        r: Result<LoopWinding<Decided>, MergeCoplanarError>,
    ) -> Result<LoopWinding<Sign>, MergeCoplanarError> {
        r.map(|w| match w {
            LoopWinding::Empty => LoopWinding::Empty,
            LoopWinding::Unsupported => LoopWinding::Unsupported,
            LoopWinding::Wound(d) => LoopWinding::Wound(d.sign),
        })
    }

    fn band(tol: Tol) -> Band {
        Band::linear(tol).unwrap()
    }

    /// **The Line-only path, unchanged.** A chord triangle is decided
    /// by the chord Newell sum and nothing else: the arc correction
    /// block is structurally skipped for it, so every decision this
    /// site made before the conic arm existed it still makes, from the
    /// same arithmetic in the same order. This row is what a mutation
    /// inside that block must leave green.
    #[test]
    fn a_line_only_cycle_is_decided_by_the_chord_sum_alone() {
        let tol = Tol::witness();
        let t = tri(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            tol,
        );
        assert_eq!(
            signed(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol))),
            Ok(LoopWinding::Wound(Sign::Positive)),
            "the chord triangle winds positively about +z"
        );
        assert_eq!(
            signed(t.body.loop_winding(t.r#loop, -Vec3::unit_z(), band(tol))),
            Ok(LoopWinding::Wound(Sign::Negative)),
            "and negatively about the opposite normal"
        );
    }

    /// **The pure-arc pair**: a disc bounded by two semicircles, and
    /// the same boundary traversed the other way. Its CHORD Newell sum
    /// is exactly zero — the two vertices and the base point are
    /// collinear, so every cross product vanishes — which is the whole
    /// reason the bulge term exists: without it a two-semicircle 2-gon
    /// reads as no region at all. With it, one traversal is Positive
    /// about the outward normal and its counterpart Negative, which is
    /// exactly the outer-loop/ring statement the role pass reads.
    #[test]
    fn a_two_arc_cycle_winds_positively_and_its_counterpart_negatively() {
        let tol = Tol::witness();
        let (body, disc, anti) = two_semicircle_disc(tol);
        let outward = Vec3::unit_z();
        assert_eq!(
            signed(body.loop_winding(disc, outward, band(tol))),
            Ok(LoopWinding::Wound(Sign::Positive)),
            "the disc's own boundary encloses material about the outward normal"
        );
        assert_eq!(
            signed(body.loop_winding(anti, outward, band(tol))),
            Ok(LoopWinding::Wound(Sign::Negative)),
            "the same boundary reversed anti-encloses — a ring's signature"
        );
    }

    /// The unit disc on the z = 0 plane bounded by two semicircles
    /// meeting at `(±1, 0, 0)`: the disc face's own loop, which winds
    /// counterclockwise about `+z`, and the seed face's counterpart.
    fn two_semicircle_disc(tol: Tol) -> (Body<f64>, LoopKey, LoopKey) {
        let (a, b) = (Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0));
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(a, true).unwrap();
        let surface = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: plane(),
                    sense: true,
                },
            )
            .unwrap();
        let arc = |axis: Vec3<f64>| EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(surface),
            carrier: geom::Curve3::Circle {
                center: Point3::new(0.0, 0.0, 0.0),
                axis,
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: core::f64::consts::PI,
        };
        // Upper semicircle a → b, then the lower one, minted so that
        // BOTH run a → b under their own `he_plus` — the axis flip is
        // what turns the second one around, not a backwards parameter
        // range (the `he_plus` forward contract).
        let e1 = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                b,
                arc(Vec3::unit_z()),
                tol,
            )
            .unwrap();
        let e2 = body
            .mef(
                MefSite::Chords {
                    he1: e1.he_plus,
                    he2: e1.he_minus,
                },
                arc(-Vec3::unit_z()),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        let disc = body.get_face(e2.face).unwrap().outer;
        let anti = body.get_face(seed.face).unwrap().outer;
        (body, disc, anti)
    }

    /// **The mixed Line + Circle cycle**, and the reason the bulge is a
    /// CORRECTION rather than a case split: this triangle's chord
    /// polygon winds positively (`2A = 1`), and its `a → b` side is a
    /// quarter arc bulging INTO it, worth `−(π/2 − 1)`. The verdict is
    /// the sum — `+0.43` — and no arm of the code ever asks whether
    /// the cycle is "an arc cycle" or "a chord cycle".
    #[test]
    fn a_mixed_line_and_circle_cycle_decides_on_chord_plus_bulge() {
        let tol = Tol::witness();
        let mut t = tri(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            tol,
        );
        t.body
            .set_edge_curve(
                t.ab,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::chart(t.surface),
                    carrier: geom::Curve3::Circle {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: -Vec3::unit_z(),
                        radius: 1.0,
                        u_ref: Vec3::unit_x(),
                    },
                    param_start: -core::f64::consts::FRAC_PI_2,
                    param_end: 0.0,
                },
                tol,
            )
            .unwrap();
        assert_eq!(
            signed(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol))),
            Ok(LoopWinding::Wound(Sign::Positive)),
            "chord 2A = 1 minus the bite the arc takes out of it"
        );
    }

    /// The margin an escalation carries back, as an `f64`.
    ///
    /// **This is the module's one test-visible seam onto the deciding
    /// scalar.** `loop_winding` hands back a [`Sign`] and nothing else
    /// when it decides, so `2A/P` is unobservable from outside on the
    /// deciding path — but an IN-BAND margin escalates typed, and
    /// [`geom_core::MarginKind::Value`] then carries the exact quantity
    /// that was classified. The two rows below aim their fixtures into
    /// that band deliberately: it is the only place the numerator and
    /// the DENOMINATOR can both be pinned, and the denominator — the
    /// re-metered arc-length perimeter — is otherwise invisible to any
    /// sign assertion, because scaling a lever cannot change a sign.
    ///
    /// Each escalation is also the winding decision's real raise for the
    /// refusal-shape guard: one recourse, its subject, no stage prefix
    /// and no declaration.
    fn escalated_margin(r: Result<LoopWinding<Decided>, MergeCoplanarError>) -> f64 {
        match r {
            Err(
                ref err @ MergeCoplanarError::Escalated {
                    decision: MergeDecision::LoopWinding,
                    diag,
                },
            ) => {
                let text = err.to_string();
                assert_eq!(test_utils::refusal::recourse_markers(&text), 1, "{text}");
                assert!(
                    test_utils::refusal::subjectless_escalations(&text).is_empty()
                        && test_utils::refusal::stage_prefixes(&text, &[]).is_empty()
                        && text.starts_with(
                            "which way a loop of the merged face winds about its normal is \
                             undecided: "
                        )
                        && !text.contains("declare"),
                    "{text}"
                );
                match diag.margin.diagnostic_f64_for_error_text() {
                    geom_core::ErrorTextReading::Value(v) => v,
                    other => panic!("expected a classified f64 margin, got {other:?}"),
                }
            }
            other => panic!("expected an in-band escalation carrying its margin, got {other:?}"),
        }
    }

    /// The relative tolerance the two value-pinning rows assert to.
    ///
    /// Bounded from BELOW by the fixtures' arithmetic and from ABOVE by
    /// the mutations they must catch, and both bounds are stated
    /// because the gap is what makes the rows meaningful:
    ///
    /// - **Floor.** Each fixture's residue is a difference of two O(1)
    ///   quantities, so it carries a few ulps of `~0.5` — about
    ///   `1e-16` absolute. The residue itself is `√(ε·K·ε)·P`, which
    ///   at the TIGHTEST gated row (ε = 1e-12) is `~1.6e-11`, giving a
    ///   relative noise near `1e-5`.
    /// - **Ceiling.** The smallest mutation these rows must redden is
    ///   deleting the perimeter re-metering on the Circle fixture:
    ///   arc-metered `5.117` against chord-metered `4.961`, a **3.2%**
    ///   move.
    ///
    /// `1e-3` sits two orders above the floor and one and a half below
    /// the ceiling.
    const TOL_REL: f64 = 1e-3;

    /// The residue to place a fixture's verdict on, so that the row
    /// travels EVERY tolerance lane.
    ///
    /// The two rows below have to land their margin strictly inside the
    /// ambiguity band, because that is the only place the deciding
    /// scalar is observable. The band is `(ε, K·ε)` from the RUN's
    /// tolerance, and CI gates several ε rows — so a hard-coded
    /// residue that sits mid-band at ε = 1e-9 DECIDES at ε = 1e-12 and
    /// the row stops testing what it is for. The geometric mean
    /// `√(ε · K·ε)` is strictly inside `(ε, K·ε)` for every ε and
    /// every K > 1; scaled by the loop's perimeter it is the `2A` that
    /// puts the verdict there.
    ///
    /// `perimeter` is a nominal value — only good enough to aim with.
    /// Each row recomputes its EXACT perimeter afterwards and asserts
    /// against `residue / that`, so nothing here enters the pin.
    fn band_centre_residue(tol: Tol, perimeter: f64) -> f64 {
        let b = band(tol);
        (b.zero() * b.escalate()).sqrt() * perimeter
    }

    /// **The re-metering is the perimeter, and the perimeter is ARC
    /// LENGTH** — pinned on the value, not on a sign.
    ///
    /// `perimeter = metered` replaces the chord perimeter wholesale on
    /// a conic cycle. No sign assertion can see that: the lever is a
    /// positive divisor, so doubling it, halving it or deleting the
    /// replacement outright leaves every verdict's SIGN untouched. The
    /// pin therefore has to read the margin itself.
    ///
    /// The fixture: the `a → b` side is a unit quarter arc, whose
    /// bulge bites `π/2 − 1` out of the chord triangle; `d` is placed
    /// so the chord area exceeds that bite by exactly `delta` —
    /// [`band_centre_residue`], which aims the verdict at the middle of
    /// the ambiguity band for whatever ε the run carries. The whole
    /// verdict is then that residue over the loop's own perimeter,
    /// `delta / P`, and the value comes back because it escalated.
    /// [`TOL_REL`] states the arithmetic this survives.
    ///
    /// What it catches: `perimeter = metered` DELETED leaves the chord
    /// perimeter (√2 for the arc's side instead of π/2 — a 3% smaller
    /// lever, asserted below to be outside the tolerance), and the
    /// lever DOUBLED halves the margin. Both red.
    #[test]
    fn the_conic_perimeter_is_re_metered_to_arc_length() {
        let tol = Tol::witness();
        // 2A = (chord Newell) + (bulge) = (y + 1) + (1 − π/2).
        // This triangle's perimeter is ≈ 5.12; aiming with 5 is ample.
        let delta = band_centre_residue(tol, 5.0);
        let bulge_z = 1.0 - core::f64::consts::FRAC_PI_2; // R²(Δ − sin Δ), Δ = π/2
        let y = delta - 1.0 - bulge_z;
        let (a, b, d) = (
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(2.0, y, 0.0),
        );
        let mut t = tri(a, b, d, tol);
        t.body
            .set_edge_curve(
                t.ab,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::chart(t.surface),
                    carrier: geom::Curve3::Circle {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: -Vec3::unit_z(),
                        radius: 1.0,
                        u_ref: Vec3::unit_x(),
                    },
                    param_start: -core::f64::consts::FRAC_PI_2,
                    param_end: 0.0,
                },
                tol,
            )
            .unwrap();
        // The lever, stated as the arm states it: the conic edge
        // contributes |Δ|·R, every Line edge its chord.
        let sides = (d - b).norm() + (a - d).norm();
        let metered = core::f64::consts::FRAC_PI_2 + sides;
        let chorded = (b - a).norm() + sides;
        let expected = delta / metered;
        assert!(
            (expected - delta / chorded).abs() > TOL_REL * expected,
            "the pin has teeth only if the two levers differ by more than its tolerance"
        );
        let got = escalated_margin(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol)));
        assert!(
            (got - expected).abs() <= TOL_REL * expected,
            "the margin is the residue over the ARC-LENGTH perimeter: \
             got {got:e}, expected {expected:e} (chord-metered would be {:e})",
            delta / chorded
        );
    }

    /// **The Ellipse arm, and both of its constants.**
    ///
    /// The `Ellipse` match arm reads `(axis, major, minor)`, and that
    /// tuple is used TWICE with different meanings: `sa·sb` is the
    /// affine area factor of the bulge, and `sa` alone is the metering
    /// bound. Two swaps therefore go undetected by any sign row —
    /// `(axis, minor, major)` leaves `sa·sb` alone and silently turns
    /// the metering bound into a LOWER bound, and `(axis, major,
    /// major)` changes only the area. This row reds under both,
    /// because both scale the margin by exactly `major/minor` (the
    /// first through the denominator, the second through the
    /// numerator) and the margin itself is what is asserted.
    ///
    /// Same construction as the mixed row: a chord triangle with its
    /// `a → b` side retyped, `d` placed so the chord area exceeds the
    /// arc's bite by `delta` and the verdict lands in the band.
    #[test]
    fn an_ellipse_arc_uses_its_major_to_meter_and_both_semi_axes_for_area() {
        let tol = Tol::witness();
        const MAJOR: f64 = 1.0;
        const MINOR: f64 = 0.5;
        // This triangle's perimeter is ≈ 4.72; aiming with 5 is ample.
        let delta = band_centre_residue(tol, 5.0);
        // The arc runs a → b over Δ = π/2; its bulge is
        // `major·minor·(Δ − sin Δ)` about −ẑ, i.e. NEGATIVE about +ẑ.
        let bulge_z = MAJOR * MINOR * (1.0 - core::f64::consts::FRAC_PI_2);
        // 2A = (chord Newell) + (bulge) = (y + MINOR) + bulge_z.
        let y = delta - MINOR - bulge_z;
        // `a` sits at the ellipse's own semi-minor tip, `b` at its
        // semi-major tip — the two points the arc below runs between.
        let (a, b, d) = (
            Point3::new(0.0, MINOR, 0.0),
            Point3::new(MAJOR, 0.0, 0.0),
            Point3::new(2.0, y, 0.0),
        );
        let mut t = tri(a, b, d, tol);
        t.body
            .set_edge_curve(
                t.ab,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::chart(t.surface),
                    carrier: geom::Curve3::Ellipse {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: -Vec3::unit_z(),
                        major: MAJOR,
                        minor: MINOR,
                        u_ref: Vec3::unit_x(),
                    },
                    param_start: -core::f64::consts::FRAC_PI_2,
                    param_end: 0.0,
                },
                tol,
            )
            .unwrap();
        let sides = (d - b).norm() + (a - d).norm();
        let expected = delta / (core::f64::consts::FRAC_PI_2 * MAJOR + sides);
        // What the two swaps would produce, and the proof that this
        // row's tolerance separates them: BOTH scale the margin by
        // `major/minor`, so one assertion covers both.
        let swapped = expected * (MAJOR / MINOR);
        assert!(
            (swapped - expected).abs() > TOL_REL * expected,
            "a major/minor swap must move the margin further than the tolerance below"
        );
        let got = escalated_margin(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol)));
        assert!(
            (got - expected).abs() <= TOL_REL * expected,
            "the ellipse meters on `major` and takes its area from `major·minor`: \
             got {got:e}, expected {expected:e} (either swap gives {swapped:e})"
        );
    }

    /// Retypes [`tri`]'s `a → b` edge, for `a = (2,0)` and `b = (0,2)`,
    /// as the rational quadratic quarter circle between them: a NURBS
    /// carrier, whose winding the kernel does not read.
    fn fit_a_nurbs_quarter(t: &mut Tri, tol: Tol) {
        // The rational quadratic quarter circle `a → b`, described as
        // the plane chart's own image of itself: the chart is
        // `origin + u·x̂ + v·ŷ` here, so the 2-D control net is the
        // 3-D one with `z` dropped.
        let knots = || {
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
        };
        let weights = || vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0];
        t.body
            .set_edge_curve(
                t.ab,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::chart_image(
                        t.surface,
                        geom_brep::pcurve_cache::Pcurve::General(std::sync::Arc::new(
                            geom::NurbsCurve2::new(
                                knots(),
                                vec![
                                    geom_core::Point2::new(2.0, 0.0),
                                    geom_core::Point2::new(2.0, 2.0),
                                    geom_core::Point2::new(0.0, 2.0),
                                ],
                                weights(),
                            )
                            .unwrap(),
                        )),
                    ),
                    carrier: geom::Curve3::Nurbs(std::sync::Arc::new(
                        geom::NurbsCurve3::new(
                            knots(),
                            vec![
                                Point3::new(2.0, 0.0, 0.0),
                                Point3::new(2.0, 2.0, 0.0),
                                Point3::new(0.0, 2.0, 0.0),
                            ],
                            weights(),
                        )
                        .unwrap(),
                    )),
                    param_start: 0.0,
                    param_end: 1.0,
                },
                tol,
            )
            .unwrap();
    }

    /// **The honest remainder**: a cycle carrying a NURBS edge is not
    /// answered. No closed form exists for the region a fitted carrier
    /// bounds, and the chord winding is not that region's — so the
    /// question comes back `None` and the caller refuses rather than
    /// guesses. The chord polygon here winds positively, so `None` is
    /// a REFUSAL to read the chords, not an absence of chords to read.
    #[test]
    fn a_nurbs_carrying_cycle_stays_undecidable() {
        let tol = Tol::witness();
        let mut t = tri(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(-1.0, -1.0, 0.0),
            tol,
        );
        assert_eq!(
            signed(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol))),
            Ok(LoopWinding::Wound(Sign::Positive)),
            "as a chord triangle it is decidable and positive"
        );
        fit_a_nurbs_quarter(&mut t, tol);
        assert_eq!(
            signed(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol))),
            Ok(LoopWinding::Unsupported),
            "one fitted carrier and the whole cycle stops being answerable"
        );
        // ...and as a merged face's only loop it is named, not taken
        // for a hole: it may be the outline, and nothing built reads it.
        let face = t.body.get_loop(t.r#loop).unwrap().face;
        let survivor = t.body.get_face(face).unwrap().clone();
        let err = t
            .body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("an outline whose winding is not built is not assigned");
        assert_one_story(&err.to_string());
        assert_eq!(
            err,
            MergeCoplanarError::MergedFaceRoleAmbiguous {
                face,
                verdict: OutlineVerdict::UnsupportedWinding { r#loop: t.r#loop },
            }
        );
    }

    /// The refusal-shape guard every outline ending passes: one
    /// recourse, no stage prefix, no subjectless escalation, and no
    /// offer of a declaration the merge cannot take.
    fn assert_one_story(text: &str) {
        assert_eq!(test_utils::refusal::recourse_markers(text), 1, "{text}");
        assert!(
            test_utils::refusal::subjectless_escalations(text).is_empty()
                && test_utils::refusal::stage_prefixes(text, &[]).is_empty()
                && !text.contains("declare"),
            "{text}"
        );
    }

    /// **A zero winding that leaves the merged face no outline tells
    /// the winding decision's in-band story**, with the tolerance its
    /// own margin gives.
    ///
    /// The fixture: one triangle `(0,0) → (2,0) → (1,h)`, the merged
    /// face's only loop, with `h` the band's zero threshold. Its
    /// margin `2A/P = 2h/P ≈ h/2` is nonzero and lies within the zero
    /// band at every tolerance row, so the winding classifies zero and
    /// no loop is an outline. [`LOOP_WINDING`] passes on either
    /// nonzero sign, so a smaller tolerance decides this margin: the
    /// ending offers `m/K`, read here off the loop's own geometry.
    #[test]
    fn a_zero_winding_that_leaves_no_outline_tells_the_winding_story() {
        let tol = Tol::witness();
        let band = band(tol);
        let h = band.zero();
        let t = tri(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(1.0, h, 0.0),
            tol,
        );
        let face = t.body.get_loop(t.r#loop).unwrap().face;
        let survivor = t.body.get_face(face).unwrap().clone();
        let text = t
            .body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("a face whose one loop winds zero has no outline")
            .to_string();
        assert_one_story(&text);
        assert!(
            text.starts_with(
                "which way a loop of the merged face winds about its normal is undecided: \
                 margin "
            ) && text.contains("lies within the zero band")
                && text.contains(LOOP_WINDING.lever),
            "{text}"
        );
        let margin = 2.0 * h / (2.0 + 2.0 * (1.0 + h * h).sqrt());
        let want = margin / (band.escalate() / band.zero());
        let offered: f64 = text
            .split_once("tighten the tolerance below ")
            .and_then(|(_, tail)| tail.strip_suffix(" m"))
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| panic!("a valued tolerance offer ends the text: {text}"));
        assert!(
            (offered - want).abs() <= 1e-9 * want,
            "the offer is the zero winding's own margin over K: offered {offered:e}, \
             want {want:e}: {text}"
        );
    }

    /// **Several positive windings end in their own story**: two
    /// counterclockwise triangles as one merged face's outline and
    /// ring. Each bounds a region of its own, so the merge would make
    /// one face of two separate regions; the refusal names both loops
    /// and the move that makes the union one region.
    #[test]
    fn several_positive_windings_name_their_loops_and_the_move() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let (first, _, _) = tri_into(
            &mut body,
            [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            tol,
        );
        let (second, _, _) = tri_into(
            &mut body,
            [
                Point3::new(3.0, 0.0, 0.0),
                Point3::new(4.0, 0.0, 0.0),
                Point3::new(3.0, 1.0, 0.0),
            ],
            tol,
        );
        let face = body.get_loop(first).unwrap().face;
        let mut survivor = body.get_face(face).unwrap().clone();
        survivor.rings = vec![second];
        let text = body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("two outlines are not one face's")
            .to_string();
        assert_one_story(&text);
        assert!(
            text.contains(&format!("{first:?}"))
                && text.contains(&format!("{second:?}"))
                && text.contains(
                    "Recourse: reshape the merged faces so their union is one connected region"
                ),
            "{text}"
        );
    }

    /// **Every loop negative ends in the orientation story**: the
    /// triangle's second face, whose one loop winds clockwise about
    /// its outward normal, taken as a merged face. Nothing about the
    /// margin is band-decided, so no tolerance is offered.
    #[test]
    fn every_loop_negative_names_the_face_and_offers_no_tolerance() {
        let tol = Tol::witness();
        let t = tri(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            tol,
        );
        let (face, survivor) = t
            .body
            .faces()
            .find(|(_, f)| f.outer != t.r#loop)
            .map(|(k, f)| (k, f.clone()))
            .expect("the triangle's second face");
        let err = t
            .body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("a face of holes has no outline");
        let text = err.to_string();
        assert_one_story(&text);
        assert!(!text.contains("tighten"), "{text}");
        assert_eq!(
            err,
            MergeCoplanarError::MergedFaceRoleAmbiguous {
                face,
                verdict: OutlineVerdict::AllNegative,
            }
        );
    }

    /// The zero-winding triangle `(0,0) → (2,0) → (1,h)`, `h` the
    /// band's zero threshold, shifted by `dx` and added to `body`: its
    /// margin `2A/P ≈ h/2` lies within the zero band.
    fn thin_into(body: &mut Body<f64>, dx: f64, tol: Tol) -> LoopKey {
        let h = band(tol).zero();
        tri_into(
            body,
            [
                Point3::new(dx, 0.0, 0.0),
                Point3::new(dx + 2.0, 0.0, 0.0),
                Point3::new(dx + 1.0, h, 0.0),
            ],
            tol,
        )
        .0
    }

    /// Whether `err` is the winding decision's zero-band refusal.
    fn is_the_zero_story(err: &MergeCoplanarError) -> bool {
        matches!(
            err,
            MergeCoplanarError::Escalated {
                decision: MergeDecision::LoopWinding,
                diag,
            } if diag.margin.kind() == geom_core::MarginKind::Value
        ) && err.to_string().contains("lies within the zero band")
    }

    /// **With no loop positive, a zero winding is named before a loop
    /// the kernel does not wind.** Such a loop already counts as not
    /// positive (beside one positive loop the merge takes it for a
    /// hole and proceeds), so the answer here turns on the zero
    /// winding, which a tighter tolerance decides; the refusal offers
    /// that tolerance rather than the carriers. The outline rides a
    /// NURBS edge and the ring is the zero triangle. (Adopted from the
    /// review's `probe_winding.rs`.)
    #[test]
    fn a_zero_winding_is_named_before_a_loop_the_kernel_does_not_wind() {
        let tol = Tol::witness();
        let mut t = tri(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(-1.0, -1.0, 0.0),
            tol,
        );
        fit_a_nurbs_quarter(&mut t, tol);
        let thin = thin_into(&mut t.body, 10.0, tol);
        let face = t.body.get_loop(t.r#loop).unwrap().face;
        let mut survivor = t.body.get_face(face).unwrap().clone();
        survivor.rings = vec![thin];
        let err = t
            .body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("no loop is positive");
        assert_one_story(&err.to_string());
        assert!(is_the_zero_story(&err), "{err:?}: {err}");
    }

    /// **An empty loop bounds no area, so it is not positive and names
    /// nothing**: beside a zero outline the zero winding tells its
    /// story, and beside a clockwise outline every loop reads as a
    /// hole. An empty loop is not a carrier the kernel does not wind.
    /// (The first row is adopted from the review's `probe_winding.rs`.)
    #[test]
    fn an_empty_ring_is_not_positive_and_names_nothing() {
        let tol = Tol::witness();
        // A zero outline beside an empty ring.
        let mut body = Body::<f64>::new();
        let thin = thin_into(&mut body, 0.0, tol);
        let lone = body.mvfs(Point3::new(5.0, 5.0, 0.0), true).unwrap().r#loop;
        let face = body.get_loop(thin).unwrap().face;
        let mut survivor = body.get_face(face).unwrap().clone();
        survivor.rings = vec![lone];
        let err = body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("a zero outline is no outline");
        assert!(is_the_zero_story(&err), "zero beside empty: {err:?}: {err}");
        // A clockwise outline beside an empty ring.
        let t = tri(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            tol,
        );
        let mut body = t.body;
        let (face, mut survivor) = body
            .faces()
            .find(|(_, f)| f.outer != t.r#loop)
            .map(|(k, f)| (k, f.clone()))
            .expect("the triangle's second face");
        let lone = body.mvfs(Point3::new(5.0, 5.0, 0.0), true).unwrap().r#loop;
        survivor.rings = vec![lone];
        let err = body
            .merged_outline_ring(face, &survivor, tol)
            .expect_err("a face of holes has no outline");
        assert_one_story(&err.to_string());
        assert_eq!(
            err,
            MergeCoplanarError::MergedFaceRoleAmbiguous {
                face,
                verdict: OutlineVerdict::AllNegative,
            },
            "clockwise beside empty"
        );
    }

    /// **A torn winding walk names what it could not read**: each row
    /// tears the triangle's loop one way and reads the winding through
    /// the door's vocabulary. A stale `next` link names the half-edge
    /// it points at; a link that resolves but never returns names the
    /// loop (`LoopCycleBroken`); a half-edge its own edge does not
    /// claim is refused rather than read as the minus half. The first
    /// row is the control. (The link and unclaimed rows are adopted
    /// from the review's `probe_winding.rs`.)
    #[test]
    fn a_torn_winding_walk_names_what_it_could_not_read() {
        type Tear = fn(&mut Body<f64>, crate::HalfEdgeKey) -> Option<MergeCoplanarError>;
        let rows: [(&str, Tear); 8] = [
            ("nothing", |_, _| None),
            ("a next link", |b, first| {
                let next = b.get_half_edge(first).unwrap().next;
                b.half_edges.remove(next);
                Some(DanglingRef::Entity(EntityId::HalfEdge(next)).into())
            }),
            ("the loop's closure", |b, first| {
                let second = b.get_half_edge(first).unwrap().next;
                b.half_edges.get_mut(second).unwrap().next = second;
                let r#loop = b.get_half_edge(first).unwrap().parent_loop;
                Some(EulerOpError::LoopCycleBroken { r#loop }.into())
            }),
            ("an edge", |b, first| {
                let edge = b.get_half_edge(first).unwrap().edge;
                b.edges.remove(edge);
                Some(DanglingRef::Entity(EntityId::Edge(edge)).into())
            }),
            ("the edge's claim", |b, first| {
                let own = b.get_half_edge(first).unwrap().edge;
                let other = b.edges().map(|(k, _)| k).find(|&k| k != own).unwrap();
                b.half_edges.get_mut(first).unwrap().edge = other;
                Some(
                    EulerOpError::UnclaimedHalfEdge {
                        he: first,
                        edge: other,
                    }
                    .into(),
                )
            }),
            ("a curve", |b, first| {
                let edge = b.get_half_edge(first).unwrap().edge;
                let curve = b.get_edge(edge).unwrap().curve;
                b.curves.remove(curve);
                Some(DanglingRef::Geometry(GeomRef::Curve(curve)).into())
            }),
            ("a vertex", |b, first| {
                let v = b.get_half_edge(first).unwrap().start;
                b.vertices.remove(v);
                Some(DanglingRef::Entity(EntityId::Vertex(v)).into())
            }),
            ("a point", |b, first| {
                let v = b.get_half_edge(first).unwrap().start;
                let p = b.get_vertex(v).unwrap().point;
                b.points.remove(p);
                Some(DanglingRef::Geometry(GeomRef::Point(p)).into())
            }),
        ];
        let tol = Tol::witness();
        for (what, tear) in rows {
            let mut t = tri(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                tol,
            );
            let crate::entity::LoopBoundary::Cycle { first } =
                t.body.get_loop(t.r#loop).unwrap().boundary
            else {
                panic!("the triangle's loop is a cycle")
            };
            let want = match tear(&mut t.body, first) {
                None => Ok(LoopWinding::Wound(Sign::Positive)),
                Some(err) => Err(err),
            };
            assert_eq!(
                signed(t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol))),
                want,
                "tearing {what}"
            );
        }
    }

    /// **A run is the loop's sum with its closing chord added**, and
    /// nothing else (`crate::loop_winding`, the boolean join's ring
    /// lane): the open run `a → b → d` closed by the chord `d → a` is
    /// the triangle, and is decided on the loop's margin bit for bit;
    /// a null-edge strut on the run winds as its zero-length chord, on
    /// the loop and the run alike; and the run reads the loop's carrier
    /// set and claim, so a NURBS edge on it is not wound and a half its
    /// edge does not claim is refused rather than read as a minus half.
    #[test]
    fn a_run_is_the_loop_sum_with_its_closing_chord() {
        let tol = Tol::witness();
        let b = band(tol);
        let n = Vec3::unit_z();
        let fresh = || {
            tri(
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(0.0, 2.0, 0.0),
                Point3::new(-1.0, -1.0, 0.0),
                tol,
            )
        };
        // The run from `a → b` to `b → d`: the loop's first two halves.
        let run_of = |t: &Tri| {
            let he_ab = t.body.get_edge(t.ab).unwrap().he_plus;
            (he_ab, t.body.get_half_edge(he_ab).unwrap().next)
        };
        let loop_margin = |t: &Tri| match t.body.planar_loop_winding_decided(t.r#loop, n, b) {
            Ok(LoopWinding::Wound(Ok(d))) => d,
            other => panic!("the triangle winds: {other:?}"),
        };
        let run_margin =
            |t: &Tri, (h1, h2)| match t.body.planar_run_winding_decided((h1, h2), Straight, n, b) {
                Ok(Some(Ok(d))) => d,
                other => panic!("the run winds: {other:?}"),
            };

        let t = fresh();
        let whole = loop_margin(&t);
        assert_eq!(whole.sign, Sign::Positive, "a → b → d is counterclockwise");
        assert_eq!(run_margin(&t, run_of(&t)), whole, "the run is the loop");

        let mut t = fresh();
        let (h1, _) = run_of(&t);
        t.body
            .mev_null(
                MevSite::Fan { he1: h1, he2: h1 },
                crate::null::NewVertexSide::Above,
            )
            .unwrap();
        assert_eq!(
            loop_margin(&t),
            whole,
            "a strut on the loop is wound as nothing"
        );
        let first = t.body.get_half_edge(h1).unwrap().prev;
        let first = t.body.get_half_edge(first).unwrap().prev;
        assert_eq!(
            run_margin(&t, (first, t.body.get_half_edge(h1).unwrap().next)),
            whole,
            "a run opening on the strut is the same region"
        );

        let mut t = fresh();
        let run = run_of(&t);
        fit_a_nurbs_quarter(&mut t, tol);
        assert_eq!(
            t.body.planar_run_winding_decided(run, Straight, n, b),
            Ok(None),
            "a NURBS edge on the run is not wound by its chord"
        );

        let mut t = fresh();
        let (h1, h2) = run_of(&t);
        let own = t.body.get_half_edge(h2).unwrap().edge;
        let other = t.body.edges().map(|(k, _)| k).find(|&k| k != own).unwrap();
        t.body.half_edges.get_mut(h2).unwrap().edge = other;
        assert_eq!(
            t.body.planar_run_winding_decided((h1, h2), Straight, n, b),
            Err(TornLoop::Unclaimed {
                he: h2,
                edge: other
            }),
            "a half its edge does not claim is not read as a minus half"
        );
    }

    /// **A run's conic bulge is read, closing curve and all**: one
    /// semicircle of [`two_semicircle_disc`] closed by its diameter is
    /// a half-disc whose chord Newell sum is exactly zero, so only the
    /// run's bulge can decide it, and it winds counterclockwise like the
    /// disc; the run over both semicircles closes on a zero-length chord
    /// and is the disc's own loop, margin for margin. A closing CURVE is
    /// one more edge: a semicircle closed by the other semicircle is the
    /// disc, margin for margin, and closed by itself run back it encloses
    /// nothing — which a closing read as its straight chord would call
    /// the half-disc.
    #[test]
    fn a_run_on_arcs_is_decided_by_its_bulge() {
        let tol = Tol::witness();
        let b = band(tol);
        let n = Vec3::unit_z();
        let (body, disc, _) = two_semicircle_disc(tol);
        let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(disc).unwrap().boundary
        else {
            panic!("the disc's loop is a cycle");
        };
        let second = body.get_half_edge(first).unwrap().next;
        let run = |h1, h2| match body.planar_run_winding_decided((h1, h2), Straight, n, b) {
            Ok(Some(Ok(d))) => d,
            other => panic!("the run winds: {other:?}"),
        };
        for half in [first, second] {
            assert_eq!(
                run(half, half).sign,
                Sign::Positive,
                "a semicircle closed by its diameter is a counterclockwise half-disc"
            );
        }
        let whole = match body.planar_loop_winding_decided(disc, n, b) {
            Ok(LoopWinding::Wound(Ok(d))) => d,
            other => panic!("the disc winds: {other:?}"),
        };
        assert_eq!(run(first, second), whole, "the whole run is the disc");
        // The one-half run `opens` closed by a chord curve, as the ring
        // lane reads it: a [`SegmentCurve`] between `opens` and the other
        // half, standing for the match's, whose spec is the edge under
        // `he`, computed running the way that edge's plus half runs;
        // `with_traversal` says whether the closing (from the run's end
        // back to its start) runs the way `he` does. Both edges' plus
        // halves run a → b, so the closing of `first` (b → a's half, its
        // edge's minus) is the curve as computed, and the closing of
        // `second` is the curve run back.
        let closed = |opens: crate::HalfEdgeKey, he: crate::HalfEdgeKey, with_traversal: bool| {
            let other = if opens == first { second } else { first };
            let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
            let curve = body
                .get_curve_geom(edge.curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = curve.params();
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: body
                        .get_face(body.get_loop(disc).unwrap().face)
                        .unwrap()
                        .surface,
                    s2: body
                        .get_face(body.get_loop(disc).unwrap().face)
                        .unwrap()
                        .surface,
                    witness: curve.carrier().mid_point(t0, t1),
                },
                carrier: curve.carrier().clone(),
                param_start: t0,
                param_end: t1,
            };
            let forward = with_traversal == edge.claim(he).unwrap().plus;
            let halves = if forward {
                (other, opens)
            } else {
                (opens, other)
            };
            let segment = crate::chord_join::SegmentCurve::of(halves, Some(spec));
            let face = body.get_loop(disc).unwrap().face;
            let closing = segment.run_closing(opens, face).unwrap();
            match body.planar_run_winding_decided(
                (opens, opens),
                RunClosing::of(closing.as_ref()),
                n,
                b,
            ) {
                Ok(Some(Ok(d))) => d,
                other => panic!("the closed run winds: {other:?}"),
            }
        };
        for (opens, other) in [(first, second), (second, first)] {
            assert_eq!(
                closed(opens, other, true),
                whole,
                "a semicircle closed by the other one is the disc"
            );
            assert_eq!(
                closed(opens, opens, false).sign,
                Sign::Zero,
                "a semicircle closed by itself run back encloses nothing"
            );
        }
    }

    /// **The ring lane's own shape**: a run that opens AND closes on a
    /// null half, as every ring-lane run does (the match's two halves
    /// are null). Struts at `a` and `d` of the triangle; the run from
    /// the strut half entering `a` through `a → b → d` to the strut half
    /// leaving `d` ends at that null half's far vertex — a copy of `d` —
    /// and its closing chord `d → a` makes it the triangle, margin for
    /// margin.
    #[test]
    fn a_run_between_two_null_halves_is_the_region_they_bracket() {
        let tol = Tol::witness();
        let b = band(tol);
        let n = Vec3::unit_z();
        let mut t = tri(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(-1.0, -1.0, 0.0),
            tol,
        );
        let whole = match t.body.planar_loop_winding_decided(t.r#loop, n, b) {
            Ok(LoopWinding::Wound(Ok(d))) => d,
            other => panic!("the triangle winds: {other:?}"),
        };
        let he_ab = t.body.get_edge(t.ab).unwrap().he_plus;
        let he_bd = t.body.get_half_edge(he_ab).unwrap().next;
        let he_da = t.body.get_half_edge(he_bd).unwrap().next;
        for at in [he_ab, he_da] {
            t.body
                .mev_null(
                    MevSite::Fan { he1: at, he2: at },
                    crate::null::NewVertexSide::Above,
                )
                .unwrap();
        }
        let h1 = t.body.get_half_edge(he_ab).unwrap().prev;
        let h2 = t.body.get_half_edge(he_bd).unwrap().next;
        let is_null = |he: crate::HalfEdgeKey| {
            let edge = t.body.get_half_edge(he).unwrap().edge;
            let curve = t.body.get_edge(edge).unwrap().curve;
            t.body.get_curve_geom(curve).unwrap().certified().is_none()
        };
        assert!(
            is_null(h1) && is_null(h2),
            "the run opens and closes on null halves"
        );
        assert_ne!(
            t.body.half_edge_end(h2),
            Some(t.body.get_half_edge(h2).unwrap().start),
            "the run ends at the null half's far vertex, not at `d` itself"
        );
        match t.body.planar_run_winding_decided((h1, h2), Straight, n, b) {
            Ok(Some(Ok(d))) => assert_eq!(d, whole, "the bracketed run is the triangle"),
            other => panic!("the run winds: {other:?}"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod declared_reach_rows {
    use crate::body::Body;
    use crate::entity::{FaceKey, LoopBoundary, VertexKey};
    use crate::euler::{FaceSurface, MefSite};
    use crate::test_support_fixtures::{line, prism_z};
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::MergeCoplanarError;

    /// The half-edge of `face`'s outer loop that starts at `v`.
    fn leaving(
        body: &Body<f64>,
        face: FaceKey,
        v: VertexKey,
    ) -> Option<crate::entity::HalfEdgeKey> {
        let f = body.get_face(face).expect("a live face");
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).expect("a loop").boundary else {
            panic!("a cycle");
        };
        body.loop_cycle(first)
            .expect("the loop walks")
            .into_iter()
            .find(|&he| body.get_half_edge(he).expect("live").start == v)
    }

    /// **A declared pair glues only where both faces lie in band of one
    /// plane**, read through the public merge: a dart prism whose top
    /// is split along its 1 cm inner diagonal, one half re-described
    /// as a plane tilted about that diagonal by 10, 50 and 90 Kε per
    /// metre. The other half's far tip stands 10 m off the hinge, so
    /// the tilted plane passes it 100 to 900 Kε away. Read over the
    /// shared edge's chord (1 cm), the tilt reads at or under the band
    /// and the pair glued; read over a ball enclosing both faces, with
    /// their vertices as the consumed points, the tip contradicts the
    /// declaration.
    #[test]
    fn a_tilt_in_band_at_the_shared_edge_but_past_it_across_the_faces_is_contradicted() {
        let tol = Tol::witness();
        let k_eps = Band::linear(tol).expect("the witness band").escalate();
        for per_metre in [10.0, 50.0, 90.0] {
            let theta = per_metre * k_eps;
            // A(0,0) → D(10,−10) → C(1 cm, 0) → B(10,10): reflex at C,
            // so A–C is an interior diagonal.
            let prism = prism_z::<f64>(
                &[(0.0, 0.0), (10.0, -10.0), (0.01, 0.0), (10.0, 10.0)],
                0.0,
                1.0,
                tol,
            );
            let mut body = prism.body;
            let (top, a, c) = (prism.top_face, prism.top[0], prism.top[2]);
            let at = |body: &Body<f64>, v: VertexKey| {
                *body
                    .get_point(body.get_vertex(v).expect("live").point)
                    .expect("live")
            };
            let (pa, pc) = (at(&body, a), at(&body, c));
            let flat = body
                .get_surface(body.get_face(top).expect("live").surface)
                .expect("live")
                .clone();
            let he1 = leaving(&body, top, a).expect("A is on the top");
            let he2 = leaving(&body, top, c).expect("C is on the top");
            // Lifts RechartStrandsDescriptions: the half on a key of its own is the declared pair; its rim's descriptions are not the row.
            let split = body
                .lifting_rechart_refusals_for_tests(|body| {
                    body.mef(
                        MefSite::Chords { he1, he2 },
                        line(pa, pc),
                        FaceSurface::New {
                            surface: flat,
                            sense: true,
                        },
                        tol,
                    )
                })
                .expect("the diagonal splits the top");
            // The half holding B (+y) turns about the x-axis hinge at
            // z = 1; the half holding D stays flat.
            let b_side = if leaving(&body, split.face, prism.top[3]).is_some() {
                split.face
            } else {
                top
            };
            let tilted = body.get_face(b_side).expect("live").surface;
            *body.surfaces.get_mut(tilted).expect("live") = geom::Surface::Plane {
                origin: Point3::new(0.0, 0.0, 1.0),
                normal: Vec3::new(0.0, -theta, 1.0).normalize(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let flat_key = body
                .get_face(if b_side == top { split.face } else { top })
                .expect("live")
                .surface;
            let got = body.merge_coplanar_faces_declared(&[(flat_key, tilted)], tol);
            assert!(
                matches!(got, Err(MergeCoplanarError::DeclarationContradicted { .. })),
                "{per_metre} Kε/m: {got:?}"
            );
        }
    }
}
