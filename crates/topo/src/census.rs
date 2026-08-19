//! The tier-3′ **global coincidence census** + declared-contact
//! certification (M3 PR 6a; F1/F2) — the injectivity pass tier 3
//! defers, run at rest against a body's declared-contact records.
//!
//! **Sweep shape**: quadratic all-pairs sweeps in arena order
//! (vertex×vertex, vertex×edge, vertex×face, edge×face, edge×edge) —
//! the boolean edge×face convention: correctness first, the BVH filter
//! is PERF-PLAN's later 10×. Exact on the F5 planar subset (`Line`
//! carriers, `Plane` surfaces). **Since M9-2 the census ADMITS every
//! carrier kind**, and its reach is exactly this, stated class by
//! class (the union fix's truth pass):
//!
//! - **Examined and decided**: everything with vertex/line/planar
//!   evidence (the exact sweeps); same-key opposed-sense curved
//!   pairs (the conformal arm, [`sweep_conformal_patches`], through
//!   the chart-region predicate); declared curve/patch records (the
//!   jet schedule; the patch certifier). A proper pierce
//!   (`EdgeFacePierce`) is CATEGORICALLY undeclarable — 3′ allows
//!   touching, never crossing; the recourse is separating the
//!   bodies or making the crossing a boolean's working state.
//! - **Refused as UNDECIDABLE** (the conservative loudness backstop,
//!   [`sweep_cross_solid_backstop`]): cross-solid face pairs with a
//!   curved side within reach — curved × curved (the conformal
//!   cradle, boss-in-hole) and curved × planar (the embedded ball
//!   cap, F5) — including EVERY pair with a cone/torus/NURBS side
//!   (no sound cheap reach bound exists: refused without a distance
//!   test); a cross-key `PatchContact` on such a pair ESCALATES
//!   through the chart predicate's divergence posture, so the class
//!   can today be neither certified nor silently passed. And one
//!   instance's extent box contained in another's (the
//!   nested-instance class — C6's interference, representable only
//!   through recorded gate-skips that do not exist yet).
//! - **Genuinely undetected until C9/C6**: SAME-solid distinct-key
//!   curved pairs only (the backstop is cross-solid — a single
//!   solid's own curved faces are its constructor's obligations).
//!   Cross-solid pairs the reach filter CLEARS are cleared soundly
//!   (the pads are sound bounds for the kinds that take the test),
//!   so clearance is a genuine no-touch certificate, not a skip.
//!   Named, not sampled.
//!
//! A record or candidate outside a certifier's lane refuses
//! [`ValidationError::CensusUnsupported`], never samples.
//!
//! **Sense-invariant** (M5 S10 audit). Every use of a face's plane
//! `normal` here is either an on-plane residual compared against
//! `Sign::Zero` (does this point/segment LIE on the face's carrier?)
//! or the in-plane frame of a ray-crossing PARITY count
//! (`contfp`-style containment). Neither reads a side: negating the
//! normal leaves a zero residual zero and leaves a crossing count
//! unchanged. So the census needs no `sense_sign`, and multiplying
//! one in would be noise, not caution — coincidence is a question
//! about position, never about which way the material lies.
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

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

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

/// The census snapshot: the exact planar geometry the vertex-granular
/// sweeps read, plus the curved inventory the face-granular arms own
/// (M9-2: the census admits every carrier kind; what changes per kind
/// is WHICH arm can certify it — module docs).
struct Geo<T: Real> {
    verts: Vec<(VertexKey, Point3<T>)>,
    edges: Vec<EdgeGeo<T>>,
    faces: Vec<FaceGeo<T>>,
    /// Key → position (the sweeps' random-access view of `verts`).
    vmap: std::collections::BTreeMap<VertexKey, Point3<T>>,
    /// Faces on non-`Plane` carriers — outside the exact planar
    /// sweeps, inside the conformal face-pair arm.
    curved_faces: Vec<FaceKey>,
    /// Vertex → the faces whose boundary holds it (EVERY face, curved
    /// included) — the face-granularity backing rung's incidence.
    vertex_faces: std::collections::BTreeMap<VertexKey, BTreeSet<FaceKey>>,
}

/// Runs the census and the two-direction certification diff (module
/// docs); returns every failure in deterministic sweep order. Assumes
/// tiers 1–3-local already passed (the caller gates).
pub(crate) fn census_and_certify<T: Decide + crate::chart_region::ChartRegionLane>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let geo = snapshot(body);
    let declared = Declared::index(contacts);
    sweep_vertex_vertex(&geo, &declared, band, &mut errors);
    sweep_vertex_edge(&geo, band, &mut errors);
    sweep_vertex_face(body, &geo, &declared, band, &mut errors);
    sweep_edge_face(body, &geo, &declared, band, &mut errors);
    sweep_edge_edge(&geo, &declared, band, &mut errors);
    sweep_conformal_patches(body, &geo, &declared, band, &mut errors);
    sweep_cross_solid_backstop(body, &geo, &declared, band, &mut errors);
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
    /// Face-granularity keys (M9-2): the face pairs the body's
    /// curve/patch records name, both orientations. A face-pair
    /// record backs the vertex-granular events SUBORDINATE to it —
    /// a boundary vertex of one declared face resting on the other,
    /// the coincident vertex pairs and segment bounds along their
    /// interface — exactly as a v-v/v-f declaration backs its own
    /// event; the record's own geometric confirmation is the confirm
    /// pass's (two-directional, as ever).
    faces: BTreeSet<(FaceKey, FaceKey)>,
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
        let mut faces = BTreeSet::new();
        for (a, b) in contacts
            .curves
            .iter()
            .map(|c| (c.face_a, c.face_b))
            .chain(contacts.patches.iter().map(|c| (c.face_a, c.face_b)))
        {
            faces.insert((a, b));
            faces.insert((b, a));
        }
        Self { vv, vf, faces }
    }

    /// The face rung for a v-v event: some declared face pair holds
    /// `a` on one boundary and `b` on the other.
    fn vv_face_backed(&self, geo: &Geo<impl Real>, a: VertexKey, b: VertexKey) -> bool {
        let (Some(fa), Some(fb)) = (geo.vertex_faces.get(&a), geo.vertex_faces.get(&b)) else {
            return false;
        };
        fa.iter()
            .any(|&ga| fb.iter().any(|&gb| self.faces.contains(&(ga, gb))))
    }

    /// The face rung for a v-on-f event: some declared face pair
    /// holds `v` on its boundary and names `f` as the other side.
    fn vf_face_backed(&self, geo: &Geo<impl Real>, v: VertexKey, f: FaceKey) -> bool {
        geo.vertex_faces
            .get(&v)
            .is_some_and(|gs| gs.iter().any(|&g| self.faces.contains(&(g, f))))
    }
}

/// Builds the geometry snapshot: exact planar entities for the
/// vertex-granular sweeps, curved entities routed to the
/// face-granular arms (M9-2 — the census ADMITS every carrier kind;
/// the blanket exact-on-planar refusal retired with the census arms
/// that replaced it, and what each arm can and cannot certify is the
/// module-docs envelope, stated rather than sampled). Total: every
/// entity lands in exactly one bucket, so there is no refusal path.
fn snapshot<T: Decide>(body: &Body<T>) -> Geo<T> {
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
            // A curved-carrier edge is outside the exact sweeps; its
            // contact obligations ride the face-granular records
            // (CurveContact's confirm pass) — no blanket refusal.
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
    let mut curved_faces = Vec::new();
    let mut vertex_faces: std::collections::BTreeMap<VertexKey, BTreeSet<FaceKey>> =
        std::collections::BTreeMap::new();
    for (key, face) in body.faces.iter() {
        let plane = match body.surfaces.get(face.surface) {
            Some(&geom_surfaces::Surface::Plane { origin, normal, .. }) => Some((origin, normal)),
            _ => {
                curved_faces.push(key);
                None
            }
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
        // Incidence for the face-granularity backing rung — EVERY
        // face, curved included (a curved declared face's boundary
        // vertices are exactly the ones its record must back).
        for &v in &boundary {
            vertex_faces.entry(v).or_default().insert(key);
        }
        if let Some((origin, normal)) = plane {
            faces.push(FaceGeo {
                key,
                origin,
                normal,
                boundary,
            });
        }
    }
    let vmap = verts.iter().copied().collect();
    Geo {
        verts,
        edges,
        faces,
        vmap,
        curved_faces,
        vertex_faces,
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
    margin: Margin<T>,
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
            let Some(zero) = gap_is_zero("pm_census_vv_gap", Margin::norm3(pa - pb), band, errors)
            else {
                continue;
            };
            if zero && !declared.vv.contains(&(ka, kb)) && !declared.vv_face_backed(geo, ka, kb) {
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
            let off = Margin::norm3((q - e.p0).cross(e.dir));
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
                match decide("pm_census_ve_span", Margin::of(m), band) {
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
    margin: Margin<T>,
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
            match signed_is_zero("pm_census_vf_residual", Margin::of(residual), band, errors) {
                Some(true) => {}
                _ => continue,
            }
            // Boundary coincidences are pass-1/2 findings; Out and
            // escalations (pushed) need nothing more here.
            if contain(body, f, q, band, errors) == Some(FaceContainment::In)
                && !declared.vf.contains(&(vk, f.key))
                && !declared.vf_face_backed(geo, vk, f.key)
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
    match decide("pm_census_span_order", Margin::of(a - b), band) {
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
    match signed_is_zero("pm_census_bound_end", Margin::of(s), band, errors) {
        Some(true) => return Some(e.v0),
        Some(false) => {}
        None => return None,
    }
    match signed_is_zero("pm_census_bound_end", Margin::of(e.len - s), band, errors) {
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
    if declared.vf.contains(&(ve, f.key))
        || f.boundary.contains(&ve)
        || declared.vf_face_backed(geo, ve, f.key)
    {
        return true;
    }
    let q = e.p0 + e.dir * s;
    for &w in &f.boundary {
        let Some(&pw) = geo.vmap.get(&w) else {
            continue;
        };
        if gap_is_zero(
            "pm_census_bound_vertex",
            Margin::norm3(pw - q),
            band,
            errors,
        ) == Some(true)
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
                decide("pm_census_ef_residual", Margin::of(r0), band),
                decide("pm_census_ef_residual", Margin::of(r1), band),
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
        let off = Margin::norm3((pw - e.p0).cross(e.dir));
        if gap_is_zero("pm_census_ef_cut_gap", off, band, errors) != Some(true) {
            continue;
        }
        let s = (pw - e.p0).dot(e.dir);
        let lo = decide("pm_census_ef_cut_span", Margin::of(s), band);
        let hi = decide("pm_census_ef_cut_span", Margin::of(e.len - s), band);
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
        if !matches!(
            decide("pm_census_span_gap", Margin::of(b - a), band),
            Ok(Sign::Positive)
        ) {
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
            // sin(angle of the unit dirs) × the shorter edge's own
            // length: the displacement the angular deviation induces
            // over the edge (a bare sine was a dimensionless comparand
            // against the length band — rim-dimensional audit, (c)).
            let arm = ea.len.min(eb.len);
            match gap_is_zero(
                "pm_census_ee_parallel",
                Margin::levered(ncross.norm(), arm),
                band,
                errors,
            ) {
                Some(false) => ee_crossing_lane(ea, eb, ncross, band, errors),
                Some(true) => ee_collinear_lane(ea, eb, geo, declared, band, errors),
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
    let gap = Margin::levered_inv(d.dot(ncross).abs(), ncross.norm());
    if gap_is_zero("pm_census_ee_gap", gap, band, errors) != Some(true) {
        return;
    }
    let sa = d.cross(eb.dir).dot(ncross) / nn;
    let sb = d.cross(ea.dir).dot(ncross) / nn;
    let mut interior = true;
    for m in [sa, ea.len - sa, sb, eb.len - sb] {
        match decide("pm_census_ee_span", Margin::of(m), band) {
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
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    let off = Margin::norm3((eb.p0 - ea.p0).cross(ea.dir));
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
    match decide("pm_census_ee_overlap", Margin::of(hi - lo), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return, // point/empty overlap: pass-1 territory
        Err(cause) => {
            errors.push(ValidationError::CensusEscalated { cause });
            return;
        }
    }
    let backed = ee_bound_backed(ea, eb, lo, geo, declared, band, errors)
        && ee_bound_backed(ea, eb, hi, geo, declared, band, errors);
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
    geo: &Geo<T>,
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
    va == vb || declared.vv.contains(&(va, vb)) || declared.vv_face_backed(geo, va, vb)
}

/// **The conformal face-pair arm** (M9-2, C2's structural rung run as
/// a census sweep): for every pair of CURVED faces sharing one
/// `SurfaceKey` with OPPOSED senses — the only configuration that is
/// conformal contact (C1: aligned coincidence is containment/flush,
/// `SameOriented`, and is not contact) — the trim regions' overlap is
/// decided in the shared chart through the PR-1 predicate.
///
/// Scope, stated exactly:
///
/// - **Curved faces only.** Planar conformal interfaces are already
///   fully evidenced at vertex granularity by the exact sweeps (their
///   every boundary event is a v-v/v-f/segment finding those passes
///   report and records back), so the face-pair arm exists for the
///   inventory the exact sweeps cannot read.
/// - **Shared key only.** C2's identity lemma makes every true
///   conformal contact same-carrier for analytic kinds, and within
///   ONE body the structural rung IS key identity; value-equal
///   independent descriptions do not glue (the ladder), and the
///   declared rung's face pairs are the confirm pass's business.
/// - **What stays outside this arm** (module docs carry the full
///   envelope): distinct-key pairs. Cross-solid ones are caught by
///   the conservative loudness backstop
///   ([`sweep_cross_solid_backstop`] — refused undecidable, never
///   silently passed); same-solid distinct-key pairs and pure
///   tangency classes are the named residue until the C9 exclusion
///   ring lands (CONTACT-DESIGN C2 step 1's missing first step).
///
/// A definitely-positive overlap is a FINDING — the kernel
/// vocabulary's [`crate::contact::ContactFinding`], carried on the
/// refusal so the recourse can quote exactly what would verify —
/// and, unbacked by a face-granularity record, refuses as
/// `UndeclaredContact` (discovery is never declaration, F1).
fn sweep_conformal_patches<T: Decide + crate::chart_region::ChartRegionLane>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    use crate::geometry::SurfaceKey;
    // Group the curved faces by carrier key (arena order, D9).
    let mut by_key: std::collections::BTreeMap<SurfaceKey, Vec<FaceKey>> =
        std::collections::BTreeMap::new();
    for &f in &geo.curved_faces {
        if let Some(face) = body.get_face(f) {
            by_key.entry(face.surface).or_default().push(f);
        }
    }
    for group in by_key.values() {
        for (i, &fa) in group.iter().enumerate() {
            for &fb in &group[i + 1..] {
                let (Some(da), Some(db)) = (body.get_face(fa), body.get_face(fb)) else {
                    continue;
                };
                if da.sense == db.sense {
                    continue; // SameOriented: flush material, not contact (C1)
                }
                match T::chart_overlap(body, fa, body, fb, band) {
                    None => {
                        // No bracket lane at this scalar (dual): the
                        // pair cannot be decided — typed, never silent.
                        errors.push(ValidationError::CensusUnsupported {
                            entity: EntityId::Face(fa),
                        });
                    }
                    Some(Ok(crate::chart_region::ChartOverlap::Empty)) => {}
                    Some(Ok(crate::chart_region::ChartOverlap::PositiveArea)) => {
                        if !declared.faces.contains(&(fa, fb)) {
                            let finding = crate::contact::ContactFinding {
                                pair: crate::contact::DeclaredContact {
                                    a: fa,
                                    b: fb,
                                    class: crate::contact::ContactClass::Rest,
                                },
                                verdict: crate::contact::ContactVerdict::Definite,
                            };
                            errors.push(ValidationError::UndeclaredContact {
                                contact: CensusContact::ConformalPatch { finding },
                                witness: format!("{fa:?}~{fb:?}"),
                            });
                        }
                    }
                    Some(Err(crate::chart_region::ChartRegionError::Escalated(cause))) => {
                        errors.push(ValidationError::CensusEscalated { cause });
                    }
                    Some(Err(_)) => {
                        // Typed predicate refusals (unbounded arms, seam
                        // branches, non-planar trims, touching
                        // boundaries): the pair is outside the certified
                        // overlap lane — refused as unsupported
                        // inventory, never skipped silently.
                        errors.push(ValidationError::CensusUnsupported {
                            entity: EntityId::Face(fa),
                        });
                    }
                }
            }
        }
    }
}

/// **The conservative loudness backstop** (M9-2 union fix F1): the
/// census must DECIDE or REFUSE — it must never silently not-examine
/// (A5's letter). Two cross-solid candidate classes have no examining
/// arm yet, and both fire a typed [`ValidationError::CensusUndecidable`]
/// here instead of passing silently:
///
/// 1. **Proximity** (the C9-ring conformal-rest / partial-embedding
///    class): a pair of faces from DIFFERENT solids, at least one
///    curved (F5: curved × planar included — a revolved cap embedded
///    in a plate's slab leaves no vertex/line/planar evidence), not
///    vertex-adjacent, whose reach boxes cannot be definitely
///    separated. Distinct-key value-equal carriers in conformal rest
///    (the cradle witness), value-equal walls at gap zero
///    (boss-in-hole), and the embedded ball cap (the delta witness)
///    land here; the certified excluder this stands in for is the C9
///    exclusion ring. The reach boxes are SOUND per-kind supersets:
///    `reach_box` is [`crate::boolean::boxes::FaceBoxRule`] — the ONE
///    face-box rule — instantiated in this lane's arithmetic (the
///    closure's own comment says why the arithmetic, and only the
///    arithmetic, is separate). A kind with no cheap sound box refuses
///    WITHOUT a distance test rather than under-claiming its reach. A
///    planar face vf-NAMED by the other solid's records defers to the
///    confirm pass (the declared boss-on-plate class).
/// 2. **Instance containment** (C6's interference class): one solid's
///    vertex-extent box inside another's REACH box (the containing
///    side must be a superset; the contained side is a subset of its
///    own locus, which is what makes a clear sound) — a nested placement
///    makes no boundary event at all (the reviewed nested-cube
///    witness), and interference is representable only through C6's
///    recorded gate-skips, which do not exist yet.
///
/// Both arms clear a pair ONLY on a definitely-positive separation
/// margin (`census_backstop_gap` / `census_backstop_containment` —
/// metre coordinate differences); anything weaker refuses.
///
/// A planar × planar pair is skipped only when BOTH faces are bounded
/// entirely by line edges (§S49). That — not the kind of solid the
/// faces sit on — is what the skip's premise is about: [`snapshot`]
/// admits line edges and drops every curved one, so a wholly
/// line-bounded planar face has its whole boundary in front of the
/// exact sweeps, and two of them cannot overlap without leaving an
/// event those sweeps read (either the boundaries cross, a line ×
/// line event, or one face's boundary lies inside the other, a
/// vertex-on-face event). A planar face with a curved boundary — a
/// cylinder's cap — leaves neither: its rim is not in the snapshot,
/// so a cap can overlap another face with nothing to see at
/// vertex/line granularity. Such a pair is THIS arm's: the conformal
/// arm takes same-carrier CURVED faces only, and the confirm pass
/// takes declared pairs only, so the undeclared cap pair has no other
/// home. Its plane is boxable ([`crate::boolean::boxes::FaceBoxRule`]
/// hulls the boundary's certified edge reaches, arcs included), so
/// this arm can clear the separated majority instead of refusing
/// every one of them.
///
/// **Jurisdiction, exactly** (a backstop refuses only what NO arm
/// examines): same-`SurfaceKey` CURVED face pairs are the conformal
/// arm's candidates and are skipped here — curved, because that arm
/// groups [`Geo::curved_faces`], so a same-key PLANAR pair is not on
/// its list and stays here; record-NAMED face pairs are the
/// patch/curve certifier's (a cross-key record escalates loudly
/// there — skipping it here loses no loudness); and a solid pair
/// BRIDGED by any contact record (vv, v-on-f, curve, patch) is
/// under the confirm pass's examination — its records either
/// confirm (the declared touching class) or error as
/// stale/contradicted, so the containment arm defers to that
/// verdict rather than double-refusing a certified assembly.
fn sweep_cross_solid_backstop<T: Decide>(
    body: &Body<T>,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    use crate::entity::{FaceKey as FK, ShellKey, SolidKey};
    if body.solids().count() < 2 {
        return; // single-solid bodies have no cross-solid pairs
    }
    let solid_of = |f: FK| -> Option<SolidKey> {
        let shell: ShellKey = body.get_face(f)?.shell;
        Some(body.get_shell(shell)?.solid)
    };
    // Boundary-vertex hull of a face (every loop), as raw points.
    let face_points = |f: FK| -> Vec<Point3<T>> {
        let mut out = Vec::new();
        let Some(face) = body.get_face(f) else {
            return out;
        };
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
                if let Some(hd) = body.half_edges.get(he)
                    && let Some(v) = body.vertices.get(hd.start)
                    && let Some(p) = body.points.get(v.point)
                {
                    out.push(*p);
                }
            }
        }
        out
    };
    // Every boundary edge of `f` carries a certified LINE. That is
    // the property the planar × planar skip rests on: [`snapshot`]
    // keeps line edges and drops curved ones, so this and only this
    // puts a face's whole boundary in front of the exact sweeps.
    // Anything unresolvable — a missing loop, an isolated-vertex
    // boundary, an uncertified carrier — is not a line, so the face
    // stays with this arm.
    let line_bounded = |f: FK| -> bool {
        let Some(face) = body.get_face(f) else {
            return false;
        };
        for &lk in core::iter::once(&face.outer).chain(&face.rings) {
            let Some(loop_) = body.loops.get(lk) else {
                return false;
            };
            let LoopBoundary::Cycle { first } = loop_.boundary else {
                return false;
            };
            let Some(cycle) = body.loop_cycle(first) else {
                return false;
            };
            for he in cycle {
                let is_line = body
                    .half_edges
                    .get(he)
                    .and_then(|hd| body.edges.get(hd.edge))
                    .and_then(|e| body.curves.get(e.curve))
                    .and_then(CurveGeom::certified)
                    .map(geom_brep::EdgeCurve::carrier)
                    .is_some_and(|c| matches!(c, geom_curves::Curve3::Line { .. }));
                if !is_line {
                    return false;
                }
            }
        }
        true
    };
    let hull = |pts: &[Point3<T>]| -> Option<(Point3<T>, Point3<T>)> {
        let mut it = pts.iter();
        let first = *it.next()?;
        let (mut lo, mut hi) = (first, first);
        for p in it {
            lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
        Some((lo, hi))
    };
    // **`boolean::boxes`'s `FaceBoxRule`/`EdgeBoxRule`, instantiated in
    // this lane's arithmetic.** The rules — which kinds have a cheap
    // sound superset, and by what construction — are read from there;
    // only the min/max is re-derived, and it must be: `face_box` reads
    // `[lo(), hi()]` brackets under a per-file `Bounds` allowlist
    // (geom-core `real.rs`) that this lane is not on and cannot join,
    // because the census validates `Dual` bodies and `Dual` has no
    // bracket. An unboxable kind is `None` here and poison there —
    // refuse without a distance test, versus never prune. Both loud.
    let edge_reach = |ek: crate::entity::EdgeKey| -> Option<(Point3<T>, Point3<T>)> {
        let e = body.edges.get(ek)?;
        let end = |he| -> Option<Point3<T>> {
            let hd = body.half_edges.get(he)?;
            let v = body.vertices.get(hd.start)?;
            body.points.get(v.point).copied()
        };
        let (a, b) = (end(e.he_plus)?, end(e.he_minus)?);
        let chord = (
            Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        );
        let carrier = body
            .curves
            .get(e.curve)
            .and_then(CurveGeom::certified)
            .map(geom_brep::EdgeCurve::carrier);
        match crate::boolean::boxes::edge_box_rule(carrier) {
            crate::boolean::boxes::EdgeBoxRule::NoSoundBox => None,
            crate::boolean::boxes::EdgeBoxRule::Chord => Some(chord),
            crate::boolean::boxes::EdgeBoxRule::ConicAmplitude {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => {
                let v_ref = axis.cross(u_ref);
                let reach = |ui: T, vi: T| ui.abs() * semi_u + vi.abs() * semi_v;
                let (rx, ry, rz) = (
                    reach(u_ref.x, v_ref.x),
                    reach(u_ref.y, v_ref.y),
                    reach(u_ref.z, v_ref.z),
                );
                Some((
                    Point3::new(
                        (center.x - rx).min(chord.0.x),
                        (center.y - ry).min(chord.0.y),
                        (center.z - rz).min(chord.0.z),
                    ),
                    Point3::new(
                        (center.x + rx).max(chord.1.x),
                        (center.y + ry).max(chord.1.y),
                        (center.z + rz).max(chord.1.z),
                    ),
                ))
            }
        }
    };
    // Every boundary edge's reach, hulled with the isolated-vertex
    // loops (which have no edge to speak for them). `None` as soon as
    // one boundary curve has no sound box.
    let boundary_reach = |f: FK| -> Option<(Point3<T>, Point3<T>)> {
        let face = body.get_face(f)?;
        let mut acc: Option<(Point3<T>, Point3<T>)> = None;
        let mut grow = |(lo, hi): (Point3<T>, Point3<T>)| {
            acc = Some(match acc {
                None => (lo, hi),
                Some((l, h)) => (
                    Point3::new(l.x.min(lo.x), l.y.min(lo.y), l.z.min(lo.z)),
                    Point3::new(h.x.max(hi.x), h.y.max(hi.y), h.z.max(hi.z)),
                ),
            });
        };
        for &lk in core::iter::once(&face.outer).chain(&face.rings) {
            let l = body.loops.get(lk)?;
            match l.boundary {
                LoopBoundary::Empty { vertex } => {
                    let v = body.vertices.get(vertex)?;
                    let p = *body.points.get(v.point)?;
                    grow((p, p));
                }
                LoopBoundary::Cycle { first } => {
                    for he in body.loop_cycle(first)? {
                        let ek = body.half_edges.get(he)?.edge;
                        grow(edge_reach(ek)?);
                    }
                }
            }
        }
        acc
    };
    let reach_box = |f: FK| -> Option<(Point3<T>, Point3<T>)> {
        let surface = body
            .get_face(f)
            .and_then(|d| body.surfaces.get(d.surface))?;
        match crate::boolean::boxes::face_box_rule(surface) {
            crate::boolean::boxes::FaceBoxRule::NoSoundBox => None,
            crate::boolean::boxes::FaceBoxRule::BoundaryHull => boundary_reach(f),
            crate::boolean::boxes::FaceBoxRule::ControlNet(patch) => {
                let mut it = patch.control().iter();
                let first = *it.next()?;
                let (mut lo, mut hi) = (first, first);
                for p in it {
                    lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
                    hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
                }
                Some((lo, hi))
            }
            crate::boolean::boxes::FaceBoxRule::WholeBall { center, radius } => Some((
                Point3::new(center.x - radius, center.y - radius, center.z - radius),
                Point3::new(center.x + radius, center.y + radius, center.z + radius),
            )),
            crate::boolean::boxes::FaceBoxRule::CylinderSlab {
                origin,
                axis,
                radius,
            } => {
                // The axial extent comes from the boundary's own
                // reach, not its vertices: the axial coordinate is
                // linear along the surface, so the face's axial
                // extremes lie ON the boundary — but not necessarily
                // at a boundary VERTEX.
                let (blo, bhi) = boundary_reach(f)?;
                let corners = [
                    Point3::new(blo.x, blo.y, blo.z),
                    Point3::new(bhi.x, bhi.y, bhi.z),
                    Point3::new(blo.x, blo.y, bhi.z),
                    Point3::new(blo.x, bhi.y, blo.z),
                    Point3::new(bhi.x, blo.y, blo.z),
                    Point3::new(blo.x, bhi.y, bhi.z),
                    Point3::new(bhi.x, blo.y, bhi.z),
                    Point3::new(bhi.x, bhi.y, blo.z),
                ];
                let proj = |p: Point3<T>| {
                    (p.x - origin.x) * axis.x
                        + (p.y - origin.y) * axis.y
                        + (p.z - origin.z) * axis.z
                };
                let mut it = corners.iter();
                let h0 = proj(*it.next()?);
                let (mut h_min, mut h_max) = (h0, h0);
                for p in it {
                    let h = proj(*p);
                    h_min = h_min.min(h);
                    h_max = h_max.max(h);
                }
                let a = origin + axis * h_min;
                let b = origin + axis * h_max;
                let r = radius;
                Some((
                    Point3::new(a.x.min(b.x) - r, a.y.min(b.y) - r, a.z.min(b.z) - r),
                    Point3::new(a.x.max(b.x) + r, a.y.max(b.y) + r, a.z.max(b.z) + r),
                ))
            }
        }
    };

    // Arm 1: cross-solid proximity — curved × curved, (F5) curved ×
    // planar, and the planar × planar pairs the exact sweeps cannot
    // see (§S49: a pair is theirs only when BOTH faces are wholly
    // line-bounded, which is what puts a whole boundary in the
    // snapshot — the module docs carry the argument).
    struct Reach<T: Real> {
        face: crate::entity::FaceKey,
        solid: crate::entity::SolidKey,
        /// The sound reach box (`reach_box` docs), or `None` for a
        /// kind with no cheap sound box — its pairs refuse without a
        /// distance test.
        boxed: Option<(Point3<T>, Point3<T>)>,
        verts: BTreeSet<VertexKey>,
        planar: bool,
        /// Planar AND bounded entirely by line edges, so the exact
        /// sweeps hold this face's whole boundary.
        line_bounded: bool,
    }
    let mut reaches: Vec<Reach<T>> = Vec::new();
    let planar_keys: Vec<FaceKey> = geo.faces.iter().map(|f| f.key).collect();
    for (&f, planar) in geo
        .curved_faces
        .iter()
        .map(|f| (f, false))
        .chain(planar_keys.iter().map(|f| (f, true)))
    {
        // A placeholder surface is "no description yet" (mid-surgery
        // scaffolding): there is no geometry to be within reach OF,
        // and a body carrying one never reaches 3′ (the tier-3 local
        // battery refuses it first) — excluded from the backstop.
        if matches!(
            body.get_face(f).and_then(|d| body.surfaces.get(d.surface)),
            Some(geom_surfaces::Surface::Nurbs(p)) if p.is_placeholder()
        ) {
            continue;
        }
        let Some(solid) = solid_of(f) else { continue };
        let pts = face_points(f);
        if pts.is_empty() {
            continue;
        }
        let verts = geo
            .vertex_faces
            .iter()
            .filter(|(_, fs)| fs.contains(&f))
            .map(|(&v, _)| v)
            .collect();
        let boxed = reach_box(f);
        reaches.push(Reach {
            face: f,
            solid,
            boxed,
            verts,
            planar,
            line_bounded: planar && line_bounded(f),
        });
    }
    // The record-bridged solid pairs (doc comment): any record
    // linking a vertex/face of one solid to a vertex/face of another
    // puts that PAIR under the confirm pass's jurisdiction.
    let vertex_solid = |v: VertexKey| -> Option<SolidKey> {
        geo.vertex_faces
            .get(&v)
            .and_then(|fs| fs.iter().next())
            .and_then(|&f| solid_of(f))
    };
    let mut bridged: BTreeSet<(SolidKey, SolidKey)> = BTreeSet::new();
    let mut bridge = |sa: Option<SolidKey>, sb: Option<SolidKey>| {
        if let (Some(sa), Some(sb)) = (sa, sb) {
            bridged.insert((sa, sb));
            bridged.insert((sb, sa));
        }
    };
    for &(va, vb) in &declared.vv {
        bridge(vertex_solid(va), vertex_solid(vb));
    }
    for &(v, f) in &declared.vf {
        bridge(vertex_solid(v), solid_of(f));
    }
    for &(fa, fb) in &declared.faces {
        bridge(solid_of(fa), solid_of(fb));
    }

    // The vf-record deferral for the curved × planar arm (F5): a
    // planar face NAMED by a v-on-f record whose vertex belongs to
    // the other solid is the declared interface the confirm pass and
    // the exact sweeps examine (the boss-on-plate acceptance class) —
    // its curved neighbours defer to that verdict; a bogus record
    // still errors there as stale, so nothing blesses silently.
    let planar_face_bridged = |f_planar: FK, other: SolidKey| -> bool {
        declared
            .vf
            .iter()
            .any(|&(v, vf)| vf == f_planar && vertex_solid(v) == Some(other))
    };
    for (i, a) in reaches.iter().enumerate() {
        for b in &reaches[i + 1..] {
            if a.solid == b.solid || !a.verts.is_disjoint(&b.verts) {
                continue; // same instance / structural adjacency
            }
            if a.planar && b.planar && a.line_bounded && b.line_bounded {
                continue; // both boundaries in the snapshot — the exact sweeps' pair
            }
            let same_key = body
                .get_face(a.face)
                .zip(body.get_face(b.face))
                .is_some_and(|(da, db)| da.surface == db.surface);
            // The conformal arm groups CURVED faces by carrier, so a
            // same-key planar pair is on no list but this one.
            if (same_key && !a.planar && !b.planar) || declared.faces.contains(&(a.face, b.face)) {
                continue; // the conformal arm's / the certifier's pair
            }
            let vf_deferred = (a.planar && planar_face_bridged(a.face, b.solid))
                || (b.planar && planar_face_bridged(b.face, a.solid));
            if vf_deferred {
                continue; // the declared interface — confirm pass's pair
            }
            // A kind with no sound reach box refuses without a
            // distance test (reach_box docs — the loud direction).
            let (Some((alo, ahi)), Some((blo, bhi))) = (a.boxed, b.boxed) else {
                errors.push(ValidationError::CensusUndecidable {
                    a: EntityId::Face(a.face),
                    b: EntityId::Face(b.face),
                    what: "a cross-solid face pair with a carrier kind that has no \
                           sound cheap reach bound (cone/torus/NURBS, or a planar \
                           face with a non-conic curved boundary) — the C9 \
                           exclusion ring is the certified excluder",
                });
                continue;
            };
            // Definite separation on ANY axis clears the pair: the
            // margin is the gap between the sound reach boxes — a
            // metre coordinate difference (audit row).
            let mut cleared = false;
            for (alo, ahi, blo, bhi) in [
                (alo.x, ahi.x, blo.x, bhi.x),
                (alo.y, ahi.y, blo.y, bhi.y),
                (alo.z, ahi.z, blo.z, bhi.z),
            ] {
                let gap = (blo - ahi).max(alo - bhi);
                if matches!(
                    decide("census_backstop_gap", Margin::of(gap), band),
                    Ok(Sign::Positive)
                ) {
                    cleared = true;
                    break;
                }
            }
            if !cleared {
                errors.push(ValidationError::CensusUndecidable {
                    a: EntityId::Face(a.face),
                    b: EntityId::Face(b.face),
                    what: "cross-solid faces within reach (curved against curved or \
                           planar) — the conformal-rest / proximity / partial-embedding \
                           class the C9 exclusion ring will examine",
                });
            }
        }
    }

    // Arm 2: instance containment (the contained side's vertex hull
    // against the containing side's reach box).
    let mut solid_boxes: std::collections::BTreeMap<SolidKey, (Point3<T>, Point3<T>)> =
        std::collections::BTreeMap::new();
    for (f, _) in body.faces.iter() {
        let Some(solid) = solid_of(f) else { continue };
        let pts = face_points(f);
        let Some((lo, hi)) = hull(&pts) else { continue };
        solid_boxes
            .entry(solid)
            .and_modify(|(l, h)| {
                *l = Point3::new(l.x.min(lo.x), l.y.min(lo.y), l.z.min(lo.z));
                *h = Point3::new(h.x.max(hi.x), h.y.max(hi.y), h.z.max(hi.z));
            })
            .or_insert((lo, hi));
    }
    // The CONTAINING side must be a superset, so it is built from the
    // one face-box rule, not from vertices: a cylinder solid's vertex
    // hull is the segment joining its two seam vertices, and using
    // that as a container would clear every body nested inside it. The
    // contained side stays the vertex hull, which is a SUBSET of the
    // solid's locus — that is what makes the clear sound, since a
    // witness point outside the container is genuinely outside. A
    // solid carrying a face with no cheap sound box has no claimable
    // extent at all and can never be the container.
    /// A solid's claimable extent, or `None` when one of its faces has
    /// no cheap sound box (so the solid can never be the container).
    type SolidReach<T> = Option<(Point3<T>, Point3<T>)>;
    let mut solid_reach: std::collections::BTreeMap<SolidKey, SolidReach<T>> =
        std::collections::BTreeMap::new();
    for (f, _) in body.faces.iter() {
        let Some(solid) = solid_of(f) else { continue };
        let this = reach_box(f);
        let slot = solid_reach.entry(solid).or_insert(this);
        *slot = match (*slot, this) {
            (Some((l, h)), Some((lo, hi))) => Some((
                Point3::new(l.x.min(lo.x), l.y.min(lo.y), l.z.min(lo.z)),
                Point3::new(h.x.max(hi.x), h.y.max(hi.y), h.z.max(hi.z)),
            )),
            _ => None,
        };
    }
    let solids: Vec<_> = solid_boxes.iter().collect();
    for (i, &(&sa, (alo, ahi))) in solids.iter().enumerate() {
        for &(&sb, (blo, bhi)) in solids.iter().skip(i + 1) {
            if bridged.contains(&(sa, sb)) {
                continue; // under the confirm pass's jurisdiction
            }
            for (outer, inner, ilo, ihi) in [(sa, sb, blo, bhi), (sb, sa, alo, ahi)] {
                let Some((olo, ohi)) = solid_reach.get(&outer).copied().flatten() else {
                    errors.push(ValidationError::CensusUndecidable {
                        a: EntityId::Solid(outer),
                        b: EntityId::Solid(inner),
                        what: "a surface kind with no cheap sound box leaves the \
                               containing instance's extent unclaimable — the \
                               same C6 interference class",
                    });
                    continue;
                };
                // Containment of `inner` in `outer`: all six extent
                // margins definitely positive ⇒ the interference
                // class; ANY definitely negative ⇒ clear; anything
                // weaker (a boundary-flush box) refuses too — only a
                // definite verdict clears (conservative direction).
                let margins = [
                    ilo.x - olo.x,
                    ohi.x - ihi.x,
                    ilo.y - olo.y,
                    ohi.y - ihi.y,
                    ilo.z - olo.z,
                    ohi.z - ihi.z,
                ];
                let mut cleared = false;
                let mut all_positive = true;
                for m in margins {
                    match decide("census_backstop_containment", Margin::of(m), band) {
                        Ok(Sign::Positive) => {}
                        Ok(Sign::Negative) => {
                            cleared = true;
                            all_positive = false;
                            break;
                        }
                        _ => all_positive = false,
                    }
                }
                if !cleared {
                    errors.push(ValidationError::CensusUndecidable {
                        a: EntityId::Solid(outer),
                        b: EntityId::Solid(inner),
                        what: if all_positive {
                            "one instance's extent box inside another's — C6's \
                             interference class (recorded gate-skips do not exist yet)"
                        } else {
                            "instance extent boxes not definitely separable from \
                             containment — the same C6 interference class, in band"
                        },
                    });
                }
            }
        }
    }
}

/// The confirmation direction of the certification diff: every
/// declaration must have a geometric witness — dead keys, equal keys,
/// and coincidence-free records are stale, typed.
fn confirm_declarations<T: Decide + crate::chart_region::ChartRegionLane>(
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
        match gap_is_zero("pm_census_confirm_vv", Margin::norm3(pa - pb), band, errors) {
            Some(true) | None => {}
            Some(false) => errors.push(stale),
        }
    }
    confirm_curve_and_patch_records(body, contacts, band, errors);
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
        match signed_is_zero(
            "pm_census_confirm_vf",
            Margin::of((q - f.origin).dot(f.normal)),
            band,
            errors,
        ) {
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

/// The at-rest confirmation of the two CURVED granularities (C3), the
/// other half of `confirm_declarations`.
///
/// A `CurveContact` is re-certified through the SAME jet door the use
/// site runs (`contact_pair_verdict`, class `Tangent`, along the
/// witness edge's own carrier) — the at-rest gate and the verify-at-use
/// gate share the door rather than mirroring it, so a false contact
/// cannot be silent at one and loud at the other. A `PatchContact`
/// (M9-2: the certifier the record's docs promised) confirms through
/// the SAME two doors its certification obligation names: the `Rest`
/// carrier/sense door (`contact_pair_verdict` — carrier identity
/// through the kind ladder with the record standing as the
/// declaration, senses opposed, aligned coincidence contradicted) and
/// the chart-region overlap predicate (region overlap in the shared
/// chart with definitely-positive area). Overlap `Empty` ⇒ the record
/// is STALE (C3's letter); an in-band overlap escalates; a pair the
/// predicate refuses typed (no exact-constant-arm chart, seam-branch
/// divergence, non-planar trims) is unsupported inventory — refused,
/// never sampled, never blessed.
///
/// ASM R2-b consumes exactly this pass (ASM-R2-SPEC-DRAFT:39-58): a
/// mate's declaration lands in the product body's `ContactRecords` —
/// the boolean 3′ currency, same type, no adapter — and THIS is the
/// at-rest evidence door those records certify through.
fn confirm_curve_and_patch_records<T: Decide + crate::chart_region::ChartRegionLane>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
    for c in &contacts.curves {
        let stale = ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::CurveLocus {
                face_a: c.face_a,
                face_b: c.face_b,
                witness: c.witness,
            },
        };
        let carrier = body
            .edges
            .get(c.witness)
            .and_then(|e| body.curves.get(e.curve))
            .and_then(CurveGeom::certified);
        let (Some(curve), true, true) = (
            carrier,
            body.get_face(c.face_a).is_some(),
            body.get_face(c.face_b).is_some(),
        ) else {
            errors.push(stale);
            continue;
        };
        let (t0, t1) = curve.params();
        match crate::boolean::contact_pair_verdict(
            body,
            c.face_a,
            body,
            c.face_b,
            crate::contact::ContactClass::Tangent,
            Some((curve.carrier(), t0, t1)),
            band,
        ) {
            Ok(_) => {}
            Err(crate::contact::ContactRefusal::Contradicted { diag, steer }) => {
                errors.push(ValidationError::ContactContradicted {
                    declaration: crate::contact::DeclaredContact {
                        a: c.face_a,
                        b: c.face_b,
                        class: crate::contact::ContactClass::Tangent,
                    },
                    witness: format!("{:?}", c.witness),
                    margin: diag,
                    steer,
                });
            }
            Err(crate::contact::ContactRefusal::Escalated { diag })
            | Err(crate::contact::ContactRefusal::Undeclared { diag }) => {
                errors.push(ValidationError::CensusEscalated { cause: diag });
            }
            Err(crate::contact::ContactRefusal::NotCertifiable { .. }) => {
                errors.push(ValidationError::CensusUnsupported {
                    entity: EntityId::Edge(c.witness),
                });
            }
        }
    }
    for c in &contacts.patches {
        let stale = ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::Patch {
                face_a: c.face_a,
                face_b: c.face_b,
            },
        };
        if body.get_face(c.face_a).is_none() || body.get_face(c.face_b).is_none() {
            errors.push(stale);
            continue;
        }
        // Door 1 — carrier identity + opposed senses, the record
        // standing as its own declaration (C3: rung 2/3, never
        // value-equal; aligned coincidence contradicts).
        match crate::boolean::contact_pair_verdict(
            body,
            c.face_a,
            body,
            c.face_b,
            crate::contact::ContactClass::Rest,
            None,
            band,
        ) {
            Ok(_) => {}
            Err(crate::contact::ContactRefusal::Contradicted { diag, steer }) => {
                errors.push(ValidationError::ContactContradicted {
                    declaration: crate::contact::DeclaredContact {
                        a: c.face_a,
                        b: c.face_b,
                        class: crate::contact::ContactClass::Rest,
                    },
                    witness: format!("{:?}~{:?}", c.face_a, c.face_b),
                    margin: diag,
                    steer,
                });
                continue;
            }
            Err(crate::contact::ContactRefusal::Escalated { diag })
            | Err(crate::contact::ContactRefusal::Undeclared { diag }) => {
                errors.push(ValidationError::CensusEscalated { cause: diag });
                continue;
            }
            Err(crate::contact::ContactRefusal::NotCertifiable { .. }) => {
                errors.push(ValidationError::CensusUnsupported {
                    entity: EntityId::Face(c.face_a),
                });
                continue;
            }
        }
        // Door 2 — region overlap in the shared chart, definitely
        // positive (the PR-1 predicate through the per-scalar lane).
        match T::chart_overlap(body, c.face_a, body, c.face_b, band) {
            None => {
                errors.push(ValidationError::CensusUnsupported {
                    entity: EntityId::Face(c.face_a),
                });
            }
            Some(Ok(crate::chart_region::ChartOverlap::PositiveArea)) => {}
            Some(Ok(crate::chart_region::ChartOverlap::Empty)) => errors.push(stale),
            Some(Err(crate::chart_region::ChartRegionError::Escalated(cause))) => {
                errors.push(ValidationError::CensusEscalated { cause });
            }
            Some(Err(_)) => {
                errors.push(ValidationError::CensusUnsupported {
                    entity: EntityId::Face(c.face_a),
                });
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The conformal face-pair arm and the patch-record certifier,
    //! pinned at the census door itself: the fixtures are open
    //! euler-built sheet scaffolds (the PR-1 chart fixtures'
    //! pattern), below tier 3's closed-body bar — which is exactly
    //! why these rows call [`census_and_certify`] directly. The
    //! closed-body end-to-end rows live in the acceptance suites.
    use super::*;
    use crate::boolean::PatchContact;
    use crate::entity::FaceKey;
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use geom_core::Vec3;
    use geom_surfaces::Surface;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn cyl_surface() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
    }

    fn cyl_pt(u: f64, z: f64) -> Point3<f64> {
        Point3::new(u.cos(), u.sin(), z)
    }

    /// An open cylinder-wall sheet `u ∈ [u0, u1] × z ∈ [z0, z1]` on
    /// the shared key, with the wall face's sense set.
    fn cyl_sheet(
        body: &mut Body<f64>,
        cyl: Option<crate::geometry::SurfaceKey>,
        u0: f64,
        u1: f64,
        z0: f64,
        z1: f64,
        sense: bool,
    ) -> (FaceKey, crate::geometry::SurfaceKey) {
        use geom_brep::{EdgeCurveSpec, EdgeGeometry};
        use geom_curves::Curve3;
        let (p00, p10, p11, p01) = (
            cyl_pt(u0, z0),
            cyl_pt(u1, z0),
            cyl_pt(u1, z1),
            cyl_pt(u0, z1),
        );
        let seed = body.mvfs(p00).unwrap();
        let cyl = cyl.unwrap_or_else(|| body.add_surface(cyl_surface()));
        let rim = |body: &mut Body<f64>, z: f64, ccw: bool| {
            let plane = body.add_surface(Surface::Plane {
                origin: Point3::new(0.0, 0.0, z),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            let (carrier, t0, t1) = if ccw {
                (
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, z),
                        axis: Vec3::unit_z(),
                        radius: 1.0,
                        u_ref: Vec3::unit_x(),
                    },
                    u0,
                    u1,
                )
            } else {
                (
                    Curve3::Circle {
                        center: Point3::new(0.0, 0.0, z),
                        axis: Vec3::new(0.0, 0.0, -1.0),
                        radius: 1.0,
                        u_ref: Vec3::new(u1.cos(), u1.sin(), 0.0),
                    },
                    0.0,
                    u1 - u0,
                )
            };
            EdgeCurveSpec {
                description: EdgeGeometry::Intersection {
                    s1: cyl,
                    s2: plane,
                    witness: cyl_pt((u0 + u1) * 0.5, z),
                },
                carrier,
                param_start: t0,
                param_end: t1,
            }
        };
        let bottom = rim(body, z0, true);
        let e_b = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p10,
                bottom,
            )
            .unwrap();
        let e_r = body
            .mev_line(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                p11,
            )
            .unwrap();
        let top = rim(body, z1, false);
        let e_t = body
            .mev(
                MevSite::Fan {
                    he1: e_r.he_minus,
                    he2: e_r.he_minus,
                },
                p01,
                top,
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_b.he_plus,
                },
                EdgeCurveSpec::line_between(p01, p00),
                FaceSurface::Shared(cyl),
            )
            .unwrap()
            .face;
        body.set_face_sense(face, sense).unwrap();
        (face, cyl)
    }

    /// Two overlapping opposed-sense wall sheets on one cylinder key.
    fn conformal_pair() -> (Body<f64>, FaceKey, FaceKey) {
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
        let (w2, _) = cyl_sheet(&mut body, Some(cyl), 1.0, 2.4, 0.3, 0.7, false);
        crate::pcurves::mint_pcurves(&mut body).unwrap();
        (body, w1, w2)
    }

    #[test]
    fn the_conformal_arm_finds_an_undeclared_pair_and_carries_the_finding() {
        let (body, w1, w2) = conformal_pair();
        let errors = census_and_certify(&body, &ContactRecords::default(), band());
        let hit = errors
            .iter()
            .find_map(|e| match e {
                ValidationError::UndeclaredContact {
                    contact: CensusContact::ConformalPatch { finding },
                    ..
                } => Some(*finding),
                _ => None,
            })
            .expect("the finding-carrying refusal: {errors:?}");
        // The kernel finding names the pair and the class that would
        // verify — the recourse is a quotable declaration.
        let pair = (hit.pair.a.min(hit.pair.b), hit.pair.a.max(hit.pair.b));
        assert_eq!(pair, (w1.min(w2), w1.max(w2)));
        assert_eq!(hit.pair.class, crate::contact::ContactClass::Rest);
        assert_eq!(hit.verdict, crate::contact::ContactVerdict::Definite);
    }

    #[test]
    fn a_patch_record_backs_the_pair_and_confirms_through_both_doors() {
        let (body, w1, w2) = conformal_pair();
        let mut records = ContactRecords::default();
        records.patches.push(PatchContact {
            face_a: w1,
            face_b: w2,
        });
        let errors = census_and_certify(&body, &records, band());
        assert!(
            errors.is_empty(),
            "the declared conformal patch certifies: {errors:?}"
        );
    }

    #[test]
    fn a_disjoint_patch_record_is_stale_typed() {
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
        let (w3, _) = cyl_sheet(&mut body, Some(cyl), 3.0, 4.0, 0.0, 1.0, false);
        crate::pcurves::mint_pcurves(&mut body).unwrap();
        let mut records = ContactRecords::default();
        records.patches.push(PatchContact {
            face_a: w1,
            face_b: w3,
        });
        let errors = census_and_certify(&body, &records, band());
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
            "overlap Empty ⇒ stale (C3's letter): {errors:?}"
        );
    }

    // ================= R1 review probes (m9-2b-r1) =================

    /// R1 probe (claim 2): an IN-BAND sliver overlap must escalate at
    /// BOTH doors — the conformal arm (undeclared direction) and the
    /// patch-record certifier (declared direction) — never decide.
    #[test]
    fn r1_probe_in_band_sliver_overlap_escalates_both_directions() {
        let mut body = Body::<f64>::new();
        // Overlap region u ∈ [0.4, 1.4] × z ∈ [0.5, 0.5 + 5e-9]:
        // mean width ≈ 5e-9 m, inside Band{1e-9, 1e-8}.
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 0.5 + 5e-9, true);
        let (w2, _) = cyl_sheet(&mut body, Some(cyl), 0.4, 1.4, 0.5, 1.0, false);
        crate::pcurves::mint_pcurves(&mut body).unwrap();
        let arm = census_and_certify(&body, &ContactRecords::default(), band());
        assert!(
            arm.iter()
                .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
            "an in-band sliver must escalate at the arm: {arm:?}"
        );
        assert!(
            !arm.iter().any(|e| matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::ConformalPatch { .. },
                    ..
                }
            )),
            "an in-band sliver must never DECIDE undeclared: {arm:?}"
        );
        let mut records = ContactRecords::default();
        records.patches.push(PatchContact {
            face_a: w1,
            face_b: w2,
        });
        let cert = census_and_certify(&body, &records, band());
        assert!(
            cert.iter()
                .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
            "the record certifier must escalate in band: {cert:?}"
        );
        assert!(
            !cert
                .iter()
                .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
            "in-band is escalation, never stale: {cert:?}"
        );
    }

    /// R1 probe (claim 1): a same-key pair AUTHORED one period apart
    /// (u-windows on the "next branch") cannot slip the sweep — the
    /// pcurve mint normalizes the branch, the windows overlap, and
    /// the arm reports the undeclared conformal contact; the same
    /// pair backed by its patch record certifies clean. (The probe
    /// originally expected the seam-branch typed refusal here; the
    /// mint normalizes first, so the refusal is unreachable from
    /// euler-authored geometry — a stronger answer, pinned.)
    #[test]
    fn r1_probe_next_branch_windows_still_find_the_overlap() {
        let tau = core::f64::consts::TAU;
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
        // Same locus, next periodic branch; u-nested and z-nested so
        // no strut/vertex coincidences muddy the face-pair question.
        let (w2, _) = cyl_sheet(&mut body, Some(cyl), 0.5 + tau, 1.2 + tau, 0.3, 0.7, false);
        crate::pcurves::mint_pcurves(&mut body).unwrap();
        let arm = census_and_certify(&body, &ContactRecords::default(), band());
        assert!(
            arm.iter().any(|e| matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::ConformalPatch { .. },
                    ..
                }
            )),
            "the next-branch authoring must not evade the arm: {arm:?}"
        );
        let mut records = ContactRecords::default();
        records.patches.push(PatchContact {
            face_a: w1,
            face_b: w2,
        });
        let cert = census_and_certify(&body, &records, band());
        assert!(
            cert.is_empty(),
            "the backed next-branch pair certifies: {cert:?}"
        );
    }

    #[test]
    fn an_aligned_same_sense_patch_record_is_contradicted() {
        // Same key, SAME sense: aligned coincidence is containment or
        // flush material, never contact (C1) — the record lies.
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
        let (w2, _) = cyl_sheet(&mut body, Some(cyl), 1.0, 2.4, 0.3, 0.7, true);
        crate::pcurves::mint_pcurves(&mut body).unwrap();
        let mut records = ContactRecords::default();
        records.patches.push(PatchContact {
            face_a: w1,
            face_b: w2,
        });
        let errors = census_and_certify(&body, &records, band());
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
            "{errors:?}"
        );
        // And the ARM stays quiet on the aligned pair: SameOriented
        // is flush, not a conformal candidate.
        let arm_only = census_and_certify(&body, &ContactRecords::default(), band());
        assert!(
            !arm_only.iter().any(|e| matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::ConformalPatch { .. },
                    ..
                }
            )),
            "{arm_only:?}"
        );
    }
}
