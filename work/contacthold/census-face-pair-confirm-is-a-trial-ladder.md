---
id: census-face-pair-confirm-is-a-trial-ladder
kind: issue
title: confirm_face_pair tries four readers in turn instead of dispatching the face pair to one
status: parked
opened: 2026-10-14
blocked_on: [a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band]
cost: M
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

`census::confirm_face_pair` (`crates/topo/src/census.rs:6629`) confirms a face-face record by trying four readers in order:
1. two planes → `confirm_patch`;
2. else `carrier_pair_relation` (`OutsideInventory` or `Distinct` fall through);
3. then `point::face_face`;
4. then `boolean::tangent_pair_at_rest`.

The order is a runtime trial ladder: which reader answers depends on which earlier one declined, not on the kind pair.

## Owed

Re-aimed to D3 "One reading per question". The earlier target, "dispatch on the face pair's kinds to the one reader", is not the ratified shape: a consumer never matches on kinds to choose a method.

- The confirm reads the face pair's one pose reading (coincident ±, point touch, line tangent and the rest), plus the outcome reading that takes it where the record's witness is an outcome. It matches on that answer, and a pose it does not confirm refuses typed.
- The ladder, where `OutsideInventory` and `Distinct` from `carrier_pair_relation` send the pair on to `point::face_face` and then `tangent_pair_at_rest`, is a method reached because another declined: a fallback D3 forbids. The two planes' `confirm_patch` is the pose reading's coincident answer, not a first rung.

`restread/the-two-carrier-face-pair-confirm-escalates-where-its-second-face-reads-the-point-out` is a symptom on the same door.

## 2026-10-17 — parked on the D10 hold, moved from RESTREAD

RESTREAD read this row against the D10 hold (`docs/DESIGN.md`, State of
the build) and the open [ev] PR 4666 (the census records what it
decides): it stands on contact records, declared pairs or the shape of
at-rest findings, which INTENT stages 4 and 5 rebuild. Per RESTREAD's
plan (a row needing declared pairs or contact parks and moves to
CONTACTHOLD), it moved here by `git mv` with its id unchanged. Priced
`M` where it carried no cost. (RESTREAD orchestrator)

**Re-read 2026-10-29 (INTENT, D10 hold):** re-parked on `a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band`. The ladder survives stage 4 unchanged (`crates/topo/src/census.rs:9688-9800`). Its second rung sends an in-band carrier pair to `confirm_patch` as "the declaration the patch door's rung 3 reads" (`:9712`), which is the bridging rung that row is deciding. One pose reading cannot be written until it is known whether that rung stays.
