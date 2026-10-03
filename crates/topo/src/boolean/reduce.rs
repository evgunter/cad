//! The reduction sweep (ch. 15 §15.5, Programs 15.2–15.4 re-derived):
//! all-pairs edge×face in BOTH directions, realizing the eight-step
//! specification in one sweep with `contfp`/`contfv` typed case codes.
//!
//! - **Candidate generation through the `bvh` tree** (M5 PR 8, C10 —
//!   the documented quadratic of M3, retired): each edge fragment
//!   queries the per-direction face tree ([`super::boxes`], padded
//!   vertex-extent boxes) instead of scanning every face. THE TREE
//!   PRUNES, PREDICATES DECIDE: candidates arrive in ascending face
//!   arena order (a subsequence of the brute-force scan), the exact
//!   per-pair classification below is untouched, and the conservative
//!   pad guarantees every pair the exact predicates would accept
//!   survives — so results are bit-identical to the brute-force sweep
//!   by construction, and the idealized/realized differential suite
//!   (PERF-PLAN §4.4; `tests/m5_pr8_bvh_diff.rs`, the corpus suite in
//!   editor-core) pins it: realized candidates ⊇ idealized accepted
//!   pairs, final results bit-equal, planted degradation caught. The
//!   brute-force scan survives as [`SweepStrategy::Idealized`] — the
//!   ten-line definition of the candidate set. One documented
//!   divergence, error channel only: a pair whose boxes are disjoint
//!   can still ESCALATE the brute path's `bool_vertex_face_side` when
//!   an edge grazes a face's *infinite* plane far from the face
//!   itself; the realized path never examines it. Pruning can drop
//!   only such spurious escalations, never an accepted event — the
//!   value channel is pinned bit-equal. In the full boolean the
//!   disjoint-operands containment witness passes over a point that
//!   reads in-band against the same plane (`super::shell_witness`), so
//!   the realized path answers wherever another witness of the shell
//!   decides, and refuses where none does; the suite's grazing fixture
//!   pins one that answers.
//! - **Worklist, not recursion** (Problem 15.3 / F12): a proper
//!   crossing splits the edge through the certified `split_edge` lane
//!   and pushes BOTH children back with the *next* face index (a line
//!   crosses a plane at most once, so the split face is done with both
//!   children); each split strictly shortens spans — termination is
//!   structural.
//! - **Coplanar edge-face pairs are skipped** (a line with both
//!   endpoints ON the face plane, or a conic lying in it ⇒ endpoint
//!   processing only): every relevant crossing inside the face is
//!   caught when the edge is swept against the face's noncoplanar
//!   NEIGHBOR faces, where the crossing point lands ON the shared
//!   boundary edge (the `OnEdge` case — tested by the coplanar-overlap
//!   acceptance fixture).
//! - **Edge-on-edge crossings** are discovered as edge-face events
//!   landing ON an edge of the face: BOTH edges are split at the
//!   (bitwise-shared) intersection point — the minted vertices are a
//!   declared v-v contact pair by construction.
//! - Sweep order (D9): direction A→B fully, then B→A; edges in arena
//!   order, faces in arena snapshot order, worklist FIFO. Then the
//!   settle stage ([`settle_deferred`]): the covered touches a direction
//!   deferred are read again on the fragments both directions left,
//!   in deferral order ([`sweep_and_settle`] runs all three).
//!
//! The module also hosts the three **pre-sweep gates**, which refuse an
//! operand pair before any edge is split: the operand gate
//! ([`gate_operand_pairs`]: each operand passes tier 2, and a kind
//! with no wired arm may not enter an undeclared pair), the
//! maximal-faces gate ([`gate_maximal_faces`]:
//! no operand carries two coplanar neighbours), and the
//! undeclared-continuation scan ([`refuse_undeclared_continuations`]:
//! no aligned one-carrier pair meets without a declaration).

use geom_core::{Band, Bounds, Decide, Margin, Point3, Sign};

use super::boxes;
use super::circle_roots::CircleRoots;
use super::contain::{ContainError, CurvedPlacement, FaceContainment, contfp};
use super::plane_eq::{LadderRefusal, PlaneDesc};
use super::refusal_routes::NeighbourOffset;
use super::{BooleanDecision, Coincide, CrossingDecision, DeclarationRead};
use super::{BooleanError, ContactRecords, Operand, VfContact, VvContact};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::splitting::{ConicPlaneMeet, PlaneCrossingLane};
use crate::validate::decide;
use geom_core::Tol;

/// Which candidate-generation path the reduction sweep runs — the
/// idealized/realized pair of PERF-PLAN §4.4 (the pattern is only
/// permitted WITH its differential suite; see the module docs).
/// Production entries always run [`SweepStrategy::Realized`]; the
/// idealized path is the executable definition of the candidate set,
/// kept alive for the suite's pins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepStrategy {
    /// BVH-pruned candidate generation (the production path).
    Realized,
    /// Brute-force all-pairs (the reference definition).
    ///
    /// `sweep-testing` feature only. The idealized half of a §4.4 pair
    /// is reference surface, exactly like [`PlantedDegradation`] and
    /// [`super::sweep_traces`] — it exists so the differential suite
    /// can execute the definition, not so a production caller can
    /// choose O(n²) candidate generation. Gating the variant is what
    /// makes "production entries always run [`SweepStrategy::Realized`]"
    /// a fact the compiler enforces rather than a convention; with the
    /// feature off the brute-force scan is not merely unreachable, it
    /// is not built (see `sweep_direction`).
    #[cfg(feature = "sweep-testing")]
    Idealized,
}

/// One direction's sweep observations, for the differential suite's
/// superset pin: `examined` = pairs whose exact classification ran
/// (the candidate set), `accepted` = pairs where the exact predicates
/// accepted at least one event (a crossing inside the face, or an
/// endpoint contact). Pairs are `(edge of x, face of y)` in
/// examination order.
#[derive(Debug, Default, Clone)]
pub struct SweepTrace {
    /// Every candidate pair the exact path examined.
    pub examined: Vec<(EdgeKey, FaceKey)>,
    /// The subset of pairs that produced an accepted event.
    pub accepted: Vec<(EdgeKey, FaceKey)>,
}

/// The suite's failure-injection seam (pin iii — "the suite must be
/// able to fail"): shrink ONE face's box to the poison-free EMPTY box
/// before building the tree, so candidate generation loses whatever
/// events that face carries and the superset pin must catch it.
/// `sweep-testing` feature only — no production consumer can name a
/// failure injector (M5 PR 8 fix pass, item 2).
#[cfg(feature = "sweep-testing")]
#[derive(Debug, Clone, Copy)]
pub struct PlantedDegradation {
    /// The face whose box is planted empty.
    pub face: FaceKey,
}

/// Internal candidate-generation knobs (private plumbing; the PUBLIC
/// doors that can set anything non-default are `sweep-testing`-gated).
/// Production entries always pass `SweepKnobs::default()`.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct SweepKnobs {
    /// Pin (iii): plant this face's box empty.
    pub(super) plant: Option<FaceKey>,
    /// Pin 1(b): override [`boxes::sweep_pad`] (a DELIBERATELY
    /// breakable knob — the suite proves a too-small pad is caught).
    pub(super) pad_override: Option<f64>,
}

/// The (deduplicating, order-preserving) contact accumulator.
#[derive(Default)]
pub(super) struct ContactAcc {
    records: ContactRecords,
    seen_vv: std::collections::BTreeSet<(VertexKey, VertexKey)>,
    seen_ab: std::collections::BTreeSet<(VertexKey, FaceKey)>,
    seen_ba: std::collections::BTreeSet<(VertexKey, FaceKey)>,
}

impl ContactAcc {
    pub(super) fn vv(&mut self, c: VvContact) {
        if self.seen_vv.insert((c.a, c.b)) {
            self.records.vv.push(c);
        }
    }
    pub(super) fn vf(&mut self, piercing: Operand, c: VfContact) {
        let (seen, list) = match piercing {
            Operand::A => (&mut self.seen_ab, &mut self.records.a_on_b),
            Operand::B => (&mut self.seen_ba, &mut self.records.b_on_a),
        };
        if seen.insert((c.vertex, c.face)) {
            list.push(c);
        }
    }
    pub(super) fn finish(self) -> ContactRecords {
        self.records
    }
}

/// **The face kinds with at least one wired boolean arm** — `Plane`,
/// `Cylinder` (the PR 5 conic arms), `Sphere` (the PR 7
/// cylinder×sphere SSI arm, structurally routed), `Torus` (the
/// crossing layer's torus arms: the circle rung's enclosure and its
/// carrier-identity rung, the certified line×torus quartic, the chart
/// containment, and the sector and pierce normals) and `Nurbs` (the
/// plane×NURBS arm, routed structurally so PR 7b's flag flip alone
/// makes it live). Pair-level refusals fire at the sites that
/// EXERCISE an arm (the sweep's crossing lanes, the join's section
/// table), citing the C5 routing; the kind with no wired arm at all
/// (`Cone`) is what [`gate_operand_pairs`] tests boxes for.
///
/// **`Approx` is absent by DECISION, not by gap.** Its fit is a
/// `Nurbs`, which is on the roster, so admitting it on the fitted
/// kind's authority would run the boolean against the APPROXIMATION
/// while reporting a result about the described surface. It stays off
/// until a rule for composing the fit's precision claim with the
/// boolean's certificates is ratified — and because it is off, the
/// refusal it earns is pair-scoped like every other kind's, naming
/// `SurfaceKind::Approx` in the germ pair.
pub(super) fn boolean_arm_exists<T: Decide>(surface: &geom::Surface<T>) -> bool {
    matches!(
        surface,
        geom::Surface::Plane { .. }
            | geom::Surface::Cylinder { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Torus { .. }
            | geom::Surface::Nurbs(_)
    )
}

/// **The face kinds ∖ and ∩ have a seam lane for** — the same roster
/// minus `Nurbs`, which has no crossing layer at all
/// (`BooleanError::CurvedPairUnsupported`'s docs carry the per-class
/// argument). ONE home, beside its sibling above, so the two rosters
/// cannot drift apart in two files: the front door in `ops` reads
/// this rather than spelling a second `matches!`.
///
/// `Approx` is off this roster for the reason it is off the one
/// above, which is strictly stronger here: `Nurbs` has no crossing
/// layer at all, and an approximating surface's chart is a `Nurbs`'s.
pub(super) fn revert_arm_exists<T: Decide>(surface: &geom::Surface<T>) -> bool {
    matches!(
        surface,
        geom::Surface::Plane { .. }
            | geom::Surface::Cylinder { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Torus { .. }
    )
}

/// One unsupported-kind face and the face of the other operand whose
/// box it may meet — [`first_unsupported_pair`]'s finding, and the
/// payload of the refusals built from it.
pub(super) struct UnsupportedPair {
    /// The operand carrying the unsupported-kind face.
    pub operand: Operand,
    /// That face.
    pub face: FaceKey,
    /// Its kind — the half of the germ pair with no arm.
    pub kind: geom::SurfaceKind,
    /// The other operand's face whose box overlaps it.
    pub other_face: FaceKey,
    /// That face's kind — the other half of the germ pair.
    pub other_kind: geom::SurfaceKind,
}

/// **The pair-scoped operand scan: the first face whose KIND has no
/// wired arm AND whose box may meet the other operand.**
///
/// A face kind is a property of a face, but an OPERATION is a
/// property of a pair, so a kind can only disqualify an operation
/// through a pair it could enter. Boxes decide that, at box-level
/// conservatism:
///
/// - **Non-overlap is a certificate.** Every box here is a superset
///   of its face's locus (`boxes` module contract), so two boxes that
///   do not overlap bound two loci that do not meet, and a face that
///   meets nothing of the other operand cannot enter any crossing,
///   any section, or any germ pair. Its kind is then irrelevant to
///   the operation and the gate has nothing to say about it.
/// - **Overlap is a MAY, not a DOES.** Boxes over-approximate, so
///   this scan still finds pairs that exact geometry would separate.
///   That is conservative in the correct direction — it never admits
///   a pair the crossing pipeline cannot handle — and the refusals
///   built from it say the faces "may meet" rather than claiming a
///   meeting the kernel has not computed.
///
/// The pad is the sweep's own ([`super::boxes::sweep_pad`]), so the
/// gate's boxes are the same boxes candidate generation reads: the
/// gate cannot admit a pair the sweep would then prune, nor refuse
/// one it would examine.
///
/// **A COVERED pair is not an offending pair.** `covered` names the
/// cross-operand pairs the caller's declarations speak for. The gate
/// refuses a kind because no arm can say what the two loci do; a
/// declaration is the author saying it, verified at the front door
/// before this gate runs, and consumed by the declared descent instead
/// of by a germ arm. So a pair the declarations cover is one the
/// pipeline HAS a verdict for, and the kind roster has nothing left to
/// object to there.
///
/// The rung is kind-generic and it is not kind-blind: what a
/// declaration may name is bounded by the certified carrier inventory
/// (`validate_declarations`' `inventory_face`, which is
/// [`mod@super::carrier_eq`]'s rung list). A kind with no rung cannot
/// survive into a declaration, so it can never be covered here — one
/// list decides both, and widening this gate is exactly and only
/// widening that ladder.
///
/// Coverage is per PAIR, never per face: an offending face still
/// disqualifies the operation through any OTHER face of the far
/// operand whose box it may meet and whose contact nobody declared.
///
/// **Only cross-operand pairs are examined**, and that is the whole
/// inventory rather than an omission: the boolean pipeline crosses
/// A's edges against B's faces and B's against A's, never a body
/// against itself — a self-intersecting operand is outside the
/// supported envelope on every kind, planar included, and is a
/// precondition rather than something this gate could decide. So a
/// cone and a torus on the SAME body do not gate each other here.
///
/// A face whose surface key does not RESOLVE is neither a pair
/// question nor a kind question: there is no description to bound and
/// no kind to name, so it is reported as the arena corruption it is
/// rather than labelled with a kind it was never shown to have.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// key does not resolve, or whose topology is corrupt
/// (`boxes::face_box`).
pub(super) fn first_unsupported_pair<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
    supported: impl Fn(&geom::Surface<T>) -> bool,
    covered: impl Fn(Operand, FaceKey, FaceKey) -> bool,
) -> Result<Option<UnsupportedPair>, BooleanError> {
    let pad = super::boxes::sweep_pad(band);
    for (operand, body, other) in [(Operand::A, a, b), (Operand::B, b, a)] {
        // Arena order both ways, and no box is built for an operand
        // that carries no unsupported kind at all — the common case
        // pays nothing for this gate.
        let mut offenders: Vec<(FaceKey, geom::SurfaceKind)> = Vec::new();
        for (key, f) in body.faces() {
            let s = surface_of(body, f)?;
            if !supported(s) {
                offenders.push((key, s.kind()));
            }
        }
        if offenders.is_empty() {
            continue;
        }
        // The other side's boxes are built ONCE, and only now that an
        // offender exists: the scan is `offenders × other faces`, so
        // re-boxing per offender would re-walk a whole body per cone.
        let others: Vec<(FaceKey, geom::SurfaceKind, bvh::Aabb)> = other
            .faces()
            .map(|(key, f)| {
                let kind = surface_of(other, f)?.kind();
                Ok((key, kind, super::boxes::face_box(other, key, pad, band)?))
            })
            .collect::<Result<_, BooleanError>>()?;
        for (face, kind) in offenders {
            let boxed = super::boxes::face_box(body, face, pad, band)?;
            for &(other_face, other_kind, ref other_box) in &others {
                if boxed.overlaps(other_box) && !covered(operand, face, other_face) {
                    return Ok(Some(UnsupportedPair {
                        operand,
                        face,
                        kind,
                        other_face,
                        other_kind,
                    }));
                }
            }
        }
    }
    Ok(None)
}

/// **The operand gate, pair-scoped** (M5 PR 9, C12.1 — the F5
/// planar-only gate retires PER C5 TABLE ARM, never wholesale).
///
/// First, each operand is a closed solid at rest, by the validator's
/// own verdict ([`crate::validate_closed`]): a tier-1 finding refuses
/// as [`BooleanError::CorruptOperand`], and tier-2 scaffolding — a
/// strut, an empty loop, a null edge, a split shell — as
/// [`BooleanError::ScaffoldingOperand`], each carrying the findings.
/// Then two rules, with different scopes on purpose:
///
/// - **Faces**: a kind with no wired arm ([`boolean_arm_exists`])
///   disqualifies the operation only through a PAIR it could enter
///   ([`first_unsupported_pair`]) and that the caller's declarations
///   do not cover. A torus wall whose box clears the other operand
///   does not gate anything, and neither does one whose contact with
///   the face it may meet the author has DECLARED.
/// - **Edges**: body-scoped. `Line`/`Circle`/`Ellipse` pass (the
///   crossing lanes handle all three; the both-split point lane still
///   needs a `Line`, and says so where it refuses); a `Nurbs` operand
///   edge refuses typed wherever it sits — a rung-3 INPUT operand is
///   outside the supported envelope, rung-3 edges being what the zip
///   MINTS rather than what it consumes, and that is a claim about
///   the operand rather than about a pair.
///
/// # Errors
///
/// [`BooleanError::CorruptOperand`] / [`BooleanError::ScaffoldingOperand`]
/// for an operand tier 1 / tier 2 refuses; [`BooleanError::CurvedPairUnsupported`] for a germ pair
/// with no arm; [`BooleanError::CurvedEdgeUnsupported`] per operand;
/// [`BooleanError::CurvedBooleanUnsupported`] for a face whose
/// surface key does not resolve.
pub(super) fn gate_operand_pairs<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    declared: &super::DeclaredPairs<T>,
    band: Band,
) -> Result<(), BooleanError> {
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        let (broken, scaffolding) = crate::validate::closed_by_tier(body);
        if !broken.is_empty() {
            return Err(BooleanError::CorruptOperand {
                operand,
                corruption: super::Corruption::Structure { errors: broken },
            });
        }
        if !scaffolding.is_empty() {
            return Err(BooleanError::ScaffoldingOperand {
                operand,
                errors: scaffolding,
            });
        }
        gate_operand_edges(body, operand)?;
    }
    // A pair is covered by the certificate its consumer reads: the
    // declared descent through the carrier ladder, which runs on a pair
    // the door verified ONE carrier. A `Tangent` claim covers nothing
    // here: its arms are the witness lane's, whose kinds are all on the
    // roster.
    if let Some(p) = first_unsupported_pair(a, b, band, boolean_arm_exists, |operand, f, other| {
        declared.verified_one_carrier(operand, f, operand.other(), other)
    })? {
        return Err(BooleanError::CurvedPairUnsupported {
            op: None,
            site: super::PairRefusalSite::OperandGate,
            operand: p.operand,
            face: p.face,
            kind: p.kind,
            other_face: p.other_face,
            other_kind: p.other_kind,
        });
    }
    Ok(())
}

/// A face's resolved surface. An unresolved key is arena corruption
/// and says so, rather than acquiring a kind label by default —
/// `Nurbs` was the old default and named a kind nothing had shown the
/// face to have.
fn surface_of<'a, T: Decide>(
    body: &'a Body<T>,
    face: &crate::entity::Face,
) -> Result<&'a geom::Surface<T>, BooleanError> {
    body.get_surface(face.surface)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "operand gate: an operand face's surface key does not resolve",
        })
}

/// The BODY-scoped half of [`gate_operand_pairs`]: the edge carriers.
fn gate_operand_edges<T: Decide>(body: &Body<T>, operand: Operand) -> Result<(), BooleanError> {
    for (edge_key, edge) in body.edges() {
        match certified(body.get_curve_geom(edge.curve))?.carrier() {
            geom::Curve3::Line { .. }
            | geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. } => {}
            // The boolean fence: no join, section or pierce arm
            // reads a spiric, so an operand carrying one refuses
            // here, at the gate, as a spline does.
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                return Err(BooleanError::CurvedEdgeUnsupported {
                    operand,
                    edge: edge_key,
                });
            }
        }
    }
    Ok(())
}

/// An operand edge's certified carrier. Tier 2 refuses a null edge and
/// tier 1 an unresolvable curve key at the operand gate, and the
/// sweep's only surgery (`split_edge`) mints certified pieces, so a
/// miss past the gate is a kernel invariant, not the operand's fault.
fn certified<T: geom_core::Real>(
    geom: Option<&CurveGeom<T>>,
) -> Result<&geom_brep::EdgeCurve<T>, BooleanError> {
    geom.and_then(CurveGeom::certified)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "an operand edge past the tier-1/2 gate has no certified carrier",
        })
}

/// The recipe source of a face's surface description, if the recipe
/// layer stamped one (N6; the plane-identity evidence at every
/// classification comparison).
pub(super) fn face_source<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<&crate::source::GeomSource> {
    body.surface_source(body.get_face(face)?.surface)
}

/// The face's plane description (post-gate: always a `Plane`), with
/// the **face's outward normal** — not the chart's.
///
/// [`PlaneDesc::normal`] is contractually the unit OUTWARD normal —
/// the surface's chart normal with [`crate::entity::Face::sense`]
/// folded in: the chart is the only place orientation is encoded, so
/// on a `sense: false` face the stored normal points INTO the
/// material, and every consumer reading a material direction off it
/// would answer backwards. The flip itself lives in
/// [`crate::face_normal`], which this function is defined in terms of
/// — one door for the planar consumers (`plane_of`, this sweep, the
/// pierce lane, the REST lane, and the SHARED [`crate::sector_face`]
/// walk, which is why the door sits at the crate root rather than
/// here), one flip, so those consumers stay orientation-blind.
///
/// Outside this crate the same fold is spelled through
/// [`geom_brep::OutwardNormal::from_chart`], the type's only
/// constructor, which takes the bit; there is no scalar sign on a face
/// for a reader anywhere to multiply by.
///
/// Consumers that only compare the plane RESIDUAL `(p − o)·n̂` against
/// Zero, or that hand the normal to a ray-parity test, are unaffected
/// either way (a residual's sign flip decides Zero the same, and
/// crossing parity is blind to frame handedness). The consumers that
/// read a MATERIAL side off the sign — `side_code`, the containment
/// ray's `d·n̂` — are exactly the ones this fixes.
pub(super) fn face_plane<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<PlaneDesc<T>> {
    let origin = match body.get_surface(body.get_face(face)?.surface) {
        Some(geom::Surface::Plane { origin, .. }) => *origin,
        _ => return None,
    };
    Some(PlaneDesc {
        origin,
        normal: face_outward_normal(body, face)?.vec(),
    })
}

// The same door, typed: a planar face's outward normal as an
// [`OutwardNormal`], which is what the material-side consumers want.
//
// INVARIANT: there is ONE flip, and since the sector walk became
// shared it lives at the crate root — [`crate::face_normal`], whose
// docs carry the argument and the consumer list. This module's four
// remaining consumers reach it through this re-export, and
// `face_plane` above is still defined in terms of it, so the invariant
// is unchanged in substance: one flip, not two that could drift.
pub(super) use crate::face_normal::face_outward_normal;

/// **The face's recipe source with its `sense` composed into
/// `orient`** ([`crate::GeomSource::reverted`] when `sense` is false) —
/// the identity the coincidence ladders are handed, which is NOT the
/// surface's source ([`face_source`]).
///
/// **On a plane face the composed tag is the material side.**
/// [`super::oriented_plane_eq`]'s rung 1 answers Same±-orientation
/// syntactically, from the two sources' `orient` tags, and asserts
/// (debug) that same-source descriptions agree bitwise. The
/// descriptions it is handed are [`face_plane`]'s — the faces' OUTWARD
/// normals — so two faces sharing one surface key and one recipe
/// source but differing in `sense` carry descriptions that are exact
/// negations of each other. N6's `orient` tag means "this description
/// is the source expression's orientation-reversal", which is what a
/// `sense: false` plane face's outward normal is, so composing the
/// sense in keeps rung 1 exact with zero numerics; left uncomposed, the
/// rung would call that pair `SameOriented` and the bit assertion would
/// fire.
///
/// **On a curved face it is not.** A curved description cannot be
/// reversed, so `Body::revert` records a curved face's reversal on its
/// `sense` AND on its source's `orient`, and the composition cancels: a
/// face and its reverted twin compose to one tag although their
/// material sides are opposite. The curved rung a curved pair reaches
/// through [`mod@super::carrier_eq`] (`source_rung`, from
/// [`super::rest::carrier_pair_verdict`] and `recl`'s declared-`Rest`
/// sector pairs) therefore reads only the sources' base here and takes
/// the material side from the descriptions' `outward` bits.
///
/// Returned owned: the flip mints a value rather than borrowing the
/// stored one (the stored source describes the SURFACE and must not be
/// rewritten by a face-level question).
pub(super) fn face_oriented_source<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<crate::source::GeomSource> {
    let source = face_source(body, face)?;
    Some(if body.get_face(face)?.sense {
        source.clone()
    } else {
        source.reverted()
    })
}

/// F7: the maximal-faces precondition through the coincidence ladder —
/// same surface key (structural) or Same±-oriented planes (declared,
/// [`super::oriented_plane_eq`]) across any edge ⇒
/// [`BooleanError::NonMaximalFaces`]. Numeric coplanarity NEVER
/// triggers the refusal; a near-coplanar dihedral surfaces as the
/// predicate's own typed escalation instead.
pub(super) fn gate_maximal_faces<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    band: Band,
) -> Result<(), BooleanError> {
    for (edge_key, _) in body.edges() {
        let Ok(sides) = crate::readback::edge_sides(body, edge_key) else {
            continue;
        };
        let (f1, f2) = sides.faces();
        if f1 == f2 {
            continue; // seam/strut inside one face: not a coplanar PAIR
        }
        let (k1, k2) = sides.surfaces();
        if k1 == k2 {
            // Same-key CURVED adjacency is the CANONICAL maximal form
            // (M5 PR 9, C12.5): a periodic wall cannot be one face
            // without its parameterization cut, so two half-walls
            // sharing one cylinder key across a meridian strut are
            // exactly what a maximal-faced curved operand looks like
            // (the cosurface merge itself KEEPS such a cut). Only the
            // PLANAR same-key pair is the F7 defect.
            let planar = body
                .get_surface(k1)
                .is_some_and(|s| matches!(s, geom::Surface::Plane { .. }));
            if planar {
                return Err(BooleanError::NonMaximalFaces {
                    operand,
                    edge: edge_key,
                });
            }
            continue;
        }
        let (Some(p1), Some(p2)) = (face_plane(body, f1), face_plane(body, f2)) else {
            continue;
        };
        let arm = edge_chord_len(body, edge_key).unwrap_or_else(T::one);
        // Same-operand comparison: sources apply (a shared recipe
        // source IS declared coplanarity — the pair should have been
        // merged by the producing op); cross-operand declared pairs
        // never do.
        let (o1, o2) = (
            face_oriented_source(body, f1),
            face_oriented_source(body, f2),
        );
        let id = super::PlaneIdentity {
            s1: o1.as_ref(),
            s2: o2.as_ref(),
            declared: false,
        };
        let coplanar = |offset| BooleanError::CoplanarNeighbours {
            operand,
            faces: [f1, f2],
            offset,
        };
        let extent = super::carrier_eq::ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(
            geom_core::Point3::origin(),
            arm,
        ));
        match super::plane_eq::plane_eq_typed(&p1, &p2, id, &extent, band) {
            Ok(super::PlaneRelation::Distinct) => {}
            Ok(_) => {
                return Err(BooleanError::NonMaximalFaces {
                    operand,
                    edge: edge_key,
                });
            }
            Err(LadderRefusal::Coplanar { offset, .. }) => {
                return Err(coplanar(NeighbourOffset::Zero(offset)));
            }
            Err(LadderRefusal::Refused(super::PlaneEqError::Escalated { rung, diag })) => {
                return Err(BooleanError::plane_identity(
                    rung,
                    super::PlaneDoor::Neighbours,
                    diag,
                ));
            }
            Err(LadderRefusal::Refused(super::PlaneEqError::Undeclared { diag, .. })) => {
                return Err(coplanar(NeighbourOffset::Undecided(diag)));
            }
            // Unreachable with `declared: false`; kept typed.
            Err(LadderRefusal::Refused(super::PlaneEqError::Contradicted { fact, .. })) => {
                return Err(BooleanError::DeclarationContradicted { fact });
            }
            // Unreachable with `declared: false` (only a declared
            // reading is unsettled); kept typed as the gate's in-band.
            Err(LadderRefusal::Refused(super::PlaneEqError::Unsettled { diag })) => {
                return Err(BooleanError::plane_identity(
                    super::PlaneRung::Parallel,
                    super::PlaneDoor::Neighbours,
                    diag,
                ));
            }
        }
    }
    Ok(())
}

/// **An undeclared continuation refuses at the reduction, on every
/// carrier kind** (C4's continuation clause).
///
/// A cross-operand face pair on one carrier with ALIGNED senses is one
/// surface carried on, whether it abuts or overlaps; this scan finds
/// the pairs that meet along their boundary. An overlapping pair whose
/// boundaries share no stretch of curve passes it, and the sweep's
/// coincident-sector classification refuses it there, with the same
/// error (`vtxfac`'s coplanar sector reaching the carrier ladder).
/// Without a declaration the op has no licence to treat the two as one
/// carrier, and no licence to merge them, so its output would carry two
/// cosurface faces side by side, which the next boolean refuses as an
/// operand. So the pair refuses here, before the sweep, naming the pair
/// and the relation a declaration would assert — the same
/// [`BooleanError::UndeclaredCoincidence`] an undeclared opposed pair
/// gets.
///
/// The carrier question is the detector posture of the verify ladder
/// ([`super::carrier_pair_relation`]); a pair it calls one carrier by
/// shared recipe source is structurally licensed, and an in-band
/// escalation is left to the stages that already own it. **"Meets along
/// its boundary" is decided point-on-edge** ([`edges_share_a_curve`]):
/// a boundary edge of each face must share a stretch of curve, not a
/// point, read off the edges' carriers through `Decide`. Boxes only
/// PRUNE here — a B face whose box clears an A face's is never asked,
/// and an edge pair whose boxes clear is never sampled — except on the
/// one fallback `edges_share_a_curve` documents for a carrier with no
/// point parameter, where box overlap stands in for the decision and
/// can only over-report a meeting, so only ever refuses.
///
/// **The boxes are handed in, not built here.** Box construction reads
/// coordinate brackets, which is the `Bounds` seam the 2026-07-29
/// driver amendment ratified for the reduction's DRIVER
/// (`boolean_reduce_declared_strategy`, geom-core `real.rs`'s Bounds
/// scope rule); this scan is `Decide`-only, so the bracket read stays
/// at that door and nothing here can compare a bracket. `face_box` and
/// `edge_box` are that door's padded boxes (`boxes::face_box`,
/// `boxes::edge_box` at `pad`).
///
/// **What it returns is the settled pairs it read**: every undeclared
/// pair the ladder called one carrier, which with `declared: false` is
/// rung 1 alone (shared recipe source, N6), in the scan's order. The
/// declared pairs it skips are settled by the declaration door instead
/// ([`super::DeclaredPairs::settled`]).
///
/// # Errors
///
/// [`BooleanError::UndeclaredCoincidence`] naming the first undeclared
/// continuation in arena order (A's faces, then B's within each);
/// [`BooleanError::ClassificationInvariant`] for a torn arena.
pub(super) fn refuse_undeclared_continuations<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    declared: &super::DeclaredPairs<T>,
    band: Band,
    pad: f64,
    face_box: impl Fn(&Body<T>, FaceKey) -> Result<bvh::Aabb, BooleanError>,
    edge_box: impl Fn(&Body<T>, EdgeKey) -> Result<bvh::Aabb, BooleanError>,
) -> Result<Vec<super::SettledPair>, BooleanError> {
    let mut settled = Vec::new();
    let a_faces: Vec<(FaceKey, bvh::Aabb)> = a
        .faces()
        .map(|(k, _)| Ok((k, face_box(a, k)?)))
        .collect::<Result<_, BooleanError>>()?;
    let b_keys: Vec<FaceKey> = b.faces().map(|(k, _)| k).collect();
    let b_boxes: Vec<bvh::Aabb> = b_keys
        .iter()
        .map(|&k| face_box(b, k))
        .collect::<Result<_, BooleanError>>()?;
    // The C10 tree over B's faces, as the sweep builds it: candidates
    // arrive in ascending arena order, so "first in arena order" is the
    // brute-force scan's first.
    let tree = bvh::Bvh::build(&b_boxes);
    let mut edge_boxes: Option<[FaceEdges; 2]> = None;
    for &(fa, box_a) in &a_faces {
        for i in tree.overlapping(&box_a) {
            let fb = *b_keys.get(i).ok_or(BooleanError::ClassificationInvariant {
                what: "continuation scan: the face tree returned an index past its input",
            })?;
            if declared.class_of(Operand::A, fa, Operand::B, fb).is_some() {
                continue;
            }
            let relation = match super::carrier_pair_relation(a, fa, b, fb, false, band) {
                Ok(relation) => relation,
                Err(super::PairUnread::OutsideInventory) => continue,
                // The operands passed the gates, whose face boxes read;
                // a face with no extent to compare it over is named.
                Err(super::PairUnread::Extent(_)) => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "continuation scan: an operand face's consumed extent cannot be read",
                    });
                }
            };
            let (diag, relation) = match relation {
                Ok(
                    relation @ (super::CarrierRelation::SameOriented
                    | super::CarrierRelation::SameOpposite),
                ) => {
                    settled.push(super::SettledPair {
                        a: fa,
                        b: fb,
                        relation,
                    });
                    continue;
                }
                Err(super::CarrierEqError::Undeclared {
                    diag,
                    relation: relation @ super::CarrierRelation::SameOriented,
                }) => (diag, relation),
                _ => continue,
            };
            if edge_boxes.is_none() {
                edge_boxes = Some([face_edges(a, &edge_box)?, face_edges(b, &edge_box)?]);
            }
            let [ea, eb] = edge_boxes
                .as_ref()
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "continuation scan: edge boxes not built",
                })?;
            let mut meets = false;
            'pairs: for &(ex, bx) in ea.get(&fa).map_or(&[][..], Vec::as_slice) {
                for &(ey, by) in eb.get(&fb).map_or(&[][..], Vec::as_slice) {
                    if bx.overlaps(&by) && edges_share_a_curve(a, ex, b, ey, &bx, &by, pad, band)? {
                        meets = true;
                        break 'pairs;
                    }
                }
            }
            if meets {
                return Err(BooleanError::UndeclaredCoincidence {
                    diag,
                    pair: [(Operand::A, fa), (Operand::B, fb)],
                    relation,
                });
            }
        }
    }
    Ok(settled)
}

/// Each face's boundary edges with their padded boxes.
type FaceEdges = std::collections::BTreeMap<FaceKey, Vec<(EdgeKey, bvh::Aabb)>>;

/// Every face's boundary edges, each with the box the driver hands in.
fn face_edges<T: Decide>(
    body: &Body<T>,
    edge_box: &impl Fn(&Body<T>, EdgeKey) -> Result<bvh::Aabb, BooleanError>,
) -> Result<FaceEdges, BooleanError> {
    let mut out = FaceEdges::new();
    for (key, edge) in body.edges() {
        let bx = edge_box(body, key)?;
        let f1 = body.face_of_half_edge(edge.he_plus);
        let f2 = body
            .face_of_half_edge(edge.he_minus)
            .filter(|&f| Some(f) != f1);
        for f in [f1, f2].into_iter().flatten() {
            out.entry(f).or_default().push((key, bx));
        }
    }
    Ok(out)
}

/// **Do two edges share a CURVE, not merely a point?** Two faces meet
/// along their boundary exactly when some edge of each overlaps the
/// other's along a stretch, and the ends of that stretch are ends of
/// the two edges. So the edges share a curve iff two definitely
/// distinct points among both edges' ends and midpoints lie on BOTH
/// edges. A point is on an edge when its distance to the edge's
/// carrier, at the carrier parameter nearest it clamped into the
/// edge's span, decides zero; an in-band distance counts as on, which
/// can only over-report a meeting and so only ever refuses.
///
/// A carrier with no point parameter (ellipse, spline) falls back to
/// the boxes: they must overlap on every axis and run past
/// [`POINT_TOUCH_RUN`] pads on some axis. This is the scan's one
/// box-decided answer, and it errs only toward "meets", so toward a
/// refusal.
/// How many pads two edge boxes must overlap by, on some axis, before
/// the box fallback of [`edges_share_a_curve`] reads them as sharing a
/// curve. Each box is its edge's extent padded by `pad` on every side,
/// so two edges that meet at ONE point and leave it in opposite
/// directions along an axis overlap there by the two pads, `2·pad`, at
/// most. Four pads is that bound doubled, so that point touches stay
/// clear of it. Edges that leave a shared point on the same side of an
/// axis (a V) can still overlap past it. That over-reports a meeting,
/// and so refuses rather than admits.
const POINT_TOUCH_RUN: f64 = 4.0;

#[allow(clippy::too_many_arguments)]
fn edges_share_a_curve<T: Decide>(
    x: &Body<T>,
    ex: EdgeKey,
    y: &Body<T>,
    ey: EdgeKey,
    bx: &bvh::Aabb,
    by: &bvh::Aabb,
    pad: f64,
    band: Band,
) -> Result<bool, BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "continuation scan: an edge lost its geometry",
    };
    let sampled = |body: &Body<T>, key: EdgeKey| -> Result<Option<_>, BooleanError> {
        let edge = body.get_edge(key).ok_or_else(corrupt)?;
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(CurveGeom::certified)
        else {
            return Ok(None);
        };
        let (t0, t1) = curve.params();
        let mid = (t0 + t1) * T::from_f64(0.5);
        let carrier = curve.carrier().clone();
        if carrier.param_near(carrier.eval(mid), mid).is_none() {
            return Ok(None);
        }
        let end = |he| -> Result<Point3<T>, BooleanError> {
            let v = body.get_half_edge(he).ok_or_else(corrupt)?.start;
            body.get_vertex(v)
                .and_then(|v| body.get_point(v.point))
                .copied()
                .ok_or_else(corrupt)
        };
        let points = [end(edge.he_plus)?, end(edge.he_minus)?, carrier.eval(mid)];
        Ok(Some((carrier, t0, t1, mid, points)))
    };
    let (Some(cx), Some(cy)) = (sampled(x, ex)?, sampled(y, ey)?) else {
        let run = |lo_x: f64, hi_x: f64, lo_y: f64, hi_y: f64| hi_x.min(hi_y) - lo_x.max(lo_y);
        let runs = [
            run(bx.min_x, bx.max_x, by.min_x, by.max_x),
            run(bx.min_y, bx.max_y, by.min_y, by.max_y),
            run(bx.min_z, bx.max_z, by.min_z, by.max_z),
        ];
        return Ok(
            runs.iter().all(|&r| r >= 0.0) && runs.iter().any(|&r| r > POINT_TOUCH_RUN * pad)
        );
    };
    let zero = |m: T| {
        !matches!(
            decide("bool_continuation_boundary", Margin::of(m), band),
            Ok(Sign::Positive | Sign::Negative)
        )
    };
    let on = |(carrier, t0, t1, mid, _): &(geom::Curve3<T>, T, T, T, [Point3<T>; 3]),
              p: Point3<T>| {
        carrier.param_near(p, *mid).is_some_and(|t| {
            let foot = carrier.eval(t.max(*t0).min(*t1));
            zero((foot - p).norm())
        })
    };
    let shared: Vec<Point3<T>> =
        cx.4.iter()
            .chain(&cy.4)
            .copied()
            .filter(|&p| on(&cx, p) && on(&cy, p))
            .collect();
    Ok(shared
        .iter()
        .enumerate()
        .any(|(i, &p)| shared[i + 1..].iter().any(|&q| !zero((q - p).norm()))))
}

fn edge_chord_len<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<T> {
    let e = body.get_edge(edge)?;
    let pa = *body.get_point(body.get_vertex(body.get_half_edge(e.he_plus)?.start)?.point)?;
    let pb = *body.get_point(
        body.get_vertex(body.get_half_edge(e.he_minus)?.start)?
            .point,
    )?;
    Some((pb - pa).norm())
}

/// One sweep direction: every edge (fragment) of `x` against the faces
/// of `y` its box can touch (module docs: the tree prunes, predicates
/// decide). `x_is` names which operand `x` is (contact orientation).
///
/// `T: Decide + Bounds` is the ratified compound-bound seam
/// (2026-07-29 — geom-core `real.rs`, Bounds scope rule): the C10
/// tree is the subdivision driver, and box construction reads
/// coordinate brackets — never a value comparison in classification.
/// The realized candidate generator's per-direction face tree, built
/// ONCE over the face snapshot (arena order = input order). Mid-sweep
/// splits of `y`'s edges only mint vertices ON existing boundary
/// (within the pad), so the snapshot boxes stay conservative for the
/// whole direction.
fn face_tree<T: Decide + Bounds>(
    y: &Body<T>,
    faces: &[FaceKey],
    knobs: &SweepKnobs,
    pad: f64,
    band: Band,
) -> Result<bvh::Bvh, BooleanError> {
    let mut face_boxes = Vec::with_capacity(faces.len());
    for &f in faces {
        let planted = knobs.plant == Some(f);
        face_boxes.push(if planted {
            // Pin (iii)'s planted degradation: the inverted box
            // overlaps nothing — this face's events get lost and the
            // suite's superset pin must catch it.
            bvh::Aabb {
                min_x: f64::INFINITY,
                min_y: f64::INFINITY,
                min_z: f64::INFINITY,
                max_x: f64::NEG_INFINITY,
                max_y: f64::NEG_INFINITY,
                max_z: f64::NEG_INFINITY,
            }
        } else {
            boxes::face_box(y, f, pad, band)?
        });
    }
    Ok(bvh::Bvh::build(&face_boxes))
}

/// What the sweep's door read of the declaration ahead of `question`,
/// asked of `edge` of `x` against `face` of the other operand: each of
/// the edge's parent faces paired with `face`, looked up in turn
/// ([`super::DeclaredPairs::read`]). No question the sweep asks of an
/// edge is one a declaration settles (the declared cover spends the
/// declaration, and an undeclared pair's refusal is the same pose's),
/// so the door admits no class that would.
fn edge_face_read<T: geom_core::Real>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::DeclaredPairs<T>,
    question: Coincide,
) -> DeclarationRead {
    let pairs: Vec<_> = [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .map(|f| (x_is, f, x_is.other(), face))
    .collect();
    declared.read(&pairs, question, &[])
}

#[allow(clippy::too_many_arguments)] // one parameter per named duty (bodies, orientation, declarations, sinks, band, strategy, plant, trace)
pub(super) fn sweep_direction<T: Decide + Bounds + crate::props::AtRestPolicy>(
    x: &mut Body<T>,
    y: &mut Body<T>,
    x_is: Operand,
    declared: &super::DeclaredPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    strategy: SweepStrategy,
    knobs: &SweepKnobs,
    mut trace: Option<&mut SweepTrace>,
    deferred: &mut Vec<DeferredTouch>,
    tol: Tol,
) -> Result<(), BooleanError> {
    let faces: Vec<FaceKey> = y.faces().map(|(k, _)| k).collect();
    let pad = knobs.pad_override.unwrap_or_else(|| boxes::sweep_pad(band));
    // With `sweep-testing`, the tree is optional so the idealized
    // reference can decline it. Without the feature there is no
    // `Idealized` variant to decline it with, so the tree is
    // unconditional and the brute-force arm below does not exist.
    #[cfg(feature = "sweep-testing")]
    let tree: Option<bvh::Bvh> = match strategy {
        SweepStrategy::Realized => Some(face_tree(y, &faces, knobs, pad, band)?),
        SweepStrategy::Idealized => None,
    };
    #[cfg(not(feature = "sweep-testing"))]
    let tree: bvh::Bvh = {
        let SweepStrategy::Realized = strategy;
        face_tree(y, &faces, knobs, pad, band)?
    };
    let mut worklist: std::collections::VecDeque<(EdgeKey, usize)> =
        x.edges().map(|(k, _)| (k, 0)).collect();

    while let Some((edge_key, start)) = worklist.pop_front() {
        // The fragment's candidate face indices, ascending — the
        // realized set is a subsequence of the idealized scan, so the
        // examination order (and with it every split/requeue) is
        // preserved pair-for-pair.
        #[cfg(feature = "sweep-testing")]
        let candidates: Vec<usize> = match &tree {
            Some(t) => t.overlapping(&boxes::edge_box(x, edge_key, pad)?),
            // The idealized reference's candidate set: every face, in
            // arena order. Reachable only through the gated
            // `SweepStrategy::Idealized`, and compiled out with it.
            None => (0..faces.len()).collect(),
        };
        #[cfg(not(feature = "sweep-testing"))]
        let candidates: Vec<usize> = tree.overlapping(&boxes::edge_box(x, edge_key, pad)?);
        let mut ci = 0;
        'faces: while let Some(&j) = candidates.get(ci) {
            ci += 1;
            if j < start {
                continue;
            }
            let Some(&face) = faces.get(j) else {
                // Unreachable: candidate indices come from the face
                // snapshot itself.
                break;
            };
            if let Some(tr) = trace.as_deref_mut() {
                tr.examined.push((edge_key, face));
            }
            let edge =
                x.get_edge(edge_key)
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "worklist edge vanished mid-sweep",
                    })?;
            let ((u, pu), (v, pv)) = edge_ends(x, &edge)?;
            // Per-kind face dispatch (M5 PR 9, C12.1): planar faces run
            // the M3 lane below (bit-identically for line edges, plus
            // the conic ROOT lane); curved faces get the clearance /
            // typed-frontier arm.
            let Some(plane) = face_plane(y, face) else {
                let event = curved_face_arm(
                    x, y, x_is, edge_key, &edge, u, v, face, pu, pv, declared, contacts, band, tol,
                )?;
                if !matches!(event, CurvedEvent::None | CurvedEvent::Deferred)
                    && let Some(tr) = trace.as_deref_mut()
                {
                    tr.accepted.push((edge_key, face));
                }
                // A wall crossing is split and recorded HERE, with the
                // same triple the conic × plane root above uses: the
                // event splits `x`'s edge, the contact is minted
                // against the piercing side, and the remainder fragment
                // is re-queued so the second root of the same span is
                // found on the next pass.
                let (t, p, at) = match event {
                    CurvedEvent::Pierce { t, p, at } => (t, p, at),
                    CurvedEvent::Deferred => {
                        deferred.push(DeferredTouch {
                            x_is,
                            edge: edge_key,
                            end: v,
                            face,
                            refusal: BooleanError::CurvedPierceUnsupported {
                                operand: x_is,
                                face,
                                edge: edge_key,
                                band,
                            },
                        });
                        continue;
                    }
                    CurvedEvent::None | CurvedEvent::Recorded => continue,
                };
                match at {
                    FaceContainment::Out => continue,
                    FaceContainment::In => {
                        let w = split_at(x, x_is, edge_key, t, tol)?;
                        contacts.vf(x_is, VfContact { vertex: w, face });
                        requeue(&mut worklist, x, edge_key, w, j)?;
                    }
                    FaceContainment::OnEdge(ey) => {
                        let w = split_at(x, x_is, edge_key, t, tol)?;
                        let wy = split_other_at_point(y, x_is.other(), ey, p, band, tol)?;
                        push_vv(contacts, x_is, w, wy);
                        requeue(&mut worklist, x, edge_key, w, j)?;
                    }
                    FaceContainment::OnVertex(vy) => {
                        let w = split_at(x, x_is, edge_key, t, tol)?;
                        push_vv(contacts, x_is, w, vy);
                        requeue(&mut worklist, x, edge_key, w, j)?;
                    }
                }
                break 'faces;
            };
            // Conic carriers against a plane (M5 PR 9): crossing
            // detection is ROOT-BASED and endpoint-verdict-free — the
            // splitting lane's C12.1 machinery reused verbatim (a
            // belly arc crosses between same-side endpoints, which the
            // endpoint-sign match below cannot see). Every interior
            // root is examined, and the FIRST one the face does not
            // place `Out` splits exactly like a proper line crossing;
            // both fragments re-examine the SAME face, so any other
            // root is found again on them.
            //
            // **The one-sided cover** (C4, as on the curved arm): when an
            // edge's parent carrier is certified to lie in one closed side
            // of this face's plane, a conic on it never crosses the plane,
            // and its residual along the carrier circle is a sinusoid of
            // one sign, zero at most once. An endpoint ON the plane is then
            // the edge's one incidence, a touch, and it takes the endpoint
            // rows; the root lane, which cannot separate the double root
            // there from a crossing, is not asked. With no endpoint on the
            // plane the roots still decide, so a touch in the middle of
            // the edge keeps their typed refusal.
            {
                let curve = certified(x.get_curve_geom(edge.curve))?.clone();
                let (t0, t1) = curve.params();
                let covers = edge_covers(x, x_is, &edge, face, declared);
                let touch_at_end = if covers.is_empty() {
                    None
                } else {
                    let read =
                        edge_face_read(x, x_is, &edge, face, declared, Coincide::VertexOnFace);
                    // One decision per end, in the carrier's own residual:
                    // the sign the cover's side is stated in. It differs
                    // from the offset along the OUTWARD normal only by the
                    // face's sense bit, so whether an end is ON the plane
                    // reads the same either way.
                    let carrier = y
                        .get_face(face)
                        .and_then(|f| y.get_surface(f.surface))
                        .ok_or(BooleanError::ClassificationInvariant {
                            what: "planar lane: the face's plane resolved above",
                        })?;
                    let side = |p: Point3<T>| {
                        decide(
                            "bool_vertex_face_side",
                            Margin::of(geom_brep::implicit_residual(carrier, p)),
                            band,
                        )
                        .map_err(|diag| {
                            BooleanError::coincidence(Coincide::VertexOnFace, read, diag)
                        })
                    };
                    let (s1, s2) = (side(pu)?, side(pv)?);
                    // The off end must lie where a parent's cover puts it.
                    let off_end_admitted = || {
                        let off = if s1 == Sign::Zero { s2 } else { s1 };
                        covers.iter().any(|c| c.admits(off))
                    };
                    ((s1 == Sign::Zero) != (s2 == Sign::Zero) && off_end_admitted())
                        .then_some(s1 == Sign::Zero)
                };
                match crate::splitting::plane_crossing_lane(
                    curve.carrier(),
                    t0,
                    t1,
                    plane.origin,
                    plane.normal,
                    band,
                ) {
                    PlaneCrossingLane::Line => {}
                    // A spiric or a spline: its endpoints' sides neither
                    // find its crossings nor place them.
                    PlaneCrossingLane::Unlaned => {
                        return Err(BooleanError::CrossingCarrierUnsupported {
                            operand: x_is,
                            edge: edge_key,
                            face,
                        });
                    }
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Miss) => continue,
                    // The conic's plane is parallel to the face's: off
                    // it, a miss; in it, the line lane's `(Zero, Zero)`
                    // posture — both endpoints through
                    // `vertex_on_face`, the interior left to the
                    // neighbour faces.
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => {
                        match decide("bool_conic_face_plane_offset", Margin::of(offset), band) {
                            Ok(Sign::Positive | Sign::Negative) => continue,
                            Ok(Sign::Zero) => {}
                            Err(diag) => {
                                return Err(BooleanError::coincidence(
                                    Coincide::EdgeOnPlane,
                                    edge_face_read(
                                        x,
                                        x_is,
                                        &edge,
                                        face,
                                        declared,
                                        Coincide::EdgeOnPlane,
                                    ),
                                    diag,
                                ));
                            }
                        }
                        let mut hit =
                            vertex_on_face(x_is, y, u, pu, face, &plane, contacts, band, tol)?;
                        if v != u {
                            hit |=
                                vertex_on_face(x_is, y, v, pv, face, &plane, contacts, band, tol)?;
                        }
                        if hit && let Some(tr) = trace.as_deref_mut() {
                            tr.accepted.push((edge_key, face));
                        }
                        continue;
                    }
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(_))
                        if touch_at_end.is_some() =>
                    {
                        let Some(first_end) = touch_at_end else {
                            return Err(BooleanError::ClassificationInvariant {
                                what: "conic lane: the one-sided touch lost its sides",
                            });
                        };
                        let (w, pw) = if first_end { (u, pu) } else { (v, pv) };
                        if vertex_on_face(x_is, y, w, pw, face, &plane, contacts, band, tol)?
                            && let Some(tr) = trace.as_deref_mut()
                        {
                            tr.accepted.push((edge_key, face));
                        }
                        continue;
                    }
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(fault))) => {
                        return Err(BooleanError::Escalated {
                            decision: BooleanDecision::of_conic_root(
                                fault,
                                edge_face_read(
                                    x,
                                    x_is,
                                    &edge,
                                    face,
                                    declared,
                                    Coincide::EdgeOnPlane,
                                ),
                            ),
                            diag: fault.diag(),
                        });
                    }
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(roots))) => {
                        for &t in &roots {
                            let p = curve.carrier().eval(t);
                            let containment =
                                contfp(y, face, plane.normal, p, band).map_err(|e| esc(e, x_is))?;
                            if !matches!(containment, FaceContainment::Out)
                                && let Some(tr) = trace.as_deref_mut()
                            {
                                tr.accepted.push((edge_key, face));
                            }
                            match containment {
                                FaceContainment::Out => {}
                                FaceContainment::In => {
                                    let w = split_at(x, x_is, edge_key, t, tol)?;
                                    contacts.vf(x_is, VfContact { vertex: w, face });
                                    requeue(&mut worklist, x, edge_key, w, j)?;
                                    break 'faces;
                                }
                                FaceContainment::OnEdge(ey) => {
                                    let w = split_at(x, x_is, edge_key, t, tol)?;
                                    let wy =
                                        split_other_at_point(y, x_is.other(), ey, p, band, tol)?;
                                    push_vv(contacts, x_is, w, wy);
                                    requeue(&mut worklist, x, edge_key, w, j)?;
                                    break 'faces;
                                }
                                FaceContainment::OnVertex(vy) => {
                                    let w = split_at(x, x_is, edge_key, t, tol)?;
                                    push_vv(contacts, x_is, w, vy);
                                    requeue(&mut worklist, x, edge_key, w, j)?;
                                    break 'faces;
                                }
                            }
                        }
                        // No interior root lands in the face:
                        // endpoint processing only.
                        let side = |p: Point3<T>| {
                            decide(
                                "bool_vertex_face_side",
                                Margin::of((p - plane.origin).dot(plane.normal)),
                                band,
                            )
                        };
                        let read =
                            edge_face_read(x, x_is, &edge, face, declared, Coincide::VertexOnFace);
                        let on_face =
                            |diag| BooleanError::coincidence(Coincide::VertexOnFace, read, diag);
                        let s1 = side(pu).map_err(on_face)?;
                        let s2 = side(pv).map_err(on_face)?;
                        let mut hit = false;
                        if s1 == Sign::Zero {
                            hit |=
                                vertex_on_face(x_is, y, u, pu, face, &plane, contacts, band, tol)?;
                        }
                        if s2 == Sign::Zero {
                            hit |=
                                vertex_on_face(x_is, y, v, pv, face, &plane, contacts, band, tol)?;
                        }
                        if hit && let Some(tr) = trace.as_deref_mut() {
                            tr.accepted.push((edge_key, face));
                        }
                        continue;
                    }
                }
            }
            let side = |p: Point3<T>| {
                decide(
                    "bool_vertex_face_side",
                    Margin::of((p - plane.origin).dot(plane.normal)),
                    band,
                )
            };
            let read = edge_face_read(x, x_is, &edge, face, declared, Coincide::VertexOnFace);
            let on_face = |diag| BooleanError::coincidence(Coincide::VertexOnFace, read, diag);
            let s1 = side(pu).map_err(on_face)?;
            let s2 = side(pv).map_err(on_face)?;
            match (s1, s2) {
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
                    // Proper plane crossing: locate p on the carrier and
                    // classify it against the face.
                    let curve = certified(x.get_curve_geom(edge.curve))?.clone();
                    let (t0, t1) = curve.params();
                    let d1 = (pu - plane.origin).dot(plane.normal);
                    let d2 = (pv - plane.origin).dot(plane.normal);
                    let t = t0 + (t1 - t0) * (d1 / (d1 - d2));
                    let p = curve.carrier().eval(t);
                    let containment =
                        contfp(y, face, plane.normal, p, band).map_err(|e| esc(e, x_is))?;
                    if !matches!(containment, FaceContainment::Out)
                        && let Some(tr) = trace.as_deref_mut()
                    {
                        tr.accepted.push((edge_key, face));
                    }
                    match containment {
                        FaceContainment::Out => {}
                        FaceContainment::In => {
                            let w = split_at(x, x_is, edge_key, t, tol)?;
                            contacts.vf(x_is, VfContact { vertex: w, face });
                            requeue(&mut worklist, x, edge_key, w, j + 1)?;
                            break 'faces;
                        }
                        FaceContainment::OnEdge(ey) => {
                            let w = split_at(x, x_is, edge_key, t, tol)?;
                            let wy = split_other_at_point(y, x_is.other(), ey, p, band, tol)?;
                            push_vv(contacts, x_is, w, wy);
                            requeue(&mut worklist, x, edge_key, w, j + 1)?;
                            break 'faces;
                        }
                        FaceContainment::OnVertex(vy) => {
                            let w = split_at(x, x_is, edge_key, t, tol)?;
                            push_vv(contacts, x_is, w, vy);
                            requeue(&mut worklist, x, edge_key, w, j + 1)?;
                            break 'faces;
                        }
                    }
                }
                // Endpoint(s) on the face plane: `dovertexonface`
                // (steps 2–4, 7–8). A fully coplanar pair (Zero, Zero)
                // deliberately gets endpoint treatment ONLY (module
                // docs: interior events surface via neighbor faces).
                (za, zb) => {
                    let mut hit = false;
                    if za == Sign::Zero {
                        hit |= vertex_on_face(x_is, y, u, pu, face, &plane, contacts, band, tol)?;
                    }
                    if zb == Sign::Zero {
                        hit |= vertex_on_face(x_is, y, v, pv, face, &plane, contacts, band, tol)?;
                    }
                    if hit && let Some(tr) = trace.as_deref_mut() {
                        tr.accepted.push((edge_key, face));
                    }
                }
            }
        }
    }
    Ok(())
}

/// An edge end: the vertex and its point.
type End<T> = (VertexKey, Point3<T>);

/// An edge's two ends, `start(he_plus)` then `start(he_minus)`.
fn edge_ends<T: Decide>(
    x: &Body<T>,
    edge: &crate::entity::Edge,
) -> Result<(End<T>, End<T>), BooleanError> {
    let vert = |he| -> Option<End<T>> {
        let vk = x.get_half_edge(he)?.start;
        Some((vk, *x.get_point(x.get_vertex(vk)?.point)?))
    };
    match (vert(edge.he_plus), vert(edge.he_minus)) {
        (Some(a), Some(b)) => Ok((a, b)),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "edge endpoints unresolvable",
        }),
    }
}

/// **The reduction sweep, both directions** (D9: A's edges first),
/// then the deferred touches settled on what both directions split
/// ([`settle_deferred`]). Every boolean driver sweeps through here.
///
/// `T: Decide + Bounds` is the driver seam [`sweep_direction`] carries
/// (the face tree reads brackets): this forwards to it and adds no
/// bracket read of its own.
#[allow(clippy::too_many_arguments)] // the two directions' knobs and traces, side by side
pub(super) fn sweep_and_settle<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &mut Body<T>,
    b: &mut Body<T>,
    declared: &super::DeclaredPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    strategy: SweepStrategy,
    [ab_knobs, ba_knobs]: [&SweepKnobs; 2],
    [mut ab_trace, mut ba_trace]: [Option<&mut SweepTrace>; 2],
    tol: Tol,
) -> Result<(), BooleanError> {
    let mut deferred = Vec::new();
    sweep_direction(
        a,
        b,
        Operand::A,
        declared,
        contacts,
        band,
        strategy,
        ab_knobs,
        ab_trace.as_deref_mut(),
        &mut deferred,
        tol,
    )?;
    sweep_direction(
        b,
        a,
        Operand::B,
        declared,
        contacts,
        band,
        strategy,
        ba_knobs,
        ba_trace.as_deref_mut(),
        &mut deferred,
        tol,
    )?;
    settle_deferred(
        a,
        b,
        deferred,
        declared,
        contacts,
        band,
        [ab_trace, ba_trace],
        tol,
    )
}

/// A covered line × curved-face pair whose touch lies inside the edge
/// ([`CurvedEvent::Deferred`]), deferred until both sweep directions
/// have run.
#[derive(Debug)]
pub(super) struct DeferredTouch {
    /// The operand the edge belongs to.
    x_is: Operand,
    /// The edge as it was read; a split keeps this key on its leading
    /// fragment.
    edge: EdgeKey,
    /// The edge's far end, `start(he_minus)`, where its fragments stop.
    end: VertexKey,
    /// The other operand's curved face.
    face: FaceKey,
    /// The typed frontier the pair answers if its fragments do not
    /// settle it.
    refusal: BooleanError,
}

/// **Settles the deferred touches on the edges' fragments**, after both
/// sweep directions have run.
///
/// Which operand's edges are swept first decides only which vertices
/// exist when a pair is read, and settling the deferred pairs last is
/// what makes that order immaterial to whether a covered touch is seen:
/// a touch inside an edge of one operand very often sits at a vertex of
/// the other (a fillet's tangent point, where its flat wall ends), and
/// that vertex splits the edge only when the other direction reaches
/// it. What settling adds is the ACCEPTANCE: the touch's records are
/// the other direction's (its vertex on this edge), and settling is
/// what licenses the pair once that vertex has put the touch at a
/// fragment's end, where the deferring direction had to refuse.
///
/// Each fragment, walked from the deferred key along `he_plus` to the
/// edge's far end, is read again by [`curved_face_arm`] against the
/// deferred face; a pair it accepts is written to its direction's
/// trace, as the sweep writes one. A fragment that clears or records is
/// done; one whose touch is still inside it, or that the arm reads as a
/// crossing, answers the pair's typed frontier. So does a pair nothing
/// split, read again exactly as it was deferred.
#[allow(clippy::too_many_arguments)]
pub(super) fn settle_deferred<T: Decide + crate::props::AtRestPolicy>(
    a: &mut Body<T>,
    b: &mut Body<T>,
    deferred: Vec<DeferredTouch>,
    declared: &super::DeclaredPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    mut traces: [Option<&mut SweepTrace>; 2],
    tol: Tol,
) -> Result<(), BooleanError> {
    for d in deferred {
        let (x, y, trace): (&Body<T>, &mut Body<T>, _) = match d.x_is {
            Operand::A => (&*a, &mut *b, traces[0].as_deref_mut()),
            Operand::B => (&*b, &mut *a, traces[1].as_deref_mut()),
        };
        let mut trace = trace;
        let mut fragment = d.edge;
        // A fragment per pass; the walk ends at the deferred edge's far
        // end within one pass per edge of `x`.
        let mut reached = false;
        for _ in 0..x.edges().count() {
            let edge =
                x.get_edge(fragment)
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "a deferred edge's fragment vanished",
                    })?;
            let ((u, pu), (v, pv)) = edge_ends(x, &edge)?;
            match curved_face_arm(
                x, y, d.x_is, fragment, &edge, u, v, d.face, pu, pv, declared, contacts, band, tol,
            )? {
                CurvedEvent::None => {}
                CurvedEvent::Recorded => {
                    if let Some(tr) = trace.as_deref_mut() {
                        tr.accepted.push((fragment, d.face));
                    }
                }
                CurvedEvent::Deferred | CurvedEvent::Pierce { .. } => return Err(d.refusal),
            }
            if v == d.end {
                reached = true;
                break;
            }
            // A split leaves its trailing child's `he_plus` next after
            // the parent's, starting at the minted vertex.
            let next_key = x.get_half_edge(edge.he_plus).map(|he| he.next);
            let next = next_key.and_then(|k| Some((k, x.get_half_edge(k)?)));
            let Some((next_key, next)) = next else {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a deferred edge's fragment chain is unresolvable",
                });
            };
            let leads = x.get_edge(next.edge).map(|e| e.he_plus) == Some(next_key);
            if next.start != v || !leads {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a deferred edge's fragments do not chain",
                });
            }
            fragment = next.edge;
        }
        if !reached {
            return Err(BooleanError::ClassificationInvariant {
                what: "a deferred edge's fragments do not reach its far end",
            });
        }
    }
    Ok(())
}

/// The curved-face sweep arm: endpoint sides come from the linearized
/// implicit residual; a definite miss is PROVEN — for a LINE carrier
/// against a cylinder or sphere the residual is convex (both-inside
/// means no wall crossing, both-outside clears through the span
/// minimum), against a torus the certified quartic's roots decide, and
/// for a CONIC carrier (a circle or an ellipse, `geom_brep::Conic`) the
/// ARC's residual range is enclosed two ways (the carrier's exact
/// harmonic bounds and the arc's own chord-dip bound), so a definitely
/// one-sided arc clears. What definitely MEETS the face is split by
/// kind, and the third paragraph below is the statement of record: a
/// LINE or a CIRCLE carrier against a CYLINDER wall, a SPHERE or a
/// TORUS, and an ELLIPSE against a cylinder wall or a sphere, is routed
/// through the certified roots and pierces; everything else — a
/// tangency, a cone, an ellipse against a torus, an undeclared
/// on-carrier edge, a trim with no verdict — refuses typed at the named
/// frontier door ([`BooleanError::CurvedPierceUnsupported`]). An
/// in-band clearance escalates (F6, the same margin's other half) —
/// except an uncovered conic's against one of those three kinds, where
/// the certified roots decide what the enclosures could not. Spiric and
/// NURBS carriers have no enclosure and take the frontier door before
/// any clearance test (behind the operand gate, which refuses them
/// first). Never a silent fallback.
///
/// **The carrier-identity rung** comes before any enclosure on a
/// CIRCLE carrier: an edge whose parent face is declared one carrier with
/// `face`, on a carrier the ladder calls the same, lies on `face`'s
/// carrier, so its clearance is zero by that certificate
/// ([`on_declared_shared_carrier`]). The sampled enclosure cannot say so
/// of a coincident pair — it is `±charge` about an identically-zero
/// residual and reads definitely negative.
///
/// **The one-sided cover rung** (C4's cover clause at the crossing
/// layer): a zero-clearance incidence whose edge has a parent face
/// whose carrier is CERTIFIED to lie in one closed side of `face`'s
/// ([`super::DeclaredPairs::one_sided`]) — by a verified `Rest` or
/// continuation (the edge bounds a face on the shared carrier, so it
/// lies ON that carrier), a verified `Tangent` (the on-carrier locus IS
/// the verified tangency: the ruling a tangent edge realizes), or a
/// structural tangency on either operand to a face verified one carrier
/// with `face` (a flat wall whose fillet strut is tangent to the other
/// plate's continued corner cylinder) — takes the planar sweep's
/// endpoint posture instead of the frontier door. A roots-lane
/// "tangent" verdict is a band decision and never a source, so a
/// graze within the band keeps the frontier. The cover records
/// endpoints only, so a covered pair whose ends are both clear of the
/// carrier and which no enclosure or root set clears is deferred
/// ([`CurvedEvent::Deferred`]) — a LINE against a wall or a sphere,
/// whose convex residual a fragment's ends read whole — and read again
/// on the edge's fragments
/// once both sweep directions have run ([`settle_deferred`]). Each
/// on-carrier endpoint is
/// classified through the boundary pre-pass rows
/// ([`super::contain::curved_face_containment`] — the boundary walk,
/// and behind it the cylinder chart trim), producing the same v-v
/// record family, or the v-f record when the trim places the endpoint
/// strictly inside.
///
/// **Each on-carrier endpoint has three outcomes, not two**
/// ([`Placement`]). It is RECORDED; or the trim certifies it OUT of
/// this face, in which case it is eventless HERE and the pair's other
/// endpoint decides; or the door returns no verdict, which keeps the
/// typed frontier refusal. The middle case is a decision and not a
/// remainder: a shared carrier is covered by several faces per side, so
/// an endpoint outside THIS face's window is a site some sibling face
/// holds, and the sweep reaches that pair on its own visit.
///
/// **An edge lying ON the carrier is asked about its interior first**
/// ([`interior`]): where it crosses `face`'s boundary between
/// its ends — a shaft's seam ruling passing a full-turn bore's rim, a
/// rim arc passing the partner's seam ruling — the crossing is split
/// and recorded as a pierce landing on that boundary. With the interior
/// certified clear, a pair with both ends `Elsewhere` lies wholly
/// outside the face and is no event; without that certificate (a
/// `Tangent`-covered arc, which only touches the carrier) it still
/// refuses, since an overlap inside this face's window with both
/// endpoints beyond it would be an incidence this arm cannot see.
///
/// UNDECLARED incidences unlock no RECORDING they did not already have
/// — the recording door only widens what a verified declaration
/// unlocks. Two undeclared arms do read the third outcome, and they
/// read it as SILENCE where the strict posture refused: a `NoInterior`
/// span — the MIXED-SIGN one (one endpoint ON the carrier, the other
/// definitely clear) and the `(Zero, Zero)` span with both ends on the
/// carrier — whose every ON endpoint is certified `Elsewhere` answers
/// NO EVENT on this face ([`Placement::undeclared_no_interior`]). That
/// is a claim of absence, and it rests on certificates, not on a
/// sibling face existing: the certified roots put every meeting of the
/// line with the carrier at a root, each interior root was placed
/// outside this face's trim, and each end was placed outside it too,
/// so the span meets this face nowhere. Where the ends' incidences live
/// — a pierce vertex interior to a sibling face, an operand's own
/// vertex on another face, or nowhere on this operand at all — is not
/// this pair's question.
///
/// **The pierce ring lane** (the definite-crossing half): a LINE edge
/// that definitely crosses a cylinder WALL, a SPHERE or a TORUS inside
/// that face's trim is no longer the frontier. Its crossing parameters
/// are the same certified roots the ray lane has always solved — the
/// quadratic on a wall ([`super::solid_contain::line_wall_roots`]) and
/// on a sphere ([`super::solid_contain::line_sphere_roots`]), the
/// quartic on a torus ([`super::solid_contain::line_torus_roots`]) —
/// taken over the edge's own span instead of a ray's forward half; on
/// a torus the roots are consulted for every endpoint-sign pattern,
/// since its residual is not convex along a line and no endpoint datum
/// bounds a crossing between; the landing point is
/// placed by [`super::contain::curved_face_containment`]; the
/// split/record triple is the planar conic lane's, verbatim. Still the
/// frontier is everything the roots do not cover: a TANGENCY (an
/// in-band discriminant, or a torus root count the quartic cannot
/// certify, is not a crossing at any order this lane sees), a conic
/// carrier against a cone, an ELLIPSE against a torus (its residual is a
/// degree-4 trigonometric polynomial, an octic in the half-angle, which
/// no ladder here solves), and a trim the chart door declines to
/// express.
///
/// **A CIRCLE against a SPHERE, a CYLINDER or a TORUS takes the same
/// arms as a line.** Against a sphere its residual is a first harmonic
/// with closed-form roots ([`super::circle_sphere`]); against a torus
/// or a cylinder wall it is a degree-2 trigonometric polynomial, a
/// quartic in the tangent half-angle ([`super::circle_torus`],
/// [`super::circle_cylinder`]; a circle square to the wall's axis is a
/// first harmonic again, and takes the square arm, the first-harmonic door).
/// An ELLIPSE against a sphere or a cylinder wall is a degree-2
/// trigonometric polynomial in its eccentric anomaly too
/// ([`super::ellipse_roots`]). Every degree-2 door's answer is the
/// certified subdivision's, decided on the residual itself
/// ([`super::circle_roots`]). A conic reaches those arms only from the
/// conic rung, after the enclosures failed to clear the arc, and never
/// through a one-sided cover arm — those rest on a line's separation
/// story.
///
/// A successful wall pierce reaches the join with a ring in the
/// pierced face, and the ring's chords take their arc from that face's
/// own azimuth window (`chord_join`'s `cross_loop_window_cycle`).
///
/// **This lane WIDENS what an undeclared pair reaches, and the widening
/// is named here rather than left to be discovered.** Before it,
/// [`vertex_on_curved_face`] was reachable only behind the declared
/// cover; the new endpoint arms (`(Zero, definite)` and the
/// `(Zero, Zero)` chord) call it on UNDECLARED pairs too. That is not
/// the C8 gate reopening: C8 protects the claim that an on-carrier EDGE
/// is cosurface, and the arms below reach the endpoint treatment only
/// after the certified roots have proved there is no interior crossing.
/// The `(Zero, Zero)` arm takes an edge two ways. A chord it takes on
/// distinct certified roots, which exclude an edge lying on the carrier
/// (on a cylinder only rulings do, and a ruling answers `Constant`; no
/// line lies on a torus); what the door then does is point-in-face
/// containment on a chart, which is a trim question and not a gluing
/// one. An arc lying on the carrier (`LiesOn`) it takes only when every
/// parent is decided a DIFFERENT carrier: a curve where two carriers
/// meet, so it asks no cosurface question either.
///
/// Returns what the caller must do about the pair — see
/// [`CurvedEvent`]. The split itself needs `&mut x` and the worklist,
/// both of which live in [`sweep_direction`], so the crossing is
/// REPORTED here and performed there rather than the body being
/// threaded in for one branch.
/// The one-sided covers an edge's parent faces hold against `face`
/// ([`super::DeclaredPairs::cover`]), one per covering parent.
fn edge_covers<T: Decide>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::DeclaredPairs<T>,
) -> Vec<super::CoverSide> {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .filter_map(|f| declared.cover(x_is, f, x_is.other(), face))
    .collect()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn curved_face_arm<T: Decide + crate::props::AtRestPolicy>(
    x: &Body<T>,
    y: &mut Body<T>,
    x_is: Operand,
    edge_key: EdgeKey,
    edge: &crate::entity::Edge,
    u: VertexKey,
    v: VertexKey,
    face: FaceKey,
    pu: Point3<T>,
    pv: Point3<T>,
    declared: &super::DeclaredPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<CurvedEvent<T>, BooleanError> {
    let surface = y
        .get_face(face)
        .and_then(|f| y.get_surface(f.surface))
        .cloned()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "curved sweep arm: face surface lost",
        })?;
    let frontier = || BooleanError::CurvedPierceUnsupported {
        operand: x_is,
        face,
        edge: edge_key,
        band,
    };
    // The one-sided cover (docs above): one of the edge's parent faces
    // has its carrier certified to lie in one closed side of `face`'s
    // carrier, so the edge's residual against it is one-signed and an
    // on-carrier endpoint is a touch, never an entry. The numeric rows
    // then certify each incidence. `read` is what each question below
    // reads of the parent faces' declaration against `face`: none of
    // them is one a declaration settles.
    let covers = edge_covers(x, x_is, edge, face, declared);
    let covered = !covers.is_empty();
    // Whether a point of the edge whose residual against `face`'s
    // carrier decided `sign` lies where some parent's cover certifies it.
    let admitted = |sign| covers.iter().any(|c| c.admits(sign));
    let read = |which| edge_face_read(x, x_is, edge, face, declared, which);
    // NURBS walls (shape (iii)'s substrate): the SECTION arm is
    // certified since PR 7b (geom_brep::intersect::route says so),
    // but the boolean's CROSSING layer for the kind — edge×NURBS-face
    // sweep events and curved trim containment — does not exist. M5
    // PR 9c was the banked unit for it and did NOT land it (M5-LOG
    // PR 9c, deviation 5): the residual sides a crossing layer needs
    // are `implicit_residual` and `classify_dihedral`, both poison on
    // a NURBS surface, and the only non-poison substitute is a
    // foot-point projection that exists at `f64` ONLY
    // (`NurbsSurface::project` is an `impl NurbsSurface<f64>` block),
    // so wiring it would kill the Interval lane. Refused typed HERE,
    // before the residual sides — poison is not a refusal.
    //
    // **`Approx` refuses on the same terms, stated rather than
    // inherited.** Its geometry is a spline fit, so `implicit_residual`
    // and `classify_dihedral` are poison on it too. The operand gate
    // does refuse the kind earlier, which makes this site unreachable
    // in the pipeline as it stands — but that is a fact about the
    // CALLER, and an unstated nesting invariant is exactly how a
    // poison path gets re-entered when a gate later narrows. The arm
    // is written for the same reason the extent scan's is.
    if let Some(kind) = match surface {
        geom::Surface::Nurbs(_) => Some(geom::SurfaceKind::Nurbs),
        geom::Surface::Approx(_) => Some(geom::SurfaceKind::Approx),
        geom::Surface::Plane { .. }
        | geom::Surface::Cylinder { .. }
        | geom::Surface::Cone { .. }
        | geom::Surface::Sphere { .. }
        | geom::Surface::Torus { .. } => None,
    } {
        return Err(BooleanError::CurvedBooleanUnsupported {
            operand: x_is,
            face,
            kind,
        });
    }
    let curve = certified(x.get_curve_geom(edge.curve))?.clone();
    // Conic carriers: a CIRCLE carrier gets a definite-miss verdict in
    // closed form, and the verdict is the edge's ARC, not the carrier
    // it rides. Two enclosures of the residual are folded, and the
    // better one decides:
    //
    // - **the carrier's**: the residual over the WHOLE circle is a
    //   degree-≤2 trigonometric polynomial against a sphere/cylinder,
    //   so its range has exact amplitude bounds
    //   (`geom_brep::circle_residual_extremes`). Tight for a full-turn
    //   edge; for a short arc it answers about geometry the edge does
    //   not occupy, which is what made a corner round's carrier — not
    //   its arc — decide a cut (#347).
    // - **the arc's**: the residual is SAMPLED across the arc at
    //   `geom_brep::ARC_RESIDUAL_SAMPLES` steps and the sample hull is
    //   widened by one sub-arc's chord-dip charge — a smooth function
    //   stays within `|F″|·h²/8` of the chord of a sub-arc of width
    //   `h` (`geom_brep::circle_arc_residual_range`). Tight for a
    //   short arc, useless for a full turn, and it is what gives the
    //   torus a verdict at all: the torus's composed residual has no
    //   harmonic form, so the sampled enclosure is the only one it
    //   has, on the arc and on the whole turn alike.
    //
    // Both are valid enclosures of the ARC's range, so the clearance
    // margin is the larger of the two one-sidedness margins: definitely
    // one-sided — the arc strictly outside, or strictly inside — means
    // no wall crossing (meters). Anything else keeps the typed frontier
    // door, and an in-band clearance escalates (two-tolerance on the
    // arm, definite ones included). The rung is the CONIC's: an ellipse
    // reads the same two enclosures (`geom_brep::Conic`).
    let side = |p: Point3<T>| {
        decide(
            "bool_vertex_face_side",
            Margin::of(geom_brep::implicit_residual(&surface, p)),
            band,
        )
    };
    // **An uncovered arc with an end ON the carrier is never asked its
    // clearance.** Its residual is exactly zero at that end, so its true
    // one-sidedness margin is at most zero and the clearance can never
    // read `Positive`, the one answer that returns early. Every other
    // answer an uncovered arc against these kinds can get (`Zero`,
    // `Negative`, escalated) falls through to the endpoint arms below.
    // So asking would decide nothing. What it would RECORD is the
    // sampled enclosure's own chord-dip charge, read as `−charge` about
    // that zero end: a margin of the enclosure, not of the geometry,
    // ε-independent and micrometres small, which the K telemetry reads as
    // a feature crowding its floor. That is the split fragment of a
    // carved sphere's meridian, ending on the cut. The endpoint sides
    // are therefore decided first, here, and handed to the endpoint arms
    // rather than decided twice. A held escalation surfaces where it
    // always did, in those arms. It is dropped only when the clearance
    // reads definitely clear, which an in-band end cannot let happen.
    let early_ends = (!covered
        && matches!(
            curve.carrier(),
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }
        )
        && matches!(
            surface,
            geom::Surface::Torus { .. }
                | geom::Surface::Sphere { .. }
                | geom::Surface::Cylinder { .. }
        ))
    .then(|| (side(pu), side(pv)));
    let end_on_carrier = matches!(early_ends, Some((Ok(Sign::Zero), _) | (_, Ok(Sign::Zero))));
    match (curve.carrier(), geom_brep::Conic::of(curve.carrier())) {
        (geom::Curve3::Line { .. }, _) => {}
        (geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }, Some(conic)) => 'clearance: {
            if end_on_carrier {
                break 'clearance;
            }
            // **The carrier-identity rung, consulted FIRST.** An edge
            // bounding a face whose carrier the door verified to BE
            // `face`'s carrier lies on `face`'s carrier identically, so
            // its clearance is zero by that certificate — not by an
            // enclosure. The enclosures below cannot say so: the
            // sampled one is `±charge` about an identically-zero
            // residual and reads definitely negative by construction,
            // and the harmonic one has no torus form. Reading the
            // certificate first is what lets a coincident pair reach
            // the one-sided cover rung at all.
            let on_carrier = on_declared_shared_carrier(x, x_is, edge, face, declared);
            // **A covered arc lying on a carrier its parents are all
            // decided distinct from is an ON event first**, the
            // uncovered `(Zero, Zero)` arm's `LiesOn` lane: a seam's
            // rim lies on the partner's carrier, and the cover must not
            // pre-empt the lane that places it. The sampled clearance
            // below reads an identically-zero residual as definitely
            // negative, which under a cover is the frontier, and the
            // roots that answer `LiesOn` sit behind the uncovered arm.
            if covered && !on_carrier {
                let (t0, t1) = curve.params();
                if let Ok(SpanVerdict::LiesOn) =
                    wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)
                    && parents_distinct_from(x, edge, y, face, band)
                {
                    let arc = ArcOnCarrier {
                        x,
                        x_is,
                        edge_key,
                        ends: [(u, pu), (v, pv)],
                        face,
                        band,
                        tol,
                    };
                    return lying_on(&arc, y, contacts)?.ok_or_else(frontier);
                }
            }
            let clearance = if on_carrier {
                Ok(Sign::Zero)
            } else {
                conic_clearance(&surface, &conic, curve.params(), band).ok_or_else(frontier)?
            };
            match clearance {
                Ok(Sign::Positive) => return Ok(CurvedEvent::None),
                // The one-sided cover rung: a covered zero-clearance
                // circle takes the planar sweep's endpoint posture —
                // each endpoint's own side decides its treatment
                // (existing row): ON the carrier ⇒ boundary
                // containment (which must decide, or the frontier
                // stands); definitely off on the side the cover
                // certifies ⇒ no event at that end (a covered circle
                // touches the carrier where it meets it); off on the
                // other side ⇒ the frontier. An interior-only touch (no
                // endpoint on the carrier) keeps the frontier door: along
                // a circle the residual is not convex, so a covered arc
                // can touch twice and an endpoint reading of its
                // fragments would not see the second touch. Uncovered
                // keeps both doors verbatim.
                //
                // **An endpoint the trim places definitely OUT of THIS
                // face is eventless HERE, not a refusal.** Reading that
                // certificate as the frontier's remainder refuses a
                // pair that has no incidence to report — and refuses
                // the whole op, since one refusing pair is enough.
                //
                // `Out` has TWO sources on the carrier, and the argument differs by
                // source, so it is made per source rather than for the
                // one that motivated the change:
                //
                // - **AZIMUTH** — the endpoint is past this face's
                //   angular window. On a closed carrier the azimuth is
                //   covered by the operand's own wall faces, so the
                //   point is a seam site a SIBLING face holds, and the
                //   sweep reaches that pair on its own visit. This is
                //   the case the rest lane is built on.
                // - **HEIGHT** — the endpoint is past the window in z.
                //   Here NO sibling need hold it: the carrier simply
                //   ends, and a floating peg's rim has no face of the
                //   other operand under it at all.
                //
                // An OFF-CARRIER answer is not a third source: the
                // endpoint's residual decided `Zero` just above, so a
                // containment that puts it definitely off the carrier
                // contradicts that decision, and
                // [`vertex_on_curved_face`] reports it `Undecided`.
                //
                // The HEIGHT source has no neighbour to appeal to, so the
                // widening does NOT rest on one existing. What carries
                // it is the **interior question**, asked before either
                // end: an arc lying on the carrier that crosses this
                // face's boundary between its ends is split there
                // ([`interior`]), and only an arc whose
                // interior is certified clear of the boundary reads an
                // all-`Elsewhere` pair as lying wholly outside the face
                // ([`Placement::declared`]).
                //
                // Still keeping the door: a no-verdict endpoint
                // ([`Placement::Undecided`]); a boundary the interior
                // question has no closed form for; and an arc that only
                // touches the carrier (a `Tangent` cover, no carrier
                // identity) with nothing recorded, whose interior this
                // arm cannot see.
                Ok(Sign::Zero) if covered => {
                    let inside = interior(on_carrier, y, x_is, face, &curve, band, frontier)?;
                    if let Interior::Crossing(event) = inside {
                        return Ok(event);
                    }
                    let side = |p: Point3<T>| {
                        decide(
                            "bool_vertex_face_side",
                            Margin::of(geom_brep::implicit_residual(&surface, p)),
                            band,
                        )
                    };
                    let mut ends = [None, None];
                    for (i, (w, pw)) in [(u, pu), (v, pv)].into_iter().enumerate() {
                        match side(pw).map_err(|diag| {
                            let which = Coincide::VertexOnCoveredFace;
                            BooleanError::coincidence(which, read(which), diag)
                        })? {
                            Sign::Zero => {
                                ends[i] = Some(vertex_on_curved_face(
                                    x_is, y, w, pw, face, contacts, band, tol,
                                )?);
                            }
                            // Definitely off at this end, on the side the
                            // certificate puts the parent: honestly
                            // eventless, and not an endpoint the rule
                            // below weighs either way. Off on the OTHER
                            // side is a crossing the certificate rules
                            // out, and keeps the door.
                            sign if admitted(sign) => {}
                            Sign::Positive | Sign::Negative => return Err(frontier()),
                        }
                    }
                    return Placement::declared(ends, inside.clear()).ok_or_else(frontier);
                }
                // **The circle × sphere, × cylinder and × torus root
                // lanes.** An arc the enclosures could not clear against
                // one of those kinds takes the same endpoint-sign arms as
                // a line below, and the certified roots of
                // [`super::circle_sphere`] / [`super::circle_cylinder`] /
                // [`super::circle_torus`] decide every one of them
                // through [`wall_crossing`].
                // Those arms never read convexity for a circle (the
                // same-side pairs go to the roots, as a torus's do), so
                // nothing they conclude rests on the carrier being
                // straight. An
                // ESCALATED clearance goes the same way: the enclosures
                // are a shortcut in front of the roots, and a margin in
                // their escalation gap says only that the shortcut did
                // not decide — the roots still can.
                //
                // **Only an UNCOVERED circle falls through.** The
                // one-sided cover arms below rest on a line's separation
                // story, so a covered circle — whose covered-Zero case
                // returned above — keeps the frontier door here rather
                // than reaching them. That makes those arms structurally
                // line-only, which they assert.
                Ok(Sign::Zero | Sign::Negative) | Err(_)
                    if !covered
                        && matches!(
                            surface,
                            geom::Surface::Torus { .. }
                                | geom::Surface::Sphere { .. }
                                | geom::Surface::Cylinder { .. }
                        ) => {}
                // Uncovered, an arc reaching here with a decided
                // clearance is against a kind with no root lane; covered,
                // it definitely crosses. The frontier door either way.
                Ok(Sign::Zero | Sign::Negative) => return Err(frontier()),
                // An uncovered ESCALATED clearance cannot reach here: a
                // plane face never reaches this arm (`face_plane` routes it
                // to the planar lane first), and cone, NURBS and `Approx`
                // have no clearance enclosure, so they returned the
                // frontier before deciding one. The three kinds left are
                // the arm above's. Reaching here is a dispatch desync.
                Err(_) if !covered => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "an uncovered arc's escalated clearance reached the covered \
                               arm: the circle rung's kinds and its root lanes disagree",
                    });
                }
                // A declared pair's declaration is spent here. An
                // undeclared one, declared `Rest` on this face's
                // carrier, is on it by the carrier-identity rung above,
                // and takes the covered arm, whose endpoint sides then
                // refuse the same in-band pose: no declaration settles
                // it.
                Err(diag) => {
                    let which = Coincide::ArcOnCoveredFace;
                    return Err(BooleanError::coincidence(which, read(which), diag));
                }
            }
        }
        // A `Spiric` or `Nurbs` carrier: no enclosure of its residual
        // along an arc exists here (the sampled one needs bounds on the
        // carrier's speed and acceleration over the span), so nothing
        // finds its crossings, as at the planar arm.
        (geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_), _) => {
            return Err(BooleanError::CrossingCarrierUnsupported {
                operand: x_is,
                edge: edge_key,
                face,
            });
        }
        (geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }, None) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a circle or an ellipse carrier has no conic frame",
            });
        }
    }
    // The one-sided cover arms rest on a LINE's separation story; only an
    // uncovered circle reaches the endpoint arms (the circle rung above).
    let on_line = matches!(curve.carrier(), geom::Curve3::Line { .. });
    let on_face = |diag| {
        let which = Coincide::VertexOnCurvedFace;
        BooleanError::coincidence(which, read(which), diag)
    };
    let (s1, s2) = match early_ends {
        Some((s1, s2)) => (s1.map_err(on_face)?, s2.map_err(on_face)?),
        None => (side(pu).map_err(on_face)?, side(pv).map_err(on_face)?),
    };
    match (s1, s2) {
        // The one-sided cover rung: a covered line with endpoint(s) ON
        // the carrier takes the planar sweep's endpoint posture — the
        // `(za, zb)` branch mirrored: each Zero endpoint gets boundary
        // containment (which must decide, or the frontier stands).
        // What makes the `(Zero, Positive)` branch eventless is NOT
        // convexity (a convex residual's endpoint bound is its
        // MAXIMUM, not its minimum — q(0) = 0, q(1) > 0 can dip
        // negative between): it is the cover's invariant — the edge's
        // parent carrier lies wholly in ONE closed residual half-space
        // of `face`'s ([`super::DeclaredPairs::one_sided`]; for a
        // `Tangent` source that is the witness lane's separation
        // invariant, the contract sentence on
        // [`geom_brep::tangent_locus`], and a one-carrier source's
        // shared carrier is residual-zero identically) — so a covered
        // on-carrier edge's residual is one-signed and a Zero endpoint
        // is a touch, never an entry.
        // A configuration without that one-sign story must not be
        // admitted to the lane. The coaxial cylinder×sphere pair has
        // one (`crates/topo/tests/verbs_cylsph_tangent_residuals.rs`);
        // what keeps its circle arm out — no consumer builds the kiss
        // edge yet, and the arm must not pre-empt the rim routing — is
        // stated at [`geom_brep::tangent_locus`]. A NEGATIVE partner is
        // a genuine crossing — never the covered posture. Uncovered
        // keeps both frontier doors verbatim.
        // Both ends on the carrier. Where the carrier identity puts the
        // line ON it (a parent face verified as `face`'s carrier), its
        // interior is asked before its ends; a `Tangent`-covered line has
        // no such certificate and keeps the ends-only rule. The ends-only
        // rule rests on the cover, not on the line's shape: the line lies
        // on its parent's carrier, which the certificate holds in one
        // closed side of `face`'s, so its residual is one-signed and its
        // interior touches the carrier nowhere it crosses it. (It need
        // not be a ruling: under a plane × torus seam a chord of the
        // tangent circle ends on the torus and runs inside the circle.)
        (Sign::Zero, Sign::Zero) if covered => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let on_carrier = on_declared_shared_carrier(x, x_is, edge, face, declared);
            let inside = interior(on_carrier, y, x_is, face, &curve, band, frontier)?;
            if let Interior::Crossing(event) = inside {
                return Ok(event);
            }
            let hu = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
            let hv = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
            Placement::declared([Some(hu), Some(hv)], inside.clear()).ok_or_else(frontier)
        }
        (Sign::Zero, Sign::Positive) if admitted(Sign::Positive) => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let h = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
            Placement::declared([Some(h), None], false).ok_or_else(frontier)
        }
        (Sign::Positive, Sign::Zero) if admitted(Sign::Positive) => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let h = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
            Placement::declared([Some(h), None], false).ok_or_else(frontier)
        }
        // **One endpoint ON the surface, the other definitely off it.**
        // This is the shape a wall crossing LEAVES behind: the split
        // puts a vertex on the carrier and re-queues both fragments
        // against the same face, so each comes back with exactly one
        // Zero end. The exact roots are consulted FIRST, because an
        // interior crossing is still possible here and the endpoint
        // rows would silently miss it — the residual along a line is
        // convex and therefore lies BELOW its endpoint chord, so
        // `f(t₀) = 0` with `f(t₁) > 0` can still dip through zero
        // between. With no interior root the incidence is exactly the
        // endpoint's, and the v-on-curved-face door mints the same
        // record the planar sweep's `vertex_on_face` writes.
        //
        // `(Zero, Zero)` is NOT here: both endpoints on the carrier is
        // where the on-carrier-edge question could arise, so it has its
        // own arm below, which separates the chord from that question
        // structurally before it answers anything.
        (Sign::Zero, Sign::Positive | Sign::Negative)
        | (Sign::Positive | Sign::Negative, Sign::Zero) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::Pierce { t, p, at } => Ok(CurvedEvent::Pierce { t, p, at }),
                // A constant residual cannot be zero at one end and
                // definite at the other, and a definite miss cannot
                // hold a zero endpoint: both CONTRADICT the endpoint
                // rows that routed here, and a contradiction between
                // two certified predicates keeps the door.
                SpanVerdict::Constant
                | SpanVerdict::LiesOn
                | SpanVerdict::Miss
                | SpanVerdict::Unsettled => Err(frontier()),
                SpanVerdict::NoInterior | SpanVerdict::Elsewhere => {
                    // UNDECLARED: the undeclared `NoInterior` rule
                    // ([`Placement::undeclared_no_interior`]), over this
                    // span's one ON end.
                    let mut ends = [None, None];
                    for (i, (on, w, pw)) in [(s1 == Sign::Zero, u, pu), (s2 == Sign::Zero, v, pv)]
                        .into_iter()
                        .enumerate()
                    {
                        if on {
                            ends[i] = Some(vertex_on_curved_face(
                                x_is, y, w, pw, face, contacts, band, tol,
                            )?);
                        }
                    }
                    Placement::undeclared_no_interior(ends).ok_or_else(frontier)
                }
            }
        }
        // **BOTH endpoints on the carrier, the pair UNDECLARED** — the
        // chord between two pierces, or any edge whose two ends sit on
        // the carrier. The verdict decides which of two edges it is:
        //
        // - **`NoInterior` / `Elsewhere`: not an on-carrier edge.** Those
        //   are reached only through a certified root count with
        //   distinct roots. For a LINE: the lines that lie on a wall are
        //   its rulings, which answer `Constant`, and no line lies on a
        //   sphere or a torus. For a CIRCLE against a sphere: a circle
        //   lying on it is centred on its axis, answered `LiesOn`. For a
        //   CIRCLE against a torus: a circle lying on it is either
        //   coaxial (a rim or latitude circle, answered `LiesOn`) or has
        //   `F ≡ 0`, whose pole no anchor can put definitely off the
        //   torus (`Unsettled`). And its interior meets this face
        //   nowhere: the edge meets the carrier only at its certified
        //   roots, each root strictly inside the span was placed outside
        //   the trim, and each root at an end is that end's own
        //   incidence. So the ends decide, under the same rule as the
        //   mixed-sign arm ([`Placement::undeclared_no_interior`]).
        // - **`LiesOn`: an arc lying on the carrier**, exactly on by the
        //   circle root door. It is an ON event (C4's one-sided cover,
        //   narrowed to touches) when every surface of a face it bounds is
        //   decided distinct from `face`'s by the carrier ladder: the arc
        //   is then a curve where two different carriers meet, not a
        //   cosurface question. It takes the coplanar conic's posture,
        //   endpoint processing only, once its interior is certified to
        //   meet this face's boundary nowhere it does not run along
        //   ([`lying_on`]).
        //
        // Every other answer keeps the door: the undeclared cosurface
        // question (CONTACT-DESIGN C2/C4), a parent the ladder does not
        // decide distinct, a tangency, and a trim with no verdict.
        (Sign::Zero, Sign::Zero) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::NoInterior | SpanVerdict::Elsewhere => {
                    let hu = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
                    let hv = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
                    Placement::undeclared_no_interior([Some(hu), Some(hv)]).ok_or_else(frontier)
                }
                SpanVerdict::LiesOn if parents_distinct_from(x, edge, y, face, band) => {
                    let arc = ArcOnCarrier {
                        x,
                        x_is,
                        edge_key,
                        ends: [(u, pu), (v, pv)],
                        face,
                        band,
                        tol,
                    };
                    lying_on(&arc, y, contacts)?.ok_or_else(frontier)
                }
                _ => Err(frontier()),
            }
        }
        // **A definite surface crossing: the pierce RING lane.** The
        // endpoints straddle the carrier, so a root exists in the span;
        // `wall_crossing` finds it exactly and the trim decides whether
        // it lands in this face. A root set that ACCOUNTS for the
        // straddle off this face ([`SpanVerdict::Elsewhere`]) is no
        // event here; a `NoInterior` that does not — no root inside the
        // span, or one at an end the endpoint signs call definite —
        // CONTRADICTS the straddle, so it keeps the door rather than
        // reporting no event: a contradiction between two certified
        // predicates is exactly what must not be resolved by picking
        // one.
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::Pierce { t, p, at } => Ok(CurvedEvent::Pierce { t, p, at }),
                // The straddle's crossing, found and certified OFF this
                // face: every root inside the span is on the carrier
                // outside the trim, and both ends are definitely off the
                // carrier, so the span meets this face nowhere. Where the
                // crossing's incidence lives — a sibling face on the same
                // carrier, or no face of this operand — is that face's
                // pair, not this one.
                SpanVerdict::Elsewhere => Ok(CurvedEvent::None),
                _ => Err(frontier()),
            }
        }
        // **Both endpoints on one side of a TORUS, or of anything along
        // a CIRCLE: the roots decide, with no endpoint bound in front of
        // them.** The two arms below lean on the residual being CONVEX
        // along a line, which holds for a cylinder and a sphere and fails
        // for a torus: its residual `((ρ − R)² + h² − r²)/2r` carries
        // `−2Rρ`, concave in the line parameter. So a segment with both
        // ends inside the tube can leave it and come back (a chord across
        // the hole), and one with both ends outside can dip through it
        // anywhere — no endpoint datum bounds either. Along a circle no
        // kind's residual is convex (against a sphere it is
        // `c₀ + A₁cos(θ − φ)`), so an arc takes this arm whatever the
        // face. The certified roots do:
        // an interior root in this face's trim is a pierce, a certified
        // absence of one is no event HERE, and an uncertain count keeps
        // the door.
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative)
            if !on_line || matches!(surface, geom::Surface::Torus { .. }) =>
        {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::Pierce { t, p, at } => Ok(CurvedEvent::Pierce { t, p, at }),
                SpanVerdict::NoInterior | SpanVerdict::Elsewhere | SpanVerdict::Miss => {
                    Ok(CurvedEvent::None)
                }
                SpanVerdict::Constant | SpanVerdict::LiesOn | SpanVerdict::Unsettled => {
                    Err(frontier())
                }
            }
        }
        // Both inside: the residual along a line is convex (cylinder,
        // sphere — the torus and every circle took the arm above), so
        // its maximum is at an endpoint — definitely no wall crossing.
        (Sign::Negative, Sign::Negative) => Ok(CurvedEvent::None),
        // Both outside: clear through a DIVISION-FREE lower bound on
        // the span minimum of the convex residual. Division-free is a
        // hard requirement, not a preference: the original
        // parabola-vertex formula divided by the transverse direction
        // norm, which is 0/0 on an axis-parallel edge — and the
        // IDEALIZED sweep examines exactly those at distance, so the
        // poison took the whole op with it.
        //
        // **The vertex is CLAMPED to the segment, and that is what the
        // bound buys.** Write `q = f″·Δt²` and `m = f(t₁) − f(t₀)`,
        // both metres. For a convex quadratic the interior minimum
        // exists only when the vertex lies inside the span, which is
        // exactly `|m| < q/2`; outside that the function is monotone
        // and its minimum IS an endpoint. So the dip below the
        // endpoint chord is
        //   dip = (q/2 − |m|)² / (2q)   when |m| < q/2,  else 0
        // and since `(q/2 − |m|) ≤ q/2` the quotient is at most
        // `(q/2 − |m|)/4`. Taking that as the charge keeps the whole
        // computation in `max` and `min`:
        //   dip ≤ max(0, q/2 − |m|) / 4
        // — no division, EXACTLY ZERO when the vertex is outside the
        // segment, and exact again at the centred vertex (`m = 0`
        // gives `q/8`, the true worst case).
        //
        // Between those two it is loose, and the looseness is worth
        // stating truthfully rather than flatteringly: the ratio of
        // charge to true dip is `q / (2(q/2 − |m|))`, which is 1 at
        // `m = 0`, 4 at `m = 3q/8`, and UNBOUNDED as `|m|` approaches
        // `q/2`. What stays bounded is the ABSOLUTE charge, which
        // vanishes linearly there — so the multiplicative claim fails
        // exactly where the quantity being multiplied is going to
        // zero, and the bound never charges more than `q/8`.
        //
        // The old `q/8` charged the centred-vertex dip to every edge
        // whatever its endpoint gap, which is what made a pocket wall
        // 2 mm clear of a corner round read as a pierce (#347's
        // measured `r ≥ 5` bound).
        //
        // Conservative direction is unchanged: a too-large charge only
        // sends more pairs to the typed frontier door, never accepts.
        (Sign::Positive, Sign::Positive) => {
            let geom::Curve3::Line { origin: _, dir } = *curve.carrier() else {
                return Err(frontier()); // unreachable: matched above
            };
            let (t0, t1) = curve.params();
            // f″ per kind (the residual's second derivative along the
            // ray, constant for these kinds).
            let f2 = match surface {
                geom::Surface::Cylinder { axis, radius, .. } => {
                    let d_ax = dir.dot(axis);
                    (dir.norm_squared() - d_ax.powi(2)) / radius
                }
                geom::Surface::Sphere { radius, .. } => dir.norm_squared() / radius,
                // Post-gate/pre-check unreachable kinds keep the
                // frontier door.
                _ => return Err(frontier()),
            };
            let span = t1 - t0;
            let r_u = geom_brep::implicit_residual(&surface, pu);
            let r_v = geom_brep::implicit_residual(&surface, pv);
            // q = f''·Δt², m = the endpoint gap; both metres.
            let q = f2 * span.powi(2);
            let m = (r_v - r_u).abs();
            let dip = (q * T::from_f64(0.5) - m).max(T::zero()) * T::from_f64(0.25);
            let min_bound = Margin::of(r_u.min(r_v) - dip);
            match decide("bool_line_cylinder_clearance", min_bound, band) {
                Ok(Sign::Positive) => Ok(CurvedEvent::None),
                // **The BELLY crossing**, and the reason the dip bound
                // is a clearance test and never a crossing test: a
                // convex residual positive at both endpoints can still
                // dip through zero between them, and the bound is a
                // charge against that dip, not a measurement of it. A
                // charge that does not clear says only "not definitely
                // clear" — so the exact roots decide, and they decide
                // BOTH ways: two roots inside the span are a pierce,
                // and roots outside it (or none at all) are an
                // exactness the bound could not reach. Only what the
                // roots cannot settle keeps the frontier door.
                Ok(Sign::Zero | Sign::Negative) => {
                    match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                        SpanVerdict::Pierce { t, p, at } => Ok(CurvedEvent::Pierce { t, p, at }),
                        // No interior root, or no root at all:
                        // definitely clear, and exactly so — the bound
                        // that sent us here could only ever have said
                        // "maybe".
                        SpanVerdict::NoInterior | SpanVerdict::Elsewhere | SpanVerdict::Miss => {
                            Ok(CurvedEvent::None)
                        }
                        // **`Constant` is NOT a clearance here.** It
                        // says the edge drifts off its distance from the
                        // axis by less than the band over its span, so
                        // the residual is constant along it, but not on
                        // which side of the wall: an edge running along
                        // the wall in band is a coincidence, and nothing
                        // upstream put this one definitely off it. So it
                        // keeps the door. (`solid_contain::cast_ray`
                        // skips the face on its own axis-parallel
                        // verdict, levered by the selection's reach,
                        // because its pre-pass has put `q` definitely off
                        // the wall.)
                        SpanVerdict::Constant | SpanVerdict::LiesOn => Err(frontier()),
                        // Covered, the edge lies in one closed side of
                        // the carrier, so a root set the lane cannot
                        // certify (a tangency) is a touch inside the
                        // span. The residual along a line is convex here
                        // (a wall or a sphere), so its zero set is one
                        // point or one interval and a fragment's ends
                        // see it: deferred for the fragments.
                        SpanVerdict::Unsettled if covered => Ok(CurvedEvent::Deferred),
                        SpanVerdict::Unsettled => Err(frontier()),
                    }
                }
                Err(diag) => {
                    let which = Coincide::EdgeOnCurvedFace;
                    Err(BooleanError::coincidence(which, read(which), diag))
                }
            }
        }
    }
}

/// The two enclosures of a conic ARC's residual against `surface` (a
/// circle's or an ellipse's, `geom_brep::Conic`), folded: the carrier's
/// harmonic bounds and the arc's sampled chord-dip range. Both enclose
/// the arc's range, each with its own rounding charged
/// (`geom_brep::conic_residual_extremes`,
/// `geom_brep::conic_arc_residual_range`), so the clearance margin is
/// the larger of the two one-sidedness margins. `None` when the carrier
/// enclosure has no form for the kind.
///
/// **What certifies here, by kind.** Against a plane, sphere or
/// cylinder both enclosures read; at a vertex whose harmonics' phases
/// align, the carrier bound IS the residual's extreme, so its rounding
/// charge is what stands between a graze and a certified clearance.
/// Against a torus the carrier enclosure is the whole turn's sampled
/// one, levered by a curvature bound that is loose in the direction that
/// refuses: on grazes within 40 bands it certified none at ε 1e-12 and
/// 1e-9 and a handful at 1e-6, each clear under the oracle
/// (`clearance_rows`). Against a cone there is no enclosure (`None`, the
/// frontier).
///
/// The line row's vertex CLAMP does not port here, and the reason is
/// the curve: along a line the residual is exactly quadratic, so "the
/// vertex is outside the span" is a statement about a parabola and is
/// decided by the endpoint gap alone. Along a conic it has up to four
/// critical parameters, so an endpoint gap says nothing about where
/// its minimum sits. Subdivision is what is available without solving
/// for them.
fn conic_clearance<T: Decide>(
    surface: &geom::Surface<T>,
    conic: &geom_brep::Conic<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Option<Result<Sign, geom_core::Indeterminate>> {
    let (lo, hi) = geom_brep::conic_residual_extremes(surface, conic)?;
    let carrier_margin = lo.max(-hi);
    let arc_margin = geom_brep::conic_arc_residual_range(surface, conic, t0, t1)
        .map_or(carrier_margin, |(arc_lo, arc_hi)| arc_lo.max(-arc_hi));
    Some(decide(
        "bool_conic_curved_clearance",
        Margin::of(carrier_margin.max(arc_margin)),
        band,
    ))
}

/// **The carrier-identity rung**: does one of the edge's parent faces
/// sit on `face`'s carrier by a verified one-carrier declaration?
///
/// `Rest` and a continuation are the classes that license it, and only
/// where the declaration door's carrier ladder CALLED the pair one carrier
/// ([`super::DeclaredPairs::verified_one_carrier`], recorded once at the
/// door rather than re-derived per event). An edge bounding a face on
/// that carrier lies on it. `Tangent` licenses nothing of the kind — a
/// tangent pair shares a locus, not a carrier — and an undeclared pair
/// is never read as coincident by value (CONTACT-DESIGN C2/C4).
fn on_declared_shared_carrier<T: Decide>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::DeclaredPairs<T>,
) -> bool {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .any(|pf| declared.verified_one_carrier(x_is, pf, x_is.other(), face))
}

/// What the declared arms know of an edge's interior against `face`.
enum Interior<T: geom_core::Real> {
    /// It crosses `face`'s boundary there: a pierce landing on that
    /// boundary, split and recorded by the caller like any other.
    Crossing(CurvedEvent<T>),
    /// Certified: the interior meets `face`'s boundary nowhere, so the
    /// endpoints' placements are the pair's whole incidence.
    Clear,
    /// Not asked: the edge has no certificate that it lies ON the
    /// carrier, so the boundary question has no subject.
    Unseen,
}

impl<T: geom_core::Real> Interior<T> {
    /// Whether [`Placement::declared`] may read an all-`Elsewhere` pair
    /// as lying outside the face.
    fn clear(&self) -> bool {
        matches!(self, Self::Clear)
    }
}

/// The declared arms' interior question, asked only of an edge the
/// carrier-identity rung puts ON `face`'s carrier (`on_carrier`): where
/// it crosses `face`'s boundary strictly inside its span
/// ([`super::carrier_cross`]). A carrier pair with no closed form keeps
/// the frontier door.
fn interior<T: Decide>(
    on_carrier: bool,
    y: &Body<T>,
    x_is: Operand,
    face: FaceKey,
    curve: &geom_brep::EdgeCurve<T>,
    band: Band,
    frontier: impl Fn() -> BooleanError,
) -> Result<Interior<T>, BooleanError> {
    use super::carrier_cross::{BoundaryCrossing, boundary_crossing};
    if !on_carrier {
        return Ok(Interior::Unseen);
    }
    match boundary_crossing(y, x_is.other(), face, curve.carrier(), curve.params(), band)? {
        BoundaryCrossing::At { t, p, at } => {
            Ok(Interior::Crossing(CurvedEvent::Pierce { t, p, at }))
        }
        BoundaryCrossing::Clear => Ok(Interior::Clear),
        BoundaryCrossing::Unread => Err(frontier()),
    }
}

/// An arc of `x` lying on `face`'s carrier, as [`lying_on`] reads it.
struct ArcOnCarrier<'a, T: geom_core::Real> {
    x: &'a Body<T>,
    x_is: Operand,
    edge_key: EdgeKey,
    /// The arc's two ends, `he_plus`'s start first.
    ends: [(VertexKey, Point3<T>); 2],
    face: FaceKey,
    band: Band,
    tol: Tol,
}

/// **An arc lying on `face`'s carrier, its parents distinct from it:**
/// the endpoint records, once the arc's interior is certified to cross
/// `face`'s boundary nowhere. Two certificates do that, and anything
/// else keeps the door (`None`):
///
/// - **off the boundary but at its ends**: the face's boundary meets the
///   arc's own circle nowhere but at the vertices of `y` the arc's ends
///   were paired with ([`boundary_meets_circle_only_at`]), so the arc's
///   interior meets the boundary nowhere and lies wholly inside the face
///   or wholly outside it. With no end on the
///   boundary the ends must agree: both placed outside is no event, both
///   recorded is the endpoint posture; with an end on it, its record is.
/// - **along edges of the partner**: both ends were paired with
///   vertices of `y`, and `y` has a chain of circle arcs between them,
///   each leaving along this arc's tangent where it starts
///   ([`super::arcs::arcs_along`]) and arriving at a point decided on
///   this arc's circle. A circle through two points with a given tangent
///   at one is unique (this arc is a circle: an ellipse answers `LiesOn`
///   too, and keeps the door below), so the chain IS this arc, and edges of a valid body
///   cross no face's interior: the end records, with the chain's inner
///   vertices that the other direction's sweep records on this arc, are
///   every incidence it has.
///
/// The ends are placed, and recorded, before either certificate runs:
/// certificate (a) reads the vertices of `y` the placements pair them
/// with, minting one where an end lands on an edge. That is sound
/// because every answer but `Recorded` either follows two `Elsewhere`
/// placements, which record nothing, or is `None`, which the caller
/// turns into the frontier that ends the op, so no record or split made
/// here outlives a certificate that did not hold.
fn lying_on<T: Decide + crate::props::AtRestPolicy>(
    arc: &ArcOnCarrier<'_, T>,
    y: &mut Body<T>,
    contacts: &mut ContactAcc,
) -> Result<Option<CurvedEvent<T>>, BooleanError> {
    let ArcOnCarrier {
        x,
        x_is,
        edge_key,
        ends,
        face,
        band,
        tol,
    } = *arc;
    let lost = |what| BooleanError::ClassificationInvariant { what };
    let e = x
        .get_edge(edge_key)
        .ok_or_else(|| lost("an arc on a carrier: the edge is lost"))?;
    let curve = x
        .get_curve_geom(e.curve)
        .and_then(CurveGeom::certified)
        .ok_or_else(|| lost("an arc on a carrier: the edge's curve is lost"))?;
    let geom::Curve3::Circle {
        center,
        axis,
        radius,
        ..
    } = *curve.carrier()
    else {
        return Ok(None);
    };
    let mut placed = [(Placement::Undecided, None); 2];
    for (slot, (w, pw)) in placed.iter_mut().zip(ends) {
        *slot = vertex_on_curved_face_at(x_is, y, w, pw, face, contacts, band, tol)?;
    }
    if placed.iter().any(|(p, _)| *p == Placement::Undecided) {
        return Ok(None);
    }
    let at_ends: Vec<VertexKey> = placed.iter().filter_map(|(_, w)| *w).collect();
    if boundary_meets_circle_only_at(y, face, (center, axis, radius), &at_ends, band)? {
        let all = |p: Placement| placed.iter().all(|(q, _)| *q == p);
        if all(Placement::Elsewhere) {
            return Ok(Some(CurvedEvent::None));
        }
        // With no end paired with a vertex of `y`, one end in and one
        // out would need the arc to cross a boundary the certificate has
        // just kept off its circle: only two certified answers
        // contradicting reach it, and that keeps the door. An end paired
        // with a vertex is outside that argument (the face-free vertex
        // search can pair one off this face's boundary), and the record
        // it made is its answer.
        let mixed = at_ends.is_empty() && !all(Placement::Recorded);
        return Ok((!mixed).then_some(CurvedEvent::Recorded));
    }
    let [(_, Some(wu)), (_, Some(wv))] = placed else {
        return Ok(None);
    };
    let (dir, _) = curve.walk_tangents(
        x.get_half_edge(e.he_plus)
            .ok_or_else(|| lost("an arc on a carrier: its half is lost"))?
            .start
            == ends[0].0,
    );
    Ok(
        arc_chain_reaches(y, wu, wv, dir, (center, axis, radius), band)?
            .then_some(CurvedEvent::Recorded),
    )
}

/// Whether `y` has a chain of circle arcs from `from` to `to`, the
/// first leaving `from` along `dir` and each next one along the last
/// one's arrival tangent ([`super::arcs::arcs_along`]), every vertex it
/// passes on the way decided on the circle (`center`, unit `axis`,
/// `radius`). `to` itself is not decided: the caller paired it with an
/// end of an arc on that circle.
fn arc_chain_reaches<T: Decide>(
    y: &Body<T>,
    from: VertexKey,
    to: VertexKey,
    mut dir: geom_core::Vec3<T>,
    (center, axis, radius): (Point3<T>, geom_core::Vec3<T>, T),
    band: Band,
) -> Result<bool, BooleanError> {
    let escalated =
        |diag| BooleanError::coincidence(Coincide::EdgeOnCurvedFace, DeclarationRead::Moot, diag);
    // A chain visits each edge of `y` at most once.
    let mut at = from;
    for _ in 0..y.edges().count() {
        let steps = super::arcs::arcs_along(y, at, dir, band)?.map_err(escalated)?;
        let [step] = steps[..] else {
            return Ok(false);
        };
        if step.to == to {
            return Ok(true);
        }
        let p = y
            .get_vertex(step.to)
            .and_then(|vd| y.get_point(vd.point))
            .copied()
            .ok_or(BooleanError::ClassificationInvariant {
                what: "an arc on a carrier: a chain vertex has no point",
            })?;
        let miss = crate::splitting::containment::circle_miss(p, center, axis, radius);
        match decide("bool_arc_chain_on_circle", Margin::of(miss), band).map_err(escalated)? {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => return Ok(false),
        }
        at = step.to;
        dir = step.arrival;
    }
    Ok(false)
}

/// Where a boundary vertex sits against the arc's plane, as
/// [`boundary_meets_circle_only_at`] reads it.
#[derive(Clone, Copy, PartialEq)]
enum PlaneSide {
    /// Decided strictly on this side.
    Off(Sign),
    /// One of the vertices the arc's ends were paired with.
    At,
    /// Decided in the plane, and decided off the circle.
    InPlaneOffCircle,
}

/// Whether `face`'s boundary meets the circle (`center`, unit `axis`,
/// `radius`) nowhere but at the vertices `at`. The circle lies in the
/// plane through `center` normal to `axis`, so the boundary is read
/// against that plane, and each point where it meets the plane must be
/// decided off the circle or be one of `at`:
///
/// - a vertex: decided strictly off the plane, or decided in it and off
///   the circle. One whose side escalates is placed nowhere, and fails;
/// - a line: no crossing between two ends on one side; its one crossing
///   between the two sides decided off the circle; an end in the plane
///   with the other strictly off. A line lying in the plane is a chord
///   only between two of `at`, and is certified when its midpoint is
///   decided off the circle;
/// - a conic: its crossings strictly inside its span (the splitting
///   lane's certified roots,
///   [`crate::splitting::plane_crossing_lane`]) each decided off
///   the circle, or a plane of its own decided parallel and off.
///
/// Everything else answers `false`: an undecided point, a conic lying
/// in the plane, and any boundary edge the certificate cannot place (a
/// NURBS or spiric carrier, or no certified curve).
fn boundary_meets_circle_only_at<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
    (center, axis, radius): (Point3<T>, geom_core::Vec3<T>, T),
    at: &[VertexKey],
    band: Band,
) -> Result<bool, BooleanError> {
    let lost = || BooleanError::ClassificationInvariant {
        what: "an arc on a carrier: the face's boundary is not walkable",
    };
    let f = y.get_face(face).ok_or_else(lost)?;
    let height = |p: Point3<T>| (p - center).dot(axis);
    let off_circle = |p: Point3<T>| {
        matches!(
            decide(
                "bool_arc_boundary_off_circle",
                Margin::of(crate::splitting::containment::circle_miss(
                    p, center, axis, radius
                )),
                band
            ),
            Ok(Sign::Positive)
        )
    };
    let point = |v: VertexKey| {
        y.get_vertex(v)
            .and_then(|vd| y.get_point(vd.point))
            .copied()
            .ok_or_else(lost)
    };
    let place = |v: VertexKey| -> Result<Option<PlaneSide>, BooleanError> {
        if at.contains(&v) {
            return Ok(Some(PlaneSide::At));
        }
        let p = point(v)?;
        Ok(
            match decide("bool_arc_plane_side", Margin::of(height(p)), band) {
                Ok(s @ (Sign::Positive | Sign::Negative)) => Some(PlaneSide::Off(s)),
                Ok(Sign::Zero) => off_circle(p).then_some(PlaneSide::InPlaneOffCircle),
                Err(_) => None,
            },
        )
    };
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        match y.get_loop(lk).ok_or_else(lost)?.boundary {
            crate::entity::LoopBoundary::Empty { vertex } => {
                if place(vertex)?.is_none() {
                    return Ok(false);
                }
            }
            crate::entity::LoopBoundary::Cycle { first } => {
                for he in y.loop_cycle(first).ok_or_else(lost)? {
                    let h = y.get_half_edge(he).ok_or_else(lost)?;
                    let e = y.get_edge(h.edge).ok_or_else(lost)?;
                    let (Some(a), Some(b)) = (
                        y.get_half_edge(e.he_plus).map(|h| h.start),
                        y.get_half_edge(e.he_minus).map(|h| h.start),
                    ) else {
                        return Err(lost());
                    };
                    let (Some(sa), Some(sb)) = (place(a)?, place(b)?) else {
                        return Ok(false);
                    };
                    let Some(c) = y.get_curve_geom(e.curve).and_then(CurveGeom::certified) else {
                        return Ok(false);
                    };
                    let (t0, t1) = c.params();
                    let clear = match crate::splitting::plane_crossing_lane(
                        c.carrier(),
                        t0,
                        t1,
                        center,
                        axis,
                        band,
                    ) {
                        PlaneCrossingLane::Line => {
                            let (pa, pb) = (point(a)?, point(b)?);
                            match (sa, sb) {
                                (PlaneSide::Off(s), PlaneSide::Off(t)) if s != t => {
                                    let (ha, hb) = (height(pa), height(pb));
                                    off_circle(pa + (pb - pa) * (ha / (ha - hb)))
                                }
                                (PlaneSide::Off(_), _) | (_, PlaneSide::Off(_)) => true,
                                (PlaneSide::At, PlaneSide::At) => {
                                    off_circle(pa.lerp(pb, T::from_f64(0.5)))
                                }
                                _ => false,
                            }
                        }
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Miss) => true,
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(roots))) => {
                            roots.iter().all(|&t| off_circle(c.carrier().eval(t)))
                        }
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => matches!(
                            decide("bool_arc_plane_side", Margin::of(offset), band),
                            Ok(Sign::Positive | Sign::Negative)
                        ),
                        PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(_)))
                        | PlaneCrossingLane::Unlaned => false,
                    };
                    if !clear {
                        return Ok(false);
                    }
                }
            }
        }
    }
    Ok(true)
}

/// Whether the carrier ladder decides EVERY surface of a face the edge
/// bounds definitely distinct from `face`'s carrier. Undeclared: a
/// same-source pair, an undeclared coincidence, an escalation or a kind
/// outside the ladder's inventory is not a decision, and answers false.
fn parents_distinct_from<T: Decide>(
    x: &Body<T>,
    edge: &crate::entity::Edge,
    y: &Body<T>,
    face: FaceKey,
    band: Band,
) -> bool {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .all(|pf| {
        pf.is_some_and(|pf| {
            matches!(
                super::rest::carrier_pair_relation(x, pf, y, face, false, band),
                Ok(Ok(super::carrier_eq::CarrierRelation::Distinct))
            )
        })
    })
}

/// What the certified carrier × wall roots say about ONE edge span.
#[derive(Debug, Clone, Copy)]
enum SpanVerdict<T: geom_core::Real> {
    /// A definite crossing strictly inside the span, which the trim
    /// placed at `at` in this face.
    Pierce {
        t: T,
        p: Point3<T>,
        at: FaceContainment,
    },
    /// A certified root set (two for a line, or a circle on a sphere;
    /// two or four for a circle on a wall or a torus, or a line on a
    /// torus), none of them STRICTLY INSIDE the span on this face: each
    /// lies outside the span, sits at one of its ends, or lands on the
    /// carrier outside the face's trim. Distinct certified roots also
    /// certify that the edge does not LIE on the carrier (a line: not a
    /// ruling of a wall, and no line lies on a sphere or a torus; a
    /// circle on a sphere or a wall answers [`Self::LiesOn`], and on a torus a
    /// certified count needs a pole definitely off the torus, which a
    /// circle lying on it has nowhere), which is what separates a chord
    /// from an on-carrier edge. What the
    /// absence of an interior crossing licenses depends on the
    /// endpoints, so the caller decides — an endpoint incidence is
    /// still an event, it is just not an interior one.
    NoInterior,
    /// The [`Self::NoInterior`] case with its crossing ACCOUNTED FOR: at
    /// least one root lies strictly inside the span, every such root
    /// was placed on the carrier and definitely OUTSIDE this face's
    /// trim, and no root sits at either end. It is `NoInterior` in every
    /// respect a caller that accepts `NoInterior` reads; what it adds is
    /// for the straddling span, whose endpoint signs PROMISE a crossing
    /// — here the promise is kept, on the carrier, off this face. A
    /// `NoInterior` with no such root cannot keep it, and that
    /// contradiction still keeps the door.
    Elsewhere,
    /// The line is parallel to the axis, so its residual is CONSTANT
    /// along the span. Nothing about the span's interior differs from
    /// its endpoints — which makes it a clearance answer for a caller
    /// whose endpoints are definitely off the wall and a cosurface
    /// question for one whose endpoints are on it.
    Constant,
    /// The conic LIES on the surface: its residual is a zero constant,
    /// decided exactly on by its root door
    /// ([`super::circle_roots::CircleRoots::OnSurface`]; in band it
    /// escalates there). Unlike [`Self::Constant`] this is a verdict
    /// on the side as well as the shape.
    LiesOn,
    /// The line definitely misses the wall entirely.
    Miss,
    /// The roots did not settle the span and the caller keeps its own
    /// typed frontier door.
    Unsettled,
}

/// The curved-wall crossing route: solve the certified roots — a
/// line's quadratic on a cylinder wall or a sphere, its quartic on a
/// torus, a circle's closed form on a sphere and its half-angle quartic
/// on a torus or a cylinder wall, an ellipse's half-angle quartic on a
/// sphere or a cylinder wall — keep the roots the EDGE's
/// span carries strictly inside, and place the landing point in the
/// face's trim.
///
/// **Roots at the span's ends are deliberately NOT interior.** A root
/// the band cannot separate from an endpoint is that endpoint's own
/// incidence, and the endpoint rows own those (they mint a v-f or v-v
/// record rather than a split, which is what a split at `t₀` could not
/// do anyway — `split_edge`'s interiority trilean refuses it). Folding
/// the two together here would have made an endpoint touch look like a
/// crossing to every caller.
///
/// **Every root is examined, and the FIRST interior one wins.** A
/// segment through a wall meets it twice (a torus up to four times);
/// the sweep splits at one root and re-queues both fragments against
/// the SAME face, so the rest are found on later passes — the shape the
/// conic × plane lane already uses, and the reason this function does
/// not return a set.
fn wall_crossing<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> Result<SpanVerdict<T>, BooleanError> {
    // A root's distance from an end of the span is metered as a length:
    // the parameter gap times the carrier's speed AT that end, so that
    // `Zero` means the root sits at the end's own point. A `Line`'s
    // parameter runs `|dir|` metres per unit, a `Circle`'s `radius`, an
    // `Ellipse`'s `|C′(t)|`, which varies over `[b, a]` (`geom_brep::Conic`).
    // The wall and sphere quadratics take any non-zero `dir`; the torus
    // quartic ([`super::solid_contain::line_torus_roots`]) assumes it UNIT,
    // the `Line` carrier's convention, which nothing checks.
    let found = match *carrier {
        geom::Curve3::Line { origin, dir } => {
            match line_wall_roots_of(origin, dir, (t1 - t0).abs(), surface, band)? {
                Ok(found) => found,
                Err(verdict) => return Ok(verdict),
            }
        }
        // The circle root doors ([`super::circle_roots`]), one per kind,
        // one answer shape. A circle against any other kind has no root
        // lane here.
        geom::Curve3::Circle { .. } => match surface {
            geom::Surface::Sphere { .. } => {
                super::circle_sphere::circle_sphere_roots(carrier, t0, t1, surface, band)?
            }
            geom::Surface::Cylinder { .. } => {
                super::circle_cylinder::circle_cylinder_roots(carrier, t0, t1, surface, band)?
            }
            geom::Surface::Torus { .. } => {
                super::circle_torus::circle_torus_roots(carrier, t0, t1, surface, band)?
            }
            _ => return Ok(SpanVerdict::Unsettled),
        },
        // The ellipse door ([`super::ellipse_roots`]), on the kinds whose
        // residual along it is a degree-2 trigonometric polynomial. Against
        // a torus it is degree four (an octic in the half-angle), which no
        // lane here solves, so that cell is `Unsettled`.
        geom::Curve3::Ellipse { .. } => match surface {
            geom::Surface::Sphere { .. } | geom::Surface::Cylinder { .. } => {
                super::ellipse_roots::ellipse_roots(carrier, t0, t1, surface, band)?
            }
            _ => return Ok(SpanVerdict::Unsettled),
        },
        _ => return Ok(SpanVerdict::Unsettled),
    };
    // The carrier's speed at a parameter: only the kinds `found` was
    // read for reach here, and an ellipse whose frame cannot be read is
    // a kernel bug, refused — never a zero speed that would read every
    // root as at the span's end.
    let speed_at = |t: T| match *carrier {
        geom::Curve3::Line { dir, .. } => Ok(dir.norm()),
        geom::Curve3::Circle { radius, .. } => Ok(radius),
        _ => geom_brep::Conic::of(carrier).map(|c| c.speed_at(t)).ok_or(
            BooleanError::ClassificationInvariant {
                what: "a wall root's carrier is neither a line nor a conic",
            },
        ),
    };
    let (count, roots) = match found {
        CircleRoots::Certified { count, thetas } => (count, thetas),
        // A conic ON the surface: its residual is a zero constant — the
        // circle or ellipse that lies on a sphere, wall or torus. (A
        // ruling on a wall is the line door's `Constant`, above.)
        CircleRoots::OnSurface => return Ok(SpanVerdict::LiesOn),
        CircleRoots::Uncertain => return Ok(SpanVerdict::Unsettled),
        CircleRoots::Miss => return Ok(SpanVerdict::Miss),
        CircleRoots::CountDisagrees => {
            return Err(BooleanError::ClassificationInvariant {
                what: "the constructed roots of a quartic disagree in number with its \
                       certified count",
            });
        }
    };
    let ts = &roots[..count];
    // Whether some root sits at an end of the span, and whether some
    // root strictly inside it was placed outside this face's trim: the
    // two facts that tell [`SpanVerdict::Elsewhere`] from
    // [`SpanVerdict::NoInterior`].
    let mut at_end = false;
    let mut crossed_elsewhere = false;
    for &t in ts {
        let mut interior = true;
        for (gap, end) in [(t - t0, t0), (t1 - t, t1)] {
            match decide(
                "bool_wall_root_in_span",
                Margin::of(gap * speed_at(end)?),
                band,
            ) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Zero) => {
                    interior = false;
                    at_end = true;
                }
                Ok(Sign::Negative) => interior = false,
                Err(diag) => {
                    return Err(BooleanError::Escalated {
                        decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
                        diag,
                    });
                }
            }
        }
        if !interior {
            continue;
        }
        let p = carrier.eval(t);
        // The face's own trim decides whether a crossing of the CARRIER
        // is a crossing of this FACE. `None` is the chart door's honest
        // remainder (a ringed face, a boundary outside its outline
        // classes, a full-period window outside the band class) and
        // keeps the caller's
        // frontier rather than reading as "outside".
        //
        // **A landing point definitely OFF the carrier is not "outside
        // the trim".** The root was certified ON the surface, so the
        // point containment decides is off it CONTRADICTS that
        // certificate — a root the band cannot stand behind at this
        // pose. Reading it as a sibling face's crossing would step over
        // a real crossing and report the span clear; it keeps the door.
        match super::contain::curved_face_placement(y, face, p, band) {
            Ok(CurvedPlacement::OffCarrier | CurvedPlacement::Trim(None)) => {
                return Ok(SpanVerdict::Unsettled);
            }
            // On the carrier and definitely outside THIS face's trim:
            // the carrier is crossed, but not here. The other roots may
            // still land in the face, so the loop continues.
            Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))) => crossed_elsewhere = true,
            Ok(CurvedPlacement::Trim(Some(at))) => return Ok(SpanVerdict::Pierce { t, p, at }),
            Err(super::contain::ContainError::Escalated(diag)) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::Containment,
                    diag,
                });
            }
            // Unwalkable topology under a query the crossing layer just
            // routed: the operand is corrupt, and saying "the roots did
            // not settle it" would report a geometry frontier for a
            // structural break. It keeps the caller's door because that
            // is the conservative direction, and the distinction is
            // recorded here rather than left to the payload.
            Err(_) => return Ok(SpanVerdict::Unsettled),
        }
    }
    Ok(no_pierce_verdict(crossed_elsewhere, at_end))
}

/// The verdict of a root set with no pierce in this face: `Elsewhere`
/// only when some root strictly inside the span was placed outside the
/// trim AND no root sits at an end — the one case that accounts for a
/// straddle's crossing. A root set with no interior root (or with one at
/// an end) is `NoInterior`, which the straddle arm reads as a
/// contradiction and keeps the door on.
fn no_pierce_verdict<T: geom_core::Real>(crossed_elsewhere: bool, at_end: bool) -> SpanVerdict<T> {
    if crossed_elsewhere && !at_end {
        SpanVerdict::Elsewhere
    } else {
        SpanVerdict::NoInterior
    }
}

/// The certified LINE × wall roots, per kind, in the root doors' answer
/// shape — or, for the one answer that shape has no word for, the
/// verdict itself: an axis-parallel line's residual is constant along it
/// without being zero ([`SpanVerdict::Constant`]). `span` is the run of
/// the line's parameter the edge covers, the lever the wall's
/// axis-parallel rung is metered over.
fn line_wall_roots_of<T: Decide>(
    origin: Point3<T>,
    dir: geom_core::Vec3<T>,
    span: T,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Result<CircleRoots<T>, SpanVerdict<T>>, BooleanError> {
    use super::solid_contain::WallRoots;
    let two = |ts: [T; 2]| CircleRoots::Certified {
        count: 2,
        thetas: [ts[0], ts[1], T::zero(), T::zero()],
    };
    // The certified roots, per kind. Every lane answers the same three
    // ways — a certified root set, a definite miss, or no certain
    // count — and the cylinder adds a fourth, the axis-parallel line
    // whose residual is constant. A line never lies on a torus or a
    // sphere, so neither has such a case. A tangency is not a crossing
    // this lane can act on — the material verdicts behind a pierce are
    // first-order, and along a tangency every first-order datum ties —
    // so it keeps the door.
    Ok(Ok(match *surface {
        geom::Surface::Cylinder {
            origin: c_origin,
            axis,
            radius,
            ..
        } => match super::solid_contain::line_wall_roots(
            origin, dir, c_origin, axis, radius, span, band,
        )
        .map_err(|fault| BooleanError::Escalated {
            decision: BooleanDecision::WallRoots(fault.rung),
            diag: fault.diag,
        })? {
            WallRoots::Two(ts) => two(ts),
            WallRoots::Tangent => CircleRoots::Uncertain,
            WallRoots::AxisParallel => return Ok(Err(SpanVerdict::Constant)),
            WallRoots::Miss => CircleRoots::Miss,
        },
        // The quartic: the ray lane's own certified root door, over the
        // edge's span instead of a ray's forward half. It answers only
        // on a CERTIFIED count, so a graze, a repeated root, or a
        // classifying sign in the band is `Uncertain` and keeps the
        // door exactly as the cylinder's tangency does.
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => super::solid_contain::line_torus_roots(
            origin,
            dir,
            center,
            axis,
            major_radius,
            minor_radius,
            band,
        )
        .map_err(|diag| BooleanError::Escalated {
            decision: BooleanDecision::TorusRoots,
            diag,
        })?
        .into(),
        // The quadratic, the ray lane's own. No line lies on a sphere,
        // so it has no constant case either.
        geom::Surface::Sphere { center, radius, .. } => {
            match super::solid_contain::line_sphere_roots(origin, dir, center, radius, band)
                .map_err(|diag| BooleanError::Escalated {
                    decision: BooleanDecision::SphereRoots,
                    diag,
                })? {
                WallRoots::Two(ts) => two(ts),
                WallRoots::Tangent => CircleRoots::Uncertain,
                WallRoots::AxisParallel | WallRoots::Miss => CircleRoots::Miss,
            }
        }
        _ => CircleRoots::Uncertain,
    }))
}

/// What one edge×curved-face pair asks of the sweep.
#[derive(Debug, Clone, Copy)]
pub(super) enum CurvedEvent<T: geom_core::Real> {
    /// No event: the pair is definitely clear, or the crossing lies
    /// outside this face's trim.
    None,
    /// The arm recorded the event itself (the one-sided cover rung,
    /// whose endpoint treatment mints its own contacts). Reported so
    /// the differential suite's accepted-pair channel still sees it.
    Recorded,
    /// A covered LINE whose touch with a cylinder or sphere, if it has
    /// one, lies strictly inside the edge: both ends are definitely off
    /// the carrier, the cover puts the edge in one closed side of it,
    /// and the roots could not settle it. The cover records endpoints
    /// only, so the sweep defers the pair ([`DeferredTouch`]) until both
    /// directions have run and then reads it again on the edge's
    /// fragments ([`settle_deferred`]).
    Deferred,
    /// A definite wall crossing at the carrier parameter `t`, whose
    /// point `p` the trim placed at `at`. The sweep splits the edge and
    /// records the contact exactly as it does for a conic × plane root.
    Pierce {
        t: T,
        p: Point3<T>,
        at: FaceContainment,
    },
}

/// Where one on-carrier endpoint's incidence lives.
///
/// The distinction between the last two is the whole point of this
/// type: `Elsewhere` is a CERTIFIED verdict — the chart trim proved
/// the point is not in this face — whereas `Undecided` is the
/// containment door's honest remainder. A caller that reads them as
/// one thing refuses at a frontier for a pair that has no incidence to
/// begin with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Placement {
    /// A contact record was written for this endpoint.
    Recorded,
    /// The point is definitely not on this face and coincides with no
    /// vertex of `y`: its incidence, if it has one, belongs to another
    /// face on the same carrier, which the sweep visits separately.
    Elsewhere,
    /// No verdict — the containment door's remainder.
    Undecided,
}

impl Placement {
    /// **The declared rungs' one rule**, in one place because all of
    /// them apply it: given each of an edge's endpoints as
    /// `Some(placement)` when the residual put it ON the carrier and
    /// `None` when it is honestly not this arm's business (definitely
    /// clear, or the arm has only one on-carrier end), does this pair
    /// record an event, have none here, or keep the typed frontier
    /// (`None`)?
    ///
    /// - **any `Undecided`** keeps the door: an endpoint the containment
    ///   door could not place leaves the pair unknown. Only the truth
    ///   table below holds this end to end: `Undecided` needs an
    ///   on-carrier end on a face whose trim the chart door declines (a
    ///   ringed face) while every boundary edge is a line or a circle
    ///   (anything else answers `Unread` first), and the one such bore
    ///   tried — a full-turn collar less a partial-revolve wedge —
    ///   refuses before any mate: `Join(SectionArcWindow{BothContained})`
    ///   with the wedge inside the collar's height, `JoinDesync` where it
    ///   crosses a cap
    ///   (`work/tang/a-wedge-across-a-full-turn-collar-desyncs-its-chord-roles.md`);
    /// - **any `Recorded`** records;
    /// - **every on-carrier end `Elsewhere`** has placed nothing on this
    ///   face. That is no event only when `interior_clear` — the arm
    ///   certified the span's interior meets this face's boundary
    ///   nowhere ([`interior`]), so a span with both ends
    ///   outside the face lies wholly outside it. Without that
    ///   certificate it keeps the door: an overlap lying wholly inside
    ///   this face's window, with both ends beyond it, must not turn
    ///   into a silent no-event.
    fn declared<T: geom_core::Real>(
        ends: [Option<Self>; 2],
        interior_clear: bool,
    ) -> Option<CurvedEvent<T>> {
        let on = ends.iter().flatten();
        if on.clone().any(|p| *p == Self::Undecided) {
            None
        } else if on.clone().any(|p| *p == Self::Recorded) {
            Some(CurvedEvent::Recorded)
        } else if interior_clear {
            Some(CurvedEvent::None)
        } else {
            None
        }
    }

    /// **The undeclared `NoInterior` arms' one rule** — the mixed-sign
    /// span (one end ON the carrier) and the `(Zero, Zero)` chord. Given
    /// each ON end's placement (`None` for an end that is not on the
    /// carrier), the pair:
    ///
    /// - **records** if any end was `Recorded` (the long-standing OR
    ///   rule: a sibling's answer does not veto a real incidence);
    /// - is **no event HERE** if every ON end is `Elsewhere`: the span
    ///   has no incidence strictly inside it on this face (`NoInterior`),
    ///   and each ON end is certified outside this face's trim;
    /// - otherwise keeps the typed door (`None`): an `Undecided` end is
    ///   not evidence of absence.
    ///
    /// An all-`Elsewhere` pair is eventless here, as it is under
    /// [`Self::declared`] with a clear interior: the distinct certified
    /// roots behind `NoInterior` exclude an on-carrier edge overlapping
    /// the window.
    fn undeclared_no_interior<T: geom_core::Real>(
        ends: [Option<Self>; 2],
    ) -> Option<CurvedEvent<T>> {
        let on = ends.iter().flatten();
        if on.clone().any(|p| *p == Self::Recorded) {
            Some(CurvedEvent::Recorded)
        } else if on.clone().all(|p| *p == Self::Elsewhere) {
            Some(CurvedEvent::None)
        } else {
            None
        }
    }
}

/// The declared-cosurface rung's endpoint treatment: classify an
/// on-carrier vertex of `x` against the CURVED face and record the
/// planar posture's contact kinds — `OnVertex` ⇒ v-v, `OnEdge` ⇒
/// split `y`'s boundary edge at the (bitwise-shared) point and pair
/// the minted vertex, `In` ⇒ the v-f record, exactly as
/// [`vertex_on_face`] does on a plane.
///
/// **`Out` and no-verdict are DIFFERENT answers, and this door reports
/// them apart.** `curved_face_containment` answers `Out` from a
/// certified comparison — the point is off the carrier, or the
/// iso-bounded wall's exact chart rectangle excludes it — so `Out`
/// says the endpoint's incidence is not this pair's. It says nothing
/// about whether the endpoint has an incidence at all: on a carrier
/// shared by several faces the endpoint is a seam site, and the face
/// that holds it records it when the sweep reaches that pair. A
/// no-verdict is the opposite: the door could not express this face's
/// trim, so nothing at all is known and the caller's typed frontier is
/// the only honest answer.
#[allow(clippy::too_many_arguments)]
pub(super) fn vertex_on_curved_face<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<Placement, BooleanError> {
    vertex_on_curved_face_at(x_is, y, vx, px, face, contacts, band, tol).map(|(p, _)| p)
}

/// [`vertex_on_curved_face`], also naming the vertex of `y` a recorded
/// v-v contact paired `vx` with (`None` for a v-f record, or no record).
#[allow(clippy::too_many_arguments)]
fn vertex_on_curved_face_at<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<(Placement, Option<VertexKey>), BooleanError> {
    let placement = super::contain::curved_face_placement(y, face, px, band)
        .map_err(|e| esc(e, x_is.other()))?;
    let verdict = match placement {
        CurvedPlacement::Trim(v) => v,
        CurvedPlacement::OffCarrier => None,
    };
    match verdict {
        Some(FaceContainment::OnVertex(vy)) => {
            push_vv(contacts, x_is, vx, vy);
            return Ok((Placement::Recorded, Some(vy)));
        }
        Some(FaceContainment::OnEdge(ey)) => {
            let wy = split_other_at_point(y, x_is.other(), ey, px, band, tol)?;
            push_vv(contacts, x_is, vx, wy);
            return Ok((Placement::Recorded, Some(wy)));
        }
        // Strictly inside the curved face's chart trim: the same
        // v-f record the planar sweep writes ([`vertex_on_face`]),
        // now that the trim can say so.
        Some(FaceContainment::In) => {
            contacts.vf(x_is, VfContact { vertex: vx, face });
            return Ok((Placement::Recorded, None));
        }
        // Definitely outside this face's trim, or no verdict at all:
        // fall through to the face-free question below.
        Some(FaceContainment::Out) | None => {}
    }
    // Not on THIS face's boundary. One face-free question is still
    // decidable by the same row: coincidence with a vertex of `y`
    // anywhere (arena order, D9). An on-carrier edge is a candidate
    // against EVERY face sharing the carrier, and against the faces
    // whose trim does not hold the endpoint the honest answer is "the
    // event belongs elsewhere": a valid body's vertices lie on face
    // boundaries, never interior to a face, so a vertex hit certifies
    // the endpoint is a boundary site — and the v-v record is
    // face-free, so it is the SAME record the holding face's pair
    // produces (the accumulator dedups).
    //
    // **A miss here is not automatically the frontier, and the reason
    // is the pierce ring.** The "vertices lie on face boundaries"
    // premise above holds for an operand's OWN vertices, but a wall
    // pierce mints a vertex INTERIOR to the pierced face: after such a
    // split the pierced edge's fragment endpoint sits strictly inside
    // one face of the carrier and strictly outside every sibling
    // face, and no vertex of `y` is there to be found. So the two
    // answers are reported apart — a definite `Out` says "this face
    // has no incidence", a `None` says "no verdict at all" — and the
    // caller decides what each licenses.
    for (vy, vertex) in y.vertices() {
        let Some(py) = y.get_point(vertex.point).copied() else {
            continue;
        };
        if super::one_vertex(px, py, band).map_err(|diag| BooleanError::Escalated {
            decision: BooleanDecision::VertexOnVertex,
            diag,
        })? {
            push_vv(contacts, x_is, vx, vy);
            return Ok((Placement::Recorded, Some(vy)));
        }
    }
    // Only an ON-carrier `Out` is a certified absence. Every caller
    // reaches this door with the endpoint's residual decided `Zero`, so
    // an off-carrier answer contradicts that decision and is not
    // evidence the incidence lives elsewhere: it keeps the door.
    Ok((
        match placement {
            CurvedPlacement::Trim(Some(FaceContainment::Out)) => Placement::Elsewhere,
            _ => Placement::Undecided,
        },
        None,
    ))
}

pub(super) fn esc(e: ContainError, operand: Operand) -> BooleanError {
    match e {
        ContainError::Escalated(diag) => BooleanError::Escalated {
            decision: BooleanDecision::Containment,
            diag,
        },
        ContainError::RayExhausted => BooleanError::ClassificationInvariant {
            what: "contfp ray schedule exhausted",
        },
        ContainError::Uncrossable(cause) => {
            BooleanError::ArcLoopContainmentUnsupported { operand, cause }
        }
        ContainError::Corrupt => BooleanError::corrupt_at(operand, VertexKey::default()),
    }
}

impl Operand {
    /// The other operand.
    pub fn other(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

/// Orients a v-v contact: `x_is` names the operand `wx` lives in.
fn push_vv(contacts: &mut ContactAcc, x_is: Operand, wx: VertexKey, wy: VertexKey) {
    let c = match x_is {
        Operand::A => VvContact { a: wx, b: wy },
        Operand::B => VvContact { a: wy, b: wx },
    };
    contacts.vv(c);
}

/// `dovertexonface`: an existing vertex of `x` lies on `face`'s plane —
/// classify it against the face and record the contact kind. Returns
/// whether the exact predicates ACCEPTED an event (anything but `Out`)
/// — the differential suite's accepted-pair channel; recording changes
/// no classification.
#[allow(clippy::too_many_arguments)]
fn vertex_on_face<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    plane: &PlaneDesc<T>,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<bool, BooleanError> {
    match contfp(y, face, plane.normal, px, band).map_err(|e| esc(e, x_is.other()))? {
        FaceContainment::Out => return Ok(false),
        FaceContainment::In => contacts.vf(x_is, VfContact { vertex: vx, face }),
        FaceContainment::OnEdge(ey) => {
            let wy = split_other_at_point(y, x_is.other(), ey, px, band, tol)?;
            push_vv(contacts, x_is, vx, wy);
        }
        FaceContainment::OnVertex(vy) => push_vv(contacts, x_is, vx, vy),
    }
    Ok(true)
}

fn split_at<T: Decide + crate::props::AtRestPolicy>(
    x: &mut Body<T>,
    x_is: Operand,
    edge: EdgeKey,
    t: T,
    tol: Tol,
) -> Result<VertexKey, BooleanError> {
    x.split_edge(edge, t, tol)
        .map(|c| c.vertex)
        .map_err(|source| BooleanError::CrossingInsertion {
            operand: x_is,
            edge,
            source,
        })
}

/// Splits the OTHER solid's boundary edge at the (already-computed)
/// event point `p` — the both-edges-split lane that turns an edge-edge
/// crossing into a v-v pair.
///
/// Two carriers have an exact point parameter and both are taken:
///
/// - **`Line`**: the projection `t = (p − origin)·dir`.
/// - **`Circle`**: the azimuth anchored at the span's MIDPOINT. Both
///   are [`geom::Curve3::param_near`], which carries the derivation and
///   the reason an anchored read needs no branch selection; the
///   midpoint is this site's anchor because the parameter wanted is the
///   one INSIDE the stored span, and under the period guard below the
///   nearest branch to the midpoint is exactly that one. Where an
///   interval enclosure of it is too wide to place `t` strictly inside
///   the span, `split_edge`'s own interiority trilean escalates.
///
/// `p` must lie ON the carrier for the azimuth to name the event: the
/// distance from `p` to the circle (radial and axial misses folded, the
/// exact hypotenuse) is classified on `bool_contact_arc` — the same row
/// the boundary pre-pass uses for the same quantity — before
/// the parameter is taken. The angular half of "on the ARC" is not
/// repeated here: `split_edge`'s interiority gate is exactly that
/// question, metered in metres at the radius.
///
/// `Ellipse` and `Nurbs` carriers keep the typed refusal
/// [`BooleanError::PointSplitCarrierUnsupported`], its own variant
/// because this precondition is NOT the operand gate's — the gate
/// admits `Ellipse` and this lane cannot take it.
fn split_other_at_point<T: Decide + crate::props::AtRestPolicy>(
    y: &mut Body<T>,
    y_is: Operand,
    edge: EdgeKey,
    p: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<VertexKey, BooleanError> {
    let curve = certified(y.get_edge(edge).and_then(|e| y.get_curve_geom(e.curve)))?.clone();
    let (t0, t1) = curve.params();
    // The circle's two preconditions, both of them this site's and
    // neither of them the shared arithmetic's.
    if let geom::Curve3::Circle {
        center,
        axis,
        radius,
        ..
    } = *curve.carrier()
    {
        // On the carrier? `point_on_circle` is the row's body, with its
        // impossible-negative arm; the boundary pre-pass's conic arm
        // asks the same distance under the same row name.
        match super::contain::point_on_circle(p, center, axis, radius, band) {
            Ok(Some(_)) => {}
            // The caller placed the event ON this edge; a point
            // definitely off its carrier means two exact rows disagree,
            // which is a broken invariant, not a frontier.
            Ok(None) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "split point definitely off the circle carrier it was placed on",
                });
            }
            Err(diag) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::SplitPointOnCircle,
                    diag,
                });
            }
        }
        // A span of at most one period is what makes the MIDPOINT
        // anchor's branch the right one, and it is CHECKED rather than
        // assumed: past a period the azimuth aliases by 2π silently,
        // and `split_edge`'s interiority gate cannot see it (an aliased
        // parameter is still inside a span that long). The row is the
        // period guard the boundary pre-pass's conic arm already spells,
        // metered the same way; a full turn is `Zero` and passes.
        match decide(
            "bool_split_span_period",
            Margin::levered(T::tau() - (t1 - t0), radius),
            band,
        ) {
            Ok(Sign::Positive | Sign::Zero) => {}
            Ok(Sign::Negative) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "circle edge span exceeds one period; the split azimuth \
                           would alias by a turn",
                });
            }
            Err(diag) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::ArcSpan,
                    diag,
                });
            }
        }
    }
    let t = curve
        .carrier()
        .param_near(p, geom::mid_param(t0, t1))
        .ok_or(BooleanError::PointSplitCarrierUnsupported {
            operand: y_is,
            edge,
        })?;
    split_at(y, y_is, edge, t, tol)
}

/// Requeues both children of a just-split edge (parent keeps the
/// leading span and its key; `w` is the minted vertex whose emanating
/// half-edge names the trailing child).
fn requeue<T: Decide>(
    worklist: &mut std::collections::VecDeque<(EdgeKey, usize)>,
    x: &Body<T>,
    parent: EdgeKey,
    w: VertexKey,
    next_face: usize,
) -> Result<(), BooleanError> {
    let emanating =
        x.get_vertex(w)
            .and_then(|v| v.emanating)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "split vertex without emanating half-edge",
            })?;
    let child = x.get_half_edge(emanating).map(|h| h.edge).ok_or(
        BooleanError::ClassificationInvariant {
            what: "split child edge unresolvable",
        },
    )?;
    worklist.push_back((parent, next_face));
    worklist.push_back((child, next_face));
    Ok(())
}

#[cfg(test)]
mod undeclared_rule_rows {
    //! **The undeclared `NoInterior` rule, over every placement pair.**
    //! Both undeclared arms — the mixed-sign span and the `(Zero, Zero)`
    //! span — call [`Placement::undeclared_no_interior`], so these rows
    //! hold the rule for both. The row a public fixture cannot reach is
    //! the `Undecided` one: an end on a face whose trim the chart door
    //! declines (a ringed face, a non-rectangular outline) is minted by
    //! no public door that does not refuse first (a notched three-face
    //! wall stops at the volume backstop), so it is held here.
    #![allow(clippy::panic)]

    use super::{CurvedEvent, Placement};

    fn rule(ends: [Option<Placement>; 2]) -> &'static str {
        match Placement::undeclared_no_interior::<f64>(ends) {
            Some(CurvedEvent::Recorded) => "record",
            Some(CurvedEvent::None) => "none",
            Some(CurvedEvent::Pierce { .. } | CurvedEvent::Deferred) => {
                panic!("the rule never pierces or holds")
            }
            None => "door",
        }
    }

    #[test]
    fn an_undecided_end_keeps_the_door_unless_the_other_end_recorded() {
        use Placement::{Elsewhere as E, Recorded as R, Undecided as U};
        let rows = [
            ([Some(R), Some(R)], "record"),
            ([Some(R), Some(E)], "record"),
            ([Some(R), Some(U)], "record"),
            ([Some(E), Some(E)], "none"),
            ([Some(E), Some(U)], "door"),
            ([Some(U), Some(E)], "door"),
            ([Some(U), Some(U)], "door"),
            // The mixed-sign span: one ON end.
            ([Some(R), None], "record"),
            ([None, Some(E)], "none"),
            ([Some(U), None], "door"),
        ];
        for (ends, want) in rows {
            assert_eq!(rule(ends), want, "{ends:?}");
        }
    }

    /// **The declared rule**: an `Undecided` end always keeps the door,
    /// and an all-`Elsewhere` pair is no event only beside a certified
    /// clear interior.
    #[test]
    fn the_declared_rule_reads_all_elsewhere_by_the_interior_certificate() {
        use Placement::{Elsewhere as E, Recorded as R, Undecided as U};
        let declared =
            |ends: [Option<Placement>; 2], clear| match Placement::declared::<f64>(ends, clear) {
                Some(CurvedEvent::Recorded) => "record",
                Some(CurvedEvent::None) => "none",
                Some(CurvedEvent::Pierce { .. } | CurvedEvent::Deferred) => {
                    panic!("the rule never pierces or holds")
                }
                None => "door",
            };
        let rows = [
            ([Some(R), Some(E)], "record", "record"),
            ([Some(R), Some(U)], "door", "door"),
            ([Some(E), Some(E)], "door", "none"),
            ([Some(E), Some(U)], "door", "door"),
            ([Some(R), None], "record", "record"),
            ([None, Some(E)], "door", "none"),
        ];
        for (ends, unseen, clear) in rows {
            assert_eq!(declared(ends, false), unseen, "{ends:?}, interior unseen");
            assert_eq!(declared(ends, true), clear, "{ends:?}, interior clear");
        }
    }
}

#[cfg(test)]
#[path = "coplanar_conic_rows.rs"]
pub(super) mod coplanar_conic_rows;

#[cfg(test)]
#[path = "planar_lane_carrier_rows.rs"]
mod planar_lane_carrier_rows;

/// **A curved face's escalations read no declaration ahead of them, and
/// offer none**, on the review's executed raises (its
/// `probe_wall_roots_in_band_with_and_without_declaration`, adopted
/// here): an edge in the side face of a prism against a unit cylinder
/// wall, run through `curved_face_arm` with that side face undeclared,
/// declared `Tangent`, and declared `Rest`.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod declaration_order_rows {
    use super::{ContactAcc, CurvedEvent, FaceContainment, curved_face_arm};
    use crate::boolean::{
        BooleanDecision, BooleanDeclarations, BooleanError, Coincide, DeclarationRead,
        DeclaredPairs, FacePairDeclaration, Operand,
    };
    use crate::contact::{BooleanCoincidence, ContactClass};
    use crate::entity::VertexKey;
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism_z};
    use geom_core::{Band, Point3, Tol};

    /// Whether the declaration door verifies the side face through the
    /// edge of [`run`]'s prism as `class` against the wall, along the
    /// vertical line `x = profile[3].0`.
    fn door_verifies(class: ContactClass, profile: [(f64, f64); 4], z0: f64) -> bool {
        let tol = Tol::witness();
        let prism = prism_z::<f64>(&profile, z0, z0 + 0.5, tol);
        let mut y: crate::body::Body<f64> = crate::body::Body::new();
        let wall = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            None,
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let witness = geom::Curve3::Line {
            origin: Point3::new(profile[3].0, 0.0, 0.0),
            dir: geom_core::Vec3::new(0.0, 0.0, 1.0),
        };
        crate::boolean::contact_pair_verdict(
            &prism.body,
            prism.side_faces[3],
            &y,
            wall,
            class,
            Some((&witness, z0, z0 + 0.5)),
            Band::linear(tol).expect("the witness band"),
        )
        .is_ok()
    }

    /// The edge from `profile[3]` to `profile[0]` at height `z0` of a
    /// prism over `profile`, against a canonical unit cylinder wall, the
    /// prism's side face through that edge declared against the wall as
    /// `class`; what `curved_face_arm` answers (the review's `run`).
    fn run(
        class: Option<ContactClass>,
        profile: [(f64, f64); 4],
        z0: f64,
    ) -> Result<(), BooleanError> {
        let tol = Tol::witness();
        let b = Band::linear(tol).expect("the witness band");
        let prism = prism_z::<f64>(&profile, z0, z0 + 0.5, tol);
        let x = prism.body;
        let (u, v) = (prism.bottom[3], prism.bottom[0]);
        let pt = |k: VertexKey| {
            *x.get_point(x.get_vertex(k).expect("a live vertex").point)
                .expect("a live point")
        };
        let (edge_key, edge) = x
            .edges()
            .find(|(_, e)| {
                let ends = [e.he_plus, e.he_minus]
                    .map(|he| x.get_half_edge(he).expect("a live half-edge").start);
                ends == [u, v] || ends == [v, u]
            })
            .map(|(k, e)| (k, e.clone()))
            .expect("the bottom edge");
        let (a, c) = (
            x.get_half_edge(edge.he_plus)
                .expect("a live half-edge")
                .start,
            x.get_half_edge(edge.he_minus)
                .expect("a live half-edge")
                .start,
        );
        let mut y: crate::body::Body<f64> = crate::body::Body::new();
        let wall = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            None,
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let decls = BooleanDeclarations {
            coincident_faces: class
                .map(|class| vec![FacePairDeclaration::new(prism.side_faces[3], wall, class)])
                .unwrap_or_default(),
            carried_a: Default::default(),
            carried_b: Default::default(),
        };
        let declared = DeclaredPairs::<f64>::without_struts(&decls, Default::default());
        let mut acc = ContactAcc::default();
        curved_face_arm(
            &x,
            &mut y,
            Operand::A,
            edge_key,
            &edge,
            a,
            c,
            wall,
            pt(a),
            pt(c),
            &declared,
            &mut acc,
            b,
            tol,
        )
        .map(|_: CurvedEvent<f64>| ())
    }

    /// The one refusal every declaration posture gives: the coincidence
    /// `which`, with what the lookup read of the pair (nothing, or the
    /// declared class spent), and the same sentence, which offers no
    /// declaration.
    fn same_refusal_without_declare(
        runs: &[(Option<ContactClass>, Result<(), BooleanError>)],
        which: Coincide,
        subject: &str,
    ) {
        for (class, got) in runs {
            let Err(err) = got else {
                panic!("{class:?}: the in-band arm refuses");
            };
            let BooleanError::Escalated { decision, .. } = err else {
                panic!("{class:?}: an escalation: {err:?}");
            };
            assert_eq!(
                *decision,
                BooleanDecision::Coincidence(
                    which,
                    class.map_or(DeclarationRead::Moot, |c| DeclarationRead::Spent(c.into()))
                ),
                "{class:?}"
            );
            let text = err.to_string();
            assert!(
                text.starts_with(&format!("{subject} is undecided: ")) && !text.contains("declare"),
                "{class:?}: no declaration settles the question, so none is offered: {text}"
            );
            assert_eq!(
                text,
                runs[0]
                    .1
                    .as_ref()
                    .expect_err("the undeclared run refuses")
                    .to_string(),
                "{class:?}: a declaration changes nothing here"
            );
        }
    }

    /// **The `(Positive, Positive)` line-clearance arm**: a line clear of
    /// the wall by an in-band distance, both ends definitely outside, so
    /// the clearance bound escalates before any declaration is read.
    /// The declaration door takes none at this pose (a plane an in-band
    /// gap off the wall escalates `Tangent`, and `Rest` names two kinds
    /// of surface), so the declared run is built past the door and shows
    /// only that the arm reads none. Before the review's fix it offered
    /// "declare the coincidence".
    #[test]
    fn an_in_band_line_clearance_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let x0 = 1.0 + (b.zero() + b.escalate()) / 2.0;
        let profile = [(x0, -0.01), (x0 + 1.0, -0.01), (x0 + 1.0, 0.01), (x0, 0.01)];
        assert!(
            !door_verifies(ContactClass::Tangent, profile, 0.25),
            "no door takes the in-band plane's Tangent"
        );
        let runs: Vec<_> = [None, Some(ContactClass::Tangent)]
            .into_iter()
            .map(|class| (class, run(class, profile, 0.25)))
            .collect();
        same_refusal_without_declare(
            &runs,
            Coincide::EdgeOnCurvedFace,
            "whether an edge of one solid clears a curved face of the other or lies on it",
        );
    }

    /// **The curved arm's endpoint side**: a line in the plane `x = 1`
    /// that touches the wall exactly, one end off the tangency by the
    /// distance whose residual `y²/2` lies in the band. The endpoint's
    /// side escalates ahead of the declared-cover rungs, so a `Tangent`
    /// declaration of that plane, which the door verifies definite
    /// (`contact_pair_verdict`), changes nothing. Before the review's fix
    /// it offered "declare the coincidence".
    #[test]
    fn an_in_band_endpoint_on_a_curved_face_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let y0 = (b.zero() + b.escalate()).sqrt();
        let profile = [(1.0, y0), (2.0, y0), (2.0, 2.0), (1.0, 2.0)];
        assert!(
            door_verifies(ContactClass::Tangent, profile, 0.25),
            "the tangent plane's declaration is one the door takes"
        );
        let runs: Vec<_> = [None, Some(ContactClass::Tangent)]
            .into_iter()
            .map(|class| (class, run(class, profile, 0.25)))
            .collect();
        same_refusal_without_declare(
            &runs,
            Coincide::VertexOnCurvedFace,
            "whether a vertex of one solid lies on a face of the other",
        );
    }

    /// The decision `err` escalated on, or a panic naming `label`.
    fn decision_of<E: core::fmt::Debug>(
        label: &str,
        got: &Result<E, BooleanError>,
    ) -> BooleanDecision {
        match got {
            Err(BooleanError::Escalated { decision, .. }) => *decision,
            other => panic!("{label}: an escalation: {other:?}"),
        }
    }

    /// Each circle edge of a sheet of the cylinder `frame` over
    /// `angles` (heights 0.25 to 0.5), its sense reversed, through
    /// `curved_face_arm` against the canonical unit wall over angles
    /// (0, 3), the sheet declared against the wall as `class`
    /// (`one_carrier`: the door called the two one carrier); with what
    /// the declaration door answers `class` on the pair.
    ///
    /// Adopted from the review of PR 3513's fix pass
    /// (`zz_coincfr_rows.rs`, its `run`).
    fn circle_run(
        frame: CylFrame,
        angles: (f64, f64),
        class: Option<ContactClass>,
        one_carrier: bool,
    ) -> (Option<bool>, Vec<Result<CurvedEvent<f64>, BooleanError>>) {
        let tol = Tol::witness();
        let b = Band::linear(tol).expect("the witness band");
        let mut x: crate::body::Body<f64> = crate::body::Body::new();
        let xw = cyl_wall_sheet(&mut x, frame, None, angles, (0.25, 0.5), tol);
        let sense = x.get_face(xw).expect("the sheet's wall").sense;
        x.set_face_sense(xw, !sense).expect("a live face");
        let mut y: crate::body::Body<f64> = crate::body::Body::new();
        let yw = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            None,
            (0.0, 3.0),
            (0.0, 1.0),
            tol,
        );
        let decls = BooleanDeclarations {
            coincident_faces: class
                .map(|class| vec![FacePairDeclaration::new(xw, yw, class)])
                .unwrap_or_default(),
            carried_a: Default::default(),
            carried_b: Default::default(),
        };
        let one = if one_carrier {
            [(xw, yw)].into_iter().collect()
        } else {
            Default::default()
        };
        let declared = DeclaredPairs::<f64>::without_struts(&decls, one);
        let door = class.map(|class| {
            crate::boolean::contact_pair_verdict(&x, xw, &y, yw, class, None, b).is_ok()
        });
        let edges: Vec<_> = x.edges().map(|(k, e)| (k, e.clone())).collect();
        let mut out = Vec::new();
        for (edge_key, edge) in edges {
            let Some(super::CurveGeom::Certified(c)) = x.get_curve_geom(edge.curve).cloned() else {
                continue;
            };
            if !matches!(c.carrier(), geom::Curve3::Circle { .. }) {
                continue;
            }
            let start = |he| x.get_half_edge(he).expect("a live half-edge").start;
            let (a, c) = (start(edge.he_plus), start(edge.he_minus));
            let pt = |k: VertexKey| {
                *x.get_point(x.get_vertex(k).expect("a live vertex").point)
                    .expect("a live point")
            };
            let mut acc = ContactAcc::default();
            out.push(curved_face_arm(
                &x,
                &mut y,
                Operand::A,
                edge_key,
                &edge,
                a,
                c,
                yw,
                pt(a),
                pt(c),
                &declared,
                &mut acc,
                b,
                tol,
            ));
        }
        assert_eq!(out.len(), 2, "the sheet's two rim circles");
        (door, out)
    }

    /// **The uncovered circle's clearance offers no declaration**, on
    /// the review's executed raise: a sheet of radius `1 + mid` against
    /// the unit wall. Undeclared, the escalated clearance hands the arc
    /// to the root lane as a sphere's or a torus's does, and the
    /// endpoint sides that guard it read the same in-band pose. Declared `Rest`, which the door verifies,
    /// with the door calling the two one carrier, the rung reads the
    /// clearance zero, and the covered arm's endpoint sides refuse the
    /// same in-band pose. Not called one carrier, or declared `Tangent`
    /// the door does not verify, the pair carries no certificate, so the
    /// circle is uncovered and its clearance refuses as the undeclared
    /// one's does. No posture passes, so no refusal offers a
    /// declaration, and each states what its door read.
    #[test]
    fn an_uncovered_circle_clearance_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let frame = CylFrame::canonical(1.0 + (b.zero() + b.escalate()) / 2.0);
        let uncovered = Coincide::VertexOnCurvedFace;
        let postures = [
            (
                None,
                false,
                None,
                Coincide::VertexOnCurvedFace,
                DeclarationRead::Moot,
            ),
            (
                Some(ContactClass::Rest),
                true,
                Some(true),
                Coincide::VertexOnCoveredFace,
                DeclarationRead::Spent(BooleanCoincidence::REST),
            ),
            (
                Some(ContactClass::Rest),
                false,
                Some(true),
                uncovered,
                DeclarationRead::Spent(BooleanCoincidence::REST),
            ),
            (
                Some(ContactClass::Tangent),
                false,
                Some(false),
                uncovered,
                DeclarationRead::Spent(BooleanCoincidence::TANGENT),
            ),
        ];
        for (class, one, door, which, read) in postures {
            let label = format!("{class:?}, one carrier {one}");
            let (verdict, runs) = circle_run(frame, (0.5, 1.0), class, one);
            assert_eq!(verdict, door, "{label}: what the door answers");
            for got in &runs {
                assert_eq!(
                    decision_of(&label, got),
                    BooleanDecision::Coincidence(which, read),
                    "{label}"
                );
                let text = got.as_ref().expect_err("it refuses").to_string();
                assert!(!text.contains("declare"), "{label}: {text}");
            }
        }
    }

    /// **The circle rung's decided arms, under each declaration**
    /// (`curved-pierce-frontier-tells-one-story-for-several-decisions`):
    /// its `Zero` arm, a sheet of radius `1 + zero/2`, refuses at the
    /// frontier undeclared and is recorded once a `Rest` declaration the
    /// door verifies calls the two one carrier, so the frontier's
    /// declare offer is true there; its `Negative` arm, a rim that
    /// crosses the wall (a sheet whose axis is offset by 0.3), takes no
    /// declaration either: the door refuses `Rest` (the axes are apart).
    /// Undeclared it is the circle × cylinder root lane's pierce, at the
    /// two unit circles' meeting point `(0.15, √(1 − 0.15²))`, and so it
    /// is under a `Tangent` the door does not verify, which carries no
    /// certificate. A pair called one carrier past the door is covered,
    /// and the covered arm keeps the frontier on a crossing (it reads
    /// its clearance zero by that certificate, and refuses all the same).
    #[test]
    fn the_circle_rungs_zero_arm_takes_a_declaration_and_its_crossing_arm_takes_none() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let frontier = |label: &str, got: &Result<CurvedEvent<f64>, BooleanError>| {
            assert!(
                matches!(got, Err(BooleanError::CurvedPierceUnsupported { .. })),
                "{label}: the frontier: {got:?}"
            );
        };
        let zero = CylFrame::canonical(1.0 + 0.5 * b.zero());
        for got in circle_run(zero, (0.5, 1.0), None, false).1 {
            frontier("zero arm, undeclared", &got);
        }
        let (door, runs) = circle_run(zero, (0.5, 1.0), Some(ContactClass::Rest), true);
        assert_eq!(door, Some(true), "the door takes the zero arm's Rest");
        for got in runs {
            assert!(got.is_ok(), "zero arm, declared Rest: recorded: {got:?}");
        }
        let crossing = CylFrame {
            origin: Point3::new(0.3, 0.0, 0.0),
            ..CylFrame::canonical(1.0)
        };
        for (class, one) in [
            (None, false),
            (Some(ContactClass::Rest), true),
            (Some(ContactClass::Tangent), false),
        ] {
            let (door, runs) = circle_run(crossing, (1.5, 2.5), class, one);
            assert_ne!(
                door,
                Some(true),
                "{class:?}: the door takes no declaration here"
            );
            if one {
                for got in &runs {
                    frontier(&format!("crossing arm, {class:?}"), got);
                }
                continue;
            }
            let mut heights = Vec::new();
            for got in &runs {
                let Ok(CurvedEvent::Pierce { p, at, .. }) = got else {
                    panic!("crossing arm, {class:?}: a pierce, got {got:?}");
                };
                assert_eq!(
                    *at,
                    FaceContainment::In,
                    "{class:?}: inside the wall's trim"
                );
                let off = (p.x - 0.15).hypot(p.y - (1.0_f64 - 0.15 * 0.15).sqrt());
                assert!(off < 1e-12, "{class:?}: the pierce {p:?} is {off} off");
                heights.push(p.z);
            }
            heights.sort_by(f64::total_cmp);
            assert!(
                heights.len() == 2
                    && (heights[0] - 0.25).abs() < 1e-12
                    && (heights[1] - 0.5).abs() < 1e-12,
                "{class:?}: one pierce per rim, got heights {heights:?}"
            );
        }
    }

    /// **The circle × torus lane escalates as its own decision, and no
    /// declaration settles it, on the geometry of its raise.** An arc
    /// of a level circle an in-band depth under the top of a torus's
    /// tube (a parallel of a torus sheet whose centre stands 0.3 off the
    /// other's axis), with both ends
    /// definitely outside the tube and its middle over the tube's top:
    /// through `curved_face_arm` the clearance does not decide, the ends
    /// do, and the root lane asked of the arc escalates on its plane
    /// height. The lane's rungs pass on different sets and units, and the
    /// escalation names its predicate only, so it ends on its lever alone.
    /// The door takes neither class for that face pair: `Rest` meets two
    /// tori whose centres are apart, and `Tangent` has no witness for a
    /// torus and no shared rim.
    #[test]
    fn the_circle_torus_lane_escalates_as_its_own_decision_and_no_declaration_settles_it() {
        use crate::boolean::boxes::tests::torus_wall;
        let tol = Tol::witness();
        let b = Band::linear(tol).expect("the witness band");
        let mid = (b.zero() + b.escalate()) / 2.0;
        let (axis, u_ref) = (
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
        );
        // The parallel at tube angle `asin(1 − 2·mid)` stands `mid`
        // under the top of a tube of radius ½, just inside the tube's
        // crown, where the sheet still certifies.
        let (x, xw) = torus_wall(
            Point3::new(0.3, 0.0, 0.0),
            axis,
            u_ref,
            2.0,
            0.5,
            (1.2, 2.1),
            (0.0, (1.0 - 2.0 * mid).asin()),
        );
        let (mut y, yw) = torus_wall(
            Point3::new(0.0, 0.0, 0.0),
            axis,
            u_ref,
            2.0,
            0.5,
            (0.0, 3.0),
            (0.0, 1.0),
        );
        for class in ContactClass::ALL {
            let door = match class {
                ContactClass::Rest => {
                    crate::boolean::contact_pair_verdict(&x, xw, &y, yw, *class, None, b).is_ok()
                }
                ContactClass::Tangent => {
                    crate::boolean::verify_tangent_declaration(&x, xw, &y, yw, b).is_ok()
                }
            };
            assert!(
                !door,
                "{class:?}: the door takes no declaration of this pair"
            );
        }
        let (edge_key, edge) = x
            .edges()
            .find(|(_, e)| {
                matches!(
                    x.get_curve_geom(e.curve),
                    Some(super::CurveGeom::Certified(g))
                        if matches!(
                            g.carrier(),
                            geom::Curve3::Circle { axis: a, center, .. }
                                if a.z.abs() > 0.9 && center.z > 0.1
                        )
                )
            })
            .map(|(k, e)| (k, e.clone()))
            .expect("the sheet's top parallel");
        let start = |he| x.get_half_edge(he).expect("a live half-edge").start;
        let (a, c) = (start(edge.he_plus), start(edge.he_minus));
        let pt = |k: VertexKey| {
            *x.get_point(x.get_vertex(k).expect("a live vertex").point)
                .expect("a live point")
        };
        let declared =
            DeclaredPairs::<f64>::without_struts(&BooleanDeclarations::none(), Default::default());
        let mut acc = ContactAcc::default();
        let got = curved_face_arm(
            &x,
            &mut y,
            Operand::A,
            edge_key,
            &edge,
            a,
            c,
            yw,
            pt(a),
            pt(c),
            &declared,
            &mut acc,
            b,
            tol,
        )
        .map(|_: CurvedEvent<f64>| ())
        .expect_err("the arc over the tube's top escalates");
        let BooleanError::Escalated { decision, diag } = &got else {
            panic!("an escalation: {got:?}");
        };
        assert_eq!(
            *decision,
            BooleanDecision::ArcTorusRoots,
            "the lane's own decision"
        );
        assert_eq!(diag.predicate, Some("bool_circle_torus_plane_height"));
        let text = got.to_string();
        assert!(
            text.starts_with("how many times an arc crosses a torus is undecided: ")
                && text.ends_with(
                    "Recourse: move the parts so the arc clearly crosses the torus or clearly \
                     misses it"
                ),
            "{text}"
        );
    }

    /// The unique planar face of `body` whose outward normal is within a
    /// milliradian of `n`.
    fn face_facing(body: &crate::body::Body<f64>, n: [f64; 3]) -> crate::entity::FaceKey {
        let n = geom_core::Vec3::from_array(n);
        let hits: Vec<_> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|&k| {
                matches!(crate::boolean::face_carrier(body, k),
                    Some(crate::boolean::CarrierDesc::Plane { normal, .. }) if normal.dot(n) > 1.0 - 1e-6)
            })
            .collect();
        assert_eq!(hits.len(), 1, "one face faces {n:?}");
        hits[0]
    }

    /// `a ∪ b` with the pair `(fa, fb)` declared as `class`.
    fn union_declared(
        a: &crate::body::Body<f64>,
        b: &crate::body::Body<f64>,
        pair: (crate::entity::FaceKey, crate::entity::FaceKey),
        class: Option<BooleanCoincidence>,
    ) -> Result<(), BooleanError> {
        let decls = BooleanDeclarations {
            coincident_faces: class
                .map(|class| vec![FacePairDeclaration::new(pair.0, pair.1, class)])
                .unwrap_or_default(),
            ..BooleanDeclarations::none()
        };
        crate::boolean::union_with(a, b, &decls, Tol::witness()).map(|_| ())
    }

    /// **A declared-`Tangent` pair's plane rung offers no second
    /// declaration**, on the review's union poses (`zz_coincfr_probe.rs`,
    /// G2, G5 and G6): a face of each solid in-band parallel, the pair
    /// declared `Tangent`. The Tangent verification's conformal screen
    /// runs the plane ladder on a pair already declared, so the refusal
    /// states that read, names the tilt, and offers no declaration and
    /// no tolerance: a smaller one decides the tilt, and the door then
    /// refuses a `Tangent` claim on two planes whatever it decided
    /// (executed: `offer_rows`' `tangent_screen_of_a_tilted_block`).
    #[test]
    fn a_declared_tangent_pairs_plane_rung_offers_no_declaration() {
        use crate::test_support_fixtures::{brick, mapped_cube};
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let d = (band.zero() + band.escalate()) / 2.0;
        let cube = |map: fn(f64, f64, f64, f64) -> Point3<f64>| {
            mapped_cube::<f64>(move |u, v, w| map(u, v, w, d), tol)
        };
        type Pose = (
            &'static str,
            crate::body::Body<f64>,
            crate::body::Body<f64>,
            [f64; 3],
        );
        let poses: [Pose; 3] = [
            (
                "G2, a tilted block on a block",
                brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol),
                cube(|u, v, w, d| Point3::new(1.0 + 2.0 * u, 1.0 + 2.0 * v, 1.0 + w + d * u)),
                [0.0, 0.0, 1.0],
            ),
            (
                "G5, a block tilted along another's edge",
                brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                cube(|u, v, w, d| Point3::new(1.0 + u + d * w, 1.0 + v, w)),
                [1.0, 0.0, 0.0],
            ),
            (
                "G6, a sheared block on another's corner",
                brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
                cube(|u, v, w, d| Point3::new(1.0 + u + d * v, 1.0 + v, 1.0 + w + d * u)),
                [0.0, 0.0, 1.0],
            ),
        ];
        for (label, a, b, n) in poses {
            let pair = (face_facing(&a, n), face_facing(&b, [-n[0], -n[1], -n[2]]));
            let got = union_declared(&a, &b, pair, Some(BooleanCoincidence::TANGENT));
            assert_eq!(
                decision_of(label, &got),
                BooleanDecision::Coincidence(
                    Coincide::Planes,
                    DeclarationRead::Spent(BooleanCoincidence::TANGENT)
                ),
                "{label}"
            );
            let text = got.expect_err("it refuses").to_string();
            assert!(
                text.starts_with("whether a face of each solid lies on one plane is undecided: ")
                    && text.ends_with(
                        "Whichever way it reads, the Boolean cannot yet act on a Tangent \
                         contact declared between two plane faces. There is no way through yet"
                    )
                    && !text.contains("declare the")
                    && !text.contains("Recourse"),
                "{label}: {text}"
            );
        }
    }

    /// **A coplanar sector's in-band parallelism offers the declaration
    /// that settles it**, on a union: a parallelepiped cornered at a
    /// point of a block's top face by a 5° wedge angle, its face there
    /// tilted by an in-band angle about the wedge's one edge, so the
    /// wedge's two edges read on that face while its normal reads
    /// in-band parallel at the corner's arm. Two poses: standing on the
    /// block, its bottom face opposed to the block's top; and sunk into
    /// it, its top face flush with the block's top and aligned with it.
    /// Undeclared, the sector refuses and offers the one-carrier
    /// coincidence the senses make the pair (`Rest` opposed, a
    /// continuation aligned). The block's top reaches far enough from
    /// the tilt axis that its corners read definitely off the wedge's
    /// plane, so the door, which reads the pair across both faces,
    /// contradicts the offered declaration too: the offer does not
    /// settle these poses
    /// (`work/hone/a-coplanar-sector-offers-a-rest-the-door-contradicts-across-the-faces.md`).
    /// The other class is contradicted as well.
    /// **The lump takes a sector's in-band residue where the door
    /// bridges it**: the two poses of the row below at a tilt the door
    /// reads in band over both faces (standing tilted down by `1.2·ε`,
    /// sunk at `2·ε`; standing at `2·ε` the zip refuses
    /// `RestZipUnsupported { ChordBetweenIsolatedPierces }`). Standing
    /// tilted UP, the union's residue crosses `vol(A) + vol(B)` and the
    /// volume backstop refuses it
    /// (`work/reach/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`,
    /// pinned in `topo/tests/door_backstop_settled_residue.rs`). Undeclared,
    /// the sector offers the class the senses make the pair; following
    /// the offer, the union builds at the volume box arithmetic gives,
    /// and the other class is contradicted.
    #[test]
    fn a_coplanar_sectors_in_band_residue_builds_through_the_lump_where_the_door_bridges_it() {
        use crate::test_support_fixtures::{brick, mapped_cube};
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let phi = 5.0_f64.to_radians();
        let p = Point3::new(0.5, 0.2, 1.0);
        let block = brick((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
        let block_volume = 3.0 * 4.5;
        for (label, theta, sunk, facing, offered, other, volume) in [
            (
                "standing on the block",
                -1.2 * band.zero(),
                false,
                -1.0,
                BooleanCoincidence::REST,
                BooleanCoincidence::Continuation,
                // The parallelepiped's volume: its base
                // parallelogram's area, `sin φ`, times its height.
                block_volume + phi.sin(),
            ),
            (
                "sunk into the block",
                2.0 * band.zero(),
                true,
                1.0,
                BooleanCoincidence::Continuation,
                BooleanCoincidence::REST,
                block_volume,
            ),
        ] {
            let (ea, eb) = (
                geom_core::Vec3::new(1.0, 0.0, 0.0),
                geom_core::Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
            );
            let wedge = mapped_cube::<f64>(
                move |u, v, w| {
                    let z = if sunk { 0.5 * (w - 1.0) } else { w };
                    p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, z)
                },
                tol,
            );
            let pair = (
                face_facing(&block, [0.0, 0.0, 1.0]),
                face_facing(&wedge, [0.0, 0.0, facing]),
            );
            let undeclared = union_declared(&block, &wedge, pair, None);
            assert!(
                matches!(
                    decision_of(label, &undeclared),
                    BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Settles(s))
                        if s.class() == offered
                ),
                "{label}: the lookup offers {offered:?}: {undeclared:?}"
            );
            let decls = BooleanDeclarations {
                coincident_faces: vec![FacePairDeclaration::new(pair.0, pair.1, offered)],
                ..BooleanDeclarations::none()
            };
            let built = crate::boolean::union_with(&block, &wedge, &decls, tol)
                .unwrap_or_else(|e| panic!("{label}: following the offer, it builds: {e:?}"));
            let body = &built.body().expect("a union is not empty").body;
            let got = crate::mass_properties(body, tol)
                .expect("its volume")
                .volume;
            // The lump takes an in-band residue: the face it glues
            // moves by less than the band over less than unit area.
            assert!(
                (got - volume).abs() <= band.escalate(),
                "{label}: {got} vs {volume}"
            );
            let contradicted = union_declared(&block, &wedge, pair, Some(other));
            assert!(
                matches!(
                    contradicted,
                    Err(BooleanError::ContactContradicted { .. }
                        | BooleanError::ContinuationContradicted { .. })
                ),
                "{label}: declared {other:?}: {contradicted:?}"
            );
        }
    }

    #[test]
    fn a_coplanar_sectors_in_band_parallelism_offers_a_declaration_the_door_reads_across_the_faces()
    {
        use crate::test_support_fixtures::{brick, mapped_cube};
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let theta = (band.zero() + band.escalate()) / 2.0;
        let phi = 5.0_f64.to_radians();
        let (ea, eb) = (
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
        );
        let p = Point3::new(0.5, 0.2, 1.0);
        // Its top face reaches far enough from the tilt axis that each
        // of its corners reads definitely off the wedge's tilted plane.
        let block = brick((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
        type Pose = (
            &'static str,
            crate::body::Body<f64>,
            f64,
            BooleanCoincidence,
            BooleanCoincidence,
        );
        let poses: [Pose; 2] = [
            (
                "standing on the block",
                mapped_cube::<f64>(
                    move |u, v, w| p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, w),
                    tol,
                ),
                -1.0,
                BooleanCoincidence::REST,
                BooleanCoincidence::Continuation,
            ),
            (
                "sunk into the block",
                mapped_cube::<f64>(
                    move |u, v, w| {
                        p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, 0.5 * (w - 1.0))
                    },
                    tol,
                ),
                1.0,
                BooleanCoincidence::Continuation,
                BooleanCoincidence::REST,
            ),
        ];
        for (label, wedge, facing, offered, other) in poses {
            let pair = (face_facing(&block, [0.0, 0.0, 1.0]), {
                let hits: Vec<_> = wedge
                    .faces()
                    .map(|(k, _)| k)
                    .filter(|&k| {
                        matches!(crate::boolean::face_carrier(&wedge, k),
                            Some(crate::boolean::CarrierDesc::Plane { normal, .. })
                                if normal.z * facing > 0.99)
                    })
                    .collect();
                assert_eq!(
                    hits.len(),
                    1,
                    "{label}: the wedge's face on the block's top"
                );
                hits[0]
            });
            let undeclared = union_declared(&block, &wedge, pair, None);
            assert!(
                matches!(
                    decision_of(label, &undeclared),
                    BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Settles(s))
                        if s.class() == offered
                ),
                "{label}: the lookup offers {offered:?}, as the senses make the pair: \
                 {undeclared:?}"
            );
            let text = undeclared.expect_err("it refuses").to_string();
            assert!(
                text.starts_with("how two corners of the two solids overlap where they meet is ")
                    && text.contains(
                        "Recourse: declare the coincidence, or move the parts so they clearly \
                         meet or clearly stand apart there, or, if this gap is intended, tighten \
                         the tolerance below "
                    ),
                "{label}: {text}"
            );
            let decls = BooleanDeclarations {
                coincident_faces: vec![FacePairDeclaration::new(pair.0, pair.1, offered)],
                ..BooleanDeclarations::none()
            };
            let followed = crate::boolean::union_with(&block, &wedge, &decls, tol);
            assert!(
                matches!(
                    followed,
                    Err(BooleanError::ContactContradicted {
                        fact: Some(
                            crate::boolean::refusal_routes::Contradiction::PlanesNotParallel
                        ),
                        ..
                    } | BooleanError::ContinuationContradicted {
                        fact: Some(
                            crate::boolean::refusal_routes::Contradiction::PlanesNotParallel
                        ),
                        ..
                    })
                ),
                "{label}: following the offer, the tilt across the block's top contradicts it: \
                 {:?}",
                followed.err()
            );
            let contradicted = union_declared(&block, &wedge, pair, Some(other));
            assert!(
                matches!(
                    contradicted,
                    Err(BooleanError::ContactContradicted { .. }
                        | BooleanError::ContinuationContradicted { .. })
                ),
                "{label}: declared {other:?}: {contradicted:?}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod wall_root_tests {
    use super::{BooleanDecision, BooleanError, line_wall_roots_of};
    use crate::boolean::WallRung;
    use geom_core::{Band, Point3, Tol, Vec3};

    /// **The line × wall root lane escalates as its own decision, on a
    /// real raise**: a line across the axis of a unit cylinder, at the
    /// distance from it that puts the discriminant's depth `(r² − d²)/2r`
    /// in the band. The refusal names the rung's question and the one
    /// lever that reaches it, no tolerance (the arms that read it pass on
    /// different sets) and no declaration: no face pair says where an
    /// edge crosses a wall.
    #[test]
    fn the_wall_root_lane_escalates_as_its_own_decision() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let mid = (b.zero() + b.escalate()) / 2.0;
        let d = (1.0 - 2.0 * mid).sqrt();
        let wall = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let got = line_wall_roots_of(
            Point3::new(d, -2.0, 0.5),
            Vec3::new(0.0, 1.0, 0.0),
            4.0,
            &wall,
            b,
        );
        let Err(err) = got else {
            panic!("an in-band discriminant escalates");
        };
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the lane escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::WallRoots(WallRung::Discriminant));
        assert_eq!(diag.predicate, Some("bool_ray_cylinder_disc"));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        assert_eq!(test_utils::refusal::recourse_markers(&text), 1, "{text}");
        assert_eq!(
            text,
            format!(
                "whether an edge crosses a cylinder wall, grazes it or misses it is undecided: \
                 {}. Recourse: move the parts so the edge clearly crosses the wall or clearly \
                 misses it",
                diag.payload()
            )
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod edge_span_tests {
    use super::{SpanVerdict, wall_crossing};
    use crate::{Body, entity::FaceKey};
    use geom_core::{Band, Point3, Tol, Vec3};

    /// **The axis-parallel rung is levered by the edge's own span.** A
    /// line outside a unit wall, drifting off its distance from the axis
    /// by `δ` per unit of its parameter, over two runs: `[−2, 2]`, where
    /// `4δ` is past the band, so the roots are found and lie far outside
    /// the run; and `[1000, 1001]`, where `δ` is inside the zero band, so
    /// the residual is constant over the run. A lever of `1` reads the
    /// first in band; a lever of the far end, or of the run's distance
    /// from the line's origin, reads the second past it.
    #[test]
    fn the_axis_parallel_rung_reads_the_edges_own_span() {
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let wall = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let y = Body::<f64>::new();
        let crossing = |delta: f64, (t0, t1): (f64, f64)| {
            let line = geom::Curve3::Line {
                origin: Point3::new(2.0, 0.0, 0.0),
                dir: Vec3::new(delta, 0.0, 1.0),
            };
            wall_crossing(&y, FaceKey::default(), &wall, &line, t0, t1, band)
        };
        let long = crossing(0.4 * band.escalate(), (-2.0, 2.0));
        assert!(
            matches!(long, Ok(SpanVerdict::NoInterior)),
            "drifting past the band over [−2, 2], the roots are found outside it: {long:?}"
        );
        let far = crossing(0.5 * band.zero(), (1000.0, 1001.0));
        assert!(
            matches!(far, Ok(SpanVerdict::Constant)),
            "drifting inside the band over [1000, 1001], the residual is constant: {far:?}"
        );
    }
    /// **A root's distance from an end is metered at the carrier's speed
    /// THERE.** An ellipse of semi-axes 1 and 0.05 is crossed square by a
    /// wall at `θ = π/2 − 2e-8`, 2e-8 m of arc short of the span's end
    /// `π/2` of the span `[π/2 − 0.1, π/2]` (the wall's other crossings lie
    /// outside it), where the carrier runs at 1 m/rad — past the escalation
    /// threshold, so the root is interior and goes on to the trim (which
    /// an empty body cannot give, so `Unsettled`). Metered at the
    /// semi-minor axis, the least speed, the same gap reads 1e-9 m, inside
    /// the zero band: the root read as the end's own incidence, and the
    /// span `NoInterior`.
    #[test]
    fn a_root_near_an_end_is_metered_at_the_ends_speed() {
        let band = Band::new(1e-9, 1e-8).expect("a band");
        let ellipse = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: 1.0,
            minor: 0.05,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let root = core::f64::consts::FRAC_PI_2 - 2e-8;
        let p = ellipse.eval(root);
        let wall = geom::Surface::Cylinder {
            origin: Point3::new(p.x + 0.3, p.y, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.3,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let y = Body::<f64>::new();
        let got = wall_crossing(
            &y,
            FaceKey::default(),
            &wall,
            &ellipse,
            core::f64::consts::FRAC_PI_2 - 0.1,
            core::f64::consts::FRAC_PI_2,
            band,
        );
        assert!(
            matches!(got, Ok(SpanVerdict::Unsettled)),
            "the root is interior, and the empty body has no trim to place it in: {got:?}"
        );
    }
}

#[cfg(test)]
mod no_pierce_tests {
    use super::{SpanVerdict, no_pierce_verdict};

    /// **A straddle with no root in the span keeps the door.** Only an
    /// interior root placed off this face, with no root at an end,
    /// accounts for a straddle's crossing; every other combination is
    /// `NoInterior`, which the straddle arm refuses on.
    #[test]
    fn only_an_interior_root_off_the_face_is_elsewhere() {
        for (crossed, at_end, want_elsewhere) in [
            (true, false, true),
            (false, false, false),
            (true, true, false),
            (false, true, false),
        ] {
            let got = no_pierce_verdict::<f64>(crossed, at_end);
            assert_eq!(
                matches!(got, SpanVerdict::Elsewhere),
                want_elsewhere,
                "crossed_elsewhere {crossed}, at_end {at_end}: {got:?}"
            );
        }
    }
}

/// **The two certificates of [`lying_on`], held on their predicates.**
/// Each row poses one boundary shape the end-to-end fixtures never
/// reach: a body that would reach it builds no differently whichever
/// way the certificate answers, or no public door mints it. The faces
/// are a unit cylinder sheet (arcs at its ends, rulings at its sides)
/// and the `y = 0` side of a unit prism (four lines); the predicate
/// reads only the face's boundary, never its carrier.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod lying_on_rows {
    use super::{
        arc_chain_reaches, boundary_meets_circle_only_at, parents_distinct_from,
        split_other_at_point,
    };
    use crate::body::Body;
    use crate::boolean::Operand;
    use crate::entity::{EdgeKey, FaceKey, VertexKey};
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism_z};
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    /// A unit-radius sheet about `+z` over azimuth `[0, u1]`, `z ∈ [0, 1]`.
    fn sheet(u1: f64) -> (Body<f64>, FaceKey) {
        let mut y = Body::new();
        let face = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            None,
            (0.0, u1),
            (0.0, 1.0),
            Tol::witness(),
        );
        (y, face)
    }

    /// The unit prism's `y = 0` side face, and its vertices `(0,0,0)`,
    /// `(1,0,0)`, `(1,0,1)`, `(0,0,1)`.
    fn side() -> (Body<f64>, FaceKey, [VertexKey; 4]) {
        let p = prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let v = [p.bottom[0], p.bottom[1], p.top[1], p.top[0]];
        (p.body, p.side_faces[0], v)
    }

    fn vertex_at(y: &Body<f64>, p: Point3<f64>) -> VertexKey {
        y.vertices()
            .find(|(_, v)| {
                y.get_point(v.point)
                    .is_some_and(|q| (*q - p).norm() < 1e-12)
            })
            .map(|(k, _)| k)
            .unwrap_or_else(|| panic!("a vertex at {p:?}"))
    }

    fn edge_between(y: &Body<f64>, a: VertexKey, b: VertexKey) -> EdgeKey {
        y.edges()
            .find(|(_, e)| {
                let ends = [e.he_plus, e.he_minus].map(|h| y.get_half_edge(h).unwrap().start);
                ends == [a, b] || ends == [b, a]
            })
            .map(|(k, _)| k)
            .expect("an edge between the two vertices")
    }

    fn meets(
        y: &Body<f64>,
        face: FaceKey,
        (center, axis, radius): (Point3<f64>, Vec3<f64>, f64),
        at: &[VertexKey],
    ) -> bool {
        boundary_meets_circle_only_at(y, face, (center, axis, radius), at, band())
            .expect("a walkable boundary")
    }

    /// A conic edge crossing the circle's plane ON the circle meets it
    /// there; the same crossing off the circle does not. The half sheet's
    /// arcs cross `x = 0` at `(0, 1, 0)` and `(0, 1, 1)`, both on the
    /// circle of radius `1/2` about `(0, 1, 1/2)`.
    #[test]
    fn a_conic_crossing_on_the_circle_is_a_meeting() {
        let (y, face) = sheet(PI);
        let on = (Point3::new(0.0, 1.0, 0.5), Vec3::unit_x(), 0.5);
        let off = (Point3::new(0.0, 1.0, 0.5), Vec3::unit_x(), 0.25);
        assert!(
            !meets(&y, face, on, &[]),
            "the arcs cross the plane on the circle"
        );
        assert!(
            meets(&y, face, off, &[]),
            "and the same crossings off it are clear"
        );
    }

    /// A conic lying in the circle's plane does not certify, even ON
    /// the circle between two of `at`: the sheet's bottom arc is the
    /// circle itself.
    #[test]
    fn a_conic_in_the_plane_does_not_certify() {
        let (y, face) = sheet(FRAC_PI_2);
        let at = [
            vertex_at(&y, Point3::new(1.0, 0.0, 0.0)),
            vertex_at(&y, Point3::new(0.0, 1.0, 0.0)),
        ];
        let circle = (Point3::origin(), Vec3::unit_z(), 1.0);
        assert!(
            !meets(&y, face, circle, &at),
            "the bottom arc lies on the circle"
        );
        let lifted = (Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), 2.0);
        assert!(
            meets(&y, face, lifted, &[]),
            "a parallel plane between the arcs, its circle off the rulings, is clear"
        );
    }

    /// A line lying in the circle's plane certifies only as a chord
    /// between two of `at` whose midpoint is decided off the circle. The
    /// prism side's edge `x = 1` lies in the plane `x = 1`; the circles
    /// are in that plane.
    #[test]
    fn a_line_in_the_plane_certifies_only_as_a_chord_decided_off_the_circle() {
        let (y, face, v) = side();
        let crossed = (Point3::new(1.0, 0.0, 0.5), Vec3::unit_x(), 0.2);
        assert!(
            !meets(&y, face, crossed, &[]),
            "the edge crosses the circle twice"
        );
        let grazed = (Point3::new(1.0, 0.3, 0.5), Vec3::unit_x(), 0.3);
        assert!(
            !meets(&y, face, grazed, &[v[1], v[2]]),
            "between two of `at`, but touching the circle at its midpoint"
        );
        let chord = (Point3::new(1.0, 0.0, 0.5), Vec3::unit_x(), 0.5);
        assert!(
            meets(&y, face, chord, &[v[1], v[2]]),
            "a chord of the circle is clear"
        );
    }

    /// A vertex whose side of the plane escalates is placed nowhere,
    /// even when the line to it leaves the plane at once: its real
    /// crossing can lie `|h| / sin θ` away. The plane is tilted through
    /// `(1, 0, 0)` and offset from it in band.
    #[test]
    fn a_vertex_in_band_of_the_plane_does_not_certify() {
        let (y, face, _) = side();
        let b = band();
        let n = Vec3::new(1.0, 0.0, 0.1).normalize();
        let h = (b.zero() + b.escalate()) / 2.0;
        let near = (Point3::new(1.0, 0.0, 0.0) + n * h, n, 5.0);
        assert!(!meets(&y, face, near, &[]), "the corner's side escalates");
        let clear = (Point3::new(1.0, 0.0, 0.0) + n * 0.25, n, 5.0);
        assert!(
            meets(&y, face, clear, &[]),
            "the same plane off the corner is clear"
        );
    }

    /// An edge the certificate cannot place does not certify: the prism
    /// side's top edge re-described as a degree-1 NURBS line.
    #[test]
    fn a_nurbs_boundary_edge_does_not_certify() {
        let (mut y, face, v) = side();
        let plane = (Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), 5.0);
        assert!(
            meets(&y, face, plane, &[]),
            "the line-bounded face is clear"
        );
        let top = edge_between(&y, v[2], v[3]);
        let (p0, p1) = (Point3::new(1.0, 0.0, 1.0), Point3::new(0.0, 0.0, 1.0));
        let plus_start = y
            .get_half_edge(y.get_edge(top).unwrap().he_plus)
            .unwrap()
            .start;
        let (p0, p1) = if plus_start == v[2] {
            (p0, p1)
        } else {
            (p1, p0)
        };
        let (s1, s2) = crate::readback::edge_sides(&y, top).unwrap().surfaces();
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: p0.lerp(p1, 0.5),
            },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        };
        y.set_edge_curve(top, spec, Tol::witness())
            .expect("the NURBS line attaches");
        assert!(!meets(&y, face, plane, &[]), "a NURBS edge is not placed");
    }

    /// An arc's parents count as distinct from a face's carrier only on
    /// a definite ladder `Distinct`. The sheet's bottom arc lies on a
    /// second unit sheet's cylinder, its own parent's carrier: undeclared,
    /// that is no decision. Against a radius-2 sheet it is `Distinct`.
    /// (Through the boolean, an undeclared same-carrier pair refuses as a
    /// continuation before the crossing layer, so only this row holds the
    /// guard.)
    #[test]
    fn only_a_decided_distinct_parent_licenses_the_arc() {
        let (x, _) = sheet(FRAC_PI_2);
        let bottom = edge_between(
            &x,
            vertex_at(&x, Point3::new(1.0, 0.0, 0.0)),
            vertex_at(&x, Point3::new(0.0, 1.0, 0.0)),
        );
        let edge = x.get_edge(bottom).unwrap();
        let (same, same_face) = sheet(FRAC_PI_2);
        assert!(
            !parents_distinct_from(&x, edge, &same, same_face, band()),
            "one carrier, undeclared: no decision"
        );
        let mut wide = Body::new();
        let wide_face = cyl_wall_sheet(
            &mut wide,
            CylFrame::canonical(2.0),
            None,
            (0.0, FRAC_PI_2),
            (0.0, 1.0),
            Tol::witness(),
        );
        assert!(
            parents_distinct_from(&x, edge, &wide, wide_face, band()),
            "a radius-2 wall is decided distinct"
        );
    }

    /// A chain of circle arcs leaving along the tangent reaches its end
    /// only through vertices decided on the circle. The quarter sheet's
    /// bottom arc, split at 45°, leaves `(1, 0, 0)` along `+y`, as the
    /// circle of radius 2 about `(−1, 0, 0)` does; its middle vertex is
    /// off that circle, and on the sheet's own.
    #[test]
    fn a_chain_vertex_off_the_circle_breaks_the_chain() {
        let (mut y, _) = sheet(FRAC_PI_2);
        let (from, to) = (
            vertex_at(&y, Point3::new(1.0, 0.0, 0.0)),
            vertex_at(&y, Point3::new(0.0, 1.0, 0.0)),
        );
        let bottom = edge_between(&y, from, to);
        let mid = Point3::new(FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0);
        split_other_at_point(&mut y, Operand::B, bottom, mid, band(), Tol::witness())
            .expect("the arc splits");
        let reach = |circle| {
            arc_chain_reaches(&y, from, to, Vec3::unit_y(), circle, band()).expect("no escalation")
        };
        assert!(
            !reach((Point3::new(-1.0, 0.0, 0.0), Vec3::unit_z(), 2.0)),
            "the middle vertex is off the tangent circle"
        );
        assert!(
            reach((Point3::origin(), Vec3::unit_z(), 1.0)),
            "and on the sheet's own circle the chain runs through"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod clearance_rows {
    //! The conic clearance rung ([`super::conic_clearance`]) on grazes:
    //! a `Positive` there is a certified "no event" that returns before
    //! any root door is asked, so it must never stand for a carrier that
    //! comes within the band of the surface.

    use core::f64::consts::FRAC_PI_2;

    use super::conic_clearance;
    use crate::boolean::conic_oracle::{distance_from_anchor, unit};
    use geom_core::{Band, Point3, Sign, Vec3};
    use test_utils::fuzz;

    /// **The reviewer's pin: a definite crossing is not certified
    /// clear.** An ellipse stored in the ordinary order and sign (major
    /// 4.85 m, minor 0.21 m) meets a 24.7 µm ball 2.71e-11 m deep at its
    /// minor vertex (a 60-digit oracle's figure), at ε = 1e-12. There
    /// the harmonics' phases align, so `c₀ − A₁ − A₂` IS the least
    /// residual, and read bare its rounding (`≈ u·a²/r ≈ 1e-10`) put it
    /// at +2.9e-11: the rung certified the crossing clear.
    #[test]
    fn a_crossing_at_the_minor_vertex_is_not_certified_clear() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let axis = Vec3::new(
            -0.962_120_141_566_121_1,
            -0.090_956_895_968_249_41,
            0.257_005_206_695_522_7,
        );
        let e = geom::Curve3::Ellipse {
            center: Point3::new(
                -0.480_548_118_168_072_4,
                -0.441_016_920_669_073_5,
                -0.888_958_285_986_655_8,
            ),
            axis,
            major: 4.846_323_757_498_574,
            minor: 0.207_528_397_062_063_02,
            u_ref: Vec3::new(
                0.087_713_558_478_483_3,
                0.789_303_434_737_465_2,
                0.607_705_865_999_894_6,
            ),
        };
        let s = geom::Surface::Sphere {
            center: Point3::new(
                -0.426_972_420_046_788_4,
                -0.567_049_134_604_379_2,
                -0.732_997_401_751_036_8,
            ),
            radius: 2.466_073_624_731_285_6e-5,
            axis,
            u_ref: Vec3::new(
                0.272_625_811_677_447_04,
                -0.320_994_777_005_651_3,
                0.906_993_671_390_437_7,
            ),
        };
        let conic = geom_brep::Conic::of(&e).unwrap();
        let v = 3.0 * FRAC_PI_2;
        let got = conic_clearance(&s, &conic, (v - 0.5, v + 0.5), band);
        assert!(
            !matches!(got, Some(Ok(Sign::Positive))),
            "a crossing 2.71e-11 m deep certified clear: {got:?}"
        );
    }

    /// **A short arc a thousand kilometres out is charged its samples'
    /// rounding.** The third delta review's pin: an ellipse stored with
    /// `major = −0.556`, `minor = 1.02`, centred 1000 km out, an arc of
    /// ±3e-4 rad about its vertex, against a 0.118 m ball, at ε = 1e-12.
    /// The ball was placed 2.1e-11 m off by its nominal, but its stored
    /// centre rounds at that distance's ulp, and the true least distance
    /// of the STORED geometry is −5.3e-12 m — in band. On an arc that
    /// short the chord-dip charge is negligible, so the arc enclosure's
    /// margin is its samples' own: read without their rounding charge
    /// (`geom_brep::conic_arc_residual_range`'s `sample_rounding`), the
    /// rung certified the touch clear.
    #[test]
    fn a_short_arc_far_out_is_charged_its_samples_rounding() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let axis = Vec3::new(
            -0.331_547_126_719_510_5,
            -0.144_826_239_786_666_6,
            0.932_256_329_039_010_6,
        );
        let center = Point3::new(
            -793_186.577_792_401_4,
            241_909.440_601_441_78,
            -783_085.727_290_743_7,
        );
        let (major, minor) = (-0.556_277_953_009_113_9, 1.019_942_477_711_699_2);
        let u_ref = Vec3::new(
            -0.346_444_852_811_330_7,
            -0.900_421_850_752_487_9,
            -0.263_090_202_493_364_96,
        );
        let e = geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        };
        let x = Vec3::new(1.0, 0.0, 0.0);
        let s = geom::Surface::Sphere {
            center: Point3::new(
                -793_187.576_342_526_2,
                241_909.907_376_568_15,
                -783_086.009_900_933_8,
            ),
            radius: 0.117_972_233_474_760_74,
            axis,
            u_ref: (x - axis * x.dot(axis)).normalize(),
        };
        let vertex: f64 = 4.712_388_980_384_69;
        let (sv, cv) = vertex.sin_cos();
        let least = distance_from_anchor(&s, |anchor| {
            (center - anchor) + u_ref * (major * cv) + axis.cross(u_ref) * (minor * sv)
        });
        assert!(
            least.abs() <= 1e-11,
            "the pose is a touch in band: {least:e}"
        );
        let conic = geom_brep::Conic::of(&e).unwrap();
        let got = conic_clearance(
            &s,
            &conic,
            (4.712_088_980_384_689_5, 4.712_688_980_384_69),
            band,
        );
        assert!(
            !matches!(got, Some(Ok(Sign::Positive))),
            "a touch {least:e} m off certified clear: {got:?}"
        );
    }

    /// **No certified clearance on a graze, through the rung.** Circles,
    /// and ellipses in all eight stored orders and signs (eccentricity
    /// up to 40), metre- or kilometre-sized, centred up to a metre, a
    /// kilometre or a thousand kilometres out (where a sampled residual's
    /// coordinate rounding, `u·|p|`, passes the band at 1e-12), each meeting a sphere or a wall (radius 1 µm to 1 m, the
    /// wall's axis across the outward normal) at a vertex, set off along
    /// the outward normal by `gap`, −40 to 40 bands. The vertex is the
    /// carrier's least distance from the surface (the carrier bends away
    /// from it), read from the stored surface's anchor: the pose crosses
    /// when that is below `−ε` and touches in band when it is within
    /// `ε`; a certified
    /// `Positive` is wrong in both. A third of the draws put a torus's
    /// tube there instead, its spine bending away from the carrier, under
    /// the same oracle. Counts printed.
    #[test]
    fn no_certified_clearance_on_a_graze() {
        let mut rng = fuzz::start("reduce::no_certified_clearance_on_a_graze");
        for eps in [1e-12, 1e-9, 1e-6] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let (mut clear, mut refused, mut torus_clear) = (0, 0, 0);
            for i in 0..fuzz::scaled(3000) {
                let n = unit(&mut rng);
                let u = unit(&mut rng);
                let u_ref = (u - n * u.dot(n)).normalize();
                let scale = if rng.below(2) == 0 { 1.0 } else { 1000.0 };
                let big = scale * rng.range(0.5, 5.0);
                let circle = i % 9 == 8;
                let combo = i % 8;
                let small = if circle {
                    big
                } else {
                    big / rng.range(1.0, 40.0)
                };
                let (mut major, mut minor) = if combo & 1 == 0 {
                    (big, small)
                } else {
                    (small, big)
                };
                if !circle && combo & 2 != 0 {
                    major = -major;
                }
                if !circle && combo & 4 != 0 {
                    minor = -minor;
                }
                let far = [1.0, 1e3, 1e6][rng.below(3)];
                let center = Point3::new(
                    rng.range(-far, far),
                    rng.range(-far, far),
                    rng.range(-far, far),
                );
                let e = if circle {
                    geom::Curve3::Circle {
                        center,
                        axis: n,
                        radius: major,
                        u_ref,
                    }
                } else {
                    geom::Curve3::Ellipse {
                        center,
                        axis: n,
                        major,
                        minor,
                        u_ref,
                    }
                };
                let conic = geom_brep::Conic::of(&e).unwrap();
                let vertex = FRAC_PI_2 * f64::from(u32::try_from(rng.below(4)).unwrap());
                let p = e.eval(vertex);
                let outward = (p - center).normalize();
                let gap = eps * rng.range(-40.0, 40.0);
                let r = 10f64.powf(rng.range(-6.0, 0.0));
                let hub = p + outward * (r + gap);
                let x = Vec3::new(1.0, 0.0, 0.0);
                let kind = rng.below(3);
                let s = match kind {
                    0 => geom::Surface::Sphere {
                        center: hub,
                        radius: r,
                        axis: n,
                        u_ref: (x - n * x.dot(n)).normalize(),
                    },
                    1 => {
                        let v = unit(&mut rng);
                        let w = (v - outward * v.dot(outward)).normalize();
                        geom::Surface::Cylinder {
                            origin: hub,
                            axis: w,
                            radius: r,
                            u_ref: outward,
                        }
                    }
                    _ => {
                        let v = unit(&mut rng);
                        let w = (v - outward * v.dot(outward)).normalize();
                        let ring = r * rng.range(1.5, 10.0);
                        let axis = outward.cross(w).normalize();
                        geom::Surface::Torus {
                            center: hub + outward * ring,
                            axis,
                            major_radius: ring,
                            minor_radius: r,
                            u_ref: (x - axis * x.dot(axis)).normalize(),
                        }
                    }
                };
                let got = conic_clearance(&s, &conic, (vertex - 0.5, vertex + 0.5), band);
                let label = || {
                    format!(
                        "ε {eps}, case {i}: gap {gap:e}, {e:?} against {s:?} — {}",
                        fuzz::replay()
                    )
                };
                let certified = matches!(got, Some(Ok(Sign::Positive)));
                // The least distance, at the vertex, of the STORED
                // geometry: the surface's anchor was itself rounded where
                // it was placed (about `u·|p|`, past the band a hundred
                // kilometres out), so the oracle reads the vertex from
                // that anchor — `C₀ − anchor` is exact, the two being
                // metres apart, and the rest is metre-sized.
                let (sv, cv) = vertex.sin_cos();
                let from = |anchor: Point3<f64>| {
                    (center - anchor) + u_ref * (major * cv) + n.cross(u_ref) * (minor * sv)
                };
                let least = distance_from_anchor(&s, from);
                if certified {
                    if kind == 2 {
                        torus_clear += 1;
                    } else {
                        clear += 1;
                    }
                    assert!(least > eps, "{}: certified clear", label());
                } else {
                    refused += 1;
                }
            }
            println!(
                "ε {eps}: {clear} sphere/wall and {torus_clear} torus certified clear, {refused} not"
            );
        }
    }
}

/// **The operand gate answers a broken body as broken.** A tier-1
/// finding is not scaffolding an edit left behind, so it refuses as
/// [`BooleanError::CorruptOperand`] carrying tier 1's findings, never
/// as [`BooleanError::ScaffoldingOperand`]. No public door tears a
/// body, so the row tears one in-crate.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod operand_gate_rows {
    use crate::boolean::{BooleanError, BooleanOp, Corruption, Operand, boolean_reduce};
    use crate::test_support_fixtures::brick;
    use geom_core::Tol;

    #[test]
    fn a_tier_one_broken_operand_refuses_as_corrupt() {
        let tol = Tol::witness();
        let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let mut b = brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol);
        let vertex = b.vertices().next().expect("a vertex").0;
        b.vertex_provenance.remove(vertex);
        let want = crate::validate::validate(&b).expect_err("the tear breaks tier 1");
        let got = boolean_reduce(BooleanOp::Union, &a, &b, tol);
        let Err(BooleanError::CorruptOperand {
            operand,
            corruption: Corruption::Structure { errors },
        }) = got
        else {
            panic!("want CorruptOperand with tier 1's findings, got {got:?}");
        };
        assert_eq!(operand, Operand::B, "the refusal names the torn operand");
        assert_eq!(errors, want, "the payload is tier 1's own verdict");
    }
}
