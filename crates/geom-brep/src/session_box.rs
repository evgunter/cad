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

/// The box's one recourse: the box bounds the model, not the tolerance,
/// so the geometry moves.
pub const SESSION_BOX_RECOURSE: &str =
    "Recourse: place the geometry nearer the origin, or model it at a smaller size";

/// **Why a region is not shown in the session box.** `U` names what has
/// no cover and why: a face of a body with its cover's refusal, or an
/// edge's own refusal ([`EdgeUncovered`]).
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
    /// Part of the region has no cover, and no point read on it is
    /// definitely past the box. Its standing is `U`'s cause: a limit
    /// where the reading is not built, a defect where the data is not
    /// finite. A point that is not finite reads nothing, so a region one
    /// such point leaves uncovered is still refused as past the box by
    /// any other point of it that is.
    Uncovered(U),
}

/// **The margin of `reach` past the box, where it is definitely past**:
/// `name`'s decision on the box's extent less `reach`. Geometry within
/// the band of the box's edge is in it.
pub fn past_the_box_as<T: Decide>(name: &'static str, reach: T, band: Band) -> Option<MarginDiag> {
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
        Ok(reach) => match past_the_box_as(SESSION_BOX_REACH, reach, band) {
            None => return Ok(()),
            Some(margin) => BoxRefusal::Unshown { margin },
        },
        Err(what) => BoxRefusal::Uncovered(what),
    };
    match past_the_box_as(SESSION_BOX_POINT, inside(), band) {
        Some(margin) => Err(BoxRefusal::Point { margin }),
        None => Err(unshown),
    }
}

/// The largest `|coordinate|` of a cover: of its box along the world
/// axes ([`crate::box_reach`]).
#[must_use]
pub fn cover_reach<T: Decide>(cover: &Cover<T>) -> T {
    crate::box_reach(cover.box_in(&BoxAxes::World))
}

/// **Why an edge's region has no cover** ([`edge_cover`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeUncovered {
    /// The carrier's own reading over the span refused.
    Carrier(geom::cover::Uncovered),
    /// An end is not a finite point.
    Ends,
}

impl EdgeUncovered {
    /// The refusal's cause, as a value: what certification's refusal
    /// carries ([`crate::CertifyError::SessionBoxUncovered`]).
    #[must_use]
    pub fn cause(&self) -> UncoveredCause {
        use geom::cover::{Entity, Unknown};
        match self {
            Self::Carrier(why) => {
                let unknowns = why.unknowns();
                let kind = unknowns
                    .first()
                    .map_or("unknown", |(entity, _)| match entity {
                        Entity::Surface(kind) => kind.name(),
                        Entity::Curve(kind) => kind.name(),
                    });
                let what = unknowns
                    .first()
                    .map_or(Unknown::NonFinite, |&(_, what)| what);
                UncoveredCause::Carrier {
                    kind,
                    what,
                    unbounded_only: unknowns
                        .iter()
                        .all(|(_, what)| matches!(what, Unknown::Unbounded)),
                }
            }
            Self::Ends => UncoveredCause::Ends,
        }
    }

    /// **This refusal's standing, by its cause** ([`UncoveredCause::refusal`]).
    ///
    /// # Errors
    ///
    /// [`geom_core::ladder::Aborts`] for a stored-data fault and a mixed
    /// carrier refusal.
    pub fn refusal(&self) -> Result<geom_core::ladder::Refusal, geom_core::ladder::Aborts> {
        self.cause().refusal()
    }
}

impl core::fmt::Display for EdgeUncovered {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Carrier(why) => write!(f, "the edge's carrier has {why}"),
            Self::Ends => f.write_str("an end vertex of the edge is not a finite point"),
        }
    }
}

/// **Why an edge has no cover, as a value** ([`EdgeUncovered::cause`]):
/// the carrier's kind and the first thing not known of it, or an end
/// that is not a finite point. A carrier refusal can name several
/// unknowns; the value shows the first (its `kind` and `what`, which is
/// all the message names) and stands for all of them (`unbounded_only`
/// is read over every one, and decides [`UncoveredCause::refusal`]), so
/// a mixed refusal shows one cause and aborts as the mix it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UncoveredCause {
    /// The carrier's reading over the span refused.
    Carrier {
        /// The kind of the first entity not known, by name.
        kind: &'static str,
        /// The first thing not known about it, the one shown.
        what: geom::cover::Unknown,
        /// Whether every unknown, not only the one shown, is an
        /// unbounded locus.
        unbounded_only: bool,
    },
    /// An end is not a finite point.
    Ends,
}

impl UncoveredCause {
    /// **The standing of an edge with no cover, by its cause**
    /// ([`geom_core::ladder::Refusal`]): a carrier whose every unknown is
    /// an unbounded locus is a reading not built (row 2), a limit; a
    /// control net carrying poison, a datum that is not a finite number
    /// and an end that is not a finite point are stored-data faults
    /// (row 4), and a carrier refusal that mixes the two classes aborts
    /// until it is split.
    ///
    /// # Errors
    ///
    /// [`geom_core::ladder::Aborts`] for a stored-data fault and a mixed
    /// carrier refusal.
    pub fn refusal(self) -> Result<geom_core::ladder::Refusal, geom_core::ladder::Aborts> {
        match self {
            Self::Carrier {
                unbounded_only: true,
                ..
            } => Ok(geom_core::ladder::Refusal::Limit),
            Self::Carrier { .. } | Self::Ends => Err(geom_core::ladder::Aborts),
        }
    }
}

impl core::fmt::Display for UncoveredCause {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use geom::cover::Unknown;
        match self {
            Self::Carrier { kind, what, .. } => {
                let what = match what {
                    Unknown::Unbounded => "its locus is unbounded",
                    Unknown::PoisonedNet => "its control net bounds no region",
                    Unknown::NonFinite => "a datum of it is not a finite number",
                };
                write!(
                    f,
                    "the edge's {kind} carrier has no cover over its span: {what}"
                )
            }
            Self::Ends => f.write_str("an end vertex of the edge is not a finite point"),
        }
    }
}

/// **An edge's region, read from outside**: `carrier`'s cover over
/// `span` hulled with the two `ends`. The one construction of an edge's
/// cover: certification's ask ([`edge_in_box`]) and topo's every reader
/// of an edge's cover (`topo::cover::edge_cover_over`) build it here.
///
/// # Errors
///
/// [`EdgeUncovered`]: the carrier's reading refused, or an end is not a
/// finite point.
pub fn edge_cover<T: geom_core::spline::SpanLocate>(
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    (a, b): (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    let arc = carrier
        .cover_over(ParamWindow::new(t0, t1))
        .map_err(EdgeUncovered::Carrier)?;
    let ends = Cover::points(vec![a, b]).ok_or(EdgeUncovered::Ends)?;
    Ok(arc.hull(ends))
}

/// **An edge's region, read from inside**: the largest `|coordinate|`
/// of points known to lie on it, its two `ends` and its carrier's own
/// points over `span` ([`crate::edge_coordinate_reach_below`], which
/// reads `cover`'s box where it is the edge's own). A point that is not
/// finite reads nothing, so it never hides another point past the box;
/// with none finite the reach is zero, past nothing. The one inside read
/// of an edge: certification's ask and the finished-body fold
/// (`topo::session_box`) read it here.
#[must_use]
pub fn edge_points_reach<T: Decide>(
    carrier: &Curve3<T>,
    span: (T, T),
    (a, b): (Point3<T>, Point3<T>),
    cover: Option<&Cover<T>>,
) -> T {
    let origin = Point3::origin();
    let on_it =
        crate::edge_coordinate_reach_below(carrier, span, cover.map(|c| c.box_in(&BoxAxes::World)));
    [on_it, (a - origin).norm_inf(), (b - origin).norm_inf()]
        .into_iter()
        .filter(|&x| geom_core::is_finite_length(x))
        .fold(T::zero(), T::max)
}

/// **An edge's region in the box**: `carrier` over `span` with its two
/// `ends`, read from outside as its cover ([`edge_cover`]) and from
/// inside off the points known to lie on it ([`edge_points_reach`]).
///
/// # Errors
///
/// The [`BoxRefusal`]: [`BoxRefusal::Point`] where a finite point of the
/// edge is past the box, whether or not the edge has a cover;
/// [`BoxRefusal::Uncovered`] with its cause where it has none and no
/// finite point of it is past the box.
pub fn edge_in_box<T: Decide>(
    carrier: &Curve3<T>,
    span: (T, T),
    ends: (Point3<T>, Point3<T>),
    band: Band,
) -> Result<(), BoxRefusal<EdgeUncovered>> {
    let cover = edge_cover(carrier, span, ends);
    region_in_box(
        cover.as_ref().map(cover_reach).map_err(Clone::clone),
        || edge_points_reach(carrier, span, ends, cover.as_ref().ok()),
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
        let (line, len) = geom::Line::chord(a, b);
        (Curve3::Line(line), (0.0, len))
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
        let stored = Curve3::line(Point3::new(1e5, 0.0, 0.0), geom_core::UnitVec3::exact_x());
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

    /// The five certification doors on one spec and its ends: `certify`,
    /// `certify_stated`, `certify_carrying`, and `recertify`,
    /// `recertify_stated` of `held`.
    fn every_door(
        spec: &crate::EdgeCurveSpec<f64>,
        held: &crate::EdgeCurve<f64>,
        (a, b): (Point3<f64>, Point3<f64>),
    ) -> [Result<(), crate::CertifyError>; 5] {
        use crate::{EdgeCurve, NurbsLane};
        let lever = || geom_core::test_support::fixture_lever(1.0);
        let lane = NurbsLane::certified;
        [
            EdgeCurve::certify(spec.clone(), a, b, |_| None, lever(), band(), lane()).map(drop),
            EdgeCurve::certify_stated(spec.clone(), a, b, |_| None, lever(), band(), lane(), None)
                .map(drop),
            EdgeCurve::certify_carrying(
                spec.clone(),
                a,
                b,
                |_| None,
                lever(),
                band(),
                lane(),
                None,
                &[],
            )
            .map(drop),
            held.recertify(a, b, |_| None, lever(), band(), lane())
                .map(drop),
            held.recertify_stated(a, b, |_| None, lever(), band(), lane(), None)
                .map(drop),
        ]
    }

    /// **Every certification door asks the box first, of the edge with its
    /// ends.** A line edge 2 km out whose stated end misses its carrier by
    /// a metre: each of the five doors (`certify`, `certify_stated`,
    /// `certify_carrying`, `recertify`, `recertify_stated`) refuses it as
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
            Curve3::line(Point3::new(2e3, 0.0, 0.0), geom_core::UnitVec3::exact_x()),
            1.0,
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

    /// The degree-2 spline through `p0`, `p1` (its middle control) and
    /// `p2` over `[0, 1]`.
    fn quadratic(p0: Point3<f64>, p1: Point3<f64>, p2: Point3<f64>) -> Curve3<f64> {
        let knots =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(knots, vec![p0, p1, p2], vec![1.0; 3]).unwrap(),
        ))
    }

    /// **An end alone past the box is read from inside, whatever the
    /// carrier**: a spline in the box (its cover not the edge's own, so
    /// the inside read is its samples) with a stated end 2 km out is past
    /// the box by that end, not a limit. *Breaks if* the inside read drops
    /// the ends (on a line or an arc the cover's box already holds them).
    #[test]
    fn a_splines_end_alone_past_the_box_is_a_point_past_it() {
        let o = Point3::origin();
        let carrier = quadratic(o, Point3::new(0.5, 0.5, 0.0), Point3::new(1.0, 0.0, 0.0));
        let got = edge_in_box(
            &carrier,
            (0.0, 1.0),
            (o, Point3::new(2e3, 0.0, 0.0)),
            band(),
        );
        assert!(matches!(got, Err(BoxRefusal::Point { .. })), "{got:?}");
    }

    /// **A point that is not finite hides no other point past the box**:
    /// a line over 5 km from the origin, its start stated as NaN, has no
    /// cover (by its ends), and its inside read skips the NaN end and
    /// finds the far one, so it is past the box by it, about 4 km, not an
    /// edge with no cover. *Breaks if* a non-finite point poisons the
    /// inside read ([`Real::max`](geom_core::Real::max) propagates NaN).
    #[test]
    fn a_nan_end_beside_an_end_5_km_out_is_past_the_box_by_the_far_end() {
        let far = Point3::new(5e3, 0.0, 0.0);
        let (line, span) = line(Point3::origin(), far);
        let nan = Point3::new(f64::NAN, 0.0, 0.0);
        let got = edge_in_box(&line, span, (nan, far), band());
        let Err(BoxRefusal::Point { margin }) = got else {
            panic!("the far end is past the box: {got:?}");
        };
        assert!(
            margin.readable_below_zero().to_string().starts_with('4'),
            "{got:?}"
        );
    }

    /// **Geometry within the band of the box's edge is in it, and past
    /// the band is past it**: a segment ending half a band's zero beyond
    /// 1 km, or as far short of it, is in the box; one ending ten band
    /// widths beyond it is past it, by that end. *Breaks if* a margin the
    /// band reads as zero refuses.
    #[test]
    fn the_boxs_edge_is_read_in_the_band() {
        let o = Point3::origin();
        let band = band();
        let at = |x: f64| {
            let (carrier, span) = line(o, Point3::new(x, 0.0, 0.0));
            edge_in_box(&carrier, span, (o, Point3::new(x, 0.0, 0.0)), band)
        };
        for x in [1e3 - 0.5 * band.zero(), 1e3 + 0.5 * band.zero()] {
            assert_eq!(
                at(x),
                Ok(()),
                "an end at {x} m is within the band of the box's edge"
            );
        }
        let past = 1e3 + 10.0 * band.escalate();
        assert!(
            matches!(at(past), Err(BoxRefusal::Point { .. })),
            "an end at {past} m is past the box: {:?}",
            at(past)
        );
    }

    /// **An ellipse arc whose crest alone is past the box is a point past
    /// it**: an arc of semi-axis `1000.003 m` about the origin whose span
    /// straddles its crest between two of its sixteen samples (each short
    /// of the box) reads its cover's box, which is the arc's own coordinate
    /// range (its support along each axis is the arc's exact extent), so
    /// its crest is 3 mm past the box. *Breaks if* the inside read of an
    /// ellipse is its samples alone, which call the arc a limit.
    #[test]
    fn an_ellipses_crest_between_its_samples_is_a_point_past_the_box() {
        let carrier = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            1000.003,
            1.0,
        ));
        let span = (-0.03, 0.037);
        let ends = (carrier.eval(span.0), carrier.eval(span.1));
        let got = edge_in_box(&carrier, span, ends, band());
        let Err(BoxRefusal::Point { margin }) = got else {
            panic!("the crest is past the box: {got:?}");
        };
        let past = margin.readable_below_zero().to_string();
        assert!(past.starts_with("0.003"), "3 mm past the box: {past}");
    }

    /// **A limit at certification ends as the gate's does, at the three
    /// minting doors**: a quadratic spline whose curve peaks 995 m out
    /// while the net of the piece about its peak reaches past 1 km (the
    /// cover is the net met with the pieces' nets, four to a window) is
    /// the box's limit, [`crate::CertifyError::SessionBoxUnshown`], at
    /// `certify`, `certify_stated` and `certify_carrying`, before any
    /// check, and ends in the box's one recourse. The two recertifying
    /// doors are not asked here: they read a held edge, and a spline
    /// carrier on a line's description certifies nowhere (`Unimplemented`),
    /// so no held spline edge can be minted in this crate's rows; they run
    /// the same check 0, which
    /// `every_certification_door_asks_the_box_before_any_residual` pins
    /// at all five. *Breaks if* the limit is admitted at a minting door,
    /// or ends otherwise.
    #[test]
    fn a_splines_cover_alone_past_the_box_is_the_limit_at_the_three_minting_doors() {
        use crate::{CertifyError, EdgeCurveSpec};
        let o = Point3::origin();
        let carrier = quadratic(o, Point3::new(1990.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
        let (t0, t1) = (0.0, 0.9);
        let (a, b) = (carrier.eval(t0), carrier.eval(t1));
        let spec = EdgeCurveSpec {
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
            ..EdgeCurveSpec::line_between(a, b)
        };
        let held = crate::EdgeCurve::certify(
            EdgeCurveSpec::line_between(o, Point3::new(1.0, 0.0, 0.0)),
            o,
            Point3::new(1.0, 0.0, 0.0),
            |_| None,
            geom_core::test_support::fixture_lever(1.0),
            band(),
            crate::NurbsLane::certified(),
        )
        .unwrap();
        let minting = 3;
        for (door, got) in every_door(&spec, &held, (a, b))
            .into_iter()
            .enumerate()
            .take(minting)
        {
            let Err(e @ CertifyError::SessionBoxUnshown { .. }) = got else {
                panic!("door {door}: the box's limit: {got:?}");
            };
            assert_eq!(
                e.ending(crate::recourse::Reading::Build),
                Some(SESSION_BOX_RECOURSE.to_owned()),
                "door {door}"
            );
        }
    }
}
