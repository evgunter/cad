---
id: BOOL-7
kind: unit
title: issue 134 — the vdiff shadow-exec rung
status: open
opened: 2026-09-01
refs: [134]
---

Under the Q3 ruling (S-BOOL takes it; M10 is dormant): when the vdiff engine
hits an empty pair population on a verdict vanish, shadow-execute exactly the
vanished pair's predicates from the prior evaluation's context and diff
those — bounded, diagnosis-time-only, recovering the full `PredicateFlip`
(Ev's standing option (a), 2026-07-29). Ground is
`editor-core/resolve/vdiff.rs` and its immediate callers only; the unit stops
and reports if the work reaches M10's Dual arms or the `AtRestPolicy` seam.
Difficulty M.

Queued on the shared lane budget — `work/bool/log.md`, the BOOL-3 and
BOOL-8 entries' slate lines.
