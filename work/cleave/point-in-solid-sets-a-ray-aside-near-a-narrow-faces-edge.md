---
id: point-in-solid-sets-a-ray-aside-near-a-narrow-faces-edge
kind: issue
title: Point containment's ray walk sets a crossing within Kε of a face boundary aside, so a needle operand thinner than 2Kε loses its rays
status: closed
opened: 2026-10-14
priority: P2
cost: M
closed: 2026-10-19
---

Found by FLUSH's needle-class design fork (fork-log row 153, PR 5119;
the FLUSH row `the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite`).

## What

`boolean::solid_contain`'s in-face walk (`point_in_loop`) refuses a
ray crossing that lands within Kε of a loop boundary. A face narrower
than 2Kε has no point more than Kε from its boundary, so every crossing
of it is in band. The shell-role reader on PR 5119 sets such rays aside
(measured: `brick (0,1)×(0,w)×(0,w)` at w 10.5–21ε, ε 1e-9 and 1e-6).
`point_in_solid`, the boolean's containment fallback, walks the same
in-face test, so a needle operand loses its rays there too (read, not
measured).

## Owed

The fork's converged design decides each edge a ray passes once, for
both faces the edge bounds, with one tie-break (a watertight cast), so
no crossing is in band of a boundary. A designer measured it reading
all 20 needle cases on planar faces (`flush/needle-design-x` at
`feb5b660e`, `ndx_side`). If Ev rules it on PR 5119, the needle unit's
ray-walk change serves `point_in_solid` as well; measure a needle
operand through `point_in_solid` there.

## Resolution

Closed by FLUSH's edge-meeting ray
(`work/flush/a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face.md`),
2026-10-19. `point_in_solid`'s schedule rays and the side reader's aimed
rays read through one walk, `boolean::solid_contain::ray_crossings`,
which decides each line edge between two planar faces once, for both
faces: a ray within the band of an edge meets it, half a crossing for
each face, counted (D10, Booleans).

Measured with `crates/topo/tests/edge_meeting_ray_probes.rs`
`probe_needles_point_in_solid` (ignored), on `brick (0,1)×(0,w)×(0,w)` at
ε 1e-9, against main:
- every answer is the same: `In` at the middle of the 21ε and 30ε
  needles, `Out` beside, above, off the corner, far off and beyond every
  needle, and `Escalated` on the plane at points no more than Kε from a
  face, as the boundary pre-pass decides;
- at 21ε and 30ε, the side at infinity a missed ray reads now comes from
  an aimed ray, not the volume, and the in-band verdicts a query records
  fall from 30 to 1 at 21ε, and from 6 to 0 at 30ε;
- at 10.5ε to 15ε, a ray through the needle meets a wedge (a face it
  passes within the band of two edges of), which D10 rules reads
  nothing. The volume reads the side there as on main. No ray through a
  needle narrower than about 2Kε reads, by the ruling, not by a defect.
