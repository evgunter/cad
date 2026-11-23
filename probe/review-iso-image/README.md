# Review probes for PR 7393 at a8b51fc7 (merge base e1830a1c)

- `crates/geom-brep/tests/review_iso_image_probes.rs`: certify-level bodies through
  `Pcurve::iso_row`/`iso_column`; compiles on both trees. Outputs:
  `certify-bodies-head.txt`, `certify-bodies-merge-base.txt` (identical line for line).
- `crates/topo/src/pcurves/review_iso_image_probes.rs`: `Lift::stored_corners`' far end
  on a row lifted one period; `Period::moved` at `Dual`.
- `crates/sweep/tests/mass_props_are_thread_count_invariant.rs`
  (`review_probe_the_validation_doors_decisions_by_predicate`): `decision-tally.txt`.
- `merge-base-probes.patch`: the merge-base-only diagonal `IsoLine` against its `General`
  twins (`diagonal-merge-base.txt`), and the tally probe.

Mutants planted one at a time on the head (not committed):
- M1 `schedule_residuals` returns before its loop: `an_image_whose_carrier_misses_its_edges_end_is_refused` red; no named probe body becomes accepted.
- M2 the line limb's hull not folded: `a_map_past_its_carrier_in_band_is_in_the_envelope` red.
- M3 `Period::moved` always adds: `a_period_moved_seam_column_is_its_other_end`, `a_stored_corner_undoes_the_rows_own_deck`, `quad_lane` `a_seam_column_moved_one_period_is_the_domains_other_end` red.
- M4 `Lift::joint_point` never reads the meet: `joint_point_rows` both green; `quad_lane` `an_entry_short_of_its_corner_adds_its_gap` red.
- M4b `Lift::exit_point` never reads the meet: `a_corner_is_the_meet_and_a_cut_is_the_maps_value` red.
