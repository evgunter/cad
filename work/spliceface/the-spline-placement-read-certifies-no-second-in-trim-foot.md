---
id: the-spline-placement-read-certifies-no-second-in-trim-foot
kind: issue
title: The spline placement read certifies no second in-trim foot within the band: a point placed In on a spline face may lie within the band of another sheet of the face, and every reader of its foot reads the first
status: open
opened: 2026-10-31
priority: P3
cost: H
---


Filed by `splice/a-vertex-piercing-a-spline-face-reads-no-outward-normal`'s
lane, on the SPLICE orchestrator's ruling (2026-10-31). Latent: no
fixture reaches it.

## What

`topo::boolean::spline_region::read` (`read_capped`) walks a spline
chart's Bézier cells breadth first and returns `Reading::In` at the
FIRST cell centre decided within the band of the point and placed in
the trim. It certifies nothing about any other kept cell: a second
sheet of the same face (a fold back over the face, a span that comes
round) that also passes within the band of the point, inside the trim,
is never looked for. `spline_chart_placement` places a chart point a
meeting was found at, and the curve × chart isolation behind it sees a
second sheet only where the span crosses it.

So "the point lies on the face at this chart point" is a claim about
one sheet. Where the point is within the band of two in-trim sheets,
the face nearly touches itself there, and a reader that takes the
placement's chart point reads that sheet's data (its normal, its side)
as the face's.

Readers that rely on it:
- `topo::face_normal::spline_outward_normal_at`, through
  `boolean::vtxfac::classify_vertex_on_face`: the pierce reads the
  pierced spline face's outward normal at the chart point the sweep
  placed the pierce at (`reduce::spline_interior`'s meeting, or
  `contain::curved_face_placement_charted`'s in-trim foot), recorded
  in `reduce::ContactAcc::charts`.
- `boolean::reduce::spline_interior` and `vertex_on_curved_face_at`,
  which record a vertex-on-face contact on that placement.
- `boolean::vtxfac::held_reads`, through
  `contain::curved_face_placement_decided`, once it reads a spline wall
  that holds a pierce point
  (`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`).

## What it needs

A certificate that the in-trim foot is the only one within the band.
A narrow cone of normals does not give it: a helicoid-like stack of
sheets has normals all near one direction. The shape that does:
continue the cell walk past the first In, and require every kept cell
(hull within the band of the point, subdivided) to be either decided
past the band, or inside a chart neighbourhood of the foot over which
the sheet is certified a single graph (a curvature bound from
`offset_meters::chart_arm`'s cells over a neighbourhood whose spatial
extent is decided small against that radius, and whose boundary is
decided past the band of the point). Where neither holds, the read
refuses by name rather than placing the point In. It is one predicate
for the uniqueness fact: no reader adds a weaker check of its own
(`[one-way.readings]`).
