---
id: a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles
kind: issue
title: carrier_cross::parallel_circles places a near-tangent coplanar circle pair's touch by the generic radical-line foot, 0.1 m off both circles: the boolean's crossing layer silently misses the meeting
status: open
opened: 2026-10-13
priority: P1
cost: E
---


## The finding

Measured by a FUSE designer lane weighing `work/fuse/two-curve-curve-meeting-finders.md`. The probe ran in a throwaway worktree and was never committed.

The fault is in `carrier_cross::parallel_circles` (`crates/topo/src/boolean/carrier_cross.rs`), in the touch branch `if apart == Sign::Zero || nested == Sign::Zero { return Ok(vec![foot]); }`. It places the touch with the generic radical-line foot `c1 + e·(d² + r1² − r2²)/2d`. That formula multiplies the decision's residue (`d − |r1 − r2|`, or `|r1 + r2| − d`) by `(r1 + r2)/2d`, which is unbounded as `d → 0`.

- **Reproduction:**
  - the witness band (zero `1e-9`, escalate `1e-8`);
  - two coplanar circles: a unit circle at the origin, and a circle at `(1e-8, 0, 0)` of radius `1 + 9e-9`.
- **Expected:** the circles pass within `1e-9` of each other at `(−1, 0, 0)`, so the touch candidate should land there.
- **Got:** `nested` reads `Zero` and the candidate lands at `(−0.9, 0, 0)`, about 0.1 m off both circles. The confirm step then reads it off, so the boolean's crossing layer reports no meeting.
- **Cross-check:** the census walk (`census::curved::reads_meet`) on the same pair answers `Some(true)`.
- **Who can reach it:** two rims of nearly equal radius in one plane, for example a thin washer's caps on a shaft.

The same probe found no drift in the two-crossing arm (residuals ≤ 5e-13 over 106 rows) or in the tilted common-line arm (no drift down to `s = 1e-8`).

## What it needs

Place a candidate admitted under a `Zero` reading with the exactly degenerate formula, not the generic one: `foot = c1 + e·r1` when `apart == Zero`, and `c1 + e·r1·sign(r1 − r2)` when `nested == Zero`. That point lies exactly on the first circle and within the residue of the second. A ci row should pin the reproduction above.

Sweep the other closed-form arms for the same class: a generic formula evaluated at data a `Zero` reading has just called degenerate. The class is also a contract clause in the FUSE curve-meeting design (`two-curve-curve-meeting-finders`).
