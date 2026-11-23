ISO-RUN-DENSITY: the classification (I never saw a 03:21Z note; this answers the one just received). Paths and sets READ from .config/nextest.toml at this branch (= PR 7096's tree for that file). ALL FIVE are in the default-filter exclusion, so they are SLOW SET and 7096's hosted PR run does not answer any of them:
1. step-import::all wild::wild_refusals_are_typed_and_name_their_class (nextest.toml l.459)
2. step-import::all tier_gate::every_corpus_import_passes_the_shared_gate (l.318)
3. sweep offer_rows::every_sphere_offer_passes_just_below_it (binary `sweep`, the lib unit tests; l.437)
4. sweep::all section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds (l.393)
5. sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs (l.355 and l.564; it took 2,321 s in the census run)
Two more failed in the same census run, also slow set: step-import snap::tests::nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them (l.317) and sweep::all bisector_nodes::the_near_node_poses_at_the_witness_band (l.351).
Caveat: the census tree was 7096's head plus a route-logging probe that changes no behaviour. It ran under heavy contention, so a timing-sensitive row could fail that way, but none of these failures was a timeout.
The box is held: mesh at 1e-12 on my head, a base sweep run, and two short probes. I am starting the alone-runs now on two fresh worktrees (origin/tesslane/ruled-density and origin/main), one at a time with their own target dirs. germ_tilted_rod will take long. I will send red/green with first failure lines as each lands.
