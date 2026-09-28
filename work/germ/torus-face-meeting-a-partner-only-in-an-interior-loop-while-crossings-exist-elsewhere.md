---
id: torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere
kind: issue
title: A torus face that meets a partner face only in an interior loop, while crossings exist elsewhere, is classified by face-region propagation that cannot see the loop
status: open
opened: 2026-09-26
refs: [torus-operand-gate-admission]
priority: P1
cost: H
---

## What

PR #3265's `torus_extent_gate` (`crates/topo/src/boolean/ops.rs`)
closes the no-crossings case. There, a torus face that may meet a
partner face in a closed loop interior to both refuses typed, instead
of reaching `classify_shells`' vertex probe.

The same loop can exist when the operation DOES have crossings
somewhere else, for example a bracket whose plate grazes a donut's
outer equator in an oval while its leg pierces the tube. The fallback
is not taken then. The torus face that holds the oval is classified
through the joined regions and `finish.rs`'s uncut-component probe,
and neither sees a loop no edge crosses. The shape is kind-generic:
cylinder wall×wall pairs have it too, and `cylinder_extent_gate`
guards only the no-crossings fallback.

**Unmeasured.** No transverse torus union completes today: every one
measured stops at the chord rule, the sagitta charge or the join. So
the wrong answer this describes is not reached by any fixture now. It
becomes reachable the day one of those doors opens, and the unit that
opens it owns this row.

## Home

GERM, beside the torus doors. Found by the PR #3265 fix pass's
roster re-sweep.
