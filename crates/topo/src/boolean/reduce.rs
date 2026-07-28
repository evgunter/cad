//! The reduction sweep (ch. 15 §15.5, Programs 15.2–15.4 re-derived):
//! all-pairs edge×face in BOTH directions, realizing the eight-step
//! specification in one sweep with `contfp`/`contfv` typed case codes.
//!
//! - **Quadratic, documented**: `O(E_A·F_B + E_B·F_A)` face loops per
//!   edge fragment — correctness first; a BVH/box filter is the
//!   PERF-PLAN's later 10× and changes nothing here.
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

use geom_core::{Band, Decide, Point3, Sign};

use super::contain::{ContainError, FaceContainment, contfp};
use super::plane_eq::PlaneDesc;
use super::{BooleanError, ContactRecords, Operand, VfContact, VvContact};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::validate::decide;

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

/// The F5/scaffolding gate for one operand (split_reduce's, with
/// operand-tagged errors).
pub(super) fn gate_planar<T: Decide>(body: &Body<T>, operand: Operand) -> Result<(), BooleanError> {
    for (face_key, face) in body.faces() {
        match body.get_surface(face.surface) {
            Some(geom_surfaces::Surface::Plane { .. }) => {}
            _ => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face: face_key,
                });
            }
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
                geom_curves::Curve3::Line { .. } => {}
                _ => {
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
            return Err(BooleanError::NonMaximalFaces {
                operand,
                edge: edge_key,
            });
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

/// One sweep direction: every edge (fragment) of `x` against every face
/// of `y`. `x_is` names which operand `x` is (contact orientation).
pub(super) fn sweep_direction<T: Decide>(
    x: &mut Body<T>,
    y: &mut Body<T>,
    x_is: Operand,
    contacts: &mut ContactAcc,
    band: Band,
) -> Result<(), BooleanError> {
    let faces: Vec<FaceKey> = y.faces().map(|(k, _)| k).collect();
    let mut worklist: std::collections::VecDeque<(EdgeKey, usize)> =
        x.edges().map(|(k, _)| (k, 0)).collect();

    while let Some((edge_key, start)) = worklist.pop_front() {
        let mut j = start;
        'faces: while j < faces.len() {
            let face = faces[j];
            let plane = face_plane(y, face).ok_or(BooleanError::ClassificationInvariant {
                what: "post-gate face lost its plane",
            })?;
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
                    match contfp(y, face, plane.normal, p, band).map_err(|e| esc(e, x_is))? {
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
                    if za == Sign::Zero {
                        vertex_on_face(x_is, y, u, pu, face, &plane, contacts, band)?;
                    }
                    if zb == Sign::Zero {
                        vertex_on_face(x_is, y, v, pv, face, &plane, contacts, band)?;
                    }
                }
            }
            j += 1;
        }
    }
    Ok(())
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
/// classify it against the face and record the contact kind.
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
) -> Result<(), BooleanError> {
    match contfp(y, face, plane.normal, px, band).map_err(|e| esc(e, x_is.other()))? {
        FaceContainment::Out => {}
        FaceContainment::In => contacts.vf(x_is, VfContact { vertex: vx, face }),
        FaceContainment::OnEdge(ey) => {
            let wy = split_other_at_point(y, x_is.other(), ey, px)?;
            push_vv(contacts, x_is, vx, wy);
        }
        FaceContainment::OnVertex(vy) => push_vv(contacts, x_is, vx, vy),
    }
    Ok(())
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
