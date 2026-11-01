//! **The at-rest gates pay tier 3′ once per aggregate, all of it at the
//! gather.**
//!
//! The gather gates its aggregate with tier 3, runs tier 3′'s census
//! over it and keeps both verdicts on the product; the at-rest check
//! reads the kept census and runs no predicate. The witness is the
//! `probe` sample log, which records every predicate decision the
//! at-rest passes make: the gather's samples must be, as a multiset,
//! exactly a standalone tier-3′ pass's over the assembled body, and the
//! at-rest check's must be none. A check that re-ran the battery or the
//! census records its samples a second time; a gather that skipped the
//! census loses the census's.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{place, two_blocks_and_their_union};

use editor_core::{CancelToken, EvalOptions, evaluate, product_recorded};
use geom_core::Tol;
use geom_core::k_stats::{self, MarginSample, Probe};

/// One sample by its bits, sortable: the two passes' orders differ
/// (tier 3 makes check 7 after checks 8 and 9, tier 3′ before), so the
/// comparison is of multisets.
type Bits = (&'static str, u64, u64, u64, String);

fn bits(s: &MarginSample) -> Bits {
    (
        s.predicate,
        s.margin.to_bits(),
        s.band_zero.to_bits(),
        s.band_escalate.to_bits(),
        format!("{:?}", s.outcome),
    )
}

fn recorded<R>(f: impl FnOnce() -> R) -> (R, Vec<Bits>) {
    k_stats::start_recording();
    let out = f();
    let mut samples: Vec<_> = k_stats::take_samples().iter().map(bits).collect();
    samples.sort();
    (out, samples)
}

#[test]
fn the_gather_pays_tier_3_prime_once_and_the_at_rest_check_nothing() {
    let tol = Tol::witness();
    let (doc, union) = two_blocks_and_their_union("assemble-one-local-battery");
    let (doc, _) = place(doc, union);
    let ev = evaluate::<Probe>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let (product, gather) = recorded(|| product_recorded(&doc, &ev, tol));
    let product = product.expect("the union gathers");
    let at_rest_only = editor_core::ChecksConfig {
        connectedness: editor_core::Severity::Off,
        unproven_coincidence: editor_core::Advisory::Off,
        ..editor_core::ChecksConfig::default()
    };
    let (report, gate) = recorded(|| {
        editor_core::run_checks_on(
            &doc,
            &ev,
            editor_core::Subject::Product(&product),
            &at_rest_only,
            tol,
        )
    });
    report.expect("the at-rest check runs over the product");
    let (whole, pass) =
        recorded(|| topo::validate_pseudomanifold(&product.body, &product.contacts, tol));
    whole.expect("the assembled body passes tier 3′");
    assert!(
        !pass.is_empty(),
        "tier 3′ recorded no sample: the equality below would see nothing"
    );
    assert!(
        gate.is_empty(),
        "the at-rest check recorded {} samples: it reads the gather's census and runs none",
        gate.len()
    );
    assert_eq!(
        gather.len(),
        pass.len(),
        "the gather recorded {} samples and tier 3′ {}: the gather must pay tier 3′ once",
        gather.len(),
        pass.len()
    );
    assert!(
        gather == pass,
        "the gather's samples are not tier 3′'s, sample for sample"
    );
}
