//! **A slice of the arc-held ladder, as a row a job runs**
//! (`analysis/oneround/conic-pair-readers/ladder.rs`; the slow set of
//! `.config/nextest.toml`). The ladder's sections, minted by
//! `plane_cone_section` on their arcs at `a` = 1e4, 1e6 and 1e9, fat and
//! thin, the seam at the anchor, with each of its poses whose spans lie
//! in the session box (no coordinate past
//! [`geom_core::tolerance::SESSION_BOX_EXTENT`] at the spans' ends and
//! 4095 parameters between; the harness's predicate, not the kernel's
//! cover-based one): a line in the ellipse's plane and a unit circle
//! across it, through a point of the arc 0.3 m, −3 m or 1 km of arc from
//! the anchor, at 90°, 30° and 5° to it, read at `f64` and at
//! `Interval`, as built and moved `20ε` and `1000ε` apart.
//!
//! **The ladder's acceptance, as assertions**: every pose read as built
//! meets within the band's escalation of the point it was built through,
//! and every twin apart reads no meeting (0 wrong). A pose may refuse only
//! where [`REFUSING`] lists it; the ladder's join showed each listed one
//! refused or read wrongly on the base, so no pose the base read rightly
//! is wrong or refused here (0 right-to-wrong, 0 right-to-refused). The
//! truth is the pose's construction: the point `point_at` gives, which
//! beside the anchor holds the stored-numbers curve to its coordinates'
//! rounding, and the twin's shift, past the escalation.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Conic, Curve3, Ellipse, Frame, Surface};
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

use super::{MeetFault, Span, meet};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The section of the cone of half-angle `alpha` about a generic axis,
/// apex `apex`, by the plane tilted `tau` from the axis, `gap` up it.
fn mint(alpha: f64, tau: f64, gap: f64, apex: Point3<f64>) -> Option<Ellipse<f64>> {
    let ez = Vec3::new(0.36, 0.48, 0.8);
    let ex = Vec3::new(0.8, -0.6, 0.0);
    let ey = ez.cross(ex);
    let cone = Surface::Cone {
        frame: Frame::new(apex, ez, ex),
        half_angle: alpha,
    };
    let n = ex * tau.sin() + ez * tau.cos();
    let plane = Surface::Plane {
        frame: Frame::new(apex + ez * gap, n, ey),
    };
    let reach = crate::Reach::fixture(apex, 10.0);
    match crate::intersect::plane_cone_section(&plane, &cone, &reach, band()) {
        Ok(crate::intersect::PlaneConeSection::TiltedEllipse(Curve3::Ellipse(e))) => Some(e),
        _ => None,
    }
}

/// A minted section with major semi-axis about `target`: `fat` holds the
/// semi-axes' ratio near 0.3, else the apex gap is 1 m. The anchor stands
/// near `(1.3, −0.7, 0.4)`.
fn minted(target: f64, fat: bool) -> Ellipse<f64> {
    const ALPHA: f64 = 1.3;
    let o = Point3::new(0.0, 0.0, 0.0);
    let major = |e: &Ellipse<f64>| e.semi_axes().0.max(e.semi_axes().1);
    let ratio = |e: &Ellipse<f64>| e.semi_axes().0.min(e.semi_axes().1) / major(e);
    let (mut lo, mut hi) = (1e-3, core::f64::consts::FRAC_PI_2 - ALPHA - 1e-12);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let grow = match mint(ALPHA, mid, 1.0, o) {
            Some(e) if fat => ratio(&e) > 0.3,
            Some(e) => major(&e) < target,
            None => false,
        };
        if grow {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let unit = mint(ALPHA, lo, 1.0, o).unwrap();
    let gap = if fat { target / major(&unit) } else { 1.0 };
    let first = mint(ALPHA, lo, gap, o).unwrap();
    let anchor = first.point_at(0.0);
    mint(ALPHA, lo, gap, o + (Point3::new(1.3, -0.7, 0.4) - anchor)).unwrap()
}

/// The harness's box predicate on a span's points.
fn span_in_box(c: &Curve3<f64>, (lo, hi): (f64, f64)) -> bool {
    let e = geom_core::tolerance::SESSION_BOX_EXTENT;
    (0..=4096).all(|i| {
        let p = c.eval(lo + (hi - lo) * f64::from(i) / 4096.0);
        p.x.abs() <= e && p.y.abs() <= e && p.z.abs() <= e
    })
}

/// A refusal's key: the band's zero, the door, `a`'s target, fat, the
/// metres of arc, γ in degrees, the shift in ε, the lane and the row.
type Listed = (
    f64,
    &'static str,
    f64,
    bool,
    f64,
    f64,
    f64,
    &'static str,
    &'static str,
);

/// The row a refusal names.
fn named(fault: &MeetFault) -> Option<&'static str> {
    match fault {
        MeetFault::Unsettled(crate::roots::Unsettled::Tangency) => Some("Tangency"),
        MeetFault::Unsettled(crate::roots::Unsettled::Meter { row }) => Some(row),
        MeetFault::Escalated(i) | MeetFault::Touch(i) => i.predicate,
        _ => None,
    }
}

/// **The in-box poses that refuse, owed nothing**, with their reasons
/// (`arc_held_rows`' [`super::arc_held_rows`] reasons, the same classes):
/// every one refused or read wrongly on the base (the ladder's join,
/// `analysis/oneround/conic-pair-readers/ladder/join-base-head.txt`). A run
/// at a band not listed here reads no list. A listed pose that answers is
/// red too.
const REFUSING: &[(Listed, &str)] = &[
    (
        (
            1e-12,
            "circle",
            1e4,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e4,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e4,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            -3.0,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            -3.0,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            0.3,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            0.3,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            -3.0,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            -3.0,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            0.3,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            0.3,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "circle",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_walk_pair_gap",
        ),
        super::arc_held_rows::UNREAD,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            true,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_walk_pair_gap",
        ),
        super::arc_held_rows::UNREAD,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            true,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            true,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            true,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e4,
            true,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_walk_pair_gap",
        ),
        super::arc_held_rows::UNREAD,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            true,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            true,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            true,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            true,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            true,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e6,
            true,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_walk_pair_gap",
        ),
        super::arc_held_rows::UNREAD,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            true,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            true,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            true,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            true,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            true,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-12,
            "line",
            1e9,
            true,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            -3.0,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            -3.0,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            0.3,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            0.3,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            -3.0,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            -3.0,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            0.3,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            0.3,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "circle",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-9,
            "line",
            1e6,
            true,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e6,
            true,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e9,
            true,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e9,
            true,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e9,
            true,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e9,
            true,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e9,
            true,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-9,
            "line",
            1e9,
            true,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            -3.0,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            -3.0,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            -3.0,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            0.3,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            0.3,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e6,
            false,
            0.3,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            -3.0,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            -3.0,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            -3.0,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            -3.0,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            0.3,
            5.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            0.3,
            5.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            0.3,
            30.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            0.3,
            30.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "circle",
            1e9,
            false,
            0.3,
            90.0,
            0.0,
            "f64",
            "curve_pair_pose_basin",
        ),
        super::arc_held_rows::TIP,
    ),
    (
        (
            1e-6,
            "line",
            1e9,
            true,
            -3.0,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
    (
        (
            1e-6,
            "line",
            1e9,
            true,
            0.3,
            90.0,
            0.0,
            "Interval",
            "curve_pair_pose_level_arc",
        ),
        super::arc_held_rows::LEVEL_ARC,
    ),
];

/// The ladder's in-box slice at this run's band.
#[test]
fn the_ladders_in_box_slice_reads_every_pose_rightly() {
    let band = band();
    let (eps, esc) = (band.zero(), band.escalate());
    let iv = <Interval as Real>::from_f64;
    let imid = |x: Interval| 0.5 * (geom_core::Bounds::lo(x) + geom_core::Bounds::hi(x));
    let (mut failed, mut met, mut read) = (Vec::new(), Vec::new(), 0);
    for target in [1e4, 1e6, 1e9] {
        for fat in [true, false] {
            let e = minted(target, fat);
            let b = e.semi_axes().0.min(e.semi_axes().1);
            let carrier = Curve3::Ellipse(e);
            let carrier_i = carrier.map_scalar(iv);
            for gamma in [90.0_f64, 30.0, 5.0] {
                for s in [0.3, -3.0, 1000.0] {
                    let theta: f64 = s / b;
                    if theta.abs() >= 0.3 {
                        continue;
                    }
                    let half = (10.0 / b).max(2.0 * theta.abs()).min(0.2);
                    let span_e = (theta - half, theta + half);
                    let (t, tan, n) = (
                        e.point_at(theta),
                        e.derivative_at(theta).normalize(),
                        e.axis(),
                    );
                    let across = n.cross(tan).normalize();
                    let g = gamma.to_radians();
                    for shift in [0.0, 20.0, 1000.0] {
                        let lift = shift * eps;
                        // The line door, in the ellipse's plane through `t`,
                        // lifted off it by the shift.
                        let d = (tan * g.cos() + across * g.sin()).normalize();
                        let line =
                            Curve3::line(t - d * 2.0 + n * lift, geom_core::UnitVec3::checked(d));
                        // The circle door, across the arc through `t`, moved
                        // along the pair's common normal.
                        let w = (across * 0.7_f64.cos() + n * 0.7_f64.sin()).normalize();
                        let normal = (tan * g.sin() + w * g.cos()).normalize();
                        let in_plane = normal.cross(w.cross(tan)).normalize();
                        let common = tan.cross(normal.cross(in_plane)).normalize();
                        let circle =
                            Curve3::circle(t - in_plane + common * lift, normal, 1.0, in_plane);
                        for (door, other, span_o, first) in [
                            ("line", &line, (0.0, 4.0), true),
                            ("circle", &circle, (-1.0, 1.0), false),
                        ] {
                            if !(span_in_box(other, span_o) && span_in_box(&carrier, span_e)) {
                                continue;
                            }
                            let other_i = other.map_scalar(iv);
                            let (sx, sy) = (Span::new(other, span_o), Span::new(&carrier, span_e));
                            let (ix, iy) = (
                                Span::new(&other_i, (iv(span_o.0), iv(span_o.1))),
                                Span::new(&carrier_i, (iv(span_e.0), iv(span_e.1))),
                            );
                            let (fx, fy, gx, gy) = if first {
                                (&sx, &sy, &ix, &iy)
                            } else {
                                (&sy, &sx, &iy, &ix)
                            };
                            let at_f = meet(fx, fy, band)
                                .map(|m| m.points.iter().map(|p| p.point).collect::<Vec<_>>())
                                .map_err(|r| r.fault);
                            let at_i = meet(gx, gy, band)
                                .map(|m| {
                                    m.points
                                        .iter()
                                        .map(|p| {
                                            Point3::new(
                                                imid(p.point.x),
                                                imid(p.point.y),
                                                imid(p.point.z),
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .map_err(|r| r.fault);
                            for (lane, r) in [("f64", at_f), ("Interval", at_i)] {
                                read += 1;
                                let what = format!(
                                    "{door} a {target:e} {} s {s} γ {gamma}° shift {shift}ε at {lane}",
                                    if fat { "fat" } else { "thin" }
                                );
                                match r {
                                    Ok(pts)
                                        if shift == 0.0
                                            && pts.iter().any(|q| (*q - t).norm() <= esc) => {}
                                    Ok(pts) if shift > 0.0 && pts.is_empty() => {}
                                    Ok(pts) => failed.push(format!(
                                        "{what}: WRONG, met at {pts:?} (built through {t:?})"
                                    )),
                                    Err(fault) => {
                                        let key = named(&fault).map(|row| {
                                            (eps, door, target, fat, s, gamma, shift, lane, row)
                                        });
                                        match key.and_then(|k| REFUSING.iter().find(|l| l.0 == k)) {
                                            Some(l) => met.push(l.0),
                                            None => failed.push(format!(
                                                "{what}: refused, not listed: {key:?} {fault:?}"
                                            )),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let stale: Vec<_> = REFUSING
        .iter()
        .filter(|l| l.0.0 == eps && !met.contains(&l.0))
        .collect();
    assert!(read > 0, "no pose read");
    assert!(
        failed.is_empty() && stale.is_empty(),
        "{} of {read} readings failed:\n{}\nlisted as refusing, but answered: {stale:?}",
        failed.len(),
        failed.join("\n")
    );
}
