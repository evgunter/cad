---
id: set-face-surface-passes-a-swap-off-the-faces-own-boundary
kind: issue
title: set_face_surface passes a swap that moves a face off its own boundary, which only tier 3 reports
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [set-face-surface-hands-the-caller-an-ordering-obligation-in-prose, mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move]
---

## What

Found by the review of PR 3580 (its C1 probe). `Body::set_face_surface`
(`crates/topo/src/attach.rs`) asks one question of a swap: whether an
edge described against the face's old key is left naming a surface
neither of its faces wears. That is exact key adjacency, so the door
takes no band. Whether the face's boundary lies on the new surface is a
band question, and the door does not ask it.

**Witness.** The unit brick (`test_support_fixtures::brick`) with a
square membrane planted in its top cap
(`test_support_fixtures::plant_ring_face`, rim at z = 1 from
(0.25, 0.25) to (0.75, 0.75)), every membrane edge re-described as an
image in the cap's chart (`EdgeDescriptionSpec::chart(cap key)`). The
body is valid at rest. Then `set_face_surface(membrane, …)`:

- `FaceSurface::New` of the cap's plane raised four units, or
- `FaceSurface::Shared` of the front face's key (the plane y = 0),

returns `Ok`. No edge is stranded, since every membrane edge names the
cap's key and the cap still wears it. Tier 3 then reports
`PlanarFaceResidual` and `PlanarBoundaryResidual` on the membrane, and
no `DescriptionNotAdjacent`.

The describing door, `Body::set_face_surfaces_describing`, refuses both
swaps (`EulerOpError::RechartOffBoundary`), because it takes a band and
checks every moved face's vertices and edge samples against a new plane.
That refusal is pinned by
`attach::tests::a_move_off_the_faces_own_boundary_is_refused_by_the_describing_door`,
which also pins the keys-only door's `Ok` and tier 3's verdict on this
witness.

## Why it is a row and not a fix

The keys-only door has no band to decide a residual with, and giving it
one makes it the describing door. The door's docs state what it does not
check. The open question is whether the keys-only door should keep
accepting a swap it cannot vouch for:

- **Refuse typed wherever the chart changes and a face edge does not
  name the face's new key** (no band needed): the swap is then only
  legal where every boundary edge names the new chart, which would send
  the witness above to the describing door. Measure first: how many
  production callers swap a face whose edges all name a neighbour's key.
- **Keep it**, and say in the door's docs that a boundary it cannot see
  is tier 3's at rest. That is the prose obligation S93 calls a defect,
  so this needs a ruling rather than a default.

A curved new chart is not checked by either door, nor by tier 3 at rest
(`validate_geometric`'s not-yet-checked list, #638).

## Ruled (orchestrator, 2026-09-30): refuse, keys-only

Of the two options above, only the refusal keeps the ratified decisions
as they stand:

- S93 (closed by PR 3161, `325a4daadc`) calls a prose-held caller
  obligation a defect. "Keep it and say so in the docs" is exactly
  that.
- Ev's PR 2527 ruling on the kill family already gives the pattern for a
  change a keys-only door cannot vouch for. The keys-only door refuses
  it typed, before mutating. The describing sibling, which takes a band,
  certifies it.

So `set_face_surface` refuses typed, before mutating, wherever a moved
face keeps a certified boundary edge whose description names no key
the face wears after the swap. The door cannot vouch for such an edge
without a band, and `set_face_surfaces_describing` is the door that can.
Scaffold edges, which carry no certificate, vouch for nothing and
strand nothing. The lane derives the exact condition from tier 3's
residual checks and states it in the door's docs. No band is added.

No `[ev]` question (the PR 3156 lesson). The lane measures first. If a
production caller relies on the keys-only door for a swap this would
refuse, and the describing door cannot serve it, the lane stops and
reports, because that would be a real fork.
