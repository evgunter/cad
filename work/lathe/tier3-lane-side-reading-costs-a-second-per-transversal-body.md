---
id: tier3-lane-side-reading-costs-a-second-per-transversal-body
kind: issue
title: Tier 3's lane side reading more than doubles validate_geometric on the transversal cylinder/ball bodies
status: open
opened: 2026-11-25
priority: P2
cost: M
---

Check 6's curved arm reads the side of a cylinder or sphere face whose
trims no closed form parses through the rotational quadrature lane
(`crates/topo/src/validate.rs:8446`, the `lane_side` closure, reaching
`crates/geom-brep/src/props/quad/rotational.rs:1975`
`rotational_material_sign`). That reading is the definition of the face's
side, and it is what closes
`a-flipped-one-band-sphere-face-met-only-transversally-finishes`. But it
assembles and enters the face a second time: check 7 assembles the same
face and enters the same chain for the volume, and nothing shares the two.

Measured in release, median of 7 `validate_geometric` calls on the honest
transversal bodies of `crates/sweep/tests/verbs_cylsph_opening.rs`
(cylinder r=1, z in [-2,2]; ball r=1.5 about (0.6,0,0)), at the default
eps, lane reading on vs off:

| body | on | off |
|---|---|---|
| union | 2835 ms | 1125 ms |
| intersection | 2869 ms | 1148 ms |
| ball - cylinder | 2758 ms | 1166 ms |

That is about 1.6 s per body, 2.5x. It scales with eps, at about 0.25 s
at 1e-6, 1.4 s at 1e-9 and 8 s at 1e-12 per validation (the review's
measurement). The boolean door's result gate validates the built body,
so every boolean producing such a body pays it too. Bodies with no
unparsed cylinder or sphere trim pay nothing.

Where it goes (the review's profile of the side reading):
- `edge_entry` (`rotational.rs:1235`), each edge's chart entry and
  schedule: about 55%.
- `rotational_face` (`crates/topo/src/props/quad_lane.rs:559`), lifting
  the face's loops: about 27%.
- the first round's terms: about 18%.

The windings and connectors are not where it goes. The reading already
stops at the first round whose mean width decides the sign.

Candidates:
- assemble the face and its edge entries once, and hand them to both check
  6's side and check 7's volume;
- take the pole count's early returns (the one-pole winding) before every
  `edge_entry` is read. This needs the windings without the entries'
  schedules, which they are computed from today.
