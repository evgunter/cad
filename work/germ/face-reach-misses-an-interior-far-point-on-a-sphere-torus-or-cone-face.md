---
id: face-reach-misses-an-interior-far-point-on-a-sphere-torus-or-cone-face
kind: issue
title: face_extent reads only a face's boundary, which misses a sphere's or torus's far side or a cone's apex inside the face
status: open
opened: 2026-10-08
priority: P3
cost: M
---

Found by TANG's span-bounded conic reach.

## What

`splitting::rules::face_extent` (and `face_reach_from`) is the farthest the face's BOUNDARY stands from a point: its vertices and each certified edge. On a plane or a cylinder that is the face's own reach, because the line or ruling through any interior point meets the boundary both ways. On other surfaces the farthest point can lie inside the face:
- **sphere**: the far side, `centre + R·(centre − p)/|centre − p|`, when it lies in the face, for example a sphere with a small hole read from the hole's rim. The rim's whole-turn lever, `|c − p| + ρ`, is small; the face reaches `2R`.
- **torus**: the far side's critical points.
- **cone**: the apex, when it lies inside the face rather than at a vertex.

The span-bounded reach keeps the whole-turn edge lever on these faces, because there it sometimes covered the interior point (a great-circle arc reads `2R`). It does not always, so the lever can under-state the face.

The face measure's callers:
- the split's `split_sector_coplanar`, `tangent_sector_osculation` and `enters_material`;
- chord_join's cone lane (`chord_join::section_reach`).

## The shape of a fix

On a sphere or torus face, fold in the carrier's far point from `p` where it lies in the face, or bound by the carrier's ball where that cannot be decided. On a cone face, fold in the apex distance where the apex lies in the face. Each needs a point-in-face reading or a sound proxy for it.

## Evidence: a worked under-reach (BOXES, 2026-11-02)

Two designer lanes of BOXES' extent-home fork, independently, gave the
arithmetic for `splitting::rules::face_reach_from` on a sphere cap past
a hemisphere: a unit sphere's cap at latitude ≥ −30° has rim radius
ρ ≈ 0.866, so from a rim point the rim levers 2ρ ≈ 1.73, while the cap
holds that point's antipode at distance 2. That is the under-statement
direction for a lever. Whether a split or join caller hands
`face_reach_from` such a face is not established; if one does, this row
is a live wrong answer and its band should rise. The extent-home design
BOXES is weighing (`work/boxes/log.md`) retires `face_reach_from` into
a reach query of the face's one cover, which answers 2 here. (BOXES
orchestrator)

Moved from APEX to GERM on 2026-11-02, by `git mv`, when APEX merged into GERM: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so GERM, GERMCERT, ROOTS and APEX came in under one budget.
