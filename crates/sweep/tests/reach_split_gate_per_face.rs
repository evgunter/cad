//! The split's operand gate is per face: a body carrying a face of a
//! kind the split has no arm for splits wherever the plane cannot reach
//! that face, and refuses naming it wherever the plane may.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::revolve_common::{axis_y, validated};
use geom::SurfaceKind;
use geom_core::{Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, SplitResult, split};
use topo::{Body, DATUM_UNIT_NORM, validate, validate_closed, validate_geometric};

fn revolved(chain: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    revolved_with(chain, Vec::new())
}

/// [`revolved`] with the profile's tangent joints declared.
fn revolved_with(chain: Vec<(Point2<f64>, f64)>, tangent_joints: Vec<usize>) -> Body<f64> {
    revolve(
        &validated(vec![bulge_loop(chain).with_tangent_joints(tangent_joints)]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A cut normal minted the way a caller holding a direction mints one.
fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance forms a band");
    UnitVec3::new(v, DATUM_UNIT_NORM, band).expect("a cut normal has a length")
}

/// The plane through `(0, qy, 0)` with normal `(sin φ, cos φ, 0)`.
fn plane(phi: f64, qy: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, qy, 0.0),
        normal: unit(Vec3::new(phi.sin(), phi.cos(), 0.0)),
    }
}

/// The unit cylinder `y ∈ [0, 1]` under a spherical cap: the arc
/// `(1, 1) → (0, 1.5)` about `(0, 0.25)` (radius 5/4), revolved about `y`.
fn capped_cylinder() -> Body<f64> {
    let (a, b) = (Point2::new(1.0, 1.0), Point2::new(0.0, 1.5));
    let bulge = bulge_from_center(a, b, Point2::new(0.0, 0.25), ArcSweep::Ccw);
    revolved(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (a, bulge),
        (b, 0.0),
    ])
}

/// The same sphere below `y = 1`, flat on top: the arc `(0, −1) → (1, 1)`
/// about `(0, 0.25)`. Its sphere face is the complement of
/// [`capped_cylinder`]'s cap.
fn truncated_ball() -> Body<f64> {
    let (a, b) = (Point2::new(0.0, -1.0), Point2::new(1.0, 1.0));
    let bulge = bulge_from_center(a, b, Point2::new(0.0, 0.25), ArcSweep::Ccw);
    revolved(vec![(a, bulge), (b, 0.0), (Point2::new(0.0, 1.0), 0.0)])
}

/// The cap's volume, `π·h²·(3R − h)/3` at `R = 5/4`, `h = 1/2`.
const CAP_VOLUME: f64 = PI * 0.25 * (3.75 - 0.5) / 3.0;

fn halves(result: &SplitResult<f64>, what: &str) -> (Body<f64>, Body<f64>) {
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("{what}: both sides carry material");
    };
    for part in [above, below] {
        assert_eq!(validate(part), Ok(()), "{what}: tier 1");
        assert_eq!(validate_closed(part), Ok(()), "{what}: tier 2");
        if let Err(errs) = validate_geometric(part, Tol::witness()) {
            panic!("{what}: tier 3: {errs:?}");
        }
    }
    (above.clone(), below.clone())
}

/// The unit cylinder `y ∈ [0, 1]` with its top rim rounded: the quarter
/// arc `(1, 1) → (3/4, 5/4)` about `(3/4, 1)`, a torus of radii 3/4 and
/// 1/4.
fn rounded_cylinder() -> Body<f64> {
    let (a, b) = (Point2::new(1.0, 1.0), Point2::new(0.75, 1.25));
    let bulge = bulge_from_center(a, b, Point2::new(0.75, 1.0), ArcSweep::Ccw);
    revolved_with(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (a, bulge),
            (b, 0.0),
            (Point2::new(0.0, 1.25), 0.0),
        ],
        vec![2, 3],
    )
}

/// The torus rounding's volume above `y = 1` by Pappus, every piece in
/// closed form: the disk `ρ ≤ 3/4` of height 1/4, and the quarter disk of
/// radius 1/4 whose centroid sits `4r/3π` beyond `ρ = 3/4`.
fn rounding_volume() -> f64 {
    let r: f64 = 0.25;
    let quarter = PI * r * r / 4.0;
    let centroid = 0.75 + 4.0 * r / (3.0 * PI);
    PI * 0.75 * 0.75 * r + 2.0 * PI * centroid * quarter
}

/// The loft prism's volume: the section is a trapezoid of height 2 and
/// parallel sides `2 + 2d(z)` and 2, `d(z) = (3/8)·z·(2 − z)` the
/// flare's quadratic through the three sections, so
/// `V = ∫₀² (4 + 2d) dz = 8 + (3/4)·(4/3) = 9`.
const LOFT_PRISM_VOLUME: f64 = 9.0;

fn props(b: &Body<f64>) -> topo::props::MassProperties<f64> {
    topo::props::mass_properties(b, Tol::witness()).unwrap()
}

/// `v` lies within the kernel's certified bracket of `oracle`.
fn assert_volume(b: &Body<f64>, oracle: f64, what: &str) {
    let p = props(b);
    assert!(
        (p.volume - oracle).abs() <= p.volume_pad + 1e-12,
        "{what}: {} ± {} against the oracle {oracle}",
        p.volume,
        p.volume_pad
    );
}

/// A cylinder under a sphere face or a torus face the plane cannot
/// reach splits: the plane through the axis at mid-height leaves the
/// half-cylinder `π/2` below whatever the tilt, and the cap or the
/// rounding whole above it. Both halves are valid at every tier.
#[test]
fn a_plane_clear_of_a_sphere_or_torus_face_splits_the_body() {
    for (name, body, top) in [
        ("sphere cap", capped_cylinder(), CAP_VOLUME),
        ("torus rounding", rounded_cylinder(), rounding_volume()),
    ] {
        assert_volume(&body, PI + top, &format!("{name}: the uncut body"));
        for phi in [0.0, 0.3] {
            let what = format!("{name}, phi {phi}");
            let result = split(&body, &plane(phi, 0.5), Tol::witness())
                .unwrap_or_else(|e| panic!("{what}: {e}"));
            let (above, below) = halves(&result, &what);
            assert_volume(&below, PI / 2.0, &format!("{what}: below"));
            assert_volume(&above, PI / 2.0 + top, &format!("{what}: above"));
            let (pa, pb) = (props(&above), props(&below));
            assert!(
                (pa.volume + pb.volume - (PI + top)).abs() <= pa.volume_pad + pb.volume_pad + 1e-12,
                "{what}: the halves sum to the body"
            );
        }
    }
}

/// The plane taking a small dome off the cap near its pole, on the
/// `−z` side: its normal is the cap's radial direction 15° from the pole
/// toward `−z`, at 1.23 from the sphere's centre, so it meets the cap in a
/// circle of angular radius `acos(1.23/1.25) ≈ 10°` that reaches neither
/// the pole, nor the rim, nor the seam meridians. It clears every point of
/// the body below the cap (the farthest, the rim, sits at 0.98).
fn dome_cut() -> SplitPlane<f64> {
    let n = Vec3::new(0.0, 15f64.to_radians().cos(), -(15f64.to_radians().sin()));
    SplitPlane {
        origin: Point3::new(0.0, 0.25, 0.0) + n * 1.23,
        normal: unit(n),
    }
}

/// A plane through the curved face refuses naming it: the gate is
/// scoped to the plane's reach, not lifted. The truncated ball's
/// sphere face shares its rim with the cap and lies on the other side
/// of it, so a plane through its interior refuses where the same plane
/// clears the cap. The level cut through the cap crosses its seam
/// meridian, so a later stage would refuse it too; the dome cut and the
/// cuts off the ball's flanks meet their faces in circles that cross no
/// edge, so no later stage sees the sphere at all and the gate is the
/// only refusal they have.
#[test]
fn a_plane_that_may_meet_the_face_refuses_naming_it() {
    let flank = |n: Vec3<f64>| SplitPlane {
        origin: Point3::new(0.0, 0.25, 0.0) + n * 1.1,
        normal: unit(n),
    };
    for (name, body, kind, cut) in [
        (
            "sphere cap",
            capped_cylinder(),
            SurfaceKind::Sphere,
            plane(0.3, 1.1),
        ),
        (
            "sphere cap, level",
            capped_cylinder(),
            SurfaceKind::Sphere,
            plane(0.0, 1.2),
        ),
        (
            "sphere cap, a dome off its -z side",
            capped_cylinder(),
            SurfaceKind::Sphere,
            dome_cut(),
        ),
        (
            "torus rounding",
            rounded_cylinder(),
            SurfaceKind::Torus,
            plane(0.3, 1.1),
        ),
        (
            "truncated ball",
            truncated_ball(),
            SurfaceKind::Sphere,
            plane(0.3, 0.5),
        ),
        (
            "truncated ball's -z flank",
            truncated_ball(),
            SurfaceKind::Sphere,
            flank(Vec3::new(0.0, 0.0, -1.0)),
        ),
        (
            "truncated ball's +z flank",
            truncated_ball(),
            SurfaceKind::Sphere,
            flank(Vec3::new(0.0, 0.0, 1.0)),
        ),
    ] {
        let err = split(&body, &cut, Tol::witness()).unwrap_err();
        let SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported { face, kind: k }) = &err
        else {
            panic!("{name}: expected the gate's refusal, got {err:?}");
        };
        assert_eq!(*k, kind, "{name}");
        let surface = body.get_face(*face).unwrap().surface;
        assert_eq!(body.get_surface(surface).unwrap().kind(), kind, "{name}");
    }
}

/// A plane that misses a spline-walled body — NURBS walls and NURBS
/// seam edges — returns the body whole on its side.
#[test]
fn a_plane_missing_a_spline_body_returns_it_whole() {
    let loft = sweep::test_support::loft_prism(Tol::witness());
    let over = SplitPlane {
        origin: Point3::new(0.0, 0.0, 3.0),
        normal: unit(Vec3::new(0.2f64.sin(), 0.0, 0.2f64.cos())),
    };
    let result = split(&loft, &over, Tol::witness()).unwrap_or_else(|e| panic!("{e}"));
    let (SplitPart::Empty, SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("the plane over the loft leaves it all below");
    };
    assert_eq!(validate(below), Ok(()), "tier 1");
    assert_eq!(validate_closed(below), Ok(()), "tier 2");
    if let Err(errs) = validate_geometric(below, Tol::witness()) {
        panic!("tier 3: {errs:?}");
    }
    assert_volume(below, LOFT_PRISM_VOLUME, "the loft");
}

/// The exact rational quadratic NURBS of the conic arc
/// `c + a·cos t·û + b·sin t·v̂`, `t ∈ [t0, t1]`, in two segments: each
/// half is the unit circle's arc of at most a quarter turn, mapped by
/// the conic's affine frame, which a rational curve's weights survive.
fn conic_arc_nurbs(
    c: Point3<f64>,
    (u, v): (Vec3<f64>, Vec3<f64>),
    (a, b): (f64, f64),
    (t0, t1): (f64, f64),
) -> geom::NurbsCurve3<f64> {
    let at = |x: f64, y: f64| c + u * (a * x) + v * (b * y);
    let half = (t1 - t0) / 2.0;
    let w = (half / 2.0).cos();
    let mut control = vec![at(t0.cos(), t0.sin())];
    let mut weights = vec![1.0];
    for k in 0..2 {
        let (s, e) = (t0 + half * f64::from(k), t0 + half * f64::from(k + 1));
        let m = (s + e) / 2.0;
        control.push(at(m.cos() / w, m.sin() / w));
        control.push(at(e.cos(), e.sin()));
        weights.extend([w, 1.0]);
    }
    let kv =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2)
            .unwrap();
    geom::NurbsCurve3::new(kv, control, weights).unwrap()
}

/// The unit cylinder cut by the plane `y = 0.5 + 0.3·z`, its lower half:
/// a planar elliptic top bounded by two half-ellipses that meet at the
/// seam points `(±1, 0.5, 0)`. The half on `z < 0` dips to `y = 0.2`
/// between them, and it is re-described as its own exact spline, an edge
/// between a plane face and a cylinder face, both armed.
fn cylinder_with_a_spline_rim() -> (Body<f64>, topo::EdgeKey) {
    let cyl = revolved(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let tilt = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: unit(Vec3::new(0.0, 1.0, -0.3)),
    };
    let SplitPart::Body(mut body) = split(&cyl, &tilt, Tol::witness()).unwrap().below else {
        panic!("the tilted cut leaves material below");
    };
    let start = |b: &Body<f64>, he| {
        *b.get_point(
            b.get_vertex(b.get_half_edge(he).unwrap().start)
                .unwrap()
                .point,
        )
        .unwrap()
    };
    let (edge, spec) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            let geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } = *c.carrier()
            else {
                return None;
            };
            let (t0, t1) = c.params();
            let mid = c.carrier().eval((t0 + t1) / 2.0);
            if mid.z > 0.0 {
                return None;
            }
            let spline =
                conic_arc_nurbs(center, (u_ref, axis.cross(u_ref)), (major, minor), (t0, t1));
            assert!(
                (spline.eval(0.0) - start(&body, e.he_plus)).norm() < 1e-12,
                "the spline runs the edge's way"
            );
            let (s1, s2) = topo::readback::edge_sides(&body, k).unwrap().surfaces();
            Some((
                k,
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        s1,
                        s2,
                        witness: mid,
                    },
                    carrier: geom::Curve3::Nurbs(std::sync::Arc::new(spline)),
                    param_start: 0.0,
                    param_end: 1.0,
                },
            ))
        })
        .expect("the tilted section has a half-ellipse on z < 0");
    body.set_edge_curve(edge, spec, Tol::witness())
        .expect("the exact spline attaches");
    (body, edge)
}

/// A spline edge between two armed faces, its ends on one side of the
/// plane and its belly across it, refuses at the gate. Nothing else
/// reads it: no crossing lane serves a spline, so passing it would
/// leave the edge uncut while the plane crosses the faces beside it.
#[test]
fn a_spline_edge_whose_belly_crosses_the_plane_refuses() {
    let (body, edge) = cylinder_with_a_spline_rim();
    for part in [validate(&body), validate_closed(&body)] {
        assert_eq!(part, Ok(()), "the re-described body is well formed");
    }
    match split(&body, &plane(0.0, 0.4), Tol::witness()) {
        Err(SplitError::Reduce(SplitReduceError::CurvedEdgeUnsupported { edge: e })) => {
            assert_eq!(e, edge, "the refusal names the spline edge");
        }
        other => panic!("the plane through the spline's belly refuses, got {other:?}"),
    }
}

/// A sphere face whose boundary bounds the outside of its latitude zone
/// keeps the whole ball. A rectangle's boundary also bounds its
/// complement, so the zone holds the face only when the side the
/// traversal encodes agrees with the sense bit.
///
/// The face is the sphere zone revolved through one radian, a chart
/// rectangle with no pole on its boundary. Reverting the body reverses
/// every loop and flips every bit; setting the zone's bit back leaves
/// its loop running the complement's way with its own sense, the class
/// of an imported face whose loop bounds the outside of its rectangle.
/// A level plane above the zone clears the zone's box and meets the
/// ball's, and nothing else in the body: the gate is the only refusal.
#[test]
fn a_sphere_face_whose_side_is_not_certified_keeps_the_ball() {
    let wedge = sweep::test_support::sphere_zone(0.5, Revolution::Partial(1.0), Tol::witness());
    let above = plane(0.0, 1.5);
    let whole = split(&wedge, &above, Tol::witness()).expect("the zone's box clears the cut");
    assert!(
        matches!(
            (&whole.above, &whole.below),
            (SplitPart::Empty, SplitPart::Body(_))
        ),
        "the wedge lies below the cut"
    );
    let mut reverted = wedge.revert().expect("the wedge reverts");
    let (zone, surface, sense) = reverted
        .faces()
        .find_map(|(k, f)| {
            let s = reverted.get_surface(f.surface)?;
            matches!(s, geom::Surface::Sphere { .. }).then(|| (k, s.clone(), f.sense))
        })
        .expect("the wedge has a sphere face");
    reverted
        .set_face_surface_stranding_for_tests(
            zone,
            topo::FaceSurface::New {
                surface,
                sense: !sense,
            },
        )
        .unwrap();
    match split(&reverted, &above, Tol::witness()) {
        Err(SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported {
            face,
            kind: SurfaceKind::Sphere,
        })) => assert_eq!(face, zone, "the refusal names the zone"),
        other => panic!("an uncertified side keeps the ball, got {other:?}"),
    }
}

