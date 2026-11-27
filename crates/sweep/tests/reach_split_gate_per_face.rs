//! The split's carrier gate is per face: a body carrying a face of a
//! kind the split has no arm for splits wherever the plane cannot reach
//! that face, and refuses naming it wherever the plane may.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use topo::construction::Construct;

use crate::revolve_common::{axis_y, validated};
use geom::SurfaceKind;
use geom_core::{Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, bulge_from_center, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, SplitResult, split};
use topo::{AtRestBody, Body, POSE_UNIT_NORM, validate, validate_closed, validate_geometric};

/// The chain revolved about the y axis, finished.
fn revolved(chain: Vec<(Point2<f64>, f64)>) -> AtRestBody<f64> {
    let body = revolve(
        &validated(vec![bulge_loop(chain)]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the revolved body", body, Tol::witness())
}

/// A cut normal minted the way a caller holding a direction mints one.
fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).expect("the witness tolerance forms a band");
    UnitVec3::new(v, POSE_UNIT_NORM, band).expect("a cut normal has a length")
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
fn capped_cylinder() -> AtRestBody<f64> {
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
fn truncated_ball() -> AtRestBody<f64> {
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
    (topo::Body::clone(above), topo::Body::clone(below))
}

/// The unit cylinder `y ∈ [0, 1]` with its top rim rounded: the quarter
/// arc `(1, 1) → (3/4, 5/4)` about `(3/4, 1)`, a torus of radii 3/4 and
/// 1/4.
fn rounded_cylinder() -> AtRestBody<f64> {
    let (a, b) = (Point2::new(1.0, 1.0), Point2::new(0.75, 1.25));
    let bulge = bulge_from_center(a, b, Point2::new(0.75, 1.0), ArcSweep::Ccw);
    revolved(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (a, bulge),
        (b, 0.0),
        (Point2::new(0.0, 1.25), 0.0),
    ])
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
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(b), Tol::witness()),
        Tol::witness(),
    )
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
            let result = split(&body, &plane(phi, 0.5), Tol::witness()).unwrap_or_else(|e| {
                panic!(
                    "{what}: {e_at}",
                    e_at = e.render(geom_brep::recourse::Reading::Build)
                )
            });
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

/// A full revolve's sphere face is one face whose seam runs from its rim
/// to the pole at its tip and back, and its cover reads the latitude
/// zone between the rim and that pole, not the ball: the cap's reaches
/// `y ∈ [1, 1.5]` (its tip the `+y` pole), the truncated ball's
/// `y ∈ [−1, 1]` (its tip the `−y` pole, past a hemisphere). The ball
/// is `y ∈ [−1, 1.5]`, so each zone's far end off its tip is the rim,
/// which a tip-blind reading of the face's side misses: the rim alone
/// bounds a cap either way.
#[test]
fn a_one_band_sphere_faces_cover_is_its_zone_not_the_ball() {
    let band = Band::linear(Tol::witness()).unwrap();
    let pad = 1e-9;
    for (name, body, (lo, hi)) in [
        ("sphere cap", capped_cylinder(), (1.0, 1.5)),
        ("truncated ball", truncated_ball(), (-1.0, 1.0)),
    ] {
        let spheres: Vec<_> = body
            .faces()
            .filter(|(_, f)| body.get_surface(f.surface).unwrap().kind() == SurfaceKind::Sphere)
            .map(|(k, _)| k)
            .collect();
        let [face] = spheres.as_slice() else {
            panic!("{name}: one sphere face, got {}", spheres.len());
        };
        let cover = body.face_cover(*face, band).unwrap_or_else(|e| {
            panic!("{name}: the sphere face has a cover: {e:?}");
        });
        let (below, above) = cover.range_along(Vec3::new(0.0, 1.0, 0.0));
        assert!(
            (lo - pad..=lo + pad).contains(&below) && (hi - pad..=hi + pad).contains(&above),
            "{name}: the face reads y ∈ [{below}, {above}], its zone is [{lo}, {hi}]"
        );
    }
}

/// The sphere face of a full revolve of `chain`: its key and cover.
fn one_sphere_face(name: &str, body: &AtRestBody<f64>) -> geom::cover::Cover<f64> {
    let band = Band::linear(Tol::witness()).unwrap();
    let spheres: Vec<_> = body
        .faces()
        .filter(|(_, f)| body.get_surface(f.surface).unwrap().kind() == SurfaceKind::Sphere)
        .map(|(k, _)| k)
        .collect();
    let [face] = spheres.as_slice() else {
        panic!("{name}: one sphere face, got {}", spheres.len());
    };
    body.face_cover(*face, band)
        .unwrap_or_else(|e| panic!("{name}: the sphere face has a cover: {e:?}"))
}

/// A one-band sphere face's cover holds the face and is its zone: every
/// sampled point of the face (the profile arc from `rim` to the pole at
/// its tip, swept a full turn about `y`) lies within the cover's range
/// along each of 64 directions spread over the sphere, and the range
/// along `y` is the zone, not the ball. A concave face (a dimple, the
/// material outside its sphere, so its sense is negative) and a cap
/// whose rim is 3° from its pole read the same way as a convex cap.
#[test]
fn a_one_band_sphere_faces_cover_holds_the_face_at_either_sense_and_near_its_pole() {
    // A dimple of radius √0.72 about (0, 1.6) in the top of the unit
    // cylinder: the arc (0.6, 1) → (0, 1.6 − √0.72) runs clockwise.
    let dimple_r = 0.72f64.sqrt();
    let dimple_c = Point2::new(0.0, 1.6);
    let dimple_pole = Point2::new(0.0, 1.6 - dimple_r);
    let dimple = {
        let (a, b) = (Point2::new(0.6, 1.0), dimple_pole);
        let bulge = bulge_from_center(a, b, dimple_c, ArcSweep::Cw);
        revolved(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (a, bulge),
            (b, 0.0),
        ])
    };
    // A thin rod of radius 0.05 under a unit sphere's cap: its rim is
    // asin(0.05) ≈ 2.9° from the pole.
    let rod_c = Point2::new(0.0, 1.0 - (1.0f64 - 0.0025).sqrt());
    let rod_pole = Point2::new(0.0, rod_c.y + 1.0);
    let rod = {
        let (a, b) = (Point2::new(0.05, 1.0), rod_pole);
        let bulge = bulge_from_center(a, b, rod_c, ArcSweep::Ccw);
        revolved(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.05, 0.0), 0.0),
            (a, bulge),
            (b, 0.0),
        ])
    };
    let pad = 1e-9;
    let golden = PI * (3.0 - 5f64.sqrt());
    let dirs: Vec<Vec3<f64>> = (0..64)
        .map(|i| {
            let y = 1.0 - 2.0 * (f64::from(i) + 0.5) / 64.0;
            let rho = (1.0 - y * y).sqrt();
            let phi = golden * f64::from(i);
            Vec3::new(rho * phi.cos(), y, rho * phi.sin())
        })
        .collect();
    for (name, body, center, radius, rim, pole) in [
        (
            "dimple",
            dimple,
            dimple_c,
            dimple_r,
            Point2::new(0.6, 1.0),
            dimple_pole,
        ),
        ("rod cap", rod, rod_c, 1.0, Point2::new(0.05, 1.0), rod_pole),
    ] {
        let cover = one_sphere_face(name, &body);
        let (below, above) = cover.range_along(Vec3::new(0.0, 1.0, 0.0));
        let (lo, hi) = (rim.y.min(pole.y), rim.y.max(pole.y));
        assert!(
            (lo - pad..=lo + pad).contains(&below) && (hi - pad..=hi + pad).contains(&above),
            "{name}: the face reads y ∈ [{below}, {above}], its zone is [{lo}, {hi}]"
        );
        let (a0, a1) = (
            (rim.y - center.y).atan2(rim.x),
            (pole.y - center.y).atan2(pole.x),
        );
        let ranges: Vec<_> = dirs.iter().map(|&d| (d, cover.range_along(d))).collect();
        for i in 0..=16 {
            let a = a0 + (a1 - a0) * f64::from(i) / 16.0;
            let (rho, y) = (radius * a.cos(), center.y + radius * a.sin());
            for k in 0..24 {
                let theta = 2.0 * PI * f64::from(k) / 24.0;
                let p = Vec3::new(rho * theta.cos(), y, rho * theta.sin());
                for &(d, (lo, hi)) in &ranges {
                    let s = p.dot(d);
                    assert!(
                        lo - pad <= s && s <= hi + pad,
                        "{name}: the face point {p:?} reads {s} along {d:?}, outside the \
                         cover's [{lo}, {hi}]"
                    );
                }
            }
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
    let result = split(
        &finished("the loft", loft, Tol::witness()),
        &over,
        Tol::witness(),
    )
    .unwrap_or_else(|e| {
        panic!(
            "{e_at}",
            e_at = e.render(geom_brep::recourse::Reading::Build)
        )
    });
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
/// `c + a·cos t·û + b·sin t·v̂`, `t ∈ [t0, t1]`, on the parameter domain
/// `[d0, d1]`, in `n` equal segments: each is the unit circle's arc of at
/// most a quarter turn, mapped by the conic's affine frame, which a
/// rational curve's weights survive, and each knot is the image of the
/// conic's angle at that segment's end.
fn conic_arc_segments(
    c: Point3<f64>,
    (u, v): (Vec3<f64>, Vec3<f64>),
    (a, b): (f64, f64),
    (t0, t1): (f64, f64),
    n: u32,
    (d0, d1): (f64, f64),
) -> geom::NurbsCurve3<f64> {
    let at = |x: f64, y: f64| c + u * (a * x) + v * (b * y);
    let step = (t1 - t0) / f64::from(n);
    let w = (step / 2.0).cos();
    let mut control = vec![at(t0.cos(), t0.sin())];
    let mut weights = vec![1.0];
    let mut knots = vec![d0; 3];
    for k in 0..n {
        let (s, e) = (t0 + step * f64::from(k), t0 + step * f64::from(k + 1));
        let m = (s + e) / 2.0;
        control.push(at(m.cos() / w, m.sin() / w));
        control.push(at(e.cos(), e.sin()));
        weights.extend([w, 1.0]);
        let knot = d0 + (d1 - d0) * f64::from(k + 1) / f64::from(n);
        knots.extend(if k + 1 == n {
            vec![d1; 3]
        } else {
            vec![knot; 2]
        });
    }
    let kv = geom_core::spline::KnotVector::clamped(knots, 2).unwrap();
    geom::NurbsCurve3::new(kv, control, weights).unwrap()
}

/// The unit cylinder cut by the plane `y = 0.5 + 0.3·z`, its lower half:
/// a planar elliptic top bounded by one ellipse through the wall's seam
/// at `(1, 0.5, 0)`, split by hand at its antipode `(−1, 0.5, 0)` into
/// two half-ellipses (construction state), and the half on `z < 0`,
/// which dips to `y = 0.2` between them: an edge between a plane face
/// and a cylinder face, both armed.
fn tilted_cut_rim() -> (Body<f64>, Rim) {
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
    let SplitPart::Body(body) = split(&cyl, &tilt, Tol::witness()).unwrap().below else {
        panic!("the tilted cut leaves material below");
    };
    // The section is one closed ellipse through the wall's seam at
    // `(1, 0.5, 0)`; split at its antipode it is the two half-ellipses.
    let mut body = body.into_body();
    let (whole, (t0, t1)) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            matches!(c.carrier(), geom::Curve3::Ellipse(..)).then(|| (k, c.params()))
        })
        .expect("the tilted section is one ellipse");
    body.split_edge(whole, 0.5 * (t0 + t1), Tol::witness())
        .expect("the ellipse splits at its antipode");
    let rim = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            let ellipse = c.carrier().clone();
            let geom::Curve3::Ellipse(..) = ellipse else {
                return None;
            };
            let params = c.params();
            let mid = ellipse.eval((params.0 + params.1) / 2.0);
            if mid.z > 0.0 {
                return None;
            }
            Some(Rim {
                edge: k,
                ellipse,
                params,
                mid,
            })
        })
        .expect("the tilted section has a half-ellipse on z < 0");
    (body, rim)
}

/// [`tilted_cut_rim`]'s half-ellipse: its edge, carrier, interval and
/// the carrier's point at the interval's middle.
struct Rim {
    edge: topo::EdgeKey,
    ellipse: geom::Curve3<f64>,
    params: (f64, f64),
    mid: Point3<f64>,
}

impl Rim {
    /// The rim's exact rational arc in `n` segments over `domain`, as
    /// an intersection of its two faces' surfaces.
    fn as_spline(
        &self,
        body: &Body<f64>,
        n: u32,
        domain: (f64, f64),
    ) -> (geom::NurbsCurve3<f64>, geom_brep::EdgeCurveSpec<f64>) {
        let geom::Curve3::Ellipse(conic_k) = &self.ellipse else {
            unreachable!("the rim is a half-ellipse");
        };
        let (center, axis, u_ref) = (
            conic_k.frame().origin(),
            conic_k.frame().axis(),
            conic_k.frame().u_ref(),
        );
        let (semi_u, semi_v) = (conic_k.semi_u(), conic_k.semi_v());
        let spline = conic_arc_segments(
            center,
            (u_ref, axis.cross(u_ref)),
            (semi_u, semi_v),
            self.params,
            n,
            domain,
        );
        let he = body.get_edge(self.edge).unwrap().he_plus;
        let start = *body
            .get_point(
                body.get_vertex(body.get_half_edge(he).unwrap().start)
                    .unwrap()
                    .point,
            )
            .unwrap();
        assert!(
            (spline.eval(domain.0) - start).norm() < 1e-12,
            "the spline runs the edge's way"
        );
        let (s1, s2) = topo::readback::edge_sides(body, self.edge)
            .unwrap()
            .surfaces();
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(s1, s2),
                selector: geom_brep::CurveSelector::Witness(self.mid),
            },
            carrier: geom::Curve3::Nurbs(std::sync::Arc::new(spline.clone())),
            param_start: domain.0,
            param_end: domain.1,
        };
        (spline, spec)
    }
}

/// [`tilted_cut_rim`] with its rim re-described as its own exact spline
/// on `[0, 1]`.
fn cylinder_with_a_spline_rim() -> (Body<f64>, topo::EdgeKey) {
    let (mut body, rim) = tilted_cut_rim();
    let (_, spec) = rim.as_spline(&body, 2, (0.0, 1.0));
    body.set_edge_curve(rim.edge, spec, Tol::witness())
        .expect("the exact spline attaches");
    (body, rim.edge)
}

/// A spline edge between two armed faces, its ends on one side of the
/// plane and its belly across it, refuses at the gate. Nothing else
/// reads it: no crossing lane serves a spline, so passing it would
/// leave the edge uncut while the plane crosses the faces beside it.
///
/// The re-description moves the rim's carrier, so it re-mints the
/// faces the rim bounds. The spline's image on the cylinder is its
/// projected one, whose hull terms read the fitted door, which the
/// description door's site mint does not hold. So the cylinder face is
/// left rowless, and the finish completes it with the projected rows
/// (`work/pcert/site-rows-derive-through-chart-pcurve-bypassing-the-routed-arm.md`).
/// The hand split that leaves the half-ellipse is a station the gate
/// reads too. The body is not a finished body and no split door takes
/// it; the row reads the carrier gate past the door.
#[test]
fn a_spline_edge_whose_belly_crosses_the_plane_refuses() {
    let (body, edge) = cylinder_with_a_spline_rim();
    for part in [validate(&body), validate_closed(&body)] {
        assert_eq!(part, Ok(()), "the re-described body is well formed");
    }
    let errors = topo::finish(body.clone(), Tol::witness())
        .expect_err("the face the spline bounds stores no row");
    let rim_faces = topo::readback::edge_sides(&body, edge).unwrap().faces();
    assert!(
        !errors.is_empty()
            && errors.iter().all(|e| matches!(
                e,
                topo::ValidationError::Pcurve {
                    finding: topo::pcurves::PcurveMintError::Unminted { face },
                } if face == &rim_faces.0 || face == &rim_faces.1
            ) || matches!(
                e,
                topo::ValidationError::JoinableVertexAtRest { .. }
            )),
        "the at-rest gate reads the face the spline bounds as owed rows it lacks, and the \
         hand split's station: {errors:?}"
    );
    let mut finished = body.clone();
    topo::mint_pcurves(&mut finished, Tol::witness()).expect("the closing mint");
    let sides = topo::readback::edge_sides(&finished, edge).unwrap();
    let projected = [sides.plus.half_edge, sides.minus.half_edge]
        .into_iter()
        .filter_map(|h| finished.pcurve(h))
        .any(|c| matches!(c.pcurve(), geom_brep::Pcurve::Projected(_)));
    assert!(projected, "the finish stores the spline's projected row");
    match topo::test_support::split_carrier_gate(&body, &plane(0.0, 0.4), Tol::witness()) {
        Err(SplitReduceError::CurvedEdgeUnsupported { edge: e }) => {
            assert_eq!(e, edge, "the refusal names the spline edge");
        }
        other => panic!("the plane through the spline's belly refuses, got {other:?}"),
    }
}

/// **A spline carrier on the rim's own locus and interval, moved only
/// between its quarters, keeps no row of the old one.** The rim's
/// half-ellipse is re-described as its exact rational arc in four
/// segments over the ellipse's own interval, each knot at the ellipse's
/// angle there: it meets the ellipse at the interval's ends and
/// quarters, and runs off it between, since a rational quadratic arc
/// does not turn at a uniform rate. Rows stated over the ellipse do not
/// image the spline, so the cylinder face is re-minted; the closed-form
/// lane has no image of a spline, and the face leaves storing nothing.
#[test]
fn a_spline_rim_moved_between_its_quarters_keeps_no_row() {
    let (mut body, rim) = tilted_cut_rim();
    let (t0, t1) = rim.params;
    let (spline, spec) = rim.as_spline(&body, 4, rim.params);
    let at = |k: f64| t0 + (t1 - t0) * k;
    for k in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let gap = (spline.eval(at(k)) - rim.ellipse.eval(at(k))).norm();
        assert!(gap < 1e-12, "the spline meets the ellipse at {k}: {gap:e}");
    }
    let between = (0..=64)
        .map(|i| at(f64::from(i) / 64.0))
        .map(|t| (spline.eval(t) - rim.ellipse.eval(t)).norm())
        .fold(0.0, f64::max);
    assert!(
        between > 1e-3,
        "the spline runs off the ellipse between its quarters: {between:e}"
    );
    let cylinder = {
        let (f0, f1) = topo::readback::edge_sides(&body, rim.edge).unwrap().faces();
        let on_cylinder = |f| {
            let surface = body.get_face(f).unwrap().surface;
            body.get_surface(surface).unwrap().kind() == SurfaceKind::Cylinder
        };
        if on_cylinder(f0) { f0 } else { f1 }
    };
    let rows_on = |body: &Body<f64>| {
        body.pcurves()
            .filter(|&(he, _)| {
                let lk = body.get_half_edge(he).unwrap().parent_loop;
                body.get_loop(lk).unwrap().face == cylinder
            })
            .count()
    };
    assert!(rows_on(&body) > 0, "the cylinder face stores rows to keep");
    body.set_edge_curve(rim.edge, spec, Tol::witness())
        .expect("the re-parameterized spline attaches");
    assert_eq!(
        rows_on(&body),
        0,
        "the cylinder face is re-minted, and stores nothing"
    );
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
///
/// Such a face is not a finished body's (the at-rest gate refuses its
/// sense, naming the zone), so no split door takes it; the row reads the
/// carrier gate past the door.
#[test]
fn a_sphere_face_whose_side_is_not_certified_keeps_the_ball() {
    let wedge = sweep::test_support::sphere_zone(0.5, Revolution::Partial(1.0), Tol::witness());
    let above = plane(0.0, 1.5);
    let whole = split(
        &finished("the wedge", wedge.clone(), Tol::witness()),
        &above,
        Tol::witness(),
    )
    .expect("the zone's box clears the cut");
    assert!(
        matches!(
            (&whole.above, &whole.below),
            (SplitPart::Empty, SplitPart::Body(_))
        ),
        "the wedge lies below the cut"
    );
    let mut reverted = wedge.revert().unwrap();
    let (zone, surface, sense) = reverted
        .faces()
        .find_map(|(k, f)| {
            let s = reverted.face_surface(k).ok()?;
            matches!(s, geom::Surface::Sphere { .. }).then(|| (k, s.clone(), f.sense))
        })
        .expect("the wedge has a sphere face");
    reverted
        .set_face_surface_unvouched_for_tests(
            zone,
            topo::construction::FaceSurface::New {
                surface,
                sense: !sense,
            },
        )
        .unwrap();
    let errors = topo::finish(reverted.clone(), Tol::witness())
        .expect_err("a zone bounding its complement is not a finished body's");
    assert!(
        errors.iter().any(
            |e| matches!(e, topo::ValidationError::CurvedSenseInverted { face } if *face == zone)
        ),
        "the at-rest gate names the zone's sense: {errors:?}"
    );
    match topo::test_support::split_carrier_gate(&reverted, &above, Tol::witness()) {
        Err(SplitReduceError::CurvedBooleanUnsupported {
            face,
            kind: SurfaceKind::Sphere,
        }) => assert_eq!(face, zone, "the refusal names the zone"),
        other => panic!("an uncertified side keeps the ball, got {other:?}"),
    }
}
