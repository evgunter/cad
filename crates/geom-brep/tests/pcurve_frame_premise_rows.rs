//! **A chart's frame rounding is never read under its residual.** A
//! chart's frame is a rigid motion, so no row here builds one off its
//! convention: one row pins the rounding a frame carries at a far
//! placement, at each scalar's mint, and one the rounding a composed frame
//! carries at a large radius, which check 4 charges at `f64`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{PcurveCache, PcurveCertifyError, chart_pcurve};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use crate::shared::tol::band;

/// **No hand-made defect: an f64-normalized tilted axis at a far
/// placement.** `normalize(1, 2, 3)` is unit only to rounding (`δ` a
/// few ulps), and the bare `Interval` lift of an f64 body carries that
/// datum as the chart. At `1e9` m along the axis — `sym11`'s furthest
/// placement — the axial channel's `2δ·h` is the residual, and the
/// ruling's origin is itself rounded there by about an ulp of `1e9`.
/// Each scalar's mint either images it or reads its incidence in band
/// (the box's slide that far out is wider than any band); never a
/// definite miss. The control, on each image minted: whatever its
/// certificate answers, it is never under the residual's certified lower
/// bound at `t = 0` (the f64 dense sample is itself rounding at 1e9 m).
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
        frame: geom::Frame::new(Point3::origin(), axis, u_ref),
        radius,
    };
    assert!(
        cyl.representability_margins()
            .iter()
            .all(|m| m.margin > 0.0)
    );
    let ruling = Curve3::line(
        Point3::origin() + u_ref * radius + axis * far,
        geom::stated_direction(axis),
    );
    let lift = |x: f64| Interval::from_f64(x);
    let (si, ci) = (cyl.map_scalar(lift), ruling.map_scalar(lift));
    let by_incidence = |lane: &str, e: &PcurveCertifyError| {
        assert!(
            geom_brep::IncidenceRead::of(e).is_some_and(|read| !read.is_definite()),
            "{lane}: the mint refuses only by its incidence read in band: {e:?}"
        );
    };
    // The residual's certified lower bound at `t = 0`, at chart point `q`.
    let floor =
        |q: geom_core::Point2<Interval>| si.eval(q.x, q.y).distance(ci.eval(lift(0.0))).lo();
    let mut minted = 0;
    match chart_pcurve(&ruling, &cyl, band()) {
        Ok(pf) => {
            minted += 1;
            let q = pf.eval(0.0);
            let r = floor(geom_core::Point2::new(lift(q.x), lift(q.y)));
            let cert = PcurveCache::certify(pf, 0.0, 1.0, &ruling, &cyl, band());
            println!(
                "[rounded axis far] f64 eps {eps:e}: certify {cert:?}; residual lower bound {r:e}"
            );
            if let Ok(cache) = cert {
                let env = cache.certificate().envelope;
                assert!(
                    env >= r,
                    "UNSOUND at f64: certified envelope {env:e} under the residual's lower bound {r:e}"
                );
            }
        }
        Err(e) => by_incidence("f64", &e),
    }
    match chart_pcurve(&ci, &si, band()) {
        Ok(pi) => {
            minted += 1;
            let r = floor(pi.eval(lift(0.0)));
            let cert = PcurveCache::certify(pi, lift(0.0), lift(1.0), &ci, &si, band());
            println!(
                "[rounded axis far] interval eps {eps:e}: certify {cert:?}; residual lower bound {r:e}"
            );
            if let Ok(cache) = cert {
                let env = cache.certificate().envelope.hi();
                assert!(
                    env >= r,
                    "UNSOUND at Interval: certified envelope {env:e} under the residual's lower bound {r:e}"
                );
            }
        }
        Err(e) => by_incidence("Interval", &e),
    }
    assert!(minted > 0, "at least the f64 mint images the ruling");
}

/// `o + Σ cᵢ·vᵢ`, each coordinate exact ([`geom_core::exact::Expansion`]),
/// the [`a_composed_frames_rounding_is_charged_at_a_large_radius`] oracle's
/// point.
fn exact_point(o: Point3<f64>, terms: &[(f64, Vec3<f64>)]) -> [geom_core::exact::Expansion; 3] {
    use geom_core::exact::Expansion;
    let mut out = [o.x, o.y, o.z].map(|x| Expansion::product(x, 1.0));
    for &(c, v) in terms {
        for (k, x) in [v.x, v.y, v.z].into_iter().enumerate() {
            out[k].add_expansion(&Expansion::product(c, x));
        }
    }
    out
}

/// The distance between two exact points, rounded once per coordinate.
fn exact_distance(
    p: &[geom_core::exact::Expansion; 3],
    q: &[geom_core::exact::Expansion; 3],
) -> f64 {
    let d = |k: usize| {
        let mut e = p[k].clone();
        e.add_expansion(&q[k].negated());
        e.components().iter().sum::<f64>()
    };
    (d(0).powi(2) + d(1).powi(2) + d(2).powi(2)).sqrt()
}

/// **A composed frame's rounding is charged at a large radius** (check 4's
/// `EnvelopeTerm::Frame`, read off [`geom::Frame::rounding`]). Composition
/// adds rounding at every turn, so a cylinder whose frame is 256 random
/// turns composed stores axes a measurable distance `d` off the rotation
/// they hold, their polar factor
/// ([`geom_core::test_support::polar_distance`]). At a radius that levers
/// it to a tenth of the band's zero, the rim drawn on the frame's squared
/// twin
/// images on the chart, and check 4's envelope covers the residual,
/// measured exactly but for `f64` trigonometry (each `cos` and `sin`
/// within an ulp, `2u·R` a point; [`SLACK_ULPS`] of the radius covers
/// both points). Red where the frame's `f64` rounding is read as zero:
/// the envelope then falls under the stored chart's slip off its twin.
#[test]
fn a_composed_frames_rounding_is_charged_at_a_large_radius() {
    use geom_core::{RigidMotion, UnitVec3};
    /// The oracle's own error, in `u` of the radius: two points, each
    /// `cos` and `sin` within an ulp.
    const SLACK_ULPS: f64 = 4.0;
    let b = band();
    let mut seed = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        #[allow(clippy::cast_precision_loss)]
        let x = (seed >> 11) as f64 / (1u64 << 53) as f64;
        x
    };
    let mut motion = RigidMotion::<f64>::world();
    for _ in 0..256 {
        let axis = Vec3::new(next() - 0.5, next() - 0.5, next() - 0.5);
        let axis = UnitVec3::new(axis, "test_composed_axis", b).unwrap();
        let turn = RigidMotion::rotation(axis, 6.0 * next()).unwrap();
        motion = turn.compose(motion).unwrap();
    }
    let frame = geom::Frame::of_motion(&motion);
    let (to_axis, to_seam) = geom_core::test_support::polar_distance(&motion);
    let defect = to_axis.max(to_seam);
    assert!(
        defect > 2e-15,
        "256 turns carry a defect to charge: {defect:e}"
    );
    let radius = 0.1 * b.zero() / defect;
    let cyl = Surface::Cylinder { frame, radius };
    let square = frame.square();
    let twin = geom::Frame::new(frame.origin(), square.axis, square.u_ref);
    let rim = Curve3::Circle(geom::Circle::new(twin, radius));
    let image = chart_pcurve(&rim, &cyl, b).expect("the rim images on its cylinder");
    let (t0, t1) = (0.0, core::f64::consts::TAU);
    let cache = PcurveCache::certify(image.clone(), t0, t1, &rim, &cyl, b)
        .expect("the rim certifies on its cylinder");
    let envelope = cache.certificate().envelope;
    let on = |f: &geom::Frame<f64>, u: f64, v: f64| {
        let (s, c) = u.sin_cos();
        exact_point(
            f.origin(),
            &[
                (radius * c, f.u_ref()),
                (radius * s, f.v_ref()),
                (v, f.axis()),
            ],
        )
    };
    let mut residual = 0.0f64;
    for i in 0..=4096 {
        let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
        let q = image.eval(t);
        residual = residual.max(exact_distance(&on(&frame, q.x, q.y), &on(&twin, t, 0.0)));
    }
    let slack = SLACK_ULPS * 0.5 * f64::EPSILON * radius;
    println!(
        "defect {defect:e}, radius {radius:e}: envelope {envelope:e}, residual {residual:e}, \
         slack {slack:e}"
    );
    assert!(
        residual > 2.0 * slack,
        "the stored chart slips off its twin measurably"
    );
    assert!(
        envelope >= residual - slack,
        "UNSOUND: envelope {envelope:e} under the residual {residual:e} less its slack {slack:e}"
    );
}
