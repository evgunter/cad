//! The tier-3′ **global coincidence census** + declared-contact
//! certification (M3 PR 6a; F1/F2) — the injectivity pass tier 3
//! defers, run at rest against a body's declared-contact records.
//!
//! **Sweep shape**: quadratic all-pairs sweeps in arena order
//! (vertex×vertex, vertex×edge, vertex×face, edge×face, edge×edge) —
//! the boolean edge×face convention: correctness first, the BVH filter
//! is PERF-PLAN's later 10×. Exact on the F5 planar corpus (`Line`
//! carriers, `Plane` surfaces — the same inventory the M3 booleans
//! that produce 3′ bodies enforce); any other entity refuses loudly
//! ([`ValidationError::CensusUnsupported`]), never samples.
//!
//! **Certification (F1/F2(iii))**: the census never blesses — every
//! finding must be *backed* by a declaration and every declaration
//! must be *confirmed* by geometry; both directions are typed errors.
//! Structural sharing (the same key) is intent by construction and
//! needs no record (the round-8 ladder's first rung).
//!
//! # The D3 segment-reconstruction rule (derived, pinned here)
//!
//! Contact records are vertex-granularity (`VvContact`, `VfContact`).
//! Continuous overlaps — two collinear edges sharing a positive-length
//! segment, an edge resting in a face's region — are certified by
//! **reconstruction from their bounding vertex events**:
//!
//! - An **edge-edge collinear overlap** is certified iff at each of
//!   its two bounds both edges hold a vertex there and the pair is
//!   v-v-declared (or is one shared vertex — structural). Derivation:
//!   the overlap of two collinear spans is an interval whose each
//!   bound is an endpoint of at least one span; if the *other* edge
//!   has no vertex there, that endpoint rests on the other edge's
//!   interior — the vertex-on-edge lane already reports it (reduction
//!   refines every such event by splitting, so a certified result
//!   always carries the vertex). Between two backed bounds the
//!   carriers coincide identically (two lines sharing two points are
//!   one line), so the interior overlap is exactly the convex closure
//!   of the bounded events on both carriers — no interior record can
//!   carry more information than the bounds on the planar corpus.
//! - An **edge-on-face overlap** is certified iff at each bound the
//!   edge holds a vertex there and that vertex is either
//!   v-on-f-declared on this face, v-v-declared with a coincident
//!   vertex of the face's boundary, or itself a vertex of the face's
//!   boundary (structural). Same argument: an uncertified bound
//!   configuration implies a vertex-on-edge / edge-edge-cross finding
//!   that hard-errors independently.
//!
//! Failure mode: a segment overlap with a missing bounding record is
//! [`ValidationError::UndeclaredContact`] — never inferred. (A
//! configuration needing MORE than bounding records would require a
//! curved carrier — out of the planar inventory, refused as
//! `CensusUnsupported`; no counterexample exists on the F5 corpus.)
//!
//! # The D4 vertex-on-edge derivation (why there is no record type)
//!
//! Reduction's sweep splits the *other* edge at every on-edge event
//! (`split_other_at_point`) in **both** lanes that can discover one —
//! the proper-crossing lane (`FaceContainment::OnEdge`) and the
//! vertex-on-plane lane (`dovertexonface` → `OnEdge`) — so every
//! vertex-on-edge(-interior) contact is refined into a v-v record
//! before records are emitted. At rest, a vertex on an edge interior
//! is therefore always an undeclarable defect: the census reports it
//! as [`CensusContact::VertexOnEdge`] with no backing path, by design.

use std::collections::BTreeSet;

use geom_core::{Band, Decide, Point3, Real, Sign, Vec3};

use crate::body::Body;
use crate::boolean::{ContactRecords, ContainError, FaceContainment, contfp};
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, VertexKey};
use crate::null::CurveGeom;
use crate::validate::{CensusContact, StaleDeclaration, ValidationError, decide};

/// One edge's exact census geometry (post-gate: a `Line` carrier).
struct EdgeGeo<T: Real> {
    key: EdgeKey,
    v0: VertexKey,
    v1: VertexKey,
    p0: Point3<T>,
    /// Unit direction v0 → v1.
    dir: Vec3<T>,
    /// Chord length (positive on tier-2 bodies).
    len: T,
    /// The two adjacent faces (structural-adjacency exclusions).
    f_plus: FaceKey,
    f_minus: FaceKey,
}

/// One face's exact census geometry (post-gate: a `Plane`).
struct FaceGeo<T: Real> {
    key: FaceKey,
    origin: Point3<T>,
    normal: Vec3<T>,
    /// Every vertex on the face's outer loop and rings.
    boundary: BTreeSet<VertexKey>,
}

/// The census snapshot: every entity's exact geometry, or the typed
/// refusal for entities outside the planar inventory.
struct Geo<T: Real> {
    verts: Vec<(VertexKey, Point3<T>)>,
    edges: Vec<EdgeGeo<T>>,
    faces: Vec<FaceGeo<T>>,
    /// Key → position (the sweeps' random-access view of `verts`).
    vmap: std::collections::BTreeMap<VertexKey, Point3<T>>,
}

/// Runs the census and the two-direction certification diff (module
/// docs); returns every failure in deterministic sweep order. Assumes
/// tiers 1–3-local already passed (the caller gates).
pub(crate) fn census_and_certify<T: Decide>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let geo = match snapshot(body) {
        Ok(geo) => geo,
        Err(errs) => return errs,
    };
    let declared = Declared::index(contacts);
    sweep_vertex_vertex(&geo, &declared, band, &mut errors);
    sweep_vertex_edge(&geo, band, &mut errors);
    sweep_vertex_face(body, &geo, &declared, band, &mut errors);
    sweep_edge_face(body, &geo, &declared, band, &mut errors);
    sweep_edge_edge(&geo, &declared, band, &mut errors);
    confirm_declarations(body, &geo, contacts, band, &mut errors);
    errors
}

/// The declaration index: v-v pairs normalized to arena order is NOT
/// meaningful across operands, so pairs are stored unordered (both
/// orientations); v-on-f pairs merge both operand directions (which
/// operand pierced is reduction bookkeeping — in result keys the pair
/// is the declaration).
struct Declared {
    vv: BTreeSet<(VertexKey, VertexKey)>,
    vf: BTreeSet<(VertexKey, FaceKey)>,
}

impl Declared {
    fn index(contacts: &ContactRecords) -> Self {
        let mut vv = BTreeSet::new();
        for c in &contacts.vv {
            vv.insert((c.a, c.b));
            vv.insert((c.b, c.a));
        }
        let mut vf = BTreeSet::new();
        for c in contacts.a_on_b.iter().chain(&contacts.b_on_a) {
            vf.insert((c.vertex, c.face));
        }
        Self { vv, vf }
    }
}

/// Builds the exact geometry snapshot, or the typed refusals for
/// entities outside the planar inventory (all of them, deterministic).
fn snapshot<T: Decide>(body: &Body<T>) -> Result<Geo<T>, Vec<ValidationError>> {
    let mut refusals = Vec::new();
    let verts: Vec<(VertexKey, Point3<T>)> = body
        .vertices
        .iter()
        .filter_map(|(k, v)| body.points.get(v.point).map(|p| (k, *p)))
        .collect();
    let mut edges = Vec::new();
    for (key, edge) in body.edges.iter() {
        let line = body
            .curves
            .get(edge.curve)
            .and_then(CurveGeom::certified)
            .map(geom_brep::EdgeCurve::carrier)
            .is_some_and(|c| matches!(c, geom_curves::Curve3::Line { .. }));
        if !line {
            refusals.push(ValidationError::CensusUnsupported {
                entity: EntityId::Edge(key),
            });
            continue;
        }
        let ends = || -> Option<EdgeGeo<T>> {
            let plus = body.half_edges.get(edge.he_plus)?;
            let v0 = plus.start;
            let v1 = body.half_edge_end(edge.he_plus)?;
            let p0 = *body.points.get(body.vertices.get(v0)?.point)?;
            let p1 = *body.points.get(body.vertices.get(v1)?.point)?;
            let f_plus = body.loops.get(plus.parent_loop)?.face;
            let f_minus = body
                .loops
                .get(body.half_edges.get(edge.he_minus)?.parent_loop)?
                .face;
            let chord = p1 - p0;
            Some(EdgeGeo {
                key,
                v0,
                v1,
                p0,
                dir: chord.normalize(),
                len: chord.norm(),
                f_plus,
                f_minus,
            })
        };
        if let Some(geo) = ends() {
            edges.push(geo); // tier 1 guarantees resolution; silent else
        }
    }
    let mut faces = Vec::new();
    for (key, face) in body.faces.iter() {
        let Some(&geom_surfaces::Surface::Plane { origin, normal, .. }) =
            body.surfaces.get(face.surface)
        else {
            refusals.push(ValidationError::CensusUnsupported {
                entity: EntityId::Face(key),
            });
            continue;
        };
        let mut boundary = BTreeSet::new();
        for &lk in core::iter::once(&face.outer).chain(&face.rings) {
            let Some(loop_) = body.loops.get(lk) else {
                continue;
            };
            let LoopBoundary::Cycle { first } = loop_.boundary else {
                continue;
            };
            let Some(cycle) = body.loop_cycle(first) else {
                continue;
            };
            for he in cycle {
                if let Some(he_data) = body.half_edges.get(he) {
                    boundary.insert(he_data.start);
                }
            }
        }
        faces.push(FaceGeo {
            key,
            origin,
            normal,
            boundary,
        });
    }
    if refusals.is_empty() {
        let vmap = verts.iter().copied().collect();
        Ok(Geo {
            verts,
            edges,
            faces,
            vmap,
        })
    } else {
        Err(refusals)
    }
}

/// A debug rendering of a witness position (the payload posture of
/// `ResultVolumeImplausible`: display data, never load-bearing).
fn witness<T: Real>(p: Point3<T>) -> String {
    format!("{p:?}")
}

/// An impossible sign from a nonnegative margin — surfaced as the
/// invalid-margin escalation (poison posture; never silent).
fn invalid(band: Band, predicate: &'static str) -> geom_core::Indeterminate {
    geom_core::Indeterminate {
        margin: geom_core::MarginDiag::Invalid,
        band,
        predicate: Some(predicate),
    }
}

/// A nonnegative gap margin as a trilean coincidence verdict:
/// `Some(true)` coincident, `Some(false)` apart, `None` escalated
/// (already pushed).
fn gap_is_zero<T: Decide>(
    name: &'static str,
    margin: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    match decide(name, margin, band) {
        Ok(Sign::Zero) => Some(true),
        Ok(Sign::Positive) => Some(false),
        Ok(Sign::Negative) => {
            errors.push(ValidationError::CensusEscalated {
                cause: invalid(band, name),
            });
            None
        }
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// Census pass 1: vertex–vertex coincidence (all pairs, arena order).
fn sweep_vertex_vertex<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    for (i, &(ka, pa)) in geo.verts.iter().enumerate() {
        for &(kb, pb) in &geo.verts[i + 1..] {
            let Some(zero) = gap_is_zero("pm_census_vv_gap", (pa - pb).norm(), band, errors)
            else {
                continue;
            };
            if zero && !declared.vv.contains(&(ka, kb)) {
                errors.push(ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexVertex { a: ka, b: kb },
                    witness: witness(pa),
                });
            }
        }
    }
}

/// Census pass 2: vertex on an edge's **interior** — undeclarable by
/// design (module docs, D4): always a hard finding.
fn sweep_vertex_edge<T: Decide>(geo: &Geo<T>, band: Band, errors: &mut Vec<ValidationError>) {
    for &(vk, q) in &geo.verts {
        for e in &geo.edges {
            if vk == e.v0 || vk == e.v1 {
                continue; // structural adjacency
            }
            let off = (q - e.p0).cross(e.dir).norm();
            let Some(on_line) = gap_is_zero("pm_census_ve_line_gap", off, band, errors) else {
                continue;
            };
            if !on_line {
                continue;
            }
            let s = (q - e.p0).dot(e.dir);
            // Span Zero ⇒ endpoint territory (pass 1's finding); a span
            // escalation on an on-line vertex is a genuine sliver.
            let mut interior = true;
            for m in [s, e.len - s] {
                match decide("pm_census_ve_span", m, band) {
                    Ok(Sign::Positive) => {}
                    Ok(_) => interior = false,
                    Err(cause) => {
                        errors.push(ValidationError::CensusEscalated { cause });
                        interior = false;
                    }
                }
            }
            if interior {
                errors.push(ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexOnEdge {
                        vertex: vk,
                        edge: e.key,
                    },
                    witness: witness(q),
                });
            }
        }
    }
}

/// A signed residual as a trilean on-verdict: `Some(true)` on,
/// `Some(false)` definitely off, `None` escalated (pushed).
fn signed_is_zero<T: Decide>(
    name: &'static str,
    margin: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<bool> {
    match decide(name, margin, band) {
        Ok(Sign::Zero) => Some(true),
        Ok(Sign::Positive | Sign::Negative) => Some(false),
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// `contfp` with census escalation plumbing (`None` = escalated,
/// already pushed).
fn contain<T: Decide>(
    body: &Body<T>,
    f: &FaceGeo<T>,
    q: Point3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<FaceContainment> {
    match contfp(body, f.key, f.normal, q, band) {
        Ok(c) => Some(c),
        Err(ContainError::Escalated(cause)) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
        Err(_) => {
            errors.push(ValidationError::CensusEscalated {
                cause: invalid(band, "pm_census_containment"),
            });
            None
        }
    }
}

/// Census pass 3: vertex on a face's **interior** (strictly inside the
/// region — boundary coincidences are pass-1/2 findings).
fn sweep_vertex_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    for &(vk, q) in &geo.verts {
        for f in &geo.faces {
            if f.boundary.contains(&vk) {
                continue; // structural adjacency
            }
            let residual = (q - f.origin).dot(f.normal);
            match signed_is_zero("pm_census_vf_residual", residual, band, errors) {
                Some(true) => {}
                _ => continue,
            }
            // Boundary coincidences are pass-1/2 findings; Out and
            // escalations (pushed) need nothing more here.
            if contain(body, f, q, band, errors) == Some(FaceContainment::In)
                && !declared.vf.contains(&(vk, f.key))
            {
                errors.push(ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexOnFace {
                        vertex: vk,
                        face: f.key,
                    },
                    witness: witness(q),
                });
            }
        }
    }
}

/// Trilean comparison for span bookkeeping (`None` = escalated,
/// pushed).
fn tri_cmp<T: Decide>(
    a: T,
    b: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<core::cmp::Ordering> {
    match decide("pm_census_span_order", a - b, band) {
        Ok(Sign::Positive) => Some(core::cmp::Ordering::Greater),
        Ok(Sign::Negative) => Some(core::cmp::Ordering::Less),
        Ok(Sign::Zero) => Some(core::cmp::Ordering::Equal),
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            None
        }
    }
}

/// The edge's vertex at span position `s` along it, if `s` is one of
/// its ends within ε (`None`: the position is interior — the bound
/// then has no carrying vertex on this edge).
fn edge_vertex_at<T: Decide>(
    e: &EdgeGeo<T>,
    s: T,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> Option<VertexKey> {
    match signed_is_zero("pm_census_bound_end", s, band, errors) {
        Some(true) => return Some(e.v0),
        Some(false) => {}
        None => return None,
    }
    match signed_is_zero("pm_census_bound_end", e.len - s, band, errors) {
        Some(true) => Some(e.v1),
        _ => None,
    }
}

/// D3 backing for one bound of an edge-on-face overlap (module docs):
/// the edge's vertex there must be v-on-f-declared on `f`, v-v-declared
/// with a coincident boundary vertex of `f`, or itself on `f`'s
/// boundary (structural).
fn ef_bound_backed<T: Decide>(
    e: &EdgeGeo<T>,
    f: &FaceGeo<T>,
    s: T,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> bool {
    let Some(ve) = edge_vertex_at(e, s, band, errors) else {
        return false; // interior bound: the v-on-e lane already fired
    };
    if declared.vf.contains(&(ve, f.key)) || f.boundary.contains(&ve) {
        return true;
    }
    let q = e.p0 + e.dir * s;
    for &w in &f.boundary {
        let Some(&pw) = geo.vmap.get(&w) else {
            continue;
        };
        if gap_is_zero("pm_census_bound_vertex", (pw - q).norm(), band, errors) == Some(true)
            && declared.vv.contains(&(ve, w))
        {
            return true;
        }
    }
    false
}

/// Census pass 4: edge × face — transversal pierces (undeclarable) and
/// in-plane overlap segments (D3-certified).
fn sweep_edge_face<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    for e in &geo.edges {
        for f in &geo.faces {
            if f.key == e.f_plus || f.key == e.f_minus {
                continue; // structural adjacency
            }
            let p1 = e.p0 + e.dir * e.len;
            let r0 = (e.p0 - f.origin).dot(f.normal);
            let r1 = (p1 - f.origin).dot(f.normal);
            let (s0, s1) = match (
                decide("pm_census_ef_residual", r0, band),
                decide("pm_census_ef_residual", r1, band),
            ) {
                (Ok(s0), Ok(s1)) => (s0, s1),
                (a, b) => {
                    for r in [a, b] {
                        if let Err(cause) = r {
                            errors.push(ValidationError::CensusEscalated { cause });
                        }
                    }
                    continue;
                }
            };
            match (s0, s1) {
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
                    // Proper plane crossing at both-strict interiors.
                    let q = e.p0 + (p1 - e.p0) * (r0 / (r0 - r1));
                    // OnEdge → the edge-edge pass's crossing finding;
                    // OnVertex → the v-on-e finding; Out/escalated →
                    // nothing more here.
                    if contain(body, f, q, band, errors) == Some(FaceContainment::In) {
                        errors.push(ValidationError::UndeclaredContact {
                            contact: CensusContact::EdgeFacePierce {
                                edge: e.key,
                                face: f.key,
                            },
                            witness: witness(q),
                        });
                    }
                }
                (Sign::Zero, Sign::Zero) => {
                    ef_overlap_lane(body, e, f, geo, declared, band, errors);
                }
                // One endpoint on the plane: pass-1/2/3 territory.
                _ => {}
            }
        }
    }
}

/// The in-plane overlap lane of pass 4: cut the edge's span at the
/// face's coincident boundary vertices, probe each cell midpoint, and
/// D3-certify every `In` cell.
fn ef_overlap_lane<T: Decide>(
    body: &Body<T>,
    e: &EdgeGeo<T>,
    f: &FaceGeo<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let mut cuts: Vec<T> = vec![T::zero(), e.len];
    for &w in &f.boundary {
        let Some(&pw) = geo.vmap.get(&w) else {
            continue;
        };
        let off = (pw - e.p0).cross(e.dir).norm();
        if gap_is_zero("pm_census_ef_cut_gap", off, band, errors) != Some(true) {
            continue;
        }
        let s = (pw - e.p0).dot(e.dir);
        let lo = decide("pm_census_ef_cut_span", s, band);
        let hi = decide("pm_census_ef_cut_span", e.len - s, band);
        if matches!(lo, Ok(Sign::Positive)) && matches!(hi, Ok(Sign::Positive)) {
            cuts.push(s);
        }
        for r in [lo, hi] {
            if let Err(cause) = r {
                errors.push(ValidationError::CensusEscalated { cause });
            }
        }
    }
    // Insertion sort through the trilean comparator (tiny lists); an
    // escalated comparison aborts this pair's lane (already reported).
    for i in 1..cuts.len() {
        let mut j = i;
        while j > 0 {
            match tri_cmp(cuts[j - 1], cuts[j], band, errors) {
                Some(core::cmp::Ordering::Greater) => {
                    cuts.swap(j - 1, j);
                    j -= 1;
                }
                Some(_) => break,
                None => return,
            }
        }
    }
    let half = T::from_f64(0.5);
    for i in 0..cuts.len() - 1 {
        let (a, b) = (cuts[i], cuts[i + 1]);
        if !matches!(decide("pm_census_span_gap", b - a, band), Ok(Sign::Positive)) {
            continue; // empty/degenerate cell (escalations via sort/gap)
        }
        let mid = e.p0 + e.dir * ((a + b) * half);
        if contain(body, f, mid, band, errors) != Some(FaceContainment::In) {
            // Out: no overlap here. OnEdge: a collinear boundary rest —
            // the edge-edge overlap pass's finding. OnVertex: degenerate
            // cell, vertex passes cover it.
            continue;
        }
        let backed = ef_bound_backed(e, f, a, geo, declared, band, errors)
            && ef_bound_backed(e, f, b, geo, declared, band, errors);
        if !backed {
            errors.push(ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeFaceOverlap {
                    edge: e.key,
                    face: f.key,
                },
                witness: witness(mid),
            });
        }
    }
}

/// Census pass 5: edge × edge — proper interior crossings
/// (undeclarable) and collinear positive-length overlaps
/// (D3-certified at both bounds).
fn sweep_edge_edge<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    for (i, ea) in geo.edges.iter().enumerate() {
        for eb in &geo.edges[i + 1..] {
            let ncross = ea.dir.cross(eb.dir);
            match gap_is_zero("pm_census_ee_parallel", ncross.norm(), band, errors) {
                Some(false) => ee_crossing_lane(ea, eb, ncross, band, errors),
                Some(true) => ee_collinear_lane(ea, eb, declared, band, errors),
                None => {}
            }
        }
    }
}

/// Non-parallel pair: report a crossing iff the lines meet (gap zero)
/// strictly inside both spans (endpoint events are pass-1/2 findings).
fn ee_crossing_lane<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    ncross: Vec3<T>,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let d = eb.p0 - ea.p0;
    let nn = ncross.norm_squared();
    let gap = d.dot(ncross).abs() / ncross.norm();
    if gap_is_zero("pm_census_ee_gap", gap, band, errors) != Some(true) {
        return;
    }
    let sa = d.cross(eb.dir).dot(ncross) / nn;
    let sb = d.cross(ea.dir).dot(ncross) / nn;
    let mut interior = true;
    for m in [sa, ea.len - sa, sb, eb.len - sb] {
        match decide("pm_census_ee_span", m, band) {
            Ok(Sign::Positive) => {}
            Ok(_) => interior = false,
            Err(cause) => {
                errors.push(ValidationError::CensusEscalated { cause });
                interior = false;
            }
        }
    }
    if interior {
        errors.push(ValidationError::UndeclaredContact {
            contact: CensusContact::EdgeEdgeCross {
                a: ea.key,
                b: eb.key,
            },
            witness: witness(ea.p0 + ea.dir * sa),
        });
    }
}

/// Parallel pair: if collinear, the span overlap `[lo, hi]` on `ea`'s
/// axis is a finding when positive-length; certified iff both bounds
/// carry a coincident vertex pair that is v-v-declared or one shared
/// vertex (structural) — the D3 rule.
fn ee_collinear_lane<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let off = (eb.p0 - ea.p0).cross(ea.dir).norm();
    if gap_is_zero("pm_census_ee_line_gap", off, band, errors) != Some(true) {
        return;
    }
    let t0 = (eb.p0 - ea.p0).dot(ea.dir);
    let t1 = t0 + eb.len * eb.dir.dot(ea.dir);
    let (blo, bhi) = match tri_cmp(t0, t1, band, errors) {
        Some(core::cmp::Ordering::Greater) => (t1, t0),
        Some(_) => (t0, t1),
        None => return,
    };
    let lo = match tri_cmp(blo, T::zero(), band, errors) {
        Some(core::cmp::Ordering::Greater) => blo,
        Some(_) => T::zero(),
        None => return,
    };
    let hi = match tri_cmp(bhi, ea.len, band, errors) {
        Some(core::cmp::Ordering::Less) => bhi,
        Some(_) => ea.len,
        None => return,
    };
    match decide("pm_census_ee_overlap", hi - lo, band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return, // point/empty overlap: pass-1 territory
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            return;
        }
    }
    let backed = ee_bound_backed(ea, eb, lo, declared, band, errors)
        && ee_bound_backed(ea, eb, hi, declared, band, errors);
    if !backed {
        let half = T::from_f64(0.5);
        errors.push(ValidationError::UndeclaredContact {
            contact: CensusContact::EdgeEdgeOverlap {
                a: ea.key,
                b: eb.key,
            },
            witness: witness(ea.p0 + ea.dir * ((lo + hi) * half)),
        });
    }
}

/// D3 backing for one bound of a collinear overlap: both edges hold a
/// vertex at the bound and the pair is declared (or is one shared
/// vertex — structural).
fn ee_bound_backed<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    s: T,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> bool {
    let Some(va) = edge_vertex_at(ea, s, band, errors) else {
        return false; // interior on ea: the v-on-e lane already fired
    };
    let q = ea.p0 + ea.dir * s;
    let sb = (q - eb.p0).dot(eb.dir);
    let Some(vb) = edge_vertex_at(eb, sb, band, errors) else {
        return false;
    };
    va == vb || declared.vv.contains(&(va, vb))
}

/// The confirmation direction of the certification diff: every
/// declaration must have a geometric witness — dead keys, equal keys,
/// and coincidence-free records are stale, typed.
fn confirm_declarations<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    contacts: &ContactRecords,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    for c in &contacts.vv {
        let stale = ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::VertexVertex { a: c.a, b: c.b },
        };
        let (Some(&pa), Some(&pb)) = (geo.vmap.get(&c.a), geo.vmap.get(&c.b)) else {
            errors.push(stale);
            continue;
        };
        if c.a == c.b {
            errors.push(stale); // consumed into one vertex: not a contact
            continue;
        }
        match gap_is_zero("pm_census_confirm_vv", (pa - pb).norm(), band, errors) {
            Some(true) | None => {}
            Some(false) => errors.push(stale),
        }
    }
    for c in contacts.a_on_b.iter().chain(&contacts.b_on_a) {
        let stale = ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::VertexOnFace {
                vertex: c.vertex,
                face: c.face,
            },
        };
        let (Some(&q), Some(f)) = (
            geo.vmap.get(&c.vertex),
            geo.faces.iter().find(|f| f.key == c.face),
        ) else {
            errors.push(stale);
            continue;
        };
        match signed_is_zero("pm_census_confirm_vf", (q - f.origin).dot(f.normal), band, errors)
        {
            Some(true) => {}
            Some(false) => {
                errors.push(stale);
                continue;
            }
            None => continue,
        }
        match contain(body, f, q, band, errors) {
            Some(FaceContainment::In) => {}
            Some(_) => errors.push(stale),
            None => {}
        }
    }
}
