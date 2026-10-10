---
id: at-infinity-probe-measures-in-closed-form-only
kind: issue
title: point-in-solid's at-infinity probe measures in closed form only, so an obliquely trimmed wall refuses VolumeUncertified
status: open
opened: 2026-10-01
priority: P1
cost: M
---


## What

`point_in_solid`'s no-hit arm reads the at-infinity side from the sign
of the selection's volume (`crates/topo/src/boolean/solid_contain.rs`,
`at_infinity_side`), and measures it with
`props::mass_properties_closed_form_of` — the closed form only, because
the door is `T: Decide` and holds no quadrature lane. A cylinder wall
trimmed by an ellipse has no closed form, so the probe refuses
`Containment(VolumeUncertified)` on a body that the certified door
(`topo::mass_properties`) measures exactly.

Measured (REACH, branch `reach/volume-backstop`, 2026-10-01): the rod
of `crates/sweep/tests/axis_lap.rs` split at `z = 0.5 + tan 20° · y`
and `z = 3.5 + tan 20° · y`, then flatted by `brick((-1, 1), (0.2, 1),
(-1, 5))`: the probe's props refusal is
`Face { source: NotIsoRectangle { what: "cylinder boundary carries an
ellipse arc (curved cut) — …" } }`. Pinned at that outcome by
`axis_lap.rs` `an_oblique_cap_flats_through_its_ellipse_arc`.

## The shape of a fix

The boolean's volume backstop had the same gap and now measures
through the certified quadrature, gated per scalar by
`AtRestPolicy::gate_volume_backstop` (run at `f64`/`Probe`/`Interval`/
`Sym`, absent at a dual). The probe can measure the same way, but
`point_in_solid`,
`point_in_solid_of` and `point_in_solid_faces` are public at
`T: Decide` with some two hundred callers, so the lane has to reach
the probe either through those signatures (an `AtRestPolicy` bound)
or as a parameter from the boolean's own call sites. The probe only
needs a SIGN, so the sign walk (`props::sign_walk`) and its bracket
ends (`ShellRole::decided_at`) are the reading to take, not the
reporting midpoint.

## Evidence (2026-10-02, REACH's ellipse-rim lane)

A boolean reaches the probe on an ordinary pose. The lower part of the
tilted drum cut (`crates/sweep/tests/conic_edge_curved_face.rs`: a
radius-0.5 cylinder of height 1, split by the plane through
`(0, 0, 0.5)` at 0.3 rad) against a ball poking up through the cut face
(radius 0.3 at `(0, 0, 0.5)`, charted about the cut's normal so the
join passes; also at `(0.1, 0.1, ·)` r 0.25 and 0.1 off the plane
either way), or a rod drilled up through the cut face (radius 0.2 at
`(0.1, 0)`, `z ∈ [0.2, 1.2]`): ∪, ∩ and ∖ all refuse
`Containment(VolumeUncertified)` once the crossing layer and the join
have passed. Which classification query reaches the probe was not
instrumented; the drum's cut wall is the only face in these operands
with an ellipse boundary. Pinned (the ball) by
`a_ball_through_the_cut_face_clears_the_rim_and_builds`.

## Also met: blind pockets in a tilted-cut cylinder (SHOW, 2026-10-02)

The `tiltedcut` scene (`demos/tour/src/curvedcut.rs`) cuts a cylinder
(r 1, height 2.5) by the plane through `(0, 0, 1.25)` with normal
`(sin 0.3, 0, cos 0.3)` and engraves blind pockets into the halves,
each tool straddling the face it cuts. Knobs varied: glyph (C, U, T,
a square, a disc), depth (0.02, 0.05, 0.2) and, on the section face,
offset in the face's frame ((0, 0), (0.3, 0.2), (−0.2, −0.3)).

- **Round caps, after the cut.** Every glyph refuses
  `Containment(VolumeUncertified)` on both halves' caps at every depth,
  a plain square included. On the unsplit cylinder's cap the same
  glyphs cut at their closed-form volumes, so the scene engraves
  before cutting; wall 3 of `curvedcut::walls` pins the upper half's
  top cap.
- **Elliptical section face, lines-only glyphs.** Pose-dependent. On
  the lower half a square and the T refuse `VolumeUncertified` at all
  nine poses (the T at offset (0.3, 0.2) crosses the rim and stops at
  the curved pierce arm instead). On the upper half a square cuts at
  8 of 9 poses and the T at offset (−0.2, −0.3) for depths 0.02 and
  0.05, each at its closed-form volume; the rest refuse
  `VolumeUncertified`.
- Arc-bearing glyphs on the section face stop earlier, at the curved
  pierce arm (`non-circle-conic-edge-refuses-against-every-curved-face` (REACH, closed by PR 3805)).

## More consumers (JOIN-3's dual review)

JOIN-3 builds blind and through D pockets whose profiles are tilted
against the block, so the block's caps cut ellipse arcs; every one of
those builds is sound (tiers 2, 3′, the certificate, the closed form)
and is not a legal operand: the far-brick union refuses
`Containment(VolumeUncertified)`, this probe's refusal. Measured
release, main against JOIN-3's fix-pass head (the batteries are
`#[ignore]`d in `crates/sweep/tests/`):

| battery | non-operands, main / head |
|---|---|
| `join3_r2_probes::j3r2_tilted_battery` (D, stadium, lens… tilted 0.15 / 0.3 rad) | 576 / 2153 |
| `join3_review_r1::j3r1_tilted_through` (random bulge profiles tilted 0.05–0.45 rad) | 460 / 526 |
| `join3_review_r1::j3r1_d_family` (the D tilted 0.2 and 0.6) | 0 / 144 |

Main already fails the gate the same way on its own tilted builds, and
on a tilted ROUND pocket (`j3r2_tilted_rod_operand`); the tilted cutters
themselves are legal operands (`j3r2_tilted_operands`, 89 of 90). No
other non-operand kind appears.

## Another witness (JOIN, PR 4008's fix pass)

`crates/sweep/tests/pocket_ring_steep_ellipse.rs`
`steep_plate_rod_battery`: a thin plate pierced by a rod tilted θ from
`z`, so every rod piece the plate leaves has elliptic planar caps. Every
body that battery builds refuses to unite with a far brick,
`Containment(VolumeUncertified)` (the differential `outcome`'s
`operand=false`): 1 151 runs on main, 1 817 once the join pairs the
steep ellipses along the conic. The rod and the plate alone unite with
the brick. Probed at θ = 30° and 70°, plate ∩ rod.


## Another witness (JOIN, the parallel cylinder arm)

Once the join splits two parallel cylinder walls along their rulings
(`parallel-cylinder-germ-pair-has-no-join-arm`, JOIN, closed by PR 4031), the
row's rods across the tilted drum cut's rim reach this probe. The
drum's lower part (radius 0.5, cut through `(0, 0, 0.5)` at 0.3 rad)
against a rod of radius 0.2 about `(0.5, 0)`, `z ∈ [0.2, 0.45]`; of
radius 0.1 about `(−0.45, 0)`, `z ∈ [0.5, 0.8]`; and of radius 0.1
about `(0, 0.48)`, `z ∈ [0.3, 0.7]`: ∪, ∩ and both differences, in
both operand orders (18 runs), refuse `Containment(VolumeUncertified)`.
The same rods against an uncut drum build sound at their closed-form
volumes, so the cut wall's ellipse trim is what the probe cannot
measure. The closed-form volumes are written down in
`crates/sweep/tests/parallel_cylinder_join.rs` (`rim_poses`), and the
poses are pinned at this door by
`the_rim_crossing_rods_build_where_a_probe_ray_meets_the_boundary`.

## Another witness (JOIN, PR 4038's pinch crossing)

PR 4038's review r1 (`r1b_pinch_probes cyl` on
`join/pierce-pinch-families-review-r1`, a prism corner on a cylinder of
axis y, r = 5): 24 intersections that main refused `JoinDesync`
("derived ring role order") now build at the exact volume (to 1e-9),
with tiers 2, 3′ and 3 and the certificate passing. Only the
legal-operand check fails: the union with a far brick refuses
`Containment(VolumeUncertified)`. Both orders of each pose: Lbot
`phi40 th120`; Ltop `phi230 th285`, `phi230 th300`, `phi300 th105`;
notch327 `phi230 th210`, `th225`, `th315`, `th330`, `phi300 th135`,
`th150`, `th30`, `th45`. Main already ships 294 such intersection
lines in that battery.

**Met again on `reach/pierce-tangent-off-face` (2026-10-06).** The
unit rod `z ∈ [0, 3]` capped by the plane `z = 1.5 + y/2` (a subtract
of a tilted brick; it builds, volume `1.5π` to 1e-15), against a brick
whose edge touches the wall's carrier above the cap, refuses
`Containment(VolumeUncertified)` in every op and both orders once the
crossing layer clears the touch. The unit took its cylinder row on a
270° extrusion instead, whose faces are all closed-form.

## Moved on branch `cleave/ray-walk`

CLEAVE's ray-walk driver unit (PR 4083) made the probe's refusal a
reading of the one ray that met nothing (`ray_walk::RayFault::Blocked`).
A ray of the schedule that meets the boundary answers, and the query
refuses `VolumeUncertified` only where no ray settles; then the refusal
says that one of its rays met nothing and no other settled it. That
can be a point whose other rays met the boundary only within the band:
`pis_arc_capped_poses::a_ray_meeting_nothing_refuses_only_where_no_ray_settles`
pins such a pose, refusing at the witness band and answering at a
tighter one.

Every pose this row and its pins name now builds, each held to its
closed form:

- `conic_edge_curved_face`'s ball through the cut face, and all three
  rim-crossing rods
  (`parallel_cylinder_join::the_rim_crossing_rods_build_in_every_op`);
- `axis_lap::an_oblique_cap_flats_through_its_ellipse_arc`;
- the tour's tilted-cut walls: the C on the lower half's section face,
  and the C in the upper half's cap after the cut.

The built rods are not yet legal operands: their union with a far brick
refuses, and why was not measured. The closed-form-only measurement is
untouched.

## Another witness (BAND, the oblique fillet's band)

A plane–plane fillet cut off at an oblique end face ends in an arc of
the end plane's elliptic section of its cylinder, so its band is a
cylinder face trimmed by two ellipse arcs. The parallelogram prism of
`crates/sweep/tests/band_planar_oblique_fillet.rs`, its top front edge
filleted at r = 0.1, measures through the certified door and
tessellates watertight. After the ray-walk above, a brick beside the
band and one through it build in every op at the closed forms, but a
brick wholly apart still refuses `Containment(VolumeUncertified)` in
subtract and union — the far-brick shape this row's last paragraph
names. Pinned by
`the_ellipse_edges_pass_the_tessellator_and_the_boolean`. The ruled
band's oblique cap and the tour bracket's filleted chords
(`demos/tour/src/bracket.rs`) carry the same face shape.

**At ε = 1e-12, through the elliptic end (PR 4173's review).** The
same fillet on the parallelogram leaning `s = 3` (its walls 72° off
square), with a brick crossing the band's end arc,
`brick((-0.5, 0.15), (-0.5, 0.05), (0.92, 1.5))`: subtract and union
refuse `VolumeUnmeasured` ("the quadrature could not decide whether
its enclosure of a face's contribution had converged"), and intersect
builds. All three build at ε = 1e-9 and 1e-6, consistent with each
other. This is not the probe's closed-form gap but the quadrature's
convergence escalation,
`work/quad/quadrature-convergence-test-escalates-instead-of-refining.md`,
met on the same face shape. Pinned by `band_planar_oblique_fillet.rs`
`a_brick_through_a_steep_elliptic_end_builds_in_every_op`.
