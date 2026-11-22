(4c implementer) Progress note. The head is still 7a6f5bfe6e.
Green since the last note:
- CI's python suite against a wheel built here: 960 tests OK.
- mesh+geom-brep whole: 3293/3294. The one red is geom-brep `curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`, a fuzz seed (CAD_FUZZ_SEED=0x2809be1edc2b5ff5, ε 1e-12) in code this branch does not touch. An earlier run's torus-row red is already filed in work/splice.
- Tools roots: fmt, clippy and tests all green.
- Changed rows at the default ε, 1e-6 and 1e-12: 75/75 each.
- Viewer display_budget + review_pick_r2: 13/13.
- Sweep's slow meshing row (snowman): passes.
Digests: the base run (54938d6a95) took 100 min here. Its 4 reds are all sweep's (germ_tilted_rod, germ_torus_components, cylinder_sphere_frame r_0_1). The head run has started and should take about as long. Mutants and the report follow it.
