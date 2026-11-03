//! **A result that does not finish is keyed by its findings** (PR 6371
//! review r2): `topo::test_support::offer_key` gives a Boolean's
//! `ResultInvalid` the variants its findings hold, so an in-band reading
//! (a volume uncomputable in band) and a definite one (a negative
//! volume) are two decisions, as the editor's offers read them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::test_support::{offer_key, validation_error_samples};
use topo::{BooleanError, NotFinished, ValidationError};

fn invalid(findings: Vec<ValidationError>) -> BooleanError {
    BooleanError::ResultInvalid {
        findings: NotFinished::for_tests(findings),
    }
}

#[test]
fn an_in_band_reading_and_a_negative_volume_key_apart() {
    let samples = validation_error_samples();
    let pick = |label: &str| {
        samples
            .iter()
            .find(|(l, _)| l.starts_with(label))
            .unwrap_or_else(|| panic!("a {label} sample"))
            .1
            .clone()
    };
    let negative = pick("NegativeVolume");
    let in_band = pick("VolumeUncomputable");
    let (k_negative, _) = offer_key(&invalid(vec![negative.clone()]));
    let (k_in_band, _) = offer_key(&invalid(vec![in_band.clone()]));
    assert_eq!(k_negative, "ResultInvalid(NegativeVolume)");
    assert_eq!(k_in_band, "ResultInvalid(VolumeUncomputable)");
    let (both, _) = offer_key(&invalid(vec![negative.clone(), in_band, negative]));
    assert_eq!(
        both, "ResultInvalid(NegativeVolume,VolumeUncomputable)",
        "each variant once, in order"
    );
}
