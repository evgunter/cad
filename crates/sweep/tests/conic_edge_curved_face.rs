//! **An ellipse rim against a curved face**: the exact tilted cut of a
//! drum (a radius-0.5 cylinder of height 1, split by the plane through
//! `(0, 0, 0.5)` with normal `(sin 0.3, 0, cos 0.3)`), its lower part,
//! against a ball or a rod. The cut face's rim is two `Ellipse` arcs, and
//! wherever their box meets a sphere or a cylinder face of the other
//! operand the crossing layer must decide them: cleared when they keep
//! away, crossed at certified roots when they do not.
//!
//! Every built body is held to all three validation tiers, a closed
//! tessellation, and its volume against a closed form computed here from
//! the radii alone. The drum's lower part is `π r² · ½` exactly (the
//! plane passes through the axis at mid-height, so it trades equal
//! wedges); a ball or rod held inside it is the whole of `∩`, absent
//! from `∪`, a void of `A ∖ B`, and leaves `B ∖ A` empty.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp, EdgeKey, FaceKey, SweepStrategy, sweep_traces};

const DRUM_RADIUS: f64 = 0.5;
const TILT: f64 = 0.3;

/// The drum's lower part: its rim is the cut face's two `Ellipse` arcs.
fn drum_lower() -> Body<f64> {
    let tol = Tol::witness();
    let r = DRUM_RADIUS;
    let cylinder = sweep::test_support::prism(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        1.0,
        tol,
    );
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(TILT.sin(), 0.0, TILT.cos()),
    };
    let result = topo::splitting::split(&cylinder, &plane, tol).expect("the tilted cut splits");
    let topo::splitting::SplitPart::Body(below) = result.below else {
        panic!("the cut leaves material below");
    };
    assert_eq!(
        ellipse_edges(&below).len(),
        2,
        "the rim is two ellipse arcs"
    );
    below
}

fn drum_volume() -> f64 {
    PI * DRUM_RADIUS.powi(2) * 0.5
}

/// The height of the cut plane above `(x, ·)`.
fn cut_height(x: f64) -> f64 {
    0.5 - x * TILT.tan()
}

/// A ball of radius `r` about `c` (a full revolve about `y`, moved).
fn ball(r: f64, c: [f64; 3]) -> Body<f64> {
    let at_origin = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    let to = Affine3::translation(Vec3::new(c[0], c[1], c[2]));
    topo::transform_rigid(&at_origin, &to, Tol::witness()).expect("a translation is rigid")
}

/// [`ball`] with its revolve axis turned onto the cut plane's normal, so
/// the plane's section of it is a latitude circle of its own chart.
fn polar_ball(r: f64, c: [f64; 3]) -> Body<f64> {
    let k = Vec3::new(TILT.cos(), 0.0, -TILT.sin());
    let turn = Affine3::rotation_about_axis(Point3::from_array(c), k, core::f64::consts::FRAC_PI_2);
    topo::transform_rigid(&ball(r, c), &turn, Tol::witness()).expect("a rotation is rigid")
}

/// A vertical rod of radius `r` about `(x, y)` over `z ∈ [z0, z0 + h]`.
fn rod(r: f64, x: f64, y: f64, z0: f64, h: f64) -> Body<f64> {
    sweep::test_support::prism_at(
        vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
        z0,
        h,
        Tol::witness(),
    )
}

fn ellipse_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Ellipse { .. }))
        })
        .map(|(k, _)| k)
        .collect()
}

fn faces_where(body: &Body<f64>, kind: fn(&geom::Surface<f64>) -> bool) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| body.get_surface(f.surface).is_some_and(kind))
        .map(|(k, _)| k)
        .collect()
}

fn curved(s: &geom::Surface<f64>) -> bool {
    matches!(
        s,
        geom::Surface::Sphere { .. } | geom::Surface::Cylinder { .. }
    )
}

/// The rim's pairs with `b`'s sphere and cylinder faces in the A → B
/// sweep: `(examined, accepted)`. On the base every examined pair refused
/// `CurvedPierceUnsupported`, so the sweep itself refused.
fn rim_pairs(label: &str, a: &Body<f64>, b: &Body<f64>) -> (usize, usize) {
    let (ab, _) = sweep_traces(a, b, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: the reduction sweep refused: {e:?}"));
    let (rim, walls) = (ellipse_edges(a), faces_where(b, curved));
    let count = |v: &[(EdgeKey, FaceKey)]| {
        v.iter()
            .filter(|(e, f)| rim.contains(e) && walls.contains(f))
            .count()
    };
    (count(&ab.examined), count(&ab.accepted))
}

fn run(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// Every tier of validation, a closed tessellation, then the volume
/// against `expected` through the kernel's mass properties. The cut wall
/// has no closed form, so its flux is a certified quadrature and the
/// slack is the enclosure's own half-width `volume_pad`, plus rounding.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: validate_geometric"
    );
    let m = mesh::tessellate(body, 1e-3, tol)
        .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&m),
        Ok(()),
        "{label}: a closed manifold mesh"
    );
    let p = topo::mass_properties(body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
    assert!(
        (p.volume - expected).abs() <= p.volume_pad + 1e-12,
        "{label}: volume {} ± {} against the closed form {expected}",
        p.volume,
        p.volume_pad
    );
}

/// A solid `b` of volume `vb` held strictly inside the drum, under ∪,
/// ∩ and both differences, each against its closed form.
fn assert_held_inside(label: &str, a: &Body<f64>, b: &Body<f64>, vb: f64) {
    let va = drum_volume();
    for (op_label, op, x, y, expected) in [
        ("A ∪ B", BooleanOp::Union, a, b, Some(va)),
        ("A ∩ B", BooleanOp::Intersect, a, b, Some(vb)),
        ("A ∖ B", BooleanOp::Subtract, a, b, Some(va - vb)),
        ("B ∖ A", BooleanOp::Subtract, b, a, None),
    ] {
        let label = format!("{label}, {op_label}");
        let out = run(op, x, y).unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
        match (out.body(), expected) {
            (Some(body), Some(v)) => assert_body(&label, &body.body, v),
            (None, None) => {}
            (got, want) => panic!(
                "{label}: a body {} where the closed form says {want:?}",
                if got.is_some() {
                    "came back"
                } else {
                    "is missing"
                }
            ),
        }
    }
}

/// **A ball held inside the drum, within reach of the rim's box.** Each
/// ball lies strictly inside the lower part (clear of the wall, the floor
/// and the cut plane), and its box meets the rim's, so the rim is
/// examined against the sphere and cleared by its enclosure. On the base
/// every pose refused `CurvedPierceUnsupported` on the rim, whose carrier
/// took the frontier door before any clearance test.
#[test]
fn a_ball_held_inside_the_drum_clears_the_rim() {
    let a = drum_lower();
    for (r, c) in [
        (0.18_f64, [0.0_f64, 0.0, 0.3]),
        (0.15, [0.3, 0.0, 0.2]),
        (0.12, [-0.3, 0.1, 0.4]),
    ] {
        let label = format!("ball r {r} at {c:?}");
        // The pose's own premises, from the geometry: inside the wall,
        // above the floor, below the cut plane by more than the radius.
        let reach = c[0].hypot(c[1]) + r;
        assert!(reach < DRUM_RADIUS, "{label}: inside the wall ({reach})");
        assert!(c[2] > r, "{label}: above the floor");
        let below_cut = (cut_height(c[0]) - c[2]) * TILT.cos();
        assert!(below_cut > r, "{label}: below the cut ({below_cut})");
        let b = ball(r, c);
        let (examined, accepted) = rim_pairs(&label, &a, &b);
        assert!(
            examined > 0,
            "{label}: the rim is examined against the sphere"
        );
        assert_eq!(accepted, 0, "{label}: and cleared");
        assert_held_inside(&label, &a, &b, 4.0 / 3.0 * PI * r.powi(3));
    }
}

/// **A rod held inside the drum, its top just under the cut**: the rim
/// against the rod's wall, cleared by the ellipse × cylinder enclosure.
/// The first pose's wall stands 0.05 inside the drum's and its top cap
/// within 6 mm of the cut plane. On the base each pose refused
/// `CurvedPierceUnsupported` on the rim.
#[test]
fn a_rod_held_inside_the_drum_clears_the_rim() {
    let a = drum_lower();
    for (r, x, y, z0, h) in [
        (0.45_f64, 0.0_f64, 0.0, 0.1, 0.255),
        (0.3, 0.15, 0.05, 0.05, 0.3),
        (0.2, 0.0, 0.0, 0.2, 0.2),
    ] {
        let label = format!("rod r {r} at ({x}, {y}), z ∈ [{z0}, {}]", z0 + h);
        assert!(x.hypot(y) + r < DRUM_RADIUS, "{label}: inside the wall");
        // The cut is lowest over the rod's disc at its far +x edge.
        assert!(cut_height(x + r) > z0 + h, "{label}: under the cut");
        let b = rod(r, x, y, z0, h);
        let (examined, accepted) = rim_pairs(&label, &a, &b);
        assert!(
            examined > 0,
            "{label}: the rim is examined against the rod wall"
        );
        assert_eq!(accepted, 0, "{label}: and cleared");
        assert_held_inside(&label, &a, &b, PI * r.powi(2) * h);
    }
}

/// The boolean's refusal under every op; a body fails with the op named.
fn refusals(a: &Body<f64>, b: &Body<f64>) -> Vec<topo::BooleanError> {
    [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract]
        .into_iter()
        .map(|op| {
            run(op, a, b)
                .err()
                .unwrap_or_else(|| panic!("{op:?} built a body where a frontier was pinned"))
        })
        .collect()
}

/// **The rim CROSSES**: balls straddling it, and rods standing across
/// it. The rim's pairs are accepted (its certified roots split it), the
/// crossing layer and the sector side pass, and every op stops at the
/// join, at the door that pose's germ pairs reach: a ball's wall ×
/// sphere pair has no section frame
/// (`work/join/cylinder-sphere-germ-pair-has-no-section-frame.md`), a
/// wide rod's parallel wall pair no join arm
/// (`work/join/parallel-cylinder-germ-pair-has-no-join-arm.md`), and a
/// narrow rod's pierce ring in the drum's wall no join arm either
/// (`work/tang/pierce-ring-has-no-join-arm.md`). On the base the balls
/// refused `CurvedPierceUnsupported` on the rim, and the rods on their
/// own rim circle, whose root on the drum wall the wall's chart trim
/// (bounded by the rim's arcs) could not place.
#[test]
fn a_rim_crossing_reaches_the_join() {
    use topo::BooleanError as E;
    let a = drum_lower();
    let no_frame = |e: &E| matches!(e, E::GermFrameUnsupported { .. });
    let no_arm = |e: &E| {
        matches!(
            e,
            E::CurvedBooleanUnsupported {
                kind: geom::SurfaceKind::Cylinder,
                ..
            }
        )
    };
    let ring = |e: &E| {
        matches!(
            e,
            E::Join(topo::SplitJoinError::SectionArcWindow {
                case: topo::ArcWindowCase::NoChartedRun,
                ..
            })
        )
    };
    type Door<'a> = &'a dyn Fn(&E) -> bool;
    let poses: [(&str, Body<f64>, Door); 5] = [
        (
            "ball r 0.2 at (0.5, 0, 0.35)",
            ball(0.2, [0.5, 0.0, 0.35]),
            &no_frame,
        ),
        (
            "ball r 0.1 at (0.45, 0, 0.3)",
            ball(0.1, [0.45, 0.0, 0.3]),
            &no_frame,
        ),
        (
            "rod r 0.2 at (0.5, 0)",
            rod(0.2, 0.5, 0.0, 0.2, 0.25),
            &no_arm,
        ),
        (
            "rod r 0.1 at (-0.45, 0)",
            rod(0.1, -0.45, 0.0, 0.5, 0.3),
            &ring,
        ),
        (
            "rod r 0.1 at (0, 0.48)",
            rod(0.1, 0.0, 0.48, 0.3, 0.4),
            &ring,
        ),
    ];
    for (label, b, at_the_door) in poses {
        let (_, accepted) = rim_pairs(label, &a, &b);
        assert!(accepted > 0, "{label}: the rim's crossings are accepted");
        for e in refusals(&a, &b) {
            assert!(at_the_door(&e), "{label}: got {e:?}");
        }
    }
}

/// **A ball poking through the cut face, clear of the rim** — the row
/// whose ball never comes within 0.2 of the rim. The rim clears, and the
/// next door depends on the ball's chart: charted about `y`, the cut
/// plane's section of it is not a latitude circle, and the join refuses
/// `SectionNotPolar`
/// (`work/reach/tilted-sphere-pair-section-refuses-at-the-polar-gate.md`);
/// charted about the cut's normal, the join passes and the
/// classification's at-infinity probe cannot measure the cut wall in
/// closed form
/// (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`).
#[test]
fn a_ball_through_the_cut_face_clears_the_rim_and_stops_downstream() {
    let a = drum_lower();
    let c = [0.0, 0.0, 0.5];
    for (label, b, polar) in [
        ("ball charted about y", ball(0.3, c), false),
        (
            "ball charted about the cut normal",
            polar_ball(0.3, c),
            true,
        ),
    ] {
        let (examined, accepted) = rim_pairs(label, &a, &b);
        assert!(examined > 0, "{label}: the rim is examined");
        assert_eq!(accepted, 0, "{label}: and cleared");
        for e in refusals(&a, &b) {
            let at_the_door = if polar {
                matches!(
                    e,
                    topo::BooleanError::Containment(topo::PointInSolidError::VolumeUncertified)
                )
            } else {
                matches!(
                    e,
                    topo::BooleanError::Join(topo::SplitJoinError::SectionNotPolar { .. })
                )
            };
            assert!(at_the_door, "{label}: got {e:?}");
        }
    }
}

/// **The cut wall's trim places a point on it**: the drum's two wall
/// faces are each bounded by a floor arc, two rulings and one rim arc,
/// and a point on the wall's carrier lies in exactly one of them when it
/// is above the floor and below the cut, in neither otherwise. That is
/// what a crossing landing on the wall asks of it (the rods' rim circles
/// above). On the base the face door had no verdict for a wall bounded
/// by a planar section and answered `None` for every point.
#[test]
fn the_cut_wall_places_points_against_the_rim() {
    let a = drum_lower();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let walls = faces_where(&a, |s| matches!(s, geom::Surface::Cylinder { .. }));
    assert_eq!(walls.len(), 2, "two wall faces");
    let mut placed = [0usize; 2];
    for k in 0..24 {
        // Off the seams at azimuth 0 and π.
        let az = (f64::from(k) + 0.5) * core::f64::consts::TAU / 24.0;
        let (x, y) = (DRUM_RADIUS * az.cos(), DRUM_RADIUS * az.sin());
        for z in [0.02, 0.2, 0.33, 0.4, 0.5, 0.6, 0.7] {
            let cut = cut_height(x);
            if (z - cut).abs() < 0.01 {
                continue;
            }
            let inside = z < cut;
            let q = Point3::new(x, y, z);
            let mut ins = 0;
            for &f in &walls {
                match topo::curved_face_containment(&a, f, q, band) {
                    Ok(Some(topo::FaceContainment::In)) => ins += 1,
                    Ok(Some(topo::FaceContainment::Out)) => {}
                    other => panic!("({x}, {y}, {z}) on {f:?}: a verdict, got {other:?}"),
                }
            }
            assert_eq!(
                ins,
                usize::from(inside),
                "({x}, {y}, {z}): the cut is at {cut}"
            );
            placed[usize::from(inside)] += 1;
        }
    }
    assert!(
        placed[0] > 20 && placed[1] > 20,
        "points on both sides of the rim: {placed:?}"
    );
}
