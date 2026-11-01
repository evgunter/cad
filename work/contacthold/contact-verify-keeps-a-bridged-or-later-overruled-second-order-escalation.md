---
id: contact-verify-keeps-a-bridged-or-later-overruled-second-order-escalation
kind: issue
title: contact_verify's tangency ladder keeps a second-order escalation on the log when a declaration bridges it, or when a later sample contradicts
status: open
opened: 2026-10-10
priority: P3
cost: M
---


Split from `contact-verify-logs-a-second-order-escalation-its-outcome-overruled`.
PR 4558 fixed that row's bullets 1 and 3 and left these two shapes.

## What

`topo::boolean::contact_verify::tangent_locus_relation` reads the
second order (`contact_tangent_second_order`) and the first order
(`contact_tangent_parallel`) detached at each sample. It splices the
second-order reading with `k_stats::splice_superseded` only when that
reading is in band and the same sample's first-order reading refuses.
Two outcomes still leave an escalation on the frame's log that the
outcome did not rest on:

- **Bridged.** With `declared`, an in-band second-order reading is
  the bridged residue. The ladder returns `Ok(Bridged)`, but the
  reading is spliced live, so a node that built reads to the driver
  (`editor-core` `drive.rs` `classify_replay`, read (2)) as an
  undecided `contact_tangent_second_order`. After INTENT E this is the
  normal path: every production caller passes `declared = true`
  (`contact_pair_verdict`, `boolean::verify_tangency_declaration`, and
  the glue door through it).
- **Across samples.** Sample i bridges, with its in-band second-order
  escalation spliced live, and then sample j > i contradicts (a
  definite residual, opposition or parallelism defect). The log keeps
  sample i's escalation, and the outcome is a contradiction that never
  rested on it.

## Owed

First, decide whether a bridged node SHOULD refine. A declaration
carries the residue by design (C4, the #175 clause). If it should
not, hold each sample's second-order recording until the loop's outcome
is known, then splice it superseded on `Bridged` and on any later
refusal. A red-first row brackets the ladder on a declared pair whose
second-order reading is in band at one sample and whose residual is
definitely off at a later one, then asserts the log holds no
second-order escalation.
