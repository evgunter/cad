---
id: the-pole-to-pole-band-reads-its-columns-off-the-area-vector-beside-the-lift
kind: issue
title: the walk's pole-to-pole band places its columns off the loop's area vector beside the stored lift
status: closed
opened: 2026-11-20
priority: P1
cost: M
refs: [singular-turn-spells-the-chart-winding-rule-by-hand, mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers]
branch: tesslie/wrap-strip-period
closed: 2026-11-21
pr: 7303
---

Found by `tesslie/wrap-strip-period`'s sweep (2026-11-20), which moved
`mesh::walk::wrap_strip`'s period onto the loop's stored lift.

**The site.** `mesh::walk::loop_polygon`, the `band_u` arm for a rimless
loop that is not the wrap strip (the `else if no_rim` branch beside
`loop_area`): it folds the loop's 3-D area vector, reads its azimuth
against the chart frame turned outward (`mid_az`), and puts each
meridian's column on the branch `Period::near(u_raw, mid_az)`.

**Why it is a sibling.** Which `2πk` branch each meridian of a
pole-to-pole band takes is the band's width round the poles, a whole
turn. The loop's lift (`topo::Body::loop_lift`) stores it: the joint at
each pole is the face's turn there (`topo::pcurves` `singular_turn`,
the distinct-edges arm), and the lifted rows sit on the branches the
face occupies. The walk reads the same quantity a second time, off a
direction of a floating-point sum, which Ev's ruling on
`the-period-jump-has-no-ruling` (PR 4975) says a whole turn is never
read off. Nothing proves the two agree; `wrap_strip`'s two derivations
did agree on every mesh witness when measured, which is evidence of
the kind wanted here, not a proof for this arm.

**What closing it takes.** Measure first: instrument the arm to compare
each column's branch against the lifted row's (`LiftedRow::lift`) on
every rimless band in the mesh and sweep suites, and report any
disagreement as a live bug. Then read the columns off the lift, retiring
`loop_area` and the outward-frame read, as `wrap_strip` now does. A loop
with no lift then refuses `DomainUnread` as the strip and the trimmed
lane do.



## Re-banded P1 (2026-11-20, PR 7202 review)

A second derivation of a whole-turn branch the stored lift already holds, the reason `singular-turn-spells-the-chart-winding-rule-by-hand` stayed P1, and against the ruling on `the-period-jump-has-no-ruling` (PR 4975) that a whole turn is never read off numbers.

## Closed (TESSLIE, PR 7303, 2026-11-21)

`walk::band_columns` takes each meridian's whole-turn branch from its lifted image, through `trimmed::face_domain` and `walk::lifted_rows`, the one home it shares with `wrap_strip`. `loop_area`, `mid_az`, the outward-frame read and the walk's last `face.sense` read are retired. A column more than a quarter turn from its image refuses `LiftedColumnOff`.

This closed a live wrong answer. A band wider than about half a turn, with a meridian SPLIT (the trigger: placement is not needed), meshed Ok and watertight as its complementary sliver, e.g. −0.0006 against 4.188 for a sphere band at 2π−1e-3. The same held on lemon and apple spindle sheets and on reverted voids. Two reviewers' probes, 18,348 and 1,160 configurations, are all correct on the branch; main was wrong on 1,892 and 32 of them. Pinned against props by `a_split_band_wider_than_a_half_turn_meshes_its_own_volume`, `a_placed_band_wider_than_a_half_turn_meshes_its_own_volume`, and volume assertions on `a_multiply_carried_iso_side_…`.

Follow-ups:
- `the-band-columns-twin-read-back-has-no-witness` (P3).
- `an-unminted-rimless-band-now-refuses-where-it-meshed` (P3).
- The sibling `mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers` is parked on CHARTTAIL's cusp row.
