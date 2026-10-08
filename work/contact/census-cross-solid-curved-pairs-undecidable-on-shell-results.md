---
id: census-cross-solid-curved-pairs-undecidable-on-shell-results
kind: issue
title: The census's cross-solid backstop answers CensusUndecidable for curved solids within reach of each other, so 26 multi-solid shell results fail the empty-contact tier 3' they should pass
status: open
opened: 2026-10-03
priority: P1
refs: [3870]
cost: H
---


Found by REACH's census-fold measurement for the finished-body bar
(`analysis/reach-measure/census-fold`, `report.md` §5 class (ii);
the [ev] PR 3870).

## Measured

At ε 1e-9, 26 `shell` results pass tier 3 and fail
`validate_pseudomanifold(&body, &ContactRecords::default(), tol)` with
`CensusUndecidable`: "a curved face of one is within reach of the
other". They come from `sweep::shell7/8/9/10_*`:
- a hollow body shelled again gives nested thin curved solids;
- multi-solid operands give curved solids side by side.

Minimal repro: `census_fold_repro_nested_shell` (in the branch's
`probes/census-fold/repros/`). It revolves a 1×2 cylinder, shells it
at 0.2, then shells that at 0.05. The result has 2 solids, 4 shells and
16 faces. Tier 3 gives `Ok`; empty 3′ gives 33 × `CensusUndecidable`.

Related: the census does not examine same-solid distinct-key curved
pairs at all (C9/C6). So the same geometry passes when the two parts
are shells of one solid and refuses when they are two solids.

## Why it matters now

Ev's ruling on PR 3870 makes a finished body's bar tier 3′ against its
own declared contacts, empty for most bodies. `topo::shell` gates its
result with `validate_geometric` today. It cannot adopt the bar without
refusing these 26 results until the cross-solid backstop can decide
curved × curved and curved × planar pairs: either certify them apart, or
find the coincidence. The lane has to land before or with shell's
adoption of the bar.

## 2026-10-03 — booleans can reach it too (FUSE, PR 3891)

Under Ev's ruling that a solid is one piece of material (PR 3901),
every boolean result is sorted one solid per piece, and pieces that
only touch are distinct solids. So a union whose pieces touch with a
curved face in reach of the other piece lands in this class as well:
the "same geometry passes as shells of one solid" asymmetry above is
gone from the verbs' outputs, which no longer put two pieces under one
solid. Measured on PR 3891's branch with a box declared `Tangent` to a
plate's fillet (two solids, tier 3 green, 3′ `CensusUndecidable` alone);
main has since refused that union (`TangentSlitArmUnbuilt`), so no row
pins it today.

## 2026-10-03 — 38 boolean results on main (REACH, `boolean-door-adopts-the-finished-body-type`)

A census run over every result `boolean_op_with` returns (`python3
scripts/door-tier3-meter.py`, `origin/main` 82b9ceb2, ε 1e-9) finds 38
`sweep` union results that pass tier 3 and fail the empty-contact 3′
with `CensusUndecidable` alone ("a curved face of one is within reach
of the other"): `snowman` ×9, `run_walls_built` ×4, `shell8_r2_probes`
×4, `m5_s13_pips` ×3, `germ_sphere_no_crossings` ×2 (a ball in a
torus's hole), and single rows of `germ_torus_doors`,
`germ_interior_oval`, `m5_s10_face_sense`, `m5_s11_concave_sense_interval`,
`offer_rows`, `verbs_cylcyl_*` and `verbs_pierce*`. So the boolean door
cannot run the census over its result until this lane lands
(`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).

## 2026-10-03 — a row reaches it on main (JOIN-2, PR 3880)

`sweep` `join2_r1_probes::join2_r1_grid` (ignored, release) prints 804
`BAD` union lines on main `82b9ceb2b` and on PR 3880's head alike, with the
same set on both trees: every one is `t2=true t3p=false cert=true
operand=true` with the volume right. They are unions of an axis-aligned box
beside or diagonally off a rounded plate (e.g. `grid r=0.5 x=[-1,0]
y=[-1,0]`, the box clear of the fillet). The result has two solids, and 3′
gives `CensusUndecidable` "a curved face of one is within reach of the
other" between the plate's fillet and the box's faces. At main `66bbdaa6b`
the same battery printed 0 `BAD`, which fits the one-solid-per-piece sort
landing in between. So the grid's assertion (`bad == 0`) stays red until
this lane decides curved × planar pairs across solids.

## 2026-10-06 — a plug cut out of a plate (ZIP)

A round tube cut right through a plate leaves a holed plate and a
loose plug, and 3′ refuses the pair. The plate is `[0,3] × [0,2] × [0,1]`.
The tube is `circle_split((1.5, 1), 0.6, ..)` with its bore
`circle_split((1.5, 1), 0.4, ..)`, extruded 2.5 from `z = −0.5`.
`topo::subtract(plate, tube)` returns two solids at the closed-form
volume. They pass tier 2 and the at-rest certificate, and 3′ gives
`CensusUndecidable` between the plug's wall and the hole's wall, which
are 0.2 apart. The 16-gon twin passes 3′. In ZIP's sweep this held for
all 144 rim and bore splits and phases tried
(`a-round-tube-standing-on-a-plate-refuses-seam-orientation`,
`## Closed`).

## Measured (JOIN's pinch unit, branch `join/pinch-one-vertex-per-cone-build`)

Booleans now reach this class too. Under Ev's ruling on PR 4057 a
pinch is one vertex per cone, so lumps of a result that meet only at a
pinch point are separate shells, and the gate sorts them into separate
solids. On curved walls the census then answers `CensusUndecidable` for
the curved faces touching at the pinch. Before, the crossed vertex kept
them one shell, and the census did not examine them.

772 lines that were already `BAD` (their legal-operand check fails) add
tier 3′ `CensusUndecidable`:
- 460 in PR 4051's island battery on cylinder walls;
- 218 in review r2's cylinder poses (e.g. `Ltop cyl fib3 psi=0 off pc I`);
- 94 in review r1's cylinder poses.

The planar pinches pass: the census's `same_point` clears the shared
point key.

## 2026-10-06 — two stubs of a tilted prism through a slab (TANG)

`sweep::planar_ring_arc_closure::a_prism_with_arc_walls_through_a_slab_builds_every_op`:
a prism with an arc wall tilted through the slab `[−4, 4]² × [−0.5,
0.5]`, `prism ∖ slab`. The result is the two stubs either side of the
slab, two solids a unit apart along the slab's normal, whose tilted
walls overhang each other. Tier 3 passes and the volume matches its
closed form; empty tier 3′ answers 2 to 9 `CensusUndecidable` pairs
on 11 of the row's 18 tilted shape-poses: all nine shapes tilted
about two axes, and the 1.4π D and the lens tilted about one. The row
accepts that refusal, and only it, on those results; the upright ones
must pass.

## 2026-10-07 — two lumps of a box cut by a ball (TANG, PR 4211)

`sweep::a_ring_on_a_sphere_face::a_ring_beside_an_outer_loop_on_the_run_is_read_from_an_edge_midpoint`,
box ∖ ball for the unit ball at the origin with its poles turned off
every axis:

- the bar `[−2, 2] × [−0.2, 0.25] × [0.1, 0.4]` (poles along
  `(−0.6, 0.2, 0.77)`): the ball cuts the bar in two, one stub each
  side;
- the box `[−0.624, 1.233] × [−0.563, 0.347] × [−0.792, −0.239]`
  (poles along `(0.636, −0.720, −0.279)`): two lumps either side of the
  sphere's cut.

Each is two shells. Tier 3 passes and the volume matches the slice
integral. Tier 3′ answers only `CensusUndecidable` pairs, between the
two lumps' sphere faces and the planar faces within reach of them. The
row accepts that refusal, and only it, on those results.

## CONTACT close-out triage (2026-10-08)

Priced P1, H: arm 1 refuses `CurvedWithinReach` (`census.rs`), which blocks SHELL adopting the tier-3-prime bar and REACHHOLD's census door. `census-backstop-separates-curved-pairs-only-along-world-axes` is a sub-step: do them together.
