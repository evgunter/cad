//! M2 PR 4 adversarial review suite (from `review/m2-4`, promoted
//! permanently as `review_m2_pr4`): falsification programs for the
//! extrude operation, run against the public API only.
//!
//! Naming convention: `survives_*` tests pass as-is and pin behavior
//! the review verified; `fixed_*` tests were the review's `finding_*`
//! pins of defective behavior, FLIPPED by the fix pass to assert the
//! corrected behavior (each keeps its finding lineage in its doc
//! comment); the remaining `finding_*` test pins a true-behavior scope
//! bound (the Dual derivative channel), not a defect. All tests are
//! ε-parameterized off the run's tolerance where the attacked band is
//! ε-relative.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_8, PI};
use profile::RawLoop;

use geom::Surface;
use geom_brep::{EdgeDescription, newell_plane};
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Real, Vec3};
use profile::{LoopRole, Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeError, Extruded, Extrusion, extrude};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, LoopBoundary, LoopKey, validate, validate_closed,
    validate_geometric,
};

fn eps() -> f64 {
    Tol::witness().get().eps
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

/// A two-vertex circle (two semicircular arcs), counterclockwise as
/// written.
fn circle_loop(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(cx - r, cy), 1.0),
        ProfileVertex::new(p2(cx + r, cy), 1.0),
    ])
}

fn assert_all_tiers(body: &Body<f64>) {
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    assert_eq!(validate_geometric(body, Tol::witness()), Ok(()));
}

/// (v, e, f, r) of a body.
fn counts(body: &Body<f64>) -> (usize, usize, usize, usize) {
    let rings: usize = body.faces().map(|(_, f)| f.rings.len()).sum();
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        rings,
    )
}

/// The vertex points of a loop's cycle in `next` order.
fn loop_points(body: &Body<f64>, r#loop: LoopKey) -> Vec<Point3<f64>> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).unwrap().boundary else {
        panic!("loop has no cycle");
    };
    body.loop_cycle(first)
        .unwrap()
        .iter()
        .map(|&he| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        })
        .collect()
}

/// Probe points of a loop in `next` order: each start vertex plus the
/// edge carrier's parameter midpoint (keeps 2-vertex loops
/// plane-determining and puts sample points on curved rims).
fn loop_probe_points(body: &Body<f64>, r#loop: LoopKey) -> Vec<Point3<f64>> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).unwrap().boundary else {
        panic!("loop has no cycle");
    };
    let mut pts = Vec::new();
    for he in body.loop_cycle(first).unwrap() {
        let he_data = body.get_half_edge(he).unwrap();
        pts.push(
            *body
                .get_point(body.get_vertex(he_data.start).unwrap().point)
                .unwrap(),
        );
        let ec = body
            .get_curve_geom(body.get_edge(he_data.edge).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = ec.params();
        pts.push(ec.carrier().eval((t0 + t1) * 0.5));
    }
    pts
}

/// The right-hand Newell normal of a face's outer loop in `next` order.
fn outward_normal(body: &Body<f64>, face: FaceKey) -> Vec3<f64> {
    let outer = body.get_face(face).unwrap().outer;
    let pts = loop_probe_points(body, outer);
    let Surface::Plane { normal, .. } =
        newell_plane(&pts, Band::linear(Tol::witness()).unwrap()).unwrap()
    else {
        panic!("newell returns a plane");
    };
    normal
}

/// The edge's stored description.
fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeDescription<f64> {
    let curve = body.get_edge(edge).unwrap().curve;
    body.get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .description()
        .clone()
}

/// Independent orientation oracle: signed volume by the ch. 13
/// divergence fan, `V = (1/6) Σ_loops Σ_i det[p₁, pᵢ, pᵢ₊₁]`, fanned
/// over every loop (outer + rings) of every face in `next` order, using
/// probe points (vertices + carrier midpoints). Exact for polyhedra
/// (planar faces; extra collinear probe points are harmless); a
/// chordal approximation on curved faces — the SIGN is the oracle.
fn signed_volume(body: &Body<f64>) -> f64 {
    let mut six_v = 0.0;
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
            let p1 = pts[0];
            for i in 1..pts.len() - 1 {
                let a = p1 - Point3::origin();
                let b = pts[i] - Point3::origin();
                let c = pts[i + 1] - Point3::origin();
                six_v += a.dot(b.cross(c));
            }
        }
    }
    six_v / 6.0
}

/// Cyclic-rotation equality of two point sequences (bitwise on
/// coordinates).
fn cyclic_eq(a: &[Point3<f64>], b: &[Point3<f64>]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let n = a.len();
    (0..n).any(|s| {
        (0..n).all(|i| {
            let (p, q) = (a[(s + i) % n], b[i]);
            p.x.to_bits() == q.x.to_bits()
                && p.y.to_bits() == q.y.to_bits()
                && p.z.to_bits() == q.z.to_bits()
        })
    })
}

/// The 6-vertex all-line L (counterclockwise; canonical start (0,0)).
fn l_loop() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ])
}

/// Byte-dump of everything geometric + structural the D9 claim covers.
fn dump(t: &Extruded<f64>) -> String {
    let mut s = String::new();
    for (k, p) in t.body.points() {
        s.push_str(&format!("{k:?} {p:?}\n"));
    }
    for (k, c) in t.body.curves() {
        match c.certified() {
            Some(c) => s.push_str(&format!(
                "{k:?} {:?} {:?} {:?}\n",
                c.description(),
                c.params(),
                c.certificate()
            )),
            None => s.push_str(&format!("{k:?} {c:?}\n")),
        }
    }
    for (k, srf) in t.body.surfaces() {
        s.push_str(&format!("{k:?} {srf:?}\n"));
    }
    for (k, f) in t.body.faces() {
        s.push_str(&format!("{k:?} {f:?}\n"));
    }
    for (k, l) in t.body.loops() {
        s.push_str(&format!("{k:?} {l:?}\n"));
    }
    for (k, h) in t.body.half_edges() {
        s.push_str(&format!("{k:?} {h:?}\n"));
    }
    s.push_str(&format!("{:?} {:?}\n", t.side_faces, t.strut_edges));
    s
}

// =====================================================================
// Assignment 1 — Euler-op boundary cases as real programs.
// =====================================================================

/// The digon (2-vertex all-arc) outer profile, BOTH directions: counts,
/// tiers, cap digon cycles, and the signed-volume oracle.
#[test]
fn survives_digon_outer_both_directions() {
    for d in [1.0, -1.0] {
        let t = extrude(
            &validated(vec![circle_loop(0.0, 0.0, 0.5)]),
            Extrusion::Distance(d),
            Tol::witness(),
        )
        .unwrap();
        assert_all_tiers(&t.body);
        assert_eq!(counts(&t.body), (4, 6, 4, 0));
        // Both caps are digon cycles (2 half-edges each).
        for cap in [t.top, t.bottom] {
            assert_eq!(
                loop_points(&t.body, t.body.get_face(cap).unwrap().outer).len(),
                2
            );
        }
        // Outward solid regardless of direction (chordal approximation;
        // the sign is the oracle).
        assert!(signed_volume(&t.body) > 0.0, "d = {d}");
        // Caps outward along ±z.
        let (top_z, bot_z) = (
            outward_normal(&t.body, t.top).z,
            outward_normal(&t.body, t.bottom).z,
        );
        if d > 0.0 {
            assert!(top_z > 0.99 && bot_z < -0.99);
        } else {
            assert!(top_z < -0.99 && bot_z > 0.99);
        }
    }
}

/// A 2-arc hole extruded BOTH ways: ring path counts, ring winding
/// against the cap, hand-traced cap cycles (top outer = raised
/// canonical chain; bottom outer = reversed canonical chain; rings
/// likewise), and the volume oracle.
#[test]
fn survives_two_arc_hole_hand_traced_cycles() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let vp = validated(vec![outer, circle_loop(0.5, 0.5, 0.1)]);
    let outer_canon: Vec<Point2<f64>> = vp.loops()[0].vertices().iter().map(|v| v.pos()).collect();
    let hole_canon: Vec<Point2<f64>> = vp.loops()[1].vertices().iter().map(|v| v.pos()).collect();
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);

    for d in [1.0f64, -0.5] {
        let t = extrude(&vp, Extrusion::Distance(d), Tol::witness()).unwrap();
        assert_all_tiers(&t.body);
        let (v, e, f, r) = counts(&t.body);
        assert_eq!((v, e, f, r), (12, 18, 8, 2));
        assert_eq!(v as isize - e as isize + f as isize - r as isize, 0); // g = 1

        // Swept traversal: canonical as-is for +n, reversed for −n.
        let swept = |canon: &[Point2<f64>], z: f64| -> Vec<Point3<f64>> {
            let n = canon.len();
            (0..n)
                .map(|j| {
                    let c = if d > 0.0 {
                        canon[j]
                    } else {
                        canon[(n - j) % n]
                    };
                    Point3::new(c.x, c.y, z)
                })
                .collect()
        };
        // Top cap (swept face): raised swept chains, outer and ring.
        let top_outer = loop_points(&t.body, t.body.get_face(t.top).unwrap().outer);
        assert!(cyclic_eq(&top_outer, &swept(&outer_canon, d)));
        let top_ring = loop_points(&t.body, t.body.get_face(t.top).unwrap().rings[0]);
        assert!(cyclic_eq(&top_ring, &swept(&hole_canon, d)));
        // Bottom cap: the mef-minted reversed chains on the sketch
        // plane. Reversed-of-swept = [v0, v_{n-1}, …, v1] of the swept
        // chain.
        let reversed = |chain: &[Point3<f64>]| -> Vec<Point3<f64>> {
            let n = chain.len();
            (0..n).map(|j| chain[(n - j) % n]).collect()
        };
        let bottom_outer = loop_points(&t.body, t.body.get_face(t.bottom).unwrap().outer);
        assert!(cyclic_eq(
            &bottom_outer,
            &reversed(&swept(&outer_canon, 0.0))
        ));
        let bottom_ring = loop_points(&t.body, t.body.get_face(t.bottom).unwrap().rings[0]);
        assert!(cyclic_eq(&bottom_ring, &reversed(&swept(&hole_canon, 0.0))));

        // Volume oracle: positive, ≈ (1 − π·0.01)·|d| within the
        // chordal error of the 2-arc hole polygonalization.
        let v_expect = (1.0 - PI * 0.01) * d.abs();
        let v_got = signed_volume(&t.body);
        assert!(v_got > 0.0);
        assert!(
            (v_got - v_expect).abs() < 0.05 * v_expect,
            "{v_got} vs {v_expect}"
        );
    }
}

/// A hole whose canonical start vertex sits close (but definitely
/// clear) to the outer loop's canonical start — the short-bridge /
/// canonical-start interaction of the hole-planting state.
#[test]
fn survives_hole_near_outer_canonical_start() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    // Hole lex-min vertex at (0.006, 0.011): bridge chord ~0.0125 m,
    // clearance to the outer edges 0.006 m — all definite at every CI ε.
    let hole = circle_loop(0.011, 0.011, 0.005);
    let t = extrude(
        &validated(vec![outer, hole]),
        Extrusion::Distance(0.3),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (12, 18, 8, 2));
    assert_eq!(v as isize - e as isize + f as isize - r as isize, 0);
    assert!(signed_volume(&t.body) > 0.0);
}

/// Two and three holes: genus h, the tier-1 component validator's own
/// E–P count agreeing with the arithmetic, adjacent holes near (but
/// definitely clear of) each other.
#[test]
fn survives_multiple_holes_genus_h() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(3.0, 0.0), p2(3.0, 1.0), p2(0.0, 1.0)]);
    // Two holes 0.05 apart edge-to-edge (definite at every CI ε).
    let holes2 = vec![circle_loop(1.0, 0.5, 0.2), circle_loop(1.45, 0.5, 0.2)];
    let t = extrude(
        &validated(
            core::iter::once(outer.clone())
                .chain(holes2)
                .collect::<Vec<_>>(),
        ),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    // Tiers passing means the tier-1/2 validators' own component E–P
    // (v − e + f − r = 2(1 − g), g ≥ 0 integer) accepted the count.
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (16, 24, 10, 4));
    assert_eq!(v as isize - e as isize + f as isize - r as isize, -2); // 2(1 − 2): g = 2
    assert!(signed_volume(&t.body) > 0.0);

    let holes3 = vec![
        circle_loop(0.5, 0.5, 0.15),
        circle_loop(1.5, 0.5, 0.15),
        circle_loop(2.5, 0.5, 0.15),
    ];
    let t3 = extrude(
        &validated(core::iter::once(outer).chain(holes3).collect::<Vec<_>>()),
        Extrusion::Distance(-0.7),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t3.body);
    let (v, e, f, r) = counts(&t3.body);
    assert_eq!((v, e, f, r), (20, 30, 12, 6));
    assert_eq!(v as isize - e as isize + f as isize - r as isize, -4); // 2(1 − 3): g = 3
    assert!(signed_volume(&t3.body) > 0.0);
}

/// Two holes within the sliver band of each other must die typed at
/// SOME layer (profile simplicity), never build silently.
#[test]
fn survives_sliver_gap_holes_die_typed() {
    let gap = 3.0 * eps();
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let holes = vec![circle_loop(0.3, 0.5, 0.1), circle_loop(0.5 + gap, 0.5, 0.1)];
    let res = Profile::new(
        SketchPlane::xy(),
        core::iter::once(outer).chain(holes).collect(),
    )
    .validate(Tol::witness());
    assert!(res.is_err(), "sliver-gap holes validated silently");
}

// =====================================================================
// Assignment 2 — orientation and the reversal bookkeeping.
// =====================================================================

/// An asymmetric arc-bearing profile extruded along +n and −n: outward
/// both ways (tiers, cap normals, EXACT fan volume on the all-planar
/// variant), and the canonical maps pinned on a marked vertex/segment:
/// swept strut j's `ExtrudedPoint.point` is canonical vertex
/// (n−j) mod n bitwise; the swept wall of canonical segment k is a
/// cylinder iff segment k is an arc, at swept index n−1−k.
#[test]
fn survives_reversal_maps_and_orientation() {
    // Asymmetric, one arc: (0,0) → (2,0) → arc → (2.5,0.5) → (2.5,1.5)
    // → (0,1). Canonical start (0,0), CCW as written, n = 5.
    let b = FRAC_PI_8.tan();
    // The quarter arc joins both neighbor lines tangentially (a
    // rounded step) -- declared per the #101 discipline.
    // Only (2,0) leaves on an arc; the two joints bracketing it are
    // the declared tangencies.
    let mut lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 0.0), b),
        ProfileVertex::new(p2(2.5, 0.5), 0.0),
        ProfileVertex::new(p2(2.5, 1.5), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    lp = lp.with_tangent_joints(vec![1, 2]);
    let vp = validated(vec![lp]);
    let canon: Vec<Point2<f64>> = vp.loops()[0].vertices().iter().map(|v| v.pos()).collect();
    let n = canon.len();
    assert_eq!(n, 5);
    assert_eq!(canon[0].x, 0.0);
    assert_eq!(canon[0].y, 0.0);
    // The arc is canonical segment 1 ((2,0) → (2.5,0.5)).
    assert!(matches!(
        vp.loops()[0].segments()[1].kind,
        profile::SegmentKind::Arc { .. }
    ));

    for d in [0.75, -0.75] {
        let t = extrude(&vp, Extrusion::Distance(d), Tol::witness()).unwrap();
        assert_all_tiers(&t.body);
        assert!(signed_volume(&t.body) > 0.0, "d = {d}");
        let (top_z, bot_z) = (
            outward_normal(&t.body, t.top).z,
            outward_normal(&t.body, t.bottom).z,
        );
        if d > 0.0 {
            assert!(top_z > 0.99 && bot_z < -0.99);
        } else {
            assert!(top_z < -0.99 && bot_z > 0.99);
        }

        // Marked-vertex map: strut j was minted from swept vertex j
        // (corner struts carry upgraded Intersection descriptions, so
        // probe the topology: `he_plus` runs bottom → raised, and its
        // start vertex is the swept vertex).
        for (j, &edge) in t.strut_edges[0].iter().enumerate() {
            let he_plus = t.body.get_edge(edge).unwrap().he_plus;
            let bottom_v = t.body.get_half_edge(he_plus).unwrap().start;
            let p = *t
                .body
                .get_point(t.body.get_vertex(bottom_v).unwrap().point)
                .unwrap();
            let expect = if d > 0.0 {
                canon[j]
            } else {
                canon[(n - j) % n]
            };
            assert_eq!(p.x.to_bits(), expect.x.to_bits(), "d {d} strut {j}");
            assert_eq!(p.y.to_bits(), expect.y.to_bits(), "d {d} strut {j}");
            assert_eq!(
                p.z, 0.0,
                "d {d} strut {j} bottom vertex on the sketch plane"
            );
        }
        // Marked-segment map: exactly one cylinder wall; at swept index
        // 1 forward, n−1−1 = 3 reversed.
        let cyl_at = if d > 0.0 { 1 } else { n - 1 - 1 };
        for (j, &fk) in t.side_faces[0].iter().enumerate() {
            let sk = t.body.get_face(fk).unwrap().surface;
            let is_cyl = matches!(t.body.get_surface(sk).unwrap(), Surface::Cylinder { .. });
            assert_eq!(is_cyl, j == cyl_at, "d {d} wall {j}");
        }
    }

    // Exact volume cross-check on the all-planar L (fan formula exact
    // for planar faces): area 3, both directions.
    let vp_l = validated(vec![l_loop()]);
    for d in [1.5, -1.5] {
        let t = extrude(&vp_l, Extrusion::Distance(d), Tol::witness()).unwrap();
        let v = signed_volume(&t.body);
        assert!((v - 3.0 * d.abs()).abs() < 1e-9, "d {d}: V = {v}");
    }
}

/// A typed error's canonical indices survive the reversal map: the
/// sliver corner sits at canonical vertex 1 and is reported there for
/// BOTH extrusion directions.
#[test]
fn survives_sliver_join_reports_canonical_index_both_directions() {
    let theta = 3000.0 * eps();
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0 + theta.cos(), theta.sin()),
        p2(0.0, 1.0),
    ]);
    let vp = validated(vec![lp]);
    // Canonical start is (0,0) (lex-min), CCW as written: the shallow
    // corner is canonical vertex 1.
    assert_eq!(vp.loops()[0].vertices()[1].pos().x, 1.0);
    for d in [1.0e-3, -1.0e-3] {
        let err = extrude(&vp, Extrusion::Distance(d), Tol::witness()).unwrap_err();
        match err {
            ExtrudeError::SliverJoin {
                loop_index,
                vertex_index,
                source,
            } => {
                assert_eq!(loop_index, 0);
                assert_eq!(vertex_index, 1, "canonical index broken for d = {d}");
                assert_eq!(source.predicate, Some("dihedral_wedge"));
            }
            other => panic!("expected SliverJoin, got {other:?}"),
        }
    }
}

// =====================================================================
// Assignment 3 — join classification across the sliver band.
// =====================================================================

/// A profile-definite corner swept across the extrude-scale band by the
/// lever arm |w| = 1e-3: dihedral margin sinθ·|w| at 0.5ε ⇒ definite
/// Smooth (conventional strut, distinct plane keys — never silently
/// shared); at 3ε ⇒ SliverJoin; at 30ε ⇒ definite Transverse
/// (Intersection). Never a wrong definite, and the in-band case is the
/// honest typed answer to "profile passed it, extrude cannot certify
/// it at ITS lever arm".
#[test]
fn survives_dihedral_band_sweep_at_the_strut_arm() {
    let w = 1.0e-3;
    let corner_profile = |theta: f64| {
        ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(1.0, 0.0),
            p2(1.0 + theta.cos(), theta.sin()),
            p2(0.0, 1.0),
        ])
    };
    // margin = sin θ · w. Choose θ so margins land at 0.5ε, 3ε, 30ε.
    let smooth_theta = 0.5 * eps() / w; // 500ε: profile-definite corner
    let sliver_theta = 3.0 * eps() / w;
    let corner_theta = 30.0 * eps() / w;

    // (a) Definite Smooth at the strut arm: builds, strut 1 stays
    // conventional, wall keys distinct (cosurface margin ≈ θ·1 m =
    // 500ε ≫ K·ε — definitely two planes, no silent sharing).
    let t = extrude(
        &validated(vec![corner_profile(smooth_theta)]),
        Extrusion::Distance(w),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert!(matches!(
        description(&t.body, t.strut_edges[0][1]),
        EdgeDescription::Scaffold(_)
    ));
    let k0 = t.body.get_face(t.side_faces[0][0]).unwrap().surface;
    let k1 = t.body.get_face(t.side_faces[0][1]).unwrap().surface;
    assert_ne!(k0, k1, "smooth-at-arm join must not silently share keys");

    // (b) In-band: the typed sliver, at the canonical vertex.
    let err = extrude(
        &validated(vec![corner_profile(sliver_theta)]),
        Extrusion::Distance(w),
        Tol::witness(),
    )
    .unwrap_err();
    assert!(
        matches!(
            err,
            ExtrudeError::SliverJoin {
                loop_index: 0,
                vertex_index: 1,
                ..
            }
        ),
        "{err:?}"
    );

    // (c) Definite Transverse: Intersection at strut 1.
    let t = extrude(
        &validated(vec![corner_profile(corner_theta)]),
        Extrusion::Distance(w),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert!(matches!(
        description(&t.body, t.strut_edges[0][1]),
        EdgeDescription::Intersection { .. }
    ));
}

// =====================================================================
// Assignments 3 + 4 — cosurface sharing and its honesty.
// =====================================================================

/// Exactly collinear adjacent lines share ONE plane key with a
/// conventional join strut.
#[test]
fn survives_collinear_lines_share_the_plane_key() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let t = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    let k0 = t.body.get_face(t.side_faces[0][0]).unwrap().surface;
    let k1 = t.body.get_face(t.side_faces[0][1]).unwrap().surface;
    assert_eq!(k0, k1, "collinear walls share one plane");
    // 2 caps + 4 distinct wall planes (5 segments, one shared pair).
    assert_eq!(t.body.surfaces().count(), 6);
    assert!(matches!(
        description(&t.body, t.strut_edges[0][1]),
        EdgeDescription::Scaffold(_)
    ));
    // The shared-key smooth join is skipped structurally; every true
    // corner upgraded.
    for j in [0usize, 2, 3, 4] {
        assert!(matches!(
            description(&t.body, t.strut_edges[0][j]),
            EdgeDescription::Intersection { .. }
        ));
    }
}

/// The notched circle: a same-carrier arc–arc pair meeting at the
/// canonical start vertex, shared via the WRAP join (prev join is
/// mixed-kind there): one cylinder key across the wrap, conventional
/// strut at vertex 0.
#[test]
fn survives_notched_circle_wrap_join_shares_the_key() {
    let q = FRAC_PI_8.tan(); // quarter-arc bulge
    // (−1,0) → arc(quarter) → (0,−1) → line → (1,0) → line → (0,1)
    // → arc(quarter) → close. Carrier: unit circle at the origin.
    // Canonical start (−1,0) is the join of LAST arc and FIRST arc.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), q),
        ProfileVertex::new(p2(0.0, -1.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), q),
    ]);
    let vp = validated(vec![lp]);
    assert_eq!(vp.loops()[0].vertices()[0].pos().x, -1.0);
    let t = extrude(&vp, Extrusion::Distance(0.5), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // Walls: [arc, line, line, arc]; the wrap join (segment 3 → 0)
    // shares faces[0]'s cylinder.
    let k0 = t.body.get_face(t.side_faces[0][0]).unwrap().surface;
    let k3 = t.body.get_face(t.side_faces[0][3]).unwrap().surface;
    assert_eq!(k0, k3, "wrap-join same-carrier arcs share one cylinder");
    // 2 caps + 1 cylinder + 2 planes.
    assert_eq!(t.body.surfaces().count(), 5);
    // Strut 0 (the wrap join) stays conventional; the line corners
    // upgrade.
    assert!(matches!(
        description(&t.body, t.strut_edges[0][0]),
        EdgeDescription::Scaffold(_)
    ));
    for j in [1usize, 2, 3] {
        assert!(matches!(
            description(&t.body, t.strut_edges[0][j]),
            EdgeDescription::Intersection { .. }
        ));
    }
    assert!(signed_volume(&t.body) > 0.0);
}

/// FIXED (was `finding_wrap_cosurface_run_split_into_two_keys`,
/// SHOULD-1): prev-join precedence used to SHORT-CIRCUIT the wrap
/// join, so a same-carrier run of ≥ 3 arcs crossing the canonical
/// start vertex was split into TWO surface keys for ONE
/// identical-by-construction cylinder — falsifying the crate-doc claim
/// that smooth joins on the identical-by-construction surface "share
/// the surface key" (and M2-PLAN PR 6's "the surface KEY is shared
/// when identical-by-construction").
///
/// Profile: unit circle cut by one chord, arcs split so the carrier
/// run is [seg 2, seg 3, seg 0] across the wrap. The fix: all of a
/// loop's consecutive-pair cosurface decisions (including the wrap
/// pair) are made BEFORE any wall is minted, so a segment whose
/// forward chain reaches segment 0 through the wrap shares `faces[0]`'s
/// key at mint time — no re-keying, no reconciliation, and the run's
/// `u_ref` stays with its first segment in sweep order (segment 0).
/// The whole run now resolves to ONE cylinder key.
#[test]
fn fixed_wrap_cosurface_run_shares_one_key() {
    let sixth = (PI / 12.0).tan(); // 60° arc bulge = tan(60°/4)
    let quarter = FRAC_PI_8.tan();
    let (c60, s60) = (0.5, 3.0f64.sqrt() / 2.0);
    // (−1,0) → arc 180°→270° (quarter) → chord to 60° → arc 60°→120°
    // → arc 120°→180° (closing). Segments: [arc, line, arc, arc]; the
    // same-carrier run {2, 3, 0} crosses the canonical start (−1,0).
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), quarter),
        ProfileVertex::new(p2(0.0, -1.0), 0.0),
        ProfileVertex::new(p2(c60, s60), sixth),
        ProfileVertex::new(p2(-c60, s60), sixth),
    ]);
    let vp = validated(vec![lp]);
    assert_eq!(vp.loops()[0].vertices()[0].pos().x, -1.0);
    let t = extrude(&vp, Extrusion::Distance(0.5), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let key = |j: usize| t.body.get_face(t.side_faces[0][j]).unwrap().surface;
    // The whole wrap-crossing run {2, 3, 0} shares ONE key…
    assert_eq!(key(2), key(3));
    assert_eq!(key(0), key(2), "the wrap-crossing run must share one key");
    // …which is one cylinder (u_ref from segment 0, the run's first
    // segment in sweep order: it points at the canonical start vertex).
    let Surface::Cylinder { origin, radius, .. } = *t.body.get_surface(key(0)).unwrap() else {
        panic!("arc walls are cylinders");
    };
    assert!(origin.x.abs() < 1e-12 && origin.y.abs() < 1e-12);
    assert!((radius - 1.0).abs() < 1e-12);
    // 2 caps + 1 plane + ONE cylinder key for the run's carrier.
    assert_eq!(t.body.surfaces().count(), 4);
    // The in-run struts (walls 3|0 at vertex 0, walls 2|3 at vertex 3)
    // are same-key smooth joins and stay conventional; the chord's two
    // corners upgrade.
    assert!(matches!(
        description(&t.body, t.strut_edges[0][0]),
        EdgeDescription::Scaffold(_)
    ));
    assert!(matches!(
        description(&t.body, t.strut_edges[0][3]),
        EdgeDescription::Scaffold(_)
    ));
    for j in [1usize, 2] {
        assert!(matches!(
            description(&t.body, t.strut_edges[0][j]),
            EdgeDescription::Intersection { .. }
        ));
    }
}

/// NEAR-cosurface honesty: geometry inside the band is refused typed at
/// SOME layer — profile validation escalates before extrude's cosurface
/// predicate can even see it (the margins are the same displacement) —
/// and is never silently shared or silently distinct.
///
/// Documents: `ExtrudeError::CosurfaceEscalated` is defense-in-depth —
/// unreachable through f64 profiles that validate (any in-band carrier
/// near-identity already escalated at profile simplicity/containment).
#[test]
fn survives_near_cosurface_dies_typed_at_the_profile_gate() {
    // (a) Lines: next chord's far endpoint 3ε off the prev carrier.
    let d = 3.0 * eps();
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(2.0, d), p2(1.0, 2.0)]);
    let profile_result = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness());
    match profile_result {
        Err(_) => {} // typed at the profile gate — honest
        Ok(vp) => {
            // If validation ever lets it through, extrude must refuse
            // typed (escalated cosurface or sliver join) — NEVER build.
            let res = extrude(&vp, Extrusion::Distance(1.0), Tol::witness());
            assert!(
                matches!(
                    res,
                    Err(ExtrudeError::CosurfaceEscalated { .. })
                        | Err(ExtrudeError::SliverJoin { .. })
                ),
                "in-band near-collinear geometry must die typed, got {res:?}"
            );
        }
    }

    // (b) Arcs: internally tangent carriers with center/radius offset
    // 2ε (cosurface margin d + |Δr| ≈ 4ε ∈ (ε, 10ε)).
    let delta = 2.0 * eps();
    let r1 = 0.5;
    let r2 = r1 - delta;
    // Both arcs tangent to the x-axis at their shared vertex (0,0):
    // centers (0, r1), (0, r2). Arc A comes in along circle 1 from
    // (−r1, r1); arc B leaves along circle 2 to (r2, r2). Close with
    // lines well away from the band.
    let b1 = (PI / 8.0).tan(); // quarter arcs
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(-r1, r1), b1),
        ProfileVertex::new(p2(0.0, 0.0), b1),
        ProfileVertex::new(p2(r2, r2), 0.0),
        ProfileVertex::new(p2(r2, 1.2), 0.0),
        ProfileVertex::new(p2(-r1, 1.2), 0.0),
    ]);
    let profile_result = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness());
    match profile_result {
        Err(_) => {} // typed at the profile gate — honest
        Ok(vp) => {
            let res = extrude(&vp, Extrusion::Distance(1.0), Tol::witness());
            assert!(
                matches!(
                    res,
                    Err(ExtrudeError::CosurfaceEscalated { .. })
                        | Err(ExtrudeError::SliverJoin { .. })
                ),
                "in-band near-cosurface arcs must die typed, got {res:?}"
            );
        }
    }
}

/// Exactly tangent same-carrier arcs (the disc) share one cylinder;
/// the bottom rim circle carriers and the shared cylinder agree on the
/// center BITWISE (the twice-computed c_world determinism claim), and
/// the rounded square keeps every line–arc tangency on distinct keys.
#[test]
fn survives_cosurface_bitwise_center_agreement() {
    let t = extrude(
        &validated(vec![circle_loop(0.25, -0.375, 0.5)]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    let k = t.body.get_face(t.side_faces[0][0]).unwrap().surface;
    assert_eq!(k, t.body.get_face(t.side_faces[0][1]).unwrap().surface);
    let Surface::Cylinder { origin, .. } = *t.body.get_surface(k).unwrap() else {
        panic!("cylinder");
    };
    // Every circle rim carrier's center is either bitwise the cylinder
    // origin (bottom rims — the identical expression) or bitwise
    // origin + w (top rims — translated placement; if this ever drifts
    // by association it is at most ulps and tier 3 catches worse, but
    // pin today's exact behavior).
    let mut bottom_rims = 0;
    let mut top_rims = 0;
    for (_, c) in t.body.curves() {
        if let Some(geom::Curve3::Circle { center, .. }) =
            c.certified().map(|c| c.carrier().clone())
        {
            if center.z.to_bits() == origin.z.to_bits() {
                assert_eq!(center.x.to_bits(), origin.x.to_bits());
                assert_eq!(center.y.to_bits(), origin.y.to_bits());
                bottom_rims += 1;
            } else {
                assert_eq!(center.z, origin.z + 1.0);
                top_rims += 1;
            }
        }
    }
    assert_eq!((bottom_rims, top_rims), (2, 2));
}

// =====================================================================
// Assignment 5 — cap Newell with apex augmentation.
// =====================================================================

/// A cap with MIXED convex/concave (CCW and CW) arc segments on the
/// outer loop still certifies, with the correct outward normals and a
/// positive signed volume; the concave arc's wall is a cylinder whose
/// axis is the turn-signed normal (−z for the clockwise segment when
/// extruding +z).
#[test]
fn survives_mixed_turn_arcs_cap_certifies() {
    let b = FRAC_PI_8.tan();
    // Square with a convex arc bottom and a CONCAVE arc top (bulge
    // −b: clockwise turn, bowing into the region).
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), b),
        ProfileVertex::new(p2(2.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.5), -b),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let t = extrude(&vp, Extrusion::Distance(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert!(outward_normal(&t.body, t.top).z > 0.99);
    assert!(outward_normal(&t.body, t.bottom).z < -0.99);
    assert!(signed_volume(&t.body) > 0.0);
    // Turn-signed cylinder axes: find the two cylinders and check the
    // axis sign against the canonical segment turns.
    let mut axes = Vec::new();
    for (j, &fk) in t.side_faces[0].iter().enumerate() {
        let sk = t.body.get_face(fk).unwrap().surface;
        if let Surface::Cylinder { axis, .. } = *t.body.get_surface(sk).unwrap() {
            let profile::SegmentKind::Arc { turn, .. } = vp.loops()[0].segments()[j].kind else {
                panic!("wall {j} is a cylinder but segment {j} is a line");
            };
            axes.push((turn, axis.z));
        }
    }
    assert_eq!(axes.len(), 2);
    for (turn, z) in axes {
        match turn {
            geom_core::Sign::Positive => assert!(z > 0.99),
            geom_core::Sign::Negative => assert!(z < -0.99),
            geom_core::Sign::Zero => panic!("arc with zero turn"),
        }
    }
}

/// The digon cap NEEDS the apexes to determine a plane (2 vertices
/// alone cannot); the stored cap planes exist, are z-planes at the
/// right heights, and the apexes lie exactly on them (sagitta closed
/// form maps z = 0 exactly).
#[test]
fn survives_digon_caps_apex_determined() {
    let r = 0.5;
    let h = 0.75;
    let t = extrude(
        &validated(vec![circle_loop(0.0, 0.0, r)]),
        Extrusion::Distance(h),
        Tol::witness(),
    )
    .unwrap();
    let cap_plane = |face| {
        let sk = t.body.get_face(face).unwrap().surface;
        let Surface::Plane { origin, normal, .. } = *t.body.get_surface(sk).unwrap() else {
            panic!("cap is a plane");
        };
        (origin, normal)
    };
    let (bo, bn) = cap_plane(t.bottom);
    let (to, tn) = cap_plane(t.top);
    assert!(bn.z < -0.99 && bo.z.abs() < 1e-15);
    assert!(tn.z > 0.99 && (to.z - h).abs() < 1e-15);
    // Apex sanity: the sagitta apexes of the two semicircles are (0,∓r)
    // — on the carrier and exactly in the cap planes (z exact by
    // construction). Verified through the Newell result being exact
    // above; here pin the closed form itself via the rim midpoint.
    let outer = t.body.get_face(t.bottom).unwrap().outer;
    for p in loop_probe_points(&t.body, outer) {
        assert_eq!(p.z, 0.0, "bottom-cap probe points lie exactly on z = 0");
        assert!((p.x.hypot(p.y) - r).abs() < 1e-12, "on the carrier");
    }
}

/// Far-from-origin profiles (1e8 m offsets): the all-line L must build
/// at every ε row (translate-to-origin Newell); the arc profile is
/// allowed to either build tier-valid or refuse TYPED (certification
/// residuals at 1e8 exceed ε = 1e-9/1e-12 honestly) — never panic,
/// never a silently invalid body.
#[test]
fn survives_far_offset_profiles_honest() {
    // Lines only: exact dyadic data at 1e8 — must succeed at every ε.
    let off = 1.0e8;
    let lp = ProfileLoop::polygon([
        p2(off, off),
        p2(off + 2.0, off),
        p2(off + 2.0, off + 1.0),
        p2(off, off + 1.0),
    ]);
    let t = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.5),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert!(signed_volume(&t.body) > 0.0);

    // Arc carrier at 1e8: honesty check (typed either way).
    let lp = circle_loop(off, off, 0.5);
    match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Err(_) => {} // typed refusal at the profile gate is honest
        Ok(vp) => match extrude(&vp, Extrusion::Distance(1.0), Tol::witness()) {
            Ok(t) => assert_all_tiers(&t.body),
            Err(e) => assert!(
                matches!(
                    e,
                    ExtrudeError::Op {
                        source: EulerOpError::Certification { .. }
                    } | ExtrudeError::CapPlane { .. }
                        | ExtrudeError::SliverJoin { .. }
                        | ExtrudeError::CosurfaceEscalated { .. }
                ),
                "far-offset arc refusal must be a typed geometry error, got {e:?}"
            ),
        },
    }
}

// =====================================================================
// Assignment 8 — error-path honesty.
// =====================================================================

/// The sub-ε oblique component: a vector whose in-plane part is 0.5ε
/// classifies coincident-with-zero and must be used AS GIVEN — no
/// hidden snapping. The stored `ExtrudedPoint.vec` is the input vector
/// bitwise and the raised vertices are measurably sheared by it.
#[test]
fn survives_sub_eps_oblique_vector_used_as_given() {
    let dx = 0.5 * eps();
    let v = Vec3::new(dx, 0.0, 1.0);
    // Use the collinear-bottom profile: its collinear join keeps strut 1
    // as a conventional ExtrudedPoint (corner struts get re-described as
    // Intersection, discarding the vec payload), so the stored vector is
    // observable.
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let t = extrude(&validated(vec![lp]), Extrusion::Vector(v), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // Stored vector bitwise = input.
    let EdgeDescription::Scaffold(geom_brep::MappedCurve::ExtrudedPoint { vec, .. }) =
        description(&t.body, t.strut_edges[0][1])
    else {
        panic!("strut description");
    };
    assert_eq!(vec.x.to_bits(), v.x.to_bits());
    assert_eq!(vec.y.to_bits(), v.y.to_bits());
    assert_eq!(vec.z.to_bits(), v.z.to_bits());
    // The raised canonical start vertex (0,0) lands at (dx, 0, 1)
    // exactly: the shear is measurable in the built body, not snapped.
    let top_outer = loop_points(&t.body, t.body.get_face(t.top).unwrap().outer);
    assert!(
        top_outer
            .iter()
            .any(|p| p.x.to_bits() == dx.to_bits() && p.y == 0.0 && p.z == 1.0),
        "raised start vertex must be sheared by exactly the sub-ε component; got {top_outer:?}"
    );
}

/// Extrusion-vector classification: zero, sliver (both signs), oblique,
/// poisoned — all typed, never a panic; and a NaN PLACEMENT surfaces as
/// the operator-layer certification report (the Op variant is reachable
/// through the public API).
#[test]
fn survives_error_paths_extended() {
    let vp = validated(vec![l_loop()]);
    // Sliver distances, both signs.
    for d in [3.0 * eps(), -3.0 * eps()] {
        let err = extrude(&vp, Extrusion::Distance(d), Tol::witness()).unwrap_err();
        assert!(
            matches!(err, ExtrudeError::ExtrusionEscalated { ref source }
                if source.predicate == Some("extrusion_normal_component")),
            "{err:?}"
        );
    }
    // Coincident-with-zero distance from below.
    assert_eq!(
        extrude(&vp, Extrusion::Distance(0.5 * eps()), Tol::witness()).unwrap_err(),
        ExtrudeError::DegenerateExtrusion
    );
    // Poisoned distance.
    assert!(matches!(
        extrude(&vp, Extrusion::Distance(f64::NAN), Tol::witness()).unwrap_err(),
        ExtrudeError::ExtrusionEscalated { .. }
    ));
    // Oblique with a definite in-plane part riding a definite normal.
    assert_eq!(
        extrude(
            &vp,
            Extrusion::Vector(Vec3::new(0.0, 2e-3, 1.0)),
            Tol::witness()
        )
        .unwrap_err(),
        ExtrudeError::ObliqueExtrusion
    );
    // NaN placement: 2-D validation cannot see it; the geometry gate
    // refuses at the first certification, typed, body discarded.
    let bad_plane = SketchPlane::from_frame(
        Point3::new(f64::NAN, 0.0, 0.0),
        Vec3::unit_x(),
        Vec3::unit_y(),
    );
    let vp_bad = Profile::new(bad_plane, vec![l_loop()])
        .validate(Tol::witness())
        .expect("validation is 2-D; the poisoned placement passes through");
    let err = extrude(&vp_bad, Extrusion::Distance(1.0), Tol::witness()).unwrap_err();
    assert!(
        matches!(
            err,
            ExtrudeError::Op {
                source: EulerOpError::Certification { .. }
            }
        ),
        "poisoned placement must surface the certification report, got {err:?}"
    );
}

// =====================================================================
// Assignment 7 — determinism (D9).
// =====================================================================

/// Byte-identical rebuilds across the review's whole shape zoo,
/// including reversed and holed builds (beyond the acceptance test's
/// single case).
#[test]
fn survives_rebuild_byte_identity_zoo() {
    let b = FRAC_PI_8.tan();
    let shapes: Vec<(ProfileLoop<f64>, Vec<ProfileLoop<f64>>, f64)> = vec![
        (l_loop(), vec![], -1.5),
        (
            ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]),
            vec![circle_loop(0.5, 0.5, 0.1)],
            -1.0,
        ),
        (
            ProfileLoop::polygon([p2(0.0, 0.0), p2(3.0, 0.0), p2(3.0, 1.0), p2(0.0, 1.0)]),
            vec![circle_loop(1.0, 0.5, 0.2), circle_loop(2.0, 0.5, 0.2)],
            0.25,
        ),
        (circle_loop(0.0, 0.0, 0.5), vec![], -2.0),
        (
            <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
                ProfileVertex::new(p2(-1.0, 0.0), b),
                ProfileVertex::new(p2(0.0, -1.0), 0.0),
                ProfileVertex::new(p2(1.0, 0.0), 0.0),
                ProfileVertex::new(p2(0.0, 1.0), b),
            ]),
            vec![],
            0.5,
        ),
    ];
    for (i, (outer, holes, d)) in shapes.iter().enumerate() {
        let build = || {
            let loops: Vec<ProfileLoop<f64>> = core::iter::once(outer.clone())
                .chain(holes.clone())
                .collect();
            extrude(&validated(loops), Extrusion::Distance(*d), Tol::witness()).unwrap()
        };
        let a = build();
        let b2 = build();
        assert_eq!(dump(&a), dump(&b2), "shape {i} not byte-identical");
    }
}

/// Writes the canonical dump of a fixed holed+reversed build into
/// CARGO_TARGET_TMPDIR, keyed by build profile — the debug-vs-release
/// certificate-identity check is `diff`ing the two files after running
/// this test under both profiles (done in the review; kept as the
/// reproducible harness).
#[test]
fn dump_for_cross_profile_diff() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let t = extrude(
        &validated(vec![outer, circle_loop(0.5, 0.5, 0.1)]),
        Extrusion::Distance(-1.0),
        Tol::witness(),
    )
    .unwrap();
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    // CARGO_TARGET_TMPDIR is baked at compile time; the directory itself
    // is NOT guaranteed to exist on the test runner (nextest archives
    // don't carry the empty dir) — create it, the repo-wide idiom.
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join(format!("review-m2-pr4-dump-{profile}.txt"));
    std::fs::write(&path, dump(&t)).unwrap();
}

// =====================================================================
// Assignment 6 — the powi(2) bit-identity claims, derived and pinned.
// Routed to the PR 3 fix pass (which owns the norm_squared tight-square
// patch after the B1 coordination revert); these pin the argument's
// true extent.
// =====================================================================

/// f64: `Real::powi(x, 2)` is bit-identical to `x * x` — including
/// subnormals, ±0, ±∞, and huge values (`powi_by_squaring` reduces
/// n = 2 to `1.0 * (x*x)`, and multiplying by 1.0 is exact).
#[test]
fn survives_powi2_bitwise_equals_mul_at_f64() {
    let cases = [
        0.0,
        -0.0,
        1.5,
        -2.7,
        1.0e-160,
        -1.0e-160,
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE / 4.0, // subnormal
        5.0e-324,                // min subnormal
        1.0e160,
        f64::MAX,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    for x in cases {
        assert_eq!(Real::powi(x, 2).to_bits(), (x * x).to_bits(), "x = {x:e}");
    }
    assert!(Real::powi(f64::NAN, 2).is_nan());
}

/// FINDING (for the PR 3 fix pass writeup, severity NIT/doc): the Dual
/// DERIVATIVE channel of `powi(2)` — `(2·x)·x′` — is NOT bit-identical
/// to the product rule `x′·x + x·x′` everywhere: rounding commutes
/// with the exact doubling only in the normal range. Concrete
/// witnesses: (a) subnormal products (the doubling happens before vs
/// after the subnormal rounding), (b) `2·x` overflow with a tiny x′
/// (the powi rule manufactures ∞ where the product rule is finite).
/// The value channels ARE bit-identical (pinned above), and tangents
/// never decide (D8), so this bounds the claim's honest scope rather
/// than breaking a decision path — the fix-pass doc should scope any
/// bit-identity language to the value channel.
#[test]
fn finding_dual_powi2_derivative_channel_not_bitwise_mul() {
    use geom_core::Dual;
    // (a) Subnormal: x = 1.5·2⁻⁵³⁷, x′ = 2⁻⁵³⁷ ⇒ x·x′ = 1.5·2⁻¹⁰⁷⁴.
    // Product rule: fl(1.5·2⁻¹⁰⁷⁴) + fl(1.5·2⁻¹⁰⁷⁴) = 2·2⁻¹⁰⁷⁴ + …
    // = 4·2⁻¹⁰⁷⁴; powi rule: (2x)·x′ = 3·2⁻¹⁰⁷⁴ exactly.
    let x = 1.5 * 2.0f64.powi(-537);
    let dx = 2.0f64.powi(-537);
    let d = Dual::new(x, dx);
    let via_powi = Real::powi(d, 2);
    let via_mul = d * d;
    assert_eq!(via_powi.value.to_bits(), via_mul.value.to_bits());
    assert_eq!(via_powi.deriv, 3.0 * 5.0e-324);
    assert_eq!(via_mul.deriv, 4.0 * 5.0e-324);
    assert_ne!(
        via_powi.deriv.to_bits(),
        via_mul.deriv.to_bits(),
        "if this fails the derivative channels became bit-identical — \
         re-scope the fix-pass writeup"
    );
    // (b) Doubling overflow: x = 1e308, x′ = 1e-300.
    let d = Dual::new(1.0e308, 1.0e-300);
    let via_powi = Real::powi(d, 2);
    let via_mul = d * d;
    assert_eq!(via_powi.value.to_bits(), via_mul.value.to_bits());
    assert!(via_powi.deriv.is_infinite(), "2x overflows inside the rule");
    assert_eq!(via_mul.deriv, 2.0e8);
}
