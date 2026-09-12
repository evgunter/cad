---
id: part-per-node-logs-dropped-with-nested-evaluation
kind: issue
title: A part's per-node verdict and escalation logs are dropped with its nested Evaluation
status: open
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, 1969]
---

## What

An instantiated part evaluates as its own nested `Evaluation`
(`crates/editor-core/src/eval/parts.rs`, `resolve_and_evaluate`), and
every node of that evaluation records its own verdict and escalation
frame. `PartValue` keeps `body`, `names` and `contacts` only, so the
nested evaluation — and with it the part's per-node logs — is dropped
at the end of the part cache's miss path. Measured (PR #1969's nesting
row): the part evaluated directly records 724 verdicts on its nodes;
the instantiate node records the placing op's own 466; nothing
surfaces the 724 through the instantiator.

## Consequence

`resolve::vdiff` cannot attribute a flip INSIDE an instantiated part
to anything — the assembly's per-node logs hold only placement and
validation decisions — so a NAMING-DESIGN N5 diagnosis of a part-level
predicate flip is unreachable from the assembly document.

## Shape to decide

Which node owns a part's decisions. Options: `PartValue` retains the
nested evaluation's per-node logs and the instantiate node's value
exposes them keyed by (instance, inner node); or the part's own
document is the place to diff (the assembly points at it); or the
instantiator's frame is defined as the flattened union. PR #1969 chose
none of these on purpose — its rule is "a node's log is its own op's
decisions, the same on a hit and a miss" — and this is the question
that rule leaves open.
