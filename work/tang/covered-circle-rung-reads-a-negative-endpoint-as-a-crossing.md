---
id: covered-circle-rung-reads-a-negative-endpoint-as-a-crossing
kind: issue
title: reduce's covered circle rung refuses a definitely-Negative endpoint as a crossing even when the cover certifies the carrier lies on the non-positive side
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## What

`reduce.rs`'s `curved_face_arm`, the covered circle rung, treats a
definitely `Negative` endpoint as a crossing even when the declared
cover certifies that the edge's carrier lies on the partner's
NON-POSITIVE closed side. An example is the sphere side of a ball
seated in an equal-radius bore. The refusal is fail-loud, not
unsound. No arm admitted today reaches it: the witness lane mints
lines, and `Negative` line endpoints go to the root lane.

## The fix's shape

The separation invariant becomes data: the locus reports which closed
side each carrier lies on, and the covered arms read "on the certified
side" rather than hard-coding `Positive` as clear. This is owed by
whoever builds a circle or curve locus (see
`dev1-cylinder-sphere-circle-locus-arm`). Found by the DEV-1
circle-arm design pair (2026-10-01), reading only.
