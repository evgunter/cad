---
id: the-f64-cavity-gate-refuses-thirty-two-valid-shells
kind: issue
title: The shell's cavity gate asks the verdict door at f64, Probe and Sym<f64> — the switch waits on the engine rows, since asked today it refuses 32 valid shell rows (22 out of pairs, 8 uncertified on the departure arm, 2 on a refused carrier)
status: open
opened: 2026-11-12
priority: P0
cost: E
refs: [the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies, the-certificate-refuses-a-plane-cap-on-a-torus-or-cone-wall-at-their-circle, departure-reads-no-curved-corner-off-its-shared-edge, the-at-rest-gate-certifies-embedding]
---

Split out of lane A of `the-at-rest-gate-certifies-embedding` (PR
6885), on the CLEAR orchestrator's ruling that the spec's ordering
covers this switch too: no gate that refuses valid bodies lands ahead
of the engine rows.

**The switch.** `topo::shell` `cavity_embedded` asks the verdict door
(`topo::clearance::embedded`) only where `CavityGate::at` answers
`Asks`, which is where the clearance measure answers: `Interval` and
`Sym<Interval>`. At `f64`, `Probe` and `Sym<f64>` it answers
`WaitsOnTheEngineRows`, and there a curved crossing still shells
silently wrong. The door itself answers at every certifying scalar; the
switch is this unit. When it lands:
- `CavityGate` goes, and `cavity_embedded` asks at every scalar that
  shells;
- `shell_cavity_embedded`'s coaxial row gains its f64 half: the vessel
  with a coaxial void refuses `CavityCrosses` at f64, on the Interval
  row's operand faces;
- its curved dumbbell (the arc-necked fixture, bulge −0.2, `t = 0.15`)
  gains its f64 half: it refuses `CavityCrosses`, the witness on the
  two neck walls. Measured on PR 6885's branch;
- the mutant "the shell's gate skipped at f64" reds both of those
  rows. The door's own f64 mutant already dies on
  `clearance::wiring_rows::the_verdict_door_answers_at_every_certifying_scalar`.

**What it will catch.** Two rows that pin main's behaviour turn into
refusals, and their pins move with the switch:
- `shell5_r2_probes::r2_a_thin_curved_wall_shells_silently_into_crossing_walls`
  pins the silent wrong shell, and will refuse `CavityCrosses`;
- `a_move_through_a_neighbour_inverts_the_body::the_shell_verb_refuses_the_crossing_at_its_closing_gate`
  will be refused `CavityCrosses` before its closing gate.

**What it waits on.** Asked at f64 on PR 6885's branch (local
`--profile ci` and hosted CI agree), the gate refused 32 valid bodies
that run at f64 only. They are the engine limits of the at-rest gate
row's table 1:

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

**Done when** every row above shells at f64 with the gate asking, and
the switch, the two f64 pins and the mutant land, with the two
crossings re-pinned to their refusals.

## Measured on main `2b68e8551c` (CLEAR, lane clear-corner2, 2026-11-13)

After PRs 6886, 6892, 6894 and 6902, the 32 rows asked at f64 through
the verdict door, with a local, uncommitted switch in `CavityGate::at`
(the rows' whole test modules, local `nextest`, opt 1). **15 now shell;
17 still refuse:**

- **9 `Budget(Pairs { max_cell_pairs: 65536 })`:**
  - `cross_solid_apart::a_hollow_shelled_again_passes_the_census`;
  - `shell7_r2_probes::e2e_a_consumer_shells_classifies_measures_and_tessellates_tori`;
  - `shell7_seam_corner::a_two_arc_sphere_shells_to_its_closed_form`;
  - `shell9_probe::sphere_grafted_body_is_tier_3_valid_before_and_after_the_closing_mint_which_shell_runs`;
  - `shell9_r2_probes::r2_e2e_two_arc_sphere_hollows_and_tessellates`;
  - three `shell_curved_mouth` rows: the dome short of tangent, the
    tangent dome, and opening a tangent dome;
  - `verbs_shell::the_full_period_torus_shells_solid_and_hollow_alike`.
- **6 `Uncertified`, arm `Departure`:**
  - the two drum rows, `shell9_r1_probes::r1_drum_reverted_cavity_alone`
    and `shell7_seam_corner::a_collinear_cap_vertex_drum_shells_to_its_closed_form`;
  - the three `torax_axial` sphere-lune rows;
  - `torax_axial::torax_the_torus_corners_survive_a_rigid_re_pose`,
    counted out of pairs above; it now refuses on the departure arm
    instead.
- **2 refused at a window:** the conical wedge and the three-quarter
  cone frustum, as above.

The 15 that shell: `shell10_r2_probes`' two e2e rows,
`shell8_r1_probes::r1_e2e_two_parts_one_body`, `shell8_r2_probes`'
consumer seat and three-solids rows, `shell_curved_mouth`'s bulging
cap, spherical cap, void's cap and sphere window,
`sf2b_axial::a_torus_belly_and_a_tangent_bullet_hollow_through_the_axial_door`,
`verbs_shell`'s two record rows and its dome sectors, `spiric_rim`'s
sectioned vessel, and `editor-core`'s capped vessel.

`departure-reads-no-curved-corner-off-its-shared-edge` traces the six
departure rows. Its lens reading makes two of the three lune rows shell
(`torax_the_sphere_lune_hollows_to_its_closed_form` and
`torax_the_sphere_lune_lifts_its_rims_onto_the_caps`), so on that
change 15 rows still refuse: 9 out of pairs, 4 on the departure arm
(the two drums, the two-arc lune, the torus belly re-posed), 2 at a
window.

## Carried by the switch (review of PR 6885)

- At `Probe` the switch puts the cavity sweep's samples into the K
  population (audit row F41). Run the k-probe sweep with it.
- The valid-shell rows of `shell_cavity_embedded.rs` (`holds`, `shells`)
  lost their `gap > 0` assertions in PR 6885, so the pot, the frustum,
  the Klein elbow and the tube at `Sym<Interval>` no longer show the
  gate ran on them. A gate skipped everywhere still dies on the vessel
  crossing rows. Put "the gate ran" back on them with the switch.
- `min_separation`'s F41 scope has no pin (no production caller reads
  it).
