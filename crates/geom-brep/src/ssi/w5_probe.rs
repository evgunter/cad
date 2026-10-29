//! **Probe only: W5's plane form**, the critical-point bound a stage in
//! front of C3's whole plane × NURBS reading would read. One closed
//! derivative box per span cell ([`NurbsBoxes::deriv_box_closed`]),
//! crossed in certification arithmetic: the normal box
//! `N = S_u × S_v`, then `N × n`. The span clears when some component
//! of `N × n` is one-signed (the normal is bounded off ±n there), and
//! the wall clears when every span does. The gradient form, the box of
//! `(n·S_u, n·S_v)` clear of the origin, is read beside it.

use geom::NurbsSurface;
use geom_core::{Interval, Real, Vec3};

use super::enclose::{Box3, NurbsBoxes, UvWindow};
use super::section::sign;

/// W5's verdict on one wall.
#[derive(Clone, Debug)]
pub struct W5Verdict {
    /// Every span's `N × n` box has a one-signed component.
    pub clears: bool,
    /// Every span's `(n·S_u, n·S_v)` box is clear of the origin.
    pub gradient_clears: bool,
    /// Span cells read.
    pub spans: usize,
    /// The first span `N × n` did not clear, as `[u0, u1] × [v0, v1]`.
    pub first_failure: Option<[f64; 4]>,
}

fn dot(b: Box3, n: [Interval; 3]) -> Interval {
    b.x * n[0] + b.y * n[1] + b.z * n[2]
}

fn cross(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn distinct_spans(k: &[f64]) -> Vec<(f64, f64)> {
    k.windows(2)
        .filter(|w| w[0] < w[1])
        .map(|w| (w[0], w[1]))
        .collect()
}

/// **W5 on `wall` against the plane normal `normal`** (module docs).
#[doc(hidden)]
#[must_use]
pub fn w5_plane(wall: &NurbsSurface<f64>, normal: Vec3<f64>) -> W5Verdict {
    let boxes = NurbsBoxes::new(wall);
    let n = [normal.x, normal.y, normal.z].map(Interval::from_f64);
    let us = distinct_spans(wall.knots_u_f64_interim().knots());
    let vs = distinct_spans(wall.knots_v_f64_interim().knots());
    let mut out = W5Verdict {
        clears: true,
        gradient_clears: true,
        spans: 0,
        first_failure: None,
    };
    for &(u0, u1) in &us {
        for &(v0, v1) in &vs {
            out.spans += 1;
            let w = UvWindow::new(u0, u1, v0, v1).expect("an ordered span cell");
            let (su, sv) = (
                boxes.deriv_box_closed(w, true),
                boxes.deriv_box_closed(w, false),
            );
            let nn = cross([su.x, su.y, su.z], [sv.x, sv.y, sv.z]);
            let c = cross(nn, n);
            if !c.iter().any(|&ci| sign(ci).is_some()) {
                out.clears = false;
                out.first_failure.get_or_insert([u0, u1, v0, v1]);
            }
            if sign(dot(su, n)).is_none() && sign(dot(sv, n)).is_none() {
                out.gradient_clears = false;
            }
        }
    }
    out
}
