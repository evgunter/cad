//! **A spline edge at the sweep's curved arm** ([`super::spline_wall_event`]):
//! the curve × face table's ring-composite arm read against a cylinder,
//! a sphere, a cone and a torus face.
//!
//! The operand `A` is a planar sheet bounded by one quadratic NURBS edge
//! and the chord back ([`spline_sheet`]); `B` is one wall face. `A`'s
//! direction of the sweep runs directly (`planar_lane_carrier_rows`), so
//! the rows read what the curved arm does with the spline: where it
//! split the edge, which contact it recorded, or what it refused.
//!
//! The oracles are closed forms, never the kernel's evaluator: each net's
//! Bernstein form, and each surface's distance in its own terms, with a
//! crossing located by bisecting that closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::planar_lane_carrier_rows::{on_face, sweep_a};
use crate::boolean::{BooleanError, CoverSide, GluedPairs, Operand};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, fresh_keyed_body, set_sheet_sense};
use crate::{Body, EdgeKey, FaceKey};
use geom::Curve3;
use geom_brep::roots::Unsettled;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point3, Sign, Tol, Vec3};

/// A planar sheet bounded by the quadratic NURBS edge on `control`
/// (weights `weights`) and the chord back from its last point to its
/// first. `normal` is the sheet's plane normal, chosen so the net bulges
/// to `normal × (last − first)`'s side, the side the chord leaves the
/// face on. The sheet, its face, and its spline edge.
fn spline_sheet(
    control: [Point3<f64>; 3],
    weights: [f64; 3],
    normal: Vec3<f64>,
) -> (Body<f64>, FaceKey, EdgeKey) {
    let tol = Tol::witness();
    let [p0, _, p1] = control;
    let u_ref = (p1 - p0) / (p1 - p0).norm();
    let v_ref = normal.cross(u_ref);
    let kv =
        || geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let net = geom::NurbsCurve3::new(kv(), control.to_vec(), weights.to_vec()).unwrap();
    let image = geom::NurbsCurve2::new(
        kv(),
        control
            .iter()
            .map(|&p| geom_core::Point2::new((p - p0).dot(u_ref), (p - p0).dot(v_ref)))
            .collect(),
        weights.to_vec(),
    )
    .unwrap();
    let mut body = fresh_keyed_body::<f64>();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Plane {
                    origin: p0,
                    normal,
                    u_ref,
                },
                sense: true,
            },
        )
        .unwrap();
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Chart {
                    surface: plane,
                    image: Some(geom_brep::Pcurve::Fitted(std::sync::Arc::new(image))),
                    wrap: false,
                    declared: None,
                },
                carrier: Curve3::Nurbs(std::sync::Arc::new(net)),
                param_start: 0.0,
                param_end: 1.0,
            },
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(plane),
            carrier: Curve3::Line {
                origin: p1,
                dir: -u_ref,
            },
            param_start: 0.0,
            param_end: (p1 - p0).norm(),
        },
        FaceSurface::Shared {
            key: plane,
            sense: true,
        },
        tol,
    )
    .unwrap();
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    let face = body.faces().map(|(k, _)| k).next().unwrap();
    (body, face, e.edge)
}

/// The quadratic Bernstein form of a non-rational net at `t`.
fn bernstein(control: [Point3<f64>; 3], t: f64) -> Point3<f64> {
    let [a, b, c] = control.map(|p| p - Point3::origin());
    Point3::origin() + a * (1.0 - t).powi(2) + b * (2.0 * t * (1.0 - t)) + c * (t * t)
}

/// The root of `f` in `[lo, hi]`, where `f` changes sign, by bisection.
fn root(f: impl Fn(f64) -> f64, (mut lo, mut hi): (f64, f64)) -> f64 {
    assert!(f(lo) * f(hi) < 0.0, "the oracle brackets a root");
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(lo) * f(mid) <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

/// A wall of the cylinder of `radius` about `z` through the origin over
/// azimuths `seam + [0, width]` and heights `zs`, and its face, the
/// patch; its lamina twin is the rest of the cylinder.
fn wall(radius: f64, seam: f64, width: f64, zs: (f64, f64)) -> (Body<f64>, FaceKey) {
    let mut b = Body::<f64>::new();
    let frame = CylFrame {
        u_ref: Vec3::new(seam.cos(), seam.sin(), 0.0),
        ..CylFrame::canonical(radius)
    };
    let face = cyl_wall_sheet(&mut b, frame, (0.0, width), zs, Tol::witness());
    set_sheet_sense(&mut b, face, false, Tol::witness());
    (b, face)
}

/// Whether `got` holds a point within `1e-9` of each of `want`, and no
/// other.
fn lands_at(got: &[Point3<f64>], want: &[Point3<f64>]) -> bool {
    let near = |p: Point3<f64>, q: Point3<f64>| (p - q).norm() < 1e-9;
    got.iter().all(|&p| want.iter().any(|&q| near(p, q)))
        && want.iter().all(|&q| got.iter().any(|&p| near(p, q)))
}

/// The arc from the origin through `(1, 2, 0)` to `(2, 0, 0)`:
/// `(2t, 4t(1 − t), 0)`.
const ARC: [Point3<f64>; 3] = [
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(1.0, 2.0, 0.0),
    Point3::new(2.0, 0.0, 0.0),
];

/// Where [`ARC`] leaves the cylinder or sphere of `radius` about the
/// origin: its distance from the `z` axis is `√(4t² + 16t²(1 − t)²)`,
/// rising from 0 past 1.
fn arc_leaves(radius: f64) -> Point3<f64> {
    let rho2 = |t: f64| 4.0 * t * t + 16.0 * (t * (1.0 - t)).powi(2) - radius * radius;
    bernstein(ARC, root(rho2, (0.0, 0.5)))
}

/// **A spline leaving a cylinder wall is split where it crosses it**,
/// and the point lands in the wall: at azimuth `0.96`, inside its window
/// `[0.5, 1.5]`.
#[test]
fn a_spline_leaving_a_cylinder_wall_is_split_where_it_crosses() {
    let at = arc_leaves(1.0);
    let (a, _, _) = spline_sheet(ARC, [1.0; 3], Vec3::unit_z());
    let (b, face) = wall(1.0, 0.5, 1.0, (-1.0, 1.0));
    let got = on_face(sweep_a(&a, &b), face);
    assert!(lands_at(&got, &[at]), "on the wall at {at:?}: {got:?}");
}

/// **A crossing outside the wall's trim is a sibling face's.** The same
/// spline against the window `[1.2, 1.9]`, which the spline's box meets
/// but its crossing, at azimuth `0.96`, is outside: no event, and no
/// refusal.
#[test]
fn a_crossing_outside_the_walls_trim_is_no_event_here() {
    let at = arc_leaves(1.0);
    let azimuth = at.y.atan2(at.x);
    assert!(
        !(1.2..=1.9).contains(&azimuth),
        "the oracle crossing is off the window: {azimuth}"
    );
    let (a, _, _) = spline_sheet(ARC, [1.0; 3], Vec3::unit_z());
    let (b, face) = wall(1.0, 1.2, 0.7, (-1.0, 1.0));
    let got = on_face(sweep_a(&a, &b), face);
    assert!(got.is_empty(), "nothing on the wall: {got:?}");
}

/// **A spline leaving a sphere's zone is split where it crosses it.**
#[test]
fn a_spline_leaving_a_sphere_zone_is_split_where_it_crosses() {
    let at = arc_leaves(1.0);
    let (a, _, _) = spline_sheet(ARC, [1.0; 3], Vec3::unit_z());
    let (b, face) = super::super::boxes::sphere_rect_rows::sphere_zone_sheet(
        (0.5, 1.5),
        (-0.5, 0.5),
        Tol::witness(),
    );
    let got = on_face(sweep_a(&a, &b), face);
    assert!(lands_at(&got, &[at]), "on the zone at {at:?}: {got:?}");
}

/// **An end of the spline on the wall is that end's incidence.** The
/// arc ends at `(2, 0, 0)` on the cylinder of radius 2, inside the window
/// about azimuth `0`, and runs inside it before: the arm reads the end
/// through the vertex door and records it, rather than refuse.
#[test]
fn a_spline_ending_on_a_wall_records_its_end() {
    let rho2 = |t: f64| 4.0 * t * t + 16.0 * (t * (1.0 - t)).powi(2);
    assert!(
        (1..100).all(|i| rho2(f64::from(i) / 100.0) < 4.0),
        "the oracle runs inside the wall before its end"
    );
    let (a, _, _) = spline_sheet(ARC, [1.0; 3], Vec3::unit_z());
    let (b, face) = wall(2.0, -0.5, 1.0, (-1.0, 1.0));
    let got = on_face(sweep_a(&a, &b), face);
    assert!(
        lands_at(&got, &[Point3::new(2.0, 0.0, 0.0)]),
        "the end on the wall: {got:?}"
    );
}

/// **The first crossing the torus face holds is the pierce; one outside
/// its trim is a sibling face's.** The spline `(1 + 2t, 1.2t(1 − t), 0)`
/// runs from the torus's hole out past it, crossing the tube (`R = 2`,
/// `r = ½`) where its distance from the axis is `1.5` and `2.5`. The
/// face is the tube's outer part about azimuth `0`, `|v| ≤ 1.2`, so the
/// first crossing, on the inner equator, is outside it, and the second
/// lands in it.
#[test]
fn a_spline_through_a_torus_tube_pierces_its_outer_face() {
    let control = [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(2.0, 0.6, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    ];
    let rho2 = |t: f64| (1.0 + 2.0 * t).powi(2) + (1.2 * t * (1.0 - t)).powi(2);
    let at = bernstein(control, root(|t| rho2(t) - 6.25, (0.5, 1.0)));
    let (a, _, _) = spline_sheet(control, [1.0; 3], Vec3::unit_z());
    let seam = -0.5f64;
    let (mut b, face) = super::super::boxes::tests::torus_wall(
        Point3::origin(),
        Vec3::unit_z(),
        Vec3::new(seam.cos(), seam.sin(), 0.0),
        2.0,
        0.5,
        (0.0, 1.0),
        (-1.2, 1.2),
    );
    twin_on_the_carrier(&mut b, face);
    // The chord back crosses the outer equator at azimuth 0, in the face.
    let chord = Point3::new(2.5, 0.0, 0.0);
    let got = on_face(sweep_a(&a, &b), face);
    assert!(
        lands_at(&got, &[at, chord]),
        "on the outer face at {at:?} and {chord:?}: {got:?}"
    );
}

/// The spline in the plane `x = 0` from `(0, 0, z0)` up through
/// `(0, 1, z0 + 2)` to `(0, 2, z0)`: `(0, 2t, z0 + 4t(1 − t))`.
fn rising(z0: f64) -> [Point3<f64>; 3] {
    [
        Point3::new(0.0, 0.0, z0),
        Point3::new(0.0, 1.0, z0 + 2.0),
        Point3::new(0.0, 2.0, z0),
    ]
}

/// The upper nappe's wall of the right cone about `z` with its apex at
/// the origin, over azimuths `[1, 2]` and heights `[0.1, 2]`. The
/// fixture's face, its sense `true`, is the rest of the cone, and its
/// twin the patch: the body and the patch.
fn upper_cone() -> (Body<f64>, FaceKey) {
    let alpha = core::f64::consts::FRAC_PI_4;
    let (mut b, rest) = super::super::boxes::tests::cone_wall(alpha, 1.0, 2.0, 0.1, 2.0);
    let patch = twin_on_the_carrier(&mut b, rest);
    (b, patch)
}

/// Puts `face`'s lamina twin, which the wall fixtures leave on the
/// seed's placeholder, on `face`'s carrier under sense `true`, and
/// returns it.
fn twin_on_the_carrier(b: &mut Body<f64>, face: FaceKey) -> FaceKey {
    let carrier = b
        .get_surface(b.get_face(face).unwrap().surface)
        .unwrap()
        .clone();
    let twin = b.faces().map(|(k, _)| k).find(|&k| k != face).unwrap();
    b.lifting_rechart_refusals_for_tests(|b| {
        b.set_face_surfaces_describing(
            vec![crate::construction::Rechart::new(carrier, twin, true)],
            &[],
            Tol::witness(),
        )
    })
    .unwrap();
    twin
}

/// **A crossing on the cone face's own nappe pierces it.** [`rising`]
/// from `z0 = 0.2` starts on the axis inside the upper nappe and leaves
/// it where its distance from the axis, `2t`, meets its height.
#[test]
fn a_spline_leaving_a_cone_on_its_faces_nappe_pierces_it() {
    let control = rising(0.2);
    let t = root(|t| 2.0 * t - (0.2 + 4.0 * t * (1.0 - t)), (0.0, 1.0));
    let at = bernstein(control, t);
    assert!(
        at.z > 0.1 && at.z < 2.0,
        "the oracle lands in the window: {at:?}"
    );
    let (a, _, _) = spline_sheet(control, [1.0; 3], Vec3::unit_x());
    let (b, face) = upper_cone();
    // The chord back, at height 0.2, leaves the cone where `y = 0.2`.
    let chord = Point3::new(0.0, 0.2, 0.2);
    let got = on_face(sweep_a(&a, &b), face);
    assert!(
        lands_at(&got, &[at, chord]),
        "on the cone at {at:?} and {chord:?}: {got:?}"
    );
}

/// **A crossing on the far nappe is not the face's.** [`rising`] from
/// `z0 = −½` starts on the axis inside the lower nappe and crosses it
/// once, at `2t = ½ − 4t(1 − t)`, below the apex; it then stays outside
/// the double cone, its box reaching the upper face's. The crossing is
/// the double cone's, told off the face's nappe: no event, no refusal.
#[test]
fn a_spline_crossing_the_far_nappe_is_no_event_on_the_cone_face() {
    let control = rising(-0.5);
    let t = root(|t| 2.0 * t + (-0.5 + 4.0 * t * (1.0 - t)), (0.0, 0.25));
    assert!(
        bernstein(control, t).z < 0.0,
        "the oracle crosses below the apex"
    );
    assert!(
        (1..100)
            .map(|i| bernstein(control, f64::from(i) / 100.0))
            .filter(|p| p.y > 2.0 * t)
            .all(|p| p.y > p.z.abs()),
        "the oracle stays outside the double cone after it"
    );
    let (a, _, _) = spline_sheet(control, [1.0; 3], Vec3::unit_x());
    let (b, face) = upper_cone();
    let got = on_face(sweep_a(&a, &b), face);
    assert!(got.is_empty(), "nothing on the cone: {got:?}");
}

/// **A spline lying on the wall refuses as a tangency.** The rational
/// quarter circle from `(1, 0, 0)` to `(0, 1, 0)` lies on the unit
/// cylinder: the composite is zero along it, and the face holds it.
#[test]
fn a_spline_lying_on_a_wall_refuses_as_a_tangency() {
    let control = [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let (a, _, edge) = spline_sheet(
        control,
        [1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
        -Vec3::unit_z(),
    );
    let (b, _) = wall(1.0, -0.3, 2.2, (-1.0, 1.0));
    match sweep_a(&a, &b) {
        Err(BooleanError::SplineCrossingUnsettled {
            operand: Operand::A,
            edge: e,
            face_of: Operand::B,
            why: Unsettled::Tangency,
            ..
        }) => assert_eq!(e, edge, "the refusal names the spline"),
        other => panic!("the spline lies on the wall: {other:?}"),
    }
}

/// The spline in the plane `y = 1`, tangent to the unit cylinder about
/// `z` along its ruling at azimuth `π/2`: `(t − ½, 1, 2t(1 − t) − ½)`
/// from `(−½, 1, −½)` up through `(0, 1, ½)` to `(½, 1, −½)`. It meets
/// the ruling at `t = ½`, at `(0, 1, 0)`, touching the cylinder there
/// from outside. The chord back runs at `z = −½`, below the wall.
const TANGENT: [Point3<f64>; 3] = [
    Point3::new(-0.5, 1.0, -0.5),
    Point3::new(0.0, 1.0, 0.5),
    Point3::new(0.5, 1.0, -0.5),
];

/// The unit wall about azimuth `π/2` over heights `[−0.3, 1]`, clear of
/// the tangent sheet's chord.
fn tangent_wall() -> (Body<f64>, FaceKey) {
    wall(1.0, 1.0, 1.2, (-0.3, 1.0))
}

/// **A spline touching the wall inside the face escalates** when no
/// declaration covers it: the arm reads the touch in the band's gap, where
/// it cannot tell it from two crossings, and the face holds it.
#[test]
fn an_uncovered_spline_touching_a_wall_escalates() {
    assert!(
        bernstein(TANGENT, 0.5).z.abs() < 1e-15,
        "the oracle touches at (0, 1, 0)"
    );
    let (a, _, _) = spline_sheet(TANGENT, [1.0; 3], -Vec3::unit_y());
    let (b, _) = tangent_wall();
    match sweep_a(&a, &b) {
        Err(BooleanError::Escalated { diag, .. }) => assert_eq!(
            diag.predicate,
            Some("spline_composite_past_the_band"),
            "the composite's reading escalates"
        ),
        other => panic!("the spline touches the wall: {other:?}"),
    }
}

/// **A touch of the wall outside the face is no event there.** The
/// spline `(t − ½, 1, …)` in the tangent plane `y = 1`, from
/// `(−½, 1, −½)` through `(0, 1, −0.4)` to `(½, 1, ½)`, meets the ruling
/// `x = 0` at height `−0.2`, below the wall's window `[0.1, 1]`, which its
/// box reaches. The composite reads the touch in the band's gap;
/// localized, it is certified off the face. The chord back touches
/// the ruling at height 0, below the window too.
#[test]
fn a_spline_touching_a_wall_below_the_face_is_no_event() {
    let control = [
        Point3::new(-0.5, 1.0, -0.5),
        Point3::new(0.0, 1.0, -0.4),
        Point3::new(0.5, 1.0, 0.5),
    ];
    assert!(
        (bernstein(control, 0.5) - Point3::new(0.0, 1.0, -0.2)).norm() < 1e-15,
        "the oracle touches the ruling at height −0.2"
    );
    let (a, _, _) = spline_sheet(control, [1.0; 3], Vec3::unit_y());
    let (b, face) = wall(1.0, 1.0, 1.2, (0.1, 1.0));
    let got = on_face(sweep_a(&a, &b), face);
    assert!(got.is_empty(), "nothing on the wall: {got:?}");
}

/// **Under the one-sided cover the same touch is deferred to the
/// fragments.** The sheet's plane `y = 1` lies outside the cylinder,
/// which the cover certifies: the spline, with both ends off the wall on
/// that side, touches it, if at all, strictly inside, and the sweep goes
/// on.
#[test]
fn a_covered_spline_touching_a_wall_is_deferred() {
    let (a, sheet, _) = spline_sheet(TANGENT, [1.0; 3], -Vec3::unit_y());
    let (b, face) = tangent_wall();
    let mut declared = GluedPairs::<f64>::default();
    for parent in a.faces().map(|(k, _)| k) {
        for target in b.faces().map(|(k, _)| k) {
            declared.one_sided.insert(
                (
                    super::super::tagged(Operand::A, parent),
                    super::super::tagged(Operand::B, target),
                ),
                CoverSide::Off(Sign::Positive),
            );
        }
    }
    let _ = (sheet, face);
    let got = sweep_declared(&a, &b, &declared);
    assert!(got.is_ok(), "the touch is deferred: {:?}", got.err());
}

/// [`sweep_a`] under `declared`.
fn sweep_declared(
    a: &Body<f64>,
    b: &Body<f64>,
    declared: &GluedPairs<f64>,
) -> Result<(), BooleanError> {
    let (mut x, mut y) = (a.clone(), b.clone());
    let mut acc = super::ContactAcc::default();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    super::sweep_direction(
        (&mut x, &crate::sector_shape::SlitTips::of(a)),
        &mut y,
        Operand::A,
        declared,
        &mut acc,
        band,
        super::SweepStrategy::Realized,
        &super::SweepKnobs::default(),
        None,
        &mut Vec::new(),
        &crate::boolean::separating::OperandAxes::new(crate::boolean::boxes::axis_key),
        Tol::witness(),
    )
    .map(|_| ())
}
