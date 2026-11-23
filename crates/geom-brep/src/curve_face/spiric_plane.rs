//! **The spiric × plane door** ([`super::Door::SpiricPlane`]): a spiric
//! span against a plane, by its *section reduction* (C5's curve × face
//! paragraph, `crates/geom-brep/README.md`).
//!
//! A spiric span is an arc of one oval of its torus `T` cut by its own
//! plane `P`. A plane `Q` not parallel to `P` meets it only on the line
//! `L = P ∩ Q`, and a point of `L` is on the oval where it is on `T` and
//! on the oval's side of the axis's projection on `P`. The pose
//! ([`super::pose()`]) excludes `Lies` and reads `L` in `P`'s own frame,
//! so `L` lies in `P` to a rounding at any tilt and stands off the exact
//! line by no more than the planes' own conditioning, `1/tilt`.
//!
//! The door reads `L` against `T` by the line × torus door's ladder
//! ([`super::line_quadric::line_torus_roots`]). It keeps each root on the
//! oval's side (`spiric_plane_oval_side`, the root's offset along
//! `m = axis × û` from the axis's projection), and names each kept root by
//! the spiric's own parameter, within half a turn of the span's middle.
//! A count the band cannot hold refuses: the count read again on the
//! lines of `P` the band's escalation off `Q` either way, and a graze of
//! `L` on either oval, which the ladder reads whole.
//!
//! Those two lines are `P ∩ (Q ± escalate·n̂)`: `L` moved `escalate/s`
//! across itself in `P`, `s` the planes' tilt sine, since a move `d`
//! across `L` in `P` is a height `d·s` off `Q`. Where exactly one extreme
//! of the oval's height off `Q` lies within the escalation of zero, a
//! miss or a dip through `Q`, the count on one of the two lines differs,
//! or that line grazes the oval. Two extremes within it can leave all
//! three counts alike: an oval wholly inside the band on one side of `Q`
//! reads no meeting on each line and is answered as a miss.
//!
//! `s` is the line's own `√(α² + β²)`, the `|û × n̂|` of the pose's
//! parallel rung to a rounding. That rung has `|û × n̂|` times the radius
//! of the span's cover ball past the band, so the move is under that
//! radius to a rounding at any lean.
//!
//! The ovals stand at least `√((R − r)² − offset²)` from the axis's
//! projection, past the band wherever the regime is (the pose's rung),
//! and `L` and its roots lie in `P` to a rounding, so the side decision is
//! never near a root.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::line_quadric::line_torus_roots;
use crate::curve_pair::{MeetFault, Span};
use crate::roots::{RootRefusal, Roots, Unsettled};

/// The row a root of the line is decided on the oval's side under.
const OVAL_SIDE: &str = "spiric_plane_oval_side";

/// **The spiric × plane method**: the span's parameters at which `line`
/// (the pose's `P ∩ Q`, with `across` its unit square to it in `P` and
/// `sine` the planes' tilt sine) meets its oval, each within half a turn
/// of the span's middle.
///
/// The oval's meetings with the line, and with the lines of `P` the
/// band's escalation off `Q` either way (the line moved `escalate/sine`
/// across itself): where the count on either differs (one extreme of the
/// oval's height off `Q` within the escalation of zero: a graze, a near
/// miss or a shallow dip), the reading refuses as a tangency rather than
/// answer a count the band cannot hold.
///
/// Each refusal is the whole span's: a count the shifted lines do not
/// keep ([`Unsettled::Tangency`]), one the ladder does not certify (a
/// graze of either oval), a ladder rung or a root's side in band.
///
/// # Errors
///
/// A kernel fault's readings: a carrier not a spiric, or readings that
/// contradict.
pub(super) fn spiric_plane_method<T: Decide>(
    span: &Span<'_, T>,
    (line, across, sine): (geom::Line<T>, Vec3<T>, T),
    band: Band,
) -> Result<super::cross::Walked<T>, &'static str> {
    use super::cross::Walked;
    match spiric_roots(span, (line, across, sine), band) {
        Ok(found) => Ok(Walked::found(found)),
        Err(MeetFault::Contradiction { readings }) => Err(readings),
        Err(MeetFault::Desync) => {
            Err("the spiric × plane method was handed a carrier that is not a spiric")
        }
        Err(why) => Ok(Walked::whole(span.params(), why)),
    }
}

/// [`spiric_plane_method`]'s reading.
fn spiric_roots<T: Decide>(
    span: &Span<'_, T>,
    (line, across, sine): (geom::Line<T>, Vec3<T>, T),
    band: Band,
) -> Result<Vec<T>, MeetFault> {
    let found = on_the_oval(span, line, band)?;
    let shift = T::from_f64(band.escalate()) / sine;
    for shift in [shift, T::zero() - shift] {
        let moved = geom::Line {
            origin: line.origin + across * shift,
            dir: line.dir,
        };
        if on_the_oval(span, moved, band)?.len() != found.len() {
            return Err(MeetFault::Unsettled(Unsettled::Tangency));
        }
    }
    let (t0, t1) = span.params();
    let mid = geom::mid_param(t0, t1);
    found.into_iter().map(|y| param_of(span, y, mid)).collect()
}

/// **Where `line`, a line of the span's own plane, meets the span's oval**:
/// its roots against the torus, each decided on the oval's side of the
/// axis's projection (`spiric_plane_oval_side`). The line lies in the
/// plane, so a root's offset along `m = â × û` from the axis's projection
/// is its distance from that projection, and the ovals stand at least
/// `√((R − r)² − offset²)` from it, past the band wherever the regime is:
/// every root is placed on one oval, and a side in band is a contradiction.
fn on_the_oval<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    band: Band,
) -> Result<Vec<Point3<T>>, MeetFault> {
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
            dir: frame.axis_unit(),
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
    let mut out = Vec::new();
    for t in ts {
        let y = line.origin + line.dir.get() * t;
        match decide(OVAL_SIDE, Margin::of((y - center).dot(m)), band)
            .map_err(MeetFault::Escalated)?
        {
            Sign::Positive => out.push(y),
            Sign::Negative => {}
            Sign::Zero => {
                return Err(MeetFault::Contradiction {
                    readings: "a root of the line on the torus at the axis's projection on the \
                               spiric's plane, which the two-oval regime keeps clear of the ovals",
                });
            }
        }
    }
    Ok(out)
}

/// The span's parameter at `y`, a point of its oval, within half a turn
/// of `mid`: the carrier's own reading, which a spiric gives at every
/// point, so none is a fault.
fn param_of<T: Decide>(span: &Span<'_, T>, y: Point3<T>, mid: T) -> Result<T, MeetFault> {
    span.carrier()
        .param_near(y, mid)
        .ok_or(MeetFault::Contradiction {
            readings: "a spiric carrier read no parameter at a point of its oval",
        })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, PI};
    use geom::{Curve3, Frame, Surface};
    use geom_core::{Band, Point3, Vec3};

    use crate::curve_face::{CrossWhy, Pose, PoseRefusal, cross};
    use crate::curve_pair::{MeetFault, Span};
    use crate::roots::Unsettled;

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
        (0..n)
            .filter_map(|i| sign_change(&f, at(i), at(i + 1)))
            .collect()
    }

    /// `f`'s sign change in `(a, b)` bisected to the bottom, where its
    /// signs at the two ends differ.
    fn sign_change(f: &impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> Option<f64> {
        if f(a).signum() == f(b).signum() {
            return None;
        }
        for _ in 0..80 {
            let m = 0.5 * (a + b);
            if f(m).signum() == f(a).signum() {
                a = m;
            } else {
                b = m;
            }
        }
        Some(0.5 * (a + b))
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
        // A tilt whose lever over the span reads in the zero band is the
        // parallel arm's: off it by more, apart.
        let got = read(
            Point3::new(OFFSET + 0.5, 0.0, 0.0),
            Vec3::new(1.0, 1e-12, 0.0),
        );
        assert!(
            matches!(got, Pose::Apart),
            "a tilt in the zero band, off it: {got:?}"
        );
    }

    /// The radius of the span's cover ball: the parallel rung's lever.
    fn lever() -> f64 {
        oval()
            .cover_over(geom::cover::ParamWindow::new(SPAN.0, SPAN.1))
            .unwrap()
            .ball()
            .1
    }

    /// **A tilt in the parallel rung's gap**: levered over the span to
    /// between the band's zero and its escalation. Clear of the plane by
    /// more than that, apart; through it, the tilt escalates on its rung,
    /// never a definite class.
    #[test]
    fn a_tilt_in_the_parallel_gap_is_apart_only_where_the_plane_clears() {
        let curve = oval();
        let tilt = 0.5 * (band().zero() + band().escalate()) / lever();
        let normal = Vec3::new(1.0, tilt, 0.0);
        let read = |origin| cross(&Span::new(&curve, SPAN), &plane(origin, normal), band());
        let clear = read(Point3::new(OFFSET + 0.5, 0.0, 0.0));
        assert!(
            matches!(clear, Ok(ref c) if matches!(c.pose, Pose::Apart)),
            "clear of the plane: {clear:?}"
        );
        let through = read(Point3::new(OFFSET, 0.0, 0.0));
        let Err(refusal) = through else {
            panic!("through the plane: {through:?}");
        };
        assert!(
            matches!(
                refusal.windows[..],
                [crate::curve_face::Undecided {
                    why: CrossWhy::Pose(PoseRefusal::InBand {
                        rung: crate::curve_face::PoseRung::PlaneParallel,
                        ..
                    }),
                    ..
                }]
            ),
            "through the plane: {refusal:?}"
        );
    }

    /// The oracle's count and the door's on one plane: `Ok(None)` where
    /// the door refused, `Ok(Some(n))` where it answered `n` meetings, as
    /// many as the oracle's, each with its height off the plane in the
    /// band's zero and the oracle's sign change within a hundred times the
    /// band's escalation of it along the oval; `Err` naming what is wrong.
    fn read_against_oracle(
        origin: Point3<f64>,
        normal: Vec3<f64>,
    ) -> Result<Option<usize>, String> {
        read_against(&oracle(&oval(), origin, normal), origin, normal)
    }

    /// [`read_against_oracle`] against the crossings `want`.
    fn read_against(
        want: &[f64],
        origin: Point3<f64>,
        normal: Vec3<f64>,
    ) -> Result<Option<usize>, String> {
        let curve = oval();
        let Ok(got) = cross(&Span::new(&curve, SPAN), &plane(origin, normal), band()) else {
            return Ok(None);
        };
        if got.meetings.len() != want.len() {
            return Err(format!(
                "{} meetings, the oracle {want:?}",
                got.meetings.len()
            ));
        }
        for (m, w) in got.meetings.iter().zip(want) {
            let height = (m.point - origin).dot(normal.normalize());
            if height.abs() > band().zero() {
                return Err(format!("a meeting {height} off the plane"));
            }
            let off = (m.point - curve.eval(*w)).norm();
            if off > 100.0 * band().escalate() {
                return Err(format!(
                    "a meeting {off} along the oval from the oracle's at {w}"
                ));
            }
        }
        Ok(Some(want.len()))
    }

    /// **A plane all but parallel to the spiric's** (`[simple.conditioned]`):
    /// tilted by θ from 1e-2 down to 3e-8 about two axes of the oval's
    /// plane, each through a line that crosses the oval twice. The line
    /// where the planes meet is read in the oval's own plane, so a meeting
    /// stands off the exact one by the planes' own conditioning, `1/θ`, and
    /// no worse: every tilt answers both meetings, each on the plane to
    /// within the band's zero and at the oracle's crossing along the oval.
    #[test]
    fn a_plane_near_the_spirics_own_answers_to_its_digits() {
        let tilts = [
            1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7, 3e-8,
        ];
        let mut wrong = Vec::new();
        for theta in tilts {
            let (sin, cos) = f64::sin_cos(theta);
            for (label, origin, normal) in [
                (
                    "about the axis",
                    Point3::new(OFFSET, 5.0, 0.0),
                    Vec3::new(cos, sin, 0.0),
                ),
                (
                    "about m",
                    Point3::new(OFFSET, 0.0, 0.3),
                    Vec3::new(cos, 0.0, sin),
                ),
            ] {
                match read_against_oracle(origin, normal) {
                    Ok(Some(2)) => {}
                    other => wrong.push(format!("θ {theta}, {label}: {other:?}")),
                }
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
    }

    /// **A near miss in the band is no miss** (`[simple.conditioned]`):
    /// the plane square to the oval's own through its tangent line at a
    /// point of the oval (its top, its outermost and innermost points, two
    /// points between), shifted by δ along the oval's normal there, either
    /// way. Within the band's escalation, a miss or a pair of crossings the
    /// band cannot hold, the reading refuses; well clear of the top, it
    /// answers no meeting.
    #[test]
    fn a_plane_missing_the_oval_by_less_than_the_band_refuses() {
        let curve = oval();
        let mut wrong = Vec::new();
        for v in [FRAC_PI_2, 0.0, PI, 0.7, 2.3] {
            let tangent = curve.deriv(v).normalize();
            let normal = Vec3::unit_x().cross(tangent);
            for side in [1.0, -1.0] {
                for delta in [1e-11, 1e-10, 1e-9, 5e-9] {
                    let origin = curve.eval(v) + normal * (side * delta);
                    let got = cross(&Span::new(&curve, SPAN), &plane(origin, normal), band());
                    if got.is_ok() {
                        wrong.push(format!("v {v}, δ {}: {got:?}", side * delta));
                    }
                }
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
        let got = cross(
            &Span::new(&curve, SPAN),
            &plane(Point3::new(0.0, 0.0, MINOR + 0.05), Vec3::unit_z()),
            band(),
        );
        assert!(
            matches!(got, Ok(ref c) if c.meetings.is_empty()),
            "clear of the oval: {got:?}"
        );
    }

    /// The oracle on a leaned plane, from the oval's own evaluation: the
    /// least `|height|` off the plane among the local extremes of the
    /// oval's height over its whole turn (each found on a sampling and
    /// narrowed by ternary search), and the height's sign changes over the
    /// span, between its ends and the extremes inside it.
    fn extremes_oracle(
        curve: &Curve3<f64>,
        origin: Point3<f64>,
        normal: Vec3<f64>,
    ) -> (f64, Vec<f64>) {
        let normal = normal.normalize();
        let f = |v: f64| (curve.eval(v) - origin).dot(normal);
        let n = 2_000_i32;
        let at = |i: i32| -PI + 2.0 * PI * f64::from(i) / f64::from(n);
        let mut gap = f64::INFINITY;
        let mut breaks = vec![SPAN.0];
        for i in 0..n {
            let (below, here, above) = (f(at(i - 1)), f(at(i)), f(at(i + 1)));
            let sense = if here <= below && here <= above {
                1.0
            } else if here >= below && here >= above {
                -1.0
            } else {
                continue;
            };
            let (mut a, mut b) = (at(i - 1), at(i + 1));
            for _ in 0..120 {
                let (p, q) = (a + (b - a) / 3.0, b - (b - a) / 3.0);
                if sense * f(p) < sense * f(q) {
                    b = q;
                } else {
                    a = p;
                }
            }
            let extreme = 0.5 * (a + b);
            gap = gap.min(f(extreme).abs());
            if SPAN.0 < extreme && extreme < SPAN.1 {
                breaks.push(extreme);
            }
        }
        breaks.push(SPAN.1);
        breaks.sort_by(f64::total_cmp);
        let roots = breaks
            .windows(2)
            .filter_map(|w| sign_change(&f, w[0], w[1]))
            .collect();
        (gap, roots)
    }

    /// **A near miss is a height off the cutting plane, at any lean**
    /// (`[simple.conditioned]`): the plane through the oval's tangent line
    /// at a point of it (the inner-equator point past the span's end
    /// among them), leaned so the planes' tilt sine is `s`, from square
    /// down to just past the parallel rung, then moved by δ along its own
    /// normal either way. The oracle reads the oval's gap off the plane
    /// from the oval's own evaluation. A gap inside the band's escalation
    /// refuses on this door at every lean, as a tangency or on the ladder's
    /// own rung where the line itself reads in its band; a gap of twice the
    /// escalation or more answers the oracle's crossings, each on the
    /// plane.
    #[test]
    fn a_leaned_plane_missing_the_oval_by_less_than_the_band_refuses() {
        let curve = oval();
        let escalate = band().escalate();
        let least = 1.05 * escalate / lever();
        let mut wrong = Vec::new();
        for s in [1.0_f64, 0.2, 0.05, 0.01, 1e-5, least] {
            let mut in_band = 0;
            for v in [0.0, 0.7, 2.3, -1.1, PI] {
                let across = Vec3::unit_x().cross(curve.deriv(v).normalize());
                let normal = Vec3::unit_x() * (1.0 - s * s).sqrt() + across * s;
                for side in [1.0, -1.0] {
                    for k in [1e-3, 0.1, 0.3, 0.9, 2.5, 10.0] {
                        let origin = curve.eval(v) + normal * (side * k * escalate);
                        let (gap, want) = extremes_oracle(&curve, origin, normal);
                        let label = format!("s {s}, v {v}, δ {}·escalate", side * k);
                        let got = cross(&Span::new(&curve, SPAN), &plane(origin, normal), band());
                        if gap < escalate {
                            in_band += 1;
                            let tangency = matches!(
                                got,
                                Err(ref refusal) if !refusal.windows.is_empty()
                                    && refusal.windows.iter().all(|w| matches!(
                                        w.why,
                                        CrossWhy::Method {
                                            door: crate::curve_face::Door::SpiricPlane,
                                            fault: MeetFault::Unsettled(Unsettled::Tangency)
                                                | MeetFault::Escalated(_),
                                        }
                                    ))
                            );
                            if !tangency {
                                wrong.push(format!("{label}: a gap {gap} in band read {got:?}"));
                            }
                        } else if gap >= 2.0 * escalate {
                            match read_against(&want, origin, normal) {
                                Ok(Some(_)) => {}
                                got => {
                                    wrong.push(format!("{label}: a gap {gap} past it read {got:?}"))
                                }
                            }
                        }
                    }
                }
            }
            if in_band < 40 {
                wrong.push(format!("s {s}: the oracle read {in_band} gaps in band"));
            }
        }
        assert!(wrong.is_empty(), "{} wrong: {wrong:#?}", wrong.len());
    }

    /// **A graze of the other oval refuses**: the plane square to the
    /// oval's plane through the mirror oval's outermost point misses this
    /// oval, but the line × torus ladder reads the whole line, which
    /// grazes the torus there, so the reading refuses rather than answer
    /// a count it cannot certify.
    #[test]
    fn a_line_grazing_the_mirror_oval_refuses() {
        let curve = oval();
        let far = ((R + MINOR).powi(2) - OFFSET.powi(2)).sqrt();
        let got = cross(
            &Span::new(&curve, SPAN),
            &plane(Point3::new(0.0, -far, 0.0), Vec3::unit_y()),
            band(),
        );
        assert!(got.is_err(), "{got:?}");
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
    /// two-oval regime, or within the band of its edge, refuses typed on
    /// the regime's rung with the regime's margin, never a fault.
    #[test]
    fn a_spiric_outside_its_two_oval_convention_refuses_typed() {
        let gap = R - MINOR;
        for offset in [gap + 0.5, 0.0, gap - 5e-9, gap - 5e-10, gap - 1e-13] {
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
                        why: CrossWhy::Pose(PoseRefusal::InBand {
                            rung: crate::curve_face::PoseRung::SpiricRegime,
                            ..
                        }),
                        ..
                    }]
                ),
                "offset {offset}: {refusal:?}"
            );
        }
    }
}
