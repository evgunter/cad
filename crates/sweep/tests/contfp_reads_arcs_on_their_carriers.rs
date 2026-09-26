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
    // Each end, with the direction that runs INTO the arc from it.
    for (end, into) in [(0.0, 1.0), (w, -1.0)] {
        for s in [50.0 * eps(), 500.0 * eps()] {
            let inside = ask(&body, cap, at(end + into * s / r));
            assert!(
                matches!(inside, FaceContainment::OnEdge(_)),
                "{s} m inside the end at {end} is on the arc: {inside:?}"
            );
            assert_eq!(
                ask(&body, cap, at(end - into * s / r)),
                FaceContainment::Out,
                "{s} m past the end at {end} is off the face"
            );
        }
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
    // The arc runs counterclockwise from `a` the long way round to `b`.
    for (end, into) in [(a, 1.0), (b, -1.0)] {
        for s in [50.0 * eps(), 500.0 * eps()] {
            let inside = ask(&body, cap, at(end + into * s / r));
            assert!(
                matches!(inside, FaceContainment::OnEdge(_)),
                "{s} m inside the end at {end} is on the arc: {inside:?}"
            );
            assert_eq!(
                ask(&body, cap, at(end - into * s / r)),
                FaceContainment::Out,
                "{s} m past the end at {end}, in the mouth, is off the face"
            );
        }
    }
    // Deep in the body, far from the polygon through its three vertices.
    assert_eq!(
        ask(&body, cap, Point3::new(-5.0, 0.0, 1.0)),
        FaceContainment::In
    );
}

/// A disc prism cut by a plane tilted 0.3 rad: the section face is
/// bounded by two ELLIPSE arcs over two vertices.
fn cut_cylinder() -> Body<f64> {
    use topo::splitting::{SplitPart, SplitPlane, split};
    let tall =
        sweep::test_support::prism(vec![(p2(-1.0, 0.0), 1.0), (p2(1.0, 0.0), 1.0)], 2.5, tol());
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos()),
    };
    let result = split(&tall, &plane, tol()).expect("the plane cuts the prism");
    let SplitPart::Body(above) = result.above else {
        panic!("the part above the cut is a body");
    };
    above
}

/// The section face of [`cut_cylinder`] — the planar face whose edges
/// are ellipses — with its unit normal and its elliptic edges.
fn section(body: &Body<f64>) -> ((FaceKey, Vec3<f64>), Vec<topo::EdgeKey>) {
    for (k, f) in body.faces() {
        let Some(&Surface::Plane { normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            continue;
        };
        let ellipses: Vec<_> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().edge)
            .filter(|&e| {
                let c = body.get_edge(e).and_then(|e| body.get_curve_geom(e.curve));
                matches!(
                    c.and_then(|c| c.certified()).map(|c| c.carrier()),
                    Some(geom::Curve3::Ellipse { .. })
                )
            })
            .collect();
        if !ellipses.is_empty() {
            return ((k, normal), ellipses);
        }
    }
    panic!("the cut has a section face bounded by ellipses");
}

/// A point ON an elliptic edge of the section face is on the boundary:
/// the pre-pass reads the ellipse on its own conic and trim, so the
/// answer names the edge (or, within the band of an end, the vertex) —
/// never an escalation over a margin nothing metred.
#[test]
fn a_point_on_an_ellipse_edge_reads_on_the_boundary() {
    let body = cut_cylinder();
    let (cap, ellipses) = section(&body);
    let mut asked = 0;
    for e in ellipses {
        let curve = body
            .get_curve_geom(body.get_edge(e).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = curve.params();
        for i in 0..40 {
            let q = curve
                .carrier()
                .eval(t0 + (t1 - t0) * (f64::from(i) + 0.371) / 40.0);
            let got = ask(&body, cap, q);
            assert!(
                matches!(got, FaceContainment::OnEdge(k) if k == e)
                    || matches!(got, FaceContainment::OnVertex(_)),
                "a point on the ellipse at {q:?} reads {got:?}"
            );
            asked += 1;
        }
    }
    assert!(asked >= 40, "the section's ellipses were asked");
}

/// The same edge through the census: a brick whose corner sits on the
/// section's elliptic edge, off both of its vertices. The census reports
/// the contact and no escalation over a margin nothing metred.
#[test]
fn the_census_reads_a_corner_on_an_ellipse_edge_without_a_minted_margin() {
    let mut body = cut_cylinder();
    // At azimuth π/2 the section plane stands at z = 1.25, so the corner
    // (0, 1, 1.25) is on the ellipse; the brick reaches away from the
    // cylinder in x and y and below the cut in z.
    let brick = sweep::test_support::brick((0.0, 0.3), (1.0, 1.3), (0.95, 1.25), tol());
    topo::graft_disjoint(&mut body, &brick, tol()).expect("two solids meeting at a point");
    let errors = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
        .expect_err("a corner on the section's edge is a contact");
    for e in &errors {
        if let ValidationError::CensusEscalated { cause } = e {
            assert!(
                !matches!(cause.margin, geom_core::MarginDiag::Invalid),
                "a census escalation over a margin nothing metred: {e:?}"
            );
        }
    }
}

/// A disc prism with its vertices at `(0, ±1)`, cut by a plane tilted
/// `tilt` about `y`: the section's two ellipse arcs have `a/b = 1/cos
/// tilt` (14.1 at 1.5 rad, 19.7 at 1.52) and their ends at the MINOR
/// vertices, where the edge's speed is `a`.
fn steep_cut(tilt: f64) -> Body<f64> {
    use topo::splitting::{SplitPart, SplitPlane, split};
    let h = 2.0 * tilt.tan() + 20.0;
    let tall =
        sweep::test_support::prism(vec![(p2(0.0, -1.0), 1.0), (p2(0.0, 1.0), 1.0)], h, tol());
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, h / 2.0),
        normal: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
    };
    let result = split(&tall, &plane, tol()).expect("the plane cuts the prism");
    let SplitPart::Body(above) = result.above else {
        panic!("the part above the cut is a body");
    };
    above
}

/// The point on `curve` a CHORD distance `s` from its end at `te`,
/// moving into the arc (`into` = ±1 along the parameter).
fn along_from_end(curve: &geom::Curve3<f64>, te: f64, into: f64, s: f64) -> Point3<f64> {
    let v = curve.eval(te);
    let (mut lo, mut hi) = (0.0f64, 1e-3f64);
    for _ in 0..200 {
        let m = 0.5 * (lo + hi);
        if (curve.eval(te + into * m) - v).norm() < s {
            lo = m;
        } else {
            hi = m;
        }
    }
    curve.eval(te + into * lo)
}

/// **A steep ellipse's end is the band's own width in metres.** At the
/// minor vertex the edge's speed is `a`, so an end zone measured as a
/// unit chord levered by `b` reached `(a/b)·ε` along the edge, past the
/// vertex pass's own `10ε`: a legal body read `Corrupt`. Points `11ε`
/// to `18ε` inside each end, at `a/b` 14.1 and 19.7, read on the edge
/// (or, in the band, escalate) and never `Corrupt`.
#[test]
fn a_steep_ellipses_end_zone_is_the_bands_own_width() {
    for tilt in [1.5f64, 1.52] {
        let body = steep_cut(tilt);
        let (cap, ellipses) = section(&body);
        for e in ellipses {
            let curve = body
                .get_curve_geom(body.get_edge(e).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let (t0, t1) = curve.params();
            for (te, into) in [(t0, 1.0), (t1, -1.0)] {
                for k in [11.0, 13.0, 15.0, 18.0, 40.0] {
                    let q = along_from_end(curve.carrier(), te, into, k * eps());
                    let got = contfp(&body, cap.0, cap.1, q, band());
                    assert!(
                        matches!(got, Ok(FaceContainment::OnEdge(k)) if k == e)
                            || (k < 15.0 && matches!(got, Err(topo::ContainError::Escalated(_)))),
                        "tilt {tilt}: {k}ε inside the end at {te} reads {got:?}"
                    );
                }
            }
        }
    }
}

/// **A steep ellipse is not ON where it is off.** At the major vertex a
/// unit-coordinate miss levered by the minor semi-axis understates the
/// distance by `b/a`, so a point `12ε` or `15ε` off read `OnEdge` — and
/// the reduction splits an edge there. It reads off the face or in it.
#[test]
fn a_steep_ellipse_is_off_where_it_is_off() {
    for tilt in [1.5f64, 1.52] {
        let body = steep_cut(tilt);
        let (cap, ellipses) = section(&body);
        for e in ellipses {
            let curve = body
                .get_curve_geom(body.get_edge(e).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let geom::Curve3::Ellipse { center, u_ref, .. } = *curve.carrier() else {
                panic!("an ellipse");
            };
            let (t0, t1) = curve.params();
            for th in [0.0f64, core::f64::consts::PI] {
                if (th - t0).rem_euclid(core::f64::consts::TAU) >= t1 - t0 {
                    continue;
                }
                let p = curve.carrier().eval(th);
                let out = (p - center).normalize();
                let _ = u_ref;
                for d in [12.0, 15.0, 20.0, 40.0] {
                    for s in [1.0, -1.0] {
                        let got = ask(&body, cap, p + out * (s * d * eps()));
                        let want = if s > 0.0 {
                            FaceContainment::Out
                        } else {
                            FaceContainment::In
                        };
                        assert_eq!(
                            got,
                            want,
                            "tilt {tilt}: {}{d}ε off the major vertex",
                            if s > 0.0 { "+" } else { "−" }
                        );
                    }
                }
            }
        }
    }
}

/// The census's door onto the same end: a brick corner on the steep
/// ellipse `15ε` from the vertex. A legal body; the census reports the
/// contact and never tells the user to repair their topology.
#[test]
fn the_census_reads_a_corner_near_a_steep_ellipses_end() {
    let mut body = steep_cut(1.52);
    let (_, ellipses) = section(&body);
    let mut corner = None;
    for e in ellipses {
        let curve = body
            .get_curve_geom(body.get_edge(e).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = curve.params();
        for (te, into) in [(t0, 1.0), (t1, -1.0)] {
            if curve.carrier().eval(te).y < 0.0 {
                continue;
            }
            let q = along_from_end(curve.carrier(), te, into, 15.0 * eps());
            if q.x > 0.0 {
                corner = Some(q);
            }
        }
    }
    let q = corner.expect("a corner on the x > 0 side");
    let brick =
        sweep::test_support::brick((q.x, q.x + 0.3), (q.y, q.y + 0.3), (q.z - 0.3, q.z), tol());
    topo::graft_disjoint(&mut body, &brick, tol()).expect("two solids meeting at a point");
    let errors = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
        .expect_err("a corner on the section's edge is a contact");
    assert!(
        errors.iter().all(|e| !matches!(
            e,
            ValidationError::CensusUnsupported {
                cause: topo::CensusUnsupportedCause::Containment(topo::ContainError::Corrupt),
                ..
            }
        )),
        "a legal body reads Corrupt: {errors:?}"
    );
}

/// **The band sweep on the steep face** (`a/b` 19.7): rings of points
/// around every vertex, and both normals of every edge, from `10.5ε` to
/// `10⁴ε`. Nothing reads `Corrupt`; nothing off an edge's interior by
/// more than the escalation band reads on the boundary, and none of the
/// ring points reads `OnVertex`.
#[test]
fn the_steep_face_sweeps_clean_from_the_band_out() {
    let body = steep_cut(1.52);
    let (cap, _) = section(&body);
    let (face, normal) = cap;
    let f = body.get_face(face).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("a cycle");
    };
    let ax = if normal.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (ax - normal * ax.dot(normal)).normalize();
    let v = normal.cross(u);
    let mut bad = Vec::new();
    let mut asked = 0;
    for he in body.loop_cycle(first).unwrap() {
        let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
        let curve = body
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = curve.params();
        let v0 = curve.carrier().eval(t0);
        for k in [10.5, 12.0, 20.0, 50.0, 200.0, 1000.0, 1e4] {
            for j in 0..48 {
                let (s, c) = ((f64::from(j) + 0.37) * core::f64::consts::TAU / 48.0).sin_cos();
                let got = contfp(
                    &body,
                    face,
                    normal,
                    v0 + (u * c + v * s) * (k * eps()),
                    band(),
                );
                asked += 1;
                if matches!(
                    got,
                    Ok(FaceContainment::OnVertex(_)) | Err(topo::ContainError::Corrupt)
                ) {
                    bad.push(format!("ring {k}ε: {got:?}"));
                }
            }
        }
        for i in 0..60 {
            let fr = (f64::from(i) + 0.5) / 60.0 * 0.9 + 0.05;
            let t = t0 + (t1 - t0) * fr;
            let p = curve.carrier().eval(t);
            let h = 1e-7 * (t1 - t0);
            let tan = (curve.carrier().eval(t + h) - curve.carrier().eval(t - h)).normalize();
            let nn = normal.cross(tan).normalize();
            for k in [10.5, 11.0, 13.0, 20.0, 50.0, 200.0, 1000.0, 1e4] {
                for s in [1.0, -1.0] {
                    let got = contfp(&body, face, normal, p + nn * (s * k * eps()), band());
                    asked += 1;
                    if matches!(
                        got,
                        Ok(FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_))
                            | Err(topo::ContainError::Corrupt)
                    ) {
                        bad.push(format!("normal {s}·{k}ε at {fr:.3}: {got:?}"));
                    }
                }
            }
        }
    }
    assert!(asked > 2000, "the sweep was asked");
    assert!(
        bad.is_empty(),
        "{} bad of {asked}: {:?}",
        bad.len(),
        &bad[..bad.len().min(8)]
    );
}

/// **An in-band clearance from an uncrossable edge's ball skips the ray,
/// it does not escalate the walk.** Points just outside the ball a
/// vessel cavity's spiric arc is held in, and 0.3 m or 1 m back from it
/// along the face's plane, are outside the face: some rays graze the
/// ball within the band and are abandoned, and the rest answer.
#[test]
fn an_in_band_ball_clearance_skips_the_ray() {
    let (_, cavity) = crate::spiric_rim::vessel_cavity(1.0 / 128.0);
    let mut asked = 0;
    for (fk, f) in cavity.faces() {
        let Some(&Surface::Plane { normal, .. }) = cavity.get_surface(f.surface) else {
            continue;
        };
        let topo::LoopBoundary::Cycle { first } = cavity.get_loop(f.outer).unwrap().boundary else {
            continue;
        };
        for he in cavity.loop_cycle(first).unwrap() {
            let e = cavity
                .get_edge(cavity.get_half_edge(he).unwrap().edge)
                .unwrap();
            let c = cavity.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let geom::Curve3::Spiric {
                major_radius: rr,
                minor_radius: r,
                offset,
                ..
            } = *c.carrier()
            else {
                continue;
            };
            let (t0, t1) = c.params();
            let inner = rr - r;
            let speed = r * inner / (inner * inner - offset * offset).sqrt();
            let center = c.carrier().eval(0.5 * (t0 + t1));
            let reach = speed * (t1 - t0).abs() * 0.5;
            let rv = Vec3::new(1.0, 0.0, 0.0);
            let d = (rv - normal * normal.dot(rv)).normalize();
            let perp = normal.cross(d);
            for sgn in [1.0, -1.0] {
                for k in [0.5, 3.0, 5.0, 20.0] {
                    for back in [0.3, 1.0] {
                        let q = center + perp * (sgn * (reach + k * eps())) - d * back;
                        let got = contfp(&cavity, fk, normal, q, band());
                        asked += 1;
                        assert!(
                            !matches!(got, Err(topo::ContainError::Escalated(_))),
                            "{fk:?}: {k}ε past the ball's reach, {back} m back, reads {got:?}"
                        );
                    }
                }
            }
        }
    }
    assert!(asked >= 16, "the cavity's spiric caps were asked");
}
