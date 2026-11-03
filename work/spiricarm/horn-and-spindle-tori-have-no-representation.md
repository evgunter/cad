---
id: horn-and-spindle-tori-have-no-representation
kind: issue
title: Surface::Torus represents only ring tori (R > r > 0); horn and spindle tori, which ACIS-family kernels represent, have none
status: open
opened: 2026-10-14
priority: P0
cost: X
points: 20
branch: spiricarm/torus-axis-ev
pr: 6464
---


Raised by Ev on `[ev]` PR 5113 (2026-10-14): "it seems plausible that we do want to represent these but it really isn't a priority".

## What is true today

`Surface::Torus` represents ring tori only.
- `geom::ring_torus` decides `R − r > 0` on the band (`ring_torus_convention`), and `geom::torus_tube` decides `r > 0`.
- `geom_brep::torus_convention`'s doc: "A torus outside it (no tube, a horn, a spindle) has no representation".
- DESIGN D3 (the Booleans conventions): "`UnsupportedToroid` is permanent: a D3 ring-torus boundary — spindle tori have no representation."
- A horn torus (`R = r`) sits on that boundary and refuses as `TorusConvention::Ring`.

## Elsewhere

- Open CASCADE's `Geom_ToroidalSurface` and Autodesk Inventor refuse `r ≥ R`.
- The ACIS family (SolidWorks' API, the ODA kernel) classifies doughnut, apple (`0 < R ≤ r`), lemon (`R ≤ 0 < R + r`) and vortex tori, so the surface exists there, self-intersecting and with apex points on the axis.

## What representing them would touch

- The convention's readers: the plane×torus section, the offset door, tier 3's `DegenerateTorus`, the trim door's meridian frame, the regular outward normal, the torus chart (the azimuth stretch floor is `R − r`, so a horn's chart degenerates at the axis, as a sphere's does at its poles).
- DESIGN D3's "permanent" sentence, which is ratified text: changing it is Ev's call.

No design work is scheduled; this records the question.

## Re-banded P0 (Ev, 2026-10-31)

Hit in the viewer: a football, drawn as a shallow arc closed by its chord and revolved about the chord, refuses at the revolve ("the arc at loop 0 segment 0 would sweep a horn or spindle torus (its circle reaches the axis), which is not supported"). Any arc revolved about the line through its own ends sweeps part of a spindle torus, so every lemon- or apple-shaped revolve refuses.

The whole spindle surface self-intersects, but the face a revolve makes is one of its two pieces: the lemon (the minor arc, between the apexes) or the apple (the major arc). Each piece alone is a regular surface away from its two apexes on the axis, as a sphere is away from its poles. A representation could be the piece rather than the whole surface; that choice is part of the design.

The refusal reads the arc's whole CIRCLE, not the arc. So the football with conical ends (the arc's ends lifted off the axis and joined to it by straight segments) still refuses, although its arc never reaches the axis. Its face is a band of the lemon away from both apexes, and nothing about it is singular. Only the support surface, a spindle torus, has no representation. A representation of the piece covers this case with no apex at all.

The refusal's recourse, "keep the arc's circle clear of the axis", cannot be met by an arc revolved about its own chord. That wording should change with whatever this decides.

Moved from `work/issues/` to `work/spiricarm/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Weighed (2026-11-03)

Two designers weighed it (fork `a-torus-reaches-its-axis`, `docs/DESIGN-FORK-LOG.md` row 252) and agreed after four rounds: a separate analytic kind for one sheet of a spindle or horn torus, with a signed major radius, sharing a revolved-circle core with `Surface::Torus`, which stays the ring. The `[ev]` PR on `spiricarm/torus-axis-ev` carries the question; the row is priced once Ev rules.

## Ruled (Ev, PR 6464, 2026-11-03)

Ev approved the agreed recommendation: a new analytic kind `Surface::Spindle`, one sheet of a spindle or horn torus with a signed major radius, sharing a revolved-circle core with `Surface::Torus`, which stays the ring. The horn is H1 (the sheet's closed end, `v* = π`, one cusp). [surfaces] gains the locus sentence. The revolve reads the arc's interior, and the signed centre distance picks the kind (`crates/topo/README.md`'s structural conventions).

**Priced X, 20 points.** It is a new `Surface` variant: every exhaustive match gets a `Spindle` arm, and an audit covers every non-exhaustive kind test (`matches!`, `if let Surface::Torus`). It has two stages, either of which may be cut into several units:
1. The sheet away from its apexes: datums, the shared core, the revolve's arc classification, the readers' window forms, typed "no lane yet" refusals for the ring-only closed forms. This lands the football with conical ends.
2. The apex as a pole (`Singularity`, `Chart::poles`, mesh's pole fans, the pole form, tier 3's apex check). This lands the football.

The consumers (offset's `TorusRing`, blend's `SpineIrregular`, STEP's `DEGENERATE_TOROIDAL_SURFACE`) are rows of their own once stage 1 lands.
