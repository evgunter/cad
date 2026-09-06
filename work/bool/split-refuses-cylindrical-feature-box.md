---
id: split-refuses-cylindrical-feature-box
kind: issue
title: topo::split refuses a box with one cylindrical feature in both orientations — and the second refusal reports CircularAxes where the closed form gives a-b = 0.049 m
status: open
opened: 2026-09-01
github: 1437
refs: [91]
---

## From GitHub issue 1437

Opened 2026-09-01; 0 comments.

`topo::split` refuses a boolean-result body carrying ONE cylindrical
feature, in both orientations I tried, with two different errors. The
second is the one worth reading: it says the section's semi-axes
coincide, and the closed form says they differ by 0.049 m.

Met building the demo tour's `cutaway`, which sections the project box
by a tilted plane. The box is all planes today; adding the round boss
its own module names as missing ("real enclosures want ROUND bosses and
drilled pilot holes; this chain does not attempt them, and says so")
makes the split refuse.

Plane: origin `(1.5, 1.0, 0.75)`, normal `n = (0.75, 0.1875, 1)`. The box
builds and validates in both variants below — genus 6 / V = 4.240943 and
V = 4.267271 respectively, tiers 1–2 + 3′, meshing, exporting. It is
only `split` that refuses.

## Variant A — cable gland, bored along +y through the near wall

`circle_split` r = 0.1875 at x = 1.5, z = 0.875, drilled through the
0.25-thick wall.

```
split of the boolean-result box:
  Join(Euler(Certification { error: ResidualExceeded { check: EndpointStart, sample: 0 } }))
```

This one is at least *plausibly* about conditioning: `|n̂·ŷ| = 0.148`, so
the section is a grazing cut whose ellipse has semi-major `r/0.148 =
1.264` on a 0.1875 bore — nearly nine times the radius. But the refusal
names an endpoint residual inside the join stage, not a conditioning
verdict, so a modeller who drills a cable gland where one goes gets an
internal-sounding certification error with nothing naming the condition.
Compare the split lane's own posture elsewhere, where ill-conditioned
input refuses with a diagnostic that says so (`SliverVertex`,
`CrossingEscalated`, both carrying `Indeterminate`).

## Variant B — round screw standoff, axis +z

`circle_split` r = 0.1875 at (2.0, 1.0), unioned onto the cavity floor
from z = 0.1875 to 0.875 (the inset-overlap pattern the square bosses
use). Sited so the plane crosses its wall at z = 0.375.

```
split of the boolean-result box:
  Join(Section { face: FaceKey(52v17), source: Carrier(CircularAxes) })
```

`EllipseInvalid::CircularAxes` means *"the semi-axes coincide
(|major − minor| ≤ ε): this configuration is a `Circle`, and D3's
one-kind-per-configuration discipline refuses to mint it as a degenerate
`Ellipse`"* (`geom/src/curves.rs`), decided by the
`ellipse_axes_distinct` trilean on the margin `major − minor`.

**That is not what the closed form predicts here.** For a cylinder of
radius `r` cut by a plane whose unit normal makes `cos θ` with the axis,
the section is `a = r/|cos θ|`, `b = r`. With `|n̂·ẑ| = 0.7911`:

```
a = 0.1875/0.7911 = 0.236997
b = 0.1875
a - b = 0.049497
```

0.049 m against a zero-band ε of 1e-9 — seven orders of magnitude clear
of the coincidence the error reports. So either the section is being
constructed from something other than this plane and this cylinder, or
the axes handed to `Curve3::ellipse` are not the ones the geometry
implies. I did not diagnose which, and I am not asserting it is a bug in
the constructor — only that the reported condition and the closed form
disagree by seven orders, which one of them has to answer for.

## Why it matters beyond the demo

`cutaway` exists to show an interior honestly (it replaced the void
box's translucency hack at #91). As it stands it can only section a body
made entirely of planes — so an enclosure gets a machinist's section
exactly until someone puts a round boss or a cable entry on it, which is
the first thing a real one has.

## Not claimed

That variant A's failure and variant B's failure share a cause. They
have different payloads from different stages, and I tried B *because* I
thought A was conditioning — B disproves that hypothesis for itself and
says nothing about A.

## Meanwhile

The demo keeps the box all-planar and `tiltedcut` keeps its own montage
cell; folding `tiltedcut` into `cutaway` was the point of the change and
is blocked on this. Nothing in the tour is contorted around it
(`memories/demo-purpose.md`) — the feature is simply not there, and this
issue is where it is recorded.

## Home

`work/bool/` — `topo::split` is the splitting lane, inside S-BOOL's territory glob `crates/topo/src/splitting/*` and its charter of operand gates that refuse legal inputs.
