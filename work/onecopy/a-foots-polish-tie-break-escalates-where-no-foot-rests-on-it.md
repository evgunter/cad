---
id: a-foots-polish-tie-break-escalates-where-no-foot-rests-on-it
kind: issue
title: on_carrier's smaller (curve_foot_polish) decides a polish tie-break at the least band and still records an escalation when an Interval enclosure straddles zero, though no foot rests on it
status: open
opened: 2026-11-03
priority: P3
cost: E
refs: [6449]
rides_with: a-foot-has-no-bare-parameter
---


Found by the PR 6449 fix pass (`onecopy/foot-and-angle`), which unlogged `angle_near`'s half-turn flip (`curve_foot_branch`, audit ledger row F31) for this reason.

`geom::curves::on_carrier`'s `smaller` (`curve_foot_polish`), which serves the ellipse, spiric and spline polish, is the same kind of choice: a heuristic tie-break between two candidates. At `Interval`, an enclosure that straddles zero records a live escalation. `editor-core`'s `drive::log_read` reads a node's first escalation as the leaf's verdict, so a leaf can refuse or bisect on a choice that no answer depends on.

Fix: establish whether any foot depends on the pick. If none does, route it through `check_unlogged` with a ledger row, as F31 does. If one does, the escalation is real, and the row should say which foot depends on it.

## Finding (2026-11-03, `onecopy/small-reads`, PR 6475's review)

A foot does rest on the pick, so the escalation is real, and `smaller` stays logged.

- `crates/topo/src/census.rs` `direction_at` reads `carrier().foot(p, mid).t()` and then `deriv(t)`, without asking `on_carrier`.
- Probe: a cubic hairpin with control points (0,0), (1.4,0), (1.4,h), (0,h), `h = 0.2`. The point is at `t = 0.08` and the seed at `0.92`.
  - On an `Interval` box of ±0.05 or wider, every candidate ties and `smaller` keeps the seed: the foot is `t = 0.92`, whose tangent is reversed and is held by no point of the box.
  - At ±1e-3 the comparison decides and picks the right branch, so a narrower box answers it.
  - Logged, `curve_foot_polish` records 5–9 escalations there; unlogged (PR 6475's first head), none.
- Pinned as `geom::curves::on_carrier::rows::a_tie_across_a_hairpins_branches_reaches_the_log`.

PR 6475 tried routing the pick through `check_unlogged` (ledger row F32) and reverted it.

What this row describes stays true: a converged polish on a point-sized box (±1e-12) still logs 3–4 escalations. The orchestrator files the structural question separately: which readers take a foot's `.t()` without a certificate (`census::direction_at`, `geom-brep` `curve_pair/walk.rs` and `curve_pair/pose.rs`).
