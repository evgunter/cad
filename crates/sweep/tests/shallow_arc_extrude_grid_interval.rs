//! **The shallow-arc extrude grid at the certified scalar.** A square
//! whose top edge is an arc of bulge `b`, at offset `off` and scale
//! `L`, validated and extruded at `Interval` and then run through
//! `validate_geometric` — the grid both reviews of #3254 asked of the
//! cap apex, whose `Interval` width decides whether a flat arc's cap
//! plane stays at the chord's scale.
//!
//! What is pinned is the outcome census per ε row: how many cells
//! certify, how many the profile validation refuses (escalations on
//! the shallow arcs, before any sweep runs), and how many the extrude
//! refuses. The census is main's, cell for cell, under the chord-scale
//! apex (`geom_core::Arc2::apex`); a cap-plane change that makes a
//! flat arc refuse where it certified shows up as a count moving out of
//! `ok`. Nine of the extrude refusals at 1e-12 predate the apex. The
//! tenth is the extrude's closing pcurve mint: the wall rows of the
//! `off = 1000, l = 1e-3, b = 0.5` cell escalate at the certificate's
//! `Envelope` at this scalar, the far placement's rounding over a box
//! as narrow as one ulp
//! (`work/sym/pcurve-certificate-checks-widen-past-the-band-over-a-parameter-box.md`).
//!
//! **Since check 4 meters the chart's frame** (`EnvelopeTerm::Frame`,
//! PCERT's PR 3812 fix pass), the three `l = 50, b = 1e-4` cells (a wall
//! radius of `6.25e4` m) refuse at the extrude again at 1e-9, one at
//! each offset, and five cells refuse there at 1e-12. Their wall
//! cylinder's `u_ref` is the rim normalised at `Interval`, whose
//! `u_ref·u_ref − 1` encloses a few ulps; times the radius that is
//! about `6e-10` m, and the envelope's sum (`[0, 1.37e-9]`) no longer
//! decides inside the band. The true defect of a rounded unit vector is
//! `R·2⁻⁵³` (`7e-12` m here); the enclosure is what the interval scalar
//! can certify of it, and since the envelope is the whole certified
//! statement over a box the premise is metered rather than assumed
//! (`work/pctail/pcurve-envelope-escalates-at-interval-on-a-wide-arc.md`,
//! reopened).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Interval, Point2, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::validate_geometric;

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

/// One cell's outcome class.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Certifies,
    ProfileRefuses,
    ExtrudeRefuses,
    GeometryRefuses,
}

fn outcome(off: f64, l: f64, b: f64) -> Outcome {
    let lp = bulge_loop(vec![
        (p2(off - l / 2.0, off), iv(b)),
        (p2(off + l / 2.0, off), iv(0.0)),
        (p2(off + l / 2.0, off - l), iv(0.0)),
        (p2(off - l / 2.0, off - l), iv(0.0)),
    ]);
    let Ok(vp) = Profile::new(SketchPlane::<Interval>::xy(), vec![lp]).validate(Tol::witness())
    else {
        return Outcome::ProfileRefuses;
    };
    match extrude(&vp, Extrusion::Distance(iv(l)), Tol::witness()) {
        Ok(t) => match validate_geometric(&t.body, Tol::witness()) {
            Ok(()) => Outcome::Certifies,
            Err(_) => Outcome::GeometryRefuses,
        },
        Err(_) => Outcome::ExtrudeRefuses,
    }
}

#[test]
fn the_shallow_arc_grid_census_is_mains_at_every_eps_row() {
    let eps = Tol::witness().eps();
    // (certifies, profile refuses, extrude refuses, geometry refuses)
    let want: (usize, usize, usize, usize) = match eps {
        1e-6 => (51, 9, 0, 0),
        1e-9 => (51, 6, 3, 0),
        1e-12 => (24, 22, 14, 0),
        _ => return,
    };
    let mut got = (0, 0, 0, 0);
    for off in [0.0, 37.0, 1000.0] {
        for l in [1e-3, 0.1, 1.0, 50.0] {
            for b in [0.5, 1e-2, 1e-3, 1e-4, 1e-5] {
                match outcome(off, l, b) {
                    Outcome::Certifies => got.0 += 1,
                    Outcome::ProfileRefuses => got.1 += 1,
                    Outcome::ExtrudeRefuses => got.2 += 1,
                    Outcome::GeometryRefuses => got.3 += 1,
                }
            }
        }
    }
    assert_eq!(
        got, want,
        "the shallow-arc grid's census moved at eps={eps:e}"
    );
}
