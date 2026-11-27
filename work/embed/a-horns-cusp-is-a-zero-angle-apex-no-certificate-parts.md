---
id: a-horns-cusp-is-a-zero-angle-apex-no-certificate-parts
kind: issue
title: A horn's cusp is a zero-angle apex its sheet faces meet at tangent to the axis: the engine refuses every pair holding it, structurally
status: open
opened: 2026-11-25
priority: P2
cost: H
refs: [a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine, the-second-order-arm-certifies-a-declared-wedge-0-edge]
---


## Finding

Found by lane `embedwalk-spindle` (2026-11-25), giving a spindle sheet
its window and by-kind discharge in the injectivity engine.

A spindle sheet's apex is a conical point of half-angle
`atan(√(r² − R²)/|R|)` (`geom::Surface::Spindle`'s docs), zero on the
horn (`R = r`), whose two apexes are its one cusp. Two sheet faces
meeting there (the needle tip's two π-bands) meet tangent to the axis,
so no first-order separator parts a cell pair at the cusp at any depth.
At an earlier stage of PR 7769, a reading that took the cusp as the
one end the face's stored extent reaches, and read it as a collapsed
side, spent the whole budget (`Budget { stop: Pairs { max_cell_pairs:
65536 } }`, 60 s an ask).

At its head the needle's pair refuses at the root, with no split:
`Uncertified { a: 3v1, b: 4v1, shared: Edge(2v1), arm: Departure }`,
the two π-bands' shared meridian. It refuses the same whether
`topo::injective`'s `singular_ends` reads a vertex at both of a sheet's
apexes as no one end of the face's `v` (as at head) or as both ends,
so no cell beside the cusp is ever asked. That site is recorded as
reached by no built body
(`work/embedwalk/three-refusal-sites-of-the-embedding-ask-are-reached-by-no-built-body.md`).

Asks it refuses at the gate (2026-11-25): the needle tip
(`sweep::all the_football_revolves::a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip`,
two), the lower needle (`the_wide_apples_and_the_lower_needle_mesh_and_measure`,
three), a bullet's band closed by a rim
(`a_sheet_faces_boundary_encodes_its_side_where_a_rim_closes_it`, one),
and a horn's cusp revolved part way (`a_horns_cusp_revolved_part_way_is_its_lune`,
two). The part-way cusp's caps meet its sheet at wedge 0 as well
(`arm: SecondOrder`, four asks).

What would read it: a certificate of second order at a point, the
sheets' contact at the cusp being the meridian circle's tangency to the
axis; the edge's analogue is `the-second-order-arm-certifies-a-declared-wedge-0-edge`.
Pinned refused, never embedded: `sweep::all self_intersection_proves_injectivity::bodies_on_a_spindle_sheet_hold_by_kind_at_both_scalars`.

## Note (2026-11-25, LATHE, PR 7414)

On the one-band revolve the needle tip is one sheet face reaching the
cusp from one side, with no meridian shared between two faces, and the
injectivity ask **holds** at both scalars, its sheet face against itself
discharged by kind (`topo::injective`'s premise: a horn's face meets
itself only where it reaches the cusp from both sides, which the revolve
refuses). The refusal this row records was the two π-bands' shared
meridian (`arm: Departure`). The row
`self_intersection_proves_injectivity::bodies_on_a_spindle_sheet_hold_by_kind_at_both_scalars`
now pins the needle holding beside the lemon and the apple. The cusp
between two faces still has no certificate; a body that puts two faces
there is what this row is about.
