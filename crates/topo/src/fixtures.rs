//! Shared test fixtures: canonical well-formed half-edge bodies, built
//! through the raw builder (insert with provisional null keys, then
//! patch — see the builder notes in [`crate::body`]).
//!
//! Test-only (`#[cfg(test)]` at the declaration site), so a `tests/`
//! binary cannot name any of it. This is one of three homes for test
//! vocabulary in this crate; `src/test_support_impl.rs`'s docs give the
//! rule for which one a new item belongs in. Three families:
//!
//! - [`ngon_pillow`] — the minimal *closed* body family: two n-gon faces
//!   glued along an n-cycle of edges ("pillow"). `n = 2` is the digon
//!   pillow (v2 e2 f2 — Euler–Poincaré 2−2+2 = 2, genus 0), the smallest
//!   closed manifold body and the successor of M0's single-face `tiny()`;
//!   `n = 1` is the legal self-loop digon (one vertex, one edge, both
//!   halves in different faces' one-half-edge loops).
//! - [`raw_prism`] — 2 n-gon caps + n quads (v = 2n, e = 3n, f = n + 2);
//!   every vertex has valence 3, exercising nontrivial vertex orbits.
//! - [`mvfs_state`] — the skeletal body `mvfs` creates: solid + shell +
//!   one face whose outer loop is `Empty`, holding a lone vertex.
//!   Tier-1-legal by design.
//!
//! Plus two whole-body observations the suites compare by —
//! [`arena_snapshot`] (every arena's length) and [`deep_snapshot`]
//! (key-for-key, field-for-field, provenance-for-provenance).
//!
//! Plus the **operator-built** family — [`ops_holed_box`] and
//! [`ops_genus2`], the acceptance-test bodies rebuilt in-crate for
//! the kill-direction, oracle, and teardown tests over
//! [`crate::test_support_fixtures::declined_cube`], and
//! [`ops_ring_bridge`] and [`ops_strut_cube`], the two shapes here
//! whose edge has both halves in one loop — the holed box with its hole
//! rim bridged back into the top face's outer loop, and the cube with a
//! pendant strut planted on that loop.
//!
//! All geometry is placeholder (structural validation never reads scalar
//! values). Coordinates are index-derived placeholders, **not** faithful
//! positions (the prism's points are collinear, not an n-gon); the
//! documented geometric pictures live in the doc comments here and in
//! the topology itself, and that is what orientation reasoning in tests
//! points at.
//!

// Test-support code: panicking is a test's failure mechanism (L5), and
// fixture unwraps are on keys the fixture itself just minted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopBoundary, LoopKey,
    Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
use crate::euler::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
use crate::euler_ring::{KemrResult, KfmrhResult, MekrResult, MekrSite};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::provenance::Provenance;
use crate::readback::euler_counts;
use crate::test_support_fixtures::{CubeOps, declined_cube};
use crate::test_support_impl::ArenaCounts;
use geom_core::Tol;

/// The fixture provenance (all fixture entities share it).
pub(crate) fn prov() -> Provenance {
    Provenance::Primordial { op: "fixture" }
}

/// A certified scaffolding curve for raw-insertion fixtures (M2 PR 3:
/// the curve arena holds certified `EdgeCurve`s and nothing else): the
/// canonical self-loop circle at `anchor` — deterministic, honest data,
/// no geometric claims about the fixture's topology (raw fixtures make
/// no validity promises anyway; the anchor keeps snapshots
/// per-call-site distinct like the M0 placeholder anchors did).
pub(crate) fn test_curve(anchor: Point3<f64>, tol: Tol) -> geom_brep::EdgeCurve<f64> {
    let spec = geom_brep::EdgeCurveSpec::self_loop_circle_at(anchor);
    geom_brep::EdgeCurve::certify(
        spec,
        anchor,
        anchor,
        |_| None,
        geom_core::Band::linear(tol).unwrap(),
    )
    .unwrap()
}

/// A raw-fixture surface: the `Nurbs` "no description yet" state (the
/// anchor argument is accepted for call-site symmetry with
/// [`test_curve`] and ignored — surfaces carry no certification).
pub(crate) fn test_surface(_anchor: Point3<f64>) -> geom::Surface<f64> {
    geom::Surface::nurbs_placeholder()
}

/// A raw plane SURFACE fixture (an operation input, not a face datum —
/// it belongs to no body, so there is no sense bit to fold and no
/// outward normal to mint; suites that need a material side for it
/// state one explicitly at the call site).
pub(crate) fn plane_surface(
    origin: Point3<f64>,
    normal: geom_core::Vec3<f64>,
    u_ref: geom_core::Vec3<f64>,
) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    }
}

/// All ten arena lengths of a body: the seven topology arenas, held as
/// the crate's one [`ArenaCounts`], plus the three geometry arenas.
/// The "body unchanged" snapshot of the atomicity tests, and the delta
/// base of the operator count checks.
///
/// The topology half is *not* restated here: an `ArenaSnapshot` is an
/// [`ArenaCounts`] extended by geometry, and the seven have exactly one
/// producer ([`Body::arena_counts`]), shared with the debug
/// postcondition that checks them against each operator's declared
/// delta.
///
/// A different quantity from [`crate::euler::ArenaDelta`]: these are
/// counts, not shifts, and they include geometry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ArenaSnapshot {
    pub counts: ArenaCounts,
    pub points: usize,
    pub curves: usize,
    pub surfaces: usize,
}

/// Captures every arena length of `body`.
pub(crate) fn arena_snapshot(body: &Body<f64>) -> ArenaSnapshot {
    ArenaSnapshot {
        counts: body.arena_counts(),
        points: body.points().count(),
        curves: body.curves().count(),
        surfaces: body.surfaces().count(),
    }
}

/// A deep, order-sensitive snapshot of a body: one line per arena entry
/// (all ten arenas, in slot-index order) carrying the **full payload**
/// plus the entity's D5 provenance record.
///
/// For atomicity and lineage-purity tests where counts-only comparison
/// is too weak: two snapshots compare equal iff the bodies are
/// key-for-key, field-for-field, provenance-for-provenance identical.
/// (PR 4's kill operators will need exactly this — a kill that removes
/// the wrong entity or leaks a provenance record still preserves
/// counts.) Payloads are compared through their `Debug` forms, which
/// for these types print every field.
pub(crate) fn deep_snapshot(body: &Body<f64>) -> Vec<String> {
    let mut lines = Vec::new();
    for (k, e) in body.solids() {
        lines.push(format!(
            "solid {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::Solid(k))
        ));
    }
    for (k, e) in body.shells() {
        lines.push(format!(
            "shell {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::Shell(k))
        ));
    }
    for (k, e) in body.faces() {
        lines.push(format!(
            "face {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::Face(k))
        ));
    }
    for (k, e) in body.loops() {
        lines.push(format!(
            "loop {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::Loop(k))
        ));
    }
    for (k, e) in body.half_edges() {
        lines.push(format!(
            "half-edge {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::HalfEdge(k))
        ));
    }
    for (k, e) in body.edges() {
        lines.push(format!(
            "edge {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::Edge(k))
        ));
    }
    for (k, e) in body.vertices() {
        lines.push(format!(
            "vertex {k:?}: {e:?} prov={:?}",
            body.provenance(EntityId::Vertex(k))
        ));
    }
    for (k, e) in body.points() {
        lines.push(format!("point {k:?}: {e:?}"));
    }
    for (k, e) in body.curves() {
        lines.push(format!("curve {k:?}: {e:?}"));
    }
    for (k, e) in body.surfaces() {
        lines.push(format!("surface {k:?}: {e:?}"));
    }
    lines
}

/// A distinct-per-index placeholder coordinate (`u32` round trip keeps
/// the cast lossless; fixture sizes are tiny).
fn index_coord(i: usize) -> f64 {
    f64::from(u32::try_from(i).unwrap_or(u32::MAX))
}

/// Key bundle for [`ngon_pillow`] (and [`pillow`], its n = 2 instance).
///
/// Naming: face A is the "front" n-gon, face B the "back". Edge `e[i]`
/// runs `v[i] → v[(i+1) % n]` in its intrinsic (plus) direction.
/// `hes_a[i]` is `e[i]`'s half in loop A (start `v[i]`, the plus half);
/// `hes_b[i]` is its half in loop B (start `v[(i+1) % n]`, the minus
/// half). Loop A walks `v0 → v1 → …`; loop B walks the same rim the
/// other way (`next(hes_b[i]) = hes_b[i-1]`), per antiparallelism.
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct NgonPillow {
    pub body: Body<f64>,
    pub points: Vec<PointKey>,
    pub curves: Vec<CurveKey>,
    pub surface_a: SurfaceKey,
    pub surface_b: SurfaceKey,
    pub vertices: Vec<VertexKey>,
    pub edges: Vec<EdgeKey>,
    pub hes_a: Vec<HalfEdgeKey>,
    pub hes_b: Vec<HalfEdgeKey>,
    pub loop_a: LoopKey,
    pub loop_b: LoopKey,
    pub face_a: FaceKey,
    pub face_b: FaceKey,
    pub shell: ShellKey,
    pub solid: SolidKey,
}

/// Builds the n-gon pillow (n ≥ 1): two faces glued along an n-cycle.
///
/// Counts: v = n, e = n, f = 2, so v − e + f = 2 (sphere, genus 0) for
/// every n. Vertex `v[i]`'s emanating half-edge is `hes_a[i]`; its orbit
/// is `[hes_a[i], hes_b[(i-1+n) % n]]` (both halves starting at `v[i]`).
///
/// # Panics
///
/// If `n == 0` (fixture misuse, not kernel behavior).
pub(crate) fn ngon_pillow(n: usize, tol: Tol) -> NgonPillow {
    assert!(n >= 1, "an n-gon pillow needs at least one edge");
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();

    // Spine, top down, patching the upward lists as children arrive.
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);

    // Geometry: n rim points (values irrelevant to structural checks),
    // one placeholder curve per edge, one placeholder surface per face.
    let points: Vec<PointKey> = (0..n)
        .map(|i| body.add_point(Point3::new(index_coord(i), 0.0, 0.0)))
        .collect();
    let curves: Vec<CurveKey> = (0..n)
        .map(|_| body.add_curve(test_curve(Point3::origin(), tol)))
        .collect();
    let surface_a = body.add_surface(test_surface(Point3::origin()));
    let surface_b = body.add_surface(test_surface(Point3::origin()));

    // Vertices (emanating patched once half-edges exist).
    let vertices: Vec<VertexKey> = points
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();

    // Edges with provisional half-edge slots.
    let edges: Vec<EdgeKey> = curves
        .iter()
        .map(|&curve| {
            body.add_edge(
                Edge {
                    he_plus: null_he,
                    he_minus: null_he,
                    curve,
                },
                prov(),
            )
        })
        .collect();

    // Half-edges with provisional links.
    let hes_a: Vec<HalfEdgeKey> = (0..n)
        .map(|i| {
            body.add_half_edge(
                HalfEdge {
                    edge: edges[i],
                    start: vertices[i],
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        })
        .collect();
    let hes_b: Vec<HalfEdgeKey> = (0..n)
        .map(|i| {
            body.add_half_edge(
                HalfEdge {
                    edge: edges[i],
                    start: vertices[(i + 1) % n],
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        })
        .collect();

    // Loops and faces, then patch the loop→face back-pointers.
    let loop_a = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: hes_a[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let loop_b = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: hes_b[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let face_a = body.add_face(
        Face {
            sense: true,
            surface: surface_a,
            outer: loop_a,
            rings: vec![],
            shell,
        },
        prov(),
    );
    let face_b = body.add_face(
        Face {
            sense: true,
            surface: surface_b,
            outer: loop_b,
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_loop_mut(loop_a).unwrap().face = face_a;
    body.get_loop_mut(loop_b).unwrap().face = face_b;
    body.get_shell_mut(shell).unwrap().faces = vec![face_a, face_b];

    // Close the cycles: loop A walks the rim forward, loop B backward.
    for i in 0..n {
        let a = body.get_half_edge_mut(hes_a[i]).unwrap();
        a.parent_loop = loop_a;
        a.next = hes_a[(i + 1) % n];
        a.prev = hes_a[(i + n - 1) % n];
        let b = body.get_half_edge_mut(hes_b[i]).unwrap();
        b.parent_loop = loop_b;
        b.next = hes_b[(i + n - 1) % n];
        b.prev = hes_b[(i + 1) % n];
    }
    // Claim the halves: the A half is the plus (intrinsic) direction.
    for i in 0..n {
        let e = body.get_edge_mut(edges[i]).unwrap();
        e.he_plus = hes_a[i];
        e.he_minus = hes_b[i];
    }
    // Anchor the vertices.
    for i in 0..n {
        body.get_vertex_mut(vertices[i]).unwrap().emanating = Some(hes_a[i]);
    }

    NgonPillow {
        body,
        points,
        curves,
        surface_a,
        surface_b,
        vertices,
        edges,
        hes_a,
        hes_b,
        loop_a,
        loop_b,
        face_a,
        face_b,
        shell,
        solid,
    }
}

/// The digon pillow — the minimal closed fixture (see [`ngon_pillow`]).
pub(crate) fn pillow(tol: Tol) -> NgonPillow {
    ngon_pillow(2, tol)
}

/// Key bundle for [`prism`].
///
/// Geometric picture (documented so orientation tests can point at it):
/// rim indices increase **counterclockwise viewed from above** (+z);
/// bottom cap at z = 0 (outward normal −z), top cap at z = 1 (outward
/// normal +z), side quads with outward normals radially out. Loops obey
/// the interior-left rule ([`crate::entity`] module docs):
///
/// - top cap: `t0 → t1 → … → t(n−1)` (counterclockwise seen from above
///   = from outside);
/// - bottom cap: `… → u1 → u0 → u(n−1) → …` (decreasing index — that is
///   counterclockwise seen from *below*, i.e. from outside);
/// - side quad `i`: `u[i] → u[i+1] → t[i+1] → t[i]`.
///
/// Edges (intrinsic/plus directions): `et[i]: t[i] → t[i+1]` (plus half
/// `ht[i]` in the top cap), `eb[i]: u[i] → u[i+1]` (plus half `s0[i]` in
/// side `i`), `ev[i]: u[i] → t[i]` (plus half `s1[(i−1+n) % n]` in side
/// `i−1`). Side quad `i`'s cycle is `s0[i] → s1[i] → s2[i] → s3[i]`
/// with starts `u[i], u[i+1], t[i+1], t[i]`.
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct RawPrism {
    pub body: Body<f64>,
    pub t: Vec<VertexKey>,
    pub u: Vec<VertexKey>,
    pub ht: Vec<HalfEdgeKey>,
    pub hb: Vec<HalfEdgeKey>,
    pub s0: Vec<HalfEdgeKey>,
    pub s1: Vec<HalfEdgeKey>,
    pub s2: Vec<HalfEdgeKey>,
    pub s3: Vec<HalfEdgeKey>,
    pub et: Vec<EdgeKey>,
    pub eb: Vec<EdgeKey>,
    pub ev: Vec<EdgeKey>,
    pub loop_top: LoopKey,
    pub loop_bottom: LoopKey,
    pub loop_side: Vec<LoopKey>,
    pub face_top: FaceKey,
    pub face_bottom: FaceKey,
    pub face_side: Vec<FaceKey>,
    pub shell: ShellKey,
    pub solid: SolidKey,
}

/// Builds the n-prism (n ≥ 2): two n-gon caps plus n side quads.
///
/// Counts: v = 2n, e = 3n, f = n + 2, so v − e + f = 2 (genus 0); every
/// vertex has valence 3. See [`RawPrism`] for the orientation picture.
///
/// # Panics
///
/// If `n < 2` (fixture misuse, not kernel behavior).
pub(crate) fn raw_prism(n: usize, tol: Tol) -> RawPrism {
    assert!(n >= 2, "a prism needs at least a digon cap");
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();

    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);

    // Geometry. Coordinates are indexed placeholders standing in for the
    // documented picture (indices counterclockwise from above; bottom
    // z = 0, top z = 1); structural validation never reads them.
    let top_points: Vec<PointKey> = (0..n)
        .map(|i| body.add_point(Point3::new(index_coord(i), 0.0, 1.0)))
        .collect();
    let bottom_points: Vec<PointKey> = (0..n)
        .map(|i| body.add_point(Point3::new(index_coord(i), 0.0, 0.0)))
        .collect();
    let mut curve = || body.add_curve(test_curve(Point3::origin(), tol));
    let curves_t: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let curves_b: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let curves_v: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let mut surface = || body.add_surface(test_surface(Point3::origin()));
    let surface_top = surface();
    let surface_bottom = surface();
    let surface_side: Vec<SurfaceKey> = (0..n).map(|_| surface()).collect();

    let t: Vec<VertexKey> = top_points
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();
    let u: Vec<VertexKey> = bottom_points
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();

    let mut edge = |curve: CurveKey| {
        body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve,
            },
            prov(),
        )
    };
    let et: Vec<EdgeKey> = curves_t.iter().map(|&c| edge(c)).collect();
    let eb: Vec<EdgeKey> = curves_b.iter().map(|&c| edge(c)).collect();
    let ev: Vec<EdgeKey> = curves_v.iter().map(|&c| edge(c)).collect();

    let mut half_edge = |edge: EdgeKey, start: VertexKey| {
        body.add_half_edge(
            HalfEdge {
                edge,
                start,
                parent_loop: LoopKey::default(),
                next: null_he,
                prev: null_he,
            },
            prov(),
        )
    };
    // Top cap: ht[i] runs t[i] → t[i+1].
    let ht: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(et[i], t[i])).collect();
    // Bottom cap: hb[i] runs u[i+1] → u[i] (edge eb[i] backward).
    let hb: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(eb[i], u[(i + 1) % n])).collect();
    // Side quad i: u[i] → u[i+1] → t[i+1] → t[i].
    let s0: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(eb[i], u[i])).collect();
    let s1: Vec<HalfEdgeKey> = (0..n)
        .map(|i| half_edge(ev[(i + 1) % n], u[(i + 1) % n]))
        .collect();
    let s2: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(et[i], t[(i + 1) % n])).collect();
    let s3: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(ev[i], t[i])).collect();

    // Loops and faces.
    let mut cycle_loop = |first: HalfEdgeKey| {
        body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first },
                face: FaceKey::default(),
            },
            prov(),
        )
    };
    let loop_top = cycle_loop(ht[0]);
    let loop_bottom = cycle_loop(hb[0]);
    let loop_side: Vec<LoopKey> = (0..n).map(|i| cycle_loop(s0[i])).collect();

    let mut face = |surface: SurfaceKey, outer: LoopKey| {
        body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings: vec![],
                shell,
            },
            prov(),
        )
    };
    let face_top = face(surface_top, loop_top);
    let face_bottom = face(surface_bottom, loop_bottom);
    let face_side: Vec<FaceKey> = (0..n)
        .map(|i| face(surface_side[i], loop_side[i]))
        .collect();

    body.get_loop_mut(loop_top).unwrap().face = face_top;
    body.get_loop_mut(loop_bottom).unwrap().face = face_bottom;
    for i in 0..n {
        body.get_loop_mut(loop_side[i]).unwrap().face = face_side[i];
    }
    {
        let faces = &mut body.get_shell_mut(shell).unwrap().faces;
        faces.push(face_top);
        faces.push(face_bottom);
        faces.extend(&face_side);
    }

    // Close the cycles.
    let link = |body: &mut Body<f64>, cycle: &[HalfEdgeKey], parent: LoopKey| {
        let len = cycle.len();
        for (i, &he) in cycle.iter().enumerate() {
            let h = body.get_half_edge_mut(he).unwrap();
            h.parent_loop = parent;
            h.next = cycle[(i + 1) % len];
            h.prev = cycle[(i + len - 1) % len];
        }
    };
    link(&mut body, &ht, loop_top);
    // Bottom cap walks hb in DECREASING index order (see the struct
    // docs), so the "cycle" slice is reversed.
    let hb_walk: Vec<HalfEdgeKey> = hb.iter().rev().copied().collect();
    link(&mut body, &hb_walk, loop_bottom);
    for i in 0..n {
        link(&mut body, &[s0[i], s1[i], s2[i], s3[i]], loop_side[i]);
    }

    // Claim the halves (plus = intrinsic direction, per the struct docs).
    for i in 0..n {
        let e = body.get_edge_mut(et[i]).unwrap();
        e.he_plus = ht[i];
        e.he_minus = s2[i];
        let e = body.get_edge_mut(eb[i]).unwrap();
        e.he_plus = s0[i];
        e.he_minus = hb[i];
        let e = body.get_edge_mut(ev[i]).unwrap();
        e.he_plus = s1[(i + n - 1) % n];
        e.he_minus = s3[i];
    }

    // Anchor the vertices.
    for i in 0..n {
        body.get_vertex_mut(t[i]).unwrap().emanating = Some(ht[i]);
        body.get_vertex_mut(u[i]).unwrap().emanating = Some(s0[i]);
    }

    RawPrism {
        body,
        t,
        u,
        ht,
        hb,
        s0,
        s1,
        s2,
        s3,
        et,
        eb,
        ev,
        loop_top,
        loop_bottom,
        loop_side,
        face_top,
        face_bottom,
        face_side,
        shell,
        solid,
    }
}

/// Key bundle for [`mvfs_state`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct MvfsState {
    pub body: Body<f64>,
    pub point: PointKey,
    pub surface: SurfaceKey,
    pub vertex: VertexKey,
    pub lone_loop: LoopKey,
    pub face: FaceKey,
    pub shell: ShellKey,
    pub solid: SolidKey,
}

/// Builds the skeletal `mvfs` state: solid + shell + one face whose
/// outer loop is [`LoopBoundary::Empty`], holding a lone vertex
/// (`emanating: None`). No edges, no half-edges. Tier-1-legal by design
/// — this is the state every Euler construction starts from.
pub(crate) fn mvfs_state() -> MvfsState {
    let mut body = Body::<f64>::new();
    let point = body.add_point(Point3::origin());
    let vertex = body.add_vertex(
        Vertex {
            point,
            emanating: None,
        },
        prov(),
    );
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let surface = body.add_surface(test_surface(Point3::origin()));
    let lone_loop = body.add_loop(
        Loop {
            boundary: LoopBoundary::Empty { vertex },
            face: FaceKey::default(),
        },
        prov(),
    );
    let face = body.add_face(
        Face {
            sense: true,
            surface,
            outer: lone_loop,
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_loop_mut(lone_loop).unwrap().face = face;
    body.get_shell_mut(shell).unwrap().faces.push(face);

    MvfsState {
        body,
        point,
        surface,
        vertex,
        lone_loop,
        face,
        shell,
        solid,
    }
}

// ---------------------------------------------------------------------
// Operator-built fixtures (M1 PR 4): the acceptance-test bodies rebuilt
// through the public Euler operators, in-crate, for the kill-direction
// unit tests, the isomorphism-oracle tests, and the teardown property
// test (which needs crate access to the provenance maps). The cube each
// one grows from is `test_support_fixtures::declined_cube`, not a
// sequence written here; what is written here is the §9.3 surgery on
// top of it.
// ---------------------------------------------------------------------

/// Key bundle for [`ops_holed_box`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsHoledBox {
    pub body: Body<f64>,
    pub seed: MvfsCreated,
    pub box_mevs: [MevCreated; 7],
    pub box_mefs: [MefCreated; 5],
    pub strut: MevCreated,
    pub kill: KemrResult,
    pub rim_mevs: [MevCreated; 3],
    pub mef_top: MefCreated,
    pub tube_mevs: [MevCreated; 4],
    pub tube_mefs: [MefCreated; 4],
    pub plug: KfmrhResult,
}

/// Builds the box with a square through-hole (genus 1) through the
/// operators — the §9.3-minimal 1 mvfs + 15 mev + 10 mef + 1 kemr +
/// 1 kfmrh, same construction as the PR 3 acceptance test (on the unit
/// cube instead of the 2×2×2 box; coordinates are scaled, structure
/// identical).
pub(crate) fn ops_holed_box(tol: Tol) -> OpsHoledBox {
    let pt = Point3::new;
    let CubeOps {
        mut body,
        seed,
        mevs,
        mefs,
    } = declined_cube::<f64>(tol);
    let strut = |body: &mut Body<f64>, at, x, y, z| {
        body.mev_line(MevSite::Fan { he1: at, he2: at }, pt(x, y, z), tol)
            .unwrap()
    };
    let mef =
        |body: &mut Body<f64>, he1, he2| body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
    let f_bottom = mefs[0];
    let f_front = mefs[1];
    // (f)–(g): plant the hole anchor P as an empty ring of the top face.
    let hole_strut = strut(&mut body, f_front.he_plus, 0.25, 0.25, 1.0); // P
    let kill = body.kemr(hole_strut.he_plus, hole_strut.he_minus).unwrap();
    // (h)–(i): grow and close the rim P→Q→R→S; a membrane face covers
    // the opening.
    let s_pq = body
        .mev_line(
            MevSite::Lone { r#loop: kill.ring },
            pt(0.75, 0.25, 1.0),
            tol,
        )
        .unwrap(); // Q
    let s_qr = strut(&mut body, s_pq.he_minus, 0.75, 0.75, 1.0); // R
    let s_rs = strut(&mut body, s_qr.he_minus, 0.25, 0.75, 1.0); // S
    let mef_top = mef(&mut body, s_pq.he_plus, s_rs.he_minus);
    // (j)–(k): drop the verticals and cut the tube walls.
    let e_pp = strut(&mut body, s_pq.he_plus, 0.25, 0.25, 0.0);
    let e_qq = strut(&mut body, s_qr.he_plus, 0.75, 0.25, 0.0);
    let e_rr = strut(&mut body, s_rs.he_plus, 0.75, 0.75, 0.0);
    let e_ss = strut(&mut body, mef_top.he_minus, 0.25, 0.75, 0.0);
    let w_front = mef(&mut body, e_pp.he_minus, e_qq.he_minus);
    let w_right = mef(&mut body, e_qq.he_minus, e_rr.he_minus);
    let w_back = mef(&mut body, e_rr.he_minus, e_ss.he_minus);
    let he_pq_bottom = body
        .find_half_edge(mef_top.face, e_pp.vertex, e_qq.vertex)
        .unwrap();
    let w_left = mef(&mut body, e_ss.he_minus, he_pq_bottom);
    // (l): the connected sum — genus 1.
    let plug = body.kfmrh(f_bottom.face, mef_top.face).unwrap();
    assert_eq!(crate::validate::validate(&body), Ok(()));
    OpsHoledBox {
        body,
        seed,
        box_mevs: mevs,
        box_mefs: mefs,
        strut: hole_strut,
        kill,
        rim_mevs: [s_pq, s_qr, s_rs],
        mef_top,
        tube_mevs: [e_pp, e_qq, e_rr, e_ss],
        tube_mefs: [w_front, w_right, w_back, w_left],
        plug,
    }
}

/// Builds the genus-2 double-hole body: [`ops_holed_box`] plus a
/// triangular through-hole carved front → back (the PR 4 review's
/// recipe, compacted from `src/review_m1_pr4.rs`'s
/// `genus_two_double_hole_body_tears_down_to_nothing` — the annotated
/// original stays in the review artifact). Euler ledger check inside:
/// v − e + f − r = 22 − 33 + 13 − 4 = −2 = 2(1 − 2).
pub(crate) fn ops_genus2(tol: Tol) -> Body<f64> {
    let pt = Point3::new;
    let t = ops_holed_box(tol);
    let mut body = t.body;
    let f_front = t.box_mefs[1].face;
    let f_back = t.box_mefs[3].face;
    let front_outer = body.get_face(f_front).unwrap().outer;
    let LoopBoundary::Cycle { first: at } = body.get_loop(front_outer).unwrap().boundary else {
        panic!("front outer is a cycle");
    };
    let rim_pts = [pt(0.3, 0.0, 0.3), pt(0.7, 0.0, 0.3), pt(0.5, 0.0, 0.7)];
    let drop_pts = [pt(0.3, 1.0, 0.3), pt(0.7, 1.0, 0.3), pt(0.5, 1.0, 0.7)];
    // Plant the rim anchor as an empty ring of the front face, then
    // grow and close the triangular rim; a membrane face covers it.
    let strut = body
        .mev_line(MevSite::Fan { he1: at, he2: at }, rim_pts[0], tol)
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let mut rim: Vec<MevCreated> = vec![
        body.mev_line(MevSite::Lone { r#loop: kill.ring }, rim_pts[1], tol)
            .unwrap(),
    ];
    for rp in &rim_pts[2..] {
        let prev = rim.last().unwrap().he_minus;
        rim.push(
            body.mev_line(
                MevSite::Fan {
                    he1: prev,
                    he2: prev,
                },
                *rp,
                tol,
            )
            .unwrap(),
        );
    }
    let membrane = body
        .mef_chord(
            MefSite::Chords {
                he1: rim[0].he_plus,
                he2: rim.last().unwrap().he_minus,
            },
            tol,
        )
        .unwrap();
    // Drop the verticals, cut the tube walls, and connect the sum.
    let mut drops: Vec<MevCreated> = Vec::new();
    for (i, dp) in drop_pts.iter().enumerate() {
        let anchor = if i < rim.len() {
            rim[i].he_plus
        } else {
            membrane.he_minus
        };
        drops.push(
            body.mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                *dp,
                tol,
            )
            .unwrap(),
        );
    }
    for i in 0..drops.len() - 1 {
        body.mef_chord(
            MefSite::Chords {
                he1: drops[i].he_minus,
                he2: drops[i + 1].he_minus,
            },
            tol,
        )
        .unwrap();
    }
    let he_first_far = body
        .find_half_edge(membrane.face, drops[0].vertex, drops[1].vertex)
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: drops.last().unwrap().he_minus,
            he2: he_first_far,
        },
        tol,
    )
    .unwrap();
    body.kfmrh(f_back, membrane.face).unwrap();
    // Genus-2 checkpoint.
    let counts = euler_counts(&body);
    assert_eq!(
        (counts.v, counts.e, counts.f, counts.r, counts.s),
        (22, 33, 13, 4, 1)
    );
    assert_eq!(counts.genus(), Ok(2), "genus 2");
    assert_eq!(crate::validate::validate(&body), Ok(()));
    body
}

/// Key bundle for [`ops_ring_bridge`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsRingBridge {
    pub body: Body<f64>,
    /// The face whose outer loop carries the bridge — the holed box's
    /// top face, the one [`ops_holed_box`] leaves holding the hole rim
    /// as a ring.
    pub face: FaceKey,
    /// That face's outer loop: after the bridge, the merged cycle
    /// holding the former rim, the two bridge halves and the former
    /// outer.
    pub outer: LoopKey,
    /// The bridge edge. Both of its halves lie in
    /// [`OpsRingBridge::outer`], which is the shape [`Body::kemr`]
    /// requires of its two arguments.
    pub bridge: MekrResult,
}

/// Builds the holed box with its top face's hole rim **joined back into
/// that face's outer loop** by one `mekr` — [`ops_holed_box`] plus one
/// operator, so genus and shell count are unchanged and the body still
/// validates.
///
/// **The shape [`Body::kemr`] needs, which no other fixture here
/// presents.** `kemr` takes two halves of ONE edge lying in ONE loop;
/// every edge of [`crate::test_support_fixtures::declined_cube`], [`ops_holed_box`] and [`ops_genus2`]
/// borders two distinct faces, so its halves sit in two loops and
/// `kemr`'s plan phase refuses at `NotSameLoop` on every pair those
/// bodies present. A bridge edge is the M1 shape whose two halves share
/// a loop, and `mekr` is the door that makes one.
///
/// Both components of the split are **non-empty** — the rim halves on
/// one side of the bridge, the former outer's on the other — so `kemr`
/// here runs both of its `link_half_edges` splices rather than the one
/// a strut kill (whose ring side is empty) reaches.
pub(crate) fn ops_ring_bridge(tol: Tol) -> OpsRingBridge {
    let t = ops_holed_box(tol);
    let mut body = t.body;
    let face = t.seed.face;
    let (outer, rings) = {
        let data = body.get_face(face).unwrap();
        (data.outer, data.rings.clone())
    };
    assert_eq!(
        rings.len(),
        1,
        "the holed box's top face carries exactly the hole rim as a ring"
    );
    let LoopBoundary::Cycle { first: target } = body.get_loop(outer).unwrap().boundary else {
        panic!("the top face's outer loop is a cycle");
    };
    let LoopBoundary::Cycle { first: rim } = body.get_loop(rings[0]).unwrap().boundary else {
        panic!("the hole rim is a cycle");
    };
    let bridge = body
        .mekr_chord(MekrSite::Cycles { target, ring: rim }, tol)
        .unwrap();
    // The property the fixture exists for, asserted here so a change to
    // `mekr`'s splice cannot leave a consumer silently back at
    // `NotSameLoop`: the bridge's two halves share one loop, and it is
    // the face's outer.
    for half in [bridge.he_plus, bridge.he_minus] {
        assert_eq!(
            body.get_half_edge(half).unwrap().parent_loop,
            outer,
            "the bridge's halves must both lie in the merged outer loop"
        );
    }
    assert!(
        body.get_face(face).unwrap().rings.is_empty(),
        "the bridge consumed the top face's only ring"
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));
    OpsRingBridge {
        body,
        face,
        outer,
        bridge,
    }
}

/// Key bundle for [`ops_strut_cube`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsStrutCube {
    pub body: Body<f64>,
    /// The loop the strut hangs in — the seed (top) face's outer loop,
    /// the same loop [`ops_holed_box`] plants its hole anchor in.
    pub outer: LoopKey,
    /// The pendant edge. Its two halves lie in
    /// [`OpsStrutCube::outer`] and are **adjacent** there, which is the
    /// shape whose [`Body::kemr`] leaves an EMPTY ring side and
    /// therefore runs one splice rather than two.
    pub strut: MevCreated,
}

/// Builds the cube with one pendant strut planted on the top face's
/// outer loop — [`crate::test_support_fixtures::declined_cube`] plus one `mev_line` at a `Fan` site, which
/// is the state [`ops_holed_box`] passes through at its hole anchor and
/// kills with `kemr` in the next line.
///
/// **The shape whose `kemr` empties the ring side.** `kemr` splits its
/// loop's cycle at the two halves it is handed; when they are ADJACENT
/// the side strictly between them is empty, the ring loop is minted
/// `Empty` and only the old loop's splice runs. [`ops_ring_bridge`]'s
/// bridge edge is the other arm — both sides non-empty, both splices —
/// so the two fixtures together present both shapes of `kemr`'s
/// mutation phase.
pub(crate) fn ops_strut_cube(tol: Tol) -> OpsStrutCube {
    let t = declined_cube::<f64>(tol);
    let mut body = t.body;
    // The same site `ops_holed_box` plants its hole anchor at: a `Fan`
    // on the front face's plus half, which lies in the top face's loop.
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: t.mefs[1].he_plus,
                he2: t.mefs[1].he_plus,
            },
            Point3::new(0.25, 0.25, 1.0),
            tol,
        )
        .unwrap();
    let outer = body.get_half_edge(strut.he_plus).unwrap().parent_loop;
    // The property the fixture exists for, asserted here rather than
    // described: the halves share a loop AND follow one another in it,
    // so `kemr`'s ring side is the empty one.
    assert_eq!(
        body.get_half_edge(strut.he_minus).unwrap().parent_loop,
        outer,
        "the strut's halves must both lie in the loop it was planted in"
    );
    assert_eq!(
        body.get_half_edge(strut.he_plus).unwrap().next,
        strut.he_minus,
        "the strut's halves must be adjacent, or `kemr` splits off a cycle here"
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));
    OpsStrutCube { body, outer, strut }
}

// ---------------------------------------------------------------------
// The offset-fit door's subject
// ---------------------------------------------------------------------

/// A gently bowed polynomial patch over `[0,1]²` — a base whose offset
/// is genuinely not a NURBS, so the fit has real work to do.
///
/// **The bow is small on purpose, and how small is a property of the
/// eps matrix.** This patch goes through the `Tol` door, so its fit
/// target is the RUN's ε and the tightest ε the gate commits is
/// `1e-12` (`.github/workflows/ci.yml`, the `eps_rows` battery). The
/// residual the refinement loop reaches scales with the bow, and at
/// `BOW = 0.15` the loop exhausts its six rounds at `3.3e-10` — fine
/// at the default `1e-9`, red at `1e-12`. Scaled by `1e-4` the same
/// loop lands near `3e-14`, inside every row of the battery with room,
/// and the certificate's limbs stay nonzero, which is what the
/// bit-identity rows measure.
pub(crate) fn bowed_patch() -> geom::NurbsSurface<f64> {
    /// The bow's amplitude in `u`; the `v` bow is two thirds of it.
    const BOW: f64 = 1.5e-5;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(
                u,
                v,
                BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v,
            ));
        }
    }
    geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

/// The certified `Approx` surface of [`bowed_patch`]'s `+0.05` offset,
/// at the RUN's tolerance — the smallest subject that reaches the
/// offset-fit door, lifted to `T` verbatim
/// (`geom::ApproxSurface::map_scalar`, which carries the stored
/// certificate rather than re-deriving it, so the lift is available at
/// scalars that have no fit).
pub(crate) fn bowed_offset_approx<T: geom_core::Real>() -> geom::ApproxSurface<T> {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let minted =
        geom_brep::approx_offset_surface(std::sync::Arc::new(bowed_patch()), 0.05, tol, band)
            .expect("the bowed patch's offset fits at every eps row the gate commits");
    let geom::Surface::Approx(approx) = minted else {
        panic!("the mint door produces `Surface::Approx`");
    };
    approx.map_scalar(T::from_f64)
}

/// The `mvfs` seed body with [`bowed_offset_approx`] on its one face —
/// the smallest body whose check-1 walk reaches the offset-fit door.
pub(crate) fn approx_faced_body<T: geom_core::Decide>() -> (Body<T>, FaceKey) {
    let mut body = Body::<T>::new();
    let created = body
        .mvfs(Point3::new(T::zero(), T::zero(), T::zero()))
        .expect("mvfs has no preconditions");
    body.set_face_surface(
        created.face,
        crate::euler::FaceSurface::New(geom::Surface::Approx(std::sync::Arc::new(
            bowed_offset_approx::<T>(),
        ))),
    )
    .expect("the seed face takes a fresh surface");
    (body, created.face)
}
