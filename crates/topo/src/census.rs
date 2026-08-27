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
//!   cap, F5). Cone, torus and NURBS sides are reach-tested like any
//!   other — every surface kind has a sound cheap reach bound
//!   ([`face_reach`]); what refuses without a distance test is a
//!   DESCRIPTION with no claim in it (a placeholder patch), not a
//!   kind. A cross-key `PatchContact` on such a pair ESCALATES
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
//! The records this rule reconstructs FROM are vertex-granularity
//! (`VvContact`, `VfContact`). Since M9-2 `ContactRecords` also carries
//! face-granularity records (`curves`, `patches`), and each bullet
//! below has a corresponding face rung — `Declared::vv_face_backed` /
//! `vf_face_backed` / `ve_face_backed`, which back a subordinate
//! vertex event from a declared FACE pair holding the entities it
//! relates on the pair's two sides. Those rungs are named in the
//! bullets rather than left out of the derivation that licenses them.
//!
//! **All three rungs are structural-incidence and region-unconfined,
//! by one deliberate decision.** A rung asks whether a declared face
//! pair HOLDS the entities of the event — one on each side, through
//! boundary membership and an edge's incidence to the faces it bounds
//! — and never where on those faces the event lies. So a declared pair
//! backs an event on the entities it holds even where that event lies
//! outside the pair's own overlap region. Confining a rung to that
//! region is not a tightening of one rung, it is a second standard the
//! other two do not meet; the region question belongs to the confirm
//! pass, which verifies the declared pair itself in both directions.
//! Continuous overlaps — two collinear edges sharing a positive-length
//! segment, an edge resting in a face's region — are certified by
//! **reconstruction from their bounding vertex events**:
//!
//! - An **edge-edge collinear overlap** is certified iff each of its
//!   two bounds is backed. Where both edges hold a vertex at the
//!   bound, that means the pair is v-v-declared, or backed by a
//!   declared face pair holding the two vertices on its two boundaries
//!   (`vv_face_backed`), or is one shared vertex — structural. Where
//!   only one edge holds a vertex there — the endpoint rests on the
//!   other edge's INTERIOR — the bound is a vertex-on-edge event and
//!   is backed by exactly that lane's rung: a declared face pair
//!   holding the vertex on one boundary and naming a face the other
//!   edge bounds (`ve_face_backed`). Derivation:
//!   the overlap of two collinear spans is an interval whose each
//!   bound is an endpoint of at least one span, so one of the two arms
//!   applies at every bound. Between two backed bounds the
//!   carriers coincide identically (two lines sharing two points are
//!   one line), so the interior overlap is exactly the convex closure
//!   of the bounded events on both carriers — no interior record can
//!   carry more information than the bounds on the planar corpus.
//! - An **edge-on-face overlap** is certified iff at each bound the
//!   edge holds a vertex there and that vertex is either
//!   v-on-f-declared on this face, v-v-declared with a coincident
//!   vertex of the face's boundary, backed by a declared face pair
//!   naming this face and one holding the vertex (`vf_face_backed`),
//!   or itself a vertex of the face's boundary (structural). Same
//!   argument, with its remaining looseness stated: a bound at which
//!   the edge holds NO vertex is a vertex-on-edge or edge-edge-cross
//!   configuration the sweeps report in their own lane, so this lane
//!   declines to reconstruct it rather than inferring one. Where that
//!   subordinate event is itself face-backed, the decline is a
//!   LOUDNESS the pair keeps — reported, never blessed — and closing
//!   it needs the same rung read at this bound, not a new one.
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
//! vertex-on-plane lane (`dovertexonface` → `OnEdge`) — so in the
//! BOOLEAN lane every vertex-on-edge(-interior) contact is refined
//! into a v-v record before records are emitted. That is why no
//! vertex-granularity record type names this configuration: in the
//! lane that mints them, it never survives to be named.
//!
//! **That premise is the boolean lane's, and it does not carry to
//! rest.** At rest nothing refines — no boolean runs, nothing is
//! zipped, the bodies arrive as they were placed — so the raw induced
//! configuration reaches the certifier intact. Its status there is:
//!
//! - **Certifiable through the face rung, and only through it**: a
//!   declared face pair holding the vertex on one boundary and naming
//!   a face the edge bounds (`ve_face_backed`) holds the whole event
//!   — the vertex on one side of the interface, the edge on the other
//!   — exactly as `vv_face_backed` holds a coincident vertex pair.
//!   A seat whose two faces share a boundary induces this event by
//!   construction, and the declaration that says the faces rest says
//!   it once for everything the seat induces.
//! - **Otherwise an undeclarable defect**: with no face pair holding
//!   it, there is no record that can name the configuration, and the
//!   census reports [`CensusContact::VertexOnEdge`] as
//!   `UndeclaredContact`. The rung consults DECLARATIONS, never the
//!   geometry's own agreement with itself — a configuration nobody
//!   declared stays the F1 hard error however exactly it coincides.
//!
//! Cross-reference: the face rung is CONTACT-DESIGN C3's declared
//! rung read at the granularity the records already carry, not a new
//! identity claim about the carriers — what the declared pair itself
//! must satisfy is the confirm pass's, on C3's ladder.

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
    sweep_vertex_edge(&geo, &declared, band, &mut errors);
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

    /// The face rung for a vertex-on-edge event: the v-on-f rung
    /// against either face the edge bounds. An edge is structurally
    /// incident to `f_plus` and `f_minus`, so a declared pair holding
    /// `v` on one boundary and naming a face of `e` on the other holds
    /// the whole event — the vertex on one side of the interface, the
    /// edge on the other.
    ///
    /// Strength, stated rather than left to be read off the code: this
    /// rung is STRUCTURAL-INCIDENCE, exactly as strong as the two
    /// rungs it is built from and no stronger. It asks whether the
    /// declared pair holds the two entities, never WHERE on the pair
    /// the event lies, so it backs an event anywhere on the incident
    /// entities — including outside the declared faces' own overlap
    /// region. Confining it to that region would be new machinery
    /// [`Declared::vv_face_backed`] and [`Declared::vf_face_backed`]
    /// do not have, and the census would then hold three rungs to two
    /// different standards.
    fn ve_face_backed<T: Real>(&self, geo: &Geo<T>, v: VertexKey, e: &EdgeGeo<T>) -> bool {
        self.vf_face_backed(geo, v, e.f_plus) || self.vf_face_backed(geo, v, e.f_minus)
    }
}

/// Builds the geometry snapshot: exact planar entities for the
/// vertex-granular sweeps, curved entities routed to the
/// face-granular arms (M9-2 — the census ADMITS every carrier kind;
/// the blanket exact-on-planar refusal retired with the census arms
/// that replaced it, and what each arm can and cannot certify is the
/// module-docs envelope, stated rather than sampled). Total: every
/// entity lands in exactly one bucket, so there is no refusal path.
/// The exact sweeps' admission test for one edge, in ONE place.
/// [`snapshot`] keeps an edge iff its carrier is a certified `Line`,
/// and `line_bounded` — the planar × planar skip's premise — asks the
/// same question of a whole face boundary. The two must not drift:
/// if snapshot's rule widens and the skip's does not, the skip keeps
/// firing on faces whose boundary is no longer in front of the
/// sweeps, which is the UNSOUND direction. Sharing the predicate is
/// what makes the coupling structural rather than remembered.
///
/// Snapshot drops one thing more than this models: a certified line
/// edge whose endpoints do not resolve (tier 1 guarantees they do, so
/// the skip's premise is not weakened by the difference in practice —
/// but the difference is real, and it is here, not implied). Three
/// further copies of the same match live outside the census
/// (`merge_faces`, `validate`, `boolean::ops`); each answers its own
/// question, not the snapshot's.
fn edge_is_line<T: Real>(body: &Body<T>, ek: crate::entity::EdgeKey) -> bool {
    body.edges
        .get(ek)
        .and_then(|e| body.curves.get(e.curve))
        .and_then(CurveGeom::certified)
        .map(geom_brep::EdgeCurve::carrier)
        .is_some_and(|c| matches!(c, geom::Curve3::Line { .. }))
}

fn snapshot<T: Decide>(body: &Body<T>) -> Geo<T> {
    let verts: Vec<(VertexKey, Point3<T>)> = body
        .vertices
        .iter()
        .filter_map(|(k, v)| body.points.get(v.point).map(|p| (k, *p)))
        .collect();
    let mut edges = Vec::new();
    for (key, edge) in body.edges.iter() {
        if !edge_is_line(body, key) {
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
            Some(&geom::Surface::Plane { origin, normal, .. }) => Some((origin, normal)),
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

/// Census pass 2: vertex on an edge's **interior** — certifiable only
/// through the face rung (module docs, D4), and otherwise a hard
/// finding: there is no vertex-granularity record type that can name
/// this configuration.
fn sweep_vertex_edge<T: Decide>(
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) {
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
            if interior && !declared.ve_face_backed(geo, vk, e) {
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
        // The bound is a vertex-on-edge event, which has a backing
        // path of its own — so this lane declining is a LOUDNESS the
        // overlap keeps, not a proof the configuration is undeclared
        // (D3's bullet states the same looseness; #973 schedules it).
        return false;
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
/// vertex — structural), or — where only ONE edge has a vertex there,
/// so the bound rests on the other edge's interior — that vertex is
/// face-backed onto the other edge.
///
/// The two arms are one rule at two granularities: a bound of a
/// collinear overlap is an endpoint of at least one span, and whether
/// the other span happens to end there too is a fact about the
/// configuration, not about what a declaration can hold. Both edges
/// with a vertex is a v-v event and takes the v-v rungs; one edge with
/// a vertex is a vertex-on-edge event and takes that lane's rung
/// ([`Declared::ve_face_backed`]) — the same declared face pair, one
/// incidence step further out.
fn ee_bound_backed<T: Decide>(
    ea: &EdgeGeo<T>,
    eb: &EdgeGeo<T>,
    s: T,
    geo: &Geo<T>,
    declared: &Declared,
    band: Band,
    errors: &mut Vec<ValidationError>,
) -> bool {
    let va = edge_vertex_at(ea, s, band, errors);
    let q = ea.p0 + ea.dir * s;
    let sb = (q - eb.p0).dot(eb.dir);
    let vb = edge_vertex_at(eb, sb, band, errors);
    match (va, vb) {
        (Some(va), Some(vb)) => {
            va == vb || declared.vv.contains(&(va, vb)) || declared.vv_face_backed(geo, va, vb)
        }
        (Some(va), None) => declared.ve_face_backed(geo, va, eb),
        (None, Some(vb)) => declared.ve_face_backed(geo, vb, ea),
        // Neither edge resolves a vertex at the bound: an escalated
        // span (already pushed), never a backing.
        (None, None) => false,
    }
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
                        // No CERTIFIED lane at this scalar (dual):
                        // `ChartRegionLane` refuses, so the pair cannot
                        // be decided here — typed, never silent. (Since
                        // D1, 2026-08-19, a dual DOES carry a bracket;
                        // the refusal is the lane's ruling, not a
                        // missing `Bounds` impl.)
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

/// **`boolean::boxes`'s `FaceBoxRule`/`EdgeBoxRule`, instantiated at
/// this lane's scalar.** Both the rules — which kinds have a cheap
/// sound superset, and by what construction — AND the extent
/// arithmetic that realizes them come from there: the per-kind
/// extents are written once, against `boxes::Span`, and this lane
/// enters them with degenerate spans (`lo == hi`) at its own `T`
/// while the boolean lane enters them with `[lo(), hi()]` brackets at
/// `f64`. Neither takes a bound the other cannot, and no bound is
/// derived twice.
///
/// What this lane still owns is its ARENA WALK — [`boundary_reach`]
/// reads `body.loops`/`body.half_edges` directly rather than through
/// the accessors the `Bounds`-allowlisted lane uses — and its answer
/// for a description with no claim in it: `None`, versus the poison
/// box there. Neither is arithmetic, and
/// `the_two_box_lanes_agree_face_for_face` in `boolean::boxes` pins
/// that what is left cannot drift.
///
/// A NURBS placeholder has a poison control net: `face_box` folding
/// it to a poison box is correct there, because poison never prunes.
/// Folding it here would produce `Some((NaN, NaN))` — neither a claim
/// nor a refusal, since every margin against it decides NEITHER sign
/// — so this answers `None`.
pub(crate) fn face_reach<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
) -> Option<(Point3<T>, Point3<T>)> {
    let surface = body
        .get_face(f)
        .and_then(|d| body.surfaces.get(d.surface))?;
    match crate::boolean::boxes::face_box_rule(surface) {
        crate::boolean::boxes::FaceBoxRule::BoundaryHull => boundary_reach(body, f),
        crate::boolean::boxes::FaceBoxRule::ControlNet(patch) => {
            if patch.is_placeholder() {
                // The mvfs placeholder's control net is poison
                // points, and this fold is `min`/`max`, which
                // propagate NaN by contract. Folding it would
                // return `Some((NaN, NaN))` — a box that is
                // neither a claim nor a refusal: every margin
                // taken against it decides NEITHER sign, so the
                // arm falls out at its in-band refusal having
                // compared no geometry at all, and the typed
                // "unclaimable extent" refusal below never fires.
                // `None` is what this function's contract already
                // says a description with no claim in it answers,
                // and a placeholder is that case par excellence:
                // it is "no description yet".
                //
                // NOT an exclusion. Dropping the face from a
                // solid's reach would UNDER-claim the container
                // and could clear a body nested inside it; `None`
                // makes the whole solid unclaimable, which is the
                // conservative direction and the one arm 2's fold
                // is already written for.
                return None;
            }
            let mut it = patch.control().iter();
            let first = *it.next()?;
            let (mut lo, mut hi) = (first, first);
            for p in it {
                lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
                hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
            }
            Some((lo, hi))
        }
        crate::boolean::boxes::FaceBoxRule::WholeBall { center, radius } => {
            Some(span_pts(crate::boolean::boxes::ball_extent(
                &crate::boolean::boxes::SpanBox::point(center),
                radius,
            )))
        }
        crate::boolean::boxes::FaceBoxRule::WholeTorus {
            center,
            axis,
            major_radius,
            minor_radius,
        } => Some(span_pts(crate::boolean::boxes::torus_extent(
            &crate::boolean::boxes::SpanBox::point(center),
            &crate::boolean::boxes::SpanBox::vector(axis),
            major_radius,
            minor_radius,
        ))),
        crate::boolean::boxes::FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => {
            // The axial extent comes from the boundary's own LOCUS
            // — not its vertices, and not a box around it: the axial
            // coordinate is linear along the surface, so the face's
            // axial extremes lie ON the boundary, but not
            // necessarily at a boundary VERTEX.
            let h = boundary_axial(body, f, origin, axis)?;
            let slab = span_pts(crate::boolean::boxes::slab_extent(
                &crate::boolean::boxes::SpanBox::point(origin),
                &crate::boolean::boxes::SpanBox::vector(axis),
                h,
                radius,
            ));
            // The azimuth clip, mirroring the boolean lane's
            // `clip_to_boundary` — the slab is the whole turn, the face
            // is a patch of it, and the boundary's own reach bounds the
            // patch's footprint perpendicular to the axis (azimuth is a
            // chart coordinate, so it takes no interior extremum). Both
            // lanes must clip the same way or
            // `the_two_box_lanes_agree_face_for_face` reds — which is
            // exactly that row's job.
            Some(match boundary_reach(body, f) {
                Some((blo, bhi)) => (
                    Point3::new(slab.0.x.max(blo.x), slab.0.y.max(blo.y), slab.0.z),
                    Point3::new(slab.1.x.min(bhi.x), slab.1.y.min(bhi.y), slab.1.z),
                ),
                None => slab,
            })
        }
        crate::boolean::boxes::FaceBoxRule::ConeSlab {
            apex,
            axis,
            half_angle,
        } => {
            let h = boundary_axial(body, f, apex, axis)?;
            let apex = crate::boolean::boxes::SpanBox::point(apex);
            let axis = crate::boolean::boxes::SpanBox::vector(axis);
            Some(span_pts(crate::boolean::boxes::cone_frustum_extent(
                &apex,
                &axis,
                h,
                half_angle.tan(),
            )))
        }
    }
}

/// The face boundary's AXIAL range about `(origin, axis)` — the
/// boundary's own locus projected on the axis, edge by edge, rather
/// than the corners of a box around it
/// ([`crate::boolean::boxes::edge_axial_span`], which carries why the
/// difference is not cosmetic at a tilted axis). `None` for a face
/// with no boundary, or one whose boundary this lane cannot walk.
fn boundary_axial<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
    origin: Point3<T>,
    axis: Vec3<T>,
) -> Option<crate::boolean::boxes::Span<T>> {
    use crate::boolean::boxes::{AxialCarrier, EdgeBoxRule, SpanBox, edge_axial_span};
    let face = body.get_face(f)?;
    let (o, ax) = (SpanBox::point(origin), SpanBox::vector(axis));
    let mut acc: Option<crate::boolean::boxes::Span<T>> = None;
    for &lk in core::iter::once(&face.outer).chain(&face.rings) {
        let l = body.loops.get(lk)?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                let v = body.vertices.get(vertex)?;
                let p = SpanBox::point(*body.points.get(v.point)?);
                let sp = edge_axial_span(&o, &ax, &AxialCarrier::Chord, (&p, &p));
                acc = Some(acc.map_or(sp, |a| a.hull(sp)));
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first)? {
                    let ek = body.half_edges.get(he)?.edge;
                    let e = body.edges.get(ek)?;
                    let end = |h| -> Option<SpanBox<T>> {
                        let hd = body.half_edges.get(h)?;
                        let v = body.vertices.get(hd.start)?;
                        Some(SpanBox::point(*body.points.get(v.point)?))
                    };
                    let carrier = body
                        .curves
                        .get(e.curve)
                        .and_then(CurveGeom::certified)
                        .map(geom_brep::EdgeCurve::carrier);
                    let axial = match crate::boolean::boxes::edge_box_rule(carrier) {
                        EdgeBoxRule::NoSoundBox => AxialCarrier::Unclaimable,
                        EdgeBoxRule::Chord => AxialCarrier::Chord,
                        EdgeBoxRule::ConicAmplitude {
                            center,
                            axis: c_axis,
                            semi_u,
                            semi_v,
                            u_ref,
                        } => AxialCarrier::Conic {
                            center: SpanBox::point(center),
                            u_ref: SpanBox::vector(u_ref),
                            v_ref: SpanBox::vector(c_axis.cross(u_ref)),
                            semi_u,
                            semi_v,
                            params: body
                                .curves
                                .get(e.curve)
                                .and_then(CurveGeom::certified)
                                .map(geom_brep::EdgeCurve::params),
                        },
                    };
                    let sp =
                        edge_axial_span(&o, &ax, &axial, (&end(e.he_plus)?, &end(e.he_minus)?));
                    acc = Some(acc.map_or(sp, |a| a.hull(sp)));
                }
            }
        }
    }
    acc
}

/// Every boundary edge's reach, hulled with the isolated-vertex loops
/// (which have no edge to speak for them). `None` as soon as one
/// boundary curve has no sound box — [`face_reach`]'s boundary walk.
fn boundary_reach<T: Decide>(
    body: &Body<T>,
    f: crate::entity::FaceKey,
) -> Option<(Point3<T>, Point3<T>)> {
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
                    grow(edge_reach(body, ek)?);
                }
            }
        }
    }
    acc
}

/// One edge's reach — [`crate::boolean::boxes::EdgeBoxRule`] at this
/// lane's scalar.
fn edge_reach<T: Decide>(
    body: &Body<T>,
    ek: crate::entity::EdgeKey,
) -> Option<(Point3<T>, Point3<T>)> {
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
            // The ARC's own extent, not the closed conic's — the same
            // construction the boolean lane reads, so the two cannot
            // drift (`the_two_box_lanes_agree_face_for_face` is what
            // says so). A carrier with no certified parameters has no
            // arc to scope and keeps the full-turn amplitude.
            let params = body
                .curves
                .get(e.curve)
                .and_then(CurveGeom::certified)
                .map(geom_brep::EdgeCurve::params);
            let (flo, fhi) = span_pts(match params {
                Some((t0, t1)) => crate::boolean::boxes::arc_extent(
                    &crate::boolean::boxes::SpanBox::point(center),
                    &crate::boolean::boxes::SpanBox::vector(u_ref),
                    &crate::boolean::boxes::SpanBox::vector(v_ref),
                    crate::boolean::boxes::Span::exact(semi_u),
                    crate::boolean::boxes::Span::exact(semi_v),
                    t0,
                    t1,
                ),
                None => crate::boolean::boxes::conic_extent(
                    &crate::boolean::boxes::SpanBox::point(center),
                    &crate::boolean::boxes::SpanBox::vector(u_ref),
                    &crate::boolean::boxes::SpanBox::vector(v_ref),
                    semi_u,
                    semi_v,
                ),
            });
            Some((
                Point3::new(
                    flo.x.min(chord.0.x),
                    flo.y.min(chord.0.y),
                    flo.z.min(chord.0.z),
                ),
                Point3::new(
                    fhi.x.max(chord.1.x),
                    fhi.y.max(chord.1.y),
                    fhi.z.max(chord.1.z),
                ),
            ))
        }
    }
}

/// A [`crate::boolean::boxes::SpanBox`] as this lane's `(lo, hi)`
/// corner pair, and back.
fn span_pts<T: Decide>(s: crate::boolean::boxes::SpanBox<T>) -> (Point3<T>, Point3<T>) {
    (
        Point3::new(s.x.lo, s.y.lo, s.z.lo),
        Point3::new(s.x.hi, s.y.hi, s.z.hi),
    )
}

/// **The conservative loudness backstop** (M9-2 union fix F1): the
/// census must DECIDE or REFUSE — it must never silently not-examine
/// (A5's letter). Two cross-solid candidate classes have no examining
/// arm yet, and both fire a typed [`ValidationError::CensusUndecidable`]
/// here instead of passing silently:
///
/// 1. **Proximity** (the C9-ring conformal-rest / partial-embedding
///    class): a pair of faces from DIFFERENT solids, not
///    vertex-adjacent, whose reach boxes cannot be definitely
///    separated, and which the exact sweeps cannot see — every pair
///    with a curved side (F5: curved × planar included, since a
///    revolved cap embedded in a plate's slab leaves no
///    vertex/line/planar evidence), PLUS planar × planar where either
///    side has a curved boundary (§S49: an arc rim is not in the
///    snapshot, so a cylinder's cap leaves nothing either). Distinct-key value-equal carriers in conformal rest
///    (the cradle witness), value-equal walls at gap zero
///    (boss-in-hole), and the embedded ball cap (the delta witness)
///    land here; the certified excluder this stands in for is the C9
///    exclusion ring. The reach boxes are SOUND per-kind supersets:
///    [`face_reach`] is [`crate::boolean::boxes::FaceBoxRule`] — the
///    ONE face-box rule — with this module's own arena walk over the
///    module's own per-kind extents, so no bound is derived twice. A
///    description with no claim in it (a placeholder patch) refuses
///    WITHOUT a distance test rather than under-claiming its reach. A
///    planar face vf-NAMED by a record whose vertex is on the OTHER
///    FACE OF THIS PAIR defers to the confirm pass (the declared
///    boss-on-plate class) — the record has to name both sides of the
///    pair it defers.
/// 2. **Instance containment** (C6's interference class): one solid's
///    vertex-extent box inside another's REACH box (the containing
///    side must be a superset; the contained side is a subset of its
///    own locus, which is what makes a clear sound) — a nested placement
///    makes no boundary event at all (the reviewed nested-cube
///    witness), and interference is representable only through C6's
///    recorded gate-skips, which do not exist yet.
///
///    **This is the arm where a box that is too BIG is wrong**, and
///    the direction is easy to get backwards: over-width in the
///    containing reach box does not cost work here, it costs an
///    answer — a solid genuinely outside stops having a definitely
///    negative margin and is refused as the interference class. Each
///    arm claims exactly its rule's construction and no more
///    (`boxes`' ceiling rows pin that), so what is left here is the
///    looseness the rules THEMSELVES state — a whole ball for a
///    sphere band, a full turn for an arc — not slack in the code.
///
/// Both arms clear a pair ONLY on a definitely-positive separation
/// margin (`census_backstop_gap` / `census_backstop_containment` —
/// metre coordinate differences); anything weaker refuses.
///
/// A planar × planar pair is skipped only when BOTH faces are bounded
/// entirely by line edges (§S49). That — not the kind of solid the
/// faces sit on — is what the skip's premise is about: [`snapshot`]
/// admits line edges and drops every curved one, so a wholly
/// line-bounded planar face has its whole boundary, every vertex AND
/// every edge, in front of the exact sweeps. The premise is that
/// nothing such a pair can do to the other is invisible — NOT that
/// contact takes one of two shapes. It takes at least five, and each
/// has a lane: coincident boundary vertices in [`sweep_vertex_vertex`],
/// a vertex on the other boundary in [`sweep_vertex_edge`], collinear
/// boundary overlap in [`ee_collinear_lane`], a boundary lying in the
/// other face in [`sweep_vertex_face`] / [`ef_overlap_lane`], and a
/// pierce in [`sweep_edge_face`]. A future reader deciding whether
/// some OTHER face class may be skipped owes the same enumeration,
/// not an appeal to the two obvious cases.
///
/// A planar face with a curved boundary — a cylinder's cap — is
/// outside all of it: its rim is not in the snapshot, so no lane
/// holds it, and a cap can rest on another face with nothing to see
/// at vertex/line granularity. Such a pair is THIS arm's: the conformal
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
/// there — skipping it here loses no loudness). Every deferral above
/// names an arm that asks the SAME question about the SAME pair; that
/// is the bar a deferral has to clear, and it is what makes a skip
/// something other than a silent not-examine (A5's letter).
///
/// **Contact records do not license a deferral in arm 2, and this is
/// the one place the rule had to be derived rather than inherited.**
/// The four record kinds each state one coincidence — vv a coincident
/// vertex pair, v-on-f a vertex resting in a face's region, `curves` a
/// tangent locus along one witness edge, `patches` a conformal region
/// overlap — and the confirm pass asks exactly that of each: is THIS
/// coincidence geometrically real. Arm 2's question is different in
/// kind: where does one instance sit relative to another. No record
/// type in [`crate::boolean::ContactRecords`] states a placement
/// relation, so no set of them can answer it, and the confirm pass
/// never asks. A solid pair carrying records is therefore examined
/// here exactly like a pair carrying none.
///
/// The unsound direction is a deferral keyed on *whether* records
/// exist: a truthful declaration then switches the containment
/// examination off, and an instance embedded in another's material
/// validates clean — which it did, until §H14. The vocabulary that
/// WOULD license a skip here is C6's recorded gate-skips, which are a
/// statement about placement and do not exist yet; when they do, the
/// deferral they license is keyed on the gate-skip, not on contact.
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
    //
    // A loop this cannot walk contributes nothing, and an EMPTY loop is
    // one of them: a lone-vertex loop has no cycle, and a face whose
    // OUTER loop is empty is unbounded — there is no hull of it. This
    // closure does not decide what that means, because its two callers
    // want opposite things from it; each answers emptiness itself, and
    // both say so at the call site. What is never allowed is the third
    // reading, "empty means nothing to look at": §H14's residue 2 is the
    // same `continue` in `splitting/rules.rs` under exactly that reading.
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
    // Every boundary edge of `f` — outer loop and rings — passes
    // [`edge_is_line`], the exact sweeps' own admission test. That is
    // the property the planar × planar skip rests on: only a face
    // whose whole boundary is admitted is in front of those sweeps.
    // Anything unresolvable — a missing loop, an isolated-vertex
    // boundary, an uncertified carrier — is not a line, so the face
    // stays with this arm. `edge_is_line` carries the one way this
    // is weaker than snapshot's admission (unresolvable endpoints).
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
                let Some(hd) = body.half_edges.get(he) else {
                    return false;
                };
                if !edge_is_line(body, hd.edge) {
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
    let reach_box = |f: FK| face_reach(body, f);

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
        // scaffolding): there is no geometry to be within reach OF —
        // excluded from the backstop. What licenses the exclusion is a
        // named check, not an accident: the census's only production
        // caller is [`crate::validate::validate_pseudomanifold`], and
        // it runs `census_and_certify` ONLY when `tier3_local_checks`
        // came back empty, whose check 1 pushes `UncertifiableSurface`
        // for every placeholder face. So no body reaching this arm
        // through the public door carries one. The unsound direction is
        // a SECOND caller of `census_and_certify` that does not gate on
        // tier 3 — this skip would go quiet on faces the arm can say
        // nothing about. (The in-src rows below are exactly that
        // caller, deliberately: they run open euler scaffolds below
        // tier 3's bar, and their seed faces still carry the mvfs
        // placeholder. They are why this line is reachable at all.)
        if matches!(
            body.get_face(f).and_then(|d| body.surfaces.get(d.surface)),
            Some(geom::Surface::Nurbs(p)) if p.is_placeholder()
        ) {
            continue;
        }
        let Some(solid) = solid_of(f) else { continue };
        let pts = face_points(f);
        if pts.is_empty() {
            // No boundary vertex at all: an unbounded face (empty outer
            // loop) or one whose boundary does not resolve. Either way
            // this arm cannot bound it, and A5's letter is that it must
            // then REFUSE rather than drop the face out of the sweep —
            // which is what this early-out did, silently, in the
            // function whose header forbids exactly that.
            //
            // `validate_closed`'s tier-2 check 1 refuses every empty
            // loop and `validate_pseudomanifold` runs it before the
            // census, so no body reaching here through the public door
            // is in this state; the refusal costs nothing and stays
            // loud if a second, ungated caller ever appears.
            errors.push(ValidationError::CensusUnsupported {
                entity: EntityId::Face(f),
            });
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
    // The vf-record deferral for the curved × planar arm (F5): a
    // planar face NAMED by a v-on-f record is the declared interface
    // the confirm pass and the exact sweeps examine (the boss-on-plate
    // acceptance class), so the face on the other side of THAT
    // interface defers to that verdict; a bogus record still errors
    // there as stale, so nothing blesses silently.
    //
    // The record's vertex must be a boundary vertex of the other face
    // of the pair, not merely a vertex of the other SOLID. The finding
    // being deferred is about the face pair, so the record that defers
    // it has to name both of its sides. A solid-granular test lets one
    // declared interface silence every other face of the same solid
    // against `f_planar` — including a face resting on it somewhere
    // else with no vertex evidence of its own, which is the class this
    // arm exists for. Widening this back to the solid is the UNSOUND
    // direction.
    //
    // The narrowing is a narrowing on every body that can reach here,
    // and that rests on a tier-1 fact rather than on the shape of the
    // test: `Reach::verts` is `{ v : vertex_faces[v] ∋ f }`, and the
    // old test read the solid of `vertex_faces[v]`'s smallest member,
    // so the new test could be true where the old was false ONLY for a
    // vertex incident to faces of two different solids — a vertex
    // orbit spanning two shells, which tier 1 pass 6 forbids and
    // `validate_pseudomanifold` checks (`validate_closed(body)?`)
    // before the census runs. So new ⊆ old here, strictly.
    let planar_face_bridged = |f_planar: FK, other: &BTreeSet<VertexKey>| -> bool {
        declared
            .vf
            .iter()
            .any(|&(v, vf)| vf == f_planar && other.contains(&v))
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
            // same-key planar pair would be on no list at all; the
            // deferral may only name an arm that takes the pair.
            // The `!planar` conjunct is a TAUTOLOGY, deliberately: one
            // surface key is one surface kind, and `planar` is
            // derived from the kind, so `same_key` already implies
            // `a.planar == b.planar`. It is written so the deferral
            // cannot quietly start covering pairs the conformal arm
            // never walks — which is what it did until §S49 narrowed
            // the skip above and made a planar pair reach this line.
            //
            // **Opposed senses is not a tautology, and it is the half
            // that makes the deferral true.** The bar this arm's docs
            // set is that a deferral names an arm which asks the SAME
            // question about the SAME pair; "the conformal arm walks
            // it" is a weaker test, and the gap between them is real.
            // [`sweep_conformal_patches`] reaches a same-sense pair and
            // `continue`s on it — aligned coincidence is flush material
            // within one solid, not contact (C1) — so it WALKS such a
            // pair and DECIDES nothing about it. Deferring on carrier
            // identity alone therefore hands a cross-solid same-key
            // same-sense pair to an arm that will drop it, and neither
            // arm ever answers this one's question (are these two
            // within reach and unexamined). Requiring opposed senses
            // here keeps exactly the pairs that arm returns a verdict
            // on; the rest fall through to the box test below, where a
            // separated pair still clears.
            let same_key_conformal = same_key
                && !a.planar
                && !b.planar
                && body
                    .get_face(a.face)
                    .zip(body.get_face(b.face))
                    .is_some_and(|(da, db)| da.sense != db.sense);
            if same_key_conformal || declared.faces.contains(&(a.face, b.face)) {
                continue; // the conformal arm's / the certifier's pair
            }
            let vf_deferred = (a.planar && planar_face_bridged(a.face, &b.verts))
                || (b.planar && planar_face_bridged(b.face, &a.verts));
            if vf_deferred {
                continue; // the declared interface — confirm pass's pair
            }
            // A DESCRIPTION with no claim in it refuses without a
            // distance test (`face_reach` docs — the loud direction).
            // Every surface KIND has a reach bound; what has none is a
            // placeholder patch, or a face whose boundary carries an
            // unboxable curve.
            let (Some((alo, ahi)), Some((blo, bhi))) = (a.boxed, b.boxed) else {
                errors.push(ValidationError::CensusUndecidable {
                    a: EntityId::Face(a.face),
                    b: EntityId::Face(b.face),
                    what: "a cross-solid face pair one of whose faces has no sound \
                           cheap reach bound — a placeholder surface, or a boundary \
                           carrying a curve with no sound box — the exclusion ring \
                           is the certified excluder",
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
                    what: "cross-solid faces within reach, at least one of them with \
                           a curved carrier or a curved boundary — the conformal-rest / \
                           proximity / partial-embedding class the exclusion ring \
                           will examine",
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
        // The CONTAINED side's hull, and it may be any subset of the
        // solid's locus (this arm's own comment below). A face that
        // contributes no vertices shrinks the hull, which makes
        // containment easier to claim and separation harder — the loud
        // direction on both branches — so skipping it here is sound
        // where the same skip in the `reaches` build above is not.
        // Arm 1 has already refused the face itself.
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
            // No deferral on records here, deliberately (arm 2's docs
            // carry the argument): every record in `ContactRecords`
            // states one coincidence, and this arm's question is where
            // one instance sits relative to another. A pair carrying
            // records is examined exactly like a pair carrying none.
            for (outer, inner, ilo, ihi) in [(sa, sb, blo, bhi), (sb, sa, alo, ahi)] {
                let Some((olo, ohi)) = solid_reach.get(&outer).copied().flatten() else {
                    errors.push(ValidationError::CensusUndecidable {
                        a: EntityId::Solid(outer),
                        b: EntityId::Solid(inner),
                        what: "a surface kind with no cheap sound box leaves the \
                               containing instance's extent unclaimable — the \
                               same interference class",
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
                            "one instance's extent box inside another's — the \
                             interference class (recorded gate-skips do not exist yet)"
                        } else {
                            "instance extent boxes not definitely separable from \
                             containment — the same interference class, in band"
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
    use geom::Surface;
    use geom_core::Tol;
    use geom_core::Vec3;

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
        use geom::Curve3;
        use geom_brep::{EdgeCurveSpec, EdgeGeometry};
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
                Tol::witness(),
            )
            .unwrap();
        let e_r = body
            .mev_line(
                MevSite::Fan {
                    he1: e_b.he_minus,
                    he2: e_b.he_minus,
                },
                p11,
                Tol::witness(),
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
                Tol::witness(),
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
                Tol::witness(),
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
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
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

    /// Every error these scaffold rows may legitimately still carry:
    /// the containment arm's solid-granular refusal.
    ///
    /// **Why it fires, exactly — and it is not about where the sheets
    /// are.** Each `cyl_sheet` is its own `mvfs` seed, so a two-sheet
    /// fixture is a two-INSTANCE body; and each seed FACE keeps the
    /// mvfs NURBS placeholder, whose control net is poison. A solid
    /// carrying a face with no claimable box can never be the container
    /// ([`sweep_cross_solid_backstop`] arm 2), so both directions refuse
    /// with *"a surface kind with no cheap sound box leaves the
    /// containing instance's extent unclaimable"*. The refusal is
    /// therefore position-independent: moving one sheet a kilometre
    /// away produces the identical pair of errors, because no extent
    /// comparison happens at all. Do not read these rows as the arm
    /// measuring the sheets.
    ///
    /// These fixtures sit below tier 3's closed-body bar and call
    /// `census_and_certify` directly, which is the only door that
    /// reaches a placeholder face; through `validate_pseudomanifold` the
    /// state cannot be minted.
    ///
    /// The filter is by ENTITY GRANULARITY rather than by count, so a
    /// face-granular finding — which is what these rows are about —
    /// can never hide inside the allowance. What it does admit,
    /// deliberately and without bound, is any number of `Solid`×`Solid`
    /// refusals of EITHER of arm 2's two `what` strings. That width is
    /// the price of not pinning a `what` string in a row about the
    /// conformal doors; the paragraph above is what keeps it honest,
    /// and it is a paragraph, not a check.
    fn face_findings(errors: &[ValidationError]) -> Vec<&ValidationError> {
        errors
            .iter()
            .filter(|e| {
                !matches!(
                    e,
                    ValidationError::CensusUndecidable {
                        a: EntityId::Solid(_),
                        b: EntityId::Solid(_),
                        ..
                    }
                )
            })
            .collect()
    }

    /// **§H14 — what the containment arm actually says about these
    /// scaffolds, and that it is not about where they are.**
    ///
    /// Each `cyl_sheet` keeps its `mvfs` NURBS placeholder on the seed
    /// face. A placeholder net is poison, so the solid has no claimable
    /// extent and can never be the container: arm 2 must reach its
    /// TYPED refusal for that case. Folding the poison net instead
    /// yields `Some((NaN, NaN))`, every margin decides neither sign,
    /// and the arm falls out at its in-band refusal having compared no
    /// geometry — which reads, in a suite, exactly like the arm
    /// measuring two overlapping sheets.
    ///
    /// The second half is what makes that unmistakable: a sheet a
    /// KILOMETRE away produces the byte-identical refusal. If this row
    /// ever fails there, an extent comparison has started happening and
    /// the `face_findings` allowance above needs re-reading.
    #[test]
    fn a_placeholder_seed_leaves_the_containing_extent_unclaimable() {
        let refusals = |z0: f64, z1: f64| -> Vec<String> {
            let mut body = Body::<f64>::new();
            let (_w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
            let (_w2, _) = cyl_sheet(&mut body, Some(cyl), 1.0, 2.4, z0, z1, false);
            crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
            census_and_certify(&body, &ContactRecords::default(), band())
                .into_iter()
                .filter_map(|e| match e {
                    ValidationError::CensusUndecidable {
                        a: EntityId::Solid(a),
                        b: EntityId::Solid(b),
                        what,
                    } => Some(format!("{a:?}~{b:?}: {what}")),
                    _ => None,
                })
                .collect()
        };
        let near = refusals(0.3, 0.7);
        assert!(
            !near.is_empty()
                && near.iter().all(|r| r.contains(
                    "a surface kind with no cheap sound box leaves the containing \
                     instance's extent unclaimable"
                )),
            "the placeholder seed must reach the TYPED refusal, not the in-band \
             fallout of a NaN box: {near:?}"
        );
        assert_eq!(
            near,
            refusals(1000.3, 1000.7),
            "the refusal is about an unclaimable extent, so moving a sheet a \
             kilometre away must not change it"
        );
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
        let findings = face_findings(&errors);
        assert!(
            findings.is_empty(),
            "the declared conformal patch certifies: {findings:?} (all: {errors:?})"
        );
    }

    #[test]
    fn a_disjoint_patch_record_is_stale_typed() {
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
        let (w3, _) = cyl_sheet(&mut body, Some(cyl), 3.0, 4.0, 0.0, 1.0, false);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
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
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
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
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
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
        let findings = face_findings(&cert);
        assert!(
            findings.is_empty(),
            "the backed next-branch pair certifies: {findings:?} (all: {cert:?})"
        );
    }

    #[test]
    fn an_aligned_same_sense_patch_record_is_contradicted() {
        // Same key, SAME sense: aligned coincidence is containment or
        // flush material, never contact (C1) — the record lies.
        let mut body = Body::<f64>::new();
        let (w1, cyl) = cyl_sheet(&mut body, None, 0.2, 1.6, 0.0, 1.0, true);
        let (w2, _) = cyl_sheet(&mut body, Some(cyl), 1.0, 2.4, 0.3, 0.7, true);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
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
