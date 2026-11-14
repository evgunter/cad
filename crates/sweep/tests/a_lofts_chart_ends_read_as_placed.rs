//! **A loft's wall rims read their chart ends as their mints placed
//! them**, so an `Interval` loft's walls take check 7's rectangle
//! certificate and the body finishes at its taker. A rim's span is a
//! computed length, an enclosure some ulps wide at `Interval`, and its
//! image's map `p0 + pl·t` read there only encloses the chart's `1`; the
//! boundary row's mint stores the ends it places the row by
//! (`geom_brep::Pcurve::iso_row`), and the quadrature reads them.
//!
//! The other side is pinned too: the certified volume is the body's,
//! and a split rim's cut is never read as a chart end.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::enclosure::box_about;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use topo::construction::Construct;
use topo::{Body, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// The quadrilateral `(−1, −1), (1, −1), (h, 1), (−1, 1)`: every side a
/// line, its area `2·(2 + (h + 1))/2`.
fn quad<T: Real>(h: T) -> Vec<ProfileLoop<T>> {
    let corner = |x: T, y: f64| (Point2::new(x, T::from_f64(y)), T::zero());
    vec![bulge_loop(vec![
        corner(T::from_f64(-1.0), -1.0),
        corner(T::one(), -1.0),
        corner(h, 1.0),
        corner(T::from_f64(-1.0), 1.0),
    ])]
}

/// Sections at heights `0`, `1` and `2`.
fn places<T: Real>() -> [SketchPlane<T>; 3] {
    [0.0, 1.0, 2.0].map(|z| SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), T::from_f64(z))))
}

/// The `Interval` loft of `middle` between two [`quad`]s at `h = 1.2`,
/// taken through `topo::take`, and its certified volume as `[lo, hi]`.
fn taken_volume(middle: Interval) -> (f64, f64) {
    let ends = Interval::from_f64(1.2);
    let lofted =
        sweep::loft_body::<Interval>(&[quad(ends), quad(middle), quad(ends)], &places(), 2, tol())
            .expect("the door builds the loft");
    let finished = topo::take::<Interval>(&std::sync::Arc::new(lofted.body), tol())
        .expect("the loft finishes at its taker");
    let m = topo::mass_properties(
        &*finished,
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("the finished loft measures")
    .enclosure;
    (m.volume.lo() - m.volume_pad, m.volume.hi() + m.volume_pad)
}

/// **A point-interval loft finishes at its taker**, and the volume check
/// 7 certifies holds the prism's: three equal sections are the straight
/// prism of area `4.2` and height `2`. A wall certified over the wrong
/// part of its chart misses `8.4`.
#[test]
fn a_point_interval_loft_finishes_at_its_taker_and_holds_its_volume() {
    let (lo, hi) = taken_volume(Interval::from_f64(1.2));
    assert!(
        lo <= 8.4 && 8.4 <= hi && hi - lo < 1e-6,
        "the certified volume [{lo}, {hi}] holds the prism's 8.4"
    );
}

/// **A loft over a box finishes at its taker**: the middle section's
/// moving corner over [`box_about`]`(1.2)`, so the middle rims' spans are
/// the box's width and their images' maps read the box there. The volume
/// holds the point loft's, which is a member of the family, and stays as
/// narrow as the point loft's bound: the box moves the volume by
/// about its own width.
#[test]
fn a_loft_over_a_box_finishes_at_its_taker() {
    let (lo, hi) = box_about(1.2);
    let (vlo, vhi) = taken_volume(Interval::from_bounds(lo, hi));
    assert!(
        vlo <= 8.4 && 8.4 <= vhi && vhi - vlo < 1e-6,
        "the boxed loft's volume [{vlo}, {vhi}] holds its member's 8.4"
    );
}

/// The point loft's body at `T`, in construction state.
fn point_loft<T: Decide + Bounds + topo::AtRestPolicy>() -> Body<T> {
    let h = T::from_f64(1.2);
    sweep::test_support::loft_body_construction::<T>(
        &[quad(h), quad(h), quad(h)],
        &places(),
        2,
        tol(),
    )
    .expect("the loft builds")
    .body
}

/// `x` as `[lo, hi]`.
fn bounds<T: Bounds>(x: T) -> (f64, f64) {
    (x.lo(), x.hi())
}

/// **A split rim's cut is never read as a chart end**, at `f64` and at
/// `Interval`. The bottom cap's rim on the wall over the profile's
/// slanted side, split at a third of its span: each child's wall row
/// reads the rim's chart corner, exactly, at the end it keeps, and at the
/// cut a value strictly inside the chart, both as the split carries the
/// row; minted again, the kept corner is read through the measured map.
/// At `f64` the split body finishes and its volume holds the prism's 8.4.
#[test]
fn a_split_rims_cut_is_never_read_as_a_chart_end() {
    fn at<T: Decide + Bounds + geom_core::CertifiedBounds + topo::AtRestPolicy>(scalar: &str) {
        let mut body = point_loft::<T>();
        let (he, row) = body
            .pcurves()
            .find_map(|(he, c)| match c.pcurve() {
                row @ Pcurve::IsoLine {
                    ends: Some(_), pl, ..
                } if bounds(pl.y) == (0.0, 0.0) => Some((he, row.clone())),
                _ => None,
            })
            .expect("a wall rim stated by its ends");
        let [start, end] = row.span_ends(body.pcurve(he).unwrap().edge_span());
        let corners = (bounds(start.x), bounds(end.x));
        assert!(
            [((0.0, 0.0), (1.0, 1.0)), ((1.0, 1.0), (0.0, 0.0))].contains(&corners),
            "{scalar}: the rim runs from one chart corner to the other: {corners:?}"
        );
        let edge = body.get_half_edge(he).unwrap().edge;
        let wall = body.face_of_half_edge(he).unwrap();
        let (t0, t1) = body.pcurve(he).unwrap().params();
        let made = body
            .split_edge(edge, t0 + (t1 - t0) / T::from_f64(3.0), tol())
            .expect("the rim splits");
        // `exact`: each piece reads its kept corner as the point it is;
        // otherwise only an enclosure of it.
        let read = |body: &Body<T>, when: &str, exact: bool| {
            for piece in [edge, made.new_edge] {
                let e = body.get_edge(piece).unwrap();
                let he = [e.he_plus, e.he_minus]
                    .into_iter()
                    .find(|h| body.face_of_half_edge(*h) == Some(wall))
                    .expect("the piece bounds the wall");
                let cache = body.pcurve(he).expect("the piece's wall row is stored");
                let [a, b] = cache.pcurve().span_ends(cache.edge_span());
                let (kept, cut, corner) = if piece == edge {
                    (a, b, corners.0)
                } else {
                    (b, a, corners.1)
                };
                let (lo, hi) = bounds(cut.x);
                assert!(
                    lo > 0.0 && hi < 1.0,
                    "{scalar}, {when}: {piece:?}'s cut reads inside the chart, not at its \
                     edge: [{lo}, {hi}]"
                );
                let (lo, hi) = bounds(kept.x);
                assert!(
                    if exact {
                        (lo, hi) == corner
                    } else {
                        lo <= corner.0 && corner.1 <= hi
                    },
                    "{scalar}, {when}: {piece:?} keeps its chart corner {corner:?}: [{lo}, {hi}]"
                );
            }
        };
        read(&body, "as split", true);
        assert_eq!(
            topo::mint_pcurves(&mut body, tol()),
            Ok(()),
            "{scalar}: the mint pass"
        );
        // The mint pass derives a piece's row again by its measured end
        // feet, which state no ends: at `f64` the map reads the corner,
        // at `Interval` only an enclosure of it, and check 7 refuses the
        // wall (`work/boxfinish/a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map.md`).
        let interval = T::NAME == geom_core::Interval::NAME;
        read(&body, "minted again", !interval);
        let finished = topo::finish(body, tol());
        if interval {
            let refusal = finished.expect_err("the split body refuses at Interval");
            assert!(
                matches!(
                    refusal.as_slice(),
                    [topo::ValidationError::VolumeUncomputable { .. }]
                ),
                "{scalar}: check 7 refuses the re-minted wall: {refusal:?}"
            );
            return;
        }
        let finished = finished.expect("the split body finishes");
        let m = topo::mass_properties(
            &finished,
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .expect("the split body measures")
        .enclosure;
        let (lo, hi) = bounds(m.volume);
        let (lo, hi) = (lo - m.volume_pad, hi + m.volume_pad);
        assert!(
            lo - 1e-9 <= 8.4 && 8.4 <= hi + 1e-9 && hi - lo < 1e-6,
            "{scalar}: the split body's volume [{lo}, {hi}] holds the prism's 8.4"
        );
    }
    at::<f64>("f64");
    at::<Interval>("Interval");
}
