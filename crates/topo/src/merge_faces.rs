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

use geom::{NetState, Surface};
use geom_brep::SurfaceKind;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Tol};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::{PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, oriented_plane_eq};
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, LoopKey, VertexKey};
use crate::euler::EulerOpError;
use crate::face_normal::plane_outward_normal;
use crate::geometry::SurfaceKey;
use crate::readback::DanglingRef;
use crate::validate::{ValidationError, validate_closed};

/// One merged run: the surviving face and what was consumed into it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergedGroup {
    /// The surviving face (the group's first face in face-arena order).
    pub kept: FaceKey,
    /// The absorbed faces (dead keys), in kill order.
    pub absorbed: Vec<FaceKey>,
    /// The killed shared edges (dead keys), in kill order.
    pub killed_edges: Vec<EdgeKey>,
    /// Rings minted by intra-face shared-edge kills (`kemr` — a merged
    /// run that surrounds a hole grows a genuine ring), in mint order.
    pub rings_made: Vec<LoopKey>,
    /// Vertices killed by the straight-seam repair (`kev`), in kill
    /// order — a junction that was interior to one straight carrier
    /// and went with its seam.
    ///
    /// Recorded rather than left implicit because this is the one
    /// thing the op destroys that no other field names: `absorbed`
    /// carries the dead faces and `killed_edges` the dead edges, and
    /// without this a caller reconciling the Euler delta would find a
    /// `v −1` with nothing accounting for it.
    pub killed_vertices: Vec<VertexKey>,
}

/// One record, two subjects, told apart by `reason`: a merge GROUP
/// that was NOT glued (its shape is outside the merge's never-elide
/// Euler inventory — loud in the record, never a silent drop, never a
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
    /// Groups left unmerged as outside the inventory, with the
    /// refusal that stopped each. Non-empty needs no declaration: a
    /// curved run that would close its chart's full period is
    /// recorded here through either entry point. Declared pairs on a
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

/// Which ladder rung licensed one mergeable adjacency (crate-
/// internal: declared-pair-licensed groups get per-group staging).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MergeRung {
    /// Same surface key or same GeomSource — the ratified hard rungs.
    Hard,
    /// A per-call declared surface pair.
    DeclaredPair,
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

/// What becomes of one group's INVENTORY refusal — the door's two
/// failure regimes, named so the boundary between them is a value
/// rather than a condition spelled inline.
///
/// Both regimes raise the SAME [`MergeCoplanarError`], which is why
/// nothing here duplicates that enum. Which regime a group runs under
/// is a property of the group — how its adjacency was licensed, and
/// whether its surface is curved.
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
    /// and the body is untouched. Structural planar runs — the
    /// ratified whole-refusal semantics.
    RefusesTheCall,
    /// An inventory refusal is recorded in
    /// [`MergeCoplanarOutcome::skipped`] and the remaining groups
    /// commit. Declared-licensed runs (the declaration served the
    /// consuming op's classification even where the glue is outside
    /// the inventory) and curved runs (a full-period closure keeps
    /// the operands' cut-carrying canonical form). The group is
    /// staged on its own clone and adopted only after its own tier-2
    /// gate, so a recorded skip is never a partial commit.
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
    /// [`SurfaceKind::of`]'s — the crate's one carrier-kind read —
    /// and the net question is [`NetState`]'s, matched exhaustively so
    /// no state is answered by a default.
    fn of<T: geom_core::Real>(surface: &Surface<T>) -> Result<Self, PoisonedNet> {
        match surface {
            Surface::Nurbs(net) => match net.net_state() {
                NetState::Placeholder => Ok(Self::Placeholder),
                NetState::Poisoned => Err(PoisonedNet),
                NetState::Described => Ok(Self::Curved),
            },
            s if SurfaceKind::of(s) == SurfaceKind::Plane => Ok(Self::Plane),
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
    /// The hard rungs glue on *source* identity, not on kind, and
    /// [`Body::set_surface_source`] is a public door that stamps a
    /// source without comparing the descriptions it joins — so a
    /// caller can declare a plane and a cylinder to be one recipe
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
        /// The contradicting predicate's diagnostics.
        diag: Indeterminate,
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
        /// keys resolved when the door read them; a caller that
        /// re-describes edges after this call may drop a surface the
        /// pair names (a surface held only by an edge curve's
        /// reference goes with that curve), so a consumer walks the
        /// record's `faces`, not the pair, for what is live.
        pair: (SurfaceKey, SurfaceKey),
        /// The carrier kind both surfaces share.
        kind: SurfaceKind,
    },
    /// After absorbing a group, the survivor's loops admit no unique
    /// positively-wound outline (zero or several positive windings) —
    /// the outer/ring roles cannot be assigned; refused rather than
    /// guessed (M5 S1 fix pass: the intra-face `kemr`'s provisional
    /// ring designation is now RESOLVED by winding, and shapes the
    /// resolution cannot decide are outside the inventory).
    MergedFaceRoleAmbiguous {
        /// The merged survivor face.
        face: FaceKey,
    },
    /// A plane-identity margin escalated while verifying a declared
    /// pair (in-band sliver) — typed, never guessed.
    Escalated {
        /// The predicate's diagnostics.
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
}

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
            Self::DeclarationContradicted { diag } => write!(
                f,
                "merge_coplanar_faces: declared coincidence contradicts the geometry ({diag}) \
                 — fix the declaration or the geometry, the op never glues a lie"
            ),
            Self::DeclaredOppositeOrientation { f1, f2 } => write!(
                f,
                "merge_coplanar_faces: declared pair ({f1:?}, {f2:?}) meets with opposite \
                 orientations — unmergeable in a closed solid"
            ),
            Self::DeclaredCarrierUnsupported { pair, kind } => write!(
                f,
                "merge_coplanar_faces: declared pair {pair:?} lies on a {kind} carrier — \
                 the declaration is legal and served the op, but this door's declared-pair \
                 rung is planar and has no {kind} arm; the pair is left unmerged and recorded",
                kind = kind.name()
            ),
            Self::MergedFaceRoleAmbiguous { face } => write!(
                f,
                "merge_coplanar_faces: merged face {face:?} has no unique positively-wound \
                 outline among its loops — outer/ring roles cannot be assigned; refused"
            ),
            Self::Escalated { diag } => write!(
                f,
                "merge_coplanar_faces: plane-identity margin escalated verifying a declared \
                 pair ({diag})"
            ),
            Self::Band { error } => write!(f, "merge_coplanar_faces: {error}"),
            Self::Pcurve { source } => write!(
                f,
                "merge_coplanar_faces: the staged result's pcurve re-mint refused \
                 ({source}) — the body is untouched"
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
    /// The strut edge's two halves start at distinct vertices, which
    /// `strut_tip`'s valence-one answer established.
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
/// | `loop_winding`, through `merged_outline_ring` | `StaleKey` |
/// | `ring_move` | `StaleKey`, `RingIsOuter` (C), `CrossShell` (C) |
/// | `kef` | `StaleKey`, `UnclaimedHalfEdge`, `LoopCycleBroken`, `LoopNotCycle`, `SameLoop` (C), `SameFace` (**R**), `FaceHasRings` (C) |
/// | `kev` | `StaleKey`, `UnclaimedHalfEdge`, `LoopNotCycle`, `OrbitBroken`, `SelfLoopEdge` (C) |
/// | `kemr` | `StaleKey`, `NotSameEdge`, `LoopNotCycle`, `LoopCycleBroken`, `EmptyAnchorsCollide`, `NotSameLoop` (C) |
///
/// The variants the table does not name take the enum's verdict like
/// any other variant this door does not contradict. One of them this
/// door does raise, outside `merge_group`: `StaleGeometry`, when the
/// kind census ([`Body::merge_kind`]) meets a face whose surface key
/// does not resolve — announced through [`crate::DanglingRef`] before
/// any group reaches the surgery. The rest belong to operators this
/// door does not call: the attachment and split gates
/// (`set_edge_curve`, `split_edge`), the make-side sites (`mev`,
/// `mef`, `mekr`), `kvfs`, `kfmrh`'s cross-solid form and the
/// shell-move door. That is not a third arm: an arm the door cannot
/// reach cannot be pinned, and a classification nothing can
/// distinguish is documentation, which is what this table is. No
/// count of the remainder is stated here; the match below is the
/// census.
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
            // carries the same `EstablishedFact` in the surgery. ----
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
            | E::Certification { .. }
            | E::RebasedCarrier { .. }
            | E::RebasedNullEdge { .. }
            | E::DescriptionNotAdjacent { .. }
            | E::FanStartMismatch { .. }
            | E::FanOrbitBroken { .. }
            | E::LoopNotEmpty { .. }
            | E::NotSameFace { .. }
            | E::SolidNotSingleShell { .. }
            | E::ShellNotSingleFace { .. }
            | E::NullScaffoldCurve { .. }
            | E::SplitParamNotInterior { .. }
            | E::SplitParamEscalated { .. }
            | E::PcurveSplit { .. }
            | E::CrossSolid { .. }
            | E::NoShellsNamed
            | E::ShellRepeated { .. }
            | E::ShellsAcrossSolids { .. }
            | E::SolidWouldEmpty { .. } => Self::TheEnumsVerdict,
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
    /// **The straight-seam repair (`kev`).** An intra-face duplicate is
    /// not always a ring. When the group's shared boundary is exactly
    /// two edges meeting at a valence-2 vertex whose two departures are
    /// collinear and OPPOSED — the vertex is interior to one straight
    /// carrier — the surviving duplicate is a dangling STRUT, and the
    /// op kills it with `kev` instead of minting a ring with `kemr`.
    /// The surgery DELETES BOTH seam edges and the junction vertex: the
    /// `kef` takes one, the `kev` takes the other along with the vertex
    /// it dangles from. Nothing is re-described, because the removed
    /// vertex was interior to a straight locus and the union of the two
    /// collinear pieces is that same locus. The motivating instance is
    /// a full revolve's axis-touching cap (the two seam edges are the
    /// halves of the disc's diameter, the vertex is the pole), but the
    /// licence is collinearity and not provenance
    /// ([`Body::redundant_subdivision_vertex`]'s docs carry the
    /// argument and its residue).
    ///
    /// **Atomic and deterministic (D9)**: the op stages on a clone —
    /// on any refusal `self` is untouched; on success the staged body
    /// replaces `self` wholesale. All scans are arena-order; the
    /// surviving face of each group is its first face in face-arena
    /// order; edges die in edge-arena order. Composite Euler delta per
    /// group: `f −(n−1)`, `e −k`, plus `r +m` for intra-face `kemr`
    /// kills, and `v −1` for each straight-seam `kev` (which is what
    /// keeps χ conserved when a ring is NOT minted: `kemr` trades an
    /// edge for a ring, `kev` trades an edge for a vertex). Each step
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
    /// declared-licensed regime's own rule — the declaration served
    /// the calling op) and never refused, even when the call has
    /// nothing else to merge.
    ///
    /// # Two failure regimes, one refusal vocabulary
    ///
    /// A group's **inventory** refusal — *this group cannot be
    /// merged* — either refuses the call or is recorded as a
    /// [`SkippedMerge`] while the remaining groups commit, and which
    /// of the two is a property of the GROUP: declared-licensed and
    /// curved runs record, structural planar runs refuse. Both raise
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
    /// there is no regime yet to record it under. **A
    /// declared-licensed group that straddles two surface kinds
    /// therefore loses the recording semantics its declaration bought
    /// it** — the call refuses where a kind-uniform licensed group
    /// would have carried on. That is the honest outcome of having no
    /// contract to give such a group, not a decision to refuse
    /// licensed work.
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
    /// was — never a partial commit, and the unglued coplanar
    /// adjacency persists as the operands already carried it (and
    /// refuses loudly downstream if reused undeclared).
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
            self.get_surface(k)
                .map(SurfaceKind::of)
                .ok_or(MergeCoplanarError::InvalidDeclaration {
                    surface: k,
                    what: "declared surface key does not resolve",
                })
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
        let kinds = self.kind_census()?;
        let mut outcome = MergeCoplanarOutcome {
            placeholders: self
                .faces()
                .map(|(k, _)| k)
                .filter(|&k| kinds.get(k) == Some(&MergeKind::Placeholder))
                .collect(),
            ..MergeCoplanarOutcome::default()
        };
        // ---- Mergeable adjacency (read-only, edge-arena order). ----
        let mut neighbors: SecondaryMap<FaceKey, Vec<FaceKey>> = SecondaryMap::new();
        let mut declared_faces: std::collections::BTreeSet<FaceKey> =
            std::collections::BTreeSet::new();
        let mut any = false;
        for (edge_key, edge) in self.edges() {
            let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
            let (fp, fm) = (hp.face, hm.face);
            if fp != fm
                && let Some(rung) =
                    self.planes_declared_equal(fp, fm, edge_key, declared_ctx.as_ref())?
            {
                if let Some(entry) = neighbors.entry(fp) {
                    entry.or_default().push(fm);
                }
                if let Some(entry) = neighbors.entry(fm) {
                    entry.or_default().push(fp);
                }
                if rung == MergeRung::DeclaredPair {
                    declared_faces.insert(fp);
                    declared_faces.insert(fm);
                }
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
            if !neighbors.contains_key(face_key) || label.contains_key(face_key) {
                continue;
            }
            let id = groups.len();
            let mut rest = Vec::new();
            label.insert(face_key, id);
            let mut pending = vec![face_key];
            while let Some(next) = pending.pop() {
                for &n in neighbors.get(next).map(Vec::as_slice).unwrap_or(&[]) {
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
        for (rep, rest) in groups {
            let (regime, kind) = match Self::group_contract(rep, &rest, &kinds, &declared_faces)? {
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
        // ---- Gate: tier-valid after; commit. ----
        if let Err(errors) = validate_closed(&work) {
            return Err(MergeCoplanarError::ResultNotClosed { errors });
        }
        // A body that carried stored pcurve caches RE-MINTS them on
        // the staged result before commit (the `topo::pcurves` module
        // docs' rule for ops that mutate minted bodies): the merge
        // rebuilds face loops, and two absorbed fragments' walks were
        // branch-anchored independently — the merged loop's one-branch
        // walk must be derived fresh, never stitched from the
        // fragments' rows. Still on the staged clone, so a mint
        // refusal keeps the untouched-on-error contract.
        //
        // LATENT (named, not reachable by any current path): the mint
        // pass holds the fitted door (`AtRestPolicy::fitted_lane`) and
        // mints U2's `General` arm through it, but the FITTED variant
        // itself still has no mint site, so a `Fitted`
        // cache (at rest since M6-2) on a merged body would still come
        // back as the mint pass's honest-skip — the face legally
        // UNCACHED, its fitted certificate silently dropped. What is
        // left of that item is `certify_fitted`'s own wiring, not the
        // bound; this site inherits the fix when that lands.
        if !self.pcurves.is_empty() {
            crate::pcurves::mint_pcurves(&mut work, tol)
                .map_err(|source| MergeCoplanarError::Pcurve { source })?;
        }
        let mut skipped = declined_records(&work);
        skipped.append(&mut outcome.skipped);
        outcome.skipped = skipped;
        self.adopt(work);
        Ok(outcome)
    }

    /// The other half of `edge`, resolved — the re-checks' shared
    /// lookup. `None` when either key does not resolve, which the
    /// re-checks read as "nothing to prove here": a dangling key is
    /// the operator's own refusal to make, and it is torn by the
    /// enum's line either way.
    fn edge_mate(
        &self,
        he: crate::entity::HalfEdgeKey,
        edge: EdgeKey,
    ) -> Option<&crate::entity::HalfEdge> {
        let e = self.get_edge(edge)?;
        let mate = if e.he_plus == he {
            e.he_minus
        } else if e.he_minus == he {
            e.he_plus
        } else {
            return None;
        };
        self.get_half_edge(mate)
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
        let surface = self
            .get_face(face)
            .ok_or(DanglingRef::Entity(EntityId::Face(face)))?
            .surface;
        let described = self
            .get_surface(surface)
            .ok_or(DanglingRef::Geometry(GeomRef::Surface(surface)))?;
        MergeKind::of(described)
            .map_err(|PoisonedNet| MergeCoplanarError::PoisonedSurfaceDescription { face })
    }

    /// Every live face's [`MergeKind`], in one pass — the one place
    /// the door asks the kind question of the arena.
    ///
    /// # Errors
    ///
    /// [`Body::merge_kind`]'s, for the first face (arena order) that
    /// raises one.
    fn kind_census(&self) -> Result<SecondaryMap<FaceKey, MergeKind>, MergeCoplanarError> {
        let mut kinds = SecondaryMap::new();
        for (face_key, _) in self.faces() {
            kinds.insert(face_key, self.merge_kind(face_key)?);
        }
        Ok(kinds)
    }

    /// **Does `toward` dangle alone at its start vertex** — the
    /// condition that licenses `kev` over `kemr` on a straight seam's
    /// surviving duplicate?
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

    /// One group's [`GroupContract`]: whether it runs, and under which
    /// failure regime its INVENTORY refusals fall.
    ///
    /// A group records a skip when its adjacency was licensed by a
    /// declared pair (any member), or when it is curved — the two
    /// cases whose refusals are statements about the merge's
    /// inventory rather than about the body, and whose unglued
    /// adjacency is a legal output the operands already carried. A
    /// structural planar run refuses the call. A placeholder run is
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
        declared_faces: &std::collections::BTreeSet<FaceKey>,
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
        let licensed =
            declared_faces.contains(&rep) || rest.iter().any(|f| declared_faces.contains(f));
        Ok(match rep_kind {
            MergeKind::Placeholder => GroupContract::SetAside,
            MergeKind::Plane => GroupContract::Runs {
                regime: if licensed {
                    GroupRegime::RecordsASkip
                } else {
                    GroupRegime::RefusesTheCall
                },
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

    /// The F6 ladder's merge test (M4 PR 5, the N6 retirement): same
    /// surface key (structural), same [`crate::GeomSource`] including
    /// orient (declared — shared recipe source, syntactic identity,
    /// zero numerics), or the pair's surfaces are declared-equivalent
    /// by this call's face pairs (verified through `plane_eq`'s
    /// declared rung at the meeting edge; contradiction refuses).
    ///
    /// The M3-era rung — bit-identical nine-scalar descriptions — is
    /// RETIRED from production: equal bits without shared source stay
    /// unglued (the ladder's ratified rung (b)). The bit comparison
    /// survives as the debug assertion that same-source records agree
    /// with the bits. *No banded comparison certifies coincidence
    /// here by design* — the declared-pair verification only checks
    /// the declaration is not a lie; the INTENT does the gluing.
    ///
    /// Non-plane surfaces never merge, same-key included (curved
    /// maximality is M5's).
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
    fn planes_declared_equal(
        &self,
        f1: FaceKey,
        f2: FaceKey,
        edge: EdgeKey,
        declared: Option<&DeclaredCtx>,
    ) -> Result<Option<MergeRung>, MergeCoplanarError> {
        let (Some(face1), Some(face2)) = (self.get_face(f1), self.get_face(f2)) else {
            return Ok(None);
        };
        let (k1, k2) = (face1.surface, face2.surface);
        let (Some(s1), Some(s2)) = (self.get_surface(k1), self.get_surface(k2)) else {
            return Ok(None);
        };
        // The shared-sense precondition (fn docs): a differing bit
        // makes the two outward normals opposite, so neither hard rung
        // — both of which certify the SURFACE, not the face — may
        // conclude the faces are one region. Falling through leaves
        // the declared rung to refuse loudly if the pair was declared.
        let same_sense = face1.sense == face2.sense;
        // The hard rungs are KIND-AGNOSTIC since M5 PR 9 (C12.5, the
        // cosurface generalization): the same-key and same-source
        // tests never touch a numeric coordinate, so nothing about
        // them was planar — the M3-era "curved same-key neighbors
        // stay unmerged" note flips here, with the same ladder, the
        // same never-numeric rule, and N3 naming semantics unchanged.
        // The named consumer: the boolean zip's re-merge of a
        // cylinder wall split by a through cut.
        if k1 == k2 && same_sense {
            return Ok(Some(MergeRung::Hard)); // structural
        }
        // Declared rung, N6 form: same recipe source INCLUDING orient
        // — a provenance lookup, no numerics (M4's GeomSource
        // retirement consumed, NOT bit_identity). The debug assertion
        // is DESIGN.md's "records agree with bits", stated for the
        // planar kind where the bit predicate exists.
        if same_sense
            && let (Some(g1), Some(g2)) = (self.surface_source(k1), self.surface_source(k2))
            && g1 == g2
        {
            // Asserted only where the scalar HAS a bit channel: the rung
            // is the provenance lookup, the bits are its evidence, and a
            // scalar with no channel (`Dual`, `Sym`) offers none —
            // `None` there is not disagreement.
            #[cfg(debug_assertions)]
            if let (
                Surface::Plane {
                    origin: o1,
                    normal: n1,
                    u_ref: u1,
                },
                Surface::Plane {
                    origin: o2,
                    normal: n2,
                    u_ref: u2,
                },
            ) = (s1.clone(), s2.clone())
                && let Some(agree) = crate::source::plane_bits_witness(o1, n1, o2, n2, false)
                    .zip(crate::source::vec3_bits_witness(u1, u2))
                    .map(|(plane, u_ref)| plane && u_ref)
            {
                debug_assert!(
                    agree,
                    "same-source theorem violated: same-source surface descriptions disagree \
                     bitwise (kernel bug: a source survived a geometric rewrite)"
                );
            }
            return Ok(Some(MergeRung::Hard));
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
            return Ok(None);
        };
        // Declared face pairs (this call's recipe intent), verified.
        if let Some(ctx) = declared
            && ctx.eq.same(k1, k2)
        {
            let band = ctx.band;
            let arm = self.edge_chord_len(edge).unwrap_or_else(T::one);
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
            return match oriented_plane_eq(&p1, &p2, id, arm, band) {
                Ok(PlaneRelation::SameOriented) => Ok(Some(MergeRung::DeclaredPair)),
                Ok(PlaneRelation::SameOpposite) => {
                    Err(MergeCoplanarError::DeclaredOppositeOrientation { f1, f2 })
                }
                // Unreachable through the declared rung; kept typed.
                Ok(PlaneRelation::Distinct) => Ok(None),
                Err(PlaneEqError::Contradicted(diag)) => {
                    Err(MergeCoplanarError::DeclarationContradicted { diag })
                }
                Err(PlaneEqError::Escalated(diag) | PlaneEqError::Undeclared { diag, .. }) => {
                    Err(MergeCoplanarError::Escalated { diag })
                }
            };
        }
        Ok(None)
    }

    /// The chord length between an edge's endpoints — the lever arm
    /// metering the declared-pair verification at that edge.
    fn edge_chord_len(&self, edge: EdgeKey) -> Option<T> {
        let e = self.get_edge(edge)?;
        let pa = *self.get_point(self.get_vertex(self.get_half_edge(e.he_plus)?.start)?.point)?;
        let pb = *self.get_point(
            self.get_vertex(self.get_half_edge(e.he_minus)?.start)?
                .point,
        )?;
        Some((pb - pa).norm())
    }

    /// **Is `v` a redundant subdivision vertex of a straight seam?**
    ///
    /// This is the geometric licence for removing a seam vertex, and it
    /// is deliberately NOT a claim about provenance. A vertex of
    /// valence 2 whose two edges lie on ONE straight carrier, leaving
    /// it in OPPOSITE directions, is interior to a single line
    /// segment: deleting it and merging its two edges replaces two
    /// collinear pieces with their union and **no locus changes**. The
    /// repair is then geometry-preserving by construction, whatever
    /// produced the vertex.
    ///
    /// A full revolve's axis-touching cap is the motivating instance —
    /// its two meridians are the two halves of the disc's DIAMETER,
    /// with the pole interior to it — but nothing here mentions poles
    /// or axes, and it should not: the same fact licenses the same
    /// removal on any straight seam.
    ///
    /// What it refuses is the case the F7 rule exists for. Two
    /// coplanar faces meeting along a bent seam — `merge_skip`'s
    /// L-corner, where two overlapping rectangles meet at a
    /// re-entrant corner — have a valence-2 junction too, and an
    /// earlier form of this trigger that tested only valence was
    /// falsified by exactly that fixture. Perpendicular departures
    /// fail the collinearity decision, so the corner survives and the
    /// merge still refuses, which is the pinned behaviour.
    ///
    /// Both decisions are metered on the shorter incident segment (the
    /// honest lever for an angular quantity read as a length).
    fn redundant_subdivision_vertex(
        &self,
        v: VertexKey,
        band: Band,
    ) -> Result<bool, MergeCoplanarError> {
        let Some(em) = self.get_vertex(v).and_then(|vd| vd.emanating) else {
            return Ok(false);
        };
        // PAIRED, deliberately unfixed here: this reads a broken
        // orbit as "not a redundant vertex", where `strut_tip` — the
        // other consumer of the identical condition, forty lines down
        // the same surgery — announces it. The two answers are the
        // open row's subject and moving one without the other would
        // hide the pair rather than settle it.
        let Some(orbit) = self.vertex_orbit(em) else {
            return Ok(false);
        };
        if orbit.len() != 2 {
            return Ok(false);
        }
        let point = |vk: VertexKey| {
            self.get_vertex(vk)
                .and_then(|vd| self.get_point(vd.point).copied())
        };
        let Some(pv) = point(v) else {
            return Ok(false);
        };
        // Each orbit member starts at `v`; its mate starts at the far
        // end. Both carriers must be straight — an arc through `v` is
        // not a subdivision of anything.
        let mut departures = Vec::with_capacity(2);
        for &he in &orbit {
            let Some(hd) = self.get_half_edge(he) else {
                return Ok(false);
            };
            let Some(e) = self.get_edge(hd.edge) else {
                return Ok(false);
            };
            let straight = self
                .get_curve_geom(e.curve)
                .and_then(crate::null::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
            if !straight {
                return Ok(false);
            }
            let far = if e.he_plus == he {
                e.he_minus
            } else {
                e.he_plus
            };
            let Some(pf) = self.get_half_edge(far).and_then(|h| point(h.start)) else {
                return Ok(false);
            };
            departures.push(pf - pv);
        }
        let (d1, d2) = (departures[0], departures[1]);
        // The lever is the SHORTER incident segment — an angular
        // quantity is read here as a length, and the shorter arm is the
        // conservative one. `Real::min`, not `<`: the scalar backends
        // order intervals, not values, so a bare comparison on `T` is
        // not available and would not mean this if it were (the S10
        // exact-bit discipline). A degenerate zero-length edge makes
        // the normalization poison, which `decide` escalates typed
        // rather than silently answering.
        let (n1, n2) = (d1.norm(), d2.norm());
        let arm = n1.min(n2);
        let (u1, u2) = (d1 / n1, d2 / n2);
        let escalate = |diag| MergeCoplanarError::Escalated { diag };
        // Collinear: the two departures span no angle.
        if crate::validate::decide(
            "merge_seam_collinear",
            Margin::levered(u1.cross(u2).norm(), arm),
            band,
        )
        .map_err(escalate)?
            != geom_core::Sign::Zero
        {
            return Ok(false);
        }
        // ...and OPPOSED, so `v` is interior to the union rather than a
        // point the seam doubles back from.
        Ok(
            crate::validate::decide("merge_seam_opposed", Margin::levered(u1.dot(u2), arm), band)
                .map_err(escalate)?
                == geom_core::Sign::Negative,
        )
    }

    /// Merges one group into `rep`, its arena-first member (see the
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
    ) -> Result<MergedGroup, MergeCoplanarError> {
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
        // **The straight-seam junction, decided ONCE** on the group as
        // it arrives — before any mutation, because the answer licenses
        // a different repair below and must not be re-derived from a
        // body that repair is halfway through changing.
        //
        // Verified against this crate's whole merge/boolean fixture
        // corpus before it was wired to anything (the method the
        // reviewers' falsifications earned): it fires on a collinear
        // subdivided seam and on nothing else in the corpus — not on
        // `merge_skip`'s L-corner, not on either review arm's bent
        // chords, not on an inset ring.
        let straight_seam = {
            let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
            // The scan keeps the start vertices it already walked, so
            // the junction test below looks nothing up.
            let mut shared: Vec<[VertexKey; 2]> = Vec::new();
            for (_, e) in self.edges() {
                let (hp, hm) = self.edge_halves(e.he_plus, e.he_minus)?;
                if hp.face != hm.face && in_group(hp.face) && in_group(hm.face) {
                    shared.push([hp.start, hm.start]);
                }
            }
            let mut verdict = false;
            if let [a, b] = shared[..] {
                for v in a.iter().filter(|v| b.contains(v)) {
                    if self.redundant_subdivision_vertex(*v, band)? {
                        verdict = true;
                    }
                }
            }
            verdict
        };
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
                self.ring_move(ring, rep)?;
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
            self.kef(dying_he)?;
            group.absorbed.push(other);
            group.killed_edges.push(edge_key);
        }
        // Intra-face duplicates: edges now occurring twice within the
        // survivor's loops. On a PLANAR survivor a same-loop duplicate
        // bounds a genuine hole and `kemr` mints the ring. On a CURVED
        // survivor (C12.5, M5 PR 9) a same-face duplicate means the
        // cosurface run CLOSED THE FULL PERIOD — a shape outside the
        // merge's inventory at M5 (neither the ring form nor the
        // kept-cut seam form is integrable by the exact-B-rep props
        // yet), refused typed here; the driver records it as a LOUD
        // skip for curved structural runs, so sub-period re-merges
        // (the C12.5 through-cut case) proceed and full closures stay
        // unmerged exactly as the operands arrived.
        loop {
            let mut found = None;
            for (edge_key, edge) in self.edges() {
                let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
                if hp.face != rep || hm.face != rep {
                    continue;
                }
                found = Some((edge_key, (edge.he_plus, hp), (edge.he_minus, hm)));
                break;
            }
            let Some((edge_key, (he_plus, hp), (he_minus, hm))) = found else {
                break;
            };
            let same_loop = hp.r#loop == hm.r#loop;
            if kind == MergeKind::Curved {
                return Err(MergeCoplanarError::PeriodClosure { edge: edge_key });
            }
            if !same_loop {
                return Err(MergeCoplanarError::UnsupportedConfiguration { edge: edge_key });
            }
            // **A straight seam's surviving duplicate is a STRUT, not a
            // ring.** The absorption above killed one of the two seam
            // edges with `kef`; the other is now a duplicate inside the
            // survivor whose junction end is left with valence 1 — a
            // dangling remnant the merge itself created, enclosing
            // nothing. `kemr` would mint a ring from it and the winding
            // pass would then find no unique positive cycle and refuse
            // `MergedFaceRoleAmbiguous`, which is the dead end this op
            // hits on every revolve cap.
            //
            // `kev` is the op for it — it kills the strut AND the far
            // vertex, leaving the face bounded by its outline alone.
            // The licence is that the removed vertex was interior to
            // one straight carrier, so the union of the two collinear
            // pieces is the same locus: geometry-preserving by
            // construction, which is why this is gated on
            // `straight_seam` and not on the strut's shape alone.
            //
            // A BROKEN orbit is announced, not read as "no tip":
            // `kev` and `kemr` are different operators with different
            // Euler deltas, so letting a torn arena answer this
            // question silently chooses which surgery runs.
            let mut tip = None;
            if straight_seam {
                // `vertex_orbit` walks the halves STARTING at its
                // argument's start vertex, so each candidate asks
                // about the far end of the other half; the start
                // vertex is the one `kev` takes with the strut.
                for (from_rim, toward, killed) in
                    [(he_plus, he_minus, hm.start), (he_minus, he_plus, hp.start)]
                {
                    if self.strut_tip(toward)? {
                        tip = Some((from_rim, killed));
                        break;
                    }
                }
            }
            if let Some((from_rim, killed)) = tip {
                // The fact `kev` could contradict, re-read at the
                // call: `strut_tip` answered about a vertex orbit,
                // and what `kev` refuses on is the derived fact that
                // the edge's two ends are distinct.
                debug_assert!(
                    self.get_half_edge(from_rim)
                        .zip(self.edge_mate(from_rim, edge_key))
                        .is_none_or(|(rim, mate)| rim.start != mate.start),
                    "merge_group: {}",
                    EstablishedFact::StrutHalvesHaveDistinctEnds.what()
                );
                #[cfg(test)]
                tear_before_kev(self, from_rim, edge_key);
                self.kev(from_rim)?;
                group.killed_edges.push(edge_key);
                group.killed_vertices.push(killed);
                continue;
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
        // Role normalization (M5 S1 fix pass, review MAJOR-1): the
        // intra-face `kemr` above designates its ring PROVISIONALLY
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
        // elsewhere; no unique positive cycle refuses typed.
        if !group.rings_made.is_empty() {
            // The survivor is resolved HERE, where its liveness is
            // proven in this call: `rings_made` is non-empty only
            // because a `kemr` returned, and `kemr` requires the
            // duplicate's loop's face — `rep`, the face both halves
            // were found on through that loop — to be live
            // (`require_key` on the loop's `face`) and kills no face.
            // The role pass then takes resolved data and performs no
            // lookup of its own.
            let Some(survivor) = self.get_face(rep) else {
                unreachable!(
                    "merge_group: `rep` was required live by the `kemr` that minted the last \
                     ring, and `kemr` kills no face"
                )
            };
            let survivor = survivor.clone();
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

    /// The signed winding of a cycle loop around `normal`, through the
    /// reified `bool_ring_run_winding` predicate (the plane's Newell
    /// functional — twice the enclosed signed area; the same margin
    /// the boolean join's ring lane decides on). `None` for empty
    /// loops (a lone-vertex ring bounds no area and stays a ring).
    ///
    /// Dimension (audit F4): the Newell area is metered to a LENGTH by
    /// the loop's own perimeter — `2A/P`, the region's mean width. The
    /// derivation, and why this predicate must state it identically at
    /// all three of its sites, is in `boolean::join::ring_run_ccw`.
    ///
    /// # The carriers this answers about
    ///
    /// Line, Circle and Ellipse. A cycle carrying a NURBS edge returns
    /// `None` — the honest remainder: a chord winding says nothing
    /// about a fitted carrier's region and no closed form exists for
    /// it, so the caller refuses rather than guesses.
    ///
    /// For the conic carriers the enclosed vector area decomposes
    /// EXACTLY, per edge — this is a substitution, not an
    /// approximation:
    ///
    /// ```text
    ///   2·A⃗ = Σ_edges       (p_prev − p₀) × (p − p₀)      [chord Newell]
    ///        + Σ_conic-edges axis · sa·sb · (Δ − sin Δ)    [bulge]
    /// ```
    ///
    /// A circular arc of radius `R` spanning signed angle `Δ` cuts off
    /// a circular segment of area `R²(Δ − sin Δ)/2` between itself and
    /// its chord; twice that is `R²(Δ − sin Δ)`, which is the
    /// cross-sum's own `2A` convention, and it is ODD in `Δ` — so it
    /// carries the traversal sign the winding question is about. The
    /// ellipse is the circle's affine image, which scales every area by
    /// `major·minor/R²`, giving `sa·sb`. The chord term is untouched
    /// for every edge, so the bulge is a CORRECTION on a chord polygon
    /// and a mixed Line+Circle cycle needs no case split beyond the
    /// per-edge carrier match.
    ///
    /// The perimeter lever moves with the area (the same F4 metering
    /// statement): a conic edge contributes `|Δ|·sa` — the circle's
    /// exact arc length, the ellipse's upper bound, and an over-large
    /// `P` understates the width, i.e. escalates rather than decides.
    ///
    /// A LINE-ONLY cycle is decided bit-identically to before this arm
    /// existed: the correction block below is structurally skipped, not
    /// zero-added into a reordered sum.
    ///
    /// Behaviour change riding with that metering (the unit's
    /// deviation 1, SECOND site — the join lane's zero-perimeter note
    /// has the same shape): a cycle whose perimeter is exactly zero —
    /// every vertex coincident — now divides `0/0`, poisons, and
    /// escalates typed, where it previously answered `Some(Zero)` and
    /// let `normalize_merged_roles` read it as "not the positively-wound
    /// cycle". Empty loops still return `None` earlier, so reaching this
    /// needs a real cycle of coincident points. The fail-loud direction
    /// is deliberate: a loop with no extent has no winding to report,
    /// and refusing typed beats handing back a role decision derived
    /// from an area and a perimeter that are both nothing.
    ///
    /// `normal` must be the face's OUTWARD normal (S10): the caller
    /// folds the sense into the chart normal exactly once, through
    /// `face_normal`'s door, and the Newell sum here is left alone.
    /// That sum is built from the
    /// loop's STORED cycle order, which `revert` reverses in the same
    /// breath as it flips the sense bit, so it already changes sign on
    /// its own — threading the sense onto both factors would cancel
    /// and leave the outer/ring roles as wrong as threading neither.
    fn loop_winding(
        &self,
        l: LoopKey,
        normal: geom_core::Vec3<T>,
        band: Band,
    ) -> Result<Option<geom_core::Sign>, MergeCoplanarError> {
        let corrupt = || MergeCoplanarError::Op {
            error: EulerOpError::StaleKey {
                key: EntityId::Loop(l),
            },
        };
        let crate::entity::LoopBoundary::Cycle { first } =
            self.get_loop(l).ok_or_else(corrupt)?.boundary
        else {
            return Ok(None);
        };
        let cycle = self.loop_cycle(first).ok_or_else(corrupt)?;
        // What this half-edge's carrier is to the winding sum: a chord,
        // a chord plus a closed-form bulge, or nothing this can answer
        // about. `None` when anything on the way to the carrier fails
        // to resolve — a torn half-edge, edge or curve leaves the cycle
        // undecidable exactly as it did when the guard was line-only.
        enum Carrier {
            Line,
            Conic,
        }
        let carrier_of = |he| {
            self.get_half_edge(he)
                .and_then(|hd| self.get_edge(hd.edge))
                .and_then(|e| self.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .and_then(|c| match c.carrier() {
                    geom::Curve3::Line { .. } => Some(Carrier::Line),
                    geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
                        Some(Carrier::Conic)
                    }
                    // A spiric is the honest remainder as a spline is:
                    // its region has no conic-bulge winding here.
                    geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
                })
        };
        // A NURBS edge is the honest remainder — its region has no
        // closed-form area and its chord winding is not that region's,
        // so the cycle stays undecidable (`None`; the caller then
        // refuses rather than guesses if roles hinge on it).
        if cycle.iter().any(|&he| carrier_of(he).is_none()) {
            return Ok(None);
        }
        // Whether the chord polygon IS the region: on a line-only cycle
        // the correction block below never runs, which is what makes
        // every pre-existing decision bit-identical.
        let all_lines = cycle
            .iter()
            .all(|&he| matches!(carrier_of(he), Some(Carrier::Line)));
        let point_of = |he| -> Result<geom_core::Point3<T>, MergeCoplanarError> {
            let v = self.get_half_edge(he).ok_or_else(corrupt)?.start;
            self.get_vertex(v)
                .and_then(|vd| self.get_point(vd.point).copied())
                .ok_or_else(corrupt)
        };
        let p0 = point_of(cycle[0])?;
        let mut newell = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
        // The F4 metering lever: this cycle's perimeter, accumulated
        // with the area (chords here; the conic arm re-meters below).
        let mut perimeter = T::zero();
        let mut prev = p0;
        for &he in &cycle[1..] {
            let p = point_of(he)?;
            newell = newell + (prev - p0).cross(p - p0);
            perimeter = perimeter + (p - prev).norm();
            prev = p;
        }
        perimeter = perimeter + (p0 - prev).norm();
        // The arc correction (fn docs), stated as
        // `boolean::join::ring_run_ccw`'s `run_term` states it: one
        // curve lookup yields both the vector area between an arc and
        // its chord and the half-edge's own boundary length. It runs
        // only when some carrier is a conic, so a line-only cycle keeps
        // the arithmetic and the accumulation order it had.
        if !all_lines {
            let end_point_of = |he| -> Result<geom_core::Point3<T>, MergeCoplanarError> {
                let v = self.half_edge_end(he).ok_or_else(corrupt)?;
                self.get_vertex(v)
                    .and_then(|vd| self.get_point(vd.point).copied())
                    .ok_or_else(corrupt)
            };
            let zero = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
            // `(bulge, boundary length)`. Every lookup here already
            // resolved for `carrier_of` above; announcing rather than
            // discarding is what keeps a body torn under us from
            // answering a role question anyway. That is the third
            // divergence from `run_term`, which degrades a failed
            // lookup to a chord — stricter here, deliberately: this
            // site's answer decides a ROLE, and a role derived from a
            // silently-shortened boundary is the silent-corrupt-export
            // class the winding pass exists to close.
            let arc_term = |he| -> Result<(geom_core::Vec3<T>, T), MergeCoplanarError> {
                let chord = || -> Result<T, MergeCoplanarError> {
                    Ok((end_point_of(he)? - point_of(he)?).norm())
                };
                let edge = self
                    .get_half_edge(he)
                    .and_then(|hd| self.get_edge(hd.edge))
                    .ok_or_else(corrupt)?;
                let curve = self
                    .get_curve_geom(edge.curve)
                    .and_then(crate::null::CurveGeom::certified)
                    .ok_or_else(corrupt)?;
                let (t0, t1) = curve.params();
                let (axis, sa, sb) = match *curve.carrier() {
                    geom::Curve3::Circle { axis, radius, .. } => (axis, radius, radius),
                    geom::Curve3::Ellipse {
                        axis, major, minor, ..
                    } => (axis, major, minor),
                    geom::Curve3::Line { .. }
                    | geom::Curve3::Spiric { .. }
                    | geom::Curve3::Nurbs(_) => {
                        return Ok((zero, chord()?));
                    }
                };
                // Signed by traversal: the half-edge runs with
                // increasing carrier parameter iff it is the plus
                // half. `|Δ|·sa` is the circle's exact arc length
                // and the ellipse's upper bound.
                let span = if edge.he_plus == he { t1 - t0 } else { t0 - t1 };
                Ok((axis * (sa * sb * (span - span.sin())), span.abs() * sa))
            };
            let mut bulge = zero;
            let mut metered = T::zero();
            for &he in &cycle {
                let (b, len) = arc_term(he)?;
                bulge = bulge + b;
                metered = metered + len;
            }
            newell = newell + bulge;
            perimeter = metered;
        }
        match crate::validate::decide(
            "bool_ring_run_winding",
            Margin::over_lever(normal.dot(newell), perimeter),
            band,
        ) {
            Ok(sign) => Ok(Some(sign)),
            Err(diag) => Err(MergeCoplanarError::Escalated { diag }),
        }
    }

    /// Which of the merged survivor's rings is its outline, decided by
    /// winding (doc at the call site): the unique positively-wound
    /// cycle is the outer loop. `Some(i)` means ring `i` must swap into
    /// the outer slot; `None` means the roles are already correct, or
    /// the survivor is not a planar rung. Zero or multiple positive
    /// cycles refuse [`MergeCoplanarError::MergedFaceRoleAmbiguous`].
    ///
    /// Takes the survivor's resolved data rather than looking it up:
    /// the caller holds the liveness proof, and a helper that cannot
    /// look anything up cannot discard a failed lookup. `face` is the
    /// refusal's payload only — nothing here dereferences it.
    fn merged_outline_ring(
        &self,
        face: FaceKey,
        survivor: &crate::entity::Face,
        tol: Tol,
    ) -> Result<Option<usize>, MergeCoplanarError> {
        let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
        // The face's OUTWARD normal (S10): "positively wound" means
        // CCW seen from OUTSIDE the material, so on a reversed face
        // the chart normal names the opposite convention and every
        // role assignment below would come out inverted.
        let normal = match self.get_surface(survivor.surface) {
            Some(Surface::Plane { normal, .. }) => plane_outward_normal(survivor, *normal).vec(),
            // The survivor is a plane at every call: `rings_made` is
            // non-empty only under a planar contract, since the curved
            // arm refuses `PeriodClosure` before any `kemr`. This arm
            // still spells "not a plane" and "unresolved" the same way.
            _ => return Ok(None),
        };
        let mut positives: Vec<Option<usize>> = Vec::new(); // None = outer
        if self.loop_winding(survivor.outer, normal, band)? == Some(geom_core::Sign::Positive) {
            positives.push(None);
        }
        for (i, &r) in survivor.rings.iter().enumerate() {
            if self.loop_winding(r, normal, band)? == Some(geom_core::Sign::Positive) {
                positives.push(Some(i));
            }
        }
        match positives[..] {
            [None] => Ok(None), // roles already correct
            [Some(i)] => Ok(Some(i)),
            _ => Err(MergeCoplanarError::MergedFaceRoleAmbiguous { face }),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::test_support_fixtures::declined_cube;

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
    /// planar, licensed only by the declaration, so the group runs
    /// under [`GroupRegime::RecordsASkip`] without depending on any
    /// face being curved.
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

    /// The declared planar cube: [`GroupRegime::RecordsASkip`] with
    /// no curved face anywhere.
    fn declared_planar_cube(tol: Tol) -> (Body<f64>, Vec<(SurfaceKey, SurfaceKey)>) {
        let mut body = declined_cube::<f64>(tol).body;
        let declared = declare_planes_pairwise(&mut body);
        (body, declared)
    }

    /// The group's contract, asked the way the door asks it: every
    /// face of these fixtures carries a declared key, so the door's
    /// own `declared_faces` set is the whole body.
    fn contract_of(body: &Body<f64>, declared: bool) -> GroupContract {
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let declared_faces: std::collections::BTreeSet<FaceKey> = if declared {
            faces.iter().copied().collect()
        } else {
            std::collections::BTreeSet::new()
        };
        let kinds = body.kind_census().expect("the fixture's faces resolve");
        Body::<f64>::group_contract(faces[0], &faces[1..], &kinds, &declared_faces)
            .expect("the census holds every face")
    }

    /// A planar run's contract under `regime`.
    fn planar(regime: GroupRegime) -> GroupContract {
        GroupContract::Runs {
            regime,
            kind: MergeKind::Plane,
        }
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

    /// **The two planar fixtures take the two regimes**, so every row
    /// below states which regime it ran under instead of inheriting
    /// one.
    #[test]
    fn the_planar_fixtures_take_the_two_regimes() {
        let tol = Tol::witness();
        assert_eq!(
            contract_of(&structural_planar_cube(tol), false),
            planar(GroupRegime::RefusesTheCall)
        );
        let (declared_body, declared) = declared_planar_cube(tol);
        assert_eq!(
            contract_of(&declared_body, true),
            planar(GroupRegime::RecordsASkip)
        );
        // ...and the recording fixture reaches the surgery untorn.
        let mut untorn = declared_body;
        let outcome = untorn.merge_coplanar_faces_declared(&declared, tol);
        assert!(outcome.is_ok(), "{outcome:?}");
    }

    /// The fixture a tear point needs, given a plane recipe: the two
    /// `ring_move` tears need a group whose absorption has a ring to
    /// re-home, which a bare cube has not.
    fn fixture_for(point: TearPoint, tol: Tol) -> Body<f64> {
        match point {
            TearPoint::RingBecomesItsFacesOuter | TearPoint::RingsFaceLeavesTheShell => {
                crate::fixtures::ops_holed_box(tol).body
            }
            _ => declined_cube::<f64>(tol).body,
        }
    }

    /// The four facts whose tear reaches its operator on these
    /// fixtures, each with the [`EstablishedFact`] the re-check
    /// before that call proves.
    ///
    /// [`EstablishedFact::StrutHalvesHaveDistinctEnds`] is not among
    /// them and is ARGUED, not executed: `kev` runs only on a
    /// straight-seam strut, which needs a group whose two shared-edge
    /// runs meet at a redundant subdivision vertex, and no fixture in
    /// the crate's corpus both reaches that repair and admits a plane
    /// recipe. Its re-check stands at the call like the others; what
    /// is missing is a body that gets there, and saying so is the
    /// honest state of the row rather than a claim of coverage.
    const EXECUTED_FACTS: [(TearPoint, EstablishedFact); 5] = [
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
        (
            TearPoint::DuplicateHalvesPartCompany,
            EstablishedFact::DuplicateHalvesShareALoop,
        ),
    ];

    /// **A refusal that contradicts a re-checked fact escapes under
    /// the RECORDING regime.** One row-body per [`EstablishedFact`]
    /// the surgery re-checks and a fixture can reach, driven through
    /// the PUBLIC door with the matching tear point armed.
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
        for (point, want) in EXECUTED_FACTS {
            let mut body = fixture_for(point, tol);
            let declared = declare_planes_pairwise(&mut body);
            assert_eq!(
                contract_of(&body, true),
                planar(GroupRegime::RecordsASkip),
                "{point:?}"
            );
            let error = escaped_refusal(point, &mut body, &declared, tol);
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

    /// The same tears under [`GroupRegime::RefusesTheCall`], where
    /// the escape changes no outcome and the row's value is that the
    /// refusal still names the operator's own variant.
    #[test]
    fn every_contradicted_fact_refuses_the_refusing_regime() {
        let tol = Tol::witness();
        for (point, want) in EXECUTED_FACTS {
            let mut body = fixture_for(point, tol);
            describe_shared_key(&mut body);
            assert_eq!(
                contract_of(&body, false),
                planar(GroupRegime::RefusesTheCall),
                "{point:?}"
            );
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
        let pt = geom_core::Point3::new;
        let crate::test_support_fixtures::CubeOps {
            mut body,
            seed,
            mefs,
            ..
        } = declined_cube::<f64>(tol);
        let strut = |body: &mut Body<f64>, at, x, y, z| {
            body.mev_line(
                crate::euler::MevSite::Fan { he1: at, he2: at },
                pt(x, y, z),
                tol,
            )
            .expect("the fan strut grows")
        };
        let hole_strut = strut(&mut body, mefs[1].he_plus, 0.25, 0.25, 1.0);
        let kill = body
            .kemr(hole_strut.he_plus, hole_strut.he_minus)
            .expect("the strut becomes a lone-vertex ring");
        let s_pq = body
            .mev_line(
                crate::euler::MevSite::Lone { r#loop: kill.ring },
                pt(0.75, 0.25, 1.0),
                tol,
            )
            .expect("the ring grows its first edge");
        let s_qr = strut(&mut body, s_pq.he_minus, 0.75, 0.75, 1.0);
        let s_rs = strut(&mut body, s_qr.he_minus, 0.25, 0.75, 1.0);
        let membrane = body
            .mef_chord(
                crate::euler::MefSite::Chords {
                    he1: s_pq.he_plus,
                    he2: s_rs.he_minus,
                },
                tol,
            )
            .expect("the rim closes into a membrane face");
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

    /// The same nesting with the inner face arena-FIRST, so the
    /// door's own group seed picks it as the survivor, and the group
    /// licensed by declared pairs so it runs under the recording
    /// regime: the public door records the refusal and returns `Ok`,
    /// which is the outcome an escape would have taken away.
    #[test]
    fn the_door_records_same_face_as_a_skip() {
        let tol = Tol::witness();
        let (mut body, membrane) = cube_with_arena_first_membrane(tol);
        let declared = declare_planes_pairwise(&mut body);
        assert_eq!(validate_closed(&body), Ok(()));
        assert_eq!(contract_of(&body, true), planar(GroupRegime::RecordsASkip));
        let outcome = body
            .merge_coplanar_faces_declared(&declared, tol)
            .expect("a legal nested group is recorded, not refused");
        let [skipped] = &outcome.skipped[..] else {
            panic!("one recorded skip: {:?}", outcome.skipped)
        };
        assert!(
            matches!(
                skipped.reason,
                MergeCoplanarError::Op {
                    error: EulerOpError::SameFace { .. }
                }
            ),
            "{:?}",
            skipped.reason
        );
        let _ = membrane;
    }

    /// The membrane fixture with the inner face arena-first: a `kef`
    /// before the rim is grown frees the seed face's slot, which the
    /// membrane's `add_face` then reuses.
    fn cube_with_arena_first_membrane(tol: Tol) -> (Body<f64>, FaceKey) {
        let pt = geom_core::Point3::new;
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
        let strut = |body: &mut Body<f64>, at, x, y, z| {
            body.mev_line(
                crate::euler::MevSite::Fan { he1: at, he2: at },
                pt(x, y, z),
                tol,
            )
            .expect("the fan strut grows")
        };
        let hole_strut = strut(&mut body, host_he, 0.25, 0.25, 1.0);
        let kill = body
            .kemr(hole_strut.he_plus, hole_strut.he_minus)
            .expect("the strut becomes a lone-vertex ring");
        let s_pq = body
            .mev_line(
                crate::euler::MevSite::Lone { r#loop: kill.ring },
                pt(0.75, 0.25, 1.0),
                tol,
            )
            .expect("the ring grows its first edge");
        let s_qr = strut(&mut body, s_pq.he_minus, 0.75, 0.75, 1.0);
        let s_rs = strut(&mut body, s_qr.he_minus, 0.25, 0.75, 1.0);
        let membrane = body
            .mef_chord(
                crate::euler::MefSite::Chords {
                    he1: s_pq.he_plus,
                    he2: s_rs.he_minus,
                },
                tol,
            )
            .expect("the rim closes into a membrane face");
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
    /// — routing the seam repair from `kev` to `kemr` and changing
    /// the group's Euler delta on a torn arena.
    ///
    /// It pins `strut_tip` rather than the surgery: reaching the tip
    /// search with a broken orbit needs the arena torn BETWEEN the
    /// straight-seam decision and the strut test, and the surgery
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
        let none = std::collections::BTreeSet::new();
        let contract = |body: &Body<f64>, a: FaceKey, b: FaceKey| {
            let kinds = body.kind_census().expect("the fixture's faces resolve");
            Body::<f64>::group_contract(a, &[b], &kinds, &none)
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
        assert_eq!(contract_of(&body, false), GroupContract::SetAside);
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
        let pt = geom_core::Point3::new;
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
            .expect("a clamped linear knot vector");
        let net = geom::NurbsSurface::new(
            kv.clone(),
            kv,
            (0..4)
                .map(|i| pt(f64::NAN, f64::from(i % 2), f64::from(i / 2)))
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
        body.set_face_surface(face, crate::euler::FaceSurface::New(poisoned_net()))
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
        body.set_face_surface(first, crate::euler::FaceSurface::New(poisoned_net()))
            .expect("a live face takes a surface");
        body.set_face_surface(
            second,
            crate::euler::FaceSurface::Shared(surface_of(&body, first)),
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
        let key = body.faces().next().expect("faces").1.surface;
        *body
            .surfaces
            .get_mut(key)
            .expect("the placeholder resolves") = unit_cylinder();
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
    /// together.** The declared nested cube with one side face — not
    /// the ringed top, not the membrane — put back on a placeholder:
    /// the nested group records its `SameFace` skip as before, the
    /// placeholder is named, and neither record swallows the other.
    #[test]
    fn an_ok_carries_a_recorded_skip_beside_the_placeholder_census() {
        let tol = Tol::witness();
        let (mut body, membrane) = cube_with_arena_first_membrane(tol);
        let mut declared = declare_planes_pairwise(&mut body);
        let side = body
            .faces()
            .find(|&(k, f)| k != membrane && f.rings.is_empty())
            .map(|(k, _)| k)
            .expect("a side face outside the nesting");
        let old_key = surface_of(&body, side);
        body.set_face_surface(
            side,
            crate::euler::FaceSurface::New(Surface::nurbs_placeholder()),
        )
        .expect("a live face takes a surface");
        declared.retain(|&(a, b)| a != old_key && b != old_key);
        let outcome = body
            .merge_coplanar_faces_declared(&declared, tol)
            .expect("a recorded skip beside a set-aside face is not a refusal");
        assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
        let [skipped] = &outcome.skipped[..] else {
            panic!("one recorded skip: {:?}", outcome.skipped)
        };
        assert!(
            matches!(
                skipped.reason,
                MergeCoplanarError::Op {
                    error: EulerOpError::SameFace { .. }
                }
            ),
            "{:?}",
            skipped.reason
        );
        assert!(!skipped.faces.contains(&side), "{:?}", skipped.faces);
        assert_eq!(outcome.placeholders, vec![side]);
    }

    /// [`crate::test_support_fixtures::declined_cube`] with its arena-first face re-described as a real
    /// plane on its OWN key, the other five still on the shared
    /// placeholder.
    fn cube_with_one_described_face(tol: Tol) -> (Body<f64>, FaceKey) {
        let mut body = declined_cube::<f64>(tol).body;
        let face = body.faces().next().expect("a cube has faces").0;
        body.set_face_surface(face, crate::euler::FaceSurface::New(flat_plane()))
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
            body.set_surface_source(surface_of(&body, face), source.clone())
                .expect("a live key takes a source");
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

    /// The two-face digon pillow — two vertices, two chord edges —
    /// with its split face on a real plane and its seed face left on
    /// the placeholder, each on its own key: a placeholder cap.
    fn pillow_with_a_placeholder_cap(tol: Tol) -> (Body<f64>, FaceKey, FaceKey) {
        let pt = geom_core::Point3::new;
        let mut body = Body::<f64>::new();
        let seed = body
            .mvfs(pt(0.0, 0.0, 0.0))
            .expect("mvfs has no preconditions");
        let seg = body
            .mev_line(
                crate::euler::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                pt(1.0, 0.0, 0.0),
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
        body.set_face_surface(split.face, crate::euler::FaceSurface::New(flat_plane()))
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
            body.set_surface_source(surface_of(&body, face), source.clone())
                .expect("a live key takes a source");
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
        assert!(
            rendered(&MergeCoplanarError::DeclarationContradicted {
                diag: Indeterminate {
                    margin: geom_core::MarginDiag::Value(0.0),
                    band: Band::linear(Tol::witness()).expect("the witness band"),
                    predicate: Some("merge_declared_plane_eq"),
                },
            })
            .contains("fix the declaration or the geometry"),
            "the declared-pair contradiction carries its recourse"
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
        let seed = body.mvfs(a).unwrap();
        let surface = body
            .set_face_surface(seed.face, FaceSurface::New(plane()))
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
        body.set_face_surface(new.face, FaceSurface::New(plane()))
            .unwrap();
        let r#loop = body.get_face(seed.face).unwrap().outer;
        let ab = body_edge(&body, e_ab.he_plus);
        Tri {
            body,
            r#loop,
            surface,
            ab,
        }
    }

    fn body_edge(body: &Body<f64>, he: crate::HalfEdgeKey) -> crate::entity::EdgeKey {
        body.get_half_edge(he).unwrap().edge
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
            t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol)),
            Ok(Some(Sign::Positive)),
            "the chord triangle winds positively about +z"
        );
        assert_eq!(
            t.body.loop_winding(t.r#loop, -Vec3::unit_z(), band(tol)),
            Ok(Some(Sign::Negative)),
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
        let (a, b) = (Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0));
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(a).unwrap();
        let surface = body
            .set_face_surface(seed.face, FaceSurface::New(plane()))
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
                FaceSurface::New(plane()),
                tol,
            )
            .unwrap();
        let outward = Vec3::unit_z();
        let disc = body.get_face(e2.face).unwrap().outer;
        let anti = body.get_face(seed.face).unwrap().outer;
        assert_eq!(
            body.loop_winding(disc, outward, band(tol)),
            Ok(Some(Sign::Positive)),
            "the disc's own boundary encloses material about the outward normal"
        );
        assert_eq!(
            body.loop_winding(anti, outward, band(tol)),
            Ok(Some(Sign::Negative)),
            "the same boundary reversed anti-encloses — a ring's signature"
        );
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
            t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol)),
            Ok(Some(Sign::Positive)),
            "chord 2A = 1 minus the bite the arc takes out of it"
        );
    }

    /// The margin an escalation carries back, as an `f64`.
    ///
    /// **This is the module's one test-visible seam onto the deciding
    /// scalar.** `loop_winding` hands back a [`Sign`] and nothing else
    /// when it decides, so `2A/P` is unobservable from outside on the
    /// deciding path — but an IN-BAND margin escalates typed, and
    /// [`geom_core::MarginDiag::Value`] then carries the exact quantity
    /// that was classified. The two rows below aim their fixtures into
    /// that band deliberately: it is the only place the numerator and
    /// the DENOMINATOR can both be pinned, and the denominator — the
    /// re-metered arc-length perimeter — is otherwise invisible to any
    /// sign assertion, because scaling a lever cannot change a sign.
    fn escalated_margin(r: Result<Option<Sign>, MergeCoplanarError>) -> f64 {
        match r {
            Err(MergeCoplanarError::Escalated { diag }) => match diag.margin {
                geom_core::MarginDiag::Value(v) => v,
                other => panic!("expected a classified f64 margin, got {other:?}"),
            },
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
            t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol)),
            Ok(Some(Sign::Positive)),
            "as a chord triangle it is decidable and positive"
        );
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
        assert_eq!(
            t.body.loop_winding(t.r#loop, Vec3::unit_z(), band(tol)),
            Ok(None),
            "one fitted carrier and the whole cycle stops being answerable"
        );
    }
}
