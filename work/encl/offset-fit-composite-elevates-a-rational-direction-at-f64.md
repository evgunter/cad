---
id: offset-fit-composite-elevates-a-rational-direction-at-f64
kind: issue
title: offset_fit::Composite::build elevates degree-1 directions at f64 before the ring composite; a rational net's elevation ratio rounds
status: open
opened: 2026-10-10
priority: P3
cost: E
---

Found by NURBS fork3 (designer B), off its question. The finding is unverified: nobody has checked whether the composite pads for it.

`crates/geom-brep/src/offset_fit.rs` `Composite::build` (`:2343`) elevates degree-1 directions at `f64` before it builds the ring composite. For a rational net the elevation runs through the projective applier with a stored `f64` λ, so the composite encloses an elevated net with rounded coefficients rather than the described one.

NURBS fork3's decision makes the projective knot algebra unreachable at `Interval` (`[ev]` PR 4539; build on `nurbs/knot-algebra-off-the-ring`). That does not cover this site, because the elevation runs at `f64` and is then lifted. The first step is to measure whether the lifted elevation is padded, or whether the composite is certified about a neighbour of the base.
