//! M2 PR 4 acceptance: extrusion end-to-end from profile data through
//! public ops only — the L-profile, the holed profile (ring path), arc
//! profiles (cylinder side patches, both smooth-join classes), both
//! extrusion directions with orientation verified, the error paths, and
//! D9 determinism. All tests are ε-parameterized (they read the run's
//! tolerance); the Interval lane lives in `extrude_interval.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_8;
use profile::RawLoop;

use geom::Surface;
use geom_brep::{EdgeGeometry, newell_plane};
use geom_core::{Band, Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeError, Extruded, Extrusion, extrude};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, LoopBoundary, validate, validate_closed,
    validate_geometric,
};
use geom_core::Tol;

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

/// The 6-vertex all-line L (counterclockwise).
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

/// A two-vertex circle (two semicircular arcs) centered at `(cx, cy)`
/// with radius `r`, counterclockwise as written.
fn circle_loop(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(cx - r, cy), 1.0),
        ProfileVertex::new(p2(cx + r, cy), 1.0),
    ])
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

/// Asserts tiers 1–3 all pass.
fn assert_all_tiers(body: &Body<f64>) {
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    assert_eq!(validate_geometric(body, Tol::witness()), Ok(()));
}

/// The vertex points of a loop's cycle in `next` order.
fn loop_points(body: &Body<f64>, r#loop: topo::LoopKey) -> Vec<Point3<f64>> {
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

/// Plane/winding probe points of a loop in `next` order: each start
/// vertex plus its edge carrier's parameter midpoint, so loops with
/// fewer than three vertices (arc digons) still determine a plane.
fn loop_probe_points(body: &Body<f64>, r#loop: topo::LoopKey) -> Vec<Point3<f64>> {
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
            .unwrap();
        let ec = ec.certified().unwrap();
        let (t0, t1) = ec.params();
        pts.push(ec.carrier().eval((t0 + t1) * 0.5));
    }
    pts
}

/// The right-hand Newell normal of a face's outer loop in `next` order
/// — the outward normal under the interior-left convention.
fn outward_normal(body: &Body<f64>, face: FaceKey) -> Vec3<f64> {
    let outer = body.get_face(face).unwrap().outer;
    let pts = loop_probe_points(body, outer);
    let Surface::Plane { normal, .. } = newell_plane(&pts, Band::linear(Tol::witness()).unwrap()).unwrap() else {
        panic!("newell returns a plane");
    };
    normal
}

/// The edge's stored description.
fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeGeometry<f64> {
    let curve = body.get_edge(edge).unwrap().curve;
    *body
        .get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .description()
}

#[test]
fn extruded_l_profile_passes_all_tiers() {
    // (a) The L-prism: 8 faces, genus 0, every join a corner.
    let t = extrude(&validated(vec![l_loop()]), Extrusion::Distance(1.5), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (12, 18, 8, 0));
    // Component Euler–Poincaré at rest: v − e + f − r = 2(1 − g), g = 0.
    assert_eq!(v as isize - e as isize + f as isize - r as isize, 2);
    // All-plane surfaces: 2 caps + 6 sides.
    assert_eq!(t.body.surfaces().count(), 8);
    assert!(
        t.body
            .surfaces()
            .all(|(_, s)| matches!(s, Surface::Plane { .. }))
    );
    // Every profile join is a corner: all six struts carry the upgraded
    // Intersection description (prefer-intrinsic, D2).
    assert_eq!(t.strut_edges[0].len(), 6);
    for &edge in &t.strut_edges[0] {
        assert!(matches!(
            description(&t.body, edge),
            EdgeGeometry::Intersection { .. }
        ));
    }
    // Rim edges upgrade too (the ratified rim decision, M2 PR 4 fix
    // pass): every edge of the L-prism is definitely transverse, so all
    // 18 (6 struts + 12 rims) carry Intersection at rest.
    let intersections = t
        .body
        .curves()
        .filter(|(_, c)| {
            matches!(
                c.certified().map(topo::EdgeCurve::description),
                Some(EdgeGeometry::Intersection { .. })
            )
        })
        .count();
    assert_eq!(intersections, 18);
    // Orientation: top cap outward along +z, bottom along −z.
    assert!(outward_normal(&t.body, t.top).z > 0.99);
    assert!(outward_normal(&t.body, t.bottom).z < -0.99);
}

#[test]
fn extruded_profile_with_hole_builds_the_ring_path() {
    // (b) Square outer + circular hole (two-vertex bulge loop): rings
    // in both caps, a two-face cylindrical hole wall sharing one
    // carrier surface, genus 1 (the boundary of a solid with a through
    // hole is a torus — v − e + f − r = 0 = 2(1 − g)).
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let hole = circle_loop(0.5, 0.5, 0.1);
    let t = extrude(&validated(vec![outer, hole]), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (12, 18, 8, 2));
    assert_eq!(v as isize - e as isize + f as isize - r as isize, 0); // g = 1
    // Both caps carry exactly one ring.
    assert_eq!(t.body.get_face(t.top).unwrap().rings.len(), 1);
    assert_eq!(t.body.get_face(t.bottom).unwrap().rings.len(), 1);
    // The hole wall is TWO faces (the carrier is split at the profile's
    // seam vertices) on ONE shared cylinder surface.
    assert_eq!(t.side_faces[1].len(), 2);
    let k0 = t.body.get_face(t.side_faces[1][0]).unwrap().surface;
    let k1 = t.body.get_face(t.side_faces[1][1]).unwrap().surface;
    assert_eq!(k0, k1, "same-carrier hole walls share one surface key");
    assert!(matches!(
        t.body.get_surface(k0).unwrap(),
        Surface::Cylinder { .. }
    ));
    // The hole's joins are same-carrier smooth: struts stay MappedCurve.
    for &edge in &t.strut_edges[1] {
        assert!(matches!(
            description(&t.body, edge),
            EdgeGeometry::MappedCurve(_)
        ));
    }
    // The outer square's corners upgrade to Intersection.
    for &edge in &t.strut_edges[0] {
        assert!(matches!(
            description(&t.body, edge),
            EdgeGeometry::Intersection { .. }
        ));
    }
    // Ring orientation: rings run clockwise viewed from outside, so
    // their next-order Newell normal points INTO the solid — opposite
    // the cap's outward normal.
    for cap in [t.top, t.bottom] {
        let outward = outward_normal(&t.body, cap);
        let ring = t.body.get_face(cap).unwrap().rings[0];
        let pts = loop_probe_points(&t.body, ring);
        let Surface::Plane { normal, .. } = newell_plane(&pts, Band::linear(Tol::witness()).unwrap()).unwrap()
        else {
            panic!("plane");
        };
        assert!(normal.dot(outward) < -0.99, "ring winds against its cap");
    }
}

#[test]
fn rounded_square_exercises_tangent_line_arc_joins() {
    // (c) Rounded-corner square: 4 line + 4 arc segments, every join an
    // exact line–arc tangency — smooth joins across genuinely distinct
    // surfaces (plane–cylinder): distinct surface keys, conventional
    // MappedCurve struts.
    let b = FRAC_PI_8.tan();
    // All eight joints are exact tangencies -- declared (#101), one
    // declaration per joint including the closing arc's two. Each
    // vertex carries the bulge of the segment LEAVING it: the four
    // straight legs leave with 0, the four quarter-arcs with b.
    let mut lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.25, 0.0), 0.0),
        ProfileVertex::new(p2(0.75, 0.0), b),
        ProfileVertex::new(p2(1.0, 0.25), 0.0),
        ProfileVertex::new(p2(1.0, 0.75), b),
        ProfileVertex::new(p2(0.75, 1.0), 0.0),
        ProfileVertex::new(p2(0.25, 1.0), b),
        ProfileVertex::new(p2(0.0, 0.75), 0.0),
        ProfileVertex::new(p2(0.0, 0.25), b),
    ]);
    lp = lp.with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let t = extrude(&validated(vec![lp]), Extrusion::Distance(0.5), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (16, 24, 10, 0));
    assert_eq!(v as isize - e as isize + f as isize - r as isize, 2);
    // 4 plane walls + 4 cylinder walls + 2 caps, no sharing.
    assert_eq!(t.body.surfaces().count(), 10);
    let cylinders = t
        .body
        .surfaces()
        .filter(|(_, s)| matches!(s, Surface::Cylinder { .. }))
        .count();
    assert_eq!(cylinders, 4);
    // All eight joins are smooth at first order — and since M5 PR 9
    // the upgrade pass descends one order (OQ7's must-carry at
    // construction): a line–arc tangency is JET-DETERMINATE (the
    // plane and cylinder DETERMINE the ruling; κ_rel = 1/r definite),
    // so the eight struts now carry the intrinsic TangentIntersection
    // description — the fillet-grade class, upgraded exactly as
    // transverse joins upgrade to Intersection.
    for &edge in &t.strut_edges[0] {
        assert!(matches!(
            description(&t.body, edge),
            EdgeGeometry::TangentIntersection { .. }
        ));
    }
    // The sixteen cap-wall rims are all transverse and upgrade to
    // Intersection (the ratified rim decision, M2 PR 4 fix pass).
    let intersections = t
        .body
        .curves()
        .filter(|(_, c)| {
            matches!(
                c.certified().map(topo::EdgeCurve::description),
                Some(EdgeGeometry::Intersection { .. })
            )
        })
        .count();
    assert_eq!(intersections, 16);
    // Adjacent walls at each tangent join really are distinct surfaces.
    for j in 0..8 {
        let a = t.body.get_face(t.side_faces[0][j]).unwrap().surface;
        let b = t
            .body
            .get_face(t.side_faces[0][(j + 1) % 8])
            .unwrap()
            .surface;
        assert_ne!(a, b, "line–arc tangency keeps distinct surfaces");
    }
}

#[test]
fn disc_extrudes_to_a_shared_carrier_cylinder() {
    // (c continued) A two-arc disc: the wall is one carrier circle
    // split into two faces sharing ONE cylinder key (the arc–arc
    // same-carrier smooth-join class).
    let t = extrude(
        &validated(vec![circle_loop(0.0, 0.0, 0.5)]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (4, 6, 4, 0));
    assert_eq!(v as isize - e as isize + f as isize - r as isize, 2);
    // 2 cap planes + exactly one shared cylinder.
    assert_eq!(t.body.surfaces().count(), 3);
    let k0 = t.body.get_face(t.side_faces[0][0]).unwrap().surface;
    let k1 = t.body.get_face(t.side_faces[0][1]).unwrap().surface;
    assert_eq!(k0, k1);
    // Both joins smooth: struts stay MappedCurve.
    for &edge in &t.strut_edges[0] {
        assert!(matches!(
            description(&t.body, edge),
            EdgeGeometry::MappedCurve(_)
        ));
    }
}

#[test]
fn d_profile_mixes_plane_and_cylinder_corners() {
    // (c continued) The D: one chord + one semicircular arc; both
    // joins are 90° corners, so both struts upgrade to plane×cylinder
    // Intersections.
    // The chord leaves (-1,0) straight; the closing semicircle leaves
    // (1,0) with bulge 1.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 1.0),
    ]);
    let t = extrude(&validated(vec![lp]), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let (v, e, f, r) = counts(&t.body);
    assert_eq!((v, e, f, r), (4, 6, 4, 0));
    assert_eq!(t.body.surfaces().count(), 4);
    for &edge in &t.strut_edges[0] {
        assert!(matches!(
            description(&t.body, edge),
            EdgeGeometry::Intersection { .. }
        ));
    }
}

#[test]
fn both_extrusion_directions_build_outward_solids() {
    // (d) ±normal with orientation verified: the cap on the far side of
    // the sweep is outward along w; the sketch-plane cap is outward
    // along −w.
    let vp = validated(vec![l_loop()]);
    let up = extrude(&vp, Extrusion::Distance(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&up.body);
    assert!(outward_normal(&up.body, up.top).z > 0.99);
    assert!(outward_normal(&up.body, up.bottom).z < -0.99);

    let down = extrude(&vp, Extrusion::Distance(-1.0), Tol::witness()).unwrap();
    assert_all_tiers(&down.body);
    // Downward: the swept (top) cap sits at z = −1, outward −z; the
    // bottom cap keeps the sketch plane, outward +z.
    assert!(outward_normal(&down.body, down.top).z < -0.99);
    assert!(outward_normal(&down.body, down.bottom).z > 0.99);
    let top_pts = loop_points(&down.body, down.body.get_face(down.top).unwrap().outer);
    assert!(top_pts.iter().all(|p| (p.z + 1.0).abs() < 1e-12));

    // The vector form agrees with the distance form structurally.
    let vec_down = extrude(&vp, Extrusion::Vector(Vec3::new(0.0, 0.0, -1.0)), Tol::witness()).unwrap();
    assert_all_tiers(&vec_down.body);
    assert_eq!(counts(&vec_down.body), counts(&down.body));
    assert!(outward_normal(&vec_down.body, vec_down.top).z < -0.99);

    // A holed profile extruded downward keeps its ring/genus structure.
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let holed = extrude(
        &validated(vec![outer, circle_loop(0.5, 0.5, 0.1)]),
        Extrusion::Distance(-0.5),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&holed.body);
    let (v, e, f, r) = counts(&holed.body);
    assert_eq!(v as isize - e as isize + f as isize - r as isize, 0); // g = 1
    assert!(outward_normal(&holed.body, holed.top).z < -0.99);
}

#[test]
fn placed_profile_extrudes_along_its_own_normal() {
    // The placement is the profile's own: an exact rigid frame with
    // normal +y (u = ẑ, v = x̂), offset from the origin.
    let plane = SketchPlane::from_frame(Point3::new(2.0, 1.0, 3.0), Vec3::unit_z(), Vec3::unit_x());
    let vp = Profile::new(plane, vec![l_loop()])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert!(outward_normal(&t.body, t.top).y > 0.99);
    assert!(outward_normal(&t.body, t.bottom).y < -0.99);
}

#[test]
fn error_paths_are_typed_and_leave_no_body() {
    let vp = validated(vec![l_loop()]);
    // In-plane vector: no normal component.
    assert_eq!(
        extrude(&vp, Extrusion::Vector(Vec3::new(1.0, 0.0, 0.0)), Tol::witness()).unwrap_err(),
        ExtrudeError::DegenerateExtrusion
    );
    // Zero distance.
    assert_eq!(
        extrude(&vp, Extrusion::Distance(0.0), Tol::witness()).unwrap_err(),
        ExtrudeError::DegenerateExtrusion
    );
    // Sliver distance: strictly inside the band (ε, K·ε) escalates.
    let err = extrude(&vp, Extrusion::Distance(3.0 * eps()), Tol::witness()).unwrap_err();
    assert!(
        matches!(err, ExtrudeError::ExtrusionEscalated { ref source }
            if source.predicate == Some("extrusion_normal_component")),
        "{err:?}"
    );
    // Oblique vector: definite normal + definite in-plane component.
    assert_eq!(
        extrude(&vp, Extrusion::Vector(Vec3::new(0.5, 0.0, 1.0)), Tol::witness()).unwrap_err(),
        ExtrudeError::ObliqueExtrusion
    );
}

#[test]
fn sliver_dihedral_join_is_a_typed_error() {
    // (e) A definite profile corner whose dihedral margin lands in the
    // sliver band at the strut's lever arm: turn angle θ = 3000ε with
    // extrusion length 1e-3 gives margin sin θ · |w| ≈ 3ε ∈ (ε, K·ε)
    // at every CI ε row, while the profile-level corner (arm ~1 m) is
    // definite.
    let theta = 3000.0 * eps();
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0 + theta.cos(), theta.sin()),
        p2(0.0, 1.0),
    ]);
    let err = extrude(&validated(vec![lp]), Extrusion::Distance(1.0e-3), Tol::witness()).unwrap_err();
    match err {
        ExtrudeError::SliverJoin {
            loop_index,
            vertex_index,
            source,
        } => {
            assert_eq!(loop_index, 0);
            assert_eq!(source.predicate, Some("dihedral_wedge"));
            // The shallow corner sits at input vertex 1 = (1, 0); the
            // canonical chain may rotate/reverse, so just check range.
            assert!(vertex_index < 4);
        }
        other => panic!("expected SliverJoin, got {other:?}"),
    }
}

#[test]
fn certification_failures_surface_the_report() {
    // A degenerate profile cannot exist post-validation, so corrupt the
    // extrusion instead: a poisoned vector escalates (typed, no panic).
    let vp = validated(vec![l_loop()]);
    let err = extrude(&vp, Extrusion::Vector(Vec3::new(0.0, 0.0, f64::NAN)), Tol::witness()).unwrap_err();
    assert!(
        matches!(err, ExtrudeError::ExtrusionEscalated { .. }),
        "{err:?}"
    );
    // And the Op wrapper carries certification reports verbatim when an
    // operator-level gate fires (exercised here through the public
    // sweep API only as the absence case: a clean build has none).
    let ok = extrude(&vp, Extrusion::Distance(1.0), Tol::witness());
    assert!(ok.is_ok());
    drop(ok);
    // Shape check: the error type embeds EulerOpError::Certification.
    fn _static_shape(e: EulerOpError) -> ExtrudeError {
        ExtrudeError::from(e)
    }
}

#[test]
fn rebuild_is_byte_identical() {
    // D9: identical input replays into a byte-identical body — keys,
    // points, certified parameter caches, certificates, surfaces.
    let dump = |t: &Extruded<f64>| {
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
        s
    };
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let build = || {
        extrude(
            &validated(vec![outer.clone(), circle_loop(0.5, 0.5, 0.1)]),
            Extrusion::Distance(1.0),
            Tol::witness(),
        )
        .unwrap()
    };
    let a = build();
    let b = build();
    assert_eq!(dump(&a), dump(&b));
    assert_eq!(
        format!("{:?}", a.strut_edges),
        format!("{:?}", b.strut_edges)
    );
    assert_eq!(format!("{:?}", a.side_faces), format!("{:?}", b.side_faces));
}

#[test]
fn dual_lane_value_channel_matches_f64_bitwise() {
    // The Dual build takes the identical decision path; certificate
    // value channels equal the f64 build bitwise (tangent data never
    // decides).
    use geom_core::{Dual, Dual64};
    let lift = |lp: &ProfileLoop<f64>| -> ProfileLoop<Dual64> {
        ProfileLoop::new(
            lp.vertices()
                .iter()
                .map(|v| {
                    ProfileVertex::new(
                        Point2::new(Dual::constant(v.pos().x), Dual::constant(v.pos().y)),
                        Dual::constant(v.bulge()),
                    )
                })
                .collect(),
        )
    };
    let f = extrude(&validated(vec![l_loop()]), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    let dp = Profile::new(SketchPlane::<Dual64>::xy(), vec![lift(&l_loop())])
        .validate(Tol::witness())
        .unwrap();
    let d = extrude(&dp, Extrusion::Distance(Dual::constant(1.0)), Tol::witness()).unwrap();
    assert_eq!(validate_geometric(&d.body, Tol::witness()), Ok(()));
    let f_res: Vec<f64> = f
        .body
        .curves()
        .map(|(_, c)| c.certified().unwrap().certificate().max_residual)
        .collect();
    let d_res: Vec<Dual<f64>> = d
        .body
        .curves()
        .map(|(_, c)| c.certified().unwrap().certificate().max_residual)
        .collect();
    assert_eq!(f_res.len(), d_res.len());
    for (fv, dv) in f_res.iter().zip(&d_res) {
        assert_eq!(fv.to_bits(), dv.value.to_bits());
    }
}
