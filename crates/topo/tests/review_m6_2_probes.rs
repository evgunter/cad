//! Adversarial-review probes for M6-2 (PR #176), kept for regression
//! value, re-pointed at the row the fixture's cylinder face now stores
//! (its projected image). Three corruption species against the at-rest
//! pass, each pinning WHY the pass answers what it answers:
//!
//! 1. a row honestly minted for a DIFFERENT arc of the same locus is
//!    rejected at check 1: its stored net is not the edge's carrier's
//!    (another knot vector), the identity check by structure;
//! 2. a corruption BETWEEN the schedule samples is refused at the
//!    envelope, at the interval scalar where no schedule runs ahead of
//!    it: the stored net's distance from the carrier's is the fidelity
//!    term, so the between-samples statement is a bound on the image,
//!    not only on the carrier (the gap an on-locus hull bound left);
//! 3. a row honestly certified over a SUB-interval of its edge's span is
//!    caught by its interval (`RowInterval`), and by nothing else.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use geom::Curve3;
use geom_brep::{EnvelopeTerm, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck};
use geom_core::{Band, Tol};
use topo::pcurves::{PcurveMintError, validate_pcurves};

/// Species 1: the foreign-arc row fails the at-rest pass at check 1 —
/// the re-certification compares the stored net against the EDGE's
/// carrier (read from the body, per the intensional description).
#[test]
fn the_foreign_arc_row_fails_on_the_carriers_structure() {
    let mut built = fixture::build::<f64>();
    let band = Band::linear(Tol::witness()).unwrap();
    let foreign = fixture::foreign_cache(&built);
    built.body.attach_pcurve(built.he_plus, foreign);
    let findings = validate_pcurves(&built.body, band);
    assert!(
        findings.iter().any(|f| matches!(
            f,
            PcurveMintError::Certify {
                half_edge,
                error: PcurveCertifyError::ImageMismatch {
                    image: geom_brep::PcurveKind::Projected,
                    ..
                },
            } if *half_edge == built.he_plus
        )),
        "the identity mismatch must surface on the corrupted half-edge: {findings:?}"
    );
}

/// Species 2: a between-samples corruption is refused at the envelope.
/// The carrier's net, one interior control moved 1e-3 m along the
/// cylinder's axis (so the moved carrier still lies on the cylinder),
/// imaged and offered as the row of the true carrier: at the interval
/// scalar the envelope is the whole certified statement, and its
/// fidelity term reads the moved control.
#[test]
fn a_between_samples_net_corruption_is_refused_at_the_envelope() {
    use geom_core::interval::Interval;
    let built = fixture::build::<Interval>();
    let band = Band::linear(Tol::witness()).unwrap();
    let mut control = built.carrier.control().to_vec();
    let mid = control.len() / 2;
    control[mid].z = control[mid].z + Interval::from_bounds(1e-3, 1e-3);
    let moved = geom::NurbsCurve3::new(
        built.carrier.knots().clone(),
        control,
        built.carrier.weights().to_vec(),
    )
    .unwrap();
    // The moved carrier is not on the sphere, so it is imaged directly on
    // the cylinder chart, where it still lies.
    let image = geom_brep::chart_pcurve(&Curve3::Nurbs(Arc::new(moved)), &built.cylinder, band)
        .expect("the moved carrier still lies on the cylinder");
    let Pcurve::Projected(image) = image else {
        panic!("a spline's cylinder image is projected")
    };
    let (d0, d1) = built.carrier.domain();
    let err = PcurveCache::certify_projected(
        *image,
        Interval::from_bounds(d0, d0),
        Interval::from_bounds(d1, d1),
        &Curve3::Nurbs(Arc::clone(&built.carrier)),
        &built.cylinder,
        band,
        Some(geom_brep::FittedLane::certified()),
    )
    .expect_err("a net off the carrier's does not certify");
    assert!(
        matches!(
            err,
            PcurveCertifyError::ResidualExceeded {
                check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::Fidelity),
                ..
            }
        ),
        "the refusal is the fidelity term's: {err:?}"
    );
}

/// Species 3: a sub-interval row. `recertify` re-derives over the row's
/// own stored `(t0, t1)`, so the certificate cannot see it; the at-rest
/// pass REJECTS it as a row that does not state its edge's interval
/// (`RowInterval`), and as nothing else.
#[test]
fn a_sub_interval_row_is_caught_by_its_interval() {
    let mut built = fixture::build::<f64>();
    let band = Band::linear(Tol::witness()).unwrap();
    let (t0, t1) = built.carrier.domain();
    let tm = t0 + (t1 - t0) * 0.5;
    let Pcurve::Projected(image) = built.body.pcurve(built.he_plus).unwrap().pcurve().clone()
    else {
        panic!("the minted row is projected")
    };
    let cache = PcurveCache::certify_projected(
        *image,
        t0,
        tm,
        &Curve3::Nurbs(Arc::clone(&built.carrier)),
        &built.cylinder,
        band,
        Some(geom_brep::FittedLane::certified()),
    )
    .expect("the half-interval certifies honestly");
    built.body.attach_pcurve(built.he_plus, cache);
    let findings = validate_pcurves(&built.body, band);
    assert_eq!(
        findings,
        vec![PcurveMintError::RowInterval {
            half_edge: built.he_plus
        }],
        "the row's interval is what catches a sub-interval row: {findings:?}"
    );
}
