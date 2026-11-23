(join-split implementer) (2), the `sweep` half: the meshing modules at the default ε, seed 1. All three runs give 933/934, the only red being main's known `bisector_nodes::the_near_node_poses_at_the_witness_band`. 3,617 calls each.

**Foot alone, against the head:**
- 42 calls move points; triangle counts and outcomes are unchanged.
- Total triangles: 8,087,070 on the head, 8,087,070 with the foot.
- Where: bisector_nodes (37 calls in 7 rows: rod against a bore, seam from twice root band, edge near a node, oblique seam, equal cylinders with meeting axes, slanted bores, family re-posed) and snowman::a_cut_ends_at… (5).

**Foot + pair, against foot alone: +176 triangles of 8,087,070 (+0.002%).** 11 calls move, no outcome flips, no refusals.
- snowman::a_cut_ends_at_the_nearest_of_several_boundary_hits: 10 calls, +168. The cap is 12 → 32 at δ 0.04/0.02/0.01, 16 → 44 at δ 0.005, 18 → 50 at δ 0.004, and 28 → 32 / 24 → 44 / 34 → 50 on the other latitude.
  - The head's 5 nonpositive calls in `sweep` are exactly this row's lat −60 cells. **Under foot + pair, `sweep` has 0 nonpositive calls.**
- **The one body outside the thin-cap class:** bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself, 1 call at δ 2e-3, 7,764 → 7,772 (+8, +0.10%). By its name and fixture it is a difference pinched to itself, two faces meeting at a pinch, so closer than δ near it.

**Condition (c), both suites, foot + pair against foot alone:**
- `mesh`: +0.012%.
- `sweep`: +0.002%.
- Every body it fires on is a fixture with two faces closer than δ. 82 of the 83 `mesh` calls and 10 of the 11 `sweep` calls gain 14% to 178% per call; the remaining one in each gains under 1% (slanted bores +0.10%).
- **No body without two faces closer than δ moved.**
- I did not run the centroid variant over `sweep`. It fired on nothing in `mesh` (13,998 calls identical), the class table or the bored ball, so it adds nothing measured.

**The PR's moved list for foot + pair**, against the head:
- `mesh`: the 11 rows above.
- `sweep`: the snowman cap row, the 7 bisector_nodes rows (points only) and the slanted-bores call (+8).

Pins that move, all by design:
- `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`: cut counts.
- `split_rulings::the_join_splits_the_same_at_every_eps`: mesh bytes.
- d9 goldens: not red in these runs. I'll confirm in the build.
- `the_eps_inventory_is_pinned` red only because of my probe plumbing.

Running now: (3), the class table for foot + pair at ε 1e-6 and 1e-12, with the base cells for the A1 bodies at those ε.
