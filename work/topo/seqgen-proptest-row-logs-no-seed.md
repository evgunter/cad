---
id: seqgen-proptest-row-logs-no-seed
kind: issue
title: seqgen's proptest fuzz row draws from entropy and logs no seed, so a green run records nothing
status: open
opened: 2026-09-06
track: P
refs: [S69, 2014]
---

## What

`crates/topo/src/seqgen/random_op_sequences.rs`'s
`random_op_sequences_hold_all_properties` is the crate's op-sequence
counterexample search. Its decision vectors come from proptest, which
seeds its RNG from entropy — correct for the shape (varying seed,
monotone in the safe direction) — but proptest prints **nothing** on a
green run and records a failing case only by appending to
`crates/topo/proptest-regressions/seqgen/random_op_sequences.txt`
— proptest names that file after the row's SOURCE FILE, and the row
moved into `seqgen/random_op_sequences.rs` before this was written.
(The tracked `proptest-regressions/seqgen.txt` beside it is the path
from before that move and is no longer read at all;
`work/topo/orphaned-proptest-corpus-for-seqgen.md` carries that.)

`memories/test-suite-cost.md` asks every fuzzer for three properties
together, and the first is *"a varying seed, logged
UNCONDITIONALLY — always, not only on failure ... or a red run is
unreproducible"*. The tree's `test_utils::fuzz` rows satisfy it
through `fuzz::start`, which prints
`seed=0x… effort=… — replay with CAD_FUZZ_SEED=0x…` every run. This
row cannot: proptest owns its stream and does not expose the seed it
drew.

So a green run of this row says only that 48 (now
`CAD_FUZZ_EFFORT`-scaled) unknown vectors passed. Two runs of it
cannot be compared, and a reader cannot tell a deep sample from a
shallow one.

## What it is NOT

Not a claim that the row is weak: the per-step assertions are the bar
and they run on every draw. And not the `proptest-regressions` file's
fault — that file does its job for a case that has already failed.
This is about what a green run records.

## What closing it looks like

Either drive the row's decision vectors from `test_utils::fuzz`'s Rng
instead of proptest's strategies — which costs proptest's shrinking,
and shrinking is what produced the `issue_60` distillation next door,
so that is a real trade to weigh — or find a proptest surface that
reports the seed it used (`ProptestConfig::rng_algorithm` /
`TestRunner::new_with_rng` let a caller SUPPLY a seeded RNG, which
would let the row draw its own seed through `fuzz::start` and keep
shrinking). The second is the one to try first.

S69 (PR 2014) put the row's case count on `CAD_FUZZ_EFFORT`, which is
the second of the three properties; this row is the first.
