---
id: bracket-scope-is-run-op-not-the-node
kind: issue
title: The verdict bracket's scope is run_op, not the node: the profile pre-pass and the mate solve decide before any bracket opens
status: open
opened: 2026-09-05
refs: [1969]
---

## What

The verdict bracket in `eval_node` (`crates/editor-core/src/eval/mod.rs`)
opens around `wire::run_op` and nothing else. Two decision-making
passes run BEFORE it and record on no node:

- the profile pre-pass — `profile_pre` / `lane_program`
  (`crates/editor-core/src/eval/mod.rs:2380` and the lift's second
  pass above it), computed before the memo lookup;
- the whole-document mate solve — `mate::solve_document`
  (`crates/editor-core/src/eval/mod.rs:2058`), run once per
  evaluation before any node.

At top level those decisions land in no frame. Measured (R2, pinned by
`kstats_bracket_rows::the_decisions_outside_every_node_bracket_are_the_pre_pass_ones`):
the one-solid part fixture records **724** verdicts on its nodes and
**75** outside every bracket (`chord_side` 28, `line_span` 8, …); the
two-instance assembly records **0** outside (the part's are shielded on
the cache's miss path). Before PR #1969 the same decisions landed on
whichever instantiate node's frame enclosed the nested run; the PR
shields them (`PartCache::get`) rather than widening the bracket,
because widening moves every Profile node's log and the verdict-log
goldens with it.

## Why it matters

`resolve::vdiff` and the driver's verdict vector see a Profile node's
op decisions but not its pre-pass ones; a flip in the pre-pass is
invisible to both. A mate-solve escalation reaches the driver only as
`NodeErrorKind::Mate` (see
`work/props/escalation-channel-misses-op-minted-indeterminates.md`).

## Home

LIB/SEAT ground (`eval/mod.rs`), so no program: the bracket's scope is
a design choice about what a node's log means, to be made with the
per-node profile pre-pass's owner.
