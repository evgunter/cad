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
//!   order, faces in arena snapshot order, worklist FIFO.

use geom_core::{Band, Bounds, Decide, Margin, Point3, Sign};

use super::boxes;
use super::contain::{ContainError, CurvedPlacement, FaceContainment, contfp};
use super::plane_eq::{LadderRefusal, PlaneDesc};
use super::refusal_routes::NeighbourOffset;
use super::{BooleanDecision, Coincide, CrossingDecision, DeclarationRead};
use super::{BooleanError, ContactRecords, Operand, VfContact, VvContact};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::splitting::ConicPlaneMeet;
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
    pub kind: geom_brep::SurfaceKind,
    /// The other operand's face whose box overlaps it.
    pub other_face: FaceKey,
    /// That face's kind — the other half of the germ pair.
    pub other_kind: geom_brep::SurfaceKind,
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
        let mut offenders: Vec<(FaceKey, geom_brep::SurfaceKind)> = Vec::new();
        for (key, f) in body.faces() {
            let s = surface_of(body, f)?;
            if !supported(s) {
                offenders.push((key, geom_brep::SurfaceKind::of(s)));
            }
        }
        if offenders.is_empty() {
            continue;
        }
        // The other side's boxes are built ONCE, and only now that an
        // offender exists: the scan is `offenders × other faces`, so
        // re-boxing per offender would re-walk a whole body per cone.
        let others: Vec<(FaceKey, geom_brep::SurfaceKind, bvh::Aabb)> = other
            .faces()
            .map(|(key, f)| {
                let kind = geom_brep::SurfaceKind::of(surface_of(other, f)?);
                Ok((key, kind, super::boxes::face_box(other, key, pad)?))
            })
            .collect::<Result<_, BooleanError>>()?;
        for (face, kind) in offenders {
            let boxed = super::boxes::face_box(body, face, pad)?;
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
/// Two rules, and they have different scopes on purpose:
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
/// [`BooleanError::CurvedPairUnsupported`] for a germ pair with no
/// arm; [`BooleanError::CurvedEdgeUnsupported`] /
/// [`BooleanError::ScaffoldingOperand`] per operand;
/// [`BooleanError::CurvedBooleanUnsupported`] for a face whose
/// surface key does not resolve.
pub(super) fn gate_operand_pairs<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    declared: &super::DeclaredPairs,
    band: Band,
) -> Result<(), BooleanError> {
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        gate_operand_edges(body, operand)?;
    }
    // **`class_of`, not `declares_rest` — and the safety of that is an
    // ORDERING fact, so it is stated here rather than left to be
    // rediscovered.** This predicate is class-BLIND by construction: any
    // declared class covers the pair. That would be wrong on its own,
    // because a `Tangent` claim licenses no same-carrier treatment and
    // the class-aware twin (`DeclaredPairs::declares_rest`) sits in
    // `mod`. What makes it right is that `verify_declared_contacts` runs
    // BEFORE this gate and has already refused every declaration it
    // cannot verify — for a curved pair outside the DEV-1 witness lane
    // that is every `Tangent` claim. So a `Tangent` pair of a kind this
    // roster refuses never survives to be covered here.
    //
    // A torus pair still reaches this predicate — a cone offender
    // against a torus face is asked whether that pair is covered — and
    // it is safe there for the same reason: a `Tangent` claim on it is
    // refused by `verify_tangent_declaration` before the gate (the
    // witness lane has no torus arm; the conformal screen contradicts
    // one-carrier pairs). Read the dependency the other way and it is a warning: whoever
    // teaches the `Tangent` door to ADMIT a curved pair of an off-roster
    // kind must revisit this predicate in the same change, because the
    // class-blindness is load-bearing only while that door refuses.
    if let Some(p) = first_unsupported_pair(a, b, band, boolean_arm_exists, |operand, f, other| {
        declared
            .class_of(operand, f, operand.other(), other)
            .is_some()
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
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
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
            },
            _ => {
                return Err(BooleanError::ScaffoldingOperand {
                    operand,
                    edge: edge_key,
                });
            }
        }
    }
    Ok(())
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
    for (edge_key, edge) in body.edges() {
        let (Some(f1), Some(f2)) = (
            body.face_of_half_edge(edge.he_plus),
            body.face_of_half_edge(edge.he_minus),
        ) else {
            continue;
        };
        if f1 == f2 {
            continue; // seam/strut inside one face: not a coplanar PAIR
        }
        let (k1, k2) = (
            body.get_face(f1).map(|f| f.surface),
            body.get_face(f2).map(|f| f.surface),
        );
        if k1.is_some() && k1 == k2 {
            // Same-key CURVED adjacency is the CANONICAL maximal form
            // (M5 PR 9, C12.5): a periodic wall cannot be one face
            // without its parameterization cut, so two half-walls
            // sharing one cylinder key across a meridian strut are
            // exactly what a maximal-faced curved operand looks like
            // (the cosurface merge itself KEEPS such a cut). Only the
            // PLANAR same-key pair is the F7 defect.
            let planar = k1
                .and_then(|k| body.get_surface(k))
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
        match super::plane_eq::plane_eq_typed(&p1, &p2, id, arm, band) {
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
        }
    }
    Ok(())
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
            boxes::face_box(y, f, pad)?
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
    declared: &super::DeclaredPairs,
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
pub(super) fn sweep_direction<T: Decide + Bounds>(
    x: &mut Body<T>,
    y: &mut Body<T>,
    x_is: Operand,
    declared: &super::DeclaredPairs,
    contacts: &mut ContactAcc,
    band: Band,
    strategy: SweepStrategy,
    knobs: &SweepKnobs,
    mut trace: Option<&mut SweepTrace>,
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
        SweepStrategy::Realized => Some(face_tree(y, &faces, knobs, pad)?),
        SweepStrategy::Idealized => None,
    };
    #[cfg(not(feature = "sweep-testing"))]
    let tree: bvh::Bvh = {
        let SweepStrategy::Realized = strategy;
        face_tree(y, &faces, knobs, pad)?
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
            let vert = |he| -> Option<(VertexKey, Point3<T>)> {
                let vk = x.get_half_edge(he)?.start;
                Some((vk, *x.get_point(x.get_vertex(vk)?.point)?))
            };
            let ((u, pu), (v, pv)) = match (vert(edge.he_plus), vert(edge.he_minus)) {
                (Some(a), Some(b)) => (a, b),
                _ => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "edge endpoints unresolvable",
                    });
                }
            };
            // Per-kind face dispatch (M5 PR 9, C12.1): planar faces run
            // the M3 lane below (bit-identically for line edges, plus
            // the conic ROOT lane); curved faces get the clearance /
            // typed-frontier arm.
            let Some(plane) = face_plane(y, face) else {
                let event = curved_face_arm(
                    x, y, x_is, edge_key, &edge, u, v, face, pu, pv, declared, contacts, band, tol,
                )?;
                if !matches!(event, CurvedEvent::None)
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
                let CurvedEvent::Pierce { t, p, at } = event else {
                    continue;
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
            {
                let curve = match x.get_curve_geom(edge.curve) {
                    Some(CurveGeom::Certified(c)) => c.clone(),
                    _ => {
                        return Err(BooleanError::ScaffoldingOperand {
                            operand: x_is,
                            edge: edge_key,
                        });
                    }
                };
                let (t0, t1) = curve.params();
                match crate::splitting::conic_plane_crossing_roots(
                    curve.carrier(),
                    t0,
                    t1,
                    plane.origin,
                    plane.normal,
                    band,
                ) {
                    Err(()) => {} // a line: the M3 lane below owns it
                    Ok(ConicPlaneMeet::Miss) => continue,
                    // The conic's plane is parallel to the face's: off
                    // it, a miss; in it, the line lane's `(Zero, Zero)`
                    // posture — both endpoints through
                    // `vertex_on_face`, the interior left to the
                    // neighbour faces.
                    Ok(ConicPlaneMeet::Parallel { offset }) => {
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
                    Ok(ConicPlaneMeet::Roots(Err(fault))) => {
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
                    Ok(ConicPlaneMeet::Roots(Ok(roots))) => {
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
                    let curve = match x.get_curve_geom(edge.curve) {
                        Some(CurveGeom::Certified(c)) => c.clone(),
                        _ => {
                            return Err(BooleanError::ScaffoldingOperand {
                                operand: x_is,
                                edge: edge_key,
                            });
                        }
                    };
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

/// The curved-face sweep arm: endpoint sides come from the linearized
/// implicit residual; a definite miss is PROVEN — for a LINE carrier
/// against a cylinder or sphere the residual is convex (both-inside
/// means no wall crossing, both-outside clears through the span
/// minimum), against a torus the certified quartic's roots decide, and
/// for a CIRCLE
/// carrier the ARC's residual range is enclosed two ways (the
/// carrier's exact harmonic bounds and the arc's own chord-dip
/// bound), so a definitely one-sided arc clears. What definitely MEETS
/// the face is split by kind, and the third paragraph below is the
/// statement of record: a LINE carrier against a CYLINDER wall, a
/// SPHERE or a TORUS, and a CIRCLE carrier against a SPHERE or a TORUS,
/// are routed through the certified roots and pierce; everything else —
/// a tangency, a circle against a cylinder, an undeclared
/// on-carrier edge, a trim with no verdict — refuses typed at the named
/// frontier door ([`BooleanError::CurvedPierceUnsupported`]). An
/// in-band clearance escalates (F6, the same margin's other half) —
/// except a circle's against a sphere or a torus, where the certified
/// roots decide what the enclosures could not.
/// Ellipse/NURBS carriers keep the unconditional M5 door. Never a
/// silent fallback.
///
/// **The carrier-identity rung** comes before any enclosure on a
/// CIRCLE carrier: an edge whose parent face is `Rest`-declared against
/// `face`, on a carrier the ladder calls the same, lies on `face`'s
/// carrier, so its clearance is zero by that certificate
/// ([`on_declared_rest_carrier`]). The sampled enclosure cannot say so
/// of a coincident pair — it is `±charge` about an identically-zero
/// residual and reads definitely negative.
///
/// **The declared-cover rung** (CONTACT-DESIGN C8 at the crossing
/// layer): a zero-clearance incidence whose edge has a parent face
/// DECLARED against `face` — `Rest` (the edge bounds a face on the
/// verified shared carrier, so it lies ON that carrier) or `Tangent`
/// (the on-carrier locus IS the verified tangency: the ruling a
/// tangent edge realizes) — takes the planar sweep's endpoint
/// posture instead of the frontier door: each on-carrier endpoint is
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
/// holds, and the sweep reaches that pair on its own visit. A pair with
/// NO recorded endpoint still refuses — an overlap lying wholly inside
/// this face's window, with both endpoints beyond it, is an incidence
/// this arm cannot see, and it stays loud rather than becoming a silent
/// no-event.
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
/// certify, is not a crossing at any order this lane sees), a CIRCLE
/// carrier against a cylinder wall (a degree-2 trigonometric residual
/// with no root lane in this tree), and a trim the chart door declines
/// to express.
///
/// **A CIRCLE against a SPHERE or a TORUS takes the same arms as a
/// line.** Against a sphere its residual is a first harmonic with
/// closed-form roots ([`super::circle_sphere`]); against a torus it is
/// a degree-2 trigonometric polynomial, a quartic in the tangent
/// half-angle, and the ray lane's certified ladder answers it
/// ([`super::circle_torus`]). It reaches those arms only from the
/// circle rung, after the enclosures failed to clear the arc, and never
/// through a declared-cover arm — those rest on a line's separation
/// story.
///
/// **What a successful wall pierce reaches next is a typed door, not
/// a body**: a ring minted in a cylinder face has no join arm (#1291),
/// so it lands on `SplitJoinError::SectionArcWindow{NoChartedRun}`. A
/// planar cap pierce joins.
///
/// **This lane WIDENS what an undeclared pair reaches, and the widening
/// is named here rather than left to be discovered.** Before it,
/// [`vertex_on_curved_face`] was reachable only behind the declared
/// cover; the new endpoint arms (`(Zero, definite)` and the
/// `(Zero, Zero)` chord) call it on UNDECLARED pairs too. That is not
/// the C8 gate reopening: C8 protects the claim that an on-carrier EDGE
/// is cosurface, and the arms below reach the endpoint treatment only
/// after the certified roots have proved there is no interior crossing
/// — which, for the `(Zero, Zero)` arm, takes distinct certified roots
/// and so structurally excludes an edge lying on the carrier (on a
/// cylinder only rulings do, and a ruling answers `Constant`; no line
/// lies on a torus). What the door then does is
/// point-in-face containment on a chart, which is a trim question and
/// not a gluing one.
///
/// Returns what the caller must do about the pair — see
/// [`CurvedEvent`]. The split itself needs `&mut x` and the worklist,
/// both of which live in [`sweep_direction`], so the crossing is
/// REPORTED here and performed there rather than the body being
/// threaded in for one branch.
#[allow(clippy::too_many_arguments)]
pub(super) fn curved_face_arm<T: Decide>(
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
    declared: &super::DeclaredPairs,
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
    // The declared cover (docs above): one of the edge's parent faces
    // is declared against `face` under a class the door VERIFIED —
    // the on-carrier claim the numeric rows then certify per
    // incidence. `cover` is what the arm read of that declaration: a
    // declared pair spends it on every question below, and none of them
    // is one a declaration settles.
    let cover = edge_face_read(x, x_is, edge, face, declared, Coincide::VertexOnCoveredFace);
    let covered = matches!(cover, DeclarationRead::Spent(_));
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
        geom::Surface::Nurbs(_) => Some(geom_brep::SurfaceKind::Nurbs),
        geom::Surface::Approx(_) => Some(geom_brep::SurfaceKind::Approx),
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
    let curve = match x.get_curve_geom(edge.curve) {
        Some(CurveGeom::Certified(c)) => c.clone(),
        _ => {
            return Err(BooleanError::ScaffoldingOperand {
                operand: x_is,
                edge: edge_key,
            });
        }
    };
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
    // arm, definite ones included). Ellipse/NURBS carriers keep the M5
    // unconditional door.
    match *curve.carrier() {
        geom::Curve3::Line { .. } => {}
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            // **The carrier-identity rung, consulted FIRST.** An edge
            // bounding a face whose carrier the door verified to BE
            // `face`'s carrier lies on `face`'s carrier identically, so
            // its clearance is zero by that certificate — not by an
            // enclosure. The enclosures below cannot say so: the
            // sampled one is `±charge` about an identically-zero
            // residual and reads definitely negative by construction,
            // and the harmonic one has no torus form. Reading the
            // certificate first is what lets a coincident pair reach
            // the declared-cover rung at all.
            let clearance = if on_declared_rest_carrier(x, x_is, edge, face, declared) {
                Ok(Sign::Zero)
            } else {
                circle_clearance(&surface, &curve, center, axis, radius, u_ref, band)
                    .ok_or_else(frontier)?
            };
            match clearance {
                Ok(Sign::Positive) => return Ok(CurvedEvent::None),
                // The declared-cover rung: a covered zero-clearance
                // circle takes the planar sweep's endpoint posture —
                // each endpoint's own side decides its treatment
                // (existing row): ON the carrier ⇒ boundary
                // containment (which must decide, or the frontier
                // stands); definitely clear ⇒ no event at that end (a
                // TANGENT-covered circle touches the carrier at one
                // point — a clear endpoint is honestly eventless);
                // definitely inside ⇒ a crossing, never the covered
                // posture. An interior-only touch (no endpoint on the
                // carrier) keeps the frontier door. Uncovered keeps
                // both doors verbatim.
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
                // them is the **nothing-recorded guard** below: a pair
                // whose every on-carrier endpoint came back `Elsewhere`
                // records nothing and keeps the frontier, whatever the
                // source. So an all-`Out` pair is exactly as loud as it
                // was, and only a pair that ALREADY placed an endpoint
                // on this face — i.e. one with a real, recorded
                // incidence here — is allowed to stop treating its
                // other end's absence as a failure.
                //
                // Still keeping the door, unchanged: a no-verdict
                // endpoint ([`Placement::Undecided`]), and the pair
                // where nothing was recorded — an arc whose interior
                // crosses this face's window with both endpoints
                // outside it is a real incidence this arm cannot see,
                // and it must stay loud rather than become a silent
                // no-event.
                Ok(Sign::Zero) if covered => {
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
                            // Definitely clear at this end: honestly
                            // eventless, and not an endpoint the rule
                            // below weighs either way.
                            Sign::Positive => {}
                            Sign::Negative => return Err(frontier()),
                        }
                    }
                    return if Placement::records_the_pair(ends) {
                        Ok(CurvedEvent::Recorded)
                    } else {
                        Err(frontier())
                    };
                }
                // **The circle × sphere and circle × torus root lanes.**
                // An arc the enclosures could not clear against a SPHERE
                // or a TORUS takes the same endpoint-sign arms as a line
                // below, and the certified roots of
                // [`super::circle_sphere`] / [`super::circle_torus`]
                // decide every one of them through [`wall_crossing`].
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
                // declared-cover arms below rest on a line's separation
                // story, so a covered circle — whose covered-Zero case
                // returned above — keeps the frontier door here rather
                // than reaching them. That makes those arms structurally
                // line-only, which they assert.
                Ok(Sign::Zero | Sign::Negative) | Err(_)
                    if !covered
                        && matches!(
                            surface,
                            geom::Surface::Torus { .. } | geom::Surface::Sphere { .. }
                        ) => {}
                Ok(Sign::Zero | Sign::Negative) => return Err(frontier()),
                // A declared pair's declaration is spent here. An
                // undeclared one, declared `Rest` on this face's
                // carrier, is on it by the carrier-identity rung above,
                // and takes the covered arm, whose endpoint sides then
                // refuse the same in-band pose: no declaration settles
                // it. Covered, zero passes (the covered arm); uncovered,
                // only a clear arc does.
                Err(diag) => {
                    let which = if covered {
                        Coincide::ArcOnCoveredFace
                    } else {
                        Coincide::ArcClearsCurvedFace
                    };
                    return Err(BooleanError::coincidence(which, read(which), diag));
                }
            }
        }
        _ => return Err(frontier()),
    }
    let side = |p: Point3<T>| {
        decide(
            "bool_vertex_face_side",
            Margin::of(geom_brep::implicit_residual(&surface, p)),
            band,
        )
    };
    // The declared-cover arms rest on a LINE's separation story; only an
    // uncovered circle reaches the endpoint arms (the circle rung above).
    let on_line = matches!(curve.carrier(), geom::Curve3::Line { .. });
    let on_face = |diag| {
        let which = Coincide::VertexOnCurvedFace;
        BooleanError::coincidence(which, read(which), diag)
    };
    let s1 = side(pu).map_err(on_face)?;
    let s2 = side(pv).map_err(on_face)?;
    match (s1, s2) {
        // The declared-cover rung: a covered line with endpoint(s) ON
        // the carrier takes the planar sweep's endpoint posture — the
        // `(za, zb)` branch mirrored: each Zero endpoint gets boundary
        // containment (which must decide, or the frontier stands).
        // What makes the `(Zero, Positive)` branch eventless is NOT
        // convexity (a convex residual's endpoint bound is its
        // MAXIMUM, not its minimum — q(0) = 0, q(1) > 0 can dip
        // negative between): it is the witness lane's SEPARATION
        // INVARIANT — every pair `tangent_locus` admits has each
        // carrier wholly in ONE closed residual half-space of the
        // other (the contract sentence on [`geom_brep::tangent_locus`];
        // a `Rest` cover's shared carrier is residual-zero
        // identically) — so a covered on-carrier edge's residual is
        // one-signed and a Zero endpoint is a touch, never an entry.
        // A configuration without that one-sign story must not be
        // admitted to the lane. **The coaxial cylinder×sphere circle
        // arm is no longer an EXAMPLE of one, and this citation is
        // corrected rather than left standing**: #974 recorded that arm
        // as blocked on the one-sign story, and VERBS-CYLSPH MEASURED
        // the story and it PASSES — at the only coaxial tangency the
        // sphere lies wholly in the cylinder's closed inside and the
        // cylinder wholly outside the sphere. The orientations are
        // OPPOSITE per direction, which this contract never forbade:
        // the internally tangent parallel cylinder pair the lane
        // already admits has exactly that structure
        // (`crates/topo/tests/verbs_cylsph_tangent_residuals.rs` pins
        // both halves). What keeps that arm out is downstream and
        // structural, and is stated at [`geom_brep::tangent_locus`]
        // itself: `TangentLocus` carries a LINE only and no consumer of
        // it has a circle story. The RULE this comment states is
        // unchanged; only the example it reached for was superseded. A
        // NEGATIVE partner is a genuine crossing — never the covered
        // posture. Uncovered keeps both frontier doors verbatim.
        (Sign::Zero, Sign::Zero) if covered => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let hu = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
            let hv = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
            // An endpoint the containment door cannot decide keeps the
            // frontier door; an endpoint it places definitely OUT of
            // this face is eventless here (the circle rung above
            // carries the argument), and a pair with nothing recorded
            // keeps the door.
            if Placement::records_the_pair([Some(hu), Some(hv)]) {
                Ok(CurvedEvent::Recorded)
            } else {
                Err(frontier())
            }
        }
        (Sign::Zero, Sign::Positive) if covered => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let h = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
            if Placement::records_the_pair([Some(h), None]) {
                Ok(CurvedEvent::Recorded)
            } else {
                Err(frontier())
            }
        }
        (Sign::Positive, Sign::Zero) if covered => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let h = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
            if Placement::records_the_pair([Some(h), None]) {
                Ok(CurvedEvent::Recorded)
            } else {
                Err(frontier())
            }
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
                SpanVerdict::Constant | SpanVerdict::Miss | SpanVerdict::Unsettled => {
                    Err(frontier())
                }
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
        // the carrier. Two invariants carry it:
        //
        // - **It is not an on-carrier edge.** `NoInterior` is reached only
        //   through a certified root count with distinct roots. For a
        //   LINE: the lines that lie on a wall are its rulings, which
        //   answer `Constant`, and no line lies on a sphere or a torus.
        //   For a CIRCLE against a sphere: a circle lying on it is
        //   centred on its axis, answered `Constant`. For a CIRCLE
        //   against a torus: a circle lying on it is either coaxial (a
        //   rim or latitude circle, answered `Constant`) or has `F ≡ 0`,
        //   whose pole no anchor can put definitely off the torus
        //   (`Unsettled`). So the undeclared cosurface question
        //   (CONTACT-DESIGN C2/C4) keeps its door, and so does a
        //   tangency, a trim with no verdict, and every other answer.
        // - **Its interior meets this face nowhere.** The face lies on its
        //   carrier; the edge meets the carrier only at its certified
        //   roots; each root strictly inside the span was placed outside
        //   the trim, and each root at an end is that end's own incidence.
        //
        // So the ends decide, under the same rule as the mixed-sign arm
        // ([`Placement::undeclared_no_interior`]).
        (Sign::Zero, Sign::Zero) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::NoInterior | SpanVerdict::Elsewhere => {
                    let hu = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
                    let hv = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
                    Placement::undeclared_no_interior([Some(hu), Some(hv)]).ok_or_else(frontier)
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
                SpanVerdict::Constant | SpanVerdict::Unsettled => Err(frontier()),
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
                        SpanVerdict::Constant => Err(frontier()),
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

/// The two enclosures of a circle ARC's residual against `surface`,
/// folded: the carrier's exact harmonic bounds and the arc's sampled
/// chord-dip range. Both enclose the arc's range, so the clearance
/// margin is the larger of the two one-sidedness margins. `None` when
/// the carrier enclosure has no form for the kind.
///
/// The line row's vertex CLAMP does not port here, and the reason is
/// the curve: along a line the residual is exactly quadratic, so "the
/// vertex is outside the span" is a statement about a parabola and is
/// decided by the endpoint gap alone. Along a circle it has up to four
/// critical parameters, so an endpoint gap says nothing about where
/// its minimum sits. Subdivision is what is available without solving
/// for them.
#[allow(clippy::too_many_arguments)]
fn circle_clearance<T: Decide>(
    surface: &geom::Surface<T>,
    curve: &geom_brep::EdgeCurve<T>,
    center: Point3<T>,
    axis: geom_core::Vec3<T>,
    radius: T,
    u_ref: geom_core::Vec3<T>,
    band: Band,
) -> Option<Result<Sign, geom_core::Indeterminate>> {
    let (lo, hi) = geom_brep::circle_residual_extremes(surface, center, axis, radius, u_ref)?;
    let carrier_margin = lo.max(-hi);
    let (t0, t1) = curve.params();
    let arc_margin =
        geom_brep::circle_arc_residual_range(surface, center, axis, radius, u_ref, t0, t1)
            .map_or(carrier_margin, |(arc_lo, arc_hi)| arc_lo.max(-arc_hi));
    Some(decide(
        "bool_circle_curved_clearance",
        Margin::of(carrier_margin.max(arc_margin)),
        band,
    ))
}

/// **The carrier-identity rung**: does one of the edge's parent faces
/// sit on `face`'s carrier by a verified `Rest` declaration?
///
/// `Rest` is the one class that licenses it, and only where the
/// declaration door's carrier ladder CALLED the pair one carrier
/// ([`super::DeclaredPairs::verified_one_carrier`], recorded once at the
/// door rather than re-derived per event). An edge bounding a face on
/// that carrier lies on it. `Tangent` licenses nothing of the kind — a
/// tangent pair shares a locus, not a carrier — and an undeclared pair
/// is never read as coincident by value (CONTACT-DESIGN C2/C4).
fn on_declared_rest_carrier<T: Decide>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::DeclaredPairs,
) -> bool {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .any(|pf| declared.verified_one_carrier(x_is, pf, x_is.other(), face))
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
    /// A certified root set (two on a cylinder or a sphere, two or four
    /// on a torus), none of them STRICTLY INSIDE the span on this face: each
    /// lies outside the span, sits at one of its ends, or lands on the
    /// carrier outside the face's trim. Distinct certified roots also
    /// certify that the edge does not LIE on the carrier (a line: not a
    /// ruling of a wall, and no line lies on a sphere or a torus; a
    /// circle on a sphere answers `Constant`, and on a torus a certified
    /// count needs a pole definitely off the torus, which a circle lying
    /// on it has nowhere), which is what separates a chord
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
    /// The line definitely misses the wall entirely.
    Miss,
    /// The roots did not settle the span and the caller keeps its own
    /// typed frontier door.
    Unsettled,
}

/// The curved-wall crossing route: solve the certified roots — a
/// line's quadratic on a cylinder wall or a sphere, its quartic on a
/// torus, a circle's closed form on a sphere and its half-angle quartic
/// on a torus — keep the roots the EDGE's
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
    let mut roots = [T::zero(); 4];
    // The carrier's metres per unit of its parameter, so that a root's
    // distance from the span's ends is metered as a length: a `Line`'s
    // parameter runs `|dir|` metres per unit, a `Circle`'s is an angle and
    // its arc length is `radius·Δθ`. The wall and sphere quadratics take
    // any non-zero `dir`; the torus quartic
    // ([`super::solid_contain::line_torus_roots`]) assumes it UNIT, the
    // `Line` carrier's convention, which nothing checks.
    let (count, metres_per_param) = match *carrier {
        geom::Curve3::Line { origin, dir } => (
            line_wall_root_count(origin, dir, (t1 - t0).abs(), surface, &mut roots, band)?,
            dir.norm(),
        ),
        // The circle × sphere first harmonic ([`super::circle_sphere`]).
        geom::Curve3::Circle { radius, .. } if matches!(surface, geom::Surface::Sphere { .. }) => {
            use super::circle_sphere::CircleSphereRoots;
            match super::circle_sphere::circle_sphere_roots(carrier, t0, t1, surface, band)? {
                CircleSphereRoots::Two(thetas) => {
                    roots[..2].copy_from_slice(&thetas);
                    (Ok(2), radius)
                }
                CircleSphereRoots::Coaxial => (Err(SpanVerdict::Constant), radius),
                CircleSphereRoots::Miss => (Err(SpanVerdict::Miss), radius),
                CircleSphereRoots::Uncertain => (Err(SpanVerdict::Unsettled), radius),
            }
        }
        // The circle × torus quartic ([`super::circle_torus`]). A circle
        // against any other kind has no root lane here, and the door
        // says so (`Uncertain`).
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => match super::circle_torus::circle_torus_roots(
            center, axis, radius, u_ref, t0, t1, surface, band,
        )
        .map_err(|diag| BooleanError::Escalated {
            decision: BooleanDecision::ArcTorusRoots,
            diag,
        })? {
            super::circle_torus::CircleTorusRoots::Certified { count, thetas } => {
                roots = thetas;
                (Ok(count), radius)
            }
            // A coaxial carrier's residual is constant: the circle rung's
            // analogue of the axis-parallel line.
            super::circle_torus::CircleTorusRoots::Coaxial => (Err(SpanVerdict::Constant), radius),
            super::circle_torus::CircleTorusRoots::Uncertain => {
                (Err(SpanVerdict::Unsettled), radius)
            }
            super::circle_torus::CircleTorusRoots::Miss => (Err(SpanVerdict::Miss), radius),
            super::circle_torus::CircleTorusRoots::CountDisagrees => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "the constructed roots of a quartic disagree in number with its \
                           certified count",
                });
            }
        },
        _ => return Ok(SpanVerdict::Unsettled),
    };
    let count = match count {
        Ok(count) => count,
        Err(verdict) => return Ok(verdict),
    };
    let ts = &roots[..count];
    // Whether some root sits at an end of the span, and whether some
    // root strictly inside it was placed outside this face's trim: the
    // two facts that tell [`SpanVerdict::Elsewhere`] from
    // [`SpanVerdict::NoInterior`].
    let mut at_end = false;
    let mut crossed_elsewhere = false;
    for &t in ts {
        // Each gap is a length: `metres_per_param` turns a carrier
        // parameter difference into arc length.
        let mut interior = true;
        for gap in [t - t0, t1 - t] {
            match decide(
                "bool_wall_root_in_span",
                Margin::of(gap * metres_per_param),
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
        // remainder (a ringed face, a non-iso boundary, a full-period
        // window outside the band class) and keeps the caller's
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

/// The certified LINE × wall roots, per kind, written into `roots`:
/// `Ok(count)` for a certified root set, `Err(verdict)` for the answers
/// that are not one. `span` is the run of the line's parameter the
/// edge covers, the lever the wall's axis-parallel rung is metered over.
fn line_wall_root_count<T: Decide>(
    origin: Point3<T>,
    dir: geom_core::Vec3<T>,
    span: T,
    surface: &geom::Surface<T>,
    roots: &mut [T; 4],
    band: Band,
) -> Result<Result<usize, SpanVerdict<T>>, BooleanError> {
    // The certified roots, per kind. Every lane answers the same three
    // ways — a certified root set, a definite miss, or no certain
    // count — and the cylinder adds a fourth, the axis-parallel line
    // whose residual is constant. A line never lies on a torus or a
    // sphere, so neither has such a case.
    Ok(match *surface {
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
            super::solid_contain::WallRoots::Two(ts) => {
                roots[..2].copy_from_slice(&ts);
                Ok(2)
            }
            // A tangency is not a crossing this lane can act on: the
            // material verdicts behind a pierce are first-order, and
            // along a tangency every first-order datum ties. It keeps
            // the door.
            super::solid_contain::WallRoots::Tangent => return Ok(Err(SpanVerdict::Unsettled)),
            // A constant residual, or no root on the infinite line at all.
            super::solid_contain::WallRoots::AxisParallel => return Ok(Err(SpanVerdict::Constant)),
            super::solid_contain::WallRoots::Miss => return Ok(Err(SpanVerdict::Miss)),
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
        } => match super::solid_contain::line_torus_roots(
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
        })? {
            super::solid_contain::TorusRoots::Certified { count, ts } => {
                *roots = ts;
                Ok(count)
            }
            super::solid_contain::TorusRoots::Uncertain => return Ok(Err(SpanVerdict::Unsettled)),
            super::solid_contain::TorusRoots::Miss => return Ok(Err(SpanVerdict::Miss)),
            super::solid_contain::TorusRoots::CountDisagrees => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "the constructed roots of a quartic disagree in number with its \
                           certified count",
                });
            }
        },
        // The quadratic, the ray lane's own. No line lies on a sphere,
        // so it has no constant case either.
        geom::Surface::Sphere { center, radius, .. } => {
            match super::solid_contain::line_sphere_roots(origin, dir, center, radius, band)
                .map_err(|diag| BooleanError::Escalated {
                    decision: BooleanDecision::SphereRoots,
                    diag,
                })? {
                super::solid_contain::WallRoots::Two(ts) => {
                    roots[..2].copy_from_slice(&ts);
                    Ok(2)
                }
                super::solid_contain::WallRoots::Tangent => return Ok(Err(SpanVerdict::Unsettled)),
                super::solid_contain::WallRoots::AxisParallel
                | super::solid_contain::WallRoots::Miss => return Ok(Err(SpanVerdict::Miss)),
            }
        }
        _ => return Ok(Err(SpanVerdict::Unsettled)),
    })
}

/// What one edge×curved-face pair asks of the sweep.
#[derive(Debug, Clone, Copy)]
pub(super) enum CurvedEvent<T: geom_core::Real> {
    /// No event: the pair is definitely clear, or the crossing lies
    /// outside this face's trim.
    None,
    /// The arm recorded the event itself (the declared-cover rung,
    /// whose endpoint treatment mints its own contacts). Reported so
    /// the differential suite's accepted-pair channel still sees it.
    Recorded,
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
    /// **The declared rungs' one rule**, in one place because all four
    /// of them apply it: given each of an edge's endpoints as
    /// `Some(placement)` when the residual put it ON the carrier and
    /// `None` when it is honestly not this arm's business (definitely
    /// clear, or the arm has only one on-carrier end), does this pair
    /// record an event, or keep the typed frontier?
    ///
    /// Two conditions, both necessary:
    ///
    /// - **no `Undecided`** — an endpoint the containment door could
    ///   not place leaves the pair unknown, and unknown keeps the door;
    /// - **at least one `Recorded`** — a pair whose on-carrier ends all
    ///   came back `Elsewhere` has placed nothing on this face, so it
    ///   keeps the door too. That is what stops an overlap lying wholly
    ///   inside this face's window, with both ends beyond it, from
    ///   turning into a silent no-event.
    ///
    /// `Elsewhere` therefore never carries a pair on its own; it only
    /// stops being fatal beside a sibling end that WAS recorded.
    fn records_the_pair(ends: [Option<Self>; 2]) -> bool {
        !ends.iter().flatten().any(|p| *p == Self::Undecided)
            && ends.iter().flatten().any(|p| *p == Self::Recorded)
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
    /// Unlike [`Self::records_the_pair`], an all-`Elsewhere` pair is
    /// eventless rather than refused: that rule's nothing-recorded guard
    /// exists for an on-carrier edge overlapping the window, which the
    /// distinct certified roots behind `NoInterior` exclude here.
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
pub(super) fn vertex_on_curved_face<T: Decide>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<Placement, BooleanError> {
    let placement = super::contain::curved_face_placement(y, face, px, band)
        .map_err(|e| esc(e, x_is.other()))?;
    let verdict = match placement {
        CurvedPlacement::Trim(v) => v,
        CurvedPlacement::OffCarrier => None,
    };
    match verdict {
        Some(FaceContainment::OnVertex(vy)) => {
            push_vv(contacts, x_is, vx, vy);
            return Ok(Placement::Recorded);
        }
        Some(FaceContainment::OnEdge(ey)) => {
            let wy = split_other_at_point(y, x_is.other(), ey, px, band, tol)?;
            push_vv(contacts, x_is, vx, wy);
            return Ok(Placement::Recorded);
        }
        // Strictly inside the curved face's chart trim: the same
        // v-f record the planar sweep writes ([`vertex_on_face`]),
        // now that the trim can say so.
        Some(FaceContainment::In) => {
            contacts.vf(x_is, VfContact { vertex: vx, face });
            return Ok(Placement::Recorded);
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
        match decide("bool_contact_vertex", Margin::norm3(px - py), band) {
            Ok(Sign::Zero) => {
                push_vv(contacts, x_is, vx, vy);
                return Ok(Placement::Recorded);
            }
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::VertexOnVertex,
                    diag: geom_core::Indeterminate {
                        margin: geom_core::MarginDiag::INVALID,
                        band,
                        predicate: Some("bool_contact_vertex"),
                        terminal_sliver: false,
                    },
                });
            }
            Err(diag) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::VertexOnVertex,
                    diag,
                });
            }
        }
    }
    // Only an ON-carrier `Out` is a certified absence. Every caller
    // reaches this door with the endpoint's residual decided `Zero`, so
    // an off-carrier answer contradicts that decision and is not
    // evidence the incidence lives elsewhere: it keeps the door.
    Ok(match placement {
        CurvedPlacement::Trim(Some(FaceContainment::Out)) => Placement::Elsewhere,
        _ => Placement::Undecided,
    })
}

fn esc(e: ContainError, operand: Operand) -> BooleanError {
    match e {
        ContainError::Escalated(diag) => BooleanError::Escalated {
            decision: BooleanDecision::Containment,
            diag,
        },
        ContainError::RayExhausted => BooleanError::ClassificationInvariant {
            what: "contfp ray schedule exhausted",
        },
        ContainError::ArcLoopUnsupported { r#loop } => {
            BooleanError::ArcLoopContainmentUnsupported { operand, r#loop }
        }
        ContainError::Corrupt => BooleanError::CorruptOperand {
            operand,
            vertex: VertexKey::default(),
        },
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
fn vertex_on_face<T: Decide>(
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

fn split_at<T: Decide>(
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
fn split_other_at_point<T: Decide>(
    y: &mut Body<T>,
    y_is: Operand,
    edge: EdgeKey,
    p: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<VertexKey, BooleanError> {
    let curve = match y.get_edge(edge).and_then(|e| y.get_curve_geom(e.curve)) {
        Some(CurveGeom::Certified(c)) => c.clone(),
        _ => {
            return Err(BooleanError::ScaffoldingOperand {
                operand: y_is,
                edge,
            });
        }
    };
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
            Some(CurvedEvent::Pierce { .. }) => panic!("the rule never pierces"),
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
}

#[cfg(test)]
#[path = "coplanar_conic_rows.rs"]
pub(super) mod coplanar_conic_rows;

/// **A curved face's escalations read no declaration ahead of them, and
/// offer none**, on the review's executed raises (its
/// `probe_wall_roots_in_band_with_and_without_declaration`, adopted
/// here): an edge in the side face of a prism against a unit cylinder
/// wall, run through `curved_face_arm` with that side face undeclared,
/// declared `Tangent`, and declared `Rest`.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod declaration_order_rows {
    use super::{ContactAcc, CurvedEvent, curved_face_arm};
    use crate::boolean::{
        BooleanDecision, BooleanDeclarations, BooleanError, Coincide, DeclarationRead,
        DeclaredPairs, FacePairDeclaration, Operand,
    };
    use crate::contact::ContactClass;
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
        let declared = DeclaredPairs::build(&decls, Default::default());
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
                    class.map_or(DeclarationRead::Moot, DeclarationRead::Spent)
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
    fn decision_of(label: &str, got: &Result<(), BooleanError>) -> BooleanDecision {
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
    ) -> (Option<bool>, Vec<Result<(), BooleanError>>) {
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
        let declared = DeclaredPairs::build(&decls, one);
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
            out.push(
                curved_face_arm(
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
                .map(|_: CurvedEvent<f64>| ()),
            );
        }
        assert_eq!(out.len(), 2, "the sheet's two rim circles");
        (door, out)
    }

    /// **The uncovered circle's clearance offers no declaration**, on
    /// the review's executed raise: a sheet of radius `1 + mid` against
    /// the unit wall. Undeclared, the clearance escalates past the
    /// carrier-identity rung. Declared `Rest`, which the door verifies,
    /// with the door calling the two one carrier, the rung reads the
    /// clearance zero, and the covered arm's endpoint sides refuse the
    /// same in-band pose; not called one carrier, or declared `Tangent`,
    /// the covered clearance refuses. No posture passes, so no refusal
    /// offers a declaration, and each states what its door read.
    #[test]
    fn an_uncovered_circle_clearance_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let frame = CylFrame::canonical(1.0 + (b.zero() + b.escalate()) / 2.0);
        let covered = Coincide::ArcOnCoveredFace;
        let postures = [
            (
                None,
                false,
                None,
                Coincide::ArcClearsCurvedFace,
                DeclarationRead::Moot,
            ),
            (
                Some(ContactClass::Rest),
                true,
                Some(true),
                Coincide::VertexOnCoveredFace,
                DeclarationRead::Spent(ContactClass::Rest),
            ),
            (
                Some(ContactClass::Rest),
                false,
                Some(true),
                covered,
                DeclarationRead::Spent(ContactClass::Rest),
            ),
            (
                Some(ContactClass::Tangent),
                false,
                Some(false),
                covered,
                DeclarationRead::Spent(ContactClass::Tangent),
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
    /// crosses the wall (a sheet whose axis is offset by 0.3), refuses
    /// at the frontier under every posture, the door refusing `Rest`
    /// (the axes are apart) and a pair called one carrier past the door
    /// refusing all the same.
    #[test]
    fn the_circle_rungs_zero_arm_takes_a_declaration_and_its_crossing_arm_takes_none() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let frontier = |label: &str, got: &Result<(), BooleanError>| {
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
            for got in &runs {
                frontier(&format!("crossing arm, {class:?}"), got);
            }
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
        let declared = DeclaredPairs::build(&BooleanDeclarations::none(), Default::default());
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
        class: Option<ContactClass>,
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
            let got = union_declared(&a, &b, pair, Some(ContactClass::Tangent));
            assert_eq!(
                decision_of(label, &got),
                BooleanDecision::Coincidence(
                    Coincide::Planes,
                    DeclarationRead::Spent(ContactClass::Tangent)
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
    /// that settles it**, on a union: a thin block whose bottom face,
    /// cornered at a point of the other block's top face by a 5° wedge,
    /// is tilted by an in-band angle about the wedge's one edge, so the
    /// wedge's two edges read on that face while its normal reads
    /// in-band parallel at the corner's arm. Undeclared, the sector
    /// refuses and offers the declaration; declared `Rest`, which the
    /// door verifies, the pair's class is read before the parallelism
    /// refuses, the lump takes the residue, and the union builds.
    #[test]
    fn a_coplanar_sectors_in_band_parallelism_is_settled_by_the_declaration_it_offers() {
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
        let wedge = mapped_cube::<f64>(
            move |u, v, w| p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, w),
            tol,
        );
        // Its top face reaches far enough from the tilt axis that each
        // of its corners reads definitely off the wedge's bottom plane.
        let block = brick((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
        let pair = (face_facing(&block, [0.0, 0.0, 1.0]), {
            let hits: Vec<_> = wedge
                .faces()
                .map(|(k, _)| k)
                .filter(|&k| {
                    matches!(crate::boolean::face_carrier(&wedge, k),
                        Some(crate::boolean::CarrierDesc::Plane { normal, .. }) if normal.z < -0.99)
                })
                .collect();
            assert_eq!(hits.len(), 1, "the wedge's bottom face");
            hits[0]
        });
        let undeclared = union_declared(&block, &wedge, pair, None);
        assert!(
            matches!(
                decision_of("undeclared", &undeclared),
                BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Settles(s))
                    if s.class() == ContactClass::Rest
            ),
            "the lookup mints the class the door admits on a planar pierced face: {undeclared:?}"
        );
        let text = undeclared.expect_err("it refuses").to_string();
        assert!(
            text.starts_with("how two corners of the two solids overlap where they meet is ")
                && text.contains(
                    "Recourse: declare the coincidence, or move the parts so they clearly meet or \
                     clearly stand apart there, or, if this gap is intended, tighten the \
                     tolerance below "
                ),
            "{text}"
        );
        let declared = union_declared(&block, &wedge, pair, Some(ContactClass::Rest));
        assert!(
            declared.is_ok(),
            "declared Rest, the union builds: {declared:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod wall_root_tests {
    use super::{BooleanDecision, BooleanError, line_wall_root_count};
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
        let mut roots = [0.0; 4];
        let got = line_wall_root_count(
            Point3::new(d, -2.0, 0.5),
            Vec3::new(0.0, 1.0, 0.0),
            4.0,
            &wall,
            &mut roots,
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
