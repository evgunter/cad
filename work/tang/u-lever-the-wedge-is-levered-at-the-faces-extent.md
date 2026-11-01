---
id: u-lever-the-wedge-is-levered-at-the-faces-extent
kind: unit
title: U-lever: the wedge is levered at min(the faces' curvature radii, the farther face's extent), one home for check 4, certification and must-carry
status: closed
opened: 2026-10-15
closed: 2026-10-16
priority: P1
cost: M
branch: tang/u-lever-face-extent
parent: pinch-carrying-machinery-valence-4
pr: 5204
---


Approved by Ev on PR 5193 (the U2 section's "An edge near a node" bullet
of `pinch-carrying-machinery-valence-4`). Check 4, `Intersection`
certification and the boolean's must-carry read meter the wedge between
an edge's two faces at min(the faces' curvature radii, the edge's own
extent), so a 2δ arc at dihedral ≈δ reads ≈δ². The lever becomes
min(the faces' curvature radii, the farther face's extent), derived
from the body, with one home for all three readers; never the edge's
own extent.

Acceptance: a digest fence over topo, sweep and editor-core at ε 1e-9,
1e-6 and 1e-12 against main, every moved key accounted for; one short,
shallow, non-node edge between large faces at rest that reads a corner
under the new lever and did not before; the pinch near-node poses'
rest-side refusals measured.

## Closed

Built in PR 5204, merged 2026-10-16 as 75cf96347d. The wedge between an
edge's two faces is levered through one home, `topo::wedge_reach`, at
min(the faces' curvature radii, the farther face's extent). It serves
check 4, `Intersection` certification, the must-carry rule,
`edge_join::joinable`, the shared-rim routing and the fillet battery's
convexity. Residue filed from its reviews:

- `the-arms-the-faces-lever-left-without-a-row`
- `the-wedge-reach-counts-a-faces-run-along-its-edge`
- `the-wedge-reach-reads-each-faces-vertices-once-per-edge`
- `a-rigid-move-levers-its-certificate-at-the-source-bodys-reach`
- `the-census-reads-a-shared-edge-face-pair-at-the-shorter-face`
- `the-onpair-re-ask-doors-have-no-under-determined-fixture`
- the pcert poleband item, which carries the open lever question
  (farther vs nearer face): `the-poleband-twins-pcurve-re-mint-escalates-at-the-fine-band`

