---
id: a-general-torus-pose-lists-no-components
kind: issue
title: A general cylinder × torus or torus × torus pose lists no components: the section value refuses it though the door marches it
status: closed
opened: 2026-10-28
priority: P0
cost: H
refs: [torus-germ-pairs-have-no-section-frame, cylinder-torus-rung-three-arm-is-unretired, torus-torus-rung-three-arm-is-unretired]
closed: 2026-10-29
branch: sect/components-by-foot
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

## Decided (2026-10-29)

Two designers (one Opus, one Fable), dispatched blind on the problem
rather than the three options above, reached one final state; the
orchestrator took it (`docs/DESIGN-FORK-LOG.md` row 212). No `[ev]` PR:
the contract it replaces ("classified, then carried" with a list at
classification, and `component_of`) is agent-written (PR 5872), and fork
row 100's "foot parameters replace frames" already reads this way.

A correction to the statement above: the join matches germs by foot on
the carried section (`section_table::germ_foot`); the branch the germ
mint reads from the classification is only cross-checked against it
(`carried_seats`). So the general poses and the `Untold` poses are one
gap, a second naming of components beside the foot.

- **The classification's list is optional:** `Counted(Vec<SectionComponent>)`
  where a closed form counts the components, `Marched` where none does.
  Classifying still costs no march and needs no lane; an empty `Counted`
  is "disjoint", and `Marched` is never read as that.
- **`carry` lists.** For `Marched` the components are the march's closed
  branches, `winds` read off each fitted loop's chart image, the witness
  `carrier(mid)`. For `Counted` the march's count must equal the arm's and
  each branch foot on one listed component, else `Desync`: the arm is a
  cross-check of the march's exhaustiveness receipt.
- **One site reading.** `component_of`, `Sides` and the `Untold`/`Site`
  refusals retire; a site's component is `CarriedSection::foot` at every
  reader. The germ mint reads it, and `Branch` gives way to a component
  index for the pairs the section table holds (the frame-era pairs keep
  `Branch` until their own rows), so three or four loops are representable.
- **The certificate** reads `Counted` as now and carries a `Marched`
  pose, reading the march's components (W3's witness margin widened by
  the tube radius); at a scalar with no lane it refuses typed by name. Its
  wildcard `Err(_)` arm becomes an exhaustive match.

Rejected: the classification marches (a lane enters classification);
keep the refusal; a certified contour count beside the march (a second
enumeration); keep `Sides` beside the foot. The build is its own unit.

## Closed

2026-10-29, by `sect/components-by-foot`. The classification's list is
`Components::Counted` where a closed form counts the components and
`Components::Marched` for a general cylinder × torus or torus × torus
pose; `RoutesToGeneralRung` left both arms. `PairSection::carry` lists:
a marched section's components are the march's closed branches, each
winding as its fitted loop's chart image does (`pair_section::winds`,
turns per channel off the carrier's interval hull, padded by its tube);
a counted section's march must match the count (as many branches, each
listed witness on exactly one branch within the band widened by its
tube, each claimed winding held), else `Desync`. That winding check is
a deviation from the decision's "winds agree": it is one-way (claimed ⇒
marched), since the parallel-tori arm lists `false` unread; scheduled
on `a-counted-sections-winding-is-a-claim-not-a-reading`. The march's
branches must also be distinct (each branch's witness on it alone), so
a loop found twice is a desync the count cannot see. `component_of`, `Sides`,
`SiteRefusal::Untold`, `SectionUnbuilt::Untold` and `CarryError::Site`
are gone: the germ mint carries the pair and takes the site's foot
(`Branch::Component(k)` for the pairs the section table holds), and the
matcher reads the same foot. The certificate reads a marched pose from
its march (W0 to W4, W3 widened by the tube), refuses R-lane at a scalar
with no march and R-march where the march refuses, and matches every
`SectionError` by name. `route_pose` serves the general poses.

Reached: GERM's radial hole passes the mint at all four rods (4 and 3
loops) and stops at the volume (`spiricarm/a-projected-boundary-…`);
the drum × donut placements all pass the mint (volume, conic trim, the
cap's spiric, a wall run image); ball × donut 5 and donut × donut 1 to 3
stop at the volume; the lily's long stem stops at the march's own
`CellBudget` (SECTRUNG's). The tilted cut's clear torus builds every op
(`orbit/a-torus-near-a-tilted-cut-stops-at-the-extent-scan`).
