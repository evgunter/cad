---
id: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
kind: issue
title: The sector passes key on the one bound-side reading's first-order tie and never descend it; curved faces, the vertex-pair charge and the Tangent witness are unbuilt
status: open
opened: 2026-11-01
priority: P1
cost: H
---


Stage 2 of `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`
(its `## Design (decided 2026-11-01, TANGRIM)` is the spec). Found by
the stage-1 lane (`tangrim/one-bound-side-reading`).

## What stage 1 left

Stage 1 built `topo::bound_side`: `read` (first order at the bound's
reach, charged where the datum carries a lever) returns `Off(side)` or
a `Tie`, and `Tie::descend` reads the tie one order down
(`Tangent(side)` / `Rides` / `Unread`). Every bound-side consumer goes
through it: `sectors::side_code` (so `pair_codes`, `wedge_classes`,
`cone_side`, `recl::flank_key` and recl's other `side_code` reading),
`sectors::pierce_side` (`vtxfac`), and `splitting::entry_side` (the
split walk and `vertex_section` at both doors). The second-order
parting is one primitive, `geom_brep::parting` / `parting_side`.

Still owed, each against the design's numbered items:

1. **The sector passes never descend** (item 4). `side_code` maps a
   `Tie` to `SideCode::On`: the gateways (`recl`'s on-edge event,
   `sectors::along`, `insert::across_tangent`, the edge-on-sector and
   edge-on-edge rules) key on that first-order tie. Re-key them on the
   tie itself (germ attribution) and on `Rides` (coincidence), then let
   `side_code` return the descended side. CONTACTHOLD's hazard list
   (15 sweep rows in `join2_r2_probes`, `rest_zip_admission`,
   `reach_continuation`, `rest_nested_strut` that went to
   `CurvedBooleanUnsupported` in `recl::require_same` when `side_code`
   alone descended; `a_rising_round_tilted_onto_a_slab_edge_keeps_its_operands`;
   `review_tilted_dip_off_the_vertex_on_face_pass`) is the outcome diff
   to run. The tie and its predicate are on `bound_side::Tie`, so the
   re-keyed gateways read the same value the descent starts from.
2. **A curved face's tie is `Unread`** (item 2's second bullet).
   `Tie::descend` returns `BoundSide::Unread` for every bound against
   a curved face; the consumers read it `On`, as before stage 1. The
   arm is `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` through `geom_brep::parting`
   at the shorter of the bound's reach and the face's lever, which
   needs the face's surface in `bound_side::FaceDatum` (today its
   normal, shape and lever). `a_split_through_a_wrap_edges_vertex.rs`'s
   `a_ball_tangent_to_a_two_arc_holes_rim_is_held_or_builds_sound` holds
   the refusal that arm moves.
3. **The vertex-pair pass takes no curvature charge** (item 2's first
   bullet, `vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces`).
   `pair_codes` passes `NO_CURVATURE` against the other sector's
   normal. `BoolSector` carries no face datum (55 constructions), so
   the charge needs `BoolSector` to carry `FaceDatum` (the same field
   item 2 needs), replacing `vtxfac::PierceDatum`'s `FaceShape` and
   `PierceLever`.
4. **A `Tangent` verdict carries no witness** (item 3, M2). The conic
   against a plane keeps `conic_bend_side`'s certified lift; the other
   arms (item 2's, and the parting sites `runs_into`, `crescent`,
   `corner_holds`, `walls_inside`, `tangent_relative_side`,
   `tied_germ_turn`) decide the leading term only. The witness's offset
   should be the covering root door's residual where one covers the
   edge and carrier kinds.
5. **`splitting::wide_entries` reads a bisector's side of the split
   plane itself** (`split_bisector_side`, `Margin::levered(d·n̂, arm)`),
   beside the reading. It decides the same sign as
   `bound_side::read` with `Reach::Bisector(arm)` (a tie against a
   plane `Rides`). It was not moved in stage 1 because
   `split_bisector_side` is a K-REPORT/k-lint row
   (`docs/K-REPORT.md`, `tools/k-lint`), so moving it renames a
   tracked predicate and the baseline needs re-deriving with it.
6. **Second-order side readings beside the reading.** PR 6219
   (`tangrim/fillet-line-arc-junction`, merged 2026-11-01) re-keys a flanker glued `Tangent` to the reference in
   `recl::resolve_edge_sector` through `tangent_relative_side`, and reads
   a tangent sector's germ bounds at second order in
   `vtxfac::classify_vertex_on_face` (`TangentRead`). Both are instances
   of the class this row retires: fold them into `bound_side` with item
   1, keeping `crates/sweep/tests/a_box_along_a_fillets_line_arc_junction.rs`
   green.
7. **A bisector's or probe's tie reads `Unread`.** `Tie::descend`
   returns `BoundSide::Unread` for `Reach::Bisector` and
   `Reach::Extent` against a plane: a direction has no bend of its own,
   and a bisector of a sector on a curved face leaves the plane at
   second order. The arm: the bisector's own sector face's normal
   curvature along it stands in for the bound's bend (the design's
   item 2, second bullet, "a bisector has no bend"), `Rides` only where
   that face is planar. `FaceDatum` (or `Reach::Bisector`) has to carry
   the sector face's curvature for it.
8. **Sites the stage-1 sweep missed or left unnamed** (found by PR
   6217's review):
   - `geom_brep::bends_into_material`: a wall's curvature sum against
     its tangent plane, a second-order reading of a face's side;
   - `splitting/rules.rs` `tangent_sector_osculation`;
   - `slit.rs` `bool_rider_wall_side` and `walls_inside`'s descent: a
     wall's first-order side against the floor, then the parting, read
     beside the reading.
9. **`d̂·n̂·arm` is spelled four times**: `bound_side::rise`,
   `geom_brep::sense_margin`, `conic_bend_side`'s residue, and
   `split_bisector_side`.
10. **`side_code`'s datum claims `FaceShape::Plane`** for every face
    (`sectors::side_code`): it reads against the face's tangent plane,
    which is a plane, but the face may be curved. Before `side_code`
    descends (item 1), its datum must carry the face's own shape, or a
    conic against a curved face's tangent plane will be read through
    `conic_bend_side`, which is the plane's arm.
11. **`FaceShape::of_face` is still in `sectors.rs`**, beside the
    enum's home in `bound_side`.
12. **"Tangent" names two states**: `EntersMaterial::Tangent` is a
    tie (undecided at that order), `BoundSide::Tangent` a decided side
    one order down. Rename one.
13. **`entry_side`'s `unreachable!` charge arm**: a plane's datum
    carries no curvature charge, but `BoundSideFault::Charge` is still
    a variant `entry_side` has to answer. Give the plane datum a type
    with no charge arm.

## Coupled: ONEREAD's cone reader

ONEREAD's lane 2 (`the-boolean-reads-the-other-operand-through-one-cone`)
deletes `sectors.rs`'s `wedge_read`…`cone_side` block and moves
`sector_face` / `sector_shape` to a neutral home. `cone_side` and
`wedge_classes` read a bound's side through `side_code`, so through
`bound_side::read`: that lane deletes callers of the reading, not a copy
of it, and item 1's descent no longer has to reach `cone_side` once it
lands.

## Lane finding (2026-11-01, stage 2, PR 6253)

Item 3 (the vertex-pair charge) was built and backed out: charging the
vertex-pair pass as the pierce lane is charged refuses short arcs at a
crescent's tip that main builds valid. The measurement and what it
needs are on `vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces`,
which stays open. PR 6253's body lists what of items 1–13 landed and
where the rest is filed.

## Lane finding (2026-11-01, stage 2, PR 6253): the descent moves six built rows to refusals

Stage 2 built items 1, 2, 6, 7, 9 (bar `split_bisector_side`), 10, 11
(the sector callers), 12 and 13 (PR 6253 lists them), and the
outcome diff at the default ε (`cargo nextest run -p sweep -p topo`,
slow set included, main `e669a2f8` against the branch at `1ceb3cf6`)
stopped the lane at a point where the design, as specified, moves
bodies main builds valid to refusals.

**1. An in-band second-order tie escalates (the design's item 2, fourth
bullet).** Six rows that build on main refuse
`Escalated { Coincidence(TangentSide, Moot), "tangent_sector_order2" }`,
the parting of a bound main read `On` at first order landing in the
ambiguity band at its reach:

| row | the parting's margin |
|---|---|
| `germ_plane_torus_frame::a_band_pose_under_the_tubes_top_reaches_the_donuts_door` (δ = 1e-6, the one-face torus, T ∪ P) | −2.0e-9 |
| `near_circular_sections::a_near_square_rod_minus_a_box_builds` (r 1e-2, sin θ 4e-6) | −5.0e-9 |
| `near_circular_sections::a_turned_near_square_rod_minus_a_box_builds` (r 1, sin θ 6e-8) | −7.5e-9 |
| `pi_seam_and_kiss_through_the_boolean::a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders` (k = 0.5) | −1.0e-9 |
| `spheres_crossing_off_every_edge::a_trimmed_face_cut_beside_its_pole_builds` (δ 1e-5, a ∪ b) | 5.0e-9 |
| `tilted_sphere_pair::a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed` (δ 1e-4) | 9.6e-9 |

Each bound's first order already tied (`|d̂·n̂·l| ≤ zero`), and its
second-order separation at its reach is a few bands: it lies within the
escalation band of the face over its whole reach, which main read as
riding it, and built from. Measured, not landed: reading an in-band
parting (the reading rung of `parting_side`'s escalation) as `Rides`
builds all six again, every one passing its own checks. So the rule
"an in-band second-order tie escalates" is not landable with the
descent: either an in-band parting rides (the bound stays within the
band of the face to the orders read, which is `Rides`' own definition),
or these six move to refusals by design. That is a decision on the
design, not the lane's.

**2. A tangent flanker in the edge-on-sector rule (resolved on the
branch).** `pi_seam_and_kiss_through_the_boolean::the_stadium_answers_every_op`
went from building to `Join(SectionInvariant)` when both of the rule's
Table II keys (`recl::flank_key`) descended. PR 6219's narrowing is
restored: the keys descend only where both flankers tie, and one tied
flanker stays `On` for Table III's lump. The stadium builds again.

Also moved, refused to refused:
`a_sheath_grafted_on_its_blade::a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`
now stops earlier, `Escalated { ConicBend, "enters_material" }`, a rim
arc tied with the other lens's spline wall it lies in
(`ReadFault::Unbent`: a spline face's bend is not certified, the
design's item 2, fourth bullet), where main read it `On` and reached
the rim's description (`EdgeUndescribed`). That row's tripwire (the rim
certifying) can no longer fire.

And one refusal now builds:
`a_minted_vertex_on_a_curved_face` (two wedges sharing the wall's
tangent-plane face), every op in both orders at its closed form, tier 3
clean (TANG's `two-wedges-sharing-a-tangent-plane-face-on-a-ruling-refuse-unpaired-loose-ends`).
