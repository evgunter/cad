---
id: a-general-torus-pose-lists-no-components
kind: issue
title: A general cylinder × torus or torus × torus pose lists no components: the section value refuses it though the door marches it
status: open
opened: 2026-10-28
priority: P0
cost: M
design: true
refs: [torus-germ-pairs-have-no-section-frame, cylinder-torus-rung-three-arm-is-unretired, torus-torus-rung-three-arm-is-unretired]
---


Filed by SECT's `sect-torusdoors` lane (branch `sect/torus-doors`),
which built the cylinder × torus and torus × torus doors and stopped
here on a design fork.

## Measured

`geom_brep::ssi::cylinder_torus_ssi` and `torus_torus_ssi` march any
pose whose critical gap is transverse, general poses included: all six
of the germ row's placements (`torus-germ-pairs-have-no-section-frame`)
certify for drum × donut and donut × drum, and five of six for donut ×
donut (the sixth is a tangency), each loop count matching an
independent contour count on the torus's chart
(`crates/geom-brep/tests/torus_doors_ssi.rs` `germ_row`). `route` and
`route_pose` answer `implemented` for both pairs.

`geom_brep::pair_section` still refuses a general pose of either pair
(`SectionError::RoutesToGeneralRung`, `pair_section::torus`
`GENERAL_POSE`): its classification has no closed-form count of the
components, so it lists none. Every reader of the classification needs
that list:

- `topo::boolean::section_cert` `of_pair_section` certifies a pair by its
  components (each with `winds` and a witness); an empty list would
  certify the faces disjoint, so the refusal there stays R-reach;
- the join's germ mint (`topo::boolean::join` `section_branch`) reads
  `PairSection::component_of` on the classification before anything is
  carried, so a general pose refuses `GermSectionUnbuilt` at the germ
  mint (`Refused(RoutesToGeneralRung)`), at drum × donut placements 1, 2,
  3 and 5 and donut × donut 1, 2, 3 (measured on this branch).

## The fork

What a general pose's classification lists, which the rows and the
ratified clauses (C5, `pair_section`'s "classified, then carried") do
not settle:

1. **The march's components.** The classification marches (at `f64`
   only, `SsiLane`): each closed branch a component, its witness
   `carrier(mid)`, its `winds` read off the fitted loop. The
   classification then costs a march, which C5's value says it does not,
   and a scalar with no lane cannot classify the pose.
2. **Components unlisted until carried.** A classification that says its
   components are the march's: `component_of` answers untold, the
   certificate refuses as now, and only `carry` lists them. The join
   gains nothing until its germ mint reads the carried section's
   `foot` instead of the classification.
3. **Keep the refusal** until a frame row needs the pose (the status
   quo this branch lands).

Whichever is chosen, the symmetric poses that list two components the
classification cannot tell apart (the square wall, parallel tori) refuse
`Untold` at the same germ mint, and the radial hole
(`germ_radial_hole.rs`) and the lily's tori (`mate7a_r1_probes.rs`)
stop there.
