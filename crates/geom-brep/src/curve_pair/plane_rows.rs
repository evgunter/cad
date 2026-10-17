//! **A crossing of the other's plane is not a meeting** (the pose's
//! module docs, "deflation"): across a plane the arms divide no found
//! meeting out, since a plane root marks a window that may hold more
//! than one meeting. Each row is a pair meeting twice, or four times,
//! inside one such window, with a meeting found first at a span's end;
//! the reader either reports every meeting or refuses typed, never a
//! decided answer with one missing.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Point3, Vec3};

use super::meet_rows::{band, circle, line};
use super::{MeetFault, MeetRefusal, Meetings, Span, meet};

/// Every point of `want` is a meeting of `got`, or `got` refuses typed
/// on the reading (an escalation or an `Unsettled`), never on a kernel
/// fault.
fn none_missing(label: &str, got: &Result<Meetings<f64>, MeetRefusal<f64>>, want: &[Point3<f64>]) {
    match got {
        Ok(m) => {
            for w in want {
                assert!(
                    m.points.iter().any(|p| (p.point - *w).norm() < 1e-6),
                    "{label}: decided with the meeting at {w:?} missing: {m:?}"
                );
            }
        }
        Err(r) => assert!(
            matches!(r.fault, MeetFault::Escalated(_) | MeetFault::Unsettled(_)),
            "{label}: refused on a kernel fault: {r:?}"
        ),
    }
}

/// **A line through a circle's plane meets it at both ends of a chord.**
/// The unit circle in `z = 0`, its seam at `ŷ`, and the line from
/// `Q = (1, 0, h)` to `P = (−1, 0, −h)`, `h = 0.9·10⁻⁹`: within the zero
/// band of the circle at both, two metres apart, the chord's middle a
/// metre off the circle, so two meetings in two stretches, both within
/// the escalation of the circle's plane. The line starts at `Q`, found
/// first at its end, or before it; the circle is read whole, with ends
/// or without. Red where the found meeting at `Q` is taken for the
/// line's one crossing of the plane and `P` is lost.
#[test]
fn a_line_through_a_circles_plane_meets_it_at_both_ends_of_a_chord() {
    let c = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let h = 0.9e-9;
    let (q, p) = (Point3::new(1.0, 0.0, h), Point3::new(-1.0, 0.0, -h));
    let l = line(q.to_array(), (p - q).to_array());
    for (label, ended) in [("endless", false), ("ended", true)] {
        let whole = Span::new(&c, (0.0, TAU));
        let whole = if ended { whole } else { whole.endless() };
        for (t0, starts) in [(0.0, "at Q"), (-1.0, "before Q")] {
            let seg = Span::new(&l, (t0, 100.0));
            none_missing(
                &format!("{label}, {starts}, line first"),
                &meet(&seg, &whole, band()),
                &[q, p],
            );
            none_missing(
                &format!("{label}, {starts}, circle first"),
                &meet(&whole, &seg, band()),
                &[q, p],
            );
        }
    }
}

/// **A line tilted out of a circle's plane meets it at both ends of a
/// short chord.** The unit circle in `z = 0` and a line tilted `2·10⁻⁸`
/// out of it along `ŷ`, its projection the chord at
/// `x₀ = √(1 − 0.03²)`: within `6·10⁻¹⁰` of the circle at both ends,
/// `0.06` m apart, the chord's middle `4.5·10⁻⁴` m off it. The line
/// starts at the first end or before it. Red where the end found first
/// is taken for the line's one crossing of the plane.
#[test]
fn a_line_tilted_out_of_a_circles_plane_meets_it_at_both_ends_of_a_short_chord() {
    let tilt = 2e-8;
    let x0 = (1.0f64 - 0.03 * 0.03).sqrt();
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let ends = [
        Point3::new(x0, -0.03, -0.03 * tilt),
        Point3::new(x0, 0.03, 0.03 * tilt),
    ];
    for (label, start) in [
        ("starts at the first end", -0.03),
        ("starts before it", -0.05),
    ] {
        let l = line([x0, start, start * tilt], [0.0, 1.0, tilt]);
        let (sl, sc) = (Span::new(&l, (0.0, 1.0)), Span::new(&c, (-0.5, 0.5)));
        none_missing(
            &format!("{label}, line first"),
            &meet(&sl, &sc, band()),
            &ends,
        );
        none_missing(
            &format!("{label}, circle first"),
            &meet(&sc, &sl, band()),
            &ends,
        );
    }
}

/// **Two circles at a shallow tilt meet twice near one root of the
/// plane.** The unit circle tilted `3·10⁻⁸` about `x̂`, and a circle of
/// radius `0.5` in `z = 0` about `(0.5001, 0, 0)` crossing its projection
/// at `y ≈ ±0.0141`, both within about `4·10⁻¹⁰` of the first, `0.028` m
/// apart, the middle `10⁻⁴` m off. The first starts at one meeting or
/// before it. Red where the transverse arm divides the found meeting out
/// of the height harmonic, or skips its root in the found meeting's ring.
#[test]
fn two_circles_at_a_shallow_tilt_meet_twice_near_one_plane_root() {
    let t = 3e-8f64;
    let a = circle([0.0; 3], [0.0, -t.sin(), t.cos()], 1.0);
    let cb = 0.5 + 1e-4;
    let b = circle([cb, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5);
    let x = (1.0 + cb * cb - 0.25) / (2.0 * cb);
    let y = (1.0 - x * x).sqrt();
    let th = -(y / t.cos()).asin();
    let want = [a.eval(th), a.eval(-th)];
    for (label, sa) in [
        ("starts at one", (th, 0.5)),
        ("starts before it", (-0.05, 0.5)),
    ] {
        let (s1, s2) = (Span::new(&a, sa), Span::new(&b, (-0.5, 0.5)));
        none_missing(
            &format!("{label}, tilted first"),
            &meet(&s1, &s2, band()),
            &want,
        );
        none_missing(
            &format!("{label}, flat first"),
            &meet(&s2, &s1, band()),
            &want,
        );
    }
}

/// **Two meetings found near one plane root leave the other root's.** A
/// unit circle tilted `2·10⁻⁸` about `x̂` and an ellipse in `z = 0` whose
/// projections cross at `θ = ±0.03` and `π ± 0.03`, all within
/// `6·10⁻¹⁰` of the circle: four meetings. The circle's span ends at
/// `±0.03`, so its ends are two meetings found first, near one root of
/// the height harmonic; the other two lie near the other root. Red where
/// two found meetings are taken to leave the harmonic no root.
#[test]
fn two_meetings_found_near_one_plane_root_leave_the_other_roots_meetings() {
    let al = 2e-8_f64;
    let (s, c) = al.sin_cos();
    let a = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -s, c),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let th = 0.03_f64;
    let big = th.cos() / (1.0 - (c * th.sin()).powi(2) / 0.81).sqrt();
    let b = Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        semi_u: 0.9,
        semi_v: big,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let sa = Span::new(&a, (th, TAU - th));
    let far = [a.eval(PI - th), a.eval(PI + th)];
    for (label, sb, want) in [
        (
            "whole ellipse",
            Span::new(&b, (0.0, TAU)).endless(),
            &far[..],
        ),
        ("half ellipse", Span::new(&b, (0.0, PI)), &far[..]),
    ] {
        none_missing(
            &format!("{label}, circle first"),
            &meet(&sa, &sb, band()),
            want,
        );
        none_missing(
            &format!("{label}, ellipse first"),
            &meet(&sb, &sa, band()),
            want,
        );
    }
}

/// The tilted unit circle of
/// `two_circles_at_a_shallow_tilt_meet_twice_near_one_plane_root`, read
/// as a whole turn whose seam (`θ = 0`) is the plane root, against a flat
/// circle crossing its image at `±y`: two meetings, one each side of the
/// seam, in one window. The flat circle's span ends at one of them, found
/// first; the other lies across the seam. Ended or endless, seam at `0`
/// or at `−π`, either span first. Red where the window is read on one
/// side of the seam only.
fn seam_rows(cases: &[(f64, f64, f64)]) {
    for &(t, rb, y) in cases {
        let a = circle([0.0; 3], [0.0, -t.sin(), t.cos()], 1.0);
        let cb = (1.0 - y * y).sqrt() - (rb * rb - y * y).sqrt();
        let b = circle([cb, 0.0, 0.0], [0.0, 0.0, 1.0], rb);
        let th = (y / t.cos()).asin();
        let want = [a.eval(th), a.eval(TAU - th)];
        let phi = y.atan2((1.0 - y * y).sqrt() - cb);
        for (sl, sb) in [("J above", (-0.5, phi)), ("J below", (-phi, 0.5))] {
            for (el, ended) in [("endless", false), ("ended", true)] {
                for (ol, a0) in [("seam 0", 0.0), ("seam −π", -PI)] {
                    let s1 = Span::new(&a, (a0, a0 + TAU));
                    let s1 = if ended { s1 } else { s1.endless() };
                    let s2 = Span::new(&b, sb);
                    let label = format!("t {t:e} rb {rb} y {y:e}: {sl}, {el}, {ol}");
                    none_missing(
                        &format!("{label}, tilted first"),
                        &meet(&s1, &s2, band()),
                        &want,
                    );
                    none_missing(
                        &format!("{label}, flat first"),
                        &meet(&s2, &s1, band()),
                        &want,
                    );
                }
            }
        }
    }
}

/// **Two meetings across a whole turn's seam, in a wide window.**
/// `seam_rows` at the shallow tilt `3·10⁻⁸` and a flat circle of radius
/// `0.5`, crossing at `y ≈ ±0.0141`.
#[test]
fn two_meetings_across_a_whole_turns_seam_are_both_read() {
    let cb: f64 = 0.5 + 1e-4;
    let x = (1.0 + cb * cb - 0.25) / (2.0 * cb);
    seam_rows(&[(3e-8, 0.5, (1.0 - x * x).sqrt())]);
}

/// **Two meetings across a whole turn's seam, in a narrow window.**
/// `seam_rows` at steeper tilts and larger flat circles, the meetings
/// `2` to `8` mm either side of the seam.
#[test]
fn two_meetings_across_a_whole_turns_seam_in_a_narrow_window_are_both_read() {
    seam_rows(&[
        (1e-7, 0.9, 4e-3),
        (2e-7, 0.9, 2e-3),
        (1e-7, 0.5, 4e-3),
        (5e-8, 0.95, 8e-3),
    ]);
}

/// **A closed rim whose seam vertex is the joint, and a second meeting
/// before the seam.** The tilted unit circle read as a whole turn
/// `(0, 2π)`, both its ends at `J = (1, 0, 0)` on the flat circle's
/// plane; a flat circle through `J` and through the rim's image `δ`
/// before the seam, `2` to `4` mm: the shape of a closed edge whose one
/// vertex is a fillet joint. Red where the window about the seam is read
/// on one side of it.
#[test]
fn a_seam_vertex_at_the_joint_leaves_the_meeting_before_the_seam() {
    seam_vertex(&[0.0]);
}

/// **The seam vertex at a far turn.** `seam_vertex` with the rim's turn
/// starting at `2πk`, and `0.002` past it, for `k` from `−4` to `3`: the
/// window's turns are counted from the span, wherever its parameters lie.
/// Red where they are fixed about zero.
#[test]
fn a_seam_vertex_at_a_far_turn_leaves_the_meeting_before_the_seam() {
    let starts: Vec<f64> = (-4..=3)
        .flat_map(|k| [0.0, 0.002].map(|shift| f64::from(k) * TAU + shift))
        .collect();
    seam_vertex(&starts);
}

/// The rim and flat circle of
/// `a_seam_vertex_at_the_joint_leaves_the_meeting_before_the_seam`, the rim
/// read as the whole turn from each of `starts`.
fn seam_vertex(starts: &[f64]) {
    for &(t, rb, delta) in &[
        (1e-7_f64, 0.9_f64, 4e-3_f64),
        (1e-7, 1.2, 4e-3),
        (2e-7, 0.9, 2e-3),
    ] {
        let c = t.cos();
        let a = circle([0.0; 3], [0.0, -t.sin(), c], 1.0);
        let j = Point3::new(1.0, 0.0, 0.0);
        let pp = Point3::new(delta.cos(), -c * delta.sin(), 0.0);
        let m = Point3::new(0.5 * (j.x + pp.x), 0.5 * (j.y + pp.y), 0.0);
        let chord = pp - j;
        let half = 0.5 * chord.norm();
        let n = Vec3::new(chord.y, -chord.x, 0.0).normalize();
        let n = if n.dot(Point3::origin() - m) > 0.0 {
            n
        } else {
            n * -1.0
        };
        let centre = m + n * (rb * rb - half * half).sqrt();
        let b = circle(centre.to_array(), [0.0, 0.0, 1.0], rb);
        let ang = |q: Point3<f64>| (q.y - centre.y).atan2(q.x - centre.x);
        let (pj, pq) = (ang(j), ang(pp));
        let want = [j, a.eval(TAU - delta)];
        for (el, ended, &t0) in [("ended", true), ("endless", false)]
            .into_iter()
            .flat_map(|(el, ended)| starts.iter().map(move |t0| (el, ended, t0)))
        {
            let s1 = Span::new(&a, (t0, t0 + TAU));
            let s1 = if ended { s1 } else { s1.endless() };
            let s2 = Span::new(&b, (pj.min(pq) - 0.3, pj.max(pq) + 0.3));
            let label = format!("t {t:e} rb {rb} δ {delta:e}: from {t0}, {el}");
            none_missing(
                &format!("{label}, rim first"),
                &meet(&s1, &s2, band()),
                &want,
            );
            none_missing(
                &format!("{label}, flat first"),
                &meet(&s2, &s1, band()),
                &want,
            );
        }
    }
}
