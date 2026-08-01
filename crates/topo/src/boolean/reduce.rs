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

use geom_core::{Band, Bounds, Decide, Point3, Sign};

use super::boxes;
use super::contain::{ContainError, FaceContainment, contfp};
use super::plane_eq::PlaneDesc;
use super::{BooleanError, ContactRecords, Operand, VfContact, VvContact};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::validate::decide;

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

/// The per-arm operand gate (M5 PR 9, C12.1 — the F5 planar-only gate
/// retires PER C5 TABLE ARM, never wholesale). Face kinds with at
/// least one wired boolean arm pass here — `Plane`, `Cylinder` (the
/// PR 5 conic arms), `Sphere` (the PR 7 cylinder×sphere SSI arm,
/// structurally routed), and `Nurbs` (the plane×NURBS arm, routed
/// structurally so PR 7b's flag flip alone makes it live) — and the
/// pair-level refusals move to the sites that EXERCISE an arm (the
/// sweep's crossing lanes, the join's section table), where they cite
/// the C5 routing. Kinds with no wired arm at all (`Cone`, `Torus`)
/// keep the gate refusal. Edge carriers: `Line`/`Circle`/`Ellipse`
/// pass (the crossing and split lanes handle all three); `Nurbs`
/// operand edges refuse typed (rung-3 INPUT operands are not in the
/// M5 envelope — rung-3 edges are what the zip MINTS).
pub(super) fn gate_planar<T: Decide>(body: &Body<T>, operand: Operand) -> Result<(), BooleanError> {
    for (face_key, face) in body.faces() {
        match body.get_surface(face.surface) {
            Some(
                geom_surfaces::Surface::Plane { .. }
                | geom_surfaces::Surface::Cylinder { .. }
                | geom_surfaces::Surface::Sphere { .. }
                | geom_surfaces::Surface::Nurbs(_),
            ) => {}
            Some(s) => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face: face_key,
                    kind: geom_brep::SurfaceKind::of(s),
                });
            }
            None => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face: face_key,
                    kind: geom_brep::SurfaceKind::Nurbs,
                });
            }
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
                geom_curves::Curve3::Line { .. }
                | geom_curves::Curve3::Circle { .. }
                | geom_curves::Curve3::Ellipse { .. } => {}
                geom_curves::Curve3::Nurbs(_) => {
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

/// The face's plane description (post-gate: always a `Plane`).
pub(super) fn face_plane<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<PlaneDesc<T>> {
    let f = body.get_face(face)?;
    match body.get_surface(f.surface) {
        Some(geom_surfaces::Surface::Plane { origin, normal, .. }) => Some(PlaneDesc {
            origin: *origin,
            normal: *normal,
        }),
        _ => None,
    }
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
                .is_some_and(|s| matches!(s, geom_surfaces::Surface::Plane { .. }));
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
        let id = super::PlaneIdentity {
            s1: face_source(body, f1),
            s2: face_source(body, f2),
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
            Err(super::PlaneEqError::Undeclared(diag)) => {
                return Err(BooleanError::UndeclaredCoincidence { diag });
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
#[allow(clippy::too_many_arguments)] // one parameter per named duty (bodies, orientation, sinks, band, strategy, plant, trace)
pub(super) fn sweep_direction<T: Decide + Bounds>(
    x: &mut Body<T>,
    y: &mut Body<T>,
    x_is: Operand,
    contacts: &mut ContactAcc,
    band: Band,
    strategy: SweepStrategy,
    knobs: &SweepKnobs,
    mut trace: Option<&mut SweepTrace>,
) -> Result<(), BooleanError> {
    let faces: Vec<FaceKey> = y.faces().map(|(k, _)| k).collect();
    // Realized: the per-direction face tree, built ONCE over the face
    // snapshot (arena order = input order). Mid-sweep splits of `y`'s
    // edges only mint vertices ON existing boundary (within the pad),
    // so the snapshot boxes stay conservative for the whole direction.
    let pad = knobs.pad_override.unwrap_or_else(|| boxes::sweep_pad(band));
    let tree = match strategy {
        SweepStrategy::Realized => {
            let mut face_boxes = Vec::with_capacity(faces.len());
            for &f in &faces {
                let planted = knobs.plant == Some(f);
                face_boxes.push(if planted {
                    // Pin (iii)'s planted degradation: the inverted box
                    // overlaps nothing — this face's events get lost
                    // and the suite's superset pin must catch it.
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
            Some(bvh::Bvh::build(&face_boxes))
        }
        SweepStrategy::Idealized => None,
    };
    let mut worklist: std::collections::VecDeque<(EdgeKey, usize)> =
        x.edges().map(|(k, _)| (k, 0)).collect();

    while let Some((edge_key, start)) = worklist.pop_front() {
        // The fragment's candidate face indices, ascending — the
        // realized set is a subsequence of the idealized scan, so the
        // examination order (and with it every split/requeue) is
        // preserved pair-for-pair.
        let candidates: Vec<usize> = match &tree {
            Some(t) => t.overlapping(&boxes::edge_box(x, edge_key, pad)?),
            None => (0..faces.len()).collect(),
        };
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
                curved_face_arm(x, y, x_is, edge_key, &edge, face, pu, pv, band)?;
                continue;
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
                                    let w = split_at(x, x_is, edge_key, t)?;
                                    contacts.vf(x_is, VfContact { vertex: w, face });
                                    requeue(&mut worklist, x, edge_key, w, j)?;
                                    break 'faces;
                                }
                                FaceContainment::OnEdge(ey) => {
                                    let w = split_at(x, x_is, edge_key, t)?;
                                    let wy = split_other_at_point(y, x_is.other(), ey, p)?;
                                    push_vv(contacts, x_is, w, wy);
                                    requeue(&mut worklist, x, edge_key, w, j)?;
                                    break 'faces;
                                }
                                FaceContainment::OnVertex(vy) => {
                                    let w = split_at(x, x_is, edge_key, t)?;
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
                                (p - plane.origin).dot(plane.normal),
                                band,
                            )
                        };
                        let s1 = side(pu).map_err(|diag| BooleanError::Escalated { diag })?;
                        let s2 = side(pv).map_err(|diag| BooleanError::Escalated { diag })?;
                        let mut hit = false;
                        if s1 == Sign::Zero {
                            hit |= vertex_on_face(x_is, y, u, pu, face, &plane, contacts, band)?;
                        }
                        if s2 == Sign::Zero {
                            hit |= vertex_on_face(x_is, y, v, pv, face, &plane, contacts, band)?;
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
                    (p - plane.origin).dot(plane.normal),
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
                            let w = split_at(x, x_is, edge_key, t)?;
                            contacts.vf(x_is, VfContact { vertex: w, face });
                            requeue(&mut worklist, x, edge_key, w, j + 1)?;
                            break 'faces;
                        }
                        FaceContainment::OnEdge(ey) => {
                            let w = split_at(x, x_is, edge_key, t)?;
                            let wy = split_other_at_point(y, x_is.other(), ey, p)?;
                            push_vv(contacts, x_is, w, wy);
                            requeue(&mut worklist, x, edge_key, w, j + 1)?;
                            break 'faces;
                        }
                        FaceContainment::OnVertex(vy) => {
                            let w = split_at(x, x_is, edge_key, t)?;
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
                        hit |= vertex_on_face(x_is, y, u, pu, face, &plane, contacts, band)?;
                    }
                    if zb == Sign::Zero {
                        hit |= vertex_on_face(x_is, y, v, pv, face, &plane, contacts, band)?;
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

/// The curved-face sweep arm (M5 PR 9, C12.1): endpoint sides come
/// from the linearized implicit residual; a definite miss is PROVEN
/// (the residual along a line is convex — both-inside means no wall
/// crossing, both-outside clears through the span minimum); anything
/// that definitely meets the face refuses typed at the named frontier
/// door ([`BooleanError::CurvedPierceUnsupported`] — curved
/// point-in-face containment at boolean classification does not exist
/// yet), and an in-band clearance escalates (F6, the same margin's
/// other half). Never a silent fallback.
#[allow(clippy::too_many_arguments)]
fn curved_face_arm<T: Decide>(
    x: &Body<T>,
    y: &Body<T>,
    x_is: Operand,
    edge_key: EdgeKey,
    edge: &crate::entity::Edge,
    face: FaceKey,
    pu: Point3<T>,
    pv: Point3<T>,
    band: Band,
) -> Result<(), BooleanError> {
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
    // NURBS walls (shape (iii)'s substrate): the SECTION arm is
    // certified since PR 7b (geom_brep::intersect::route says so),
    // but the boolean's CROSSING layer for the kind — edge×NURBS-face
    // sweep events and curved trim containment — does not exist yet;
    // it is banked as M5 PR 9c. Refused typed HERE, before the
    // residual sides (a NURBS surface has no implicit form — the
    // sides would poison, and poison is not a refusal).
    if matches!(surface, geom_surfaces::Surface::Nurbs(_)) {
        return Err(BooleanError::CurvedBooleanUnsupported {
            operand: x_is,
            face,
            kind: geom_brep::SurfaceKind::Nurbs,
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
    // Conic carriers against a curved face have no cheap definite-miss
    // proof at M5: examined ⇒ the frontier door, typed.
    if !matches!(curve.carrier(), geom_curves::Curve3::Line { .. }) {
        return Err(frontier());
    }
    let side = |p: Point3<T>| {
        decide(
            "bool_vertex_face_side",
            geom_brep::implicit_residual(&surface, p),
            band,
        )
    };
    let s1 = side(pu).map_err(|diag| BooleanError::Escalated { diag })?;
    let s2 = side(pv).map_err(|diag| BooleanError::Escalated { diag })?;
    match (s1, s2) {
        // A vertex ON the curved surface: the v-on-curved-face door.
        (Sign::Zero, _) | (_, Sign::Zero) => Err(frontier()),
        // A definite surface crossing: the pierce door.
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Err(frontier()),
        // Both inside: the residual along a line is convex, so its
        // maximum is at an endpoint — definitely no wall crossing.
        (Sign::Negative, Sign::Negative) => Ok(()),
        // Both outside: clear through a DIVISION-FREE lower bound on
        // the span minimum of the convex residual (fix pass: the old
        // parabola-vertex formula divided by the transverse direction
        // norm, which is 0/0 on axis-parallel edges — the IDEALIZED
        // sweep examines exactly those at distance and poisoned the
        // whole op). A convex quadratic dips below its endpoint chord
        // by at most f″·Δt²/8, and f″ is closed-form per kind, so
        //   min_span f ≥ min(f(t₀), f(t₁)) − f″·Δt²/8
        // — total arithmetic, conservative direction (a too-small
        // bound only sends more pairs to the typed frontier door,
        // never accepts).
        (Sign::Positive, Sign::Positive) => {
            let geom_curves::Curve3::Line { origin: _, dir } = *curve.carrier() else {
                return Err(frontier()); // unreachable: matched above
            };
            let (t0, t1) = curve.params();
            // f″ per kind (the residual's second derivative along the
            // ray, constant for these kinds).
            let f2 = match surface {
                geom_surfaces::Surface::Cylinder { axis, radius, .. } => {
                    let d_ax = dir.dot(axis);
                    (dir.norm_squared() - d_ax.powi(2)) / radius
                }
                geom_surfaces::Surface::Sphere { radius, .. } => dir.norm_squared() / radius,
                // Post-gate/pre-check unreachable kinds keep the
                // frontier door.
                _ => return Err(frontier()),
            };
            let span = t1 - t0;
            let dip = f2 * span.powi(2) * T::from_f64(0.125);
            let r_u = geom_brep::implicit_residual(&surface, pu);
            let r_v = geom_brep::implicit_residual(&surface, pv);
            let min_bound = r_u.min(r_v) - dip;
            match decide("bool_line_cylinder_clearance", min_bound, band) {
                Ok(Sign::Positive) => Ok(()),
                Ok(Sign::Zero | Sign::Negative) => Err(frontier()),
                Err(diag) => Err(BooleanError::Escalated { diag }),
            }
        }
    }
}

fn esc(e: ContainError, operand: Operand) -> BooleanError {
    match e {
        ContainError::Escalated(diag) => BooleanError::Escalated { diag },
        ContainError::RayExhausted => BooleanError::ClassificationInvariant {
            what: "contfp ray schedule exhausted",
        },
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
) -> Result<bool, BooleanError> {
    match contfp(y, face, plane.normal, px, band).map_err(|e| esc(e, x_is.other()))? {
        FaceContainment::Out => return Ok(false),
        FaceContainment::In => contacts.vf(x_is, VfContact { vertex: vx, face }),
        FaceContainment::OnEdge(ey) => {
            let wy = split_other_at_point(y, x_is.other(), ey, px)?;
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
) -> Result<VertexKey, BooleanError> {
    x.split_edge(edge, t)
        .map(|c| c.vertex)
        .map_err(|source| BooleanError::CrossingInsertion {
            operand: x_is,
            edge,
            source,
        })
}

/// Splits the OTHER solid's boundary edge at the (already-computed)
/// event point `p` — the both-edges-split lane that turns an edge-edge
/// crossing into a v-v pair. The carrier is a line (post-gate), so the
/// parameter is the exact projection `t = (p − origin)·dir`.
fn split_other_at_point<T: Decide>(
    y: &mut Body<T>,
    y_is: Operand,
    edge: EdgeKey,
    p: Point3<T>,
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
    let geom_curves::Curve3::Line { origin, dir } = *curve.carrier() else {
        return Err(BooleanError::CurvedEdgeUnsupported {
            operand: y_is,
            edge,
        });
    };
    let t = (p - origin).dot(dir);
    split_at(y, y_is, edge, t)
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
