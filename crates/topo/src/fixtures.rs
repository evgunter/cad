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
//! - [`prism`] — 2 n-gon caps + n quads (v = 2n, e = 3n, f = n + 2);
//!   every vertex has valence 3, exercising nontrivial vertex orbits.
//! - [`mvfs_state`] — the skeletal body `mvfs` creates: solid + shell +
//!   one face whose outer loop is `Empty`, holding a lone vertex.
//!   Tier-1-legal by design.
//!
//! Plus two whole-body observations the suites compare by —
//! [`arena_snapshot`] (every arena's length) and [`deep_snapshot`]
//! (key-for-key, field-for-field, provenance-for-provenance).
//!
//! Plus (M1 PR 4) two **operator-built** fixtures — [`ops_cube`] and
//! [`ops_holed_box`] — the acceptance-test bodies rebuilt in-crate for
//! the kill-direction, oracle, and teardown tests.
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
use crate::euler_ring::{KemrResult, KfmrhResult};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::provenance::Provenance;
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
pub(crate) struct Prism {
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
/// vertex has valence 3. See [`Prism`] for the orientation picture.
///
/// # Panics
///
/// If `n < 2` (fixture misuse, not kernel behavior).
pub(crate) fn prism(n: usize, tol: Tol) -> Prism {
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

    Prism {
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
// test (which needs crate access to the provenance maps). The
// construction sequences mirror `tests/cube_by_hand.rs` and
// `tests/box_with_hole.rs`.
// ---------------------------------------------------------------------

/// Key bundle for [`ops_cube`]: every operator result in call order.
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsCube {
    pub body: Body<f64>,
    pub seed: MvfsCreated,
    /// `[e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd]` — the bottom chain
    /// then the four verticals.
    pub mevs: [MevCreated; 7],
    /// `[f_bottom, f_front, f_right, f_back, f_left]`; the seed face
    /// remains as the top.
    pub mefs: [MefCreated; 5],
}

/// Builds the unit cube through the operators (1 mvfs + 7 mev + 5 mef,
/// the §9.4.2-minimal sequence; same construction as the PR 2
/// acceptance test).
pub(crate) fn ops_cube(tol: Tol) -> OpsCube {
    let pt = Point3::new;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap(); // A
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, x, y, z| {
        body.mev_line(MevSite::Fan { he1: at, he2: at }, pt(x, y, z), tol)
            .unwrap()
    };
    let mef =
        |body: &mut Body<f64>, he1, he2| body.mef_chord(MefSite::Chords { he1, he2 }, tol).unwrap();
    let e_bc = strut(&mut body, e_ab.he_minus, 1.0, 1.0, 0.0);
    let e_cd = strut(&mut body, e_bc.he_minus, 0.0, 1.0, 0.0);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = mef(&mut body, he_dc, e_ab.he_plus);
    let e_aa = strut(&mut body, e_ab.he_plus, 0.0, 0.0, 1.0);
    let e_bb = strut(&mut body, e_bc.he_plus, 1.0, 0.0, 1.0);
    let e_cc = strut(&mut body, e_cd.he_plus, 1.0, 1.0, 1.0);
    let e_dd = strut(&mut body, f_bottom.he_plus, 0.0, 1.0, 1.0);
    let f_front = mef(&mut body, e_aa.he_minus, e_bb.he_minus);
    let f_right = mef(&mut body, e_bb.he_minus, e_cc.he_minus);
    let f_back = mef(&mut body, e_cc.he_minus, e_dd.he_minus);
    let f_left = mef(&mut body, e_dd.he_minus, f_front.he_plus);
    OpsCube {
        body,
        seed,
        mevs: [e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd],
        mefs: [f_bottom, f_front, f_right, f_back, f_left],
    }
}

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
    let OpsCube {
        mut body,
        seed,
        mevs,
        mefs,
    } = ops_cube(tol);
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
    let v = body.vertices().count() as i64;
    let e = body.edges().count() as i64;
    let f = body.faces().count() as i64;
    let r: i64 = body.faces().map(|(_, fd)| fd.rings.len() as i64).sum();
    assert_eq!((v, e, f, r), (22, 33, 13, 4));
    assert_eq!(v - e + f - r, -2, "genus 2");
    assert_eq!(crate::validate::validate(&body), Ok(()));
    body
}

/// `text` with every **comment** and every **literal body** blanked —
/// the shared answer to *"is this text code?"* for the crate's textual
/// guards.
///
/// Lives here for [`src_root`]'s reason one paragraph up: the guards
/// that walk this crate's sources each carried their own
/// `trim_start().starts_with("//")` line test, which is blind to a
/// `/* … */` block, to `#[doc = "…"]`, and to a needle sitting inside a
/// string. A guard against duplication should not be the next copy of
/// its own walk, and the *predicate* is the part that was being copied.
///
/// Removed: `//` line comments (anywhere on the line, not only at its
/// start), `/* … */` blocks (nested), and the CONTENTS of string, byte
/// and char literals — nothing that a read of an identifier can hide
/// inside, so blanking them can only remove false positives, never
/// create a false negative. Byte offsets and line structure are
/// preserved (newlines survive; removed bytes become spaces), so a
/// caller may still count lines.
///
/// **What it does not model**, because `topo/src` contains none and a
/// guard that silently mis-parses is worse than one that says so:
/// **raw strings** (`r"…"`, `r#"…"#`) are treated as an ordinary
/// string opened at the quote, which is correct unless the body
/// contains a backslash-quote pair; and an identifier assembled by a
/// macro (`concat_idents!`, `paste!`) is invisible to any textual walk.
pub(crate) fn code_only(text: &str) -> String {
    let b = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    // Blank a byte range, keeping newlines so line numbers survive.
    let blank = |out: &mut Vec<u8>, b: &[u8], from: usize, to: usize| {
        for &c in &b[from..to.min(b.len())] {
            out.push(if c == b'\n' { b'\n' } else { b' ' });
        }
    };
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                let start = i;
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                blank(&mut out, b, start, i);
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let (start, mut depth) = (i, 1usize);
                i += 2;
                while i + 1 < b.len() && depth > 0 {
                    if b[i] == b'/' && b[i + 1] == b'*' {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && b[i + 1] == b'/' {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                if depth > 0 {
                    i = b.len();
                }
                blank(&mut out, b, start, i);
            }
            b'"' => {
                let start = i;
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += usize::from(b[i] == b'\\') + 1;
                }
                i = (i + 1).min(b.len());
                blank(&mut out, b, start, i);
            }
            // A quote is a char literal only if it closes within one
            // character (or one escape). Otherwise it is a LIFETIME and
            // must stay code — mis-reading `'a` as an opening quote
            // would swallow the rest of the file.
            b'\'' => {
                let close = if b.get(i + 1) == Some(&b'\\') {
                    (i + 2..b.len().min(i + 8)).find(|&k| b[k] == b'\'')
                } else {
                    let mut k = i + 1;
                    // One UTF-8 scalar: 1 byte plus its continuations.
                    k += 1;
                    while k < b.len() && (b[k] & 0b1100_0000) == 0b1000_0000 {
                        k += 1;
                    }
                    (b.get(k) == Some(&b'\'')).then_some(k)
                };
                match close {
                    Some(k) => {
                        blank(&mut out, b, i, k + 1);
                        i = k + 1;
                    }
                    None => {
                        out.push(b'\'');
                        i += 1;
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8(out).unwrap_or_default()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod code_only_tests {
    /// The shared "is this text code?" predicate, pinned on the four
    /// shapes the line-prefix test it replaces was blind to, and on the
    /// one shape a naive quote-scanner breaks on (a lifetime).
    #[test]
    fn comments_and_literal_bodies_are_blanked_and_code_is_not() {
        let n = concat!("sense", "_sign");
        let blanked = [
            "// a line comment naming sense_sign",
            "/// a doc comment naming sense_sign",
            "//! a module comment naming sense_sign",
            "/* a block naming sense_sign */",
            "/* outer /* nested naming sense_sign */ still inside */",
            "let x = 1; // trailing comment naming sense_sign",
            "const S: &str = \"a string naming sense_sign\";",
            "#[doc = \"an attribute naming sense_sign\"]",
        ];
        for row in blanked {
            let out = super::code_only(row);
            assert_eq!(out.matches(n).count(), 0, "not blanked: {row}");
            assert_eq!(out.len(), row.len(), "byte offsets moved: {row}");
        }

        // Code survives, including code that FOLLOWS a blanked region
        // on the same line — the prefix test could not see it at all.
        for row in [
            "let s = f.sense_sign::<T>();",
            "/* c */ let s = f.sense_sign::<T>();",
            "let q = '\\'' ; let s = f.sense_sign::<T>();",
            // A double-quote CHAR literal: mis-read as an opening
            // string, this swallows the rest of the file.
            "if line.contains('\"') { let s = f.sense_sign::<T>(); }",
            // A lifetime is not a quote.
            "fn f<'a>(x: &'a str) -> T { x.sense_sign() }",
        ] {
            assert_eq!(super::code_only(row).matches(n).count(), 1, "lost: {row}");
        }

        // Newlines survive, so a caller may still count lines.
        let multi = "// sense_sign\nlet s = f.sense_sign();\n";
        assert_eq!(super::code_only(multi).lines().count(), 2);
    }
}
