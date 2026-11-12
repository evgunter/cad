---
id: a-self-touch-point-its-tangent-cones-leave-to-second-order-passes-unread
kind: issue
title: A self-touch point whose curved faces' tangent cones leave it to second order passes unread
status: dispatched
opened: 2026-11-02
priority: P1
cost: H
refs: [tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face]
---


Found by `tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face`'s
lane (branch `restread/self-touch-curved-sector`).

## What

**A named silent pass.** Where a solid touches itself at a point with
a curved face or edge in its link, `sweep_self_touch_points`
(`crates/topo/src/census.rs`) reads the faces' tangent cones
(`self_touch_verdict`, `Link::tangent_of`, `FaceSector::tangent`).
`rest::one_material` decides the point wherever the cones' first order
does (`rest::first_order`). Where it does not, the verdict is
`SelfTouch::SecondOrder`, and the point is skipped, as every such point
was before. The classes, as `SelfTouch::SecondOrder` lists them:

- **A cone's apex** in the link, or a vertex in the band of its axis
  (`corner_normal`: `VertexFace::Apex` or `NearAxis`).
- **A cusp corner**: a face whose two edges leave the vertex along one
  tangent (`corner_fan`) — a line tangent to an arc at its end, or two
  arcs tangent there. A spur's tip (the loop back along the half-edge it came by) is read as the
  whole turn, not a cusp.
- **A first-order tangency**: a bound lying on another sheet's plane or
  tangent plane where the faces keep no such coincidence, or two whole
  tangent planes on one plane (`rest::first_order`).
- **Sectors read lying on each other** (`Degenerate`): coinciding
  tangent sectors, or a slit or doubled sheet of planar faces in a link
  with a curved face.
- **A face the tangent reading cannot build**:
  - a spline face held off a corner;
  - a face whose foot normal (`foot_normal`), reach (`face_reach_from`)
    or corner face (`sector_face::resolve`) does not read;
  - a curved edge with no certified curve.

## Measured (head `edff118e41`, merge base `c3626bf54a`, topo + sweep, slow set, default ε)

The unit is a distinct (test, point, verdict) triple: the witness a
dumped reading names, as printed, in one test. Readings repeat across
a test's poses and orders: 688 readings collapse to 124 triples, over 80
distinct exact points in 37 tests and 17 test files. The tangent path's
own witness rows (`census::self_touch_curved`) are left out.

- **Read at first order: 34.**
  - 33 `Rest`, none `Crossing`.
  - 1 refused in band:
    `m5_pr9_boss_union::the_curved_inventory_is_admitted_and_the_bogus_record_is_stale`,
    which stays green.
- **Left to second order: 90.**

  | class | triples |
  |---|---|
  | cusp corner | 43 |
  | an arc's tangent lying in a planar face | 27 |
  | sectors on each other (`Degenerate`), all at `bisector_nodes`' equal-cylinder nodes | 10 |
  | a bound on another sector's bound | 4 |
  | a cone's apex | 2 |
  | an arc's tangent in a curved face's tangent plane | 2 |
  | a foot normal | 1 |
  | a `resolve` refusal | 1 |

The skip now lets each test run on past its first residue point, so
more points are reached than when the residue refused. The first
measurement refused it, at head `95102fc368`, and read 33 / 1 / 73.
The 8 distinct `Degenerate` exact points (4 when rounded to 1e-6) are
the reviewers' 20 and 8, counted under their dedupe keys.

Refusing the residue (`BoundaryCrossesItselfUndecided`) turned 13
tests red at that first head, every one in `crates/sweep/tests/`:

- `a_closed_conic_in_the_partners_face::a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`
  (two cylinders tangent inside);
- `a_cone_apex_on_a_plane_through_it::a_plane_meeting_a_tipped_cone_at_its_apex_alone_keeps_it_whole`
  (a cone's apex);
- `a_split_through_a_wrap_edges_vertex::` ×5 (cusp corners, a line on an
  arc's tangent):
  - `a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound`
  - `a_boolean_tangent_to_a_two_arc_holes_rim_builds_sound`
  - `a_plane_tangent_to_a_holes_rim_at_a_vertex_splits_sound`
  - `a_plane_tangent_to_a_two_arc_holes_rim_splits_sound`
  - `a_split_through_a_hole_walls_strut_vertex_keeps_one_face_where_the_piece_goes_round`
- `bisector_nodes::a_rod_against_a_bore_builds_every_operation` and
  `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`
  (equal cylinders' nodes, where all four tangent planes are one:
  `Degenerate`);
- `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_splits_alike_under_either_normal`
  and
  `split_across_a_revolve_seam::a_plane_touching_a_bore_rim_at_its_seam_splits_alike_under_either_normal`
  (cusp corners);
- `two_wedge_ends_on_one_line::a_brick_floating_in_a_lune_beside_unread_wedge_ends_refuses_unexamined`
  and
  `two_wedge_ends_on_one_line::two_lune_cavities_kissing_at_their_cusps_refuse_their_wedge_ends`
  (lune cusps).

Five more tests reach the residue and stay green when it is refused,
because their assertions accept a refusal:

- `review_6020_seam_probes::` ×2 (cusps, and an arc's tangent on a
  planar face);
- `review_5956_r2_probes::r2_pit_secants` (a cone's apex);
- `reach_wall_chord_rows::a_vertex_on_a_curved_face_is_confirmed_by_its_trim`
  (a foot normal);
- `census::tests::a_vertex_on_edge_record_off_a_circle_edge_reads_stale`
  (a `resolve` refusal).

## The fix

A reading of second order, as the cross-solid curved lanes take one:
at a cusp, which side of the shared tangent each edge bends to
(`geom_brep::conic_bend_side`); at a tangency, the sheets' relative
curvature along the shared plane; at a cone's apex, the cone of
rulings itself (`sector_face::ConeApex`). Each must refuse where it
does not decide. Whether the residue refuses
(`BoundaryCrossesItselfUndecided`) in the meantime, and the 13 rows go
red with it, is a decision for the orchestrator with Ev.

## Read one order down (branch `restread/self-touch-second-order`)

`link::second_order` reads two of the classes: a cusp corner, by
which side of its ray each edge bends to (a horn, or the whole turn
but a horn), with a ring of horns read as the thorn it bounds; and a
bound tied on another sheet's plane or bound, by whether the sheets
along that ray part one order down. The argument, with the band
charged, is in `crates/topo/README.md` beside the first-order one.

**Re-measured at merge base `dc4e9b279e`** (same instrument, topo +
the ten sweep families that reach the skip, default ε). Main moved
since the head above: the tangrim program's crescent rows now reach
the skip too. 122 (test, point, verdict) triples were left to second
order there; now 67 read `Rest`, none `Crossing`, and 55 are still
skipped, named and counted:

| class still skipped | triples |
|---|---|
| a slit (an inside plane and a coplanar face of the other sense, `review_6020_seam_poses`), stopped at an arc's tangent in that plane | 30 (27, and 3 of the cusps) |
| sectors on each other (`Degenerate`), `bisector_nodes`' equal cylinders | 14 |
| a lune: a horn beside two cylinder sectors coinciding at first order (`two_wedge_ends_on_one_line`) | 4 (cusps) |
| a curved sheet holding the tied ray inside it, whose line `parts` does not read: an arc's tangent in a curved face's tangent plane (`a_rim_flush_…`, 2) and a bound on a bound (`a_hole_sharing_the_rim_…`, 1) | 3 |
| a cone's apex | 2 |
| a foot normal | 1 |
| a `resolve` refusal | 1 |
| a thorn the cone round it reaches (its ray not decidedly outside the cone's material, or inside it for a needle), which may be a real overlap | 0 |

The last class reaches no sweep row; it is witnessed at body level,
where a thorn grafted round a box corner's ray stays skipped
(`census::self_touch_curved`). A sign one order down left in band is
`Unreadable` with the structural refusals: the skip's `why` does not
part them.

Of the cusps, 36 read and 7 are still skipped; of the bounds on a
bound, 35 read and 1 is still skipped (4 triples carried both tags).

**Refusing what is left** (`BoundaryCrossesItselfUndecided`, a local
variant never committed) turns 7 sweep rows red: 6 of the 13 above
(`a_rim_flush_…`, the cone apex, `bisector_nodes` ×2,
`two_wedge_ends_on_one_line` ×2) and
`a_closed_conic_in_the_partners_face::a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent`,
added since. All five `a_split_through_a_wrap_edges_vertex` rows and
both `split_across_a_revolve_seam` rows read whole and stay green.
Whether the remainder refuses is still Ev's call.

## After PR 6422

PR 6422 (`restread/self-touch-second-order`) merged on 2026-11-03 with
the reading above, so this row is open again for the 55 triples still
skipped. The two biggest classes are the slits (30) and
`bisector_nodes`' equal cylinders (14). Whether the remainder refuses
in the meantime is still Ev's call.

## Coincident sheets (branch `restread/self-touch-coincident-sheets`)

**Measured** with the same instrument (topo + the ten sweep families).
A triple is (test, witness, verdict), the witness the census's own
string (`census::witness`, each coordinate's `Debug` form), so two
points differing in their last bits count twice. At `8a117ad3ee` 57
triples were skipped; at merge base `3f4161d8dc`, 59 (58 in the sweep
families, and the `resolve` one in topo): main's `bisector_nodes` rows
reach 16 equal-cylinder triples where the row says 14, and its
`review_6020` poses 32 slits where it says 30. Rounded to 1e-9 the 59
are 36 points (17 slits, 8 equal-cylinder). At `3f4161d8dc`:

| class | triples | now |
|---|---|---|
| slit (`review_6020_seam_poses`) | 32 | refused `Degenerate` (`rest::slit`), as the planar link refuses two planar faces on each other |
| `Degenerate`, `bisector_nodes`' equal cylinders | 16 | read `Rest`: two lenses each (`second_order::lenses`, `rest::lensed`) |
| lune (`two_wedge_ends_on_one_line`) | 4 | skipped: a horn beside a lens sharing one edge |
| curved sheet holding a tied ray | 3 | skipped: its line now reads (`second_order::parts`), and the two `a_rim_flush_…` points then stop at two cylinders tangent inside on each other, no lens; `a_hole_sharing_…` at a bound on a bound |
| cone apex | 2 | skipped |
| foot normal | 1 | skipped |
| `resolve` refusal | 1 | skipped |

These counts are exact at branch head `279faf4e51`, before main was
merged at `6ff9bbe916`. After that merge the witnesses moved by ulps, so
the triples no longer key one to one with these, and 11 are still
skipped. The slits are the counterbore split's
cut faces lying on each other, measured on
`work/heldrecords/a-plane-touching-a-counterbores-floor-rim-ships-sides-tier-three-prime-refuses.md`;
`review_6020_seam_poses` asserts nothing at tier 3′, so no row goes
red. The arguments are in `crates/topo/README.md` beside the others.

**Refusing what is left** (local variant, never committed) turns 5
sweep rows red: `a_rim_flush_…`, `a_hole_sharing_…`, the cone apex, and
`two_wedge_ends_on_one_line` ×2. Both `bisector_nodes` rows now read
whole. Whether the remainder refuses is still Ev's call.

## After PR 6526

PR 6526 (`restread/self-touch-coincident-sheets`) merged on 2026-11-10
with the reading above, so this row is open again for the 11 triples
still skipped: the lunes (a horn beside a lens sharing one edge), two
cylinders tangent inside on each other, a bound on a bound, the cone
apex, a foot normal and a `resolve` refusal. Refusing them turns 5
sweep rows red. The next unit is the lunes and the tangent-inside
cylinders. Whether the remainder refuses in the meantime is still Ev's
call.

## Lunes (branch `restread/self-touch-lunes`)

Tier: **dual (class H): a GRANT in the self-touch reading.**

`second_order::lenses` now also reads a **lune**: two sectors on one
wedge `W` of opposed normals, `W` one sector's whole fan and the
other's first or last wedge, each end of `W` a pinch (one edge, or two
lines on one line), a cap (a horn whose two edges are the sectors'
bounds there) or a mouth (the other sector runs on past it). The form
`q` is read in Bernstein form along `W`: a pinch reads Zero, a cap or
mouth reads one decided sign `σ` with both planes' parting there
charged, and the polar value `b₁`, read as the gap it makes along the
unit middle (`b₁/(1 + cos φ)`), reads `σ`. `b₁` may read Zero only
with no pinch, or where every pinch is two lines on developable faces
(tangent along the whole line): beside a curved pinch a zero `b₁`
leaves the parting to third order, where the faces may meet in a
tacnode inside `W` (the review's torus and cylinder), so such a pair
stays skipped. The lens is the two-pinch case of the same reader. `rest::lensed`
reads lunes with the lenses, the remainder holding the mouth's sector
cut short at the mouth (bounded by the other sector's edge there) and
without the caps; a cap whose ray another sector reaches refuses in
band. The argument is in `crates/topo/README.md` beside the lens's.

What the real bodies hold: the lune cavities' points are two lunes
(a horn of the floor capping two cylinder sectors pinched along their
shared cusp line); `a_rim_flush_…`'s point is two lunes with mouths
(`B`'s wall runs on past the rim, `A`'s seam and `B`'s split line two
lines on the pinch).

**Measured at head `1785f0d865`** (the second fix pass, main merged)
against merge base `4d193f273d`, the same counts as at `5f1ddd91bb`
against `b125b84b30` and as first measured at `4a2c743e4e` against
`ea1c8517ce`,
same instrument (key: test, `census::witness`, verdict; topo plus the
nine sweep families `a_closed_conic_in_the_partners_face`,
`a_cone_apex_on_a_plane_through_it`, `a_split_through_a_wrap_edges_vertex`,
`bisector_nodes`, `split_across_a_revolve_seam`,
`two_wedge_ends_on_one_line`, `review_6020_seam_probes`,
`review_5956_r2_probes`, `reach_wall_chord_rows`, default ε). 478
triples both sides. Skipped at the base: 11, as above. At head 6 read
`Rest` (the four lune triples, both `a_rim_flush_…` triples) and 5
stay skipped:

| class still skipped | triples |
|---|---|
| a bound on a bound whose bends are equal (`a_hole_sharing_…`): the cut's anchors coincide, third order | 1 |
| a cone's apex | 2 |
| a foot normal | 1 |
| a `resolve` refusal | 1 |

No other triple changes verdict, and the head reaches no point the base
does not. Not reached by any measured body, and skipped: a curved pinch
whose polar value reads zero; a pair whose other sector runs on past
both ends of `W`; and a mouth that falls inside a wedge of the sector
running on, which happens where that sector's edge is turned off a
quarter turn from the other's (`corner_fan` splits at quarter turns),
so `a_rim_flush_…` turned so is skipped again: conservative.

**Refusing what is left** (local variant, never committed) turns 2
sweep rows red: `a_hole_sharing_…` and the cone apex. Both
`two_wedge_ends_on_one_line` rows and `a_rim_flush_…` now read whole.
Whether the remainder refuses is still Ev's call.


## After PR 6816

PR 6816 (`restread/self-touch-lunes`) merged on 2026-11-11 with the
reading above, so this row is open again for the 5 triples still
skipped: a bound on a bound whose bends are equal (`a_hole_sharing_…`,
third order), the cone apex (2), a foot normal and a `resolve` refusal.
Refusing them turns 2 sweep rows red. Also skipped, and reached by no
measured body: a curved pinch whose polar value reads zero (third
order), a pair whose other sector runs on past both ends, and a mouth
inside a wedge of the running-on sector (the pose dependence above).
Whether the remainder refuses is still Ev's call.

## A cone's apex, and two arcs of one circle (branch `restread/self-touch-apex`)

Tier: **dual (class H): a GRANT in the self-touch reading.**

What the real bodies hold: at the cone apex (`(0, 1, 0)`) a void cone's
whole nappe, its face's corner a spur at the apex (one wrap edge), under
a plane face holding the point inside, its material below; at the pit
(`(0, 0.134, 0)`) the same needle as two half-turn corners of one
carrier, split at the revolve's two meridians, under the cut plane. At
`a_hole_sharing_…` the boss's rim and the hole's rim are one circle
through the point (the boss's vertex sits inside the shared arc), so
the four sheets' anchors coincide at every order: no order of the cut
parts them, third order included.

- **A cone's apex** (`rest::at_apex`): a cone is its own tangent cone at
  its apex, so the face is its rulings there. A nappe held whole (one
  spur corner, or two corners of one carrier and sense, each bounded by
  the other's edges, their rulings decidedly apart) bounds the convex
  cone `C`. Every other sector's plane must hold the nappe strictly to
  one side, read at the axis point of the rulings' circle at the arm
  with the circle's spread off the plane (`ρ·|m × a|`) charged; then
  `C` lies in one region of the rest of the link, which must read one
  material and read `C`'s axis outside it for a thorn, inside it for a
  needle. A whole plane with rulings decidedly on both sides is a
  crossing; a side in band refuses `InBand`; a horn beside the nappe, a
  half-plane cutting it, a nappe not held whole, or the axis on the
  wrong side is left unread.
- **Two arcs of one circle** (`second_order::one_circle`,
  `rest::kept_bound`): two circle edges along one ray whose curvature
  vectors read Zero apart at the arm, off the sector's plane and square
  to the ray in it, are kept as the faces keep them, as two lines on one
  line are: a point, a tangent and a curvature vector fix a circle.
  Arcs of other conics are not kept (the edge's carrier must be a
  circle, `Bound::circle`).

The arguments are in `crates/topo/README.md` beside the others.

**Measured at head `d44b40b182` against merge base `c43415ae04`**, same
instrument (key: test, `census::witness`, verdict; topo plus the nine
sweep families, default ε). 478 triples both sides. Skipped at the base:
5. At head the cone apex (1), the pit (1) and `a_hole_sharing_…` (1) read
`Rest`; no other triple changes verdict and the head reaches no point the
base does not. The later commits change rows and one equivalent arm of
the apex reading (a same-side read of the two far rulings, which the
charged reading already decides); the three triples read `Rest` again
there. Still skipped (2), both in rows that accept a refusal:

| class still skipped | triples |
|---|---|
| a NURBS face held inside at the point (`reach_wall_chord_rows::a_vertex_on_a_curved_face_is_confirmed_by_its_trim`): its tangent plane's fan needs a carrier frame, which a spline has none of (`census::carrier_frame`) | 1 |
| a NURBS face's corner `sector_face::resolve` refuses: no sector arm (`census::tests::a_vertex_on_edge_record_off_a_circle_edge_reads_stale`, `NoSectorArm`) | 1 |

**Refusing what is left** (local variant, never committed) turns no
sweep row red: both remaining triples are in rows that accept a refusal.
Not reached by any measured body, and skipped: a second apex in one link,
a horn beside a nappe, a nappe of more than two corners, and a
half-plane cutting a nappe. Whether the remainder refuses is still Ev's
call.

## After PR 6835

PR 6835 (`restread/self-touch-apex`) merged on 2026-11-11 with the
reading above, so this row is open again for the 2 triples still
skipped. Both are NURBS faces: one has no carrier frame, and
`sector_face::resolve` refuses the other's corner (`NoSectorArm`).
Refusing them turns no sweep row red.

The review's fix pass added to the reading:
- a nappe's rulings are read against the plane square to its axis, so
  two lobes on opposite nappes are not a nappe, and a ruling in band of
  that plane refuses `InBand`;
- the thorn/needle material read is shared (`Cone::material_round`).

Also skipped, and reached by no measured body:
- a second apex in one link;
- a horn beside a nappe;
- a nappe of more than two corners;
- a half-plane cutting a nappe;
- the axis decided on the wrong side of the rest's material, an overlap
  read but skipped, as `thorns` skips it.

Whether the remainder refuses is still Ev's call.

## The remainder refuses (branch `restread/self-touch-refuses`)

Ev ruled on PR 6847 (2026-11-12): a self-touch point the reading
cannot decide refuses `BoundaryCrossesItselfUndecided`, with its cause,
and is not skipped. The spline faces that will now refuse are a
separate question.

The unit has four parts:
- Remove the skip and `SelfTouch`. `self_touch_verdict` returns a plain
  verdict.
- A sign one order down that lands in band reports `InBand`, not
  `Unreadable`.
- An overlap the reading has decided (an apex axis on the wrong side,
  a thorn) reports a crossing where its argument certifies one, and
  refuses otherwise.
- File the pose-dependent lune (`corner_fan`'s quarter-turn split).

**Spec (before the draws):**
- **Letter: M.** It removes a state and reroutes reasons. The one new
  argument is the decided overlap.
- **Tier: dual.** It is a soundness change in the self-touch reading:
  every undecided point now refuses, and a decided overlap becomes a
  new crossing verdict.
- **Arm:** under `docs/MODEL-COMPARE-PROTOCOL.md` rule 2, a dual M unit
  runs sequentially. A drawn model reviews first, and the other model
  reviews second only if the first raises a MAJOR.
