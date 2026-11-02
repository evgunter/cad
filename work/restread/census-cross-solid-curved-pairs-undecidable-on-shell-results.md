---
id: census-cross-solid-curved-pairs-undecidable-on-shell-results
kind: issue
title: The census's cross-solid backstop answers CensusUndecidable for curved solids within reach of each other, so 26 multi-solid shell results fail the empty-contact tier 3' they should pass
status: open
opened: 2026-10-03
priority: P0
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
plate's fillet (two solids, tier 3 green, 3′ `CensusUndecidable` alone).
Main refused that union for a while (`TangentSlitArmUnbuilt`); it answers
again since `contacthold/face-face-touch-row` (PR 4921), two solids with
the face–face record of the box's wall and the fillet
(`reach_continuation::a_tangent_beside_a_fillet_builds_every_op_in_either_order`,
which does not run 3′). The like rod resting on a plate
(`face_face_touch::a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it`)
refuses 3′ `CensusUndecidable` on the rod's wall and a plate face,
measured on PR 4921's review; no row pins that verdict.

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

## Also in `join1_delta_arc_battery` (2026-10-08)

Measured on main `047d10d5` and on `join/battery-hygiene`, unmoved
between them: 336 lines of `crates/sweep/tests/join1_delta_probes.rs`
`join1_delta_arc_battery` build at the closed-form volume, with tier
2, the certificate and a legal operand, and fail tier 3′ on
`CensusUndecidable` alone. They are z-prisms over arc profiles that
meet at an edge or a cap and come out as two solids. Sampled:
`ARC half lower d=(0.5, 0.0) zb=(2.0, 4.0) decl=false U` is two
solids, and every error is "a curved face of one is within reach of
the other".

## CONTACT close-out triage (2026-10-08)

Priced P1, H: arm 1 refuses `CurvedWithinReach` (`census.rs`), which blocks SHELL adopting the tier-3-prime bar and REACHHOLD's census door. `census-backstop-separates-curved-pairs-only-along-world-axes` is a sub-step: do them together.

## Also an annular tube in a plate (JOIN, `join/annular-tube-roles`)

`crates/sweep/tests/an_annular_tube_through_a_plate.rs` pins four
results that build at the closed form, with tier 2, the certificate
and a legal operand, and fail tier 3′ on `CensusUndecidable` alone
(all 8 and all 15 errors). Each is two solids, one in the other's
bore:
- the plate `(−3, 3)² × (0, 1)` less the tube `r ∈ [0.5, 1]`,
  `z ∈ [−0.5, 1.5]`: the holed plate and the rod in its bore;
- the union of the nested tubes `r ∈ [0.75, 1]` and `[0.25, 0.5]`,
  `z ∈ [0.5, 2.5]`, and its `∩` with the plate (both orders) and its
  `∖` the plate.

## 2026-10-11 — 542 boolean results at a curved corner pinch, after CONTACTHOLD (FLUSH)

Same probe as the at-infinity row's 2026-10-11 section
(`curved_notch_pinch`, main `6e3985cb7`, after CONTACTHOLD's PRs 4598
and 4604): 542 built bodies fail tier 3′ with `CensusUndecidable`
alone. Each has the oracle's volume, passes tier 2 and the
certificate, and has no definite tier-3′ finding. That is the 461 of
main `875e049a` plus 81 that refused at the boolean there
(`ClassificationInvariant`, `SharedVertexCrossings`) and build now.
CONTACTHOLD's rewrite moved none of the 461.

Read on poses `9 4 4 ab S` and `0 0 0 ab I`: the result is two solids
that meet only at the pinch point, one vertex per cone, and the one
record is the vertex–vertex contact there. The undecidable pairs are
the cross-solid backstop's (`census.rs`, `CurvedWithinReach`), between
the faces through `v`: the bowed wall (`Cylinder`) against the other
solid's planes, and the notch's arc-bounded caps (planar, not
line-bounded, so not skipped to the exact sweeps) against them. These
pairs touch at `v`, so no box gap or `backstop_apart` direction can
separate them, and the backstop does not read a vertex–vertex record.
This is the JOIN pinch class above ("the planar pinches pass: the
census's `same_point` clears the shared point key"). A curved face at
the pinch, or a planar face carrying an arc edge, does not clear.

Pinned by `a_curved_notch_at_a_corner_pinch_builds_bodies_the_census_cannot_read_yet`
(poses `0 0 0 ab I` and `9 4 4 ab S` undecidable, `0 0 0 ab U` one
solid and clean): when this lane lands, flip those two expectations.

## Boolean results meet it too (FLUSH, 2026-10-11)

`crates/sweep/tests/join3_r2_probes.rs` `j3r2_pocket_battery` reads 14
of its 10 800 runs BAD on tier 3′ alone (tiers 2 and the certificate
pass, and the body unites with a far brick), on `main` at `121741a2b`
and on `flush/e-tail` alike. All are at the offset `(0.75, 0.75)`,
where the cutter reaches past the block's corner, and each result is
two solids: block ∪ a `D-0.45` turned 4 rad, which the block does not
meet (2 solids, 10 faces, the volume the block's 4 plus the cutter's,
in all five entries, both orders), and block ∖ a `D0`, `D0.3` or
`lens` cutter run through, which severs a corner (2 solids). Each
fails `validate_pseudomanifold` with 5 to 7 `CensusUndecidable` ("a
curved face of one is within reach of the other") between the cutter's
or the corner's curved wall and the other solid's faces. The tags are
`pocket D-0.45 phi=4.000 off=(0.75, 0.75) {top,bottom,through,flushtop,flushbot} U {AB,BA}`
and `pocket {D0,D0.3,lens} phi={0.700,4.000} off=(0.75, 0.75) through S AB`.

## 2026-10-11 — touching pieces, where no separation clears (FLUSH)

`sweep` `join1_delta_probes::join1_delta_arc_battery` (ignored,
release): `ARC tri {half, quarter, shallow} d=(0.0, 0.5)
zb=(0.5, 1.5) U` build at their closed forms, tiers 2 and the
certificate green, a legal operand, and fail tier 3′ on this arm alone
(main `bef9a8f1`, and `b65bc54d` before PR 4613). The triangle prism's
apex edge `x = 0, y = 0.5` lies along the partner's face `y = 0.5`
over `z ∈ [0.5, 1.5]`, so the two solids TOUCH along that segment, and
every census finding is `CensusUndecidable { CurvedWithinReach }`: the
triangle's two slanted faces against the partner's arc-bounded caps,
and its cylinder wall (planar but not line-bounded counts as curved
here). These pairs meet at the apex segment's ends, so no box or
turned direction separates them at any pose: the arm needs a contact
read for curved or arc-bounded faces, not a sharper separation test.
The all-planar twin `ARC tri tri` passes 3′. The op's records of the
touch are complete since FLUSH's `an-apex-edge-…` (PR on
`flush/apex-edge-in-face-plane`): the bound records plus the apex
edge's span against the face (or the collinear edge, `quarter`).

## 2026-10-12: a tangent touch's union reaches it (`contacthold/same-side-tangent-touch`)

Ev's ruling on `[ev]` PR 4723 makes the union of two operands that
touch only along a tangent line a two-solid `Assembly`, and that union
now builds. Its 3′ pass on its own records refuses only with
`CensusUndecidable` (`CurvedWithinReach` on the face pairs, then the
solid pair). Three fixtures show it:

- two discs kissing externally;
- two externally tangent upright cylinders;
- PR 4704's `sameside-corner` quarter round on a slab.

`crates/sweep/tests/common/touch_union.rs` admits exactly that
refusal (`CurvedWithinReach` on face pairs across the two solids, and
the `Unexamined` solid pair it leaves) and no other finding, and cites
this row. PR 4781's review fixtures add cylinders of unequal radii and
a box edge on a cylinder's ruling, which read the same. When this lane
lands, that check can demand `Ok`.

## 2026-10-12 — lobes touching at a pinch (CONTACTHOLD)

Pieces that touch at one point, where the touch is recorded, land here
too. The L-prism (reflex top corner `(1, 1, 1)`) intersected with a
cylinder of radius 3 or 6 whose side passes through that corner, at
the normals `(1, 1.3, 0.7)`, `(1, 1.3, 0.4)` and `(1.2, 1.0, 0.3)`,
builds in either operand order: two lobes on one point key at the
corner, and the corner's `VertexFusion` row (the prism's corner on
the cylinder's side). Each of those 12 results then fails
`validate_pseudomanifold` against its own records with
`CensusUndecidable` ("a curved face of one is within reach of the
other") between the cylinder wall of one lobe and a face of the other.
The unions of the same pairs pass. Measured at
`contacthold/zero-to-distinct-cells-audit` (PR 4765) by its review
probe `review_4765_curved_probes::probe_curved_pinch` (sweep), which
was dropped once it had been read. The pairs meet at the pinch, so no
separation clears them: the arm needs a contact read at a recorded
pinch, not a sharper separation test.

## 2026-10-12 — five more at the curved pinch (FLUSH)

FLUSH's pinch-residue lane (branch `flush/pinch-residue`) made 15
`curved_notch_pinch` lines build that refused `JoinDesync` on main.
Five of the new bodies fail tier 3′ with `CensusUndecidable` alone,
the signature above (the oracle's volume, tier 2, the certificate, no
definite finding): `0 3 2 ba S`, `1 6 0 ba S`, `3 1 0 ba S`,
`6 1 0 ab I`, `6 1 0 ba S`. Replay with `CURVED_POSE="i j k order op"
cargo test -p sweep --release --test all curved_notch_pinch_one --
--ignored --nocapture`.

## 2026-10-14 — the node family's `∖` (TANG, branch `tang/pinch-node-ring`)

Two equal cylinders whose axes meet now build every boolean, each
node minted as a ring in both walls. `A ∖ B` is two solids, the
pieces of `A`'s rod either side of `B`, touching at the two nodes
`q ± r·unit(â₁ × â₂)` (`q` the axes' meeting point), each piece with
its own valence-2 vertex on each node. Every one of
`sweep::bisector_nodes`'s ∖ results (θ ∈ {90°, 60°, 120°, 35°}, both
operand orders, and the re-posed twin) builds at the closed form
`2πrh − 16r³/(3 sin θ)`. Tier 3 passes, ∩ and ∪ pass 3′ on their own
records, and ∖'s 3′ answers only this lane's frontier: 7
`CensusUndecidable` "a curved face of one is within reach of the
other" between the two pieces' faces, and the `Unexamined` pair of
the two solids. The row admits exactly that
(`touch_union::only_curved_within_reach`); flip it to `Ok` when this
lane lands.

The pairs touch at the nodes, so no box gap separates them, but one
plane always does: the plane through `q` with normal `â₁` has each
piece of `A ∖ B` (`B ∖ A`: normal `â₂`) wholly on one side, and meets
the pieces only at the two nodes, which are shared point keys.

## 2026-10-17 — scope under the D10 hold (RESTREAD)

RESTREAD builds the half that certifies curved × curved and
curved × planar cross-solid pairs apart. Where such a pair turns out
to be a genuine coincidence, what the census does with it is the
"census records what it decides" path of the open [ev] PR 4666 and
INTENT stage 5; that half is not built here, and a pair the lane finds
touching keeps its `CensusUndecidable` until then.

## 2026-10-19 — the certify-apart half lands (RESTREAD, `restread/cross-solid-apart`)

Arm 1 now also clears a pair on a gap in depth (`census::depth_gap`):
the distance from an axis line, a centre or a torus's core circle that
either face's carrier or a conic on its boundary proposes, one face's
greatest depth below the other's least past both pads. Measured with a
probe that runs the empty-record 3′ over every `shell` result of
`sweep::shell7/8/9/10_*` (222 results, 100 of them multi-solid): 30
refused `CensusUndecidable` on main `31a6c491`, 0 on the branch, the
tori among them. Flipped to pass, each pinned: the nested shell repro,
a hollow torus shelled again, a rod in a bore (on and off the axis), a
part in a blind bore (`s16_box_soundness`), the plug and nested tubes
of `an_annular_tube_through_a_plate`, and both `B ∖ A` runs of
`a_plane_across_a_one_face_wall`. A rod flush in its bore or half a
band inside it, and a rod crossing its bore, still refuse
(`cross_solid_apart`).

What is left here:

- **Touches** — the pinch, tangent, apex-edge, node and touch-union
  sections above. No gap clears a pair that meets; they keep
  `CurvedWithinReach` until the census reads a curved touch (the
  D10-held half: [ev] PR 4666, INTENT stage 5).
- **Not re-measured on this branch**: REACH's 38 union results, JOIN-2's
  grid (850 `BAD` lines on the branch, its sampled lines a box face on
  the plate's; no main count at this head), TANG's tilted stubs and
  ball-cut lumps, FLUSH's pocket battery and JOIN's delta-arc battery.
  The door meter (`scripts/door-tier3-meter.py`) re-reads the first.
  A pair the depth gap cannot read: a cylinder or cone face's least
  depth about another axis (it has none but its own carrier's), and a
  planar face's about a core circle.

## 2026-10-13 — a rod flush with a plate's height (CONTACTHOLD)

Measured on `contacthold/fold-step-face-face-row`: a rod (r 0.5,
vertices off the ruling) resting on a plate's side face over the
plate's full height, z 0 to 1, unions into two solids carrying the
face–face record of the plate's face and the rod's wall and two
edge–edge records where the rod's cap circles touch the plate's top and
bottom edges. Its 3′ refuses `CensusUndecidable` on the rod's wall
against a plate face that it meets only at a cap circle's recorded
edge–edge point. The n-ary union row that uses it
(`editor_core` `union_fold_face_face_touch`) does not run 3′.

## 2026-10-26 — two more apart readings (RESTREAD, `restread/curved-pairs-apart`)

Re-measured on main `ed68a59e` before building:
- **ZIP's plug** passes (both splits tried).
- **REACH's door classes**: the boolean door's gate (`door-tier3-meter`,
  every `sweep` result in the `ci` profile, release) refuses 0 of 3794
  gated results.
- **TANG's tilted stubs** pass: every pose of `planar_ring_arc_closure`
  decides.
- **Still refusing on main:**
  - TANG's ball-cut lumps (`a_ring_on_a_sphere_face`, 3 rows);
  - JOIN-2's grid, 850 `BAD`;
  - FLUSH's pocket, 12 `BAD`;
  - a box off a rod's wall within about 0.04 of it.

Arm 1's turned-direction test now reads two more things:
- **The offset from an axis or centre** either face proposes to the
  other's nearest boundary segment point (`census::depth_gap::toward`),
  where a curved face lies within its carrier's radius of that axis or
  centre.
- **A sphere face's held reach** (`census::held_reach_in`): along each
  direction, no further than its boundary's reach at an end whose pole
  the face places `Out`.

Moved:
- **The ball-cut lumps pass** (`a_ring_on_a_sphere_face` re-baselined
  to tier 3′ `Ok`; `cross_solid_apart::a_bar_through_a_ball_…`).
- **A box off a rod passes at 0.04 and 1e-3**, and touching still
  refuses (`cross_solid_apart::a_box_off_a_rod_…`).
- **The tilted stubs' `Undecided` acceptance is retired**
  (`planar_ring_arc_closure`).
- **JOIN-2's grid: 850 → 794 `BAD`.** All 48 rounded-plate poses apart
  in plan clear. The 544 rounded-plate lines left all touch or overlap
  in plan. In the L grid, 258 → 250.
- **FLUSH's pocket stays at 12.**

What is left of the certify-apart half:
- **A sliver or a lens** that only a third face's normal or a chord
  parts: `census-arm-1-reads-directions-from-the-pairs-own-faces-only`.
- **Coplanar faces no line separates** (the L's notch):
  `census-arm-1-cannot-part-coplanar-faces-no-line-separates`.

Touches keep `CurvedWithinReach` under the D10 hold, as above. JOIN's
delta-arc battery reads 346 lines failing 3′ alone on the branch;
nothing measured it on today's main, and this change only adds clears.

## 2026-10-27 — the touch behind it now reads (ATREST, `atrest/gap-reads-a-cell-against-a-curved-face`)

The touch analysis now reads a vertex resting strictly inside a sphere or cylinder face of another solid, where that face is all the other solid holds at the point. It judges the vertex's planar link against the face's carrier (`topo::link::rest::by_round`, through `Germ::touch_at_point` in `census.rs`). This arm still refuses such a pair (`CurvedWithinReach`) before any record site is read: the pair is the vertex's planar faces against the round face. So no verdict moves, and the instrumented topo, sweep, step-import and editor-core suites never reach the new reading. When this row clears a curved × planar pair down to its touches, a recorded vertex-on-sphere or vertex-on-cylinder touch reads Rest or Crossing, not `TouchUnreadable`. A plain deferral of the pair to its recorded touches is not sound. This arm is the only reader of a curved × planar pair, because the exact sweeps hold planar boundaries only. The two faces can also meet away from every record, for example where the vertex's planar face cuts the sphere elsewhere, and a deferral would hide that meeting. The clear needs the pair's whole meeting accounted for: its separation off a neighbourhood of each recorded touch, or a meeting set the touches cover.

## Also measured: two lumps touching along a line, one a disc slab (2026-10-30)

PR 6053's review (`crates/sweep/tests/review_arm_vs_reach_probes.rs`
`review_curved`): the disc slab of radius `r` (`0.5`, `0.05`, `2.0`)
through the origin, centred below it, beside a quad slab whose long
edge leaves the origin `θ` off the disc's tangent. Where the two touch
along a line, the union is two lumps and fails tier 3′ on
`CensusUndecidable { MayOverlap }` (curved faces of two lumps in
reach), on main as on that PR's head, at `θ = −0.1` too. PR 6053 makes
more of these unions build where main refused (`θ = ±10⁻²`), so the
finding now shows at more poses; the census reading is unchanged.

## Also measured: two rods tangent outside along a ruling (CLEAVE, PR 5852, 2026-10-31)

PR 5852's probe (`rev5808r`'s `S7 rods out`): `A` the cylinder
`r = 0.3` about the origin, `z ∈ [0, 1]`, its vertex at azimuth 0 on
the tangent line through `(0.3, 0)`; `B` the cylinder `r = 0.1` about
`(0.4, 0)` over `z ∈ [0, 1]`, `[0.5, 1.5]` or `[−0.5, 1.5]`, its vertex
at azimuth 0 or π; the one-segment and two-arc forms, at rest and
turned. `A ∪ B` and `B ∪ A` return `Ok`, two solids touching along the
ruling, the closed-form volume, tiers 1 and 3 and the certificate
passing, and tier 3′ refuses with `CensusUndecidable` ("a curved face
of one is within reach of the other"): 44 cells at the default ε, on
PR 5852's head and on `main` (`45f6be756b`) alike, and at ε 1e-6 and
1e-12 the same 44. With `A`'s vertex off the line every cell refuses
typed, `CurvedPierceUnsupported`. With PR 5793 merged (riders reading
every edge in a face), four more join them at the default ε: the
one-segment unions with `B`'s vertex at 0 over `[−0.5, 1.5]`, at rest
and turned, which refused `EdgeInFace` before, now build and reach the
same `CensusUndecidable`, 48 in all.
