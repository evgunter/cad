---
id: topo-tests-straddle-seat-hand-copies
kind: issue
title: common::straddle_seat claims one shared builder and at least two suites hand-copy it, plus a five-way crossing-post profile
status: open
opened: 2026-09-16
priority: P4
cost: E
---

## Finding

- **Where**: `crates/topo/tests/` — `common/mod.rs`'s `straddle_seat`,
  against `mate9_crossing_rung.rs`, `review_mate9_r1_probes.rs`,
  `review_mate9_r2_probes.rs`; and the crossing-post profile in
  `m9_c1_r1_probes.rs`, `mate4a_ef_bound_rung.rs`,
  `mate8_witness_schedule.rs`, `r1_mate8_probes.rs`,
  `review_mate4a_r2_probes.rs`.
- **Importance**: medium
- **Confidence**: sure about the copies; the right home for the second
  half is a judgement the unit has to make
- **Raised by**: the `dup-brick` lane (S-DUP), 2026-09-16, from the
  construction half of its sweep

`common::straddle_seat`'s own doc says **"ONE builder, shared … two
hand-copies would let a drift silently decouple them"**, and names
`mate4a_ef_bound_rung` and `mate9_crossing_rung` as the pair it serves.
Both of those do call it. But the post profile it builds —
`[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)]` — is written
out **eight more times** by hand next to a shelf
`[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)]`:

| file | hand copies |
| --- | --- |
| `mate9_crossing_rung.rs` | 4 (the same file that calls the shared builder elsewhere) |
| `review_mate9_r1_probes.rs` | 2 |
| `review_mate9_r2_probes.rs` | 2 |

Some of them vary the z range deliberately (`mate9_crossing_rung.rs`
lifts the post to `0.5 .. 1.0` in one row and `0.0 .. 1.0` in another),
which is exactly why they were written out — `straddle_seat()` fixes
z and returns a grafted body with four named keys, and a row that wants
the post at a different height, or wants the two prisms ungrafted,
cannot use it. So the remedy is not "call the existing builder": it is
to decide what the shared door should take, and the honest first step is
to stop `straddle_seat`'s header asserting a sharing that four files
contradict.

**The second half is a different profile with the same problem.** Five
suites spell a seven-to-eight-corner "crossing post" —
`(0.20, 0.20) A`, `(0.40, 0.30) B` on the shelf edge, `(0.60, 0.42) C1`,
`(0.70, 0.30) T` tangent, `(0.80, 0.42) C2`, … — with the SAME corner
comments copied along with it in three of them
(`m9_c1_r1_probes.rs`, `mate8_witness_schedule.rs`,
`review_mate4a_r2_probes.rs`; `r1_mate8_probes.rs` carries the corners
without the comments, `mate4a_ef_bound_rung.rs` has its own). A comment
copied by hand alongside a coordinate list is the drift hazard the
`straddle_seat` header was written to name.

**Commands.** `grep -c '(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)' crates/topo/tests/*.rs`
and `grep -c '(0.40, 0.30)' crates/topo/tests/*.rs`, both at
`79e81be5`. **Blind spot**: both greps are coordinate-literal greps, so
a copy that rounded a corner, reordered the profile cyclically, or built
the same seat from a transform of another body does not appear in
either count.
