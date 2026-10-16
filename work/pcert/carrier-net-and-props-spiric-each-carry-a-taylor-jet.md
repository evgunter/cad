---
id: carrier-net-and-props-spiric-each-carry-a-taylor-jet
kind: issue
title: carrier_net and props/spiric each carry an interval Taylor jet
status: open
opened: 2026-10-12
priority: P3
---


## What

Two interval Taylor-series implementations of the same functions
(`cos`, `sin`, the spiric's `√((R + r·cos t)² − d²)`) live side by
side:

- `crates/geom-brep/src/carrier_net.rs` (PR 4900): degree 6 plus a
  remainder term (`ORDER = 6`, `TERMS = 8`), `cos`/`sin` from
  `Interval::sin_cos`, with `mul` and `sqrt` recurrences on `[Interval; TERMS]`.
- `crates/geom-brep/src/props/spiric.rs`: `Jet([Interval; 5])` (degree
  4), with `cos`/`sin` from `super::quad`'s transcendental-free
  rotation (`trig_at`, `trig_range`), and `mul`, `recip`, `sqrt`
  and `rho_f`.

Both feed certified bounds, and both are sound as written.

Folding them into one home is not local to either change. The orders
differ, the trig sources differ (an outward-padded `sin_cos` against
the rotation from the exact `(1, 0)`), and `props/spiric.rs` is the
props program's. One `Jet<const N: usize>` with an injected trig source would serve
both.

## Open

Whether one jet is wanted, where it lives, and which trig source is
the canonical one (the rotation is transcendental-free and was chosen
for that, per `props/spiric.rs`'s module docs).
