---
id: torus-germ-pairs-have-no-section-frame
kind: issue
title: A torus wall against a sphere, a cylinder or another torus reaches the join and refuses GermFrameUnsupported: no frame names its section
status: open
opened: 2026-10-04
priority: P0
cost: H
design: true
refs: [c5-plane-torus-cone-cylinder-arms, cylinder-sphere-germ-pair-has-no-join-lane]
---

Found by the cylinder × sphere frame lane's sweep of the germ pairs that
reach `FrameError::NoArm` (branch `join/cylinder-sphere-frame`, 2026-10-04).

## Measured

Poses (`kind_pair_sweep_probe`, run on this branch and not committed:
shapes at the origin, B moved by a rotation then a translation, ∪ in both
member orders): a drum `cyl(0.5, 1.0)` (radius 0.5 about `z`,
`z ∈ [−1, 1]`, `common::germ_pair`), a ball of radius 0.5 revolved about
`y`, a donut revolved about `y` (major 0.6, minor 0.2), a frustum, and a
brick `[−0.4, 0.4]³`; the six placements are rotation 0 then
`t = (0.45, 0.1, 0.05)`; 1.5707963 rad about `x` (a hair short of a right angle), `(0.3, 0.2, 0.1)`; 0.7 rad about
`x`, `(0.1, 0.35, 0.2)`; 1.1 rad about `(0.3, 1, 0.2)`,
`(0.42, −0.15, 0.3)`; 0.4 rad about `y`, `(0.6, 0, 0)`; 0.9 rad about
`(1, 1, 0)`, `(0.05, 0.1, 0.02)`.

`GermFrameUnsupported` from `pair_section_frame`'s catch-all, by germ pair
(the count of the 12 runs per ordered shape pair, both orders summed):

| shapes | germ pair | lines |
|---|---|---|
| ball × donut, donut × ball | Sphere × Torus | 22 |
| drum × donut, donut × drum | Cylinder × Torus | 19 |
| donut × donut | Torus × Torus | 6 |
| brick × donut, donut × brick | Plane × Torus | 21 (the plane × torus half is GERM's `c5-plane-torus-cone-cylinder-arms`) |

The remaining donut lines escalate (`ArcCylinderRoots`, `PierceCurvature`)
or stop at `CurvedPierceUnsupported` before the join. No cone pair was
in this measurement: it stopped at the operand gate then, and since the
cone joined the roster (VERBS-CONE U7) a cone germ against a curved face
is documented to refuse at its frame (`GermFrameUnsupported`; the Cone
bullet of `BooleanError::CurvedPairUnsupported`'s docs), unmeasured here.

## What a fix has to supply

A frame per torus pair, or a proof that none exists for a section shape
(a torus section can have up to four loops, and a loop need not turn
monotonically about any one axis). This is a different construction from
the cylinder × sphere frame, whose loops are graphs over a circle of one
operand. Past a frame, each pair meets the lane door
(`cylinder-sphere-germ-pair-has-no-join-lane`).

## The radial hole stops here too (GERM, 2026-10-10)

Measured on `germ/radial-hole-through-a-tube`, which gives the section
certificate its square-wall arm (a cylinder whose axis is square to the
torus axis). The donut (`R = 2`, `r = 0.5`, revolved about `y`) minus a
rod along `x` of radius `0.2`, through both walls of the tube
(`x ∈ [1, 3]`), through the inner side (`[1, 2]`), through the outer side
(`[2, 3]`), and off the axis (`[1, 3]` at `(y, z) = (0.1, 0.15)`): every
torus × wall pair of the section report answers `Ok` (each was
`Err(Reach)` on main), and ∖ refuses `GermFrameUnsupported { a_kind:
Torus, b_kind: Cylinder }` at the join. Pinned by
`crates/sweep/tests/germ_radial_hole.rs`
`a_radial_hole_through_the_tube_stops_at_the_germ_frame`, which turns red
the day this frame lands. The section there is the arm's: in the
meridian plane normal to the rod, its trace circle against the tube
circles, each loop lying over a run of that circle with two points
(`±t` along the rod) over each of its points (`section_cert.rs`
`torus_square_cylinder`), which may be the frame's starting point for
this one pose.

## Re-banded to P0 (on request in chat, 2026-10-25)

Teapot wall 2 (`demos/tour/src/teapot.rs`, the handle joined to the pot) refuses here: the vessel's sphere face against the handle's torus, the ball × donut row above. Joining a torus handle to a round body is an everyday join.

## Evidence: two tori on parallel axes (2026-10-27)

SECT's lily lane: the lily's stem run to 30° instead of 22° against its
arch (`crates/sweep/tests/mate7a_r1_probes.rs`
`a_stem_window_reaching_over_a_loop_does_not_glue`) puts the stem's seams
across the arch's tube, and both unions refuse `GermFrameUnsupported`
on the two tube walls (torus × torus, axes parallel and apart).

## Torus germs refuse at the section value (2026-10-28)

`each-torus-pair-decides-tangency-on-one-pose` put the three torus pairs
in the section table, so a torus germ inside both faces no longer
reaches the frame: a general cylinder × torus or torus × torus pose
refuses `GermSectionUnbuilt` (`Refused(RoutesToGeneralRung)`) at the
join, and a section of more than two components, or two the value
cannot tell apart at a site (the square wall, parallel tori, a sphere
cutting both tube circles), refuses `GermSectionUnbuilt` (`Untold`) at
the reduction's germ mint. A coaxial torus pair carries its parallels
and stops at the join's segment arm (`CurvedBooleanUnsupported`). The
radial hole's pin is now
`germ_radial_hole.rs` `a_radial_hole_through_the_tube_stops_at_the_germ_mint`,
the lily's `mate7a_r1_probes.rs`
`a_stem_window_reaching_over_a_loop_does_not_glue`. What a frame row
for these pairs owes now is a site reading that tells their components
apart, and the door rows' carriers.


## The doors march every transverse pose (SECT, 2026-10-28)

`sect/torus-doors` built `ssi::cylinder_torus_ssi` and
`ssi::torus_torus_ssi`. Re-measured on this row's placements: every
transverse pose of both pairs certifies at the door (drum × donut and
donut × drum at all six placements, donut × donut at five, the sixth a
tangency), and the joins answer as before but for the note. The
symmetric poses (placement 0's square wall and parallel donuts,
placement 4's square wall) carry the general rung's carrier and stop at
the germ mint `Untold`, as do the radial hole and the lily; the general
poses (drum × donut 1, 2, 3, 5; donut × donut 1, 2, 3) refuse
`Refused(RoutesToGeneralRung)` at the germ mint, the section value
listing no components for them
(`a-general-torus-pose-lists-no-components`, a design fork). Placements
whose union never reaches the section (`ArcCylinderRoots`,
`CurvedPierceUnsupported`, a plane × torus spiric) are unchanged.

## Components are the march's, a site's component its foot (SECT, 2026-10-29)

`sect/components-by-foot` (`a-general-torus-pose-lists-no-components`)
retired the classification's site reading: every torus germ inside both
faces takes its component from its foot on the carried section, so no
pose here refuses at the germ mint any more
(`crates/sweep/tests/germ_torus_components.rs`). What each half reaches:

- **sphere × torus**: ball × donut 5 (the sphere round the axis) passes
  the mint and stops at the ring lane's island on the torus face
  (`RingIslandUnread`, the torus face's spline arm, unbuilt); the other
  placements are as before.
- **cylinder × torus**: drum × donut 0 to 5 pass the mint; 0, 1 and 2
  stop at the result gate's volume (a projected boundary, or a conic
  trim on the torus chart), 3 at the cap plane's spiric, 4 and 5 at the
  ring island. The radial hole stops at the volume.
- **torus × torus**: donut × donut 2 and 3 stop at the ring island, 1 at
  the join's segment dispatch (`CurvedBooleanUnsupported`); the lily's
  long stem at the march's `CellBudget`.

What this row owes now is the torus face's ring-lane arm for a spline
island and the segment lane past the ring island.
