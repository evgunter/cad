---
id: sphere-flux-arm-refuses-partial-bands
kind: issue
title: the sphere flux arm's coplanar premise leaves lune-family bodies outside tier 3
status: open
opened: 2026-09-03
refs: [1674]
---

## Measured (VERBS-RIMCAP PR-1, at its head)

`geom-brep`'s sphere flux arm computes a face's volume contribution
only when every boundary meridian lies on ONE great circle
(`props_band_coplanar`, the `Δu = π` premise — `props/curved.rs`,
`fn sphere`). Two lune-family faces fall outside it:

- the OPERAND lune's wall (a partial revolve of a half-disc): two
  meridian great circles on two different planes. Measured:
  `topo::validate_geometric(&lune)` =
  `Err([VolumeUncomputable { source: Face { source: NotIsoRectangle {
  what: "props_band_coplanar" } } }])` — the operand is tier-3 red
  today, before any offset is asked for;
- the lune CAVITY's wall, minted by the rim-construction capability:
  bounded by two GENERAL sphere circles (plane sections whose planes
  are parallel to the axis but off it) — neither rims nor meridians in
  `props_circle_axis_class`'s vocabulary, so the domain is not an
  iso-parameter rectangle at all (the `cross.step` class, D2 addendum
  row 2: valid input, lane not built).

The consequence: `shell`'s last act is `validate_geometric`, whose +V
invariant needs the volume, so a sphere-walled PARTIAL revolve cannot
leave `shell` even though the whole hollow now constructs (corners,
carriers, pcurves, containment all pass — pinned by `torax_axial`'s
`torax_the_sphere_lune_next_door_is_the_props_inventory`, which goes
red the day this lands and forces the flip to the true hollow row).

## What a fix owes

Two extensions of the sphere arm, each with S58-grade certification
arguments (the #649 lesson: a wrong `Δu` measures by that factor,
silently):

1. the rimless band with TWO non-coplanar meridians: `Δu` derived from
   the arcs' own azimuths and the traversal's side, not assumed `π`;
2. the general-circle boundary (the cavity wall): outside the
   iso-rectangle inventory — either a new closed form (Gauss–Bonnet
   over small-circle arcs) or the certified-quadrature lane, which is
   a design conversation, not a patch.

The closed-form volume the acceptance would pin is already derived and
parked in `torax_axial::torax_the_sphere_lune_rim_solves_in_closed_form`'s
docs: `V = (2/3)[R³·atan(RQ/a²) − a(3R²−a²)·atan(Q/a) + a²Q]`,
`Q = √(R²−2a²)`, `R = r − t`, `a = t`.

## Home

Filed from the VERBS-RIMCAP PR-1 lane as the declared schedule for the
spec's shell-level acceptance (the hollow row), which this wall blocks
from outside the unit's scope; adjudication decides whether it is
funded as its own props unit.

**Re-homed to PROPS** (2026-09-04, at CURVED's opening, Ev's in-chat
"put them wherever you see fit"): the flip is a props unit on
`geom-brep/src/props/*` — S-CERT's ground until PROPS inherits it — and
PROPS holds its items from opening exactly as CURVED held S-MATE's.
