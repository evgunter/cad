---
id: no-guard-reds-a-decide-name-missing-from-the-k-report
kind: issue
title: the K-roster claim has no mechanical guard: a new decide("…") name can ship with no docs/K-REPORT.md row and nothing reds
status: open
opened: 2026-09-14
priority: P3
cost: D
---

Filed by TOPO's `tier3-accepts-a-ring-outside-its-outer-loop` (PR 2529)
out of its own fix pass, where the lane had to make the claim and both
blinded reviews had to check it by hand.

## The claim every lane makes, and what checks it

A lane that adds a predicate says one of two things in its PR: *"this
mints a new predicate row, and here is its `docs/K-REPORT.md` entry"*,
or *"this is a new CONSUMER of existing rows, so the roster is
unchanged"*. PR 2529 says the second — check 9's nesting arm pools into
`splitting::containment`'s `point_in_loop_*` rows, the way
`boolean::contfp` and the solid-containment sweep already do — and
`crates/topo/src/ray_parity.rs`'s `ParityRows` states the rule that
makes the claim meaningful: **a new `ParityRows` VALUE is a roster
change.**

Nothing executes that rule, or any rule of its shape.

- `crates/geom-core/tests/flagged_census.rs` scans the shipped trees
  for `decide_flagged` sites and asserts each names a
  `docs/predicate-dimension-audit.md` row. That is the pattern this
  row asks for — for a different door, and only for the flagged lane.
- `crates/profile/tests/recourse_roster.rs` enumerates every
  `decide("…")` name `crates/profile/src` decides and asserts each is
  routed or explicitly unrouted with a reason. Again the right shape,
  and scoped to one crate and to ROUTING rather than to the K roster.
- `crates/sweep/tests/k_report.rs` builds the acceptance shapes at the
  recording scalar and dumps every `MarginSample` as CSV. It PRODUCES
  the data; it asserts nothing about which names ought to be in it.
- `tools/k-lint` reads the produced distributions against thresholds.
  A name that was never sampled has no distribution to lint, so a
  missing row is exactly the case it cannot see.

So a lane that mints `decide("my_new_row")` and forgets `K-REPORT.md`
gets a green PR, twelve green `test (…)` jobs and five green
`k-lint (gate, …)` jobs. The roster silently stops being the roster,
and the next lane to reason from it reasons from a stale document.

## What a guard would look like

The two precedents above are both already in the tree, so this is a
third instance of a pattern rather than a new idea: walk every
`crates/*/src` source for the `decide` / `decide_flagged` /
`decide_invariant` funnel's string-literal first argument, and assert
each name either appears in `docs/K-REPORT.md` (or its data CSVs) or is
listed as a deliberate exemption with a reason. `test_utils::source`
already supplies the code-only walk that keeps such a scan from
counting comments, and `crates/topo/src/face_normal.rs`'s
`every_hand_multiply_of_the_face_sign_is_inventoried` is the local
model for the per-file inventory shape and for its "what this cannot
match" honesty section.

The open questions, which is why this is an issue and not a unit: where
the roster of record actually lives (the document's prose, its M-era
addenda, or `docs/k-report-data/*.csv`), and whether the guard belongs
in the per-PR gate or reads as a `tools/` row. Both are INSTR's.

## Home

`docs/K-REPORT.md` and `tools/*` are INSTR's territory
(`work/instr/program.md`'s `paths`). The producing sites are spread
across `crates/*/src` and belong to no one program, which is precisely
why the guard belongs beside the document rather than beside any one
producer.
