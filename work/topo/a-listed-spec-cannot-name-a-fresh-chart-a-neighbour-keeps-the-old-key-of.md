---
id: a-listed-spec-cannot-name-a-fresh-chart-a-neighbour-keeps-the-old-key-of
kind: issue
title: a listed re-description cannot name a fresh chart where the edge's other face keeps the key the moving face leaves, so a curved move onto a copy of its own surface has no recourse
status: open
opened: 2026-10-05
priority: P3
cost: M
refs: [boundary-on-the-new-chart-has-two-homes-in-the-attach-doors, validate-tier3-curved-boundary-containment, an-offset-door-restates-a-neighbour-chart-rim-the-describing-door-cannot-vouch-for]
---

## What

Found by `boundary-on-the-new-chart-has-two-homes-in-the-attach-doors`.
Every re-chart door now asks one question (`Body::unvouched`,
`crates/topo/src/attach.rs`): a certified edge on a moved face is
vouched for where its description names the chart the face moves onto,
or, at `Body::set_face_surfaces_describing` onto a plane, by its
residuals. Onto a curved chart no residual is read, so the describing
door refuses (`RechartUnvouched { door: SetFaceSurfacesDescribing, .. }`)
an edge no description names there, and its lever is "list a
re-description of each on the new chart".

That lever cannot be pulled when the edge's other face keeps the key the
moving face leaves. A listed spec names a `Rechart::new` chart by the
key the face wears now (`Sides::repoint`), and a key one of the edge's
faces still wears stands for itself. So for an edge between a face
moving onto a fresh key and a neighbour that stays on the old one, no
spelling of a spec names the fresh chart: the move has no recourse.

The constructions that hit it are honest ones, each now built through
`Body::lifting_rechart_refusals_for_tests` with a `// Lifts` comment:

- `crates/sweep/tests/curved_mergedoor.rs` `on_a_key_of_its_own`: a
  peg's wall sectors, each moved onto a fresh key holding the SAME
  cylinder; the generators between sectors name the key the neighbours
  keep.
- `crates/topo/tests/loop_reparenting_pcurve_rows.rs`
  `on_a_key_of_its_own` and two sites in
  `ring_move_and_mfkrh_carry_every_row_across_one_payload` /
  `a_swap_onto_an_equal_surface_on_another_key_reads_as_a_chart_change`:
  a panel onto a fresh key of an equal (or rotated) cylinder.

One refusal witness is an instance too:
`crates/topo/src/attach.rs`
`a_curved_chart_no_edge_names_is_refused_by_both_doors`, "the patch
through the cap". The membrane wears the cap's key (it was planted on
it), and its rim edges are images in the cap's chart. Moved onto a
NURBS patch that is the cap's plane, the boundary lies on the new
chart, but a listed rim can name only keys the body holds, the cap
among them, which the top keeps; `carried_redescriptions` returns
nothing. Probed: listing each rim edge restated (an image in the cap's
chart) is refused `RechartUnvouched` naming all four. So that
witness refuses a sound move with no recourse.

The production callers of the describing door that move onto curved
charts are the offset doors (`replace_face_offset` /
`replace_faces_offset`, `offset_charts_together`, through
`replace_face::move_points_then_rechart`): cylinder, cone and sphere
offsets, and the curved shell. They pass because they list a spec for
every boundary edge, and a restated spec naming the face's old key
reaches the minted chart through `Sides::repoint`. They do not reach
this row's shape: each mints one fresh chart per moved group, and
`replace_faces_offset` refuses `SharedSurfaceKey` before the door. The
offset doors' cousin, a restated spec naming a held neighbour's chart,
was
`an-offset-door-restates-a-neighbour-chart-rim-the-describing-door-cannot-vouch-for`,
closed by restating such an edge on the minted chart.
The join batteries do not move.

## Shapes a fix could take

- **An equal payload vouches.** A certificate is a function of the
  payload value it was taken on, so an edge naming a key whose surface
  is exactly equal to the new chart's is certified on it. That closes
  the same-surface cases (not the rotated cylinder, whose `u_ref`
  differs) at every door, and changes what
  `attach::tests::the_keys_only_door_moves_a_face_onto_the_key_its_edges_name`
  pins: a move onto a fresh copy of the plane the edges name is refused
  today because "a minted key is one no description names".
  `Body::same_chart` answers identity only, for rows, on purpose.
- **A spelling for the chart this call mints.** Let a listed spec name
  `Rechart` `i` directly, so the repoint rule is not the only way to
  reach a minted chart. An API change to `EdgeCurveSpec`'s keys or to
  the door's argument.
- **#638's curved residual**
  (`work/restfront/validate-tier3-curved-boundary-containment`): read
  curved residuals where a band is in hand, as the plane arm does.


