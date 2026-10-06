---
id: d4-placement-carrier-to-locus-within-eps
kind: issue
title: D4 placement: a certified edge's carrier may lie up to 2·residual/margin (to 10^4 ε) from the locus, not within ε of it
status: open
opened: 2026-10-04
priority: P2
cost: H
design: true
refs: [limb3-at-rest-proves-the-graph-not-the-arc]
---


## Found (PR 4012's design, both designers; filed by the limb-3 lane, 2026-10-04)

C2's limbs 1 and 2 bound the carrier's residual against each surface
(`|S(P(t)) − C(t)|`, `crates/geom-brep/README.md` ~284), and limb 3
proves the tube's chain holds one arc spanning the carrier. Nothing
bounds the carrier's distance to that arc. Near a tangential crossing it
is about `2·residual / margin`, the margin being limb 3's transversality
`sin θ`. At the lane's smallest certified margins that reaches about
10⁴ ε, while D4 reads an edge's placement as within ε of the locus.

Executed on PR 4012 (`crates/geom-brep/tests/ssi_limb3_one_arc.rs`,
probed, not a row): `z = 0.01x + h(y)`, `h = −2ε·y(1 − y)`, against
`z = 0`, with the carrier along `x = 0`. The carrier lies within ε of
both surfaces, and the one arc lies about 50ε inside. The edge
certifies.

## Design question

Should C2 state the carrier-to-locus distance, as limb 2's displacement
divided by the margin and refused past ε (or past a named multiple of
ε)? Or should D4 read an edge's placement as "within residual of both
surfaces"? Either way one of the two texts moves, so this is a design
fork for Ev.
