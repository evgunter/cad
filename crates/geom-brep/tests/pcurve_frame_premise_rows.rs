//! **Check 4's frame premise is metered, not assumed.** The per-arm
//! lemma on `EnvelopeStatement::MapResidualClosedForm` holds for an
//! orthonormal frame (`axis`, `u_ref` unit, `u_ref ⊥ axis`). Over an
//! interval scalar check 3 does not run, so the envelope is the whole
//! certified statement and the premise is load-bearing. Check 4 applies
//! the lemma on the chart's Gram–Schmidt twin and adds the two maps'
//! distance over the stored image (`EnvelopeTerm::Frame`), levered by
//! the radius and by the image's actual axial reach.
//!
//! These rows (PR 3812's review R2, claim 1) build a chart whose frame
//! is off its convention, mint the image `chart_pcurve` derives, certify
//! it at `Interval` (no schedule) and at `f64` (the schedule's
//! cross-check), and compare the certified envelope with a dense sample
//! of the true residual. Before the `Frame` term, the first two
//! certified at `Interval` with an envelope 4–8 orders under the
//! residual (546·ε and 5000·ε).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EnvelopeTerm, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck, chart_pcurve};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use crate::shared::tol::band;

fn dense_sup(p: &Pcurve<f64>, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    (0..=4096)
        .map(|k| {
            let t = t0 + (t1 - t0) * (f64::from(k) / 4096.0);
            let q = p.eval(t);
            s.eval(q.x, q.y).distance(c.eval(t))
        })
        .fold(0.0, f64::max)
}

/// Certify at the interval scalar (the box lane: no schedule) and at
/// f64 (the point lane: schedule as cross-check); report both, and the
/// dense residual of the f64 image.
fn run(name: &str, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> (bool, bool, f64, f64) {
    run_named(name, s, c, t0, t1).0
}

/// [`run`], with whether the interval refusal (if any) names the
/// frame's own term.
fn run_named(
    name: &str,
    s: &Surface<f64>,
    c: &Curve3<f64>,
    t0: f64,
    t1: f64,
) -> ((bool, bool, f64, f64), bool) {
    let eps = geom_core::Tol::witness().eps();
    let lift = |x: f64| Interval::from_f64(x);
    let (si, ci) = (s.map_scalar(lift), c.map_scalar(lift));
    let pi = chart_pcurve(&ci, &si, band()).expect("interval derivation");
    let at_box = PcurveCache::certify(pi, lift(t0), lift(t1), &ci, &si, band());
    if let Err(e) = &at_box {
        println!("[{name}] the interval refusal: {e:?}");
    }
    let pf = chart_pcurve(c, s, band()).expect("f64 derivation");
    let at_point = PcurveCache::certify(pf.clone(), t0, t1, c, s, band());
    let sup = dense_sup(&pf, s, c, t0, t1);
    let env = at_box
        .as_ref()
        .map(|cache| cache.certificate().envelope.hi())
        .unwrap_or(f64::NAN);
    println!(
        "[{name}] eps {eps:e}: interval certify {} (envelope hi {env:e}, samples {:?}); f64 certify \
         {}; dense residual {sup:e} = {:.1}·eps",
        if at_box.is_ok() { "OK" } else { "REFUSED" },
        at_box.as_ref().map(|c| c.certificate().samples).ok(),
        match &at_point {
            Ok(_) => "OK".to_string(),
            Err(e) => format!("REFUSED ({e})"),
        },
        sup / eps
    );
    let names_frame = matches!(
        at_box,
        Err(PcurveCertifyError::ResidualExceeded {
            check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::Frame),
            ..
        } | PcurveCertifyError::Escalated {
            check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::Frame),
            ..
        })
    );
    ((at_box.is_ok(), at_point.is_ok(), env, sup), names_frame)
}

/// **A cone whose `u_ref` is not ⊥ its axis.** The cone's at-rest
/// margins cover the half-angle only (`Surface::representability_margins`
/// has no frame rows for the cone), so this chart is valid at rest.
/// The image `chart_pcurve` derives from an exact rim sits on the chart
/// far more than ε from the rim, and the interval certificate, which
/// runs no schedule, refuses it on its `Frame` term.
#[test]
fn a_cone_frame_off_convention_refuses_over_the_box() {
    let eps = geom_core::Tol::witness().eps();
    let tilt = 1e3 * eps;
    let half_angle = 0.5_f64;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::new(1.0, 0.0, tilt),
    };
    assert!(
        cone.representability_margins(band())
            .iter()
            .all(|m| m.margin > 0.0),
        "the cone is at rest"
    );
    let h = 1.0;
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, h),
        axis: Vec3::unit_z(),
        radius: h * half_angle.tan(),
        u_ref: Vec3::unit_x(),
    };
    let ((box_ok, point_ok, env, sup), names_frame) =
        run_named("cone frame", &cone, &rim, 0.0, 3.0);
    assert!(
        sup > 100.0 * eps,
        "the probe is off the chart by far more than ε"
    );
    assert!(!point_ok, "the f64 cross-check sees the residual");
    assert!(
        !box_ok,
        "the interval certificate refuses (envelope {env:e}, true residual {sup:e})"
    );
    assert!(names_frame, "the refusal names `EnvelopeTerm::Frame`");
}

/// **A cylinder whose axis is long by `δ`, inside the at-rest margin**
/// (`|‖axis‖ − 1|·R ≤ ε` at the radius arm), carrying a ruling far up
/// the axis. The axial channel `v = w·axis` maps back through `axis·v`,
/// so the residual is `≈ 2δ·|h|` — scaled by the HEIGHT, which no
/// incidence term reads off a constant (`w_r`'s axial leak costs only
/// `‖w_r‖ − R ≈ 2δ²h²/R`), and which the at-rest margin never meters.
/// The `Frame` term levers the axis's defect by the image's `|v|`.
#[test]
fn a_long_axis_inside_the_rest_margin_refuses_a_far_ruling_over_the_box() {
    let eps = geom_core::Tol::witness().eps();
    let radius = 1.0;
    let delta = 0.25 * eps / radius;
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0 + delta),
        radius,
        u_ref: Vec3::unit_x(),
    };
    assert!(
        cyl.representability_margins(band())
            .iter()
            .all(|m| m.margin > 0.0),
        "the cylinder is at rest: {:?}",
        cyl.representability_margins(band())
            .iter()
            .map(|m| m.margin)
            .collect::<Vec<_>>()
    );
    let height = 1e4;
    let ruling = Curve3::Line {
        origin: Point3::new(radius, 0.0, height),
        dir: Vec3::unit_z(),
    };
    let ((box_ok, point_ok, env, sup), names_frame) =
        run_named("cylinder axis", &cyl, &ruling, 0.0, 1.0);
    assert!(
        sup > 100.0 * eps,
        "the probe is off the chart by far more than ε"
    );
    assert!(!point_ok, "the f64 cross-check sees the residual");
    assert!(
        !box_ok,
        "the interval certificate refuses (envelope {env:e}, true residual {sup:e})"
    );
    assert!(names_frame, "the refusal names `EnvelopeTerm::Frame`");
}

/// **No hand-made defect: an f64-normalized tilted axis at a far
/// placement.** `normalize(1, 2, 3)` is unit only to rounding (`δ` a
/// few ulps), and the bare `Interval` lift of an f64 body carries that
/// datum as the chart. At `1e9` m along the axis — `sym11`'s furthest
/// placement — the axial channel's `2δ·h` is the residual. The control:
/// whatever the interval certificate answers, it is never under the
/// residual's certified lower bound.
#[test]
fn a_rounded_unit_axis_at_a_far_placement_is_never_under_its_residual() {
    let eps = geom_core::Tol::witness().eps();
    let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
    let u_ref = axis.cross(Vec3::unit_x()).normalize();
    let radius = 1.0;
    let far = 1.0e9;
    println!(
        "axis length − 1 = {:e}, u_ref length − 1 = {:e}, tilt = {:e}",
        axis.norm() - 1.0,
        u_ref.norm() - 1.0,
        axis.dot(u_ref)
    );
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis,
        radius,
        u_ref,
    };
    assert!(
        cyl.representability_margins(band())
            .iter()
            .all(|m| m.margin > 0.0)
    );
    let ruling = Curve3::Line {
        origin: Point3::origin() + u_ref * radius + axis * far,
        dir: axis,
    };
    let (box_ok, _point_ok, env, _sup) = run("rounded axis far", &cyl, &ruling, 0.0, 1.0);
    // The f64 dense sample is itself rounding at 1e9 m; the interval
    // enclosure of the residual at t = 0 is the honest oracle.
    let lift = |x: f64| Interval::from_f64(x);
    let (si, ci) = (cyl.map_scalar(lift), ruling.map_scalar(lift));
    let pi = chart_pcurve(&ci, &si, band()).unwrap();
    let q = pi.eval(lift(0.0));
    let r = si.eval(q.x, q.y).distance(ci.eval(lift(0.0)));
    println!(
        "interval residual at t = 0: [{:e}, {:e}] = [{:.1}, {:.1}]·eps; certified envelope hi {env:e}",
        r.lo(),
        r.hi(),
        r.lo() / eps,
        r.hi() / eps
    );
    assert!(
        !box_ok || env >= r.lo(),
        "UNSOUND: certified envelope {env:e} under the residual's certified lower bound {:e}",
        r.lo()
    );
}
