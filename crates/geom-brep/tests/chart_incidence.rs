//! **The class gates of the sphere and cone charts against their
//! incidence tests.** `chart_pcurve`'s sphere arm reads two class gates
//! — polar (the carrier plane ⊥ the axis, centred on it) and meridian —
//! and the cone's reads one (a rim: ⊥ the axis, centred on it). Every
//! carrier failing them is decided by an incidence test
//! (`pcurve_sphere_chart_incident`, `pcurve_cone_chart_incident`): on
//! the chart, its image, or a class with no general lane yet; off
//! it, a carrier off the chart. A
//! gate that reads a SECOND, amplified quantity after an in-band first
//! one — or a first-order quantity for a second-order departure — must
//! not decide off-chart on its own; these rows build on-chart carriers
//! at exactly that boundary.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::off_cone;
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{Pcurve, PcurveCertifyError, chart_pcurve};
use geom_core::spline::KnotVector;
use geom_core::{Interval, Point3, Real, Tol, Vec3};

/// The right circular cone of half-angle `alpha` about `+z` with its
/// apex at the origin.
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// The cone's section by the plane through `(0, 0, h)` whose normal is
/// tilted `theta` off the axis toward `+x`: an ellipse, major along the
/// plane's `x` direction. With `T = tan²α` and `A = cos²θ − T·sin²θ`,
/// the centre sits `−T·h·sin θ / A` along that direction and the
/// semi-axes are `h·tan α·cos θ / A` and `h·tan α·cos θ / √A`.
fn section(alpha: f64, h: f64, theta: f64) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64) {
    let t = alpha.tan().powi(2);
    let (s, c) = theta.sin_cos();
    let a = c * c - t * s * s;
    let u = Vec3::new(c, 0.0, -s);
    let centre = Point3::new(0.0, 0.0, h) + u * (-t * h * s / a);
    let k = h * alpha.tan() * c;
    (centre, Vec3::new(s, 0.0, c), u, k / a, k / a.sqrt())
}

/// A tilted plane section of a cone lies on it, and is imaged; the
/// same ellipse moved off it by `2·K·ε` — just past the sliver, so a
/// meter loose by more than a factor of two would excuse it — is a
/// carrier off the chart.
#[test]
fn a_cone_section_is_imaged_and_a_moved_one_is_off_the_chart() {
    let alpha = 0.5;
    let (centre, axis, u_ref, major, minor) = section(alpha, 2.0, 0.3);
    let on = Curve3::Ellipse {
        center: centre,
        axis,
        semi_u: major,
        semi_v: minor,
        u_ref,
    };
    let dist = off_cone(&on, &cone(alpha));
    assert!(dist < 1e-14, "on the cone: {dist:e}");
    let got = chart_pcurve(&on, &cone(alpha), band());
    assert!(
        matches!(got, Ok(Pcurve::FocalSection(_))),
        "a cone section derives its section image: {got:?}"
    );
    let shift = 2.0 * Tol::witness().k() * eps();
    let moved = Curve3::Ellipse {
        center: centre + axis * shift,
        axis,
        semi_u: major,
        semi_v: minor,
        u_ref,
    };
    let dist = off_cone(&moved, &cone(alpha));
    assert!(
        dist > Tol::witness().k() * eps(),
        "the move is past the sliver: {dist:e}"
    );
    let got = chart_pcurve(&moved, &cone(alpha), band());
    assert!(
        matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
        "a moved section is off the cone: {got:?}"
    );
}

/// The cone incidence margin, `|k₀| + |(k₁, k₂)| + |(k₃, k₄)|` over
/// `2·sin α·z_min`, read through one harmonic group at a time: each row
/// builds a conic off the cone whose residual lives in ONE group, at
/// `5·K·ε` — past the sliver — with every other group inside `ε`, so a
/// margin that dropped that group would call it uncovered.
#[test]
fn each_harmonic_group_of_the_cone_residual_decides_alone() {
    let (e, k) = (eps(), Tol::witness().k());
    let off = |what: &str, carrier: Curve3<f64>, alpha: f64| {
        let got = chart_pcurve(&carrier, &cone(alpha), band());
        assert!(
            matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "{what}: {got:?}"
        );
    };
    // k₀ alone: a circle ⊥ the axis of the wrong radius, centred 2·K·ε
    // off the axis so the rim's centring gate sends it to the test.
    // `k₀ ≈ cos²α·2ρ·δρ`, `k₁ = 2·cos²α·w·a` stays under ε.
    let alpha = 1.55_f64;
    let rho = alpha.tan();
    let d_rho = 5.0 * k * e * 2.0 * alpha.sin() / (alpha.cos().powi(2) * 2.0 * rho);
    off(
        "k0: a wrong radius",
        Curve3::Circle {
            center: Point3::new(2.0 * k * e, 0.0, 1.0),
            axis: Vec3::unit_z(),
            radius: rho + d_rho,
            u_ref: Vec3::unit_x(),
        },
        alpha,
    );
    // (k₁, k₂) alone: the right radius, centred δ off the axis — the
    // first harmonic is `2·cos²α·δ·ρ`, k₀ = `cos²α·δ²` is second
    // order, and the margin reads `δ·cos α`.
    let alpha = 0.5_f64;
    let delta = 5.0 * k * e / alpha.cos();
    off(
        "k1,k2: an off-axis rim",
        Curve3::Circle {
            center: Point3::new(delta, 0.0, 1.0),
            axis: Vec3::unit_z(),
            radius: alpha.tan(),
            u_ref: Vec3::unit_x(),
        },
        alpha,
    );
    // (k₃, k₄) alone: an ellipse ⊥ the axis, centred on it, with
    // `A² + B² = 2ρ²` (so k₀ = 0) and `A² − B² = 2s` (k₃ = cos²α·s).
    let rho = alpha.tan();
    let s = 5.0 * k * e * 2.0 * alpha.sin() / alpha.cos().powi(2);
    off(
        "k3,k4: an ellipse round the axis",
        Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::unit_z(),
            semi_u: (rho * rho + s).sqrt(),
            semi_v: (rho * rho - s).sqrt(),
            u_ref: Vec3::unit_x(),
        },
        alpha,
    );
}

/// **The cone's rim gates on a needle and on a wide cone.** A circle
/// approximating the section by a plane tilted inside the axial band
/// (`ρ·sin θ = ε/2`) lies on the cone to rounding. Its centre is
/// `≈ tan α · ε/2` off the axis: inside the centring band on a needle
/// cone, where it is a rim and images in closed form, and past it on a
/// wide one, where the centring gate fails. There the incidence test,
/// not the gate, decides — and the circle is not shown off the cone, a
/// circle that is none of its rims (a cone holds no other circle), whose
/// general lane is not written yet.
#[test]
fn a_near_rim_circle_tilted_inside_the_axial_band_is_on_the_cone() {
    let e = eps();
    for (alpha, rim) in [(0.05_f64, true), (1.55, false)] {
        let h = 1.0;
        let theta = 0.5 * e / (h * alpha.tan());
        let (centre, axis, u_ref, major, minor) = section(alpha, h, theta);
        let circle = Curve3::Circle {
            center: centre,
            axis,
            radius: (major * minor).sqrt(),
            u_ref,
        };
        let dist = off_cone(&circle, &cone(alpha));
        assert!(dist < 0.01 * e, "alpha = {alpha}: on the cone: {dist:e}");
        let got = chart_pcurve(&circle, &cone(alpha), band());
        if rim {
            assert!(
                got.is_ok(),
                "alpha = {alpha}: a needle cone's near-rim is a rim: {got:?}"
            );
        } else {
            assert!(
                centre.x.abs() > 10.0 * e,
                "alpha = {alpha}: the centre offset is past the band: {:e}",
                centre.x
            );
            assert!(
                matches!(got, Err(PcurveCertifyError::NoGeneralLane { .. })),
                "alpha = {alpha}: an on-cone circle that is no rim refuses not yet (its general \
                 lane is unwritten), never off the chart: {got:?}"
            );
        }
    }
}

/// A small circle exactly on the unit sphere near its pole, whose plane
/// is tilted off ⊥-the-axis by an angle `θ` small enough that the
/// polar class's axial gate (`|a·z| + |b·z| = ρ·sin θ`) reads ZERO, but
/// whose centre's radial offset (`h·sin θ`, `h ≈ R`) is `h/ρ` times
/// larger and reads nonzero. The circle lies on the sphere (its centre
/// is the foot of the sphere's centre on its plane, `ρ² + h² = R²`), so
/// the pair is an uncovered general circle, never off the chart.
#[test]
fn a_near_pole_small_circle_tilted_inside_the_axial_band_is_on_the_sphere() {
    let e = eps();
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for rho in [1e-2_f64, 1e-3, 1e-4] {
        let h = (1.0 - rho * rho).sqrt();
        // ρ·sin θ = ε/2: inside the axial gate's zero half.
        let s = 0.5 * e / rho;
        let c = (1.0 - s * s).sqrt();
        let n = Vec3::new(s, 0.0, c);
        let carrier = Curve3::Circle {
            center: Point3::new(h * s, 0.0, h * c),
            axis: n,
            radius: rho,
            u_ref: Vec3::new(c, 0.0, -s),
        };
        let worst = (0..64)
            .map(|i| {
                let p = carrier.eval(f64::from(i) * core::f64::consts::TAU / 64.0);
                ((p.x * p.x + p.y * p.y + p.z * p.z).sqrt() - 1.0).abs()
            })
            .fold(0.0_f64, f64::max);
        assert!(worst < 1e-14, "rho = {rho:e}: on the sphere: {worst:e}");
        // The amplified quantity really is past the band, so the row
        // exercises the second gate rather than passing the first two.
        assert!(h * s > 10.0 * e, "rho = {rho:e}: centre offset {:e}", h * s);
        let got = chart_pcurve(&carrier, &sphere, band());
        assert!(
            matches!(got, Ok(Pcurve::Projected(_))),
            "rho = {rho:e}: an on-sphere circle is a general circle with a projected image: \
             {got:?}"
        );
    }
}

/// **An in-band move is never read off the cone or the sphere** (the
/// one-sided incidence test). A cone section moved along the cone's
/// axis by half the band's zero half derives its image; a sphere's
/// general circle moved by as much is the general circle the projected
/// route images, never a carrier off the chart.
#[test]
fn an_in_band_move_is_never_read_off_the_cone_or_the_sphere() {
    let alpha = 0.5;
    let (centre, axis, u_ref, major, minor) = section(alpha, 2.0, 0.3);
    let moved = Curve3::Ellipse {
        center: centre + Vec3::unit_z() * (0.5 * eps()),
        axis,
        semi_u: major,
        semi_v: minor,
        u_ref,
    };
    let got = chart_pcurve(&moved, &cone(alpha), band());
    assert!(
        matches!(got, Ok(Pcurve::FocalSection(_))),
        "a cone section moved ε/2: {got:?}"
    );
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let n = Vec3::new(0.3, 0.0, 1.0).normalize();
    let (h, rho) = (0.6_f64, 0.8_f64);
    let circle = Curve3::Circle {
        center: Point3::origin() + n * (h + 0.5 * eps()),
        axis: n,
        radius: rho,
        u_ref: Vec3::unit_y(),
    };
    let got = chart_pcurve(&circle, &sphere, band());
    assert!(
        matches!(got, Ok(Pcurve::Projected(_))),
        "a general circle moved ε/2 off the sphere: {got:?}"
    );
}

/// **A poison end of the interval locates no piece, so the projected
/// image's box is poison.** The `Net` carrier is the sharp case: its
/// box is built from knot values alone, so a poison end that located
/// the first piece read a certified box of that piece for a window
/// nobody asked about. At the interval scalar a NaI end now answers a
/// box with nothing certified in it.
#[test]
fn a_projected_net_image_boxes_a_nai_end_as_poison() {
    let lift = |x: f64| Interval::from_f64(x);
    let carrier = Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(
            geom_core::spline::Knots::from(
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            )
            .lift(geom_core::Real::from_f64),
            vec![
                Point3::new(lift(0.1), lift(-0.2), lift(0.0)),
                Point3::new(lift(0.7), lift(0.9), lift(0.0)),
                Point3::new(lift(1.3), lift(0.2), lift(0.0)),
                Point3::new(lift(1.6), lift(0.8), lift(0.0)),
            ],
            [1.0, 0.6, 1.0, 1.0].map(lift).to_vec(),
        )
        .unwrap(),
    ));
    let plane = Surface::Plane {
        origin: Point3::new(lift(0.05), lift(0.1), lift(0.0)),
        normal: Vec3::new(lift(0.0), lift(0.0), lift(1.0)),
        u_ref: Vec3::new(lift(1.0), lift(0.0), lift(0.0)),
    };
    let image = chart_pcurve(&carrier, &plane, band()).unwrap();
    assert!(
        matches!(&image, Pcurve::Projected(p) if matches!(p.carrier(), geom_brep::FramedCarrier::Net(_))),
        "the fixture is a projected net: {image:?}"
    );
    let certified = |b: geom_brep::ChartWindow<Interval>| {
        [b.u_min, b.u_max, b.v_min, b.v_max]
            .iter()
            .all(|x| x.is_certified())
    };
    let ok = image.chart_box(lift(0.0), lift(1.0));
    assert!(certified(ok), "CONTROL: the whole interval boxes: {ok:?}");
    let nai = lift(f64::NAN);
    for (name, b) in [
        ("NaI start", image.chart_box(nai, lift(1.0))),
        ("NaI end", image.chart_box(lift(0.0), nai)),
    ] {
        assert!(
            [b.u_min, b.u_max, b.v_min, b.v_max]
                .iter()
                .all(|x| x.is_poison()),
            "{name}: {b:?}"
        );
    }
}

/// **The projected image answers a poison parameter with poison in
/// every channel it carries.** At `Dual` that includes the derivative
/// channel, where a bare `from_f64(NaN)` would be a dual constant whose
/// derivative reads as a zero: the point (`ProjectedImage::eval`) and
/// the box (`chart_box`'s poison window) are both checked.
#[test]
fn a_projected_image_answers_a_poison_dual_parameter_with_poison_in_every_channel() {
    use geom_core::Dual64;
    let lift = Dual64::constant;
    let carrier = Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(
            geom_core::spline::Knots::from(
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            )
            .lift(geom_core::Real::from_f64),
            vec![
                Point3::new(lift(0.1), lift(-0.2), lift(0.0)),
                Point3::new(lift(0.7), lift(0.9), lift(0.0)),
                Point3::new(lift(1.3), lift(0.2), lift(0.0)),
                Point3::new(lift(1.6), lift(0.8), lift(0.0)),
            ],
            [1.0, 0.6, 1.0, 1.0].map(lift).to_vec(),
        )
        .unwrap(),
    ));
    let plane = Surface::Plane {
        origin: Point3::new(lift(0.05), lift(0.1), lift(0.0)),
        normal: Vec3::new(lift(0.0), lift(0.0), lift(1.0)),
        u_ref: Vec3::new(lift(1.0), lift(0.0), lift(0.0)),
    };
    let image = chart_pcurve(&carrier, &plane, band()).unwrap();
    assert!(
        matches!(&image, Pcurve::Projected(p) if matches!(p.carrier(), geom_brep::FramedCarrier::Net(_))),
        "the fixture is a projected net: {image:?}"
    );
    let poison = |x: Dual64| x.value.is_nan() && x.deriv.is_nan();
    let t = Dual64::variable(f64::NAN);
    let p = image.eval(t);
    assert!(poison(p.x) && poison(p.y), "eval: {p:?}");
    let b = image.chart_box(t, lift(1.0));
    assert!(
        [b.u_min, b.u_max, b.v_min, b.v_max].into_iter().all(poison),
        "chart_box: {b:?}"
    );
}

/// **A plane chart's box covers every span of its net.** A plane chart
/// reads the whole net as one piece, so the piece crosses the net's
/// knots; its box is each span's own restriction, hulled. The fixture is
/// a two-span polyline `(0,0) → (1,0) → (1,5)`, whose first span
/// extended past its knot runs along `v = 0`: a box read off that
/// extension misses the corner `(1, 5)` the curve ends at. At `f64` and
/// at the certification scalar, where the box must also certify.
#[test]
fn a_plane_chart_box_covers_every_span_of_its_net() {
    fn check<T: geom_core::Decide>(
        lift: impl Fn(f64) -> T,
        lo: impl Fn(T) -> f64,
        hi: impl Fn(T) -> f64,
        tag: &str,
    ) {
        let carrier = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::Knots::from(
                    KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
                )
                .lift(&lift),
                vec![
                    Point3::new(lift(0.0), lift(0.0), lift(0.0)),
                    Point3::new(lift(1.0), lift(0.0), lift(0.0)),
                    Point3::new(lift(1.0), lift(5.0), lift(0.0)),
                ],
                vec![lift(1.0), lift(1.0), lift(1.0)],
            )
            .unwrap(),
        ));
        let plane = Surface::Plane {
            origin: Point3::new(lift(0.0), lift(0.0), lift(0.0)),
            normal: Vec3::new(lift(0.0), lift(0.0), lift(1.0)),
            u_ref: Vec3::new(lift(1.0), lift(0.0), lift(0.0)),
        };
        let image = chart_pcurve(&carrier, &plane, band()).unwrap();
        let b = image.chart_box(lift(0.0), lift(1.0));
        let (u, v) = ((lo(b.u_min), hi(b.u_max)), (lo(b.v_min), hi(b.v_max)));
        assert!(
            u.0 <= 0.0 && u.1 >= 1.0 && v.0 <= 0.0 && v.1 >= 5.0,
            "{tag}: the box u {u:?} v {v:?} misses a corner of the polyline [0, 1] x [0, 5]"
        );
    }
    check(|x| x, |x| x, |x| x, "f64");
    check(
        Interval::from_f64,
        |x: Interval| {
            if x.is_certified() {
                geom_core::Bounds::lo(x)
            } else {
                f64::NAN
            }
        },
        |x: Interval| {
            if x.is_certified() {
                geom_core::Bounds::hi(x)
            } else {
                f64::NAN
            }
        },
        "Interval",
    );
}

/// **A rim whose height is not shown away from the apex's refuses as
/// not shown, quoting its margin** (`pcurve_cone_chart_nappe`'s Zero
/// arm). A circle ⊥ the axis, centred on it at a height inside the zero
/// band, passes the rim gates and reaches the nappe: a band-decided Zero
/// is not an exact one, so the refusal says the height "is not shown
/// away from the apex's" and carries the margin the decision read, never
/// the bare fact that the apex plane meets the cone only at the apex.
/// Read at `f64` and at `Interval`.
#[test]
fn a_rim_at_the_apex_height_refuses_as_not_shown_with_its_margin() {
    let surface = cone(0.5);
    let Surface::Cone {
        apex, axis, u_ref, ..
    } = surface
    else {
        unreachable!()
    };
    let rim = Curve3::Circle {
        center: apex + axis * (0.2 * eps()),
        axis,
        radius: 0.3,
        u_ref,
    };
    let check = |got: Result<String, PcurveCertifyError>, lane: &str| match got {
        Err(PcurveCertifyError::CarrierOffChart {
            why,
            margin: Some(geom_brep::OffMargin::InBand(_)),
            ..
        }) => {
            assert!(
                why.contains("is not shown away from the apex"),
                "{lane}: {why}"
            );
        }
        other => panic!("{lane}: a rim at the apex's height: {other:?}"),
    };
    let got = chart_pcurve(&rim, &surface, band());
    if let Err(e) = &got {
        assert!(e.to_string().contains("(margin "), "{e}");
    }
    check(got.map(|p| format!("{p:?}")), "f64");
    let rim_i: Curve3<Interval> = rim.map_scalar(Interval::from_f64);
    let surface_i: Surface<Interval> = surface.map_scalar(Interval::from_f64);
    check(
        chart_pcurve(&rim_i, &surface_i, band()).map(|p| format!("{p:?}")),
        "Interval",
    );
}

/// **Only an incidence reading ends as one** (D3, D4 ¶1 (iv)): a pose
/// reading ends in the pose lever, a definite miss as the door's defect,
/// and neither reaches "not shown on the face", the undecided incidence
/// read's ending. Read at `f64` and at `Interval`:
/// - a rim whose height sits in the apex's sliver
///   (`pcurve_cone_chart_nappe`, `cone_conic_incidence`): the pose the
///   in-band Zero names, so the pose lever, though it may lie on the cone;
/// - a near-round ellipse 10 m off the sphere and off the torus: definitely
///   off, the kernel's defect at a build, whatever its roundness (review
///   on PR 5844). Today the kind gate refuses it by kind; once E1c reads
///   incidence first it is the incidence test's definite miss, the same
///   ending.
///
/// The Villarceau tangent's parts, a class reading at `K ≤ 3`, are
/// pinned in `pcurve_villarceau`. A tiny ellipse within ε of the sphere,
/// which the kind gate still calls off, is E1c's row
/// (`work/pcert/plane-and-cylinder-mints-meter-incidence-as-a-residual.md`).
#[test]
fn only_an_incidence_reading_ends_as_one() {
    use geom_brep::OffMargin;
    use geom_brep::recourse::{INCIDENCE_AT_POSE, INCIDENCE_UNSHOWN, Reading};
    let e = eps();
    let at_pose = |what: &str, got: Result<(), PcurveCertifyError>| {
        let Err(
            err @ PcurveCertifyError::CarrierOffChart {
                margin: Some(OffMargin::InBand(_)),
                ..
            },
        ) = &got
        else {
            panic!("{what}: a pose reading: {got:?}");
        };
        let text = err.render(Reading::Build);
        assert!(
            text.contains(INCIDENCE_AT_POSE.lever)
                && !text.contains(INCIDENCE_UNSHOWN.lever)
                && !text.contains("defect"),
            "{what}: {text}"
        );
    };
    let definitely_off = |what: &str, got: Result<(), PcurveCertifyError>| {
        let Err(err @ PcurveCertifyError::CarrierOffChart { margin, .. }) = &got else {
            panic!("{what}: definitely off: {got:?}");
        };
        assert!(
            !matches!(margin, Some(OffMargin::InBand(_)))
                && err
                    .render(Reading::Build)
                    .ends_with(geom_core::KERNEL_DEFECT_ENDING),
            "{what}: {err}"
        );
    };
    let both = |what: &str,
                carrier: Curve3<f64>,
                surface: Surface<f64>,
                want: &dyn Fn(&str, Result<(), PcurveCertifyError>)| {
        want(
            &format!("{what} (f64)"),
            chart_pcurve(&carrier, &surface, band()).map(|_| ()),
        );
        want(
            &format!("{what} (Interval)"),
            chart_pcurve(
                &carrier.map_scalar(Interval::from_f64),
                &surface.map_scalar(Interval::from_f64),
                band(),
            )
            .map(|_| ()),
        );
    };
    // The apex's sliver: past the zero band, under the escalation band.
    let alpha = 0.5_f64;
    for h in [2.0 * e, 5.5 * e] {
        both(
            &format!("a rim at h = {h:e}"),
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, h),
                axis: Vec3::unit_z(),
                radius: h * alpha.tan(),
                u_ref: Vec3::unit_x(),
            },
            cone(alpha),
            &at_pose,
        );
    }
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let ellipse = |z: f64, r: f64, d: f64| Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::unit_z(),
        semi_u: r,
        semi_v: r - d,
        u_ref: Vec3::unit_x(),
    };
    both(
        "a near-round ellipse 10 m off the sphere",
        ellipse(10.0, 0.8, 0.5 * e),
        sphere,
        &definitely_off,
    );
    both(
        "a near-round ellipse 10 m off the torus",
        ellipse(10.0, 2.5, 0.5 * e),
        torus,
        &definitely_off,
    );
}

/// **A class rate read Zero is no incidence reading, so the incidence test
/// decides** (D3; review on PR 5844, H2). Each closed-form arm that reads
/// a class's rate (`pcurve_chart_orientation` on a cone ellipse,
/// `pcurve_sphere_chart_polar_rate`, `pcurve_torus_chart_meridional_rate`)
/// and reads it Zero sends the carrier through D3's one door to the
/// general lane: a carrier off the chart is the test's definite miss, one
/// not shown off is the general lane's, here "not yet". Each arm is
/// reached:
/// - a thin ellipse (minor semi-axis under ε) along a cone generator,
///   whose radial projection does not wind: on the cone, "not yet";
/// - a great circle of the sphere whose plane leans `4.1e-10` off the
///   axis, turned so its radial part passes the escalation band, at band
///   `(1e-9, 1.05e-9)`: its polar rate reads `9.75e-10`, Zero, though it
///   lies on the sphere (`K < √2`): "not yet", never off;
/// - a circle centred on the tube's centre circle in the vertical plane ⊥
///   the meridian: its meridional rate is exactly 0, and it is off the
///   torus, so the incidence test's definite miss, the kernel's defect.
///
/// A Zero read here as a pose (`OffMargin::InBand`) reds the first two
/// as an incidence ending and the third as a lever.
#[test]
fn a_class_rate_read_zero_takes_the_incidence_door() {
    use geom_brep::recourse::Reading;
    use geom_brep::{IncidenceRead, OffMargin};
    let e = eps();
    let not_incidence = |what: &str, got: Result<(), PcurveCertifyError>| {
        let Err(err @ PcurveCertifyError::NoGeneralLane { .. }) = &got else {
            panic!("{what}: on the chart, no lane yet: {got:?}");
        };
        assert!(IncidenceRead::of(err).is_none(), "{what}: {err:?}");
    };
    let alpha = 0.5_f64;
    let g = Vec3::new(alpha.sin(), 0.0, alpha.cos());
    not_incidence(
        "a thin ellipse along a generator",
        chart_pcurve(
            &Curve3::Ellipse {
                center: Point3::origin() + g * 2.0,
                axis: Vec3::unit_y(),
                semi_u: 0.5,
                semi_v: 0.3 * e,
                u_ref: g,
            },
            &cone(alpha),
            band(),
        )
        .map(|_| ()),
    );
    let (lean, turn) = (4.125e-10_f64, 9.75e-10_f64);
    let up = (1.0 - lean * lean).sqrt();
    let (e1, e2) = (Vec3::new(-lean, 0.0, up), Vec3::unit_y());
    let th = (turn / up).asin();
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    not_incidence(
        "a leaning great circle at K < √2",
        chart_pcurve(
            &Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(up, 0.0, lean),
                radius: 1.0,
                u_ref: e1 * th.cos() + e2 * th.sin(),
            },
            &sphere,
            geom_core::Band::new(1e-9, 1.05e-9).unwrap(),
        )
        .map(|_| ()),
    );
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let got = chart_pcurve(
        &Curve3::Circle {
            center: Point3::new(2.0, 0.0, 0.0),
            axis: Vec3::unit_x(),
            radius: 0.5,
            u_ref: Vec3::unit_z(),
        },
        &torus,
        band(),
    );
    let Err(err @ PcurveCertifyError::CarrierOffChart { margin, .. }) = &got else {
        panic!("a circle ⊥ the meridian, off the torus: {got:?}");
    };
    assert!(
        !matches!(margin, Some(OffMargin::InBand(_)))
            && err
                .render(Reading::Build)
                .ends_with(geom_core::KERNEL_DEFECT_ENDING),
        "{err}"
    );
}

/// **A closed form minted at Zero, re-read with its class reading in
/// band, is undecided at check 4: typed, escalating, never a pass and
/// never a defect** (D3; only a definite contradiction of what is stored
/// is a defect). The image is a sphere parallel's, minted where the
/// axial gate reads Zero. The carrier it is re-read against is the same
/// circle tilted on the sphere so its axial reading is in the band:
/// still exactly on the sphere, so incidence reads it on.
///
/// The two lanes read different checks first, each pinned. At `f64`
/// (an inexact witness) certification runs check 3's schedule as the
/// closed form's cross-check before check 4, and its first sample reads
/// the tilt at the circle's radius in band: `MapResidual`. At `Interval`
/// (an exact witness) the schedule is not run, so check 4's
/// re-derivation reads the class in band: `ChartClass`, "not yet". The
/// class re-read is also pinned at the door check 4 re-derives through
/// (`pcurve_cache::check_4_class_rows::check_4s_re_derivation_of_an_in_band_class_is_undecided`).
#[test]
fn a_closed_form_re_read_with_its_class_in_band_is_undecided() {
    use geom_brep::PcurveCache;
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let (h, rho) = (0.6, 0.8);
    let parallel = Curve3::Circle {
        center: Point3::new(0.0, 0.0, h),
        axis: Vec3::unit_z(),
        radius: rho,
        u_ref: Vec3::unit_x(),
    };
    let image = chart_pcurve(&parallel, &sphere, band()).unwrap();
    // Tilt the circle's plane about `x` through the sphere's centre, so
    // it stays on the sphere; the axial reading is `ρ·sin θ`.
    let mid = 0.5 * (band().zero() + band().escalate());
    let theta = (mid / rho).asin();
    let (s, c) = theta.sin_cos();
    let n = Vec3::new(0.0, -s, c);
    let tilted = Curve3::Circle {
        center: Point3::origin() + n * h,
        axis: n,
        radius: rho,
        u_ref: Vec3::unit_x(),
    };
    let tau = core::f64::consts::TAU;
    let got = PcurveCache::certify(image, 0.0, tau, &tilted, &sphere, band());
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::Escalated {
                check: geom_brep::PcurveCheck::MapResidual,
                ..
            })
        ),
        "f64: check 3's cross-check reads the tilt first: {got:?}"
    );
    let at = <Interval as Real>::from_f64;
    let sphere_i = sphere.map_scalar(at);
    let image_i = chart_pcurve(&parallel.map_scalar(at), &sphere_i, band()).unwrap();
    let got = PcurveCache::certify(
        image_i,
        at(0.0),
        at(tau),
        &tilted.map_scalar(at),
        &sphere_i,
        band(),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::Escalated {
                check: geom_brep::PcurveCheck::ChartClass,
                ..
            })
        ),
        "Interval: check 4 re-reads the class in band: {got:?}"
    );
}

/// **An edge described on a chart whose general lane is unwritten for
/// its carrier ends lever-only "not yet" at the edge certificate, never
/// as the description's defect** (D3). The carrier is the on-cone
/// circle that is no rim (`a_near_rim_circle_tilted_inside_the_axial_band_is_on_the_cone`'s
/// wide cone): the mint refuses `NoGeneralLane`, and the edge
/// certificate carries it as its own `NoGeneralLane` under the
/// class check, not folded into `ChartImageUnavailable`'s defect. Read
/// at a build and at rest.
#[test]
fn an_edge_on_a_chart_with_no_general_lane_ends_not_yet() {
    use geom_brep::recourse::Reading;
    use geom_brep::{CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec};
    let e = eps();
    let alpha = 1.55_f64;
    let h = 1.0;
    let theta = 0.5 * e / (h * alpha.tan());
    let (centre, axis, u_ref, major, minor) = section(alpha, h, theta);
    let circle = Curve3::Circle {
        center: centre,
        axis,
        radius: (major * minor).sqrt(),
        u_ref,
    };
    let (keys, lookup) = crate::shared::surf::table(vec![cone(alpha)]);
    let (t0, t1) = (0.0, 1.0);
    let err = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: circle.clone(),
            param_start: t0,
            param_end: t1,
        },
        circle.eval(t0),
        circle.eval(t1),
        &lookup,
        1.0,
        band(),
        geom_brep::NurbsLane::certified(),
    )
    .expect_err("an on-cone circle that is no rim has no image yet");
    assert!(matches!(err, CertifyError::NoGeneralLane { .. }), "{err:?}");
    assert_eq!(
        err.decision().map(|(check, _)| check),
        Some(CertCheck::ChartClass)
    );
    for reading in [Reading::Build, Reading::AtRest] {
        let ending = err
            .ending(reading)
            .expect("a decision's refusal has an ending");
        assert!(
            ending.starts_with(geom_core::NOT_YET_ENDING),
            "{reading:?}: {ending}"
        );
        for word in ["defect", "tolerance", "Recourse:"] {
            assert!(!ending.contains(word), "{reading:?}: {word}: {ending}");
        }
    }
}

/// **The edge certificate ends a carrier's incidence by the door, and a
/// read in band by its lever** (D3, D4 ¶1 (iv)): a carrier off its
/// chart is the kernel's defect at a build and the stored culprit at
/// rest; one whose incidence test lands in band, and one read within the
/// band at a pose where the chart holds no curve of its kind, each end in
/// the geometry's lever worded for that read, never a defect. Who placed
/// the carrier moves none of it: the same carrier with and without a
/// declaration ends alike. At `f64`.
///
/// - **Off**: a line on the unit sphere, which holds none.
/// - **Undecided**: a great circle tilted off the axis (none of the
///   sphere's parallels or meridians) of radius `1 + mid`, its distance
///   from the sphere mid-band.
/// - **At a pose**: a circle of radius 2 about `(0, 0, 1)` tilted 30° off
///   a cone's axis, whose lowest point sits at the apex's height: its
///   nappe reading is a band-decided Zero (`cone_conic_incidence`), and
///   the apex plane meets the cone only at the apex.
#[test]
fn a_carrier_ends_its_incidence_at_the_edge_certificate_by_the_door() {
    use geom_brep::recourse::Reading;
    use geom_brep::{
        CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve,
        MappedSource, OffMargin,
    };
    use geom_core::{Affine3, Point2};
    let mid = 0.5 * (band().zero() + band().escalate());
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let declaration = MappedCurve::whole(MappedSource::ExtrudedPoint {
        point: Point2::new(0.0, 0.0),
        place: Affine3::identity(),
        vec: Vec3::unit_z(),
    });
    let line = |origin: Point3<f64>, dir: Vec3<f64>| Curve3::Line { origin, dir };
    // (read, face, carrier, span, the in-band read's lever, if any)
    type Row = (
        &'static str,
        Surface<f64>,
        Curve3<f64>,
        (f64, f64),
        Option<&'static str>,
    );
    let rows: [Row; 3] = [
        (
            "off",
            sphere.clone(),
            line(Point3::new(0.0, 0.0, 0.0), Vec3::unit_x()),
            (0.2, 0.6),
            None,
        ),
        (
            "undecided",
            sphere,
            Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(0.3_f64.sin(), 0.0, 0.3_f64.cos()),
                radius: 1.0 + mid,
                u_ref: Vec3::unit_y(),
            },
            (0.0, 1.0),
            Some("Recourse: the edge's curve is not shown on the face it bounds"),
        ),
        (
            "at a pose",
            cone(0.5),
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::new(0.5, 0.0, 0.75_f64.sqrt()),
                radius: 2.0,
                u_ref: Vec3::new(0.75_f64.sqrt(), 0.0, -0.5),
            },
            (0.0, 1.0),
            Some("Recourse: move the geometry so the edge sits clear of that pose"),
        ),
    ];
    for (what, surface, carrier, (t0, t1), lever) in rows {
        let (keys, lookup) = crate::shared::surf::table(vec![surface]);
        let certify = |declared: bool| {
            let chart = EdgeDescriptionSpec::chart(keys[0]);
            EdgeCurve::certify(
                EdgeCurveSpec {
                    description: if declared {
                        chart.declared_by(declaration)
                    } else {
                        chart
                    },
                    carrier: carrier.clone(),
                    param_start: t0,
                    param_end: t1,
                },
                carrier.eval(t0),
                carrier.eval(t1),
                &lookup,
                1.0,
                band(),
                geom_brep::NurbsLane::certified(),
            )
            .expect_err("the carrier is no curve of its chart here")
        };
        let refusal = certify(false);
        match (what, &refusal) {
            ("off", CertifyError::CarrierOffChart { margin, .. }) => {
                assert!(!matches!(margin, Some(OffMargin::InBand(_))), "{refusal:?}");
            }
            (
                "undecided",
                CertifyError::Escalated {
                    check: CertCheck::ChartIncidence,
                    ..
                },
            )
            | (
                "at a pose",
                CertifyError::CarrierOffChart {
                    margin: Some(OffMargin::InBand(_)),
                    ..
                },
            ) => {}
            _ => panic!("{what}: {refusal:?}"),
        }
        assert_eq!(
            refusal.decision().map(|(check, _)| check),
            Some(CertCheck::ChartIncidence),
            "{what}"
        );
        let declared = certify(true);
        for (reading, culprit) in [
            (Reading::Build, geom_core::KERNEL_DEFECT_ENDING),
            (Reading::AtRest, geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
        ] {
            let ending = refusal.ending(reading).unwrap();
            assert_eq!(
                declared.ending(reading).as_deref(),
                Some(ending.as_str()),
                "{what} {reading:?}: who placed the carrier moves no ending"
            );
            match lever {
                None => assert_eq!(ending, culprit, "{what} {reading:?}"),
                Some(lever) => {
                    assert!(ending.starts_with(lever), "{what} {reading:?}: {ending}");
                    for word in ["defect", "loosen", "tighten", "declared"] {
                        assert!(
                            !ending.contains(word),
                            "{what} {reading:?}: {word}: {ending}"
                        );
                    }
                }
            }
        }
    }
}
