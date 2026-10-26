---
id: lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces
kind: issue
title: A declared pair is levered over both whole faces, so a small part on a large plate refuses a tilt its contact patch holds in band
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## What (design note)

The declared `Rest` door reads a pair as one displacement over its
consumed extent (`carrier_eq::declared_reading`), and the extent is the
ball enclosing BOTH WHOLE FACES (`rest::pair_extent`). The tilt term is
levered at that ball's reach. A small part resting on a large plate —
a 10 mm puck on a 2 m bed — therefore has its tilt levered at the
plate's ~1.4 m, not at the puck's 10 mm footprint where the two faces
actually meet. A tilt above `Kε / plate size` reads past the band:
the door refuses (unsettled, or contradicted where a plate vertex
stands off), even though every point of the actual contact patch is
in band and the contact is sound.

The verdict is consumed on the contact patch (the overlap region the
crossing layer and the REST zip consume), not on the parts of either
face the other never touches. Levering by both whole faces is sound
(it only refuses more), but over-refuses exactly the "small part on a
large base" assemblies the declared door exists for.

## The fix's shape

Lever by the consumed contact patch: a ball enclosing the overlap of
the two faces (the chart-region overlap `chart_region` already computes
for `Rest`, or a cheaper sound superset: the intersection of the two
faces' boxes, which encloses the overlap), and the witnesses restricted
to the boundary points inside it. This changes which faces' extent a
verdict is read over, so the declared rows (`rest::lever_rows`,
`contact9_side_codes`' wedge rows) need re-measuring with it. Filed
from the fix pass of PR 3795 (item 6b).

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the fix site is the declared Rest door (carrier_eq::declared_reading, rest::pair_extent), retired with declared pairs at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by stage 4 F (2026-10-11)

Stage 4 F (`declared-pairs-retire`) retired every boolean declaration: no author can declare a pair, the glue is what the margins decide, and the declare offer, the flush detector and the carried declarations are gone. What this row waited on has fired; re-read it against that tree.

## Measured: the undeclared reading too (2026-10-26)

FLUSH's glue-verdict designers (PR 5737, round 3) met the same lever on
the undeclared path, through the plain pair boolean. Probes are on the
scratch branches `flush/glue-verdict-design-u92Pc` @ `84c44eeb`
(`crates/topo/tests/honest_fold_probe.rs`) and
`flush/glue-verdict-design-015xnd` @ `278f1fab`.

- **The pose.** A ten-unit plate beside a unit plate rolled by θ about a
  line along the row. Every point of either face lies within θ·1 of the
  other's plane.
- **The refusal.** The union refuses in band at θ = 0.2ε (margin
  1.55e-9) and at θ = 0.4ε (3.4e-9):
  `Merge(BooleanMerge(Escalated { Neighbours(Parallel) }))`.
- **The path.** It comes from `merge_faces.rs`'s undeclared neighbour
  read (`undeclared_pair_verdict` over `pair_extent`'s ball) into
  `plane_eq::plane_ladder` rung 2. That rung levers the angle at the
  ball's radius, about 8.5 here, where the vertices are already in hand.
- **The honest margin.** The declared rung reads each face's points
  against the other's plane; that is the honest margin.
- **Unchecked.** At θ = 0.8ε the pair refuses
  `Containment … Loop(Boundary)` at 1.131e-9 = 0.8ε·√2, against a true
  displacement of 0.8ε. This may be a second over-meter, by √2, in the
  containment reading.

## Ev's ruling (PR 5737, 2026-10-26)

"tolerance.epsilon should meter an angle in whatever way is required to
make all geometry correct to within epsilon". `[tolerance.epsilon]` now
says so: an angle's lever arm is the reach at which it displaces the
geometry it governs, and for parallelism that is each face's own points
measured against the other's carrier, not the extent of both whole
faces. The declared and the undeclared readings above both follow from
this. So do the code comments that quote the old "face extent for
parallelism": `geom-brep` `dihedral.rs` (`reach`) and `enters.rs`.

