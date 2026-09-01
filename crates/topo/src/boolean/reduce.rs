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
//!   value channel is pinned bit-equal. In the full boolean the same
//!   in-band margin typically resurfaces at a LATER stage anyway (the
//!   disjoint-operands containment walk decides against the same
//!   plane), so what actually diverges is the refusal SITE, not
//!   success: pinned predicate-by-predicate in the suite's grazing
//!   fixture.
//! - **Worklist, not recursion** (Problem 15.3 / F12): a proper
//!   crossing splits the edge through the certified `split_edge` lane
//!   and pushes BOTH children back with the *next* face index (a line
//!   crosses a plane at most once, so the split face is done with both
//!   children); each split strictly shortens spans — termination is
//!   structural.
//! - **Coplanar edge-face pairs are skipped** (both endpoints ON the
//!   face plane ⇒ endpoint processing only): every relevant crossing
//!   inside the face is caught when the edge is swept against the
//!   face's noncoplanar NEIGHBOR faces, where the crossing point lands
//!   ON the shared boundary edge (the `OnEdge` case — tested by the
//!   coplanar-overlap acceptance fixture).
//! - **Edge-on-edge crossings** are discovered as edge-face events
//!   landing ON an edge of the face: BOTH edges are split at the
//!   (bitwise-shared) intersection point — the minted vertices are a
//!   declared v-v contact pair by construction.
//! - Sweep order (D9): direction A→B fully, then B→A; edges in arena
//!   order, faces in arena snapshot order, worklist FIFO.

use geom_core::{Band, Bounds, Decide, Margin, Point3, Sign};

use super::boxes;
use super::contain::{ContainError, FaceContainment, contfp};
use super::plane_eq::PlaneDesc;
use super::{BooleanError, ContactRecords, Operand, VfContact, VvContact};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
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
/// cylinder×sphere SSI arm, structurally routed) and `Nurbs` (the
/// plane×NURBS arm, routed structurally so PR 7b's flag flip alone
/// makes it live). Pair-level refusals fire at the sites that
/// EXERCISE an arm (the sweep's crossing lanes, the join's section
/// table), citing the C5 routing; kinds with no wired arm at all
/// (`Cone`, `Torus`) are what [`gate_operand_pairs`] tests boxes for.
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
        geom::Surface::Plane { .. } | geom::Surface::Cylinder { .. } | geom::Surface::Sphere { .. }
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
///   built from it say "may intersect" rather than claiming a
///   meeting the kernel has not computed.
///
/// The pad is the sweep's own ([`super::boxes::sweep_pad`]), so the
/// gate's boxes are the same boxes candidate generation reads: the
/// gate cannot admit a pair the sweep would then prune, nor refuse
/// one it would examine.
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
                if boxed.overlaps(other_box) {
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
///   ([`first_unsupported_pair`]). A torus wall whose box clears the
///   other operand does not gate anything.
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
    band: Band,
) -> Result<(), BooleanError> {
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        gate_operand_edges(body, operand)?;
    }
    if let Some(p) = first_unsupported_pair(a, b, band, boolean_arm_exists)? {
        return Err(BooleanError::CurvedPairUnsupported {
            op: None,
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
                geom::Curve3::Nurbs(_) => {
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
/// [`PlaneDesc::normal`] is contractually the unit OUTWARD normal, and
/// since S10 that is the surface's chart normal times
/// [`crate::entity::Face::sense_sign`]: the chart is the only place
/// orientation was ever encoded, so on a `sense: false` face the
/// stored normal points INTO the material, and every consumer reading
/// a material direction off it would answer backwards. The flip
/// itself lives in [`crate::face_normal`], which this function is
/// defined in terms of — one door for the planar consumers
/// (`plane_of`, this sweep, the pierce lane, the REST lane, and the
/// SHARED [`crate::sector_face`] walk, which is why the door sits at
/// the crate root rather than here), one flip, so those consumers stay
/// orientation-blind.
///
/// "One door" is true of those consumers, not of the workspace: other
/// faces' outward normals are still hand-multiplied. The ones **in
/// this crate** are inventoried by [`crate::face_normal`]'s guard,
/// which COMPUTES them rather than reciting them. The four outside it
/// — in `editor-core`, `mesh` and `sweep` — are beyond any `topo`
/// walk; they are listed once in `docs/SMELL-SCAN-2026-08.md` at S67,
/// beside D6's work order, and nowhere in this tree (smell-scan D6).
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

/// The recipe source of the face's **oriented plane description** —
/// the datum [`super::oriented_plane_eq`]'s rung 1 needs, which is NOT
/// the same thing as the surface's source ([`face_source`]).
///
/// Rung 1 answers Same±-orientation syntactically, from the two
/// sources' `orient` tags, and asserts (debug) that same-source
/// descriptions agree bitwise. Since S10 the descriptions rung 1 is
/// handed are [`face_plane`]'s — the faces' OUTWARD normals — so two
/// faces sharing one surface key and one recipe source but differing
/// in `sense` carry descriptions that are exact negations of each
/// other. Left uncomposed, the rung would call that pair
/// `SameOriented` on the strength of the surface sources alone, and
/// the bit assertion would fire on the very configuration S10 exists
/// to express.
///
/// N6's `orient` tag already MEANS "this description is the source
/// expression's orientation-reversal", which is exactly what a
/// `sense: false` face's outward normal is, so the sense bit composes
/// into it through [`crate::GeomSource::reverted`] and rung 1 keeps
/// deciding exactly, with zero numerics. Returned owned: the flip
/// mints a value rather than borrowing the stored one (the stored
/// source describes the SURFACE and must not be rewritten by a
/// face-level question).
pub(super) fn face_plane_source<T: Decide>(
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
        let face_of = |he| {
            let parent = body.get_half_edge(he)?.parent_loop;
            Some(body.get_loop(parent)?.face)
        };
        let (Some(f1), Some(f2)) = (face_of(edge.he_plus), face_of(edge.he_minus)) else {
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
        let (o1, o2) = (face_plane_source(body, f1), face_plane_source(body, f2));
        let id = super::PlaneIdentity {
            s1: o1.as_ref(),
            s2: o2.as_ref(),
            declared: false,
        };
        match super::oriented_plane_eq(&p1, &p2, id, arm, band) {
            Ok(super::PlaneRelation::Distinct) => {}
            Ok(_) => {
                return Err(BooleanError::NonMaximalFaces {
                    operand,
                    edge: edge_key,
                });
            }
            Err(super::PlaneEqError::Escalated(diag)) => {
                return Err(BooleanError::Escalated { diag });
            }
            Err(super::PlaneEqError::Undeclared { diag, relation }) => {
                // Same-operand pair (the F7 maximal-faces gate): both
                // entries carry THIS operand's tag.
                return Err(BooleanError::UndeclaredCoincidence {
                    diag,
                    pair: [(operand, f1), (operand, f2)],
                    relation,
                });
            }
            // Unreachable with `declared: false`; kept typed.
            Err(super::PlaneEqError::Contradicted(diag)) => {
                return Err(BooleanError::DeclarationContradicted { diag });
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
            // endpoint-sign match below cannot see). Interior roots
            // split exactly like proper line crossings; the remainder
            // fragment re-examines the SAME face for the second root.
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
                    Ok(None) => {
                        // A conic that definitely never meets the
                        // plane: endpoint processing only (Zero
                        // endpoints are impossible here; fall through
                        // for the trace's sake).
                        continue;
                    }
                    Ok(Some(Err(diag))) => {
                        return Err(BooleanError::Escalated { diag });
                    }
                    Ok(Some(Ok(roots))) => {
                        if let Some(&t) = roots.first() {
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
                        // No interior root: endpoint processing only.
                        let side = |p: Point3<T>| {
                            decide(
                                "bool_vertex_face_side",
                                Margin::of((p - plane.origin).dot(plane.normal)),
                                band,
                            )
                        };
                        let s1 = side(pu).map_err(|diag| BooleanError::Escalated { diag })?;
                        let s2 = side(pv).map_err(|diag| BooleanError::Escalated { diag })?;
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
            let s1 = side(pu).map_err(|diag| BooleanError::Escalated { diag })?;
            let s2 = side(pv).map_err(|diag| BooleanError::Escalated { diag })?;
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
/// the residual is convex (both-inside means no wall crossing,
/// both-outside clears through the span minimum), and for a CIRCLE
/// carrier the ARC's residual range is enclosed two ways (the
/// carrier's exact harmonic bounds and the arc's own chord-dip
/// bound), so a definitely one-sided arc clears. What definitely MEETS
/// the face is split by kind, and the third paragraph below is the
/// statement of record: a LINE carrier against a CYLINDER wall is
/// routed through the certified roots and pierces; everything else —
/// a tangency, a CIRCLE carrier, a sphere face, an undeclared
/// on-carrier edge, a trim with no verdict — refuses typed at the named
/// frontier door ([`BooleanError::CurvedPierceUnsupported`]). An
/// in-band clearance escalates (F6, the same margin's other half).
/// Ellipse/NURBS carriers keep the unconditional M5 door. Never a
/// silent fallback.
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
/// unlocks. One undeclared arm does read the third outcome, and only in
/// the refusing direction: the MIXED-SIGN `NoInterior` span (one
/// endpoint ON the carrier, the other definitely clear) whose every ON
/// endpoint is certified `Elsewhere` answers NO EVENT instead of
/// refusing, because the pierce ring that produced that fragment mints
/// the vertex interior to the sibling face which does hold it. Nothing
/// is recorded there; a refusal becomes an honest silence.
///
/// **The pierce ring lane** (the definite-crossing half): a LINE edge
/// that definitely crosses a cylinder WALL inside that wall's trim is
/// no longer the frontier. Its crossing parameters are the same
/// certified quadratic the ray lane has always solved
/// ([`super::solid_contain::line_wall_roots`]) taken over the edge's
/// own span instead of a ray's forward half; the landing point is
/// placed by [`super::contain::curved_face_containment`]; the
/// split/record triple is the planar conic lane's, verbatim. Still the
/// frontier is everything the roots do not cover: a TANGENCY (an
/// in-band discriminant is not a crossing at any order this lane sees),
/// a CIRCLE carrier against a wall (a degree-2 trigonometric residual
/// with no root lane in this tree), a SPHERE face, and a trim the chart
/// door declines to express.
///
/// **What a successful pierce reaches next is a typed door, not a
/// body**: a ring minted in a face has no join arm on any carrier
/// (#1291), so a wall pierce lands on
/// `SplitJoinError::SectionArcWindow{NoChartedRun}` exactly as the
/// planar cap pierce lands on `SectionLoopMixed`.
///
/// **This lane WIDENS what an undeclared pair reaches, and the widening
/// is named here rather than left to be discovered.** Before it,
/// [`vertex_on_curved_face`] was reachable only behind the declared
/// cover; the new endpoint arms (`(Zero, definite)` and the
/// `(Zero, Zero)` chord) call it on UNDECLARED pairs too. That is not
/// the C8 gate reopening: C8 protects the claim that an on-carrier EDGE
/// is cosurface, and the arms below reach the endpoint treatment only
/// after the certified roots have proved there is no interior crossing
/// — which, for the `(Zero, Zero)` arm, takes two DISTINCT roots and so
/// structurally excludes an edge lying on the wall (only rulings do,
/// and a ruling answers `Constant`). What the door then does is
/// point-in-face containment on a chart, which is a trim question and
/// not a gluing one.
///
/// Returns what the caller must do about the pair — see
/// [`CurvedEvent`]. The split itself needs `&mut x` and the worklist,
/// both of which live in [`sweep_direction`], so the crossing is
/// REPORTED here and performed there rather than the body being
/// threaded in for one branch.
#[allow(clippy::too_many_arguments)]
fn curved_face_arm<T: Decide>(
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
    // incidence.
    let covered = {
        let parent = |he| {
            x.get_half_edge(he)
                .and_then(|h| x.get_loop(h.parent_loop))
                .map(|l| l.face)
        };
        [parent(edge.he_plus), parent(edge.he_minus)]
            .into_iter()
            .flatten()
            .any(|f| declared.class_of(x_is, f, x_is.other(), face).is_some())
    };
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
    // - **the arc's**: the residual at the two ENDPOINTS bounds the
    //   interior through the same chord-dip argument the line row uses
    //   along a segment — a smooth function stays within `|F″|·Δθ²/8`
    //   of its endpoint chord, and `|F″|` is the harmonics' own bound
    //   (`geom_brep::circle_residual_curvature_bound`). Tight for a
    //   short arc, useless for a full turn.
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
            let Some((lo, hi)) =
                geom_brep::circle_residual_extremes(&surface, center, axis, radius, u_ref)
            else {
                return Err(frontier());
            };
            let carrier_margin = lo.max(-hi);
            let arc_margin =
                geom_brep::circle_residual_curvature_bound(&surface, center, axis, radius, u_ref)
                    .map_or(carrier_margin, |f2| {
                        let (t0, t1) = curve.params();
                        // The line row's vertex CLAMP does not port
                        // here, and the reason is the curve: along a
                        // line the residual is exactly quadratic, so
                        // "the vertex is outside the span" is a
                        // statement about a parabola and is decided by
                        // the endpoint gap alone. Along a circle it is
                        // a degree-≤2 TRIGONOMETRIC polynomial with up
                        // to four critical parameters, so an endpoint
                        // gap says nothing about where its minimum
                        // sits. The unclamped chord-dip charge is what
                        // is available without solving for them.
                        let dip = f2 * (t1 - t0).powi(2) * T::from_f64(0.125);
                        let r_u = geom_brep::implicit_residual(&surface, pu);
                        let r_v = geom_brep::implicit_residual(&surface, pv);
                        (r_u.min(r_v) - dip).max(-(r_u.max(r_v) + dip))
                    });
            let margin = Margin::of(carrier_margin.max(arc_margin));
            return match decide("bool_circle_curved_clearance", margin, band) {
                Ok(Sign::Positive) => Ok(CurvedEvent::None),
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
                // `Out` has THREE sources, and the argument differs by
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
                // - **OFF-CARRIER** — the containment door's own first
                //   test, reachable when the clearance row called the
                //   arc on-carrier within the band and the point row
                //   then resolves definitely off it at the same band.
                //
                // The last two have no neighbour to appeal to, so the
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
                        match side(pw).map_err(|diag| BooleanError::Escalated { diag })? {
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
                    if Placement::records_the_pair(ends) {
                        Ok(CurvedEvent::Recorded)
                    } else {
                        Err(frontier())
                    }
                }
                Ok(Sign::Zero | Sign::Negative) => Err(frontier()),
                Err(diag) => Err(BooleanError::Escalated { diag }),
            };
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
    let s1 = side(pu).map_err(|diag| BooleanError::Escalated { diag })?;
    let s2 = side(pv).map_err(|diag| BooleanError::Escalated { diag })?;
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
        // other (the contract sentence on [`super::rest::tangent_locus`];
        // a `Rest` cover's shared carrier is residual-zero
        // identically) — so a covered on-carrier edge's residual is
        // one-signed and a Zero endpoint is a touch, never an entry.
        // A configuration without that one-sign story must not be
        // admitted to the lane (issue #974 names the coaxial
        // cylinder×sphere circle arm's blocking precondition). A
        // NEGATIVE partner is a genuine crossing — never the covered
        // posture. Uncovered keeps both frontier doors verbatim.
        (Sign::Zero, Sign::Zero) if covered => {
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
            let h = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
            if Placement::records_the_pair([Some(h), None]) {
                Ok(CurvedEvent::Recorded)
            } else {
                Err(frontier())
            }
        }
        (Sign::Positive, Sign::Zero) if covered => {
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
        // the on-carrier-edge question, which is a cosurface claim and
        // stays behind the declaration gate above.
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
                SpanVerdict::NoInterior => {
                    // UNDECLARED, and deliberately NOT
                    // [`Placement::records_the_pair`] — that rule is
                    // the DECLARED rungs', bought by a verified
                    // declaration, and it is stricter than this arm in
                    // one direction and laxer in the other. This arm
                    // keeps its own long-standing OR rule for the
                    // recording half (one `Recorded` end records, and
                    // the sibling's answer does not veto it), and adds
                    // one thing on top: an ON endpoint the trim places
                    // definitely `Elsewhere` is a CERTIFIED absence
                    // from this face, not the door's remainder, so a
                    // span whose every ON end came back that way is
                    // honestly eventless HERE. That is the TRIM
                    // question, separate from the gluing question the
                    // declared rungs answer; the derivation is below.
                    let mut hit = false;
                    // Every ON endpoint definitely placed OUTSIDE this
                    // face's trim (see below).
                    let mut all_elsewhere = true;
                    for (on, w, pw) in [(s1 == Sign::Zero, u, pu), (s2 == Sign::Zero, v, pv)] {
                        if !on {
                            continue;
                        }
                        match vertex_on_curved_face(x_is, y, w, pw, face, contacts, band, tol)? {
                            Placement::Recorded => hit = true,
                            Placement::Elsewhere => {}
                            Placement::Undecided => all_elsewhere = false,
                        }
                    }
                    if hit {
                        return Ok(CurvedEvent::Recorded);
                    }
                    // **No incidence with THIS face, certified.** The
                    // span's only contacts with the wall's CARRIER are
                    // this arm's ON endpoint(s): `NoInterior` reports
                    // two distinct definite roots with none of them
                    // strictly inside the span AND inside this face's
                    // trim, and a line meets a cylinder at most twice.
                    // A face is a subset of its carrier, so an ON
                    // endpoint the trim places definitely OUT is not
                    // an incidence of this face — the same sentence
                    // `wall_crossing` already writes when it steps
                    // over an interior root the trim puts `Out`. The
                    // event is a SIBLING face's: a wall pierce mints
                    // its vertex interior to the face that holds it,
                    // and the split's fragments are then re-queried
                    // against every other face of the same carrier.
                    //
                    // The `Undecided` half is what keeps this honest.
                    // An endpoint with no verdict is not evidence of
                    // absence, so a single one of them takes the whole
                    // span back to the typed door.
                    if all_elsewhere {
                        Ok(CurvedEvent::None)
                    } else {
                        Err(frontier())
                    }
                }
            }
        }
        // **BOTH endpoints on the wall, the pair UNDECLARED** — the
        // CHORD a wall pierce leaves behind: an edge that crossed the
        // wall twice is split at both roots, and the middle fragment
        // is what remains.
        //
        // This is the one arm where the cosurface fence and the ring
        // lane meet, so the separation is STRUCTURAL rather than
        // numeric: `NoInterior` is reached only through two DISTINCT
        // definite roots, and a line with two distinct points on a
        // cylinder is a secant. The only lines that LIE on a wall are
        // its rulings; a ruling is axis-parallel and answers
        // `Constant`. So an on-carrier edge cannot reach the endpoint
        // treatment here and the undeclared cosurface question keeps
        // its door untouched (CONTACT-DESIGN C2/C4) — as does a
        // tangency, a trim with no verdict, and every other answer.
        //
        // **What this paragraph argues is narrow, and it is not the
        // ruling on the undeclared TRIM question.** That question —
        // whether a definite `Elsewhere` is answered as no-event on an
        // undeclared pair — was settled separately and in the
        // affirmative, but only for the MIXED-SIGN arm above, where
        // the span has exactly one ON end and the pierce ring supplies
        // the sibling face that holds the event. Here BOTH ends are ON
        // and the chord's own interior is the thing in question, so
        // this arm keeps the strict rule: only a `Recorded` end
        // records. What is argued above is the C2/C4 COSURFACE
        // question, which cannot be reached from here for the
        // structural reason given, and that holds either way.
        (Sign::Zero, Sign::Zero) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::NoInterior => {
                    // UNDECLARED, and STRICTER than the mixed-sign arm
                    // above: only a RECORDED endpoint counts, every
                    // other answer keeps the frontier door.
                    // [`Placement::records_the_pair`] — the declared
                    // rungs' rule, which lets an `Elsewhere` end pass
                    // beside a recorded one — is deliberately NOT
                    // applied outside a verified declaration; and the
                    // mixed-sign arm's certified-`Elsewhere` no-event
                    // does not reach here either, because with BOTH
                    // ends ON the carrier the chord's interior is
                    // itself the unanswered question.
                    let hu = vertex_on_curved_face(x_is, y, u, pu, face, contacts, band, tol)?;
                    let hv = vertex_on_curved_face(x_is, y, v, pv, face, contacts, band, tol)?;
                    if hu == Placement::Recorded || hv == Placement::Recorded {
                        Ok(CurvedEvent::Recorded)
                    } else {
                        Err(frontier())
                    }
                }
                _ => Err(frontier()),
            }
        }
        // **A definite surface crossing: the pierce RING lane.** The
        // endpoints straddle the carrier, so a root exists in the span;
        // `wall_crossing` finds it exactly and the trim decides whether
        // it lands in this face. `NoInterior` CONTRADICTS the straddle,
        // so it keeps the door rather than reporting no event — a
        // contradiction between two certified predicates is exactly
        // what must not be resolved by picking one.
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band)? {
                SpanVerdict::Pierce { t, p, at } => Ok(CurvedEvent::Pierce { t, p, at }),
                _ => Err(frontier()),
            }
        }
        // Both inside: the residual along a line is convex, so its
        // maximum is at an endpoint — definitely no wall crossing.
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
                        SpanVerdict::NoInterior | SpanVerdict::Miss => Ok(CurvedEvent::None),
                        // **`Constant` is NOT a clearance here.** It
                        // reports that the axis-parallel test —
                        // `|d_perp|²/2r`, a SQUARED transverse
                        // component — did not come back definitely
                        // positive, which is an L²-amplified window: a
                        // direction a band-width off the axis has a
                        // transverse component of order `sqrt(band)`,
                        // and the residual it accumulates over a long
                        // span is not the constant this verdict would
                        // read it as. Clearing on it would be a silent
                        // wrong answer at exactly the poses the belly
                        // arm exists for, so it keeps the door. (The
                        // ray lane's sibling at
                        // `solid_contain::cast_ray` skips the face on
                        // the same verdict and is not touched here: its
                        // pre-pass has already put `q` definitely off
                        // the wall, which is the one-sign story this
                        // arm does not have.)
                        SpanVerdict::Constant => Err(frontier()),
                        SpanVerdict::Unsettled => Err(frontier()),
                    }
                }
                Err(diag) => Err(BooleanError::Escalated { diag }),
            }
        }
    }
}

/// What the certified line × wall roots say about ONE edge span.
#[derive(Debug, Clone, Copy)]
enum SpanVerdict<T: geom_core::Real> {
    /// A definite crossing strictly inside the span, which the trim
    /// placed at `at` in this face.
    Pierce {
        t: T,
        p: Point3<T>,
        at: FaceContainment,
    },
    /// Two definite roots, none of them STRICTLY INSIDE the span on
    /// this face: each lies outside the span, sits at one of its ends,
    /// or falls outside the face's trim. Two distinct roots also
    /// certify that the line is NOT a ruling of the wall, which is
    /// what separates a chord from an on-carrier edge. What the
    /// absence of an interior crossing licenses depends on the
    /// endpoints, so the caller decides — an endpoint incidence is
    /// still an event, it is just not an interior one.
    NoInterior,
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

/// The line × cylinder-wall crossing route: solve the certified
/// quadratic, keep the roots the EDGE's span carries strictly inside,
/// and place the landing point in the face's trim.
///
/// **Roots at the span's ends are deliberately NOT interior.** A root
/// the band cannot separate from an endpoint is that endpoint's own
/// incidence, and the endpoint rows own those (they mint a v-f or v-v
/// record rather than a split, which is what a split at `t₀` could not
/// do anyway — `split_edge`'s interiority trilean refuses it). Folding
/// the two together here would have made an endpoint touch look like a
/// crossing to every caller.
///
/// **Both roots are examined, and the FIRST interior one wins.** A
/// segment through a wall meets it twice; the sweep splits at one root
/// and re-queues both fragments against the SAME face, so the second is
/// found on the next pass — the shape the conic × plane lane already
/// uses, and the reason this function does not return a pair.
fn wall_crossing<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> Result<SpanVerdict<T>, BooleanError> {
    let (
        geom::Curve3::Line { origin, dir },
        geom::Surface::Cylinder {
            origin: c_origin,
            axis,
            radius,
            ..
        },
    ) = (carrier, surface)
    else {
        // A sphere face, or a carrier that is not a line: no root lane
        // here, and inventing one is the fence this PR keeps.
        return Ok(SpanVerdict::Unsettled);
    };
    let roots =
        super::solid_contain::line_wall_roots(*origin, *dir, *c_origin, *axis, *radius, band)
            .map_err(|diag| BooleanError::Escalated { diag })?;
    let ts = match roots {
        super::solid_contain::WallRoots::Two(ts) => ts,
        // A tangency is not a crossing this lane can act on: the
        // material verdicts behind a pierce are first-order, and along
        // a tangency every first-order datum ties. It keeps the door.
        super::solid_contain::WallRoots::Tangent => return Ok(SpanVerdict::Unsettled),
        // A constant residual, or no root on the infinite line at all.
        super::solid_contain::WallRoots::AxisParallel => return Ok(SpanVerdict::Constant),
        super::solid_contain::WallRoots::Miss => return Ok(SpanVerdict::Miss),
    };
    for t in ts {
        // `t` is arc length in metres (a `Line` carrier's `dir` is
        // unit), so both gaps are lengths and take `Margin::of`.
        let mut interior = true;
        for gap in [t - t0, t1 - t] {
            match decide("bool_wall_root_in_span", Margin::of(gap), band) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Negative | Sign::Zero) => interior = false,
                Err(diag) => return Err(BooleanError::Escalated { diag }),
            }
        }
        if !interior {
            continue;
        }
        let p = carrier.eval(t);
        // The face's own trim decides whether a crossing of the CARRIER
        // is a crossing of this FACE. `None` is the chart door's honest
        // remainder (a ringed face, a non-iso boundary, a full-period
        // azimuth window) and keeps the caller's frontier rather than
        // reading as "outside".
        match super::contain::curved_face_containment(y, face, p, band) {
            Ok(None) => return Ok(SpanVerdict::Unsettled),
            // Definitely outside THIS face's trim: the carrier is
            // crossed, but not here. The other root may still land in
            // the face, so the loop continues rather than concluding.
            Ok(Some(FaceContainment::Out)) => {}
            Ok(Some(at)) => return Ok(SpanVerdict::Pierce { t, p, at }),
            Err(super::contain::ContainError::Escalated(diag)) => {
                return Err(BooleanError::Escalated { diag });
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
    Ok(SpanVerdict::NoInterior)
}

/// What one edge×curved-face pair asks of the sweep.
#[derive(Debug, Clone, Copy)]
enum CurvedEvent<T: geom_core::Real> {
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
enum Placement {
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
fn vertex_on_curved_face<T: Decide>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<Placement, BooleanError> {
    let verdict = super::contain::curved_face_containment(y, face, px, band)
        .map_err(|e| esc(e, x_is.other()))?;
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
                    diag: geom_core::Indeterminate {
                        margin: geom_core::MarginDiag::Invalid,
                        band,
                        predicate: Some("bool_contact_vertex"),
                    },
                });
            }
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
    }
    Ok(match verdict {
        Some(FaceContainment::Out) => Placement::Elsewhere,
        _ => Placement::Undecided,
    })
}

fn esc(e: ContainError, operand: Operand) -> BooleanError {
    match e {
        ContainError::Escalated(diag) => BooleanError::Escalated { diag },
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
/// [`super::contain::point_on_arc`] uses for the same quantity — before
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
        // On the carrier? The row and its impossible-negative arm have
        // one body, shared with `super::contain::point_on_arc`, which
        // asks the same question of the same quantity.
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
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
        // A span of at most one period is what makes the MIDPOINT
        // anchor's branch the right one, and it is CHECKED rather than
        // assumed: past a period the azimuth aliases by 2π silently,
        // and `split_edge`'s interiority gate cannot see it (an aliased
        // parameter is still inside a span that long). The row is the
        // period guard `super::contain::point_on_arc` already spells,
        // metered the same way; a full turn is `Zero` and passes.
        match decide(
            "bool_split_span_period",
            Margin::levered(T::tau() - (t1 - t0), radius),
            band,
        ) {
            Ok(Sign::Positive | Sign::Zero) => {}
            Ok(Sign::Negative) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "circle edge span exceeds one period; the split azimuth                                would alias by a turn",
                });
            }
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
    }
    let t = curve
        .carrier()
        .param_near(p, (t0 + t1) * T::from_f64(0.5))
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
