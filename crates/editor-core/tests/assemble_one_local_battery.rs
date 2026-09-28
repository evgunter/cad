//! **The assembly gate pays tier 3's local battery once per aggregate.**
//!
//! The gather gates its aggregate with tier 3 and keeps the verdict on
//! the product's body; the assembly gate over that product then runs
//! tier 3′'s census alone. The witness is the `probe` sample log, which
//! records every predicate decision the at-rest passes make: over the
//! assembled body, the assembly gate's samples plus a standalone tier-3
//! pass's samples must be, as a multiset, exactly a standalone tier-3′
//! pass's. A gate that re-ran the battery records the battery's samples
//! a second time and breaks the equality; one that skipped the census
//! loses the census's.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::two_blocks_and_their_union;

use editor_core::{CancelToken, EvalOptions, assemble_gathered, evaluate, product_recorded};
use geom_core::Tol;
use geom_core::k_stats::{self, MarginSample, Probe};

/// One sample by its bits, sortable: the two passes' orders differ
/// (tier 3 makes check 7 after checks 8 and 9, tier 3′ before), so the
/// comparison is of multisets.
fn bits(s: &MarginSample) -> (&'static str, u64, u64, u64, String) {
    (
        s.predicate,
        s.margin.to_bits(),
        s.band_zero.to_bits(),
        s.band_escalate.to_bits(),
        format!("{:?}", s.outcome),
    )
}

fn recorded<R>(f: impl FnOnce() -> R) -> (R, Vec<(&'static str, u64, u64, u64, String)>) {
    k_stats::start_recording();
    let out = f();
    let mut samples: Vec<_> = k_stats::take_samples().iter().map(bits).collect();
    samples.sort();
    (out, samples)
}

#[test]
fn the_assembly_gate_runs_the_census_over_the_gathers_verdict() {
    let tol = Tol::witness();
    let (doc, _) = two_blocks_and_their_union("assemble-one-local-battery");
    let ev = evaluate::<Probe>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let product = product_recorded(&doc, &ev, tol).expect("the union gathers");
    let (assembly, gate) = recorded(|| assemble_gathered(product, tol));
    let assembly = assembly.expect("an unmated product assembles");
    let (local, battery) = recorded(|| topo::validate_geometric(&assembly.body, tol));
    local.expect("the assembled body passes tier 3");
    let (whole, pass) =
        recorded(|| topo::validate_pseudomanifold(&assembly.body, &assembly.contacts, tol));
    whole.expect("the assembled body passes tier 3′");
    assert!(
        !battery.is_empty(),
        "tier 3 recorded no sample: the equality below would not see a second battery"
    );
    let mut paid: Vec<_> = gate.iter().chain(&battery).cloned().collect();
    paid.sort();
    assert_eq!(
        paid.len(),
        pass.len(),
        "the assembly gate recorded {} samples, tier 3 {} and tier 3′ {}: the gate must be \
         tier 3′ less one battery",
        gate.len(),
        battery.len(),
        pass.len()
    );
    assert!(
        paid == pass,
        "the assembly gate's samples plus one battery are not tier 3′'s, sample for sample"
    );
}
