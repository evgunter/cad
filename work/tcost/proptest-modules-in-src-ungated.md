---
id: proptest-modules-in-src-ungated
kind: issue
title: the 14 in-src proptest modules stay ungated: a split per file buys 0.62 cpu-s
status: open
opened: 2026-09-03
---


TCOST-9 gated the `proptest!` population. Twenty-three files use it on
`cfc826b9`; nine are whole suite files or split out into one, and are
gated in PR TCOST-9. **Fourteen are `#[cfg(test)]` modules inside
PRODUCTION source files, and every one of them mixes deterministic pins
with the property rows**, so a marker — which gates a whole file's module —
would take the pins with it. That is the trade TCOST-1's batch-2 review
rejected for `mesh/src/nurbs_cert.rs` and `step-import/src/chart.rs`, and
the fix there was to move the property rows into their own `#[cfg(test)]`
FILE. Fourteen such splits were not made here, on the measurement below.

**What the split would buy, per file** (2026-09-02 hosted census,
`~/tcost-work/timing-history/per_test.csv`, median cpu-s over 21–22 runs
per lane). "module" is the whole `#[cfg(test)]` module a whole-file marker
would gate; "rows" is the `proptest!` rows inside it, which is all a split
would take out of a PR gate:

| file | `#[test]`s: pins / proptest | module cpu-s (default) | proptest rows cpu-s (default) |
|---|---|--:|--:|
| `crates/geom-core/src/dual.rs` | 30 / 13 | 0.180 | 0.078 |
| `crates/geom-core/src/real.rs` | 16 / 14 | 0.176 | 0.084 |
| `crates/geom-core/src/linalg/vec.rs` | 9 / 15 | 0.124 | 0.090 |
| `crates/topo/src/validate.rs` | 50 / 4 | 0.424 | 0.121 |
| `crates/geom/src/curves.rs` | 18 / 10 | 0.120 | 0.060 |
| `crates/geom/src/surfaces.rs` | 18 / 5 | 0.104 | 0.031 |
| `crates/geom-core/src/predicate.rs` | 12 / 4 | 0.094 | 0.024 |
| `crates/geom-core/src/linalg/mat.rs` | 10 / 6 | 0.093 | 0.037 |
| `crates/quantity/src/tests.rs` | 13 / 1 | 0.083 | 0.006 |
| `crates/geom-core/src/linalg/affine.rs` | 7 / 4 | 0.054 | 0.027 |
| `crates/geom-core/src/linalg/point.rs` | 4 / 5 | 0.052 | 0.030 |
| `crates/geom-brep/src/implicit.rs` | 7 / 3 | 0.042 | 0.018 |
| `crates/step-export/src/real.rs` | 2 / 2 | 0.024 | 0.012 |
| `crates/geom-core/src/interval.rs` | 23 / 6 | 0 (0.165 interval) | 0 (0.036 interval) |
| **total** | **219 / 92** | **1.57** | **0.62** |

Fourteen splits of production source — each one moving rows, widening a
`tests` module's helpers to `pub(super)`, and adding a mount — to take
**0.62 cpu-s** off a pull-request gate whose two shards cost ~165 and ~388
cpu-s per lane. That is 0.19 % of the default leg, and the split's own
cost is paid in kernel-source churn and in test-only surface that did not
exist before. `memories/test-suite-cost.md`: *"cost concentrates savagely
— a handful of tests hold most of the test time and the long tail is
free, so profile before cutting."* This is the long tail, measured.

**Why it is filed rather than closed.** `memories/test-suite-cost.md`
binds ALL fuzzing — *"a fuzzer that is not gated is a defect in the
fuzzer"* — and these rows are counterexample searches by its own taxonomy,
so the rule reaches them even though its rationale (per-run cost) does
not bite at 6 ms a row. The gap between the rule and its reason is a
question for Ev, not a judgement a lane should make silently. Two ways
to close it:

1. Rule that the marker's granularity is the thing to fix, not the
   files: a marker that names a MODULE inside a file (a second argument,
   or a marker sited on the `mod` item) would gate these rows with no
   split at all, and would also serve the next fourteen.
2. Rule that the cost threshold is real, and write it down — a fuzzer
   under some named cpu-s is exempt — so the next lane does not re-derive
   this table.

The one row in the src population where the trade was clearly worth it
was split in TCOST-9 rather than left here:
`crates/topo/src/seqgen.rs`'s `random_op_sequences_hold_all_properties`,
at **1.64 cpu-s** on its own — more than all fourteen files above
together — now `crates/topo/src/seqgen/random_op_sequences.rs`, gated.

Census pattern: `grep -rln "proptest!" crates --include=*.rs` (23 files,
identical to `grep -rln "proptest::"`). It cannot match a property sweep
written without the macro — a hand-rolled loop over a `Strategy`, or a
row that takes its cases from a helper crate — and TCOST-1's wider RNG
sweep (`rand::|StdRng|SmallRng|thread_rng|quickcheck|xorshift|splitmix|wyrand|pcg`)
found none of those beyond the two xorshift suites it gated.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
