//! A span whose continuation past its end meets the chart on the chart's
//! domain edge: an arc in the plane `x = 1`, which holds the edge `u = 1`
//! of the flat unit patch, crossing that edge transversally at `θ = π/2`.
//! The walk reads the carrier [`super::END_REACH`] escalation lengths
//! past each end, so at a coarse band the crossing sits inside the
//! window of a span that ends short of it. The offsets scale with the
//! run's band.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{END_REACH, SplineFaceRefusal};
use crate::curve_face::{Boundary, OnBoundary, boundary};
use crate::curve_pair::{Hit, Span};
use crate::roots::Unsettled;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The arc's radius, the span's pace.
const R: f64 = 0.25;

/// The flat unit patch `z = 0` over `[0, 1]²`, its chart point its
/// `(x, y)`.
fn flat() -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap()
}

/// The circle of radius [`R`] about `(1, ½, 0)` in the plane `x = 1`:
/// at `θ = 0` it stands `R` below the patch, and at `θ = π/2` it crosses
/// the patch's edge `x = 1` at `(1, ¾, 0)`, leaving the patch's side of
/// the plane `z = 0` for the other. It never enters the face: the
/// nearest point of the patch to any of its points lies on that edge.
fn arc() -> Curve3<f64> {
    let ring = Curve3::circle(
        Point3::new(1.0, 0.5, 0.0),
        Vec3::unit_x(),
        R,
        -Vec3::unit_z(),
    );
    let crossing = ring.eval(FRAC_PI_2);
    assert!(
        (crossing - Point3::new(1.0, 0.75, 0.0)).norm() < 1e-15,
        "the fixture's arc crosses the edge at (1, 3/4, 0): {crossing:?}"
    );
    ring
}

/// The span of arc length `length` ending `gap` metres short of the
/// crossing.
fn short_of_the_crossing(gap: f64, length: f64) -> (f64, f64) {
    let end = FRAC_PI_2 - gap / R;
    (end - length / R, end)
}

/// The witness span's arc length: `END_REACH² · escalate`, at most
/// [`R`], so the walk's floor, `√(length · escalate) / 8`, is at most a
/// quarter of the witness's gap (half the reach) at every band, and a
/// box at the floor round the crossing lies wholly past the end.
fn short() -> f64 {
    (END_REACH * END_REACH * band().escalate()).min(R)
}

/// **The witness.** The span ends half the walk's reach short of the
/// crossing, which is clear of the span by that much: the crossing is
/// the next span's, and this one reads no meeting.
#[test]
fn a_crossing_on_the_charts_edge_past_the_spans_end_is_not_the_spans() {
    let gap = 0.5 * END_REACH * band().escalate();
    let got = crate::NurbsLane::<f64>::certified().chart_meetings(
        &arc(),
        short_of_the_crossing(gap, short()),
        &flat(),
        band(),
    );
    assert!(
        matches!(&got, Ok(found) if found.is_empty()),
        "{gap:e} m short of the crossing: no meeting, no refusal: {got:?}"
    );
}

/// **A crossing within the band of the end is the end's.** No box past
/// the band of the end holds it: the end's box accounts for it, the
/// walk reports no meeting inside the span, and the end's point reads
/// on the chart within the band, which is where its meeting is placed.
#[test]
fn a_crossing_on_the_charts_edge_within_the_band_of_the_end_is_the_ends() {
    let gap = 0.5 * band().zero();
    let span = short_of_the_crossing(gap, short());
    let got = crate::NurbsLane::<f64>::certified().chart_meetings(&arc(), span, &flat(), band());
    assert!(
        matches!(&got, Ok(found) if found.is_empty()),
        "{gap:e} m short of the crossing: the end's: {got:?}"
    );
    let end = arc().eval(span.1);
    assert!(
        end.z.abs() <= band().zero() && (end.x - 1.0).abs() <= band().zero(),
        "the end reads on the patch's edge: {end:?}"
    );
}

/// **A crossing nearer the end than the floor still refuses.** On a span
/// long enough that a box at the floor round the crossing reaches back
/// over the end, the box is not past the end, and the walk refuses as
/// it does for any box it cannot read.
#[test]
fn a_crossing_on_the_charts_edge_within_the_floor_of_the_end_refuses() {
    let gap = 4.0 * band().escalate();
    let got = crate::NurbsLane::<f64>::certified().chart_meetings(
        &arc(),
        short_of_the_crossing(gap, R),
        &flat(),
        band(),
    );
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "{gap:e} m short of the crossing: {got:?}"
    );
}

/// **The crossing is read by the span that holds it**: the boundary pass
/// of the span past the witness's end meets the patch's edge `x = 1` at
/// `θ = π/2`, inside that edge.
#[test]
fn the_next_span_meets_the_crossing_on_the_boundary() {
    let gap = 0.5 * END_REACH * band().escalate();
    let next = (short_of_the_crossing(gap, short()).1, PI);
    let corners =
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)].map(|(x, y)| Point3::new(x, y, 0.0));
    let sides: Vec<Curve3<f64>> = (0..4)
        .map(|i| Curve3::line(corners[i], corners[(i + 1) % 4] - corners[i]))
        .collect();
    let edges: Vec<_> = sides
        .iter()
        .map(|c| (c.eval(0.0), Span::new(c, (0.0, 1.0))))
        .collect();
    let got = boundary(&Span::new(&arc(), next), &edges, &[], band()).unwrap();
    let Boundary::Meets {
        t,
        point,
        at: OnBoundary::Edge(1, Hit::On { t: s, .. }),
    } = got
    else {
        panic!("a meeting inside the edge x = 1: {got:?}");
    };
    assert!(
        (t - FRAC_PI_2).abs() * R <= band().zero()
            && (point - Point3::new(1.0, 0.75, 0.0)).norm() <= band().zero()
            && (s - 0.75).abs() <= band().zero(),
        "at θ = π/2, (1, 3/4, 0), 3/4 along the edge: {got:?}"
    );
}
