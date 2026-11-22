(4c implementer) **Mutants on e760867739.** Filter: split_rulings, route_rows, errors, ruled_walk, sizing, reach_probes (91 rows, the `--lib` units included). Each is red under its own rows:
- **zip off: 4.**
  - `--lib`: `ruled_walk::tests::a_one_chord_face_to_the_apex_zips_its_sides` (1).
  - split_rulings (3): `a_one_chord_face_to_the_apex_split_beside_both_rim_ends_meshes`, `a_one_chord_face_through_the_apex_with_both_sides_split_meshes`, `no_walked_triangle_has_an_edge_a_neighbour_makes`.
- **cut never: 4.** The bored-ball row reds by **`OverusedSide { uses: 4 }`** (first cell: ball 1.05, sign +1, δ 3, "meshes, got OverusedSide"), so the census is the backstop. The others: `no_walked_triangle_has_an_edge_a_neighbour_makes` (its pinned band cuts), `a_side_whose_reach_the_band_cannot_decide_is_cut`, and `route_rows::a_split_side_body_records_the_walks_decisions`.
- **cut always: 3.** The bored-ball row (its uncut cells), the undecided-reach row, and the K row.
- **route's walked fact off: 11.**
  - split_rulings: `a_split_beside_a_large_cones_rim_end_meshes`, `a_thin_d_prism…`, `a_ruling_split_a_hair_from_its_end_meshes`, `a_cone_band_shorter…`, `a_side_whose_reach…`, the bored ball, `a_ruling_split_within_two_to_the_minus_27…`, the 218-cell row, `a_split_side_adds_one_triangle_per_point`, `a_one_chord_face_through_the_apex…`.
  - route_rows: the K row.
- **unmarked face reaching the walk: 22.** 17 reach_probes rows (`crossing_rods…`, `a_rod_sliver…` and `a_ball_less_a_half_space…` among them); route_rows: main's K stream and bytes; split_rulings: the notched wall and the needle row.
- **undecided read as greater: 1**, `a_side_whose_reach_the_band_cannot_decide_is_cut`.

**The 24 D-prism cells +2 against main** (0.5 rad at δ 0.1, 0.2 rad at δ 0.03, the flat face holding both sides): these are not cuts.
- The 218-cell row pins 0 cuts on every D-prism cell, and with the cut off the row passes on every one of them, so the census needs none.
- The +2 is the walk chording the rim as the unsplit face is chorded (no headroom), against main's chart lane; it is not a walked cut.

Next: the binding census against a wheel, the viewer's slow rows, sweep's meshing rows, and the `mesh` digest against 8ad2d3fd0f.
