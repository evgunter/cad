//! Shared test fixtures: canonical well-formed half-edge bodies, built
//! through the raw builder (insert with provisional null keys, then
//! patch — see the builder notes in [`crate::body`]).
//!
//! Test-only (`#[cfg(test)]` at the declaration site). Three families:
//!
//! - [`ngon_pillow`] — the minimal *closed* body family: two n-gon faces
//!   glued along an n-cycle of edges ("pillow"). `n = 2` is the digon
//!   pillow (v2 e2 f2 — Euler–Poincaré 2−2+2 = 2, genus 0), the smallest
//!   closed manifold body and the successor of M0's single-face `tiny()`;
//!   `n = 1` is the legal self-loop digon (one vertex, one edge, both
//!   halves in different faces' one-half-edge loops).
//! - [`prism`] — 2 n-gon caps + n quads (v = 2n, e = 3n, f = n + 2);
//!   every vertex has valence 3, exercising nontrivial vertex orbits.
//! - [`mvfs_state`] — the skeletal body `mvfs` will create in PR 2:
//!   solid + shell + one face whose outer loop is `Empty`, holding a
//!   lone vertex. Tier-1-legal by design.
//!
//! All geometry is placeholder (structural validation never reads scalar
//! values). Coordinates are index-derived placeholders, **not** faithful
//! positions (the prism's points are collinear, not an n-gon); the
//! documented geometric pictures live in the doc comments here and in
//! the topology itself, and that is what orientation reasoning in tests
//! points at.

// Test-support code: panicking is a test's failure mechanism (L5), and
// fixture unwraps are on keys the fixture itself just minted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopBoundary, LoopKey,
    Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
use crate::geometry::{CurveGeom, CurveKey, PointKey, SurfaceGeom, SurfaceKey};
use crate::provenance::Provenance;

/// The fixture provenance (all fixture entities share it).
pub(crate) fn prov() -> Provenance {
    Provenance::Primordial { op: "fixture" }
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
pub(crate) fn ngon_pillow(n: usize) -> NgonPillow {
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
        .map(|_| {
            body.add_curve(CurveGeom::Placeholder {
                anchor: Point3::origin(),
            })
        })
        .collect();
    let surface_a = body.add_surface(SurfaceGeom::Placeholder {
        anchor: Point3::origin(),
    });
    let surface_b = body.add_surface(SurfaceGeom::Placeholder {
        anchor: Point3::origin(),
    });

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
            surface: surface_a,
            outer: loop_a,
            rings: vec![],
            shell,
        },
        prov(),
    );
    let face_b = body.add_face(
        Face {
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
pub(crate) fn pillow() -> NgonPillow {
    ngon_pillow(2)
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
pub(crate) fn prism(n: usize) -> Prism {
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
    let mut curve = || {
        body.add_curve(CurveGeom::Placeholder {
            anchor: Point3::origin(),
        })
    };
    let curves_t: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let curves_b: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let curves_v: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let mut surface = || {
        body.add_surface(SurfaceGeom::Placeholder {
            anchor: Point3::origin(),
        })
    };
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
    let surface = body.add_surface(SurfaceGeom::Placeholder {
        anchor: Point3::origin(),
    });
    let lone_loop = body.add_loop(
        Loop {
            boundary: LoopBoundary::Empty { vertex },
            face: FaceKey::default(),
        },
        prov(),
    );
    let face = body.add_face(
        Face {
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
