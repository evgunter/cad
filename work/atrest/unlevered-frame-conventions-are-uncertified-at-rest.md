---
id: unlevered-frame-conventions-are-uncertified-at-rest
kind: issue
title: A line's unit dir, a plane's unit frame and u_ref ⊥ normal, and a cone's frame are certified nowhere at rest, and check 9 reads a line's dir as unit
status: open
opened: 2026-09-25
priority: P4
cost: D
refs: [ATREST-13]
---


## What

ATREST-13 made check 1 read every analytic frame: a zero `normal`,
`axis`, `u_ref` or `dir` is `PoisonedSurfaceDatum` /
`PoisonedCurveDatum`, and the `axis`/`u_ref` frame of a cylinder,
sphere, torus, circle, ellipse or spiric must be unit and orthogonal to
within the run's ε of locus movement at the kind's radius
(`geom::Surface::representability_margins`,
`geom::Curve3::representability_margins`, `geom::surfaces::frame_margins`).
The frame conventions it does NOT certify, each for a stated reason:

- **a line's unit `dir`** and **a plane's unit `normal` / `u_ref`**:
  each spans the same locus at any length, so a non-unit one is not a
  different locus. It is a different METRIC — `t` stops being arc
  length, `(p − o)·n` stops being a distance — and consumers read that
  metric as metres;
- **a plane's `u_ref ⊥ normal`**: a tilted `u_ref` tilts the chart
  plane off the implicit plane by an amount that grows with the
  distance from `origin`, which no stored datum bounds;
- **a cone's frame**: moves the half-angle, a locus movement that grows
  along the slant.

**Consumers that read the unlevered half as if certified** (ATREST-13's
sweep, curves): check 9's `locus_gap` (`crates/topo/src/validate.rs`)
computes a point's distance to a `Line` as `|d − dir·(d·dir)|`, which is
the distance only for a unit `dir` — at `|dir| = 2` it is not. Check 9's
`meet_segment` / `lines_meet` read the same `dir`. The measured import
surface: `step-import` adopts a near-unit `DIRECTION` verbatim within
the file's ε_in and divides by the norm otherwise
(`Resolver::direction`), so its lines are unit to ε_in; a struct-literal
`Curve3::Line` is not, and **ATREST-13's D-2 table** measured a pillow
chord re-minted with `dir = 2·x̂` (parameters `0 … 0.5`) and with
`dir = ½·x̂` (`0 … 2`): both mint and pass tier 3.

**Measured frame surface (ATREST-13, CI run on the corpus instrument)**
— see ATREST-13's PR for the per-kind maxima; no f64 frame in the
corpus is off unit beyond rounding.

## What must be decided

Whether each unlevered convention is tier 3's (with which lever: the
face's or edge's own extent is a body quantity, not a datum, so it
would be a metered decision rather than a representability read), or
whether the consumers above must read the metric they assume (a
normalized `dir`) and the convention is dropped.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1, check 9);
`crates/geom/src/lib.rs`'s conventions paragraph is `props` ground.
