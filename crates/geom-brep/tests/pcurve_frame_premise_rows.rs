//! **A rounded unit axis far out is never read under its residual.**
//! A chart's frame is a rigid motion, so no row here builds one off its
//! convention; the one row pins the rounding a frame carries at a far
//! placement, at each scalar's mint.

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
        cyl.representability_margins(band())
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
