---
id: covered-circle-rung-reads-a-negative-endpoint-as-a-crossing
kind: issue
title: reduce's covered circle rung refuses a definitely-Negative endpoint as a crossing even when the cover certifies the carrier lies on the non-positive side
status: closed
opened: 2026-10-01
priority: P3
cost: M
closed: 2026-10-26
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

## Closed (2026-10-26, tracker sweep)

Fixed on main, in the shape this row asked for. The separation fact is
data now: `CoverSide` (`crates/topo/src/boolean/mod.rs:1503-1530`) is
`On`, `Off(Sign)` (the closed side, read off the parent face's boundary)
or `Unread`, and `CoverSide::admits(sign)` takes a decided endpoint only
on the certified side. The covered circle rung in
`reduce::curved_face_arm` reads each end through it
(`crates/topo/src/boolean/reduce.rs:2283-2322`): an end definitely off
on the side the certificate puts the parent is eventless
(`sign if admitted(sign) => {}`, `:2321`), and only the other side keeps
the frontier door. So a `Negative` endpoint under a non-positive cover no
longer reads as a crossing. Nothing here hard-codes `Positive` as the
clear side. The change predates this checkout's shallow history, so no
commit is cited.

