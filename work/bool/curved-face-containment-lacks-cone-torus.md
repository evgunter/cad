---
id: curved-face-containment-lacks-cone-torus
kind: issue
title: curved_face_containment has no cone or torus arm while point_in_solid now answers both kinds
status: open
opened: 2026-09-01
github: 1484
refs: [1464, 1425]
---

## From GitHub issue 1484

opened 2026-09-01, 0 comments.

Recorded at BOOL-3's adjudication (PR [#1464](https://github.com/evgunter/cad/pull/1464)); both blinded reviews flagged the unscheduled deviation independently.

`topo::boolean::contain::curved_face_containment` resolves `{Cylinder, Sphere}` and answers `Ok(None)` for every other kind — an honest "no answer" rather than a refusal, so it does not surface as a typed boundary anywhere. The SOLID-level door (`point_in_solid`) has served cones since PR [#1425](https://github.com/evgunter/cad/pull/1425) (BOOL-2) and tori since PR #1464 (issue 1011's torus half), so the FACE-level question is now unanswered for two kinds whose solid-level question is answered, and callers that fall back on the `None` get a worse answer than the kernel can give.

BOOL-2 left the cone there and BOOL-3 left the torus; neither unit's fence included `contain.rs`. The work is the two arms plus whatever their callers do with a `Some` they previously never saw — which is the part that needs scoping, since `None` is currently a supported outcome rather than a gap.

## Home

`work/bool/` — `crates/topo/src/boolean/contain.rs` is inside S-BOOL's territory glob and the charter names containment doors; BOOL-2 and BOOL-3 are the units that left the two arms.
