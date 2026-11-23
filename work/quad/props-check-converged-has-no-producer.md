---
id: props-check-converged-has-no-producer
kind: issue
title: PropsCheck::Converged and its recourse arm have no producer once the reporting target is a resolution
status: closed
opened: 2026-10-23
priority: P4
cost: E
parent: the-measurement-door-refines-until-a-predicate-resolves
closed: 2026-10-26
---


Found while building the measurement door's unit B
(`docs/QUAD-MEASURE-SPEC.md`). The reporting target is now a resolution,
which `topo` compares with a face's width rather than deciding
(`crates/topo/src/props/quad_lane.rs`, `cut_face_rounds`). So nothing
in the tree raises `PropsError::Escalated { check: PropsCheck::Converged }`
any more.

- The variant (`crates/geom-brep/src/props/mod.rs`, `PropsCheck::Converged`)
  and its two arms in `PropsCheck`'s words and recourse are dead.
- One test still reads it by hand: the `Converged` leg of
  `an_escalation_ends_by_the_check_that_raised_it`. The other,
  `a_last_rounds_rejected_zero_ends_in_the_last_resort`, which minted
  `props_quad_converged` to read the retired gate, went in unit B's fix
  pass.

Fix: delete the variant, its arms and that leg. That is a public
enum change in `geom_brep`, so it rides with unit C, which removes the
rest of the reporting target's vocabulary (R10).

## Closed

Measure door unit C removed `PropsCheck::Converged`, its words and recourse arms, and the `Converged` leg of `an_escalation_ends_by_the_check_that_raised_it` (`crates/geom-brep/src/props/mod.rs`), with `topo::validate`'s checks-window arm.
