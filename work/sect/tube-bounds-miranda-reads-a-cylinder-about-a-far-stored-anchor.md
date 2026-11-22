---
id: tube-bounds-miranda-reads-a-cylinder-about-a-far-stored-anchor
kind: issue
title: tube_bounds' Miranda square reads a cylinder's implicit form about its stored anchor, which rounds past ε 1e-12 when the anchor stands hundreds of metres along the axis
status: open
opened: 2026-11-20
priority: P3
cost: C
---


Found by SECT's `sect/ball-cylinder-cost` lane (2026-11-20), adding the
review's far-anchor pose to `tests/section_bounds.rs`.

## Measured

`geom_brep::intersect::section_bounds`'s `Quadric::value` evaluates a
cylinder as `‖v‖² − (v·â)² − R²` with `v = y − O`, `O` the stored anchor.
A sphere × cylinder pose near `(−400, 500, 300)` whose cylinder anchor is
stored 500 back along its axis states nothing at
`CAD_TOLERANCE_EPS=1e-12` (`Unstated::Pose("no point of the section is
certified within the moat of the carrier's point")`, from `miranda`):
the two squares are about 2.5e5 and cancel, rounding the value by about
3e-11, past the moat `16ε/s ≈ 2e-11`. With the anchor 50 back the same
pose states its floor at every ε row. The refusal is sound (the tube
stays owed); any cylinder pair reads the same form.

## What closing it takes

Read a cylinder's form about the anchor's foot nearest the square's
centre (`O + ((c − O)·â)â`), which is the same surface.
