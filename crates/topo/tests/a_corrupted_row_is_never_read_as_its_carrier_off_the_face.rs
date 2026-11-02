//! **A corrupted row is never read as its carrier off the face** (D3).
//! The at-rest pass reads the edge's carrier for incidence over the
//! EDGE's span (`PcurveCache::recertify`'s `edge_span`), so a row that
//! is wrong in itself is named by its own checks and never wears an
//! incidence ending:
//!
//! - a row honestly minted for another arc of the same locus fails
//!   check 1's identity (`ImageMismatch`);
//! - a row certified over half its edge's span fails its interval
//!   (`RowInterval`).
//!
//! Over the row's own span instead, the foreign arc's carrier is read
//! past its edge's end, off the cylinder, and the finding is
//! `CarrierOffChart`: the shape this row guards against.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use topo::construction::Construct;

use std::sync::Arc;

use geom::Curve3;
use geom_brep::{IncidenceRead, Pcurve, PcurveCache};
use geom_core::{Band, Tol};
use topo::pcurves::{PcurveMintError, validate_pcurves};

/// The findings that wear an incidence ending.
fn incidence_findings(findings: &[PcurveMintError]) -> Vec<&PcurveMintError> {
    findings
        .iter()
        .filter(|f| match f {
            PcurveMintError::Certify { error, .. } => IncidenceRead::of(error).is_some(),
            _ => false,
        })
        .collect()
}

#[test]
fn a_corrupted_row_is_never_read_as_its_carrier_off_the_face() {
    let band = Band::linear(Tol::witness()).unwrap();

    let built = fixture::build::<f64>();
    assert_eq!(
        validate_pcurves(&built.body, band),
        vec![],
        "the honest body validates"
    );

    let mut foreign = fixture::build::<f64>();
    let row = fixture::foreign_cache(&foreign);
    foreign.body.attach_pcurve(foreign.he_plus, row);
    let findings = validate_pcurves(&foreign.body, band);
    assert!(
        !findings.is_empty() && incidence_findings(&findings).is_empty(),
        "a foreign arc's row is named by its own checks, never as off the face: {findings:?}"
    );

    let mut cut = fixture::build::<f64>();
    let (t0, t1) = cut.carrier.domain();
    let Pcurve::Projected(image) = cut.body.pcurve(cut.he_plus).unwrap().pcurve().clone() else {
        panic!("the minted row is projected")
    };
    let half = PcurveCache::certify_projected(
        *image,
        t0,
        t0 + (t1 - t0) * 0.5,
        &Curve3::Nurbs(Arc::clone(&cut.carrier)),
        &cut.cylinder,
        band,
        Some(geom_brep::FittedLane::certified()),
    )
    .expect("the half-interval certifies honestly");
    cut.body.attach_pcurve(cut.he_plus, half);
    let findings = validate_pcurves(&cut.body, band);
    assert!(
        !findings.is_empty() && incidence_findings(&findings).is_empty(),
        "a row cut short is named by its interval, never as off the face: {findings:?}"
    );
}
