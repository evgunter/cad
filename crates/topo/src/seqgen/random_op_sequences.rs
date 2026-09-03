//! The op-sequence counterexample SEARCH: properties (a)–(d) of
//! `seqgen`'s module docs over random valid op sequences.
//!
//! It lives in its own file because a marker gates a whole file's
//! module, and `seqgen.rs`'s `tests` module next door holds the
//! deterministic pins — the issue-60 distillation, the genus-one
//! teardown, the generator's determinism and the pinned selection —
//! which must keep running on every leg. The helpers stay where their
//! other callers are and are borrowed from here.

// Gated to the code it tests (TCOST-1), as `memories/test-suite-cost.md`
// requires of every fuzzer: proptest draws this row's decision vectors
// from entropy, so it is a counterexample search and pays for itself only
// on a diff that could move what it searches. The claim is the Euler
// operators' own — tier-1 validity after every op, the E–P ledger at every
// step, a make/kill roundtrip that restores the body, and a teardown that
// empties the arenas AND the provenance maps — so it rests on the generator
// here, on every euler module the walk calls, on the body and entity arenas
// it validates, and on the validator, the ledger's iso oracle and the
// fixture the walk starts from. Named wide on purpose: a run costs about a
// second and a half, and a break this misses waits for the nightly.
test_utils::gated_to![
    "crates/topo/src/seqgen.rs",
    "crates/topo/src/euler.rs",
    "crates/topo/src/euler_ring.rs",
    "crates/topo/src/euler_kill.rs",
    "crates/topo/src/body.rs",
    "crates/topo/src/entity.rs",
    "crates/topo/src/validate.rs",
    "crates/topo/src/iso.rs",
    "crates/topo/src/fixtures.rs",
];

use proptest::prelude::*;

use super::tests::{RoundtripTally, run_properties};

/// Properties (a)–(d) over random valid op sequences (module
/// docs): tier-1 validity after every op, the E–P ledger at
/// every step, make/kill roundtrips at random points, and full
/// teardown to empty arenas + empty provenance maps.
///
/// How much of property (c) ran is checked, and the check is
/// per-step: the two documented irreversible-by-one-op subcases
/// live in [`roundtrip`]'s `Kev`/`Kef` arms only, so a selection on
/// any other choice must execute, and `run_properties` asserts
/// exactly that as each step happens. **That per-step assertion is
/// the whole of the bar.**
///
/// The totals below are two different things, and the difference
/// matters more than either:
///
/// - `executed > 0` is the only one that can fail on its own, and
///   it is a COLLAPSE FLOOR, not a bar. It is not implied by the
///   design — a run whose every selection landed on an irreversible
///   site would legally execute none — but such a run tested
///   nothing and should say so rather than pass green.
/// - `executed + skipped == selected` and `skipped <= skippable`
///   are BOOKKEEPING IDENTITIES. The first follows from how the
///   tally is accumulated, the second from the per-step assertion
///   that has already run. Neither can independently go red; they
///   are here to state the shape of the tally for a reader, and
///   nothing more.
///
/// No numeric threshold is asserted, because there is no number to
/// assert: proptest seeds its RNG from entropy, so every run draws a
/// fresh sample. Four consecutive runs gave 339/335/4/47,
/// 331/325/6/43, 333/328/5/51 and 351/345/6/47 for
/// selected/executed/skipped/skippable — a threshold would have
/// pinned that spread, not a property.
#[test]
fn random_op_sequences_hold_all_properties() {
    let tally = std::cell::Cell::new(RoundtripTally::default());
    proptest!(
        ProptestConfig {
            cases: 48,
            ..ProptestConfig::default()
        },
        |(decisions in proptest::collection::vec(
            (any::<u32>(), any::<u32>(), any::<u32>()),
            1..48,
        ))| {
            let mut total = tally.get();
            total.add(run_properties(&decisions)?);
            tally.set(total);
        }
    );
    let t = tally.get();
    // The only run-level check that can fail on its own.
    assert!(
        t.executed > 0,
        "no make/kill roundtrip executed across the whole run: \
         property (c) went untested",
    );
    // Bookkeeping identities (see the doc): the first is how the
    // tally is accumulated, the second is what the per-step
    // assertion has already guaranteed. Stated, not relied on.
    assert_eq!(
        t.executed + t.skipped,
        t.selected,
        "every selected step either roundtripped or skipped",
    );
    assert!(
        t.skipped <= t.skippable,
        "{} skips against {} selections that could legally skip",
        t.skipped,
        t.skippable,
    );
}
