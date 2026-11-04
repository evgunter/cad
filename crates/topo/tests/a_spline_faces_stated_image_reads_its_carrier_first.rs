//! **A stated chart image on a spline face is read against its carrier's
//! incidence first** (D3): at a build, at rest and at the import door,
//! a carrier off its wall ends `CarrierOffChart` by that door's term (the
//! kernel's defect, the stored culprit, the damaged file), one in band
//! of it ends in the read's lever, and one on it certifies as it did,
//! never ending as the image's own bound.
//!
//! The fixture is the triangle lamina on the bilinear identity chart
//! (`an_off_iso_straight_edge_takes_its_general_row::lamina`), every edge
//! restated through `set_edge_curve` as this face's own chart image (the
//! row and column as iso lines, the diagonal as a curve in UV): the edges
//! `nurbs_iso_derive`'s own-chart arm hands on unread, so every at-rest
//! derivation here is that arm's. The chart is then moved off the lamina
//! along its normal, its key kept, so each carrier is off its wall by the
//! lift:
//!
//! - **build**: the row's stated image restated on the moved chart, the
//!   stated route's read (`canonical_of`), whose ending is also read at
//!   the import door's term;
//! - **at rest, stored**: the rows minted on the true chart, re-certified
//!   on the moved one (`recertify`);
//! - **at rest, unstored**: the face's derivation, whose own-chart arm
//!   returns each stated image unread and the mint's doors certify it
//!   (`certify`, `certify_general`), each reading the carrier first.
//!
//! Read at `f64` and `Interval`, and the unstored pass at a dual, where
//! the read refuses before any fitted door is asked for.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{CertifyError, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, PcurveCertifyError};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Bounds, Point2, Point3, Real, Tol, Vec2};
use std::sync::Arc;
use topo::construction::Construct;
use topo::{AtRestPolicy, Body, EulerOpError, FaceKey, HalfEdgeKey, PcurveMintError};

use crate::an_off_iso_straight_edge_takes_its_general_row::lamina;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Where the chart is moved to: on the lamina, a millimetre off, in band.
#[derive(Clone, Copy, Debug)]
enum Place {
    On,
    Off,
    InBand,
}

impl Place {
    fn lift(self) -> f64 {
        let b = band();
        match self {
            Self::On => 0.0,
            Self::Off => 1e-3,
            Self::InBand => 0.5 * (b.zero() + b.escalate()),
        }
    }
}

/// The bilinear identity chart lifted `lift` along `z`.
fn chart<T: AtRestPolicy>(lift: f64) -> Surface<T> {
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
        Knots::from(unit()).lift(Real::from_f64),
        Knots::from(unit()).lift(Real::from_f64),
        [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(lift)))
            .collect(),
        vec![Real::one(); 4],
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net))
}

/// `half`'s edge restated as `face`'s own chart image on the identity
/// chart: its carrier's `(x, y)`, an iso line where the carrier runs
/// along `u` or `v` (the row and column), and a degree-one curve in UV
/// otherwise (the diagonal).
fn stated<T: AtRestPolicy + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    half: HalfEdgeKey,
) -> (topo::EdgeKey, EdgeCurveSpec<T>) {
    let edge = body.get_half_edge(half).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .clone();
    let (t0, t1) = curve.params();
    let carrier = curve.carrier();
    let (o, d) = (
        carrier.eval(T::zero()),
        carrier.eval(T::one()) - carrier.eval(T::zero()),
    );
    let zero = |x: T| x.lo() == 0.0 && x.hi() == 0.0;
    let image = if zero(d.x) || zero(d.y) {
        Pcurve::IsoLine {
            p0: Point2::new(o.x, o.y),
            pl: Vec2::new(d.x, d.y),
        }
    } else {
        let (a, b) = (carrier.eval(t0), carrier.eval(t1));
        let knots = KnotVector::clamped(vec![t0.lo(), t0.lo(), t1.hi(), t1.hi()], 1).unwrap();
        Pcurve::General(Arc::new(
            geom::NurbsCurve2::new(
                Knots::from(knots).lift(Real::from_f64),
                vec![Point2::new(a.x, a.y), Point2::new(b.x, b.y)],
                vec![T::one(); 2],
            )
            .unwrap(),
        ))
    };
    let key = body.get_face(face).unwrap().surface;
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(key, image),
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    };
    (edge, spec)
}

/// The lamina with every edge restated as its own face's chart image on
/// the true chart, and the row's half-edge.
fn restated<T: AtRestPolicy + Bounds>() -> (Body<T>, [FaceKey; 2], HalfEdgeKey) {
    let (mut body, faces, halves) = lamina::<T>(0.0);
    let face = body.face_of_half_edge(halves[0]).unwrap();
    for half in halves {
        let (edge, spec) = stated(&body, face, half);
        if let Err(e) = body.set_edge_curve(edge, spec, Tol::witness()) {
            panic!(
                "{half:?} restates as its own chart image on the true chart, at {}: {e:?}",
                T::NAME
            );
        }
    }
    (body, faces, halves[0])
}

/// `body` with its chart moved to `place`.
fn moved<T: AtRestPolicy + Bounds>(body: &Body<T>, row: HalfEdgeKey, place: Place) -> Body<T> {
    let face = body.face_of_half_edge(row).unwrap();
    let key = body.get_face(face).unwrap().surface;
    body.with_surface_moved_for_tests(key, chart(place.lift()))
        .unwrap()
}

/// The build door: the row's stated image restated on the moved chart.
fn at_build<T: AtRestPolicy + Bounds>(place: Place) -> Result<(), CertifyError> {
    let (body, _, row) = restated::<T>();
    let mut body = moved(&body, row, place);
    let face = body.face_of_half_edge(row).unwrap();
    let (edge, spec) = stated(&body, face, row);
    match body.set_edge_curve(edge, spec, Tol::witness()) {
        Ok(_) => Ok(()),
        Err(EulerOpError::Certification { error }) => Err(error),
        Err(other) => {
            panic!("{place:?}: the stated route refuses through certification: {other:?}")
        }
    }
}

/// The at-rest pass's certification findings, the rows stored or not.
fn at_rest<T: AtRestPolicy + Bounds>(place: Place, stored: bool) -> Vec<PcurveCertifyError> {
    let (mut body, faces, row) = restated::<T>();
    if stored {
        let minted = topo::mint_pcurves_of(&mut body, &faces, Tol::witness());
        assert!(
            matches!(minted, Ok(n) if n > 0),
            "the rows mint on the true chart, at {}: {minted:?}",
            T::NAME
        );
    }
    let body = moved(&body, row, place);
    topo::pcurves::validate_pcurves(&body, band())
        .into_iter()
        .filter_map(|finding| match finding {
            PcurveMintError::Certify { error, .. } => Some(error),
            _ => None,
        })
        .collect()
}

fn is_off(e: &PcurveCertifyError) -> bool {
    matches!(
        e,
        PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Nurbs,
            carrier: geom::CurveKind::Line,
            ..
        }
    )
}

fn is_in_band(e: &PcurveCertifyError) -> bool {
    matches!(
        e,
        PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::ChartIncidence,
            ..
        }
    )
}

/// Every door's reading at every place, the misreads gathered so each
/// door that misreads is named.
fn every_door_reads_the_wall<T: AtRestPolicy + Bounds>() {
    let mut misread: Vec<String> = Vec::new();
    let mut check = |door: &str, ok: bool, got: &dyn core::fmt::Debug| {
        if !ok {
            misread.push(format!("{door}: {got:?}"));
        }
    };
    // Build: the stated route.
    let on = at_build::<T>(Place::On);
    check("build, on the wall", on.is_ok(), &on);
    let off = at_build::<T>(Place::Off);
    let read_off = matches!(
        off,
        Err(CertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Nurbs,
            carrier: geom::CurveKind::Line,
            ..
        })
    );
    check("build, off the wall", read_off, &off);
    if let Err(off) = &off {
        let at_build = off.ending(Reading::Build);
        check(
            "build, off the wall: the kernel's defect",
            at_build.as_deref() == Some(geom_core::KERNEL_DEFECT_ENDING),
            &at_build,
        );
        let at_import = off.ending(geom_core::FileCoincidence::new(1e-6));
        check(
            "import, off the wall: the damaged file",
            read_off && at_import.as_deref() == Some(geom_core::DAMAGED_FILE_ENDING),
            &at_import,
        );
    }
    let near = at_build::<T>(Place::InBand);
    let read_near = matches!(
        near,
        Err(CertifyError::Escalated {
            check: geom_brep::CertCheck::ChartIncidence,
            ..
        })
    );
    check("build, in band of the wall", read_near, &near);
    if let Err(near) = &near {
        for at in [
            geom_brep::recourse::ReadAt::from(Reading::Build),
            geom_core::FileCoincidence::new(1e-6).into(),
        ] {
            let ending = near.ending(at);
            check(
                "build and import, in band: the read's lever",
                ending.as_deref().is_some_and(|e| {
                    e != geom_core::KERNEL_DEFECT_ENDING && e != geom_core::DAMAGED_FILE_ENDING
                }),
                &ending,
            );
        }
    }
    // At rest, the rows stored (`recertify`) and not (the own-chart arm).
    for stored in [true, false] {
        let on = at_rest::<T>(Place::On, stored);
        check(
            &format!("at rest (stored: {stored}), on the wall"),
            on.is_empty(),
            &on,
        );
        let off = at_rest::<T>(Place::Off, stored);
        let read_off = !off.is_empty() && off.iter().all(is_off);
        check(
            &format!("at rest (stored: {stored}), off the wall"),
            read_off,
            &off,
        );
        if let Some(first) = off.first() {
            let rendered = topo::ValidationError::Pcurve {
                finding: PcurveMintError::Certify {
                    half_edge: HalfEdgeKey::default(),
                    error: first.clone(),
                },
            }
            .render(Reading::AtRest);
            check(
                &format!("at rest (stored: {stored}), off the wall: the stored culprit"),
                rendered.contains(topo::test_support::incidence_reasons::OFF)
                    && rendered.ends_with(geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
                &rendered,
            );
        }
        let near = at_rest::<T>(Place::InBand, stored);
        check(
            &format!("at rest (stored: {stored}), in band of the wall"),
            !near.is_empty() && near.iter().all(is_in_band),
            &near,
        );
    }
    assert!(
        misread.is_empty(),
        "on the wall each door certifies, off it each ends off the chart by its term, and in \
         band of it each ends in the read's lever, at {}:\n{}",
        T::NAME,
        misread.join("\n")
    );
}

#[test]
fn every_door_reads_the_wall_at_f64() {
    every_door_reads_the_wall::<f64>();
}

#[test]
fn every_door_reads_the_wall_at_interval() {
    every_door_reads_the_wall::<geom_core::Interval>();
}

/// **At a dual the read refuses first.** The unstored face's derivation
/// certifies each stated image, the diagonal's curve in UV included,
/// through a door whose incidence read needs no fitted door: off the
/// wall it is a finding, never the excused `FittedLaneUnsupported`.
#[test]
fn at_a_dual_an_off_carrier_is_a_finding() {
    type T = geom_core::Dual64;
    let (body, _, row) = restated::<T>();
    let body = moved(&body, row, Place::Off);
    let findings = topo::pcurves::validate_pcurves(&body, band());
    assert!(
        findings.iter().any(|f| matches!(
            f,
            PcurveMintError::Certify { error, .. } if is_off(error)
        )),
        "the dual's at-rest pass reads the row off the wall: {findings:?}"
    );
}
