---
id: offset-derive-net-folds-drop-a-poisoned-control-point
kind: issue
title: offset_derive folds a wall's or carrier's control net with f64::max, which drops a poisoned point and answers a short finite bound
status: open
opened: 2026-10-10
---


## What

Two folds in `crates/topo/src/offset_derive.rs` take a bound over a
control net with the INHERENT `f64::max`, which is IEEE `maxNum`: a
NaN operand is dropped and the other returned. `geom_core::Real::max`
exists precisely because of this (its docs: `maxNum` "would silently
launder a poisoned value"), but on a concrete `f64` receiver the
inherent method wins.

- The spline section lane's window (`level_row`'s `None` arm in the
  fn that calls it, ~:618-622): `wall.control().iter().map(|p|
  p.distance(origin)).fold(0.0, f64::max)` — a wall net with a
  poisoned point answers the half-extent of its finite points, so the
  `SsiDomain` it builds can be short of the net it claims to hold.
  `level_row` reads every point through `zero(...)?` only until its
  first non-level column breaks the walk, so it does not screen the
  whole net first.
- `polynomial_speed` (~:810-818): `sup = sup.max((ctl[i+1] -
  ctl[i]).norm() * p / dt)` on `f64` — a poisoned control point's
  segments vanish from the sup, and the "speed bound" comes back finite
  and short.

Neither is reproduced: tier-3 check 1 refuses a poisoned or infinite
net at rest (`PoisonedSurfaceDescription`), and whether a poisoned net
can reach either fold mid-operation was not traced. The fix shape is
either a `net_state()` / finiteness screen ahead of each fold, or
`Real::max` spelled as such (`<f64 as Real>::max`), so the NaN
carries into the bound and the consumer refuses on it.

Found by PIPE's S350 sweep (min/max folds over a control net in
`crates/topo/src` and `crates/mesh/src`). `offset_derive.rs` is on no
program's `paths:`; SHELL wrote its section lane (`shell:` commits), so
it is filed here.
