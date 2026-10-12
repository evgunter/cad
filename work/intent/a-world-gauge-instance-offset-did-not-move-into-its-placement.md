---
id: a-world-gauge-instance-offset-did-not-move-into-its-placement
kind: issue
title: Q9's migration exception is unbuilt: a world-gauge root instance keeps its offset on the instance and is placed at the identity, so the world pose has three homes
status: open
opened: 2026-10-08
refs: [a-placement-is-the-bundle-of-mates]
---

Found by the review of INTENT stage 2 C (PR #4359, MINOR-3); disclosed
in that PR, not built there.

Spec §4's migration note (Q9, Ev's residue 3 of #4220) moved a root
`InstantiatePart` on the world gauge with no placing mate: its offset
was to become the placement's pose, the instance sitting at the empty
offset. C keeps every offset on the instance and places it at the
identity, so test 6's digests hold either way, but the world pose of
such an instance now has three homes: the instance's offset, a gauge
placement, and the `PlaceInWorld` pose. The demo tour's assembly
scene authors the offsets this would move (`SetOffset` rows in
`demos/tour/src/assembly.rs`).

Not built in C because the orchestrator ruled the `PlaceInWorld` pose
a transient interim that only the gather and export read, and stage 3
replaces it: moving offsets into it would grow the home that retires.
Stage 3's placement redesign (`a-placement-is-the-bundle-of-mates`)
decides where an instance's world pose lives, and this row is settled
there.
