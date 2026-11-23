ISO-RUN-DENSITY — STAGE 1 REPORT, part 1/2 (the class, the reference, the table). STOPPED: building nothing further until you answer.

Probes pushed on tesslane/iso-run-density (env-gated, not for merge: crates/mesh/src/iso_probe.rs, switches in trimmed.rs/tessellate.rs, harness crates/mesh/tests/iso_density_probe.rs, #[ignore]). All numbers MEASURED in a debug build unless marked "read".

THE CLASS (measured by logging tessellate::route_of)
- The route's Trim::Read branch takes it, but Read is WIDER than the class: a cylinder wall bounded by a tilted section stores a Harmonic image (v a sinusoid), `is_non_iso_form` answers no for it, the shape door says "no rectangle", and it reaches Read too (m5_pr11's tilted walls; 348 such routings vs 106 iso ones in the mesh suite). The class is Read ∧ every outer-loop image has a still channel.
- Only two charts reach it: cylinder walls and sphere faces. No cone, no torus (as PR 7041 said).
- mesh suite: 7 tests reach the class (reach_probes notched wall + notched sphere rows, split_rulings' notched-wall row, the keyway in iso_rectangle_door and r2_mesh7_probes, two d9 golden rows). Sibling class the spec leaves out: holed walls (a ring, every loop iso) go Trim::Structural, same lattice, same headroom, and behave identically under every cure below.
- sweep + step-import: census still running (~1300/4848 when I wrote this); no class face seen in either so far. The tour is not yet censused. Final counts follow in a separate note.

THE REFERENCE
r_dec = the rectangle lane's own count (curved::grid_counts' formula at δ: 2·⌈Δu/h_u⌉ on a cylinder, 2·n_u·n_v on a sphere) summed over the iso-rectangles the face cuts into along constant-u (or constant-v, whichever is cheaper) lines through its corners. It is what the proven rectangle theorems give a face of this shape. It is computed inside the lane from the trim polygon. The other candidate (bounding-box count) is logged too; for a ruled wall it is ≤ r_dec, and area scaling is wrong for a cylinder, whose count is set by angle, not height. Caveats: pole fans are ignored (r_dec overcounts a little at a pole) and every piece's ceil rounds up, so ratios below 1 appear at coarse δ.

CORPUS: notched pipe walls (a bar half in across the seam, ∪ and ∖; notches down from the rim, 3 widths × 3 depths, across or beside the seam; two notches), slender (r 0.1), squat (h 0.4) and large (r 10) walls; holed walls; a ball less a quarter-turn wedge through the south pole, through the north pole, beside the poles (that one becomes a ring), and to below the equator; small and large balls. Every cell is run at axis and oblique placement (identical counts: the chart moves rigidly), δ 0.1/0.03/0.01/0.003/0.001, at ε 1e-9, 1e-6 and 1e-12. The three ε rows are IDENTICAL in every column.

TABLE: triangles of the class faces / r_dec, min/median/max, over (cylinder walls 210 cells | sphere faces 60 cells). check_mesh and the winding check are clean in EVERY cell that meshes, for every cure.
| cure | cyl meshes | cyl ratio | sph meshes | sph ratio | worst cert/δ | worst sampled dev/δ | passes | lane secs* |
| today (headroom, lattice) | 210/210 | 1.2 / 22 / 223 (slender) | 60/60 | 1.65 / 1.96 / 4.4 | 0.75 | 0.75 | 1 | 4.8 |
| δ only | 192/210 (CertExceeded at δ 1e-2..1e-3) | 1.0 / 11 / 115 | 60/60 | 0.75 / 0.95 / 3.0 | 0.998 | 0.998 | 1 | 2.5 |
| δ + rows off, NO refine (control) | 196/210 (CertExceeded, incl. δ 0.1) | 1.00 / 1.02 / 1.17 | 60/60 | as above | 0.998 | 0.998 | 1 | 1.1 |
| 1a: δ + rows off + refine, centroid | 210/210 | 1.00 / 1.02 / 1.40 | 60/60 | 0.75 / 0.95 / 3.0 | 0.998 | 0.998 | ≤2 (one round) | 1.25 |
| 1b: δ + rows off + refine, widest-edge split | 210/210 | 1.00 / 1.02 / 1.20 | 60/60 | 0.75 / 0.95 / 3.0 | 0.998 | 0.998 | ≤2 | 1.56 |
| 2: 1a + held iso lines through corners | 210/210 | = 1a | 60/60 | 0.83 / 0.98 / 3.2 | 0.995 | 0.995 | ≤2 | 2.0 |
| 4: aligned lattice (lines through every run) at δ, rows kept | 210/210 | 1.5 / 13 / 120 | 60/60 | 0.86 / 0.99 / 1.02 | 0.998 | 0.998 | ≤2 | 2.4 |
| 4 + rows off + edge refine | = 1b on cylinders (with rows off a cylinder has no lattice to align) | | = 4 on spheres | | | | | 1.4 |
| 3: iso-rectangle decomposition | not built; its count IS r_dec by construction | | | | | | | |
*Lane wall time summed over the corpus's chart-lane faces at the default ε: debug build on a box shared with the census, so read it as relative only. "passes" counts attempts; 2 = one refinement round.
Refinement: under 1a and 1b, every refusing cell is cleared by ONE round, at every ε. Unlike 4c, no cell needs more: no split side fans a vertex across columns here.
Neighbours: summed over 310 cells (1b against today), triangles on the faces sharing the class faces' rims fall 266,860 → 259,640 (−2.7%), because the rims are chorded at δ instead of δ/2. No cell's neighbours gain a triangle. The class faces themselves fall 924,920 → 219,900.
(part 2: recommendation and doubts)
