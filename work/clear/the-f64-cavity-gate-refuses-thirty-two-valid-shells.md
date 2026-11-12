---
id: the-f64-cavity-gate-refuses-thirty-two-valid-shells
kind: issue
title: The shell's cavity gate asked at f64 refuses 32 valid shell rows the engine cannot certify: 22 out of pairs, 8 uncertified on the departure arm, 2 on a refused carrier
status: open
opened: 2026-11-12
priority: P0
cost: H
refs: [the-at-rest-gate-certifies-embedding, the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies, departure-reads-no-curved-corner-off-its-shared-edge]
---


Found by CLEAR's lane A of `the-at-rest-gate-certifies-embedding` (PR
6885). Run locally under `--profile ci` and on hosted CI; the two agree.

With the shell's cavity gate on the verdict door
(`topo::clearance::embedded`, called from `topo::shell`
`cavity_embedded`), the gate runs at f64. 34 shell rows that run at
f64 only turn red. Two are true crossings. The other 32 are valid
bodies that the engine refuses:

- **22 `Budget(Pairs { max_cell_pairs: 65536 })`:**
  - `cross_solid_apart::a_hollow_shelled_again_passes_the_census`;
  - `shell10_r2_probes`' two e2e rows;
  - `shell7_r2_probes::e2e_a_consumer_shells_classifies_measures_and_tessellates_tori`;
  - `shell7_seam_corner::a_two_arc_sphere_shells_to_its_closed_form`;
  - `shell8_r1_probes::r1_e2e_two_parts_one_body`, and `shell8_r2_probes`'
    consumer seat and three-solids rows;
  - `shell9_probe::sphere_grafted_body_is_tier_3_valid_before_and_after_the_closing_mint_which_shell_runs`;
  - `shell9_r2_probes::r2_e2e_two_arc_sphere_hollows_and_tessellates`;
  - six `shell_curved_mouth` rows: the bulging cap, the dome short of
    tangent, the spherical cap, the tangent dome, the void's cap, and
    opening a tangent dome;
  - `torax_axial::torax_the_torus_corners_survive_a_rigid_re_pose`;
  - `sf2b_axial::a_torus_belly_and_a_tangent_bullet_hollow_through_the_axial_door`;
  - `verbs_shell`'s full-period torus and its two record rows;
  - `editor-core`'s `lib_g17_shell_node::the_capped_vessel_opens_its_cap_into_a_seamed_band`.
- **8 `Uncertified`, arm `Departure`:**
  - `shell9_r1_probes::r1_drum_reverted_cavity_alone`;
  - `shell_curved_mouth::a_sphere_window_opens_to_a_ring_and_passes_tier_3`;
  - `spiric_rim::the_sectioned_vessel_hollows_to_its_measured_volume`;
  - the three `torax_axial` sphere-lune rows;
  - `shell7_seam_corner::a_collinear_cap_vertex_drum_shells_to_its_closed_form`;
  - `verbs_shell::a_dome_sectors_arc_bounded_end_walls_clear`.
- **2 refused at a window** ("a wall is a face whose carrier…"):
  `sf2b_r2_probes::r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges`
  and `shell7_seam_corner::a_three_quarter_turn_cone_frustum_hollows`.

The two true crossings:
- `shell5_r2_probes::r2_a_thin_curved_wall_shells_silently_into_crossing_walls`
  pinned the silent wrong shell, and it now refuses `CavityCrosses`;
- `a_move_through_a_neighbour_inverts_the_body::the_shell_verb_refuses_the_crossing_at_its_closing_gate`
  is refused `CavityCrosses` before its closing gate.

These are the engine limits of the at-rest gate row's table 1: the
pair budget on spheres, domes and tori, and the departure floor. The
row sequenced the engine rows before lane C, so that the gate would
not refuse valid bodies. It did not sequence lane A's f64 switch for
the shell, which has the same exposure. Not measured: the tour's shell
scenes (nightly) and their wall time at f64.

**Done when** every row above shells at f64 with the gate on, or is
re-pinned to the refusal it should give. Until then, whether the shell
asks at f64 is the orchestrator's call on PR 6885.

