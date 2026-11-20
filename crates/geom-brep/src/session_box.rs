//! **The session box's decision** (`crates/geom-core/README.md`, "Float
//! rounding", "The session box"): the one core every ask of the box
//! reaches, whatever region it is asked of. A region is read from
//! outside as its cover and from inside off points known to lie on it.
//! It is in the box only where its cover is; one with a point past it is
//! refused as past the box ([`BoxRefusal::Point`]); one whose cover alone
//! reaches past it is refused as a limit ([`BoxRefusal::Unshown`],
//! [`BoxRefusal::Uncovered`]).
//!
//! Two regions ask it: an edge's at its certification ([`edge_in_box`],
//! which `crate::certify`'s engine asks before any check), and a body's
//! faces at the finished-body gate (`topo::session_box`'s fold, which
//! reads the cover and the points and hands them to [`region_in_box`]).

use geom::Curve3;
use geom::cover::{BoxAxes, Cover, ParamWindow};
use geom_core::{Band, Decide, Lever, Margin, MarginDiag, Point3, Sign};

/// The decision on a region's cover: its margin the box's extent less
/// the cover's largest `|coordinate|`.
pub const SESSION_BOX_REACH: &str = "session_box_reach";

/// The decision on points known to lie on a region, asked where its
/// cover reaches past the box: its margin the box's extent less their
/// largest `|coordinate|`.
pub const SESSION_BOX_POINT: &str = "session_box_point";

/// **Why a region is not shown in the session box.** `U` names what has
/// no cover: a face of a body, or nothing for an edge, which is one
/// region.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoxRefusal<U> {
    /// A point known to lie on the region is definitely past the box.
    Point {
        /// The box's extent less the points' reach, metres (negative).
        margin: MarginDiag,
    },
    /// The region's cover reaches definitely past the box and no point
    /// known to lie on it does: a limit, not a finding.
    Unshown {
        /// The box's extent less the cover's reach, metres (negative).
        margin: MarginDiag,
    },
    /// Part of the region has no cover, and no point known to lie on it
    /// is past the box: a limit.
    Uncovered(U),
}

/// **The margin of `reach` past the box, where it is definitely past**:
/// `name`'s decision on the box's extent less `reach`. Geometry within
/// the band of the box's edge is in it.
pub fn past_the_box<T: Decide>(name: &'static str, reach: T, band: Band) -> Option<MarginDiag> {
    let margin = Margin::of(Lever::<T>::session_box().length() - reach);
    match geom_core::k_stats::decide_reported(name, margin, band) {
        Ok(geom_core::Decided {
            sign: Sign::Negative,
            margin,
        }) => Some(margin),
        _ => None,
    }
}

/// **The decision core**: a region whose cover reaches `cover` (its
/// largest `|coordinate|`, or what has no cover) is in the box where
/// that is not definitely past it; otherwise `inside`, the largest
/// `|coordinate|` of points known to lie on the region, decides which
/// refusal it is. `inside` is read only where the cover does not decide.
///
/// # Errors
///
/// The [`BoxRefusal`].
pub fn region_in_box<T: Decide, U>(
    cover: Result<T, U>,
    inside: impl FnOnce() -> T,
    band: Band,
) -> Result<(), BoxRefusal<U>> {
    let unshown = match cover {
        Ok(reach) => match past_the_box(SESSION_BOX_REACH, reach, band) {
            None => return Ok(()),
            Some(margin) => BoxRefusal::Unshown { margin },
        },
        Err(what) => BoxRefusal::Uncovered(what),
    };
    match past_the_box(SESSION_BOX_POINT, inside(), band) {
        Some(margin) => Err(BoxRefusal::Point { margin }),
        None => Err(unshown),
    }
}

/// The largest `|coordinate|` of a cover: of its box along the world
/// axes.
#[must_use]
pub fn cover_reach<T: Decide>(cover: &Cover<T>) -> T {
    let origin = Point3::origin();
    let (lo, hi) = cover.box_in(&BoxAxes::World);
    (lo - origin).norm_inf().max((hi - origin).norm_inf())
}

/// **An edge's region in the box**: `carrier` over `span` with its two
/// `ends`, read from outside as the carrier's cover over the span hulled
/// with the ends (topo's `cover::edge_cover_over`), and from inside off
/// the ends and the carrier's own points over the span
/// ([`crate::edge_coordinate_reach_below`]).
///
/// # Errors
///
/// The [`BoxRefusal`]; [`BoxRefusal::Uncovered`] where the carrier or an
/// end is no finite cover, which certification leaves to its checks.
pub fn edge_in_box<T: Decide>(
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    (a, b): (Point3<T>, Point3<T>),
    band: Band,
) -> Result<(), BoxRefusal<()>> {
    let cover = carrier
        .cover_over(ParamWindow::new(t0, t1))
        .ok()
        .zip(Cover::points(vec![a, b]))
        .map(|(arc, ends)| arc.hull(ends));
    let reach = cover.as_ref().map(cover_reach).ok_or(());
    region_in_box(
        reach,
        || {
            let origin = Point3::origin();
            crate::edge_coordinate_reach_below(
                carrier,
                (t0, t1),
                cover.as_ref().map(|c| c.box_in(&BoxAxes::World)),
            )
            .max((a - origin).norm_inf())
            .max((b - origin).norm_inf())
        },
        band,
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn line(a: Point3<f64>, b: Point3<f64>) -> (Curve3<f64>, (f64, f64)) {
        let d = b - a;
        let len = d.norm();
        (Curve3::line(a, d / len), (0.0, len))
    }

    /// **An edge is read with its ends, its carrier over its span, and
    /// nothing else.** A segment from the origin to 1001 m is past the
    /// box by a point (its end), by 1 m; one to 999 m is in it. A short
    /// segment whose carrier's origin is stored 1e5 m out but whose span
    /// stands at the origin is in it: the box reads the region, never a
    /// carrier's stored terms. *Breaks if* the stored origin is read, or
    /// if the box's edge moves (the ends are pinned by the next row).
    #[test]
    fn an_edge_is_read_over_its_span_with_its_ends() {
        let o = Point3::origin();
        let (far, span) = line(o, Point3::new(1001.0, 0.0, 0.0));
        let Err(BoxRefusal::Point { margin }) =
            edge_in_box(&far, span, (o, Point3::new(1001.0, 0.0, 0.0)), band())
        else {
            panic!("an end 1 m past the box is a point past it");
        };
        assert!(margin.readable_below_zero().to_string().starts_with('1'));
        let (near, span) = line(o, Point3::new(999.0, 0.0, 0.0));
        assert_eq!(
            edge_in_box(&near, span, (o, Point3::new(999.0, 0.0, 0.0)), band()),
            Ok(())
        );
        let stored = Curve3::line(Point3::new(1e5, 0.0, 0.0), Vec3::unit_x());
        let ends = (Point3::origin(), Point3::new(1.0, 0.0, 0.0));
        assert_eq!(
            edge_in_box(&stored, (-1e5, -1e5 + 1.0), ends, band()),
            Ok(())
        );
    }

    /// **An end alone past the box refuses**: a segment in the box whose
    /// stated end vertex stands 2 km out (no residual has yet been read
    /// that would refuse the pair) is past the box by that point. *Breaks
    /// if* the ends are left out of the edge's region.
    #[test]
    fn an_end_past_the_box_refuses() {
        let o = Point3::origin();
        let (carrier, span) = line(o, Point3::new(1.0, 0.0, 0.0));
        let far_end = Point3::new(2e3, 0.0, 0.0);
        assert!(matches!(
            edge_in_box(&carrier, span, (o, far_end), band()),
            Err(BoxRefusal::Point { .. })
        ));
    }

    /// **A cover alone past the box is a limit, not a finding**: the
    /// core given a cover past the box and points inside it refuses as
    /// [`BoxRefusal::Unshown`]; given points past it too, as
    /// [`BoxRefusal::Point`]; and given no cover and points inside, as
    /// [`BoxRefusal::Uncovered`]. *Breaks if* the limit is refused as
    /// past the box, or the cover's verdict is skipped.
    #[test]
    fn a_cover_alone_past_the_box_is_a_limit() {
        let b = band();
        assert!(matches!(
            region_in_box::<f64, ()>(Ok(1.5e3), || 900.0, b),
            Err(BoxRefusal::Unshown { .. })
        ));
        assert!(matches!(
            region_in_box::<f64, ()>(Ok(1.5e3), || 1.2e3, b),
            Err(BoxRefusal::Point { .. })
        ));
        assert_eq!(
            region_in_box::<f64, &str>(Err("face"), || 900.0, b),
            Err(BoxRefusal::Uncovered("face"))
        );
        let mut read = false;
        assert_eq!(
            region_in_box::<f64, ()>(
                Ok(999.0),
                || {
                    read = true;
                    2e3
                },
                b
            ),
            Ok(())
        );
        assert!(!read, "a cover in the box decides alone");
    }

    /// **A circle arc's far side is read, not only its ends**: a half
    /// circle of radius 600 m centred 500 m out, from (500, 600) to
    /// (500, −600) through (1100, 0), has both ends in the box and its
    /// middle 100 m past it. *Breaks if* the inside read takes the ends
    /// alone.
    #[test]
    fn an_arcs_bulge_past_the_box_is_a_point_past_it() {
        let c = Point3::new(500.0, 0.0, 0.0);
        let arc = Curve3::circle(c, Vec3::unit_z(), 600.0, Vec3::unit_x());
        let span = (-core::f64::consts::FRAC_PI_2, core::f64::consts::FRAC_PI_2);
        let ends = (arc.eval(span.0), arc.eval(span.1));
        let Err(BoxRefusal::Point { margin }) = edge_in_box(&arc, span, ends, band()) else {
            panic!("the arc's middle is 100 m past the box");
        };
        assert!(margin.readable_below_zero().to_string().starts_with("100"));
    }

    /// The four certification doors on one spec and its ends: `certify`,
    /// `certify_stated`, and `recertify`, `recertify_stated` of `held`.
    fn every_door(
        spec: &crate::EdgeCurveSpec<f64>,
        held: &crate::EdgeCurve<f64>,
        (a, b): (Point3<f64>, Point3<f64>),
    ) -> [Result<(), crate::CertifyError>; 4] {
        use crate::{EdgeCurve, NurbsLane};
        let lever = || geom_core::test_support::fixture_lever(1.0);
        let lane = NurbsLane::certified;
        [
            EdgeCurve::certify(spec.clone(), a, b, |_| None, lever(), band(), lane()).map(drop),
            EdgeCurve::certify_stated(spec.clone(), a, b, |_| None, lever(), band(), lane(), None)
                .map(drop),
            held.recertify(a, b, |_| None, lever(), band(), lane())
                .map(drop),
            held.recertify_stated(a, b, |_| None, lever(), band(), lane(), None)
                .map(drop),
        ]
    }

    /// **Every certification door asks the box first, of the edge with its
    /// ends.** A line edge 2 km out whose stated end misses its carrier by
    /// a metre: each of the four doors refuses it as
    /// [`crate::CertifyError::OutsideSessionBox`], not as the endpoint
    /// residual a check after the box would read. The same edge moved home
    /// (by `EdgeCurve::carried`, the image a move holds) certifies at each
    /// door with its true ends, and with an end 2 km out refuses as past
    /// the box: the ends are read, not only the carrier. *Breaks if* the
    /// ask is dropped at one door, moved after the residuals, or reads the
    /// carrier without its ends.
    #[test]
    fn every_certification_door_asks_the_box_before_any_residual() {
        use crate::{CertifyError, EdgeCurve, EdgeCurveSpec};
        let (p, q) = (Point3::new(2e3, 0.0, 0.0), Point3::new(2e3 + 1.0, 0.0, 0.0));
        let home = EdgeCurve::certify(
            EdgeCurveSpec::line_between(Point3::origin(), Point3::new(1.0, 0.0, 0.0)),
            Point3::origin(),
            Point3::new(1.0, 0.0, 0.0),
            |_| None,
            geom_core::test_support::fixture_lever(1.0),
            band(),
            crate::NurbsLane::certified(),
        )
        .unwrap();
        let far = home.carried(
            Curve3::line(Point3::new(2e3, 0.0, 0.0), Vec3::unit_x()),
            home.description().clone(),
            home.authority(),
        );
        let missed = Point3::new(2e3 + 1.0, 1.0, 0.0);
        for (door, got) in every_door(&EdgeCurveSpec::line_between(p, q), &far, (p, missed))
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Err(CertifyError::OutsideSessionBox { .. })),
                "door {door}: the box's refusal before the endpoint residual: {got:?}"
            );
        }
        let (o, x) = (Point3::origin(), Point3::new(1.0, 0.0, 0.0));
        let spec = EdgeCurveSpec::line_between(o, x);
        for (door, got) in every_door(&spec, &home, (o, x)).into_iter().enumerate() {
            assert!(
                got.is_ok(),
                "door {door}: an edge at home certifies: {got:?}"
            );
        }
        for (door, got) in every_door(&spec, &home, (o, Point3::new(2e3, 0.0, 0.0)))
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Err(CertifyError::OutsideSessionBox { .. })),
                "door {door}: an end 2 km out is past the box: {got:?}"
            );
        }
    }
}
