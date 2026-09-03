---
id: blend-suite-fixture-and-oracle-copies
kind: issue
title: Blend suite tree - one test-support home for the cavity/cube fixture builders and the blend volume oracles
status: open
opened: 2026-08-31
github: 1364
refs: [1360]
---

## From GitHub issue 1364

Opened 2026-08-31; 0 comments.

(BLEND-4 fix pass, PR #1360 — the declared-but-unscheduled half of its deviation 3, filed per the adjudication.)

Two copy classes run through the blend suite tree, each declared at its sites but homed nowhere:

**1. Fixture builders.** `brick`/`rod`(/`prism`)/vented-cavity/cavity-edge-finder are restated per suite: `blend3_concave_chamfer.rs`, `blend3_r2_probes.rs`, `review_blend3_r1_probes.rs`, `blend4_concave_fillet.rs`, `blend4_r1_probes.rs` (prism-generalized), `review_blend4_r2_probes.rs`. Each restatement is a place for a fixture to drift from the body its siblings measure.

**2. Closed-form volume oracles.**
- The rounded-box Steiner form `L³ + 6L²r + 3πLr² + (4/3)πr³` (and its die-shaped spellings): `blend4_concave_fillet.rs::filleted_cavity_volume`, `review_blend3_r1_probes.rs` (two-sided fillet row), `blend4_r1_probes.rs::rounded_void_volume` (prism-generalized), `m5_pr12_die.rs`, `m5_pr12_die_body.rs` (twice), `m6_surgery.rs`, `m6_surgery_interval.rs`.
- The chamfered-cube form `6ad² − (16/3)d³` family: `blend3_concave_chamfer.rs` (whose doc already declares itself "the FIFTH copy"), `review_blend3_r1_probes.rs` (twice), `review_chamfer_r1_probes.rs` (twice), the chamfer acceptance suite, `sf2a_r2_probes`.

The ask: one home in `sweep::test_support` (or a `tests/common` module) carrying the builders and the oracles together, with the per-suite restatements retired to calls. Independent-oracle value is preserved by keeping the oracle's derivation documented at the home rather than per site — the point of the copies was never independence (they are byte-identical), only locality.

Until it lands, sites touched by PR #1360 cite this issue at their restatements.

## Home

S-TCOST: every site named is under `crates/*/tests/*` and the ask is one `test_support` home for shared fixture initialization — S-TCOST's territory and its "merging tests that share initialization" lever.
