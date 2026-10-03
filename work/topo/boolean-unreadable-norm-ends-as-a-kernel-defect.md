---
id: boolean-unreadable-norm-ends-as-a-kernel-defect
kind: issue
title: topo: an unreadable plane norm at the Boolean ends as a kernel defect, not as the operand's at-rest poison
status: open
opened: 2026-10-03
priority: P3
cost: E
---


(TOPO, filed by the PR 3974 fix pass, review MINOR-2.)

## What

A plane norm the carrier ladder cannot read (`plane_eq::unreadable_norm`,
`PlaneRung::Norm`) is a stored operand face whose normal is not finite:
poison read at rest. The Boolean routes it through
`BooleanDecision::of_plane_rung` (`refusal_routes.rs`, ~:1404) to
`SelfCheck::Normals`, which `BooleanDecision::ending` ends as
`Unsized::Defect`: `KERNEL_DEFECT_ENDING`, under the lead "whether the
normals of two faces can be read is undecided".

Both halves misstate the case, as they did for the offset datum before PR
3974:

- a NaN normal is decided not finite, not undecided;
- the face is an operand's, so the ending is the kernel's *or the file's*
  (`KERNEL_OR_FILE_DEFECT_ENDING`), as tier 3's
  `ValidationError::PoisonedSurfaceDatum` (`validate.rs`) ends the same
  datum and `BooleanError::VolumeCorrupt` ends an operand.

## Fix

Give it the route the offset datum took in PR 3974:
`BooleanError::PoisonedCarrierDatum` (or that variant widened to name which
datum), reached from the plane rung's door rather than through
`SelfCheck`. Pin the text with a row, as
`flush::rows::an_undeclared_coincidence_on_a_poisoned_offset_says_it_is_poisoned`
does for the offset. The flush detector already sends `PlaneRung::Norm` to
`FlushRefusal::PairUnreadable` (`flush.rs` `pair_finding`).
