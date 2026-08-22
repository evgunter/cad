//! Adversarial e2e review artifact for M0 PR 7 (topology arenas,
//! 2026-07-15/16), salvaged from the M0 orchestrator session scratchpad
//! and promoted into CI 2026-07-16 (archived at
//! `references/review-artifacts-m0/pr7-review-demos/`). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! # What survived the M1 half-edge restructure
//!
//! The review was written against M0's placeholder adjacency (loops held
//! edge lists, edges held start/end vertices). M1 replaced that store
//! with the half-edge structure, so the demos are ported at the level of
//! the *invariants* they pinned, re-derived against today's API on an
//! independently hand-built minimal body (a lone edge: the mvfs+mev
//! shape, built through the raw builder + patching accessors, no Euler
//! ops). Ported: builder round-trip counts + birth provenance; the Q1
//! boundary as a consumer (generic `bbox<T: Real>`, a scalar-free spine
//! walk); external validator attacks (dangling topology/geometry keys,
//! orphan vertex, doubly-owned face, orphan geometry in sweep order);
//! determinism (identical builds ⇒ identical keys, identical
//! malformations ⇒ identical error lists); clone independence.
//!
//! Dropped as obsolete or duplicate:
//!
//! - the hand-built 6-face cube and its M0 entity shapes (superseded by
//!   `tests/cube_by_hand.rs` and `src/review_m1_pr1.rs`'s cube);
//! - the "empty loop validates silently at M0" observation (M1's typed
//!   [`topo::LoopBoundary::Empty`] made emptiness first-class and
//!   validated);
//! - the "orphan edge" scenario (edges are anchored through the
//!   edge↔half-edge bijection now, a different — validated — invariant);
//! - the cross-body foreign-key hazard (promoted already:
//!   `review_m1_pr1.rs::foreign_keys_resolve_arbitrarily_as_documented`).
//!
//! # Type-level probes (compile-fail, documentary)
//!
//! The review's probe crate proved cross-arena key confusion is
//! unrepresentable. Most of its cases are now shipped doctests
//! (`src/entity.rs`, `src/body.rs`); the two variants below are not, and
//! are kept in `compile_fail` doctest form to match that idiom (rustdoc
//! does not run integration-test doctests, so these are documentation of
//! the review's findings, not executed gates).
//!
//! A geometry key of the wrong kind is rejected too — a `PointKey` is
//! not a `CurveKey`:
//!
//! ```compile_fail,E0308
//! use topo::{Edge, HalfEdgeKey, PointKey};
//! let _ = Edge {
//!     he_plus: HalfEdgeKey::default(),
//!     he_minus: HalfEdgeKey::default(),
//!     curve: PointKey::default(),
//! };
//! ```
//!
//! And same-stratum containment cycles are unrepresentable — a shell
//! cannot contain shells:
//!
//! ```compile_fail,E0308
//! use topo::{Shell, ShellKey, SolidKey};
//! let _ = Shell {
//!     faces: vec![ShellKey::default()],
//!     solid: SolidKey::default(),
//! };
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::{
    Body, EdgeKey, EntityId, Face, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary, LoopKey,
    PointKey, Provenance, Shell, ShellKey, Solid, ValidationError, Vertex, validate,
};
use geom_core::Tol;
use geom_core::{Point3, Real};

fn prov() -> Provenance {
    Provenance::Primordial {
        op: "m0-pr7-review",
    }
}

/// Keys of the hand-built lone-edge body, kept for breakage scenarios.
struct LoneEdge {
    body: Body<f64>,
    p: [PointKey; 2],
    v: [crate::VertexKey; 2],
    he: [HalfEdgeKey; 2],
    edge: EdgeKey,
    lp: LoopKey,
    face: crate::FaceKey,
    shell: ShellKey,
    solid: crate::SolidKey,
    curve: crate::CurveKey,
    surface: crate::SurfaceKey,
}

/// Build the minimal cyclic body through the public raw builder +
/// patching accessors: one edge between two vertices, both half-edges in
/// the single face's outer loop (the state `mvfs; mev` reaches, derived
/// by hand here, not through the Euler ops). Geometry: v0 at the origin,
/// v1 at (1,0,0).
fn build_lone_edge(tol: Tol) -> LoneEdge {
    let mut body = Body::<f64>::new();

    let p0 = body.add_point(Point3::new(0.0, 0.0, 0.0));
    let p1 = body.add_point(Point3::new(1.0, 0.0, 0.0));
    let curve = body.add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
    let surface = body.add_surface(crate::fixtures::test_surface(Point3::origin()));

    let v0 = body.add_vertex(
        Vertex {
            point: p0,
            emanating: None,
        },
        prov(),
    );
    let v1 = body.add_vertex(
        Vertex {
            point: p1,
            emanating: None,
        },
        prov(),
    );

    // Half-edges first with placeholder links, patched below once every
    // key exists (the builder's documented raw-construction pattern).
    let he0 = body.add_half_edge(
        HalfEdge {
            edge: EdgeKey::default(),
            start: v0,
            parent_loop: LoopKey::default(),
            next: HalfEdgeKey::default(),
            prev: HalfEdgeKey::default(),
        },
        prov(),
    );
    let he1 = body.add_half_edge(
        HalfEdge {
            edge: EdgeKey::default(),
            start: v1,
            parent_loop: LoopKey::default(),
            next: HalfEdgeKey::default(),
            prev: HalfEdgeKey::default(),
        },
        prov(),
    );
    let edge = body.add_edge(
        crate::Edge {
            he_plus: he0,
            he_minus: he1,
            curve,
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
    let face = body.add_face(
        Face {
            sense: true,
            surface,
            outer: LoopKey::default(),
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_shell_mut(shell).unwrap().faces.push(face);
    let lp = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: he0 },
            face,
        },
        prov(),
    );
    body.get_face_mut(face).unwrap().outer = lp;

    // Patch the cycle: he0 <-> he1, both in lp, both claiming the edge.
    {
        let h = body.get_half_edge_mut(he0).unwrap();
        h.edge = edge;
        h.parent_loop = lp;
        h.next = he1;
        h.prev = he1;
    }
    {
        let h = body.get_half_edge_mut(he1).unwrap();
        h.edge = edge;
        h.parent_loop = lp;
        h.next = he0;
        h.prev = he0;
    }
    body.get_vertex_mut(v0).unwrap().emanating = Some(he0);
    body.get_vertex_mut(v1).unwrap().emanating = Some(he1);

    LoneEdge {
        body,
        p: [p0, p1],
        v: [v0, v1],
        he: [he0, he1],
        edge,
        lp,
        face,
        shell,
        solid,
        curve,
        surface,
    }
}

// ---------------------------------------------------------------------
// Builder round trip: counts, validation, birth provenance.
// ---------------------------------------------------------------------

#[test]
fn lone_edge_body_validates_with_expected_counts() {
    let tol = Tol::witness();
    let c = build_lone_edge(tol);
    assert_eq!(c.body.solids().count(), 1);
    assert_eq!(c.body.shells().count(), 1);
    assert_eq!(c.body.faces().count(), 1);
    assert_eq!(c.body.loops().count(), 1);
    assert_eq!(c.body.half_edges().count(), 2);
    assert_eq!(c.body.edges().count(), 1);
    assert_eq!(c.body.vertices().count(), 2);
    assert_eq!(c.body.points().count(), 2);
    assert_eq!(c.body.curves().count(), 1);
    assert_eq!(c.body.surfaces().count(), 1);
    assert_eq!(validate(&c.body), Ok(()));

    // Provenance present on every topology entity, recorded at birth.
    for id in [
        EntityId::Solid(c.solid),
        EntityId::Shell(c.shell),
        EntityId::Face(c.face),
        EntityId::Loop(c.lp),
        EntityId::HalfEdge(c.he[0]),
        EntityId::HalfEdge(c.he[1]),
        EntityId::Edge(c.edge),
        EntityId::Vertex(c.v[0]),
        EntityId::Vertex(c.v[1]),
    ] {
        assert_eq!(c.body.provenance(id), Some(&prov()), "{id:?}");
    }
}

// ---------------------------------------------------------------------
// The Q1 boundary as a consumer sees it.
// ---------------------------------------------------------------------

/// Generic over T: Real — geometric computation from vertex points
/// (bbox via componentwise min/max; pure value ops, no comparisons).
fn bbox<T: Real>(body: &Body<T>) -> Option<(Point3<T>, Point3<T>)> {
    let mut corners: Option<(Point3<T>, Point3<T>)> = None;
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point)?; // demo: treat dangling as absent
        corners = Some(match corners {
            None => (p, p),
            Some((lo, hi)) => (lo.min(p), hi.max(p)),
        });
    }
    corners
}

/// A scalar-free topology walk: counts entities reachable from each solid
/// down the containment spine, walking loop cycles by hand. Note the
/// signature: T: Real is still REQUIRED to name Body<T> at all, even
/// though no scalar is touched.
fn spine_census<T: Real>(body: &Body<T>) -> SpineCensus {
    let mut shells = 0;
    let mut faces = 0;
    let mut loops = 0;
    let mut half_edge_slots = 0;
    let bound = body.half_edges().count();
    for (_, solid) in body.solids() {
        for &sh in &solid.shells {
            let Some(shell) = body.get_shell(sh) else {
                continue;
            };
            shells += 1;
            for &f in &shell.faces {
                let Some(face) = body.get_face(f) else {
                    continue;
                };
                faces += 1;
                for &lp in core::iter::once(&face.outer).chain(face.rings.iter()) {
                    let Some(l) = body.get_loop(lp) else {
                        continue;
                    };
                    loops += 1;
                    if let LoopBoundary::Cycle { first } = l.boundary {
                        // Bounded manual walk (the consumer cannot assume
                        // coherence the validator has not certified).
                        let mut he = first;
                        for _ in 0..bound {
                            half_edge_slots += 1;
                            let Some(h) = body.get_half_edge(he) else {
                                break;
                            };
                            he = h.next;
                            if he == first {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    SpineCensus {
        shells,
        faces,
        loops,
        half_edge_slots,
    }
}

/// What a spine walk counts. `half_edge_slots` is not an arena length:
/// it counts the slots the loop walks visit, so a half-edge in two
/// loops (there are none in a coherent body) would count twice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SpineCensus {
    shells: usize,
    faces: usize,
    loops: usize,
    half_edge_slots: usize,
}

#[test]
fn generic_bbox_and_scalar_free_spine_census() {
    let tol = Tol::witness();
    let c = build_lone_edge(tol);
    let bb = bbox(&c.body).expect("lone edge has vertices");
    assert_eq!((bb.0.x, bb.0.y, bb.0.z), (0.0, 0.0, 0.0));
    assert_eq!((bb.1.x, bb.1.y, bb.1.z), (1.0, 0.0, 0.0));
    assert_eq!(
        spine_census(&c.body),
        SpineCensus {
            shells: 1,
            faces: 1,
            loops: 1,
            half_edge_slots: 2,
        }
    );
}

// ---------------------------------------------------------------------
// Breakage scenarios. Each starts from a fresh valid body. The public
// API has no removal, so external breakage is (a) malformed additions,
// (b) default (null) keys, and (c) mutation through the patching
// accessors.
// ---------------------------------------------------------------------

fn expect_errors(label: &str, body: &Body<f64>, expected: &[ValidationError]) {
    match validate(body) {
        Ok(()) => panic!("{label}: expected errors, got Ok"),
        Err(errors) => {
            assert_eq!(errors, expected, "{label}: error list mismatch");
        }
    }
}

/// A null key is the only dangling topology key mintable from outside
/// the crate without a second body.
#[test]
fn dangling_shell_key_is_caught() {
    let tol = Tol::witness();
    let mut c = build_lone_edge(tol);
    let s2 = c.body.add_solid(
        Solid {
            shells: vec![ShellKey::default()],
        },
        prov(),
    );
    expect_errors(
        "dangling shell key",
        &c.body,
        &[ValidationError::DanglingTopology {
            from: EntityId::Solid(s2),
            to: EntityId::Shell(ShellKey::default()),
        }],
    );
}

/// A dangling geometry key plus the orphanhood of its holder must arrive
/// in one validation pass, resolution errors before anchoring errors.
/// (The review pinned this on an edge with a dangling curve; edges are
/// anchored through the half-edge bijection now, so the port pins the
/// same one-pass/two-errors invariant on a vertex with a dangling point.)
#[test]
fn dangling_point_key_and_orphan_vertex_in_one_pass() {
    let tol = Tol::witness();
    let mut c = build_lone_edge(tol);
    let v = c.body.add_vertex(
        Vertex {
            point: PointKey::default(),
            emanating: None,
        },
        prov(),
    );
    expect_errors(
        "dangling point key + orphan vertex (one pass, two errors)",
        &c.body,
        &[
            ValidationError::DanglingGeometry {
                from: EntityId::Vertex(v),
                to: GeomRef::Point(PointKey::default()),
            },
            ValidationError::OrphanEntity {
                entity: EntityId::Vertex(v),
            },
        ],
    );
}

/// An orphan vertex (its point is referenced, hence NOT orphan geometry).
#[test]
fn orphan_vertex_is_caught() {
    let tol = Tol::witness();
    let mut c = build_lone_edge(tol);
    let p = c.body.add_point(Point3::new(9.0, 9.0, 9.0));
    let v = c.body.add_vertex(
        Vertex {
            point: p,
            emanating: None,
        },
        prov(),
    );
    expect_errors(
        "orphan vertex",
        &c.body,
        &[ValidationError::OrphanEntity {
            entity: EntityId::Vertex(v),
        }],
    );
}

/// A face listed by two shells (each shell itself coherently owned).
#[test]
fn doubly_owned_face_is_caught() {
    let tol = Tol::witness();
    let mut c = build_lone_edge(tol);
    let solid2 = c.body.add_solid(Solid { shells: vec![] }, prov());
    let shell2 = c.body.add_shell(
        Shell {
            faces: vec![c.face],
            solid: solid2,
        },
        prov(),
    );
    c.body.get_solid_mut(solid2).unwrap().shells.push(shell2);
    expect_errors(
        "doubly-owned face",
        &c.body,
        &[ValidationError::MultiplyOwned {
            child: EntityId::Face(c.face),
            owners: 2,
        }],
    );
}

/// Orphan geometry in all three arenas, reported in the geometry-sweep
/// order points → curves → surfaces.
#[test]
fn orphan_geometry_reported_in_sweep_order() {
    let tol = Tol::witness();
    let mut c = build_lone_edge(tol);
    let p = c.body.add_point(Point3::origin());
    let cv = c
        .body
        .add_curve(crate::fixtures::test_curve(Point3::origin(), tol));
    let s = c
        .body
        .add_surface(crate::fixtures::test_surface(Point3::origin()));
    expect_errors(
        "orphan geometry (point, curve, surface)",
        &c.body,
        &[
            ValidationError::OrphanGeometry {
                geometry: GeomRef::Point(p),
            },
            ValidationError::OrphanGeometry {
                geometry: GeomRef::Curve(cv),
            },
            ValidationError::OrphanGeometry {
                geometry: GeomRef::Surface(s),
            },
        ],
    );
}

// ---------------------------------------------------------------------
// Determinism and clone independence.
// ---------------------------------------------------------------------

/// Identical insertion sequences mint identical keys with identical
/// iteration order (== insertion order), and identical malformations
/// yield identical error lists — order included.
#[test]
fn identical_histories_are_bit_for_bit_deterministic() {
    let tol = Tol::witness();
    let a = build_lone_edge(tol);
    let b = build_lone_edge(tol);
    let a_vkeys: Vec<_> = a.body.vertices().map(|(k, _)| k).collect();
    let b_vkeys: Vec<_> = b.body.vertices().map(|(k, _)| k).collect();
    assert_eq!(a_vkeys, b_vkeys);
    assert_eq!(a_vkeys, a.v.to_vec(), "iteration = insertion order");
    let a_hkeys: Vec<_> = a.body.half_edges().map(|(k, _)| k).collect();
    let b_hkeys: Vec<_> = b.body.half_edges().map(|(k, _)| k).collect();
    assert_eq!(a_hkeys, b_hkeys);
    assert_eq!(a_hkeys, a.he.to_vec());

    // Identical malformations => identical error lists (order included).
    let broken = |mut c: LoneEdge| {
        c.body.add_point(Point3::origin()); // orphan geometry
        c.body.add_solid(
            Solid {
                shells: vec![ShellKey::default()], // dangling topology
            },
            prov(),
        );
        let p = c.body.add_point(Point3::origin());
        c.body.add_vertex(
            Vertex {
                point: p,
                emanating: None, // orphan vertex
            },
            prov(),
        );
        c.body
    };
    let e1 = validate(&broken(build_lone_edge(tol))).unwrap_err();
    let e2 = validate(&broken(build_lone_edge(tol))).unwrap_err();
    assert_eq!(e1, e2);
    assert_eq!(e1.len(), 3);
}

#[test]
fn clone_diverges_independently() {
    let tol = Tol::witness();
    let original = build_lone_edge(tol);
    let mut clone = original.body.clone();
    let extra_p = clone.add_point(Point3::new(5.0, 5.0, 5.0));
    let extra_v = clone.add_vertex(
        Vertex {
            point: extra_p,
            emanating: None,
        },
        prov(),
    );
    assert_eq!(original.body.vertices().count(), 2);
    assert_eq!(clone.vertices().count(), 3);
    assert_eq!(validate(&original.body), Ok(()));
    assert!(validate(&clone).is_err()); // orphan vertex in clone only

    // A key minted in the clone AFTER divergence does not resolve in the
    // original (the slot does not exist there).
    assert!(original.body.get_vertex(extra_v).is_none());

    // Unused keys kept for hand-derivation completeness.
    let _ = (original.p, original.curve, original.surface);
}
