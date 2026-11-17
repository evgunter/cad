---
id: a-chart-branch-guard-row-is-red-locally-and-green-on-the-hosted-kernel-job
kind: issue
title: topo's no_file_outside_the_mint_pins_a_chart_branch is red in a clean checkout and green on the hosted kernel job
status: open
opened: 2026-11-17
---



## What

`topo`'s `pcurves::lift_agreement_rows::no_file_outside_the_mint_pins_a_chart_branch`
(`crates/topo/src/pcurves/lift_agreement_rows.rs`) is red in a clean
checkout of `origin/main` at `a56edec9d6` and of
`tesslane/route-by-domain`: it counts 6 uses of the listed chart-branch
spellings in `mesh/src/walk.rs` against a listed 2 (`unwrap_tie`, "the
curved lane's pole junction"). The hosted kernel job is green on both
heads, so the gate either does not run this row or runs it differently
(a feature set, a filter, a profile that leaves it out). Found by the
`tesslane/route-by-domain` review (2026-11-17) and reproduced on that
unit's container with `cargo nextest run -p topo` whole.

## What would settle it

Read the hosted test job's selection for `topo` (`.github/workflows/ci.yml`,
`.config/nextest.toml`'s `ci` profile) for whether this row runs, and
under what features; then either make it run, or say in the row why it
does not. The count itself (6 against 2) is the next question, for
whoever owns `mesh/src/walk.rs`'s pole junction.

- 2026-11-17: green again locally from `origin/main` at `6396c45978`
  (PR 7051 re-listed the walk's count). The question this row asks
  stands: the row was red for as long as main carried the 6-against-2
  count, and the hosted kernel job did not see it.
