---
id: a-ray-shallow-to-both-faces-of-a-curved-edge-reads-it-clear-at-their-crossings
kind: issue
title: A ray shallow to both faces of a curved edge can pass within its band with both faces' crossings definitely off it, and is set aside
status: open
opened: 2026-10-25
priority: P3
cost: M
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
---


Filed by the curved edge-meeting ray
(`a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim`).

**What is read.** `boolean::solid_contain::curved_edges::passes` reads an
edge from each carrier crossing of its two faces. A crossing definitely
off the edge's carrier (its closed-form lower bound, `Curve3::clearance`,
past the band) reads the edge clear there; only one that is not is
carried to the ray's closest approach by alternate feet. A crossing far
off the carrier must not be carried: on a circle's axis its foot has no
place, and at `Interval` the alternate feet then widen until the
on-carrier reading escalates (`pole_slit_window`'s
`a_slit_dome_answers_containment_at_interval` refused `VolumeUncertified`
on the build that carried every crossing).

**What it misses.** A ray at incidence `θ` to a face, passing `δ` from
an edge of it, crosses the face's carrier about `δ/sin θ` from the edge.
Where both faces of a curved edge are crossed shallow enough that both
crossings lie definitely past the band (`δ/sin θ > 10ε` at K = 10, with
`δ` in band), the edge reads clear, and each face's own trim then reads
its crossing, in band of the edge or not:
- in band, it sets the ray aside: a lost read, never a wrong one;
- a face that a met edge stands for is read again at its other edges
  from where the ray passes the met one (`curved_edges::narrow`), so a
  face narrower than the band there is still a wedge.

No fixture of the curved probes
(`crates/sweep/tests/curved_edge_ray_probes.rs`) measured a ray lost
this way; an aimed ray meets its aimed face at 10° off the normal. A
schedule ray (`point_in_solid`) can.

**What would close it.** A reading of the ray's distance from a curved
edge that is a lower bound over the whole span, not at a point (a line
against a circle's span is a quartic; `geom_brep::curve_pair::meet`'s
line–conic closed form reads meetings, not distances), or a seed that
is not a carrier crossing.
