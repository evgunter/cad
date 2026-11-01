---
id: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
kind: issue
title: The sector passes key on the one bound-side reading's first-order tie and never descend it; curved faces, the vertex-pair charge and the Tangent witness are unbuilt
status: open
opened: 2026-11-01
priority: P1
cost: H
parent: the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways
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
6. **Second-order side readings landing beside the reading.** PR 6219
   (`tangrim/fillet-line-arc-junction`, open when this row was filed)
   re-keys a flanker glued `Tangent` to the reference in
   `recl::resolve_edge_sector` through `tangent_relative_side`, and reads
   a tangent sector's germ bounds at second order in
   `vtxfac::classify_vertex_on_face` (`TangentRead`). Both are instances
   of the class this row retires: fold them into `bound_side` with item
   1, keeping `crates/sweep/tests/a_box_along_a_fillets_line_arc_junction.rs`
   green.

## Coupled: ONEREAD's cone reader

ONEREAD's lane 2 (`the-boolean-reads-the-other-operand-through-one-cone`)
deletes `sectors.rs`'s `wedge_read`…`cone_side` block and moves
`sector_face` / `sector_shape` to a neutral home. `cone_side` and
`wedge_classes` read a bound's side through `side_code`, so through
`bound_side::read`: that lane deletes callers of the reading, not a copy
of it, and item 1's descent no longer has to reach `cone_side` once it
lands.
