---
id: lamina-plane-annulus-keeps-its-slit
kind: issue
title: "sweep: a lamina full revolve's plane annulus keeps its seam slit and its Meridian(Seam, s) name, where the ruling says a planar wall is one face with no meridian"
status: open
opened: 2026-10-01
priority: P2
cost: M
parent: swept-continuation-walls-reach-the-boolean-unmerged
---

Left behind by `swept-continuation-walls-reach-the-boolean-unmerged`.
Ev's ruling builds a full revolve's planar wall as ONE face: plain
`Band(s)`, no `BandPi`, no `Meridian(·, s)`. The wire case (an outer
loop touching the axis) does that: `crates/sweep/src/revolve/full.rs`
kills the angle-0 meridian with `kemr`, so an annulus's inner circle
becomes a ring. The LAMINA case (no axis contact, e.g. a washer or a
ring's flat faces) does not. `build_lamina` (`full.rs` ~l.301) leaves
the plane annulus as one face whose boundary is a single cycle through
a doubly-traversed seam slit, and the emitter names that slit
`Meridian(Seam, s)`.

## Why it was left

Unslitting the lamina annulus was built and measured, then reverted:

- The one-edge rim blend reads the slit. `blend/surgery.rs`
  `resolve_annulus` / `wall_seam` (~l.1794, ~l.1874) admit a one-edge
  closed rim by finding the doubly-traversed seam meridian on each
  support. With the plane support unslit, that admission fails, and
  the band's surgery refuses `FaceHasRings` on the ringed annulus.
- The boolean accepts the slit annulus as built. A slit is one face on
  one plane key, so it is not a same-key adjacency, and F7's gate is
  satisfied.

So F7 holds for the lamina. What is false is the naming half of the
ruling (`Meridian(Seam, s)` still names a plane wall's edge), along
with the uniformity "a planar wall has no meridian".

## The row

Teach `resolve_annulus` / `wall_seam` to take a plane support that
carries the rim as a ring (no seam edge on that side), then unslit the
lamina's plane annulus the way the wire case does (`kemr`). Drop the
`Meridian(Seam, s)` row for a plane wall in
`editor-core/src/names/emit_sweep.rs::name_revolve`, and update
`RevolvedKind::Full`'s doc (`revolve/mod.rs`), which states the lamina
exception today.
