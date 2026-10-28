---
id: a-boxed-rotation-refuses-not-rigid-at-every-placer
kind: issue
title: transform_rigid at Interval refuses a boxed placer — a widened rotation NotRigid, a translation boxed wider than about eps at its endpoint certification — so every boxed placer, instance or tilted mate frame refuses in a box run
status: closed
opened: 2026-10-01
priority: P3
cost: M
design: true
closed: 2026-10-20
---

Found by a designer weighing MSOLVE's plan item 19 (the mate solve on
analysis lanes). It is filed here because the door is TOPO's. It was
read, not probed.

## What

`topo::transform::check_rigid` decides the orthogonality residuals of
an `Affine3<T>` under `Decide`. Inside `transform_rigid` this means:
- At `Interval`, a map whose rotation part is widened (a boxed angle)
  has residuals that straddle zero beyond `band.zero()`.
- So it is `Indeterminate`, and the door refuses `NotRigid`: "a
  maybe-rigid map is not a rigid map".

Every boxed rotation therefore refuses in a box run. That covers a
`Transform` or `Pattern` angle bound by the box today, and a tilted
mate frame once the solve runs at the box's scalar. A boxed
translation passes, because it needs only finiteness.

## The question

Should an enclosure of a family of rigid maps pass the rigidity door?
For example: certify rigidity per member of the family (a rotation
parametrised by an angle is rigid at every angle) rather than of the
widened matrix.

If not, the refusal should name the box's angle as the cause rather
than read as a non-rigid map.


## Measured: a box-wide translation refuses too (MSOLVE-14, PR 3986)

"A boxed translation passes, because it needs only finiteness" holds
for the rigidity check and not for the door as a whole. Probed on a
`Transform` node over an instance, translation `x = gap`, box
`gap ∈ nominal + [−0.25, 0.25]`, at `Interval`: the node refuses
`NodeErrorKind::Transform(Certify { .. EndpointStart ..
carrier_endpoint_start, margin [0.0, 0.5] })`. The placed edge's
endpoint and its placed carrier are each widened by the box
independently, so the on-carrier check encloses `[0, 2·width]` and
escalates; a box narrower than the band places. A mate solved at the
box's scalar meets the same refusal at the mated instance, pinned by
`msolve14_run_scalar::a2_over_a_wide_box_the_bolts_refusal_is_the_placement_doors` (since INTENT E
`a2_over_a_wide_box_the_bolt_builds_its_copy_unbuilt`: a copy is its
source's body under its frame and is not re-certified, so the mated
copy builds).

No instance is needed for it: a plain `Transform` over a plain extrude,
its translation boxed at `gap ± 0.25`, refuses
`Certify(EndpointStart, carrier_endpoint_start, margin [0, 0.5])`
(measured by MSOLVE-14's review).

The priority deserves reconsidering: this row now gates every boxed
translated instance, not only a boxed rotation.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage3-is-built`, not on the whole program: the fix site is boxed placement (transform_rigid on a Transform/instance/mate frame); stage 3 makes frames variables, placement a bundle of mates, and retires Transform-as-placement. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Re-homed at TOPO's cut (2026-10-11)

Moved from TOPO to TOPOTAIL when TOPO dropped to P2 and split on its priority seam (`work/topotail/log.md`). TOPO kept the Euler surgery doors; this row is a torn-record, refusal-recourse or code-hygiene row, or a parked boolean or placement row.

## The rotation half, answered by SYM-17 (2026-10-12)

The question above, "should an enclosure of a family of rigid maps pass
the rigidity door?", is Ev's ruling on SYM-17
(`work/sym/a-widened-rotation-angle-refuses-on-the-plain-interval-lane.md`,
"Question, and Ev's answer"), built by SYM-17's Phase 2 (PR #4197):
`topo::transform_rigid` takes a `geom_core::RigidMotion` and decides
nothing about rigidity. A `Transform` step and a stepped pattern mint
their motion by construction, so a boxed angle there no longer refuses
`NotRigid`. A raw map (a literal frame, a STEP placement, a pose the
mate solve composed) goes through `RigidMotion::check_rigid`, which is
today's predicate and now returns an in-band margin as `Undecided`.

What is left here:
- the tilted mate frame: the mate solve's poses are still `Affine3`
  and are decided at the door where they move a body, until the mate
  side is typed (INTENT's
  `the-mate-solve-s-representatives-and-inverses-mint-the-rigid-motion`);
- the boxed translation's endpoint refusal, which is SHELF's
  `transform-rigid-recertifies-images-enclosed-apart`.

## Closed

Closed by INTENT stage 3 D's lane (2026-10-20), checked on D's head. Both halves the 2026-10-12 section left are gone, and the residue it names has its own row:
- **The placement half (the tilted mate frame).** A placement's map is a fold of mates and values composed as motions, and the evaluation places it as the motion it is (`eval/wire.rs`, the `Folded::Pinned` arm: `composed.motion`), never through the widened matrix's seven margins; a map not composed so is admitted by `mate::solve::admitted`, which re-witnesses an `Undecided` one by Gram–Schmidt. Pinned by `msolve14_run_scalar::a2_a_wide_box_on_a_spin_is_no_rigidity_refusal`: a bolt turned by a spin boxed ±0.025 rad at `Interval` refuses at no rigidity door. With the composed arm sent through `RigidMotion::check_rigid` instead, the row goes red with `Motion(Undecided { transform_rigid_col0_unit, margin ±0.028 })`.
- **The boxed `Pattern` angle.** A stepped pattern's motion is minted by construction (`eval/wire/stepped.rs`, `stepped_rule_map`: `RigidMotion::rotation_about_axis_by`), so a boxed angle meets no rigidity check. Listed frames are literal and hold no boxed number. Spec §5 sends this half to G, which retires `PatternKind` whole; nothing of it refuses before then.
- **The boxed translation's endpoint refusal**, which the same row still meets over a box that wide (`Transform(Certify { EndpointEnd, carrier_endpoint_end, margin [0, 0.05] })`), is SHELF's `transform-rigid-recertifies-images-enclosed-apart`, open.
