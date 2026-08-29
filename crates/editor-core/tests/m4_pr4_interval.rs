//! M4 PR 4 spec D7: f64/Interval agreement extends to DIAGNOSIS
//! output — the diagnosis corpus resolves to byte-identical
//! `Resolution` values at both scalar types (tables agree per PR 3's
//! invariant; verdict logs are scalar-independent; the diagnosis is a
//! function of both plus the document pair).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use fixture::pr4::diagnosis_corpus;
use geom_core::Interval;

#[test]
fn diagnosis_output_agrees_between_f64_and_interval() {
    let f = diagnosis_corpus::<f64>();
    let i = diagnosis_corpus::<Interval>();
    assert_eq!(f.len(), i.len());
    for ((lf, rf), (li, ri)) in f.iter().zip(i.iter()) {
        assert_eq!(lf, li);
        assert_eq!(
            format!("{rf:?}"),
            format!("{ri:?}"),
            "diagnosis output diverged between scalars at '{lf}'"
        );
    }
}
