//! **A spline chart's certificate doors end a refusal by the carrier's
//! incidence read** (D3): a carrier off its wall at a read sample ends
//! `Incidence` read off, one read in band of it ends in the read's lever
//! (an `Incidence` read [`crate::IncidenceRead::Undecided`]), never as the image's
//! own bound, and one on it certifies as it did. Each door is read at
//! `f64` and at `Interval`:
//!
//! - [`PcurveCache::certify`] on a stated closed-form image (an iso
//!   column);
//! - [`PcurveCache::certify_general`] and [`PcurveCache::certify_fitted`]
//!   on a stated curve-in-UV;
//! - [`PcurveCache::recertify`], a row certified on the wall restated at
//!   rest against a carrier moved off it;
//! - the stated route, [`crate::read_incidence`] and the edge
//!   certificate's stated-image arm ([`crate::EdgeCurve::certify`] of
//!   [`crate::EdgeDescriptionSpec::chart_image`]);
//! - [`crate::read_point_incidence`].
//!
//! On a spline chart a certificate door reads the carrier where the image
//! refuses, or where the certificate's span is not shown to cover the
//! span it is asked about (`incidence_ended`): a certificate that holds
//! already bounds every carrier point of its own span within the band of
//! the wall.
//!
//! The wall is the unit cylinder `z ∈ [0, 1]` over `[0, 1.2]` rad in two
//! rational quadratic sub-arcs, whose knot `u = ½` is the ruling at
//! `0.6` rad. The carrier is that ruling moved out to radius `ρ`: on the
//! wall at `ρ = 1`, a millimetre off it, and a band-scale distance off.
//! The `spline-shown-unread` mutant (`read_shown`'s spline arm passing
//! every carrier unread) reds every off and in-band row.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Decide, Interval, Point2, Point3, Tol, Vec2, Vec3};

use super::{Pcurve, PcurveCache, PcurveCertifyError};
use crate::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, FittedLane, NurbsLane, SurfaceKey,
};

/// The certified doors a scalar that may certify holds, handed in by its
/// concrete row.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
struct Lanes<T: geom_core::Real> {
    fitted: FittedLane<T>,
    edge: fn(
        EdgeCurveSpec<T>,
        (Point3<T>, Point3<T>),
        &slotmap::SlotMap<SurfaceKey, Surface<T>>,
    ) -> Result<(), CertifyError>,
}

/// The ruling's angle, the wall's knot `u = ½`.
const RULING: f64 = 0.6;

pub(crate) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A distance inside the band: past `zero()`, short of `escalate()`.
pub(crate) fn in_band() -> f64 {
    let b = band();
    0.5 * (b.zero() + b.escalate())
}

/// Where the carrier is put: on the wall, a millimetre off, in band.
#[derive(Clone, Copy, Debug)]
enum Place {
    On,
    Off,
    InBand,
}

impl Place {
    fn radius(self) -> f64 {
        match self {
            Self::On => 1.0,
            Self::Off => 1.0 + 1e-3,
            Self::InBand => 1.0 + in_band(),
        }
    }
}

/// The wall (module docs).
pub(crate) fn ring<T: Decide>() -> Surface<T> {
    let h = 1.2 / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = f64::from(j) * h / 2.0;
        let (r, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for z in [0.0, 1.0] {
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// The ruling at `RULING` rad, at radius `place`'s, rising `z = t` over
/// `t ∈ [0, 1]`.
fn ruling<T: Decide>(place: Place) -> Curve3<T> {
    let (c, r) = (T::from_f64, place.radius());
    Curve3::line(
        Point3::new(c(r * RULING.cos()), c(r * RULING.sin()), c(0.0)),
        Vec3::new(c(0.0), c(0.0), c(1.0)),
    )
}

/// The ruling's column, `(½, t)`.
fn column<T: Decide>() -> Pcurve<T> {
    let c = T::from_f64;
    Pcurve::IsoLine {
        p0: Point2::new(c(0.5), c(0.0)),
        pl: Vec2::new(c(0.0), c(1.0)),
    }
}

/// The same column as a curve in UV, degree 1.
fn column_curve<T: Decide>() -> Arc<NurbsCurve2<T>> {
    let image = NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point2::new(0.5, 0.0), Point2::new(0.5, 1.0)],
        vec![1.0, 1.0],
    )
    .unwrap();
    Arc::new(image.map_scalar(T::from_f64))
}

/// The doors' readings for the ruling at `place`, each named.
fn doors<T: Decide>(
    place: Place,
    lanes: Lanes<T>,
) -> Vec<(&'static str, Result<(), PcurveCertifyError>)> {
    let (c, b) = (T::from_f64, band());
    let (wall, carrier) = (ring::<T>(), ruling::<T>(place));
    let (t0, t1) = (c(0.0), c(1.0));
    let lane = lanes.fitted;
    let stored = PcurveCache::certify(column(), t0, t1, &ruling::<T>(Place::On), &wall, b)
        .expect("the column certifies on the wall");
    let point = Point3::new(
        c(place.radius() * RULING.cos()),
        c(place.radius() * RULING.sin()),
        c(0.5),
    );
    vec![
        (
            "certify",
            PcurveCache::certify(column(), t0, t1, &carrier, &wall, b).map(|_| ()),
        ),
        (
            "certify_general",
            PcurveCache::certify_general(column_curve(), t0, t1, &carrier, &wall, b, Some(lane))
                .map(|_| ()),
        ),
        (
            "certify_fitted",
            PcurveCache::certify_fitted(column_curve(), t0, t1, &carrier, &wall, b, lane)
                .map(|_| ()),
        ),
        (
            "recertify",
            stored
                .recertify(&carrier, (t0, t1), &wall, b, Some(lane))
                .map(|_| ()),
        ),
        (
            "read_incidence",
            crate::read_incidence(&carrier, t0, t1, &wall, b),
        ),
        (
            "read_point_incidence",
            crate::read_point_incidence(point, &wall, b),
        ),
    ]
}

/// The edge certificate of the ruling at `place`, described by its
/// stated column on the wall.
fn stated<T: Decide>(place: Place, lanes: Lanes<T>) -> Result<(), CertifyError> {
    let c = T::from_f64;
    let mut table: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
    let key = table.insert(ring::<T>());
    let carrier = ruling::<T>(place);
    let ends = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(key, column()),
        carrier,
        param_start: c(0.0),
        param_end: c(1.0),
    };
    (lanes.edge)(spec, ends, &table)
}

/// The edge certificate's door at one scalar that may certify.
macro_rules! edge_at {
    ($name:ident, $t:ty) => {
        fn $name(
            spec: EdgeCurveSpec<$t>,
            (start, end): (Point3<$t>, Point3<$t>),
            table: &slotmap::SlotMap<SurfaceKey, Surface<$t>>,
        ) -> Result<(), CertifyError> {
            EdgeCurve::certify(
                spec,
                start,
                end,
                |k| table.get(k).cloned(),
                geom_core::test_support::fixture_lever(<$t as geom_core::Real>::one()),
                band(),
                NurbsLane::certified(),
            )
            .map(|_| ())
        }
    };
}
edge_at!(edge_f64, f64);
edge_at!(edge_interval, Interval);

/// Every door's reading at every place, the misreads gathered so each
/// door that misreads is named.
fn every_door_reads_the_wall<T: Decide>(lanes: Lanes<T>) {
    let mut misread = Vec::new();
    let mut check = |door: &str, place: Place, ok: bool, got: String| {
        if !ok {
            misread.push(format!("{door} at {place:?}: {got}"));
        }
    };
    for place in [Place::On, Place::Off, Place::InBand] {
        for (door, got) in doors::<T>(place, lanes) {
            let ok = match place {
                Place::On => got.is_ok(),
                Place::Off => got
                    .as_ref()
                    .err()
                    .is_some_and(|e| read_off(e, Some(geom::SurfaceKind::Nurbs), None)),
                Place::InBand => got.as_ref().err().is_some_and(read_in_band),
            };
            check(door, place, ok, format!("{got:?}"));
        }
        let got = stated::<T>(place, lanes);
        let ok = match place {
            Place::On => got.is_ok(),
            Place::Off => got.as_ref().err().is_some_and(|e| {
                edge_read_off(
                    e,
                    Some(geom::SurfaceKind::Nurbs),
                    Some(geom::CurveKind::Line),
                )
            }),
            Place::InBand => got.as_ref().err().is_some_and(edge_read_in_band),
        };
        check("stated", place, ok, format!("{got:?}"));
    }
    assert!(
        misread.is_empty(),
        "on the wall each door certifies, off it each ends off the chart, and in band of it \
         each ends in the read's lever, at {}:\n{}",
        T::NAME,
        misread.join("\n")
    );
}

#[test]
fn every_door_reads_the_wall_at_f64() {
    every_door_reads_the_wall::<f64>(Lanes {
        fitted: FittedLane::certified(),
        edge: edge_f64,
    });
}

#[test]
fn every_door_reads_the_wall_at_interval() {
    every_door_reads_the_wall::<Interval>(Lanes {
        fitted: FittedLane::certified(),
        edge: edge_interval,
    });
}

/// **At a dual, a curve in UV with no fitted door is read first where it
/// refuses**: on the wall it refuses `FittedLaneUnsupported`, the scalar
/// certifying nothing fitted; off it, the read ends it read off (`Incidence`),
/// so `topo`'s `not_owed`, which excuses the first, never excuses an off
/// carrier.
#[test]
fn at_a_dual_an_off_carrier_ends_off_the_chart_not_at_the_fitted_door() {
    type T = geom_core::Dual64;
    let (c, b) = (<T as geom_core::Real>::from_f64, band());
    let general = |place| {
        PcurveCache::certify_general(
            column_curve::<T>(),
            c(0.0),
            c(1.0),
            &ruling::<T>(place),
            &ring::<T>(),
            b,
            None,
        )
        .map(|_| ())
    };
    let on = general(Place::On);
    assert!(
        matches!(on, Err(PcurveCertifyError::FittedLaneUnsupported { .. })),
        "on the wall the dual has no fitted door: {on:?}"
    );
    let off = general(Place::Off);
    assert!(
        off.as_ref()
            .err()
            .is_some_and(|e| read_off(e, Some(geom::SurfaceKind::Nurbs), None)),
        "off the wall the read ends it: {off:?}"
    );
}

/// **A row re-certified for an edge longer than the row is read over the
/// edge** (`recertify`): the row certified over `t ∈ [0, 1]` holds, but
/// the edge runs to `t = 1.5`, off the bounded patch past `z = 1`, which
/// the row's certificate says nothing about.
fn a_row_shorter_than_its_edge_is_read_over_the_edge<T: Decide>(lane: FittedLane<T>) {
    let (c, b) = (T::from_f64, band());
    let (wall, carrier) = (ring::<T>(), ruling::<T>(Place::On));
    let row = PcurveCache::certify(column(), c(0.0), c(1.0), &carrier, &wall, b)
        .expect("the row certifies over its own span");
    let over_row = row.recertify(&carrier, (c(0.0), c(1.0)), &wall, b, Some(lane));
    assert!(
        over_row.is_ok(),
        "over the row's own span, at {}: {over_row:?}",
        T::NAME
    );
    let over_edge = row.recertify(&carrier, (c(0.0), c(1.5)), &wall, b, Some(lane));
    assert!(
        over_edge
            .as_ref()
            .err()
            .is_some_and(|e| read_off(e, Some(geom::SurfaceKind::Nurbs), None)),
        "over the edge, past the patch, at {}: {over_edge:?}",
        T::NAME
    );
}

/// **A reversed span asked about is read, not taken as covered**
/// (`covers`): the row certified over `t ∈ [0, 1]` holds no span that runs
/// backwards, so `recertify` over `(1.5, 0)` or `(1, −0.5)`, each reaching
/// past the patch, reads the carrier there and ends it off the chart.
fn a_reversed_edge_span_is_read<T: Decide>(lane: FittedLane<T>) {
    let (c, b) = (T::from_f64, band());
    let (wall, carrier) = (ring::<T>(), ruling::<T>(Place::On));
    let row = PcurveCache::certify(column(), c(0.0), c(1.0), &carrier, &wall, b)
        .expect("the row certifies over its own span");
    for (a0, a1) in [(1.5, 0.0), (1.0, -0.5)] {
        let got = row.recertify(&carrier, (c(a0), c(a1)), &wall, b, Some(lane));
        assert!(
            got.as_ref()
                .err()
                .is_some_and(|e| read_off(e, Some(geom::SurfaceKind::Nurbs), None)),
            "over the reversed span ({a0}, {a1}), past the patch, at {}: {got:?}",
            T::NAME
        );
    }
}

#[test]
fn a_reversed_edge_span_is_read_at_f64() {
    a_reversed_edge_span_is_read::<f64>(FittedLane::certified());
}

#[test]
fn a_reversed_edge_span_is_read_at_interval() {
    a_reversed_edge_span_is_read::<Interval>(FittedLane::certified());
}

#[test]
fn a_row_shorter_than_its_edge_is_read_over_the_edge_at_f64() {
    a_row_shorter_than_its_edge_is_read_over_the_edge::<f64>(FittedLane::certified());
}

#[test]
fn a_row_shorter_than_its_edge_is_read_over_the_edge_at_interval() {
    a_row_shorter_than_its_edge_is_read_over_the_edge::<Interval>(FittedLane::certified());
}

/// **A chart a few roundings wide reads without its foot**: the foot's
/// rectangle, shrunk inside the patch, is empty there, so the read takes
/// the walk alone, on the chart and off it, and never panics.
#[test]
fn a_chart_too_narrow_for_its_foot_is_walked() {
    let lo = 1.0f64;
    let hi = (0..4).fold(lo, |x, _| x.next_up());
    let net = NurbsSurface::new(
        KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let wall = Surface::Nurbs(Arc::new(net));
    let on = crate::read_point_incidence(Point3::new(0.5, 0.5, 0.0), &wall, band());
    assert_eq!(on, Ok(()), "a point on the narrow chart");
    let off = crate::read_point_incidence(Point3::new(0.5, 0.5, 1e-3), &wall, band());
    assert!(
        off.as_ref().err().is_some_and(|e| read_off(e, None, None)),
        "a point a millimetre off it: {off:?}"
    );
}

/// **On an analytic chart the fitted doors read the carrier first**, as
/// every closed-form door does: a curve in UV offered for a line off its
/// cylinder ends read off (`Incidence`), not the lane's refusal of an analytic
/// chart (`ImageMismatch`), which a line on it still takes.
#[test]
fn an_analytic_chart_reads_the_carrier_before_the_fitted_lane() {
    let b = band();
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let line = |r: f64| Curve3::line(Point3::new(r, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let general = |r| {
        PcurveCache::certify_general(
            column_curve::<f64>(),
            0.0,
            1.0,
            &line(r),
            &cylinder,
            b,
            Some(FittedLane::certified()),
        )
        .map(|_| ())
    };
    let on = general(1.0);
    assert!(
        matches!(on, Err(PcurveCertifyError::ImageMismatch { .. })),
        "a line on the cylinder takes the lane's refusal: {on:?}"
    );
    let off = general(1.0 + 1e-3);
    assert!(
        off.as_ref()
            .err()
            .is_some_and(|e| read_off(e, Some(geom::SurfaceKind::Cylinder), None)),
        "a line off the cylinder ends off the chart: {off:?}"
    );
}

fn read_off(
    e: &crate::PcurveCertifyError,
    chart: Option<geom::SurfaceKind>,
    carrier: Option<geom::CurveKind>,
) -> bool {
    let crate::PcurveCertifyError::Incidence(r) = e else {
        return false;
    };
    matches!(r.read(), crate::IncidenceRead::Off(_)) && kinds_are(r.kinds(), chart, carrier)
}

fn read_in_band(e: &crate::PcurveCertifyError) -> bool {
    matches!(
        crate::IncidenceRead::of(e),
        Some(crate::IncidenceRead::Undecided(_))
    )
}

fn edge_read_off(
    e: &crate::CertifyError,
    chart: Option<geom::SurfaceKind>,
    carrier: Option<geom::CurveKind>,
) -> bool {
    let crate::CertifyError::Incidence(r) = e else {
        return false;
    };
    matches!(r.read(), crate::IncidenceRead::Off(_)) && kinds_are(r.kinds(), chart, carrier)
}

fn edge_read_in_band(e: &crate::CertifyError) -> bool {
    matches!(e.incidence_read(), Some(crate::IncidenceRead::Undecided(_)))
}

fn kinds_are(
    kinds: Option<(geom::SurfaceKind, geom::CurveKind)>,
    chart: Option<geom::SurfaceKind>,
    carrier: Option<geom::CurveKind>,
) -> bool {
    kinds.is_some_and(|(c, k)| chart.is_none_or(|x| x == c) && carrier.is_none_or(|x| x == k))
}
