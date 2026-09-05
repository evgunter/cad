//! **The cap-rim dihedral is transverse by construction** — the
//! executable form of the argument written at `extrude`'s cap-rim
//! `Smooth` arm.
//!
//! `extrude` admits only an extrusion vector trilean-parallel to the
//! sketch plane's normal `n` (a definite in-plane component is
//! `ObliqueExtrusion`), so every wall is ruled in `n` and every cap is
//! a plane of normal `±n`. A wall's normal is therefore perpendicular
//! to `n` at every rim point and the cap-wall tangent planes never
//! coincide: `classify_dihedral` decides `Transverse` wherever it
//! decides at all, and the only other outcome is the typed escalation
//! (`SliverRim`).
//!
//! Each row builds a body through the public door and reads BOTH
//! instruments at every cap rim: the description the arm actually
//! stored (`Intersection` iff the transverse arm ran) and
//! `classify_dihedral` re-run on the arm's own inputs (the certified
//! carrier's mid-parameter witness, `edge_extent` as the meter).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{DihedralClass, EdgeDescription, classify_dihedral, edge_extent};
use geom_core::{Band, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeError, Extruded, Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops).validate(Tol::witness()).unwrap()
}

/// Every edge of a face, over its outer loop and every ring.
fn face_edges(body: &Body<f64>, face: FaceKey) -> Vec<EdgeKey> {
    let fd = body.get_face(face).unwrap();
    let mut edges = Vec::new();
    for lk in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            edges.push(body.get_half_edge(he).unwrap().edge);
        }
    }
    edges
}

/// The face on the other side of `edge` from `face`.
fn face_across(body: &Body<f64>, edge: EdgeKey, face: FaceKey) -> FaceKey {
    let e = body.get_edge(edge).unwrap();
    let of = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let (plus, minus) = (of(e.he_plus), of(e.he_minus));
    assert!(plus == face || minus == face, "edge is not on the face");
    if plus == face { minus } else { plus }
}

/// The classifier's verdict at one cap rim, on the arm's own inputs.
fn verdict(body: &Body<f64>, cap: FaceKey, edge: EdgeKey) -> Result<DihedralClass, String> {
    let band = Band::linear(Tol::witness()).unwrap();
    let wall = face_across(body, edge, cap);
    let s_cap = body
        .get_surface(body.get_face(cap).unwrap().surface)
        .unwrap()
        .clone();
    let s_wall = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let (t0, t1) = curve.params();
    let witness = curve.carrier().eval(t0 + (t1 - t0) * 0.5);
    let ends = curve.carrier().eval(t0).distance(curve.carrier().eval(t1));
    let extent = edge_extent(curve.carrier(), t0, t1, ends);
    classify_dihedral(&s_cap, &s_wall, witness, extent, band).map_err(|e| format!("{e:?}"))
}

/// Asserts that every cap rim of `built` reached the transverse arm —
/// by the stored description, and by the classifier re-run on the
/// arm's inputs.
fn assert_every_cap_rim_transverse(name: &str, built: &Extruded<f64>) {
    let body = &built.body;
    let mut rims = 0usize;
    for cap in [built.bottom, built.top] {
        for edge in face_edges(body, cap) {
            rims += 1;
            let desc = body
                .get_curve_geom(body.get_edge(edge).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap()
                .description()
                .clone();
            assert!(
                matches!(desc, EdgeDescription::Intersection { .. }),
                "{name}: cap rim {edge:?} did not take the transverse arm: {desc:?}",
            );
            assert_eq!(
                verdict(body, cap, edge),
                Ok(DihedralClass::Transverse),
                "{name}: cap rim {edge:?} classified non-transversely",
            );
        }
    }
    assert!(rims >= 2, "{name}: no cap rims were read");
}

fn square() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 2.0), p2(0.0, 2.0)])
}

/// The 6-vertex all-line L: a concave corner among convex ones.
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

/// A circle as two semicircular arcs — one cylinder wall, two
/// near-closed rim arcs whose chord is a diameter.
fn circle_loop(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(cx - r, cy), 1.0),
        ProfileVertex::new(p2(cx + r, cy), 1.0),
    ])
}

/// An obround: two lines closed by two semicircular arcs, every join
/// tangent-continuous — the profile whose STRUT joins are smooth, so
/// its walls meet the caps as a cylinder-plane pair and a plane-plane
/// pair at the same rim.
fn obround_loop() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(-1.0, -0.5), 0.0),
        ProfileVertex::new(p2(1.0, -0.5), 1.0),
        ProfileVertex::new(p2(1.0, 0.5), 0.0),
        ProfileVertex::new(p2(-1.0, 0.5), 1.0),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3])
}

/// A rounded-corner square: line legs joined by quarter-arc fillets,
/// every join tangent-continuous.
fn stadium_corners_loop() -> ProfileLoop<f64> {
    let q = (core::f64::consts::FRAC_PI_4 / 2.0).tan();
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(-1.0, -2.0), 0.0),
        ProfileVertex::new(p2(1.0, -2.0), q),
        ProfileVertex::new(p2(2.0, -1.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.0), q),
        ProfileVertex::new(p2(1.0, 2.0), 0.0),
        ProfileVertex::new(p2(-1.0, 2.0), q),
        ProfileVertex::new(p2(-2.0, 1.0), 0.0),
        ProfileVertex::new(p2(-2.0, -1.0), q),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7])
}

/// A concave arc leg (negative bulge): the wall cylinder's material is
/// OUTSIDE the carrier, so the wall face carries sense `false`.
fn concave_arc_loop() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0, 2.0), 0.0),
        ProfileVertex::new(p2(0.0, 2.0), -0.4),
    ])
}

#[test]
fn every_extruded_cap_rim_is_transverse() {
    let tol = Tol::witness();
    let plane = SketchPlane::xy();

    // Straight extrusions over every profile leg kind, in both
    // directions and through both doors.
    let rows: Vec<(&str, ValidatedProfile<f64>, Extrusion<f64>)> = vec![
        (
            "square/+distance",
            validated(plane, vec![square()]),
            Extrusion::Distance(1.0),
        ),
        (
            "square/-distance",
            validated(plane, vec![square()]),
            Extrusion::Distance(-1.0),
        ),
        (
            "square/vector",
            validated(plane, vec![square()]),
            Extrusion::Vector(Vec3::new(0.0, 0.0, 3.0)),
        ),
        (
            "L (concave corner)",
            validated(plane, vec![l_loop()]),
            Extrusion::Distance(0.75),
        ),
        (
            "circle (two semicircle arcs)",
            validated(plane, vec![circle_loop(0.0, 0.0, 1.5)]),
            Extrusion::Distance(2.0),
        ),
        (
            "obround (tangent line-arc joins)",
            validated(plane, vec![obround_loop()]),
            Extrusion::Distance(1.0),
        ),
        (
            "rounded square (quarter-arc fillets)",
            validated(plane, vec![stadium_corners_loop()]),
            Extrusion::Distance(0.5),
        ),
        (
            "concave arc leg",
            validated(plane, vec![concave_arc_loop()]),
            Extrusion::Distance(1.25),
        ),
        (
            "holed (square with a circular ring)",
            validated(plane, vec![square(), circle_loop(1.0, 1.0, 0.5)]),
            Extrusion::Distance(1.0),
        ),
        (
            // A tilted sketch plane moves the whole configuration
            // rigidly; the relation the arm rests on is a relation
            // between the cap and the walls, not to world z.
            "tilted sketch plane",
            validated(
                SketchPlane::from_frame(
                    geom_core::Point3::new(0.3, -0.2, 0.7),
                    Vec3::new(1.0, 1.0, 0.0).normalize(),
                    Vec3::new(-1.0, 1.0, 2.0).normalize(),
                ),
                vec![obround_loop()],
            ),
            Extrusion::Distance(1.0),
        ),
        (
            // The direction gates admit an in-plane component up to
            // the coincidence threshold against a normal component at
            // the escalation one, so the WORST tilt any admitted
            // extrusion can give the walls is 1/K — independent of ε.
            // At that ratio the rim is still definitely transverse.
            "worst admitted obliquity (in-plane eps, height K*eps)",
            validated(plane, vec![square()]),
            Extrusion::Vector(Vec3::new(tol.eps(), 0.0, tol.k() * tol.eps())),
        ),
        (
            "admitted obliquity at unit height",
            validated(plane, vec![square()]),
            Extrusion::Vector(Vec3::new(tol.eps(), 0.0, 1.0)),
        ),
        (
            // A wall cylinder whose radius is far below the caps'
            // extent: the folded lever arm is the radius, not the
            // chord, and the wedge margin survives it.
            "tiny-radius arc leg (r = 1e-7)",
            validated(plane, vec![circle_loop(0.0, 0.0, 1e-7)]),
            Extrusion::Distance(1.0),
        ),
        (
            // A near-closed arc rim, whose CHORD collapses: the meter
            // is `edge_extent`'s carrier diameter instead.
            "near-closed single arc rim",
            validated(
                plane,
                vec![ProfileLoop::new(vec![
                    ProfileVertex::new(p2(-0.01, 0.0), 100.0),
                    ProfileVertex::new(p2(0.01, 0.0), 0.0),
                ])],
            ),
            Extrusion::Distance(1.0),
        ),
    ];

    for (name, profile, extrusion) in rows {
        let built = extrude(&profile, extrusion, tol).expect("the row's profile extrudes");
        assert_every_cap_rim_transverse(name, &built);
    }
}

/// The two direction gates are what make the argument at the arm true,
/// so their refusals are asserted beside it: a definite in-plane
/// component is `ObliqueExtrusion`, an in-band one escalates typed
/// under its own predicate, and a sliver height escalates under the
/// other. Neither builds a body, so neither reaches the rim upgrade.
#[test]
fn the_direction_gates_refuse_before_the_arm() {
    let tol = Tol::witness();
    let profile = || validated(SketchPlane::xy(), vec![square()]);
    let err = extrude(&profile(), Extrusion::Vector(Vec3::new(0.5, 0.0, 1.0)), tol)
        .expect_err("a definitely oblique extrusion vector is refused");
    assert!(
        matches!(err, ExtrudeError::ObliqueExtrusion),
        "expected the oblique refusal, got {err:?}",
    );

    let err = extrude(
        &profile(),
        Extrusion::Vector(Vec3::new(5.0 * tol.eps(), 0.0, 1.0)),
        tol,
    )
    .expect_err("an in-band in-plane component escalates");
    assert!(
        matches!(
            &err,
            ExtrudeError::ExtrusionEscalated { source }
                if source.predicate == Some("extrusion_obliquity")
        ),
        "expected the obliquity escalation, got {err:?}",
    );

    let err = extrude(&profile(), Extrusion::Distance(5.0 * tol.eps()), tol)
        .expect_err("a sliver height escalates");
    assert!(
        matches!(
            &err,
            ExtrudeError::ExtrusionEscalated { source }
                if source.predicate == Some("extrusion_normal_component")
        ),
        "expected the normal-component escalation, got {err:?}",
    );
}
