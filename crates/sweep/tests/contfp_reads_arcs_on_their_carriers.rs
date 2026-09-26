//! **`contfp` reads every loop on its edges' own carriers.**
//!
//! A planar face bounded by arcs is not the polygon through its
//! vertices: an arc bowing outward leaves region between the polygon
//! and the boundary, and one bowing past a period turns the polygon
//! into a different shape altogether. `contfp` answers interior and
//! exterior from the carrier walk, and its boundary pre-pass decides
//! an arc's trim as distances, so an arc's end neighbourhood is the
//! band's own width along the carrier however short or long the arc.
//!
//! Every face here comes through the public door a user takes — a
//! profile of bulged segments, extruded — and every probe point is
//! placed in the cap's plane from the profile's own numbers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::fillet_edges;
use sweep::test_support::{ROD_FILLET, ROD_L, extruded, rod_chord_at, rod_creases};
use topo::{Body, ContactRecords, FaceContainment, FaceKey, ValidationError, contfp};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness tolerance makes a band")
}

/// The band's zero width, metres: the ε every probe offset is scaled by,
/// so the rows hold at every ε row the gate runs.
fn eps() -> f64 {
    band().zero()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The planar face of `body` lying in `z = z` — its key and its unit
/// normal.
fn cap_at(body: &Body<f64>, z: f64) -> (FaceKey, Vec3<f64>) {
    body.faces()
        .find_map(|(k, f)| match body.get_surface(f.surface)? {
            Surface::Plane { origin, normal, .. }
                if normal.x == 0.0 && normal.y == 0.0 && origin.z == z =>
            {
                Some((k, *normal))
            }
            _ => None,
        })
        .expect("the extrude has a cap in that plane")
}

/// The x-coordinates of a face's outer-loop vertices.
fn outer_vertex_xs(body: &Body<f64>, face: FaceKey) -> Vec<f64> {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the outer loop is a cycle");
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| {
            let v = body.get_half_edge(he).unwrap().start;
            body.get_point(body.get_vertex(v).unwrap().point).unwrap().x
        })
        .collect()
}

fn ask(body: &Body<f64>, (face, normal): (FaceKey, Vec3<f64>), q: Point3<f64>) -> FaceContainment {
    contfp(body, face, normal, q, band()).unwrap_or_else(|e| panic!("contfp refused at {q:?}: {e}"))
}

/// The bored D-rod with both of its creases filleted: its transverse
/// caps' outer loops are a flat, two fillet arcs and the rod's MAJOR
/// arc, which dips past the chord its two ends span, and each cap
/// carries the bore as a ring.
fn filleted_bored_d_rod() -> Body<f64> {
    let flat = 0.3;
    let c = rod_chord_at(flat);
    let loops: Vec<ProfileLoop<f64>> = vec![
        bulge_loop(vec![
            (p2(flat, c.half), c.wall_bulge),
            (p2(flat, -c.half), 0.0),
        ]),
        profile::circle(p2(0.0, 0.0), 0.15, tol())
            .expect("the bore's disc")
            .into(),
    ];
    let source = extruded(SketchPlane::xy(), loops, ROD_L, tol());
    let creases = rod_creases(&source);
    fillet_edges(&source, &creases, ROD_FILLET, tol())
        .expect("the D-rod's creases fillet")
        .body
}

/// The lune point: in the rod, out of the bore, and on the far side of
/// the axis from every vertex of the cap's outer loop — so the polygon
/// through those vertices excludes it while the loop's region holds it.
const LUNE: (f64, f64) = (-0.3, 0.0);

#[test]
fn the_d_rod_cap_holds_the_lune_point() {
    let body = filleted_bored_d_rod();
    for z in [0.0, ROD_L] {
        let cap = cap_at(&body, z);
        let xs = outer_vertex_xs(&body, cap.0);
        assert!(
            xs.len() >= 3 && xs.iter().all(|&x| x > 0.0),
            "the premise: every outer vertex lies at x > 0, so the vertex polygon \
             excludes the lune: {xs:?}"
        );
        assert_eq!(
            ask(&body, cap, Point3::new(LUNE.0, LUNE.1, z)),
            FaceContainment::In,
            "the lune point lies inside the cap at z = {z}"
        );
        // And the bore stays a hole.
        assert_eq!(
            ask(&body, cap, Point3::new(0.0, 0.0, z)),
            FaceContainment::Out,
            "the bore's centre lies outside the cap at z = {z}"
        );
    }
}

/// The same question through the census, the public door that asks it:
/// a brick standing on the D-rod's top cap over the lune. Its bottom
/// vertices lie in the cap's plane and inside the cap, which is a
/// contact the census must report; read from the vertex polygon they
/// were outside the cap, and the body passed as a clean one.
#[test]
fn the_census_sees_a_brick_standing_in_the_lune() {
    let mut body = filleted_bored_d_rod();
    let brick =
        sweep::test_support::brick((-0.35, -0.25), (-0.05, 0.05), (ROD_L, ROD_L + 0.5), tol());
    topo::graft_disjoint(&mut body, &brick, tol()).expect("two disjoint solids in one body");
    let (cap, _) = cap_at(&body, ROD_L);
    let errors = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
        .expect_err("a brick standing on the cap touches it");
    let on_cap = errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: topo::CensusContact::VertexOnFace { face, .. },
                    ..
                } if *face == cap
            )
        })
        .count();
    assert_eq!(
        on_cap, 4,
        "each of the brick's four bottom vertices lies inside the cap: {errors:?}"
    );
}

/// A slot (two semicircular ends) and a rounded rectangle (four quarter
/// arcs), the shapes the vertex-polygon walk was reviewed on: every probe
/// on a grid answers as the analytic region does.
#[test]
fn a_slot_and_a_rounded_rectangle_answer_their_regions() {
    // The slot: centre segment x ∈ [−1, 1], radius 0.5.
    let slot = extruded(
        SketchPlane::xy(),
        vec![
            bulge_loop(vec![
                (p2(-1.0, -0.5), 0.0),
                (p2(1.0, -0.5), 1.0),
                (p2(1.0, 0.5), 0.0),
                (p2(-1.0, 0.5), 1.0),
            ])
            .with_tangent_joints(vec![0, 1, 2, 3]),
        ],
        1.0,
        tol(),
    );
    let slot_in = |x: f64, y: f64| {
        let cx = x.clamp(-1.0, 1.0);
        (x - cx).hypot(y) < 0.5
    };
    // The rounded rectangle: [−1, 1] × [−0.6, 0.6], corner radius 0.3.
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let rr = extruded(
        SketchPlane::xy(),
        vec![
            bulge_loop(vec![
                (p2(-0.7, -0.6), 0.0),
                (p2(0.7, -0.6), q),
                (p2(1.0, -0.3), 0.0),
                (p2(1.0, 0.3), q),
                (p2(0.7, 0.6), 0.0),
                (p2(-0.7, 0.6), q),
                (p2(-1.0, 0.3), 0.0),
                (p2(-1.0, -0.3), q),
            ])
            .with_tangent_joints((0..8).collect()),
        ],
        1.0,
        tol(),
    );
    let rr_in = |x: f64, y: f64| {
        let (cx, cy) = (x.clamp(-0.7, 0.7), y.clamp(-0.3, 0.3));
        (x - cx).hypot(y - cy) < 0.3
    };
    for (name, body, inside) in [
        ("slot", &slot, &slot_in as &dyn Fn(f64, f64) -> bool),
        ("rounded rectangle", &rr, &rr_in),
    ] {
        let cap = cap_at(body, 1.0);
        let mut asked = 0;
        for i in 0..=40 {
            for j in 0..=24 {
                // An irrational offset keeps every probe off the boundary
                // and off every vertex's ray line.
                let x = -1.6 + 0.08 * f64::from(i) + 0.001_234_5;
                let y = -0.96 + 0.08 * f64::from(j) + 0.000_987_6;
                let want = if inside(x, y) {
                    FaceContainment::In
                } else {
                    FaceContainment::Out
                };
                assert_eq!(
                    ask(body, cap, Point3::new(x, y, 1.0)),
                    want,
                    "{name} at ({x}, {y})"
                );
                asked += 1;
            }
        }
        assert!(asked > 1000, "{name}: the grid was asked");
    }
}

/// A pie slice whose arc is SHORT (`w = 0.02` rad, radius 10): the
/// cosine window's end zone along the carrier would be `ε / sin(w/2)`,
/// a hundred times `ε`, with escalation ten times further out. Asked
/// along the carrier `s` from the arc's end `B`, on both sides of it.
#[test]
fn a_short_arc_is_on_and_off_at_the_bands_own_width() {
    let (r, w) = (10.0f64, 0.02f64);
    let body = extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (p2(0.0, 0.0), 0.0),
            (p2(r, 0.0), (w / 4.0).tan()),
            (p2(r * w.cos(), r * w.sin()), 0.0),
        ])],
        1.0,
        tol(),
    );
    let cap = cap_at(&body, 1.0);
    let at = |theta: f64| Point3::new(r * theta.cos(), r * theta.sin(), 1.0);
    for s in [50.0 * eps(), 500.0 * eps()] {
        let inside = ask(&body, cap, at(w - s / r));
        assert!(
            matches!(inside, FaceContainment::OnEdge(_)),
            "{s} m inside the end is on the arc: {inside:?}"
        );
        assert_eq!(
            ask(&body, cap, at(w + s / r)),
            FaceContainment::Out,
            "{s} m past the end is off the face"
        );
    }
}

/// The mirror: a pac-man whose arc runs the long way round
/// (`w = 2π − 0.02`), with its mouth closed by two radii. The same end
/// zone compresses by `sin` of the complement.
#[test]
fn a_near_full_arc_is_on_and_off_at_the_bands_own_width() {
    let (r, gap) = (10.0f64, 0.02f64);
    let w = core::f64::consts::TAU - gap;
    let (a, b) = (gap / 2.0, -gap / 2.0);
    let body = extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (p2(0.0, 0.0), 0.0),
            (p2(r * a.cos(), r * a.sin()), (w / 4.0).tan()),
            (p2(r * b.cos(), r * b.sin()), 0.0),
        ])],
        1.0,
        tol(),
    );
    let cap = cap_at(&body, 1.0);
    let at = |theta: f64| Point3::new(r * theta.cos(), r * theta.sin(), 1.0);
    for s in [50.0 * eps(), 500.0 * eps()] {
        let inside = ask(&body, cap, at(b - s / r));
        assert!(
            matches!(inside, FaceContainment::OnEdge(_)),
            "{s} m inside the end is on the arc: {inside:?}"
        );
        assert_eq!(
            ask(&body, cap, at(b + s / r)),
            FaceContainment::Out,
            "{s} m past the end, in the mouth, is off the face"
        );
    }
    // Deep in the body, far from the polygon through its three vertices.
    assert_eq!(
        ask(&body, cap, Point3::new(-5.0, 0.0, 1.0)),
        FaceContainment::In
    );
}
