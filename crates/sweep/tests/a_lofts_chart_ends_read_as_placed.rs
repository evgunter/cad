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
use geom_core::{Bounds, Interval, Point2, Point3, Real, Tol};
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
/// holds the point loft's, which is a member of the family.
#[test]
fn a_loft_over_a_box_finishes_at_its_taker() {
    let (lo, hi) = box_about(1.2);
    let (vlo, vhi) = taken_volume(Interval::from_bounds(lo, hi));
    assert!(
        vlo <= 8.4 && 8.4 <= vhi,
        "the boxed loft's volume [{vlo}, {vhi}] holds its member's 8.4"
    );
}

/// The `f64` point loft's body.
fn point_loft() -> Body<f64> {
    sweep::loft_body::<f64>(&[quad(1.2), quad(1.2), quad(1.2)], &places(), 2, tol())
        .expect("the loft builds")
        .body
        .into_body()
}

/// **A split rim's cut is never read as a chart end.** The bottom cap's
/// rim on the wall over the profile's slanted side, split at a third of
/// its span: each child's wall row reads the rim's chart corner at the
/// end it keeps, and at the cut a value strictly inside the chart, both
/// as the split carries the row and after the rows are minted again; the
/// split body finishes and measures the unsplit body's volume.
#[test]
fn a_split_rims_cut_is_never_read_as_a_chart_end() {
    let mut body = point_loft();
    let (he, row) = body
        .pcurves()
        .find_map(|(he, c)| match c.pcurve() {
            row @ Pcurve::IsoLine {
                ends: Some(_), pl, ..
            } if pl.y == 0.0 => Some((he, row.clone())),
            _ => None,
        })
        .expect("a wall rim stated by its ends");
    let [start, end] = row.span_ends(body.pcurve(he).unwrap().params());
    let corners = (start.x, end.x);
    assert!(
        [(0.0, 1.0), (1.0, 0.0)].contains(&corners),
        "the rim runs from one chart corner to the other: {corners:?}"
    );
    let edge = body.get_half_edge(he).unwrap().edge;
    let wall = body.face_of_half_edge(he).unwrap();
    let (t0, t1) = body.pcurve(he).unwrap().params();
    let made = body
        .split_edge(edge, t0 + (t1 - t0) / 3.0, tol())
        .expect("the rim splits");
    let read = |body: &Body<f64>, when: &str| {
        let mut outer = Vec::new();
        for piece in [edge, made.new_edge] {
            let e = body.get_edge(piece).unwrap();
            let he = [e.he_plus, e.he_minus]
                .into_iter()
                .find(|h| body.face_of_half_edge(*h) == Some(wall))
                .expect("the piece bounds the wall");
            let cache = body.pcurve(he).expect("the piece's wall row is stored");
            let [a, b] = cache.pcurve().span_ends(cache.params());
            let (kept, cut) = if piece == edge { (a, b) } else { (b, a) };
            assert!(
                cut.x > 0.0 && cut.x < 1.0,
                "{when}: {piece:?}'s cut reads inside the chart, not at its edge: {cut:?}"
            );
            outer.push(kept.x);
        }
        assert_eq!(
            (outer[0], outer[1]),
            corners,
            "{when}: each piece keeps its own chart corner"
        );
    };
    read(&body, "as split");
    assert_eq!(
        topo::mint_pcurves(&mut body, tol()),
        Ok(()),
        "the mint pass"
    );
    read(&body, "minted again");
    let volume = |body: Body<f64>| {
        let finished = topo::finish(body, tol()).expect("the body finishes");
        topo::mass_properties(
            &finished,
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .expect("the body measures")
        .enclosure
        .volume
    };
    let (split, whole) = (volume(body), volume(point_loft()));
    assert!(
        (split - whole).abs() < 1e-9,
        "the split body's volume {split} is the unsplit's {whole}"
    );
}
