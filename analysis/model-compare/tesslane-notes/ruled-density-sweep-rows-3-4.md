(4c implementer) Rows 3 and 4. Each was run alone (`--ignore-default-filter`, debug, default ε, no probe). Trees: the head 25083a00d0 (pulled), and origin/main 688eaa5f24 in the second worktree, each in its own target.

**4. `section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds`: red on BOTH, the identical failure. Main's.**
`section_end_poses.rs:288`:
`SquareBox t1e-9 ab U: got Some(Escalated { decision: Coincidence(Section, Moot), diag: Indeterminate { margin: MarginDiag(Value(1.0000000272292198e-9 …)), band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("curve_carrier_gap") } })`, the same for I and S.
It is a boolean coincidence escalation; no mesh is involved.

**3. `offer_rows::every_sphere_offer_passes_just_below_it` (sweep lib): red on the head, green on main 688eaa5f24, and RED on the head's merge base with main, 1076990327. Not this unit's: main fixed it after the head's base.**
- Head (`offer_rows.rs:333`): `false offers: nested_off_axis_in_the_zero_band_union: quotes None and offers Some(4.999e-11), computed margin 5e-10: … tighten the tolerance below 4.999e-11 m`.
- The merge base fails identically.
- Between 1076990327 and origin/main, `crates/sweep/src/offer_rows.rs` changed in its test comparison only (`4e5d60295a`, "the sphere offer row's evidence…"). The row's local `near` (1e-4 relative) is replaced by `test_utils::offer::near_margin` and `geom_core::test_support::is_offer_of_a_margin_within(design_band(), v, margin, margin_precision(margin))`, and `design_band()` by `default_k_band(DESIGN_EPS)`.
- This branch's diff against its merge base touches nothing in `sweep`, `topo`, `geom-brep` or `geom-core`. The failure is a sphere-sphere boolean offer, with no mesh call.
- **It goes green on the branch with the next merge of main, with nothing of this unit's involved.**

Row 5 (`germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`) starts now, on the head and then on main, one at a time. It took about 19 min in release-ish debug on my earlier runs, so expect about 20 to 40 min per tree. Note: it was red on both trees in my digest runs on 18 and 22 November, on main 54938d6a95 too.
