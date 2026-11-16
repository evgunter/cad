---
id: plane-distance-reads-a-face-at-its-carrier-origin-a-contingent-point
kind: issue
title: The PlaneDistance selector reads a face at its carrier origin, a point the construction is free to choose, so its answers follow a convention and not the face
status: open
opened: 2026-11-15
priority: P1
cost: M
design: true
refs: [6972]
---

Filed by the BOXFINISH orchestrator from PR 6972's two reviews
(2026-11-15). Line numbers are at that PR's reviewed head `032ac185`.

**What is there.** `GeomPred::PlaneDistance` measures how far a face
lies from a plane by reading ONE point of the face: its carrier's
origin (`crates/editor-core/src/names/geompred.rs:145-155`,
`crates/editor-core/src/names/interrogate.rs:374-381`, `entity_point`).
Its doc says the point "need not lie on the entity".

**Why that is a defect.** A carrier's origin is whatever the
construction chose. For a planar face any point of the plane is a
correct origin, so the selector's answer for a face not parallel to
the query plane depends on a convention, not on the face
(`docs/DESIGN.md` `[intrinsic.contingent]`).

**Measured.** PR 6972 moved a swept line wall's origin from the middle
of the wall (a fitted centroid) to its segment's start vertex. On a 1 m
cube "faces 0.5 above the floor" stopped finding the four walls, and
"faces more than 0.9 behind the top" went from the floor to the floor
plus the walls; the answer also came to depend on which end a segment
starts at. BOXFINISH ruled the wall's origin back to the middle of the
wall so that no shipped answer moves, which keeps the convention the
selector was leaning on and does not cure it. A cap's origin did move
(to the sketch frame's origin, ratified on PR 6936); a parallel-plane
query does not see that, a tilted one would.

**The question.** What should "this face is at distance d from that
plane" mean for a face that is not a point: the face's extent along the
plane's normal (an interval, then which end, or both), a point the face
intrinsically has (which one), or a refusal where the answer is not
single-valued? And is the mate and pose side sound as it is (reviewers
read `eval/wire/pose.rs:666-674` as using the normal and the plane's
offset only)?

**Not checked:** which other selectors or measures read `entity_point`.
