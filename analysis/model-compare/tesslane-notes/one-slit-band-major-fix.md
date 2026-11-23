one-slit-band MAJOR fix: commit d0e59643b8, pushed (head confirmed with ls-remote).

One part of the ruling is wrong, so I say it first. "On a sphere that aspect is within √2 by construction, so k = 1, no sphere byte moves" does not hold. The counts' ceilings move a curved sphere face's realized cell aspect off √2. Over 63 sphere faces with rows (zones and caps, 23 bodies × 5 δ), raw aspect runs 0.42 to 2.4. So `cell_metric` rounds to k = 2 or 1/2 on many of them, and those patches move. I implemented the rule as ruled. If you want zero sphere motion instead, the curved lane would have to read k = 1 on the sphere arm: one line, but a second rule. Say if you want it.

WHAT CHANGED
- `chart_table.rs`: `AnalyticRow::metric` is split in two.
  - `cell_metric`: the lattice's realized aspect `(Δu/nu)/(Δv/nv)` to the nearest power of two, on every chart.
  - `trimmed_metric`: the sphere's chart-own mid-latitude choice, else `cell_metric`. Its doc keeps the shared-side-cure reason.
- `trimmed.rs` calls `trimmed_metric`. Its bytes do not move.
- `curved.rs`: a new `cdt_metric(row, box, counts)` returns `cell_metric`, and the lane calls it. The module doc now names it and says it is never the trimmed lane's sphere choice. The sentence "within √2 of one" is now true on every chart, and the doc ties it to `sphere_grid_steps`' lemma premise: aspect under 2√2 on either axis. Spindle: `cell_metric` gives its lattice's realized aspect (same arm as the torus), so yes, it is its lattice's.

NEW ROW: `curved::tests::the_cdt_cell_keeps_the_containment_lemmas_aspect_on_every_chart`
- Built from 10 synthetic sphere windows (zones off the equator, caps to the pole, the whole sphere) and a thin torus band (0.3, 1e-3), at 6 δ, through the lane's own `grid_counts` and `cdt_metric`.
- It asserts that each cell enters the CDT at an aspect in (1/2√2, 2√2), the lemma's premise on either axis.
- Anti-vacuity: more than 40 faces with rows, and the thin band's raw aspect lies outside the bound.
- Mutant "the curved lane reads `trimmed_metric`": red on this row. The one-slit rows stay green under it, so this row is what catches it.
- Mutant "metric 1": red on this row and on the three one-slit rows.

The `the_one_slit_band.rs` sphere row keeps its relation (meshes within δ by sampling). Its doc no longer claims k = 2 from mid latitude. It cannot pin byte-identity to the base, because the base bytes move under the rule.

WHAT MOVED, base (curved metric = 1) vs head
- 23 bodies × 5 δ: 515 patches at both. 52 moved, all with equal triangle counts:
  - donut torus: 10, as before;
  - curved sphere: 42 (caps at 30/60/80°, zones 0–30 … 70–89°, sphere_wedge 1.2 and π/4);
  - planes, cylinders and cones: none;
  - ball: none.
- `d9_mesh_goldens`: sphere_wedge at δ 0.15 moves, from 70f42f79af650fee to d0e17c1a5ff99fee. That old value is the committed constant, which the base reads, so the move is this change's. Re-pinned. The donut pins are as before.
- viewer `review_pick_r2`: head still reads 147,844, and the per-document diff against the base is still only tube_arc (11901→11898) and hollow_tube_elbow (7052→7049). No sphere landing moved a graze. The pin is unchanged.
- viewer `display_budget`: green.
- sweep's 65 meshing modules: 601/602. The one red is main's `bisector_nodes::the_near_node_poses_at_the_witness_band`.
- mesh, the whole suite with the slow set: green after the re-pin. The new and re-pinned rows also pass at ε 1e-9, 1e-6 and 1e-12.

PR body edits
- In "Bytes that move" replace the sphere-symmetric row with: "curved sphere faces whose realized cell aspect rounds to a power of two other than 1 (42 of the table's sphere patches; sphere_wedge at δ 0.15 in d9_mesh_goldens): moved, counts equal; their cells now enter the CDT within √2 of square, inside sphere_grid_steps' lemma premise, which raw UV also met (aspect ≤ 2.4 with rows, measured)". Drop the "zones at metric 2" paragraph.
- Add the d9 sphere_wedge re-pin to the re-pin list.
- Add the new row and its mutant to "Rows" and "Mutants".
