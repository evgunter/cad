---
id: lamina-plane-annulus-keeps-its-slit
kind: issue
title: sweep: a lamina full revolve's plane annulus keeps its seam slit and its Meridian(Seam, s) name, where the ruling says a planar wall is one face with no meridian
status: closed
opened: 2026-10-01
priority: P2
cost: M
parent: swept-continuation-walls-reach-the-boolean-unmerged
pr: 4136
closed: 2026-10-06
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

## Findings (`band/lamina-annulus-is-one-face`)

Measured on the tree: the lamina needs nothing the solid's plane wall
doesn't. `build_lamina` kills each plane wall's meridian slit with the
same `kemr` the wire case uses (`unslit_plane_wall`, `SlitEnd::Annulus`);
tier 2 holds, the washer's volume bits are unchanged, and the emitter
drops `Meridian(Seam, s)` on its own (it reads `meridians`, now `None`
for a plane wall).

What read the slit was the blend, as this row said, and one more door
than it named:

- `resolve_annulus` took the slit as the plane side's seam. A plane
  host whose cycle carrying the rim is the rim alone is now the
  hostless crossing (`HostFoot::Strut`), the rim the face's outer cycle
  or one of its rings (`AnnulusRim::host_ring`).
- With ONE crossing the host trim spans no two feet, and a `mef` from
  the foot to itself hands the lone circle to the NEW face — the source
  plane would keep the strip. `lone_host_trim` takes the trim through
  the host's own cycle instead (`kemr` the strut, lone-vertex `mef`,
  `ring_move` the rim's ring onto the strip, `mekr` the strut back).
- The ring pass read only circle rings; a notch cut into the bore now
  lands in a ring, so a ring that is not one circle is metered edge by
  edge with the outer cycle (`support_boundary_clearance`).
- The ruled band refused any ringed support; a washer's bottom face
  now carries its bore as a ring. A plane support's rings stay on it
  through the trimline `mef` and are metered by the ring pass, so only
  a CURVED support's ring refuses (`RuledPlan::plan`).
