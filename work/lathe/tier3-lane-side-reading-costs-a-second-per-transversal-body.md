---
id: tier3-lane-side-reading-costs-a-second-per-transversal-body
kind: issue
title: Tier 3's lane side reading more than doubles validate_geometric on the transversal cylinder/ball bodies
status: open
opened: 2026-11-25
---


Check 6's curved arm reads the side of a cylinder or sphere face whose
trims no closed form parses through the rotational quadrature lane
(`crates/topo/src/validate.rs:8440`, the `lane_side` closure, reaching
`crates/geom-brep/src/props/quad/rotational.rs:1939`
`rotational_material_sign`). That is the definition of the face's side,
and it is what closes `a-flipped-one-band-sphere-face-met-only-transversally-finishes`,
but it integrates the face's Green form a second time: check 7 integrates
the same chain for the volume, and nothing shares the two.

Measured in release, median of 7 `validate_geometric` calls on the honest
transversal bodies of `crates/sweep/tests/verbs_cylsph_opening.rs`
(cylinder r=1, z in [-2,2]; ball r=1.5 about (0.6,0,0)), lane reading on
vs off:

| body | on | off |
|---|---|---|
| union | 2835 ms | 1125 ms |
| intersection | 2869 ms | 1148 ms |
| ball - cylinder | 2758 ms | 1166 ms |

About 1.6 s per body, 2.5x. Not yet profiled. The reading already stops
at the first round whose mean width decides the sign, so the cost is
likely that first round itself: building the face's chain, its connectors
and its windings, which check 7 then builds again for the volume. Sharing
the face's built chain with check 7, or a cheaper first window for a sign
alone, are the candidates. Bodies with no unparsed curved trim pay
nothing.
