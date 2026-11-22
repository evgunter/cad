//! **The spiric × plane door** ([`super::Door::SpiricPlane`]): a spiric
//! span against a plane, by its *section reduction* (C5's curve × face
//! paragraph, `crates/geom-brep/README.md`).
//!
//! A spiric span is an arc of one oval of its torus `T` cut by its own
//! plane `P`. A plane `Q` not parallel to `P` meets it only on the line
//! `L = P ∩ Q` (its pose, [`super::pose()`], reads `L` and excludes
//! `Lies`), and a point of `L` is on the oval where it is on `T` and on
//! the oval's side of the axis's projection on `P`. So the door reads `L`
//! against `T` by the line × torus door's ladder
//! ([`super::line_quadric::line_torus_roots`]), keeps each root on the
//! oval's side (`spiric_plane_oval_side`, the root's offset along
//! `m = axis × û` from the axis's projection), and names each kept root by
//! its tube angle, the spiric's own parameter, within half a turn of the
//! span's middle.
//!
//! The ovals stand apart by at least `2·√((R − r)² − offset²)`, which is
//! past the band wherever the regime is (the pose's rung), so the side
//! decision is never near a root. The ladder reads the whole line, so a
//! graze of `L` on the other oval escalates the reading too.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Sign};

use super::line_quadric::line_torus_roots;
use crate::curve_pair::{MeetFault, Span};
use crate::roots::{RootRefusal, Roots};

/// The row a root of the line is decided on the oval's side under.
const OVAL_SIDE: &str = "spiric_plane_oval_side";

/// **The spiric × plane method**: the span's parameters at which `line`
/// (the pose's `P ∩ Q`) meets its oval, each within half a turn of the
/// span's middle.
///
/// # Errors
///
/// [`MeetFault::Unsettled`] where the ladder's count is uncertain (a
/// graze of either oval); [`MeetFault::Escalated`] on a ladder rung or a
/// root's side in band; [`MeetFault::Desync`] on a carrier not a spiric.
pub(super) fn spiric_plane_method<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    band: Band,
) -> Result<Vec<T>, MeetFault> {
    let &geom::Curve3::Spiric {
        frame,
        major_radius,
        minor_radius,
        ..
    } = span.carrier()
    else {
        return Err(MeetFault::Desync);
    };
    let (center, axis) = (frame.origin(), frame.axis());
    let m = axis.cross(frame.u_ref());
    let found = line_torus_roots(
        line,
        geom::Line {
            origin: center,
            dir: axis,
        },
        major_radius,
        minor_radius,
        band,
    )
    .map_err(|refusal| match refusal {
        RootRefusal::Unsettled(why) => MeetFault::Unsettled(why),
        RootRefusal::Escalated(diag) | RootRefusal::Touch(diag) => MeetFault::Escalated(diag),
        RootRefusal::Fault(fault) => MeetFault::Contradiction {
            readings: fault.readings(),
        },
    })?;
    let ts = match found {
        Roots::Certified(found) => found.roots().to_vec(),
        Roots::Miss => Vec::new(),
    };
    let (t0, t1) = span.params();
    let mid = geom::mid_param(t0, t1);
    let mut out = Vec::new();
    for t in ts {
        let y = line.origin + line.dir * t;
        let w = y - center;
        match decide(OVAL_SIDE, Margin::of(w.dot(m)), band).map_err(MeetFault::Escalated)? {
            Sign::Positive => {}
            Sign::Negative => continue,
            Sign::Zero => {
                return Err(MeetFault::Contradiction {
                    readings: "a root of the line on the torus at the axis's projection on the \
                               spiric's plane, which the two-oval regime keeps clear of the ovals",
                });
            }
        }
        let (h, rho) = geom::Line {
            origin: center,
            dir: axis,
        }
        .height_and_distance(y);
        out.push(geom::Azimuth::of(h, rho - major_radius).near(mid));
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom::{Curve3, Frame, Surface};
    use geom_core::{Band, Point3, Vec3};

    use crate::curve_face::{CrossWhy, Pose, PoseRefusal, cross};
    use crate::curve_pair::{MeetFault, Span};

    const R: f64 = 5.0;
    const MINOR: f64 = 1.5;
    const OFFSET: f64 = 1.0;
    /// The span every row reads: most of a turn, its ends clear of every
    /// root a row's plane has.
    const SPAN: (f64, f64) = (-3.0, 3.0);

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// The oval of the torus `(R, r)` about `z` cut by `x = OFFSET`, on the
    /// `+y` side.
    fn oval() -> Curve3<f64> {
        Curve3::spiric(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            R,
            MINOR,
            OFFSET,
            band(),
        )
        .unwrap()
    }

    fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
        let normal = normal.normalize();
        let seed = if normal.x.abs() < 0.9 {
            Vec3::unit_x()
        } else {
            Vec3::unit_y()
        };
        let u_ref = (seed - normal * seed.dot(normal)).normalize();
        Surface::Plane {
            frame: Frame::new(origin, normal, u_ref),
        }
    }

    /// The oracle: the span's sign changes of the oval's height above the
    /// plane on a fine sampling, each bisected to the bottom.
    fn oracle(curve: &Curve3<f64>, origin: Point3<f64>, normal: Vec3<f64>) -> Vec<f64> {
        let normal = normal.normalize();
        let f = |v: f64| (curve.eval(v) - origin).dot(normal);
        let n = 20_000;
        let at = |i: i32| SPAN.0 + (SPAN.1 - SPAN.0) * f64::from(i) / f64::from(n);
        let mut roots = Vec::new();
        for i in 0..n {
            let (mut a, mut b) = (at(i), at(i + 1));
            if f(a).signum() == f(b).signum() {
                continue;
            }
            for _ in 0..80 {
                let m = 0.5 * (a + b);
                if f(m).signum() == f(a).signum() {
                    a = m;
                } else {
                    b = m;
                }
            }
            roots.push(0.5 * (a + b));
        }
        roots
    }

    /// The door's meetings' parameters, ascending, against the oracle's.
    fn meets_as_oracle(
        label: &str,
        (origin, normal): (Point3<f64>, Vec3<f64>),
        count: usize,
    ) -> Pose<f64> {
        let curve = oval();
        let got = cross(&Span::new(&curve, SPAN), &plane(origin, normal), band())
            .unwrap_or_else(|refusal| panic!("{label}: refused {refusal:?}"));
        let ts: Vec<f64> = got
            .meetings
            .iter()
            .map(|m| match m.on_span {
                crate::curve_face::OnSpan::Inside { t } => t,
                crate::curve_face::OnSpan::End(end) => panic!("{label}: a root at {end:?}"),
            })
            .collect();
        let want = oracle(&curve, origin, normal);
        assert_eq!(
            want.len(),
            count,
            "{label}: the oracle's own count {want:?}"
        );
        assert_eq!(
            ts.len(),
            want.len(),
            "{label}: door {ts:?}, oracle {want:?}"
        );
        for (t, w) in ts.iter().zip(&want) {
            assert!(
                (t - w).abs() < 1e-7,
                "{label}: door {ts:?}, oracle {want:?}"
            );
            let p = curve.eval(*t);
            let off = (p - origin).dot(normal.normalize());
            assert!(
                off.abs() < 1e-9,
                "{label}: the meeting at {t} is {off} off the plane"
            );
        }
        got.pose
    }

    /// **Isolated**: planes across the oval, their line parallel to the
    /// axis, oblique, and square to it across both ovals (four roots on
    /// the torus, two on this oval), each read as the oracle reads it.
    #[test]
    fn a_plane_across_the_oval_meets_it_where_the_oracle_does() {
        for (label, origin, normal, count) in [
            ("y = 5", Point3::new(0.0, 5.0, 0.0), Vec3::unit_y(), 2),
            (
                "oblique",
                Point3::new(0.0, 5.0, 0.3),
                Vec3::new(0.2, 0.6, 0.8),
                2,
            ),
            (
                "shallow",
                Point3::new(3.0, 4.0, 0.5),
                Vec3::new(0.3, -0.1, 1.0),
                2,
            ),
            (
                "z = 0.5, across both ovals",
                Point3::new(0.0, 0.0, 0.5),
                Vec3::unit_z(),
                2,
            ),
        ] {
            let pose = meets_as_oracle(label, (origin, normal), count);
            assert!(matches!(pose, Pose::Isolated(_)), "{label}: {pose:?}");
        }
    }

    /// **The oval's side**: the line `P ∩ Q` meets the torus on the
    /// other oval, across the axis's projection, at roots that are no
    /// meeting of this one.
    #[test]
    fn a_line_across_the_mirror_oval_alone_meets_none_of_this_one() {
        let pose = meets_as_oracle("y = −5", (Point3::new(0.0, -5.0, 0.0), Vec3::unit_y()), 0);
        assert!(matches!(pose, Pose::Isolated(_)), "y = −5: {pose:?}");
        let pose = meets_as_oracle(
            "past the tube",
            (Point3::new(0.0, 0.0, 2.0), Vec3::unit_z()),
            0,
        );
        assert!(matches!(pose, Pose::Isolated(_)), "past the tube: {pose:?}");
    }

    /// **Lies and Apart**: the spiric's own plane in either sense, and a
    /// parallel plane off it, read by the tilt and then the offset.
    #[test]
    fn a_plane_parallel_to_the_spirics_lies_or_stands_apart() {
        let curve = oval();
        let read = |origin, normal| {
            cross(&Span::new(&curve, SPAN), &plane(origin, normal), band())
                .unwrap()
                .pose
        };
        for normal in [Vec3::unit_x(), -Vec3::unit_x()] {
            let got = read(Point3::new(OFFSET, 3.0, -2.0), normal);
            assert!(
                matches!(got, Pose::Lies),
                "its own plane {normal:?}: {got:?}"
            );
        }
        let got = read(Point3::new(OFFSET + 0.5, 0.0, 0.0), Vec3::unit_x());
        assert!(
            matches!(got, Pose::Apart),
            "a parallel plane off it: {got:?}"
        );
        // Tilted inside the band over the oval's reach, off it by more.
        let got = read(
            Point3::new(OFFSET + 0.5, 0.0, 0.0),
            Vec3::new(1.0, 1e-12, 0.0),
        );
        assert!(
            matches!(got, Pose::Apart),
            "a tilt in band, off it: {got:?}"
        );
    }

    /// **Tangent**: a plane touching the oval at its top, where the line
    /// `P ∩ Q` grazes the torus's top circle: the ladder's count is
    /// uncertain there, so the reading refuses typed over the span and
    /// claims no crossing.
    #[test]
    fn a_plane_touching_the_oval_refuses_its_graze() {
        let curve = oval();
        let got = cross(
            &Span::new(&curve, SPAN),
            &plane(Point3::new(0.0, 0.0, MINOR), Vec3::unit_z()),
            band(),
        );
        let Err(refusal) = got else {
            panic!("a graze reads as a crossing: {got:?}");
        };
        assert!(refusal.found.is_empty(), "{refusal:?}");
        assert!(
            refusal.windows.iter().all(|w| matches!(
                w.why,
                CrossWhy::Method {
                    fault: MeetFault::Unsettled(_) | MeetFault::Escalated(_),
                    ..
                }
            )),
            "{refusal:?}"
        );
    }

    /// **The convention**: a spiric spelled past its mint outside the
    /// two-oval regime is no carrier the door reads; it refuses as a
    /// fault before any pose.
    #[test]
    fn a_spiric_outside_its_two_oval_convention_refuses_as_a_fault() {
        for offset in [R - MINOR + 0.5, 0.0] {
            let curve = Curve3::Spiric {
                frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                major_radius: R,
                minor_radius: MINOR,
                offset,
            };
            let got = cross(
                &Span::new(&curve, SPAN),
                &plane(Point3::new(0.0, 5.0, 0.0), Vec3::unit_y()),
                band(),
            );
            let Err(refusal) = got else {
                panic!("offset {offset}: read {got:?}");
            };
            assert!(
                matches!(
                    refusal.windows[..],
                    [crate::curve_face::Undecided {
                        why: CrossWhy::Pose(PoseRefusal::Fault(_)),
                        ..
                    }]
                ),
                "offset {offset}: {refusal:?}"
            );
        }
    }
}
