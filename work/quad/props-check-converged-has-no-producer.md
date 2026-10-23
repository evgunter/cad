---
id: props-check-converged-has-no-producer
kind: issue
title: PropsCheck::Converged and its recourse arm have no producer once the reporting target is a resolution
status: open
opened: 2026-10-23
priority: P4
cost: E
parent: the-measurement-door-refines-until-a-predicate-resolves
---


Found while building the measurement door's unit B
(`docs/QUAD-MEASURE-SPEC.md`). The reporting target is now a resolution,
which `topo` compares with a face's width rather than deciding
(`crates/topo/src/props/quad_lane.rs`, `cut_face_rounds`). So nothing
in the tree raises `PropsError::Escalated { check: PropsCheck::Converged }`
any more.

- The variant (`crates/geom-brep/src/props/mod.rs`, `PropsCheck::Converged`)
  and its two arms in `PropsCheck`'s words and recourse are dead.
- Two tests still mint `props_quad_converged` by hand to read them:
  `a_last_rounds_rejected_zero_ends_in_the_last_resort`, and the
  `Converged` leg of `an_escalation_ends_by_the_check_that_raised_it`.

Fix: delete the variant, its arms and those two legs. That is a public
enum change in `geom_brep`, so it rides with unit C, which removes the
rest of the reporting target's vocabulary (R10).
