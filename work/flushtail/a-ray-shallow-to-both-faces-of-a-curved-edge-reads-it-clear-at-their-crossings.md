---
id: a-ray-shallow-to-both-faces-of-a-curved-edge-reads-it-clear-at-their-crossings
kind: issue
title: A ray shallow to a face can pass a curved edge of it within the band with every crossing definitely off it, and a met edge's half then reads nothing where it would count right
status: open
opened: 2026-10-25
priority: P3
cost: M
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
---


Filed by the curved edge-meeting ray
(`a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim`, PR 5697),
and rewritten on its fix pass.

**What is read.** `boolean::solid_contain::curved_edges::passes` reads an
edge from each carrier crossing of its two faces. A crossing definitely
off the edge's carrier (`Curve3::clearance` past the band) reads the edge
clear there; only one that is not is carried to the ray's closest
approach by alternate feet. Carrying every crossing does not work: on a
circle's axis a foot has no place, and at `Interval` the feet widen
until the on-carrier reading escalates
(`pole_slit_window::a_slit_dome_answers_containment_at_interval` refused
`VolumeUncertified` on the build that carried every crossing).

**What it misses.** A ray at incidence `θ` to a face, passing `δ` from
an edge of it, crosses the face's carrier about `δ/sin θ` from the edge.
The ray can pass an edge in band with every crossing of that edge's
faces definitely off its carrier. One of PR 5697's reviews demonstrated
this on the kinked bore (`crates/sweep/tests/curved_edge_corpus.rs`,
`kinked_bore`). The ray is shallow to the face `x = 1` and passes the
bore's mouth on it in band, steep to the bore's wall. The mouth was
met: its span starts at the ray's `t = 3.0`. The ray then passes the
kink edge, which read clear at every crossing of its faces, and crosses
the plane `x = 1` at `X`, beyond the kink edge and outside the face.

On the PR's first head, the mouth stood for that plane crossing: the
hits on 3c24a8b1 show one count at `t ≈ 2.99999`, its span running from
the mouth's passage to `X` (`t ≈ 3.2785`). It counted
`(s_T + s_W)/2 = 0` where the truth is ±1, and 140 of 3042 rays read
the wrong side. This row's earlier text said
"a lost read, never a wrong one", which was false.

**The fix pass's rule.** A met edge stands for a face's crossing only
where the crossing lies near the passage
(`side_at_infinity_met_edge_represents` /
`bool_point_in_solid_met_edge_represents`, not past the band from the
passage's span), or where the face definitely holds it. This applies on a face not every
edge of which is a line edge read exactly; on a face every edge of which
is (planes only, `edge_meets`), every edge the ray passes in band is
met. Otherwise the edge reads nothing there.
- Measured on the kinked bore, the scan reads 0 wrong at ε 1e-9, 1e-6
  and 1e-12 (`a_ray_shallow_to_a_face_past_its_curved_edge_reads_right`).
- **What it costs.** The rule also reads nothing where the half would
  have counted right. A ray nearly along a cylinder's wall passes the
  rim and crosses the cap's plane outside the disc, across the rim
  itself.
  - At Kε/3 and Kε into each face, the cylinder's aimed rays read 10 of
    24 one way out, against 16 without the rule.
  - Over the curved probes' bodies, nominal bodies read 42–88% of those
    rays, against 12–46% on main.

**What would close it.** Either of:
- a reading of the ray's distance from a curved edge that is a lower
  bound over the whole span, not at a point (a line against a circle's
  span is a quartic; `geom_brep::curve_pair::meet`'s line–conic closed
  form reads meetings, not distances); or
- a test of whether a face's crossing lies across the met edge itself,
  not across another edge of the face, which would let the half stand
  for a crossing outside the face across the edge it was met at.
