---
id: test-files-copy-small-vector-helpers
kind: issue
title: Test files copy dot, cross and sub helpers over [f64; 3] instead of sharing one
status: open
opened: 2026-10-27
priority: P4
cost: E
---

Found by PR 5812's style review.

## What

A grep for `fn (dot|cross|sub)\(` under `crates/*/tests` (2026-10-27)
finds 23 test files, 20 of them with their own copies of the
vector helpers:

- `crates/sweep/tests/`: `band_turn_overrun_probe.rs:38,44,47`,
  `common/pinch_cones.rs:48,52`, `join_pierce_runs_sweep.rs:52,56`,
  `near_circular_sections.rs:112,149`, `pinch_faces_tessellate.rs:25,29`,
  `review_r2_vv_probes.rs:40,43`, `review_sixx_r2_probes.rs:47,50`,
  `ring_carry_through_by_piece.rs:25`, `split_cylindrical_feature_box.rs:44`;
- `crates/topo/tests/`: `contact7_touch_sweeps.rs:193,196`,
  `sliver_shell_role.rs:26,29`, `the_fin_crown_winds_as_a_solid.rs:38,42,46`,
  `union_flush_onto_edge_contact.rs:1450,1454,1458`;
- `crates/editor-core/tests/`: `emit_sphere_cut_in_cap.rs:63`,
  `name_invariance.rs:1232`, `review_gui1_r1.rs:147,150,157`,
  `review_pick3_r2_probes.rs:78,84,91`, `value_chart_sweeps.rs:81`;
- `crates/geom-brep/tests/props_cone_incidence.rs:31,34`;
- `crates/step-export/tests/orientation_oracle.rs:154,158,166`.

The other 3, in `crates/geom-core/tests/` (`exact_q/mod.rs:85`,
`interval_backend_differential.rs:295,322`, `ring2_r2_probes.rs:213`),
are `sub` methods of a scalar or backend type. They match the grep but
are not copies of the vector helpers.

## Owed

One home (`geom_core::Vec3`, which the files can already name, or a
shared test helper), and the copies gone.


## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). ONEREAD holds the boolean questions read by two homes; id and body unchanged. (CLEAVE orchestrator)
