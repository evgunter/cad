---
id: the-strip-screen-reads-a-supports-vertices-not-its-edges
kind: issue
title: blend: reach::screened reads a support's vertices alone, so an edge across the strip with no vertex in it goes unseen
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## Finding

`reach::screened` (`blend/reach.rs`) is the metered premise of
`Reach::replaces`: the strip a straight link's band replaces holds no
boundary feature of the support but the request's own. It reads the
support's VERTICES against the region, so a boundary edge can cross
the region with neither end inside it.

**The shape that can occur is a curved edge.** A straight edge cannot
cross the region unseen by predicate 2's sampled screen. Its distance
to the requested edge's line is linear along it, so within the window
it is least at a window end, and both window ends are screen samples.
The screen reads that distance, less than the band's chord, and
refuses. A curved edge can dip into the region between two samples
with both its ends outside.

**Witness of the premise.**
`band_planar_cut_off_meters::a_far_side_arc_with_no_vertex_in_the_tube_is_clear_of_the_strip`
builds it. A far-side plateau carries a semicircular bump of radius
`0.03` under `x = 3.0625`, midway between two of the tip's samples.
- Its apex stands `0.09` from the tip's line, inside the tube of
  radius `D`. Its two vertices stand `0.12` from it, outside.
- Instrumented (a temporary print on `screened`'s return), the top
  face reads `vacant: true` for every request in the row, although an
  arc of the face lies across the line within the tube.
- Every request builds at the closed form, tier 3 and 3′: alone and
  with a second tab's tip a chain apart, pocket and boss, both verbs.

Two readers stand on the premise:
- `screened`'s own refusal, on the trimline's side. A near-side
  curved edge of that shape is read elsewhere: arm (d)'s strip meter
  for a plane–plane link reads edges
  (`a_ring_arc_with_no_vertex_in_a_requested_edges_strip_refuses_at_the_strip_meter`),
  and the cell meter for the edge's own walls.
- `band_reach`'s cross-chain excuse, which reads the strip on BOTH
  sides of the line where `screened` finds no vertex across it (that
  is what lets an oblique cell straddling the line be excused). A
  far-side curved edge of that shape keeps the excuse two-sided, so a
  cell of the support's far part could be excused though no band
  replaces it.

**Why the witness shows no wrong excuse.** No band reads the excuse
there. Every chain whose band reaches the bumped face in that row has
the face as a support, and a band skips its own supports. The excuse
matters only for a band from a chain the face does NOT support that
reaches the face's far lobe: a concave band adding material over it,
as in the sealed void. In the void-and-island arrangement such a band
touches the island top only within the pad of the island's outer
edge, which must then be requested to excuse its wall, and that
edge's own strip, truly removed, covers the touched cells. No
arrangement found makes the over-coverage excuse something no band
removes. That is argument, not proof. The merge base read the excuse
two-sided always, so this is no wider than it was.

## What would close it

Read each curved boundary edge of the support against the region,
certified (an arc's least distance to the edge's line over the window
is a closed form), rather than its ends; or build a band that reads
the excuse over such an edge and show it refuses.

