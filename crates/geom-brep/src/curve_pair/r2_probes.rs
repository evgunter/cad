//! R2 review probes (PR 5227). Not for merge.
#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stderr)]

use geom::Curve3;
use geom_core::Point3;

use super::meet_rows::{band, circle, line};
use super::{Span, meet};

fn report(label: &str, got: &Result<super::Meetings<f64>, super::MeetRefusal<f64>>) -> usize {
    match got {
        Ok(m) => {
            eprintln!("{label}: OK {} meetings: {:?}", m.points.len(), m.points.iter().map(|p| p.point).collect::<Vec<_>>());
            m.points.len()
        }
        Err(r) => {
            eprintln!("{label}: REFUSED {:?}, found {}", r.fault, r.found.len());
            usize::MAX
        }
    }
}

/// A line tilted 2e-8 out of a unit circle's plane, its projection a chord
/// at x0 = sqrt(1 - 0.03^2): it is within 6e-10 of the circle at both
/// chord ends C1 = (x0, -0.03) and C2 = (x0, 0.03), 0.06 m apart, the
/// chord's middle 4.5e-4 off the circle. Two meetings in two stretches.
#[test]
fn r2_line_shallow_across_circle_plane_two_meetings() {
    let tilt = 2e-8;
    let x0 = (1.0f64 - 0.03 * 0.03).sqrt();
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    for (label, start) in [("line starts at C1", -0.03), ("line starts before C1", -0.05)] {
        let l = line([x0, start, start * tilt], [0.0, 1.0, tilt]);
        let ls = (0.0, 1.0);
        let cs = (-0.5, 0.5);
        let (ca, cb) = (Span::new(&l, ls), Span::new(&c, cs));
        for p in [Point3::new(x0, -0.03, -0.03 * tilt), Point3::new(x0, 0.03, 0.03 * tilt)] {
            eprintln!("  dist to circle of {p:?}: {}", super::meet_rows::distance(&c, p));
        }
        let n1 = report(&format!("{label}, line first"), &meet(&ca, &cb, band()));
        let n2 = report(&format!("{label}, circle first"), &meet(&cb, &ca, band()));
        assert!(n1 != 1 && n2 != 1, "{label}: a meeting lost");
    }
}

/// Two circles nearly coplanar (a tilted 3e-8 about x), b crossing a's
/// projection twice near a's plane root (1,0,0), at y = ±0.0141, both
/// within ~4e-10 of a: two meetings, 0.028 m apart, the middle 1e-4 off.
#[test]
fn r2_two_circles_shallow_tilt_two_meetings_near_one_plane_root() {
    let t = 3e-8f64;
    let a = circle([0.0; 3], [0.0, -t.sin(), t.cos()], 1.0);
    let cb = 0.5 + 1e-4;
    let b = circle([cb, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5);
    let x = (1.0 + cb * cb - 0.25) / (2.0 * cb);
    let y = (1.0 - x * x).sqrt();
    let th_p = -(y / t.cos()).asin();
    let pa = a.eval(th_p);
    eprintln!("P on a {pa:?}; dist to b {}", super::meet_rows::distance(&b, pa));
    let pq = a.eval(-th_p);
    eprintln!("Q on a {pq:?}; dist to b {}", super::meet_rows::distance(&b, pq));
    for (label, sa) in [("a starts at P", (th_p, 0.5)), ("a starts before P", (-0.05, 0.5))] {
        let (s1, s2) = (Span::new(&a, sa), Span::new(&b, (-0.5, 0.5)));
        let n1 = report(&format!("{label}, a first"), &meet(&s1, &s2, band()));
        let n2 = report(&format!("{label}, b first"), &meet(&s2, &s1, band()));
        assert!(n1 != 1 && n2 != 1, "{label}: a meeting lost");
    }
}
