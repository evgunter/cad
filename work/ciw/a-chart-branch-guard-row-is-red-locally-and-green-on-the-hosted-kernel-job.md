---
id: a-chart-branch-guard-row-is-red-locally-and-green-on-the-hosted-kernel-job
kind: issue
title: topo's no_file_outside_the_mint_pins_a_chart_branch is red in a clean checkout and green on the hosted kernel job
status: closed
opened: 2026-11-17
closed: 2026-11-22
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

## Closed

2026-11-22, CIW triage (stale). Question answered: the row is in the `ci` slow set (`.config/nextest.toml:39`, `:260`,
0.73 s), and the PR gate runs a slow row only when the diff edits/names it; the input here
is every crate's source, read by a directory walk (`lift_agreement_rows.rs:345-360`), so a
`mesh/src/walk.rs` edit did not select it. Greened by PR 7051 (2026-11-16; green/log.md:425).
Duplicates GREEN's open `work/green/a-slow-row-reaching-its-data-file-by-a-built-name-is-not-gated`
(title names "a directory walk"), which is the better home.
