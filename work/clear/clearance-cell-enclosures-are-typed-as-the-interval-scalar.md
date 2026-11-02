---
id: clearance-cell-enclosures-are-typed-as-the-interval-scalar
kind: issue
title: The clearance engine's cell enclosures are typed as the Interval scalar: every input value enters through Interval::from_certified, so a bound on a function over a cell and an uncertain number are one type
status: open
opened: 2026-11-02
priority: P0
cost: M
---


Raised by Ev on PR 6308 (2026-11-02): "it seems wrong to be putting in
an f64 as an interval? ... isn't it semantically a different type than
the input type? i guess that might be a bit ugly since `Sym` isn't
compatible", then "i'm not sure if `Sym` is completely incompatible or
just difficult/impractical".

`topo::clearance` and `topo::injective` subdivide each face's parameter
domain. For each cell they need an enclosure of the cell's whole image:
a set bound on a function over a domain, which no point evaluation
gives. They build it in `Interval` arithmetic, and every stored value
of the `Body<T>` enters through `Interval::from_certified`
(`clearance.rs` module docs; `window_of`, `boundary_of`, the `lift`
module). So one type, `Interval`, plays two roles. One is the kernel's
uncertain-number scalar: the value of a written quantity across its
family. The other is the engine's range bound. At `f64` a stored value
enters as a degenerate interval, which is exact as a conversion but
reads as treating the float as the value.

**The question, for designers before the at-rest-gate unit is specced**
(that unit is where f64 first enters the engine):
- Whether the engine gets its own enclosure type, built from `Body<T>`
  per scalar, and how each scalar enters it: f64 and Probe as points,
  with the 2ε margin `[topology.tiers]` now states; Interval as the
  family's brackets.
- What `Sym` gives. Ev's question is whether it is incompatible, or
  only difficult or impractical. Today it reaches the engine only
  through its interval bounds.

Priority: P0, because `the-at-rest-gate-certifies-embedding` (P0) waits
on it (a blocker inherits the band of what waits on it).
