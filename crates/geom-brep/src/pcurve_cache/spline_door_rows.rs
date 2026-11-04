//! **A spline chart's certificate doors read the carrier's incidence
//! first** (D3): a carrier off its wall ends `CarrierOffChart`, one read
//! in band of it ends in the read's lever (`Escalated`,
//! [`PcurveCheck::ChartIncidence`]), never as the image's own bound, and
//! one on it certifies as it did. Each door is read at `f64` and at
//! `Interval`:
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
//! On a spline chart a certificate door meters the image first and reads
//! the carrier only where the image refuses (`ended_by_incidence`): a
//! certificate that holds already bounds every carrier point within the
//! band of the wall.
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

use super::{Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck};
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

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A distance inside the band: past `zero()`, short of `escalate()`.
fn in_band() -> f64 {
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
fn ring<T: Decide>() -> Surface<T> {
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
                Place::Off => matches!(
                    got,
                    Err(PcurveCertifyError::CarrierOffChart {
                        chart: geom::SurfaceKind::Nurbs,
                        ..
                    })
                ),
                Place::InBand => matches!(
                    got,
                    Err(PcurveCertifyError::Escalated {
                        check: PcurveCheck::ChartIncidence,
                        ..
                    })
                ),
            };
            check(door, place, ok, format!("{got:?}"));
        }
        let got = stated::<T>(place, lanes);
        let ok = match place {
            Place::On => got.is_ok(),
            Place::Off => matches!(
                got,
                Err(CertifyError::CarrierOffChart {
                    chart: geom::SurfaceKind::Nurbs,
                    carrier: geom::CurveKind::Line,
                    ..
                })
            ),
            Place::InBand => matches!(
                got,
                Err(CertifyError::Escalated {
                    check: crate::CertCheck::ChartIncidence,
                    ..
                })
            ),
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
/// certifying nothing fitted; off it, the read ends it `CarrierOffChart`,
/// so `topo`'s `not_owed`, which excuses the first, never excuses an off
/// carrier.
#[test]
fn at_a_dual_an_off_carrier_ends_off_the_chart_before_the_fitted_door() {
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
        matches!(
            off,
            Err(PcurveCertifyError::CarrierOffChart {
                chart: geom::SurfaceKind::Nurbs,
                ..
            })
        ),
        "off the wall the read ends it: {off:?}"
    );
}

// ---- review 6531 probes (not for merge) ----

/// R1: a row certified over `[0, 1]` re-certified for an edge spanning
/// `[0, 1.5]` (the carrier past `z = 1` is off the bounded patch).
fn r1_cut_short<T: Decide + geom_core::CertifiedBounds>() -> (String, String) {
    let (c, b) = (T::from_f64, band());
    let wall = ring::<T>();
    let carrier = ruling::<T>(Place::On);
    let stored = PcurveCache::certify(column(), c(0.0), c(1.0), &carrier, &wall, b).unwrap();
    let re = stored
        .recertify(&carrier, (c(0.0), c(1.5)), &wall, b, Some(FittedLane::certified()))
        .map(|_| ());
    let read = crate::read_incidence(&carrier, c(0.0), c(1.5), &wall, b);
    (format!("{re:?}"), format!("{read:?}"))
}

#[test]
fn review_r1_cut_short_recertify() {
    let (re, read) = r1_cut_short::<f64>();
    eprintln!("R1 f64 recertify over edge span [0,1.5]: {re}\n   read_incidence: {read}");
    let (re, read) = r1_cut_short::<Interval>();
    eprintln!("R1 Interval recertify: {re}\n   read_incidence: {read}");
}

/// R2: the ruling with a 1 mm radial bulge wholly inside `(0, 3/256)`,
/// between every residual sample (i/8) and every read sample (i/64).
fn bulged<T: Decide>() -> Curve3<T> {
    let ts = [0.0, 1.0 / 256.0, 1.0 / 128.0, 3.0 / 256.0, 1.0];
    let mut k = vec![0.0, 0.0];
    k.extend(&ts[1..4]);
    k.extend([1.0, 1.0]);
    let control = ts
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let r = if i == 2 { 1.0 + 1e-3 } else { 1.0 };
            Point3::new(r * RULING.cos(), r * RULING.sin(), t)
        })
        .collect();
    let net = geom::NurbsCurve3::new(KnotVector::clamped(k, 1).unwrap(), control, vec![1.0; 5])
        .unwrap();
    Curve3::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

fn r2<T: Decide>(lanes: Lanes<T>) -> Vec<String> {
    let (c, b) = (T::from_f64, band());
    let wall = ring::<T>();
    let carrier = bulged::<T>();
    let mid = carrier.eval(c(1.0 / 128.0));
    let mut out = vec![format!("bulge point {mid:?}")];
    out.push(format!(
        "certify(column): {:?}",
        PcurveCache::certify(column(), c(0.0), c(1.0), &carrier, &wall, b).map(|_| ())
    ));
    out.push(format!(
        "certify_general: {:?}",
        PcurveCache::certify_general(column_curve(), c(0.0), c(1.0), &carrier, &wall, b, Some(lanes.fitted)).map(|_| ())
    ));
    out.push(format!(
        "read_incidence: {:?}",
        crate::read_incidence(&carrier, c(0.0), c(1.0), &wall, b)
    ));
    out.push(format!("read_point_incidence(bulge): {:?}", crate::read_point_incidence(mid, &wall, b)));
    let mut table: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
    let key = table.insert(wall.clone());
    let ends = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(key, column()),
        carrier,
        param_start: c(0.0),
        param_end: c(1.0),
    };
    out.push(format!("stated route: {:?}", (lanes.edge)(spec, ends, &table)));
    out
}

#[test]
fn review_r2_between_samples() {
    for l in r2::<f64>(Lanes { fitted: FittedLane::certified(), edge: edge_f64 }) {
        eprintln!("R2 f64 {l}");
    }
    for l in r2::<Interval>(Lanes { fitted: FittedLane::certified(), edge: edge_interval }) {
        eprintln!("R2 Interval {l}");
    }
}

/// R3: a chart whose `u` domain is a few ulps wide: does the foot's
/// clamp (`inside`) panic?
#[test]
fn review_r3_narrow_domain_foot() {
    let lo = 1.0f64;
    let hi = lo + 4.0 * f64::EPSILON;
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
    );
    eprintln!("R3 surface built: {}", net.is_ok());
    let wall = Surface::Nurbs(Arc::new(net.unwrap()));
    let got = std::panic::catch_unwind(|| {
        crate::read_point_incidence(Point3::new(0.5, 0.5, 0.0), &wall, band())
    });
    eprintln!("R3 read_point_incidence: {got:?}");
    let got = std::panic::catch_unwind(|| {
        crate::read_point_incidence(Point3::new(0.5, 0.5, 1e-3), &wall, band())
    });
    eprintln!("R3 read_point_incidence off: {got:?}");
}
