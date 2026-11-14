---
id: a-whole-turn-rims-headroom-is-read-through-a-one-ulp-tau
kind: issue
title: A whole-turn rim's headroom against its period is read through a one-ulp-wide τ, so at Interval a full rim past ε·2^50 metres escalates props_rim_span_winding
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by BOXFINISH's measurement of its `f64` hairline row (2026-11-14,
main `b363f5fa`; a cloud lane with local debug hooks, nothing pushed to
main). Filed here because the witness is this track's 1.3 km tube
(`tier-3-refuses-a-major-1300-torus-tube-at-1e-12-that-the-measurement-converges-on`)
and the width grows with distance.

## Measured

The hollow tube (major 1300, outer 2.5, wall 0.001, full period) at
ε = 1e-12, with the session box bypassed locally (as the door stands it
refuses `OutsideSessionBox`, margin −302.5, before any tier-3 check):

```
VolumeUncomputable { source: Face { face: FaceKey(1v5), source: Escalated {
  cause: Indeterminate { margin: Enclosure { lo: 0.0, hi: 1.1524123877393322e-12 },
  band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: "props_rim_span_winding" },
  check: Inventory } } }
```

- **Raising site:** `require_rim_span` → `require_circle_span` →
  `require_span_within_period` → `Period::not_over` → `Exact::read`
  (`crates/geom-core/src/period.rs`), at `Interval`, inside the role walk
  (`role_walk_set`) that re-derives the `f64` round. The same rim at
  `f64` decides `Zero` on `Value(0.0)`.
- **The quantity:** the headroom of the rim's stored span against one
  turn, `(τ − extent)·r_c`, in metres at the rim's own radius. The band
  is plain: zero ε, escalate K·ε.
- **The enclosure** is `[0, r_c·2^-50]` exactly: 1297.501 m gives
  1.15241e-12, 1302.5 m gives 1.15685e-12. Inferred from that exact
  match: the interval τ is `[τ_f64, next_up(τ_f64)]` and the extent is
  the point `τ_f64`.
- **The true value** (60-digit decimal, independent of the reader):
  `2π − τ_f64` = 2.4493e-16 rad, so the headroom is 3.18e-13 m, under ε:
  decidably a whole turn. The enclosure is honest and 3.6× loose.
- **Reach:** the tube builds at ε ≥ 1.16e-12 and refuses at ≤ 1.15e-12;
  the wall thickness plays no part (refuses at wall 0.001 through 2.0).
  The rule is any full-turn rim with `r_c > ε·2^50`, 1125.9 m at 1e-12.
  Inside the session box that is reached only by a tilted full rim of
  about 1126 to 1225 m: a tube about the axis (1,1,1)/√3 with major 1130
  and 1215 refuses this row (`[0, 1.0014e-12]`, `[0, 1.0769e-12]`).

## What closes it

A stored span that IS one period reads its headroom as exactly zero
(the extent and the period are one value by construction), rather than
through the difference of a point and a one-ulp interval. Unmeasured
whether another reader takes the same difference.
