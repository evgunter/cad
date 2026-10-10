---
id: offset-held-neighbour-image-keeps-a-declared-chart-on-a-transverse-section
kind: issue
title: offset_restate::held_neighbour_image moves a declared chart image onto a moving side that usually meets the held side transversally; check 4 would refuse it at rest (unverified)
status: open
opened: 2026-10-06
priority: P3
cost: E
blocked_on: [restatement-derives-each-moved-edges-kind]
---


## What

A round-5 designer on PR 3970 found this while reading, but did not run it.

`crates/topo/src/offset_restate.rs` `held_neighbour_image` restates a *declared* chart image on a held neighbour as `EdgeDescriptionSpec::chart(moving).declared_by(mc)`. That is a declared chart image on the moving side. The moving side and the held side usually meet transversally, at a section. Tier 3 check 4 (`validate.rs`) refuses a *declared* chart image on a definitely-transverse edge with `TransverseNotIntrinsic`, so this restatement may leave a body that is invalid at rest.

PR 4080's witness `the_planar_door_moves_a_declared_edge_into_the_moved_faces_chart` asserts tier 3 = `[TransverseNotIntrinsic { edge }]` on exactly this case. So the state is reached and pinned as the expected outcome, not refused.

## Depends on

How Ev rules D2's prefer-intrinsic authority question on PR 3970 (round 5). Under Ev's 2026-07-19 ratified rule, which has no authority exemption, every definitely-transverse edge must carry `Intersection`, and an `Intersection` has no slot for a declaration. Whether the door should then refuse, or drop the declaration, is that question.

## Re-pointed from the D10 hold (2026-10-10)

This row was filed on the orchestration branch, so INTENT's 2026-10-08 re-homing of the parked rows did not reach it. It turns on D2's prefer-intrinsic authority question. Since INTENT stage 4 E (2026-10-09) that question lives on `restatement-derives-each-moved-edges-kind`, released for a designer pass, so this row waits on that row. `held_neighbour_image` still restates `chart(moving).declared_by(mc)` on main (`crates/topo/src/offset_restate.rs:108`).
