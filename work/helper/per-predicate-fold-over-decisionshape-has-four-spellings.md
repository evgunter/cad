---
id: per-predicate-fold-over-decisionshape-has-four-spellings
kind: issue
title: the per-predicate fold over a replay's DecisionShape stream is written four times across editor-core's suites
status: open
opened: 2026-09-21
priority: P2
cost: E
---


**Found by DECIDE-1's R1 review** (the self-dot census, #3001), which
caught the unit adding a FIFTH. Every measurement suite over the
symbolic tier reduces one replay's `Vec<DecisionShape>` to a
`BTreeMap<&'static str, …>` keyed by predicate, and each writes the
fold itself:

- `crates/editor-core/tests/m10_8_harness.rs`, `split` (`:114`) —
  `[theorem, sign-gated, registered, numeric]`, the discharge table the
  pins assert whole.
- `crates/editor-core/tests/m10_8_harness.rs`, `over_band_set`
  (`:235`) — the widest unclassified enclosure and the over-band
  counts, sorted widest first.
- `crates/editor-core/tests/m10_9_r2_probes_interval.rs`, `envelopes`
  (`:564`) — `(lo, hi, outcome, n, numeric)` per predicate.
- `crates/editor-core/tests/m10_7_r1_census_probe.rs`, `split_at_point`
  (`:96`) — a two-column `(u64, u64)` at the nominal.

DECIDE-1's `blocked` was the fifth and is now a sibling of `split` in
`m10_8_harness.rs` (`:138`), which is the home this row says the other
three belong in too — the harness's own module doc already claims to be
"the ONE home of the whole-box probe … that the pins, the evidence
suite and both reviewers' suites share".

**What is owed:** one fold over the stream with the per-predicate
accumulator a parameter, or — cheaper and probably enough — the two
strays moved beside `split` and `blocked` and their callers pointed at
the harness. What must NOT happen is a sixth: the next suite that wants
a per-predicate view of a replay has four to copy from and no door.

The cost of the copies is not lines. `split` carries the doc that says
which claim its shape supports (discharge, not blocking); the strays
carry none, so a reader cannot tell from `envelopes` whether its
`numeric` column includes the `Invalid`s — which is exactly the
distinction DECIDE-1's measurement turned on.
