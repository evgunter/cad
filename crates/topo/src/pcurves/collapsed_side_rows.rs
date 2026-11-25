#![allow(clippy::unwrap_used)]

//! **A joint at a spline net's collapsed side** (DESIGN `[extensibility]`):
//! where the side is one stored point the channel along it is free and
//! the element is the identity, and where its two ends are one ulp
//! apart the side is regular and the joint's chart gap, a whole span
//! along it, refuses.

use super::{ChartPeriods, DescribedChart, PinMiss, decide_plain_joint};
use crate::joint::JointElement;
use geom::{NurbsSurface, Surface};
use geom_brep::Pcurve;
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Point2, Point3};
use std::sync::Arc;

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// A triangle of a wall ([`geom::test_support::collapsed::triangle`]).
fn triangle(pole: Point3<f64>, top: Point3<f64>) -> Surface<f64> {
    Surface::Nurbs(Arc::new(geom::test_support::collapsed::triangle(pole, top)))
}

/// The joint into the bottom row `(0, 0) → (1, 0)` from the top row's
/// end at `(0, 1)`, at the vertex `pole`.
fn joint(surface: &Surface<f64>, pole: Point3<f64>) -> Result<JointElement, PinMiss> {
    let image = Pcurve::iso_line_for_tests([0.0, 0.0], [1.0, 0.0]).unwrap();
    decide_plain_joint(
        DescribedChart::of(surface),
        &image,
        0.0,
        Point2::new(0.0, 1.0),
        pole,
        ChartPeriods::of(surface, band()),
        band(),
    )
}

/// The pole at (1, 1, 0); one ulp off, the column's top a ulp along `x`.
#[test]
fn a_joint_on_a_collapsed_side_is_the_identity_and_one_ulp_off_it_is_a_gap() {
    let pole = Point3::new(1.0, 1.0, 0.0);
    let one = triangle(pole, pole);
    assert!(
        matches!(joint(&one, pole), Ok(JointElement::IDENTITY)),
        "one stored point: the identity, read {:?}",
        joint(&one, pole).ok()
    );
    let ulp = geom::test_support::collapsed::ulp_along_x(pole);
    let apart = triangle(pole, ulp);
    assert!(
        geom_brep::chart_singularity::singular_points(&apart).is_empty(),
        "one ulp apart: no singular point"
    );
    assert!(
        matches!(joint(&apart, pole), Err(PinMiss::Discontinuity)),
        "one ulp apart: the chart gap refuses, read {:?}",
        joint(&apart, pole).ok()
    );
}

/// A NURBS cone about `z`: its `u` row the rational circle, closed, and
/// its side `v = 0` the apex, one stored point.
fn spline_cone() -> Surface<f64> {
    let ku = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let ring = [
        (1.0, 0.0, 1.0),
        (1.0, 1.0, s),
        (0.0, 1.0, 1.0),
        (-1.0, 1.0, s),
        (-1.0, 0.0, 1.0),
        (-1.0, -1.0, s),
        (0.0, -1.0, 1.0),
        (1.0, -1.0, s),
        (1.0, 0.0, 1.0),
    ];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for &(x, y, w) in &ring {
        control.push(Point3::new(0.0, 0.0, 0.0));
        control.push(Point3::new(x, y, -1.0));
        weights.extend([w, w]);
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(Knots::from(ku), Knots::from(kv), control, weights).unwrap(),
    ))
}

/// The joint into `entry`'s image (an iso line leaving `entry`) from
/// `prev`, at `vertex` on `surface`.
fn joint_at(
    surface: &Surface<f64>,
    prev: (f64, f64),
    entry: (f64, f64),
    vertex: Point3<f64>,
) -> Result<JointElement, PinMiss> {
    let image = Pcurve::iso_line_for_tests([entry.0, entry.1], [0.0, 1.0]).unwrap();
    decide_plain_joint(
        DescribedChart::of(surface),
        &image,
        0.0,
        Point2::new(prev.0, prev.1),
        vertex,
        ChartPeriods::of(surface, band()),
        band(),
    )
}

/// **The free channel is the side's only where both ends are on it.** At
/// the triangle's pole, a joint whose two ends lie a whole `v` span apart
/// on the regular side `u = 1`, or in the chart's interior at `u = ½`,
/// walks no collapsed side: its chart gap refuses.
#[test]
fn a_joint_at_the_pole_whose_ends_are_off_its_side_is_a_gap() {
    let pole = Point3::new(1.0, 1.0, 0.0);
    let one = triangle(pole, pole);
    for u in [1.0, 0.5] {
        let got = joint_at(&one, (u, 1.0), (u, 0.0), pole);
        assert!(
            matches!(got, Err(PinMiss::Discontinuity)),
            "u = {u}: the ends are off the side, read {:?}",
            got.ok()
        );
    }
}

/// **An incidence the band cannot call refuses**, on an open net as on a
/// closed one: a vertex half a band off the pole may be on the side or
/// off it.
#[test]
fn a_vertex_in_band_of_a_collapsed_side_refuses() {
    let pole = Point3::new(1.0, 1.0, 0.0);
    let one = triangle(pole, pole);
    let near = Point3::new(1.0 + 5e-9, 1.0, 0.0);
    let got = joint_at(&one, (0.0, 1.0), (0.0, 0.0), near);
    assert!(
        matches!(got, Err(PinMiss::NetStretchUndecided(_))),
        "in band: undecided, read {:?}",
        got.ok()
    );
}

/// **Two adjacent collapsed sides at one point**: a net whose sides
/// `u = 0` and `v = 0` are both the point P. A joint along either side,
/// or one from the first across their corner onto the second, walks
/// them and is the identity; which side is read off its ends, not the
/// first side the vertex is on.
#[test]
fn a_joint_along_or_across_two_adjacent_collapsed_sides_is_the_identity() {
    let p = Point3::new(0.0, 0.0, 0.0);
    let kv = || Knots::from(KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap());
    let mut control = Vec::new();
    for iu in 0..3_u32 {
        for iv in 0..3_u32 {
            control.push(if iu == 0 || iv == 0 {
                p
            } else {
                Point3::new(f64::from(iu), f64::from(iv), f64::from(iu * iv) * 0.1)
            });
        }
    }
    let fan = Surface::Nurbs(Arc::new(
        NurbsSurface::new(kv(), kv(), control, vec![1.0; 9]).unwrap(),
    ));
    for (what, prev, entry) in [
        ("along u = 0", (0.0, 0.2), (0.0, 0.8)),
        ("along v = 0", (0.2, 0.0), (0.8, 0.0)),
        (
            "from u = 0 across the corner onto v = 0",
            (0.0, 0.7),
            (0.4, 0.0),
        ),
        (
            "from v = 0 across the corner onto u = 0",
            (1.0, 0.0),
            (0.0, 1.0),
        ),
    ] {
        let got = joint_at(&fan, prev, entry, p);
        assert!(
            matches!(got, Ok(JointElement::IDENTITY)),
            "{what}: the identity, read {:?}",
            got.ok()
        );
    }
}

/// **A collapsed side along a channel the net closes**: at a NURBS cone's
/// apex the marks' decision stands. A joint whose chart gap is a period
/// across the seam is a period over; one whose gap is half a period is
/// the marks' decided [`PinMiss::Discontinuity`], kept as itself; and one
/// whose gap lies within the band of a period, which the marks leave
/// undecided, refuses as unbuilt, [`super::SingularMiss::ClosedSide`]:
/// the turn about the point and the face's side of it have no spline
/// reading yet
/// (`work/charttail/a-closed-collapsed-side-has-no-turn-or-side-reading.md`).
#[test]
fn a_collapsed_side_along_a_closed_channel_keeps_what_its_gap_decides() {
    use crate::joint::Deck;
    let cone = spline_cone();
    let apex = Point3::new(0.0, 0.0, 0.0);
    assert!(
        ChartPeriods::of(&cone, band()).u().is_some(),
        "the cone's u closes"
    );
    let seam = joint_at(&cone, (1.0, 0.0), (0.0, 0.0), apex);
    assert!(
        matches!(
            seam,
            Ok(JointElement(Deck {
                u: 1,
                v: 0,
                twin: false
            }))
        ),
        "across the seam: a period over, read {:?}",
        seam.ok()
    );
    let half = joint_at(&cone, (0.75, 0.0), (0.25, 0.0), apex);
    assert!(
        matches!(half, Err(PinMiss::Discontinuity)),
        "half a turn: the marks' decided refusal, read {:?}",
        half.ok()
    );
    let in_band = joint_at(&cone, (1.0, 0.0), (1e-9, 0.0), apex);
    assert!(
        matches!(
            in_band,
            Err(PinMiss::Singular(super::SingularMiss::ClosedSide))
        ),
        "a period off by the band: unbuilt, read {:?}",
        in_band.ok()
    );
}
