//! Review probes for PR 7294 (oneround/unsourced-unions). Not for merge:
//! each states the README's contract and goes red where the tree departs.
#![allow(clippy::unwrap_used, clippy::panic)]
use geom_core::{Band, Point3};

use super::meet_rows::line;
use super::{Span, meet};

/// Two unit-half-length segments crossing at their common middle, tilted
/// `s` apart, under the band `(1e-9, 1.5e-9)` with `s = 0.9e-9`: every
/// point of each lies within `s·1 < zero` of the other's line, so the
/// pair is one curve within the band (`one_line` reads it so). The far
/// distance of either span from the crossing (the datum's point) is 1;
/// its diameter is 2. A tilt levered by the diameter reads `1.8e-9`,
/// definite, and the router answers a crossing of two segments the band
/// reads as one. Red where the crossing lever is the diameter, not the
/// reach from the point the tilt is read at.
#[test]
fn probe_two_segments_within_band_everywhere_are_not_a_crossing() {
    let band = Band::new(1e-9, 1.5e-9).unwrap();
    let s: f64 = 0.9e-9;
    let a = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = line([0.0, 0.0, 0.0], [s.cos(), s.sin(), 0.0]);
    let got = meet(&Span::new(&a, (-1.0, 1.0)), &Span::new(&b, (-1.0, 1.0)), band);
    eprintln!("PROBE got={got:?}");
    match &got {
        Ok(m) => assert!(
            m.shared.is_some(),
            "segments within {s:e} of each other everywhere read as a crossing: {m:?}"
        ),
        Err(_) => {}
    }
}

/// The measured gap between `edge_extent_inf` (the "from below" crossing
/// lever) and the reach the README owes from a point of the arc: a
/// semicircle of radius 1 read from its middle reaches `√2`; its
/// diameter, which `edge_extent_inf` reads, is 2.
#[test]
fn probe_edge_extent_inf_exceeds_the_reach_from_an_arcs_middle() {
    let k = super::meet_rows::circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let (t0, t1) = (0.0, core::f64::consts::PI);
    let inf = crate::edge_extent_inf(&k, t0, t1).length();
    let mid = k.eval(0.5 * (t0 + t1));
    let cover = k.cover_over(geom::cover::ParamWindow::new(t0, t1)).ok();
    let reach = crate::ExtentBall::of_cover_from(cover.as_ref(), mid).radius();
    eprintln!("PROBE inf={inf} reach_from_middle={reach} ratio={}", inf / reach);
    assert!(
        inf <= reach * (1.0 + 1e-12),
        "edge_extent_inf {inf} passes the arc's reach {reach} from its middle"
    );
    let _ = Point3::<f64>::origin();
}
