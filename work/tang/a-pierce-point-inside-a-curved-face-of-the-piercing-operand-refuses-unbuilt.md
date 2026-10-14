---
id: a-pierce-point-inside-a-curved-face-of-the-piercing-operand-refuses-unbuilt
kind: issue
title: A pierce point inside a curved face of the piercing operand, or a plane face holding one over a curved pierced face, refuses unbuilt
status: closed
branch: tang/pierce-held-built
opened: 2026-10-13
closed: 2026-10-14
priority: P3
cost: M
refs: [an-edge-lying-in-a-face-at-a-pierce-refuses-ring-meets-ring]
---

## What

`vtxfac::faces_holding_pierce` (`crates/topo/src/boolean/vtxfac.rs`)
gives a ring vertex to a face of the piercing operand that holds a
pierce point inside it, so the face's section with the pierced face
passes a vertex there. It builds that vertex on two planes only, and
refuses `BooleanError::HeldPierceUnbuilt` before anything is built in
three cases:

- the point lies inside a curved face of the piercing operand
  (`placed: true`);
- a curved face's box holds the point but its trim cannot say whether
  the face does (`placed: false`): a ringed wall, a NURBS face, an
  outline the chart door does not read;
- the face holding the point is a plane but the pierced face is curved.

Before that function, all three built a body whose section passed the
pierce with no vertex. Only the result's tier-3 gate caught it, if
anything did.

## The shape to give

All three want what the plane case has. The face takes a ring vertex at
the point. Its one Out run reads the face's half above the pierced face,
with germs along the two faces' section through the point. On a curved
face, that section is a conic or a ruling, not a line. The germs are its
tangents (`pierce_germ_dir` reads the tangent plane already), and the
run is levered for the face's curvature as `pierce_side` levers it.

No row reaches either case yet. A witness: a cylinder standing on a
plate, with a prism whose edge lies along one of the cylinder's
rulings, folded so the cylinder and the prism are one operand when the
plate joins them.

## Closed (2026-10-14, TANG)

Rows: `crates/sweep/tests/a_pierce_held_in_a_curved_face.rs`. Each of
the three refused `HeldPierceUnbuilt` on main (`9fda94407`).

- **The item's witness** is the rod with a square prism whose edge is
  one of its rulings, joined, against a plate. On main it refused
  `placed: false`, not `placed: true`. The wall's outline read as
  unsupported because the pierce's own strut already stood in its
  loop (`solid_contain::wall_outline`'s chain check). A null half is
  zero-length, so the chain now steps past it, and the trim places the
  point In.
- **A plane face over a curved pierced face**: two prisms touching
  along a line, joined, against a lying cylinder.
- **`placed: false`**: the joined pair pocketed beside the point, so
  the wall carries a ring the cylinder trim does not read.

`faces_holding_pierce` now builds the curved cases as it builds the
plane one:

- a ring vertex at the point;
- one Out run, with germs `±n × n_p` across the two faces' tangent
  planes;
- the run's half charged for both faces' bends at the pierce's arm
  (`sectors::sagitta_charge`, shared with `read_side`).

A trim that cannot say still refuses `HeldPierceUnbuilt` typed. Its
recourse now names cutting the face after the join, which builds.

The first two build:

- every op, either operand first;
- both join orders and three poses of the ruling;
- every member order of the fold, carried.

Each passes tier 3 and tier 3′, matches its closed-form volume, and
matches the material at the probes, at all three ε.

A curved face over a curved pierced face is not reached. The rod alone
refuses against a lying cylinder (`GermFrameUnsupported`) and against
a coaxial ball (`CurvedBooleanUnsupported`).
