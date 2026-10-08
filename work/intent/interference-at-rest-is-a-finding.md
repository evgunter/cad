---
id: interference-at-rest-is-a-finding
kind: issue
title: D10 stage 5 PR B: interference between world copies is its own finding, quiet under a one-sided gap assertion at its site
status: parked
opened: 2026-10-08
priority: P0
cost: H
needs_ev: true
blocked_on: [the-product-is-an-explicit-list, measure-is-an-operation, select-defines-face-and-edge-variables]
---


INTENT stage 5, PR B. Spec: `docs/INTENT-STAGE5-SPEC.md` §3.

D10: at rest, "interference is a finding of its own; neither refuses
where the census has a lane". Today the census's containment arm
(`crates/topo/src/census.rs:5634`) pushes
`ValidationError::InstanceInterference`, and A5's gate attributes it
`Unattributed` (`crates/editor-core/src/assembly.rs:1791`) and refuses
`AssemblyError::AtRest`. A transverse pierce between two copies refuses
as `UndeclaredContact { EdgeFacePierce }`.

B partitions those verdicts out of the gate's refusal into an
`InterferenceFinding` per overlap. The site is per FORK-S5-2 (recommended:
a connected component of the two copies' intersection, named by the
faces bounding it). B lands the quieting rule's one home
(`checks/at_rest.rs`) with its interference half: a holding `Gap`
assertion over the finding's faces of the two world copies, whose
admitted set is negative (FORK-S5-1, FORK-S5-3). The kernel is
unchanged except for one typed `SameSide` field. ASSEMBLY A5
*Interference.*, topo C6's invariant and MATE-4B's "EdgeFacePierce stays
categorical" are re-worded at the at-rest door.

It needs no stage-3 or stage-4 work. It needs stage 2's world copies (C),
single-primitive measures (D) and selections (E) to state a site.

Design forks open: FORK-S5-1, FORK-S5-2 and FORK-S5-3 (spec §11).

FORK-S5-1, S5-2 and S5-3 were weighed as one fork (FORK-S5Q, fork log
row 98) and went to Ev in an `[ev]` PR. The converged answer differs
from the spec's recommendation in three places, and this unit builds
on it provisionally: `Distance = 0` quiets nothing (only `Gap`, over an
opposed face pair); an interference is quiet only when every face
bounding the overlap lies between the carriers of one asserted pair;
an overlap the kernel cannot intersect is a loud interference that
nothing quiets, not a could-not-look finding.

After Ev's comments on #4319 (rounds 3 and 4): an interference is quiet
only when the asserted pair's own faces bound the overlap, so a second
lug bore on the same carrier needs its own assertion. Every other new
overlap is loud by default. Regions of one carrier pair are one site.
Ev's witness point is deferred to a future pointed local `Gap` arm; it
is not a slot on `Assert`. The exact-cell contact rule (stage 5 C's
half) needs stage 4 to record a face-on-face planar contact as the face
pair: the census records planar contact at vertex and edge level, and no
`Vertex` selection exists, so without that lift a planar contact could
never be quieted.
