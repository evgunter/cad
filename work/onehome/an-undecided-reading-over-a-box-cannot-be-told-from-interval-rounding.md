---
id: an-undecided-reading-over-a-box-cannot-be-told-from-interval-rounding
kind: issue
title: An undecided reading over a parameter box cannot be told from Interval's own rounding, so its ending cannot name the box
status: open
opened: 2026-11-12
priority: P3
cost: M
---


## What

A later finish of a boxed move (`crates/sweep/tests/a_boxed_motion_moves_a_finished_body.rs`, `a_later_finish_of_a_boxed_move_reports_only_undecided_findings`) reads every reading over the box, and some of its undecided findings end in a check's own lever, not in the box scalars' recourse (`geom_brep::recourse::BOX_SCALAR_RECOURSE`, "narrow the box, or evaluate at the symbolic scalar"):

- **`Pcurve::Escalated`** (the general lane's `pcurve_row_interval`) ends "There is no way through yet": `crates/topo/src/validate.rs` `classify_pcurve`, through `geom_brep::recourse::not_yet`.
- **`Pcurve` / `IncidenceRefusal::Undecided`** ends "Recourse: move the geometry so the boundary lies clearly on the face it bounds at this tolerance": `crates/geom-brep/src/pcurve_cache.rs` `IncidenceRead::ending`, `INCIDENCE_UNSHOWN`.
- **`PlanarFaceEscalated` / `PlanarBoundaryEscalated`** name no ending without a door: `crates/topo/src/validate.rs` `incidence_ending`.

ONEHOME's review of PR 6820 asked for each to end in the box's recourse.

## Why it was not done in PR 6820

The only signal at a finding is its margin's kind, `geom_core::MarginKind::Enclosure`, and plain `Interval` arithmetic at a POINT reads every margin as an enclosure too, at rounding width. Routing on it was tried and reverted. It turned these rows red, each pinning a check's own lever on an `Interval` read at a point, mid-band, where there is no box to narrow:

- `crates/geom-brep/tests/chart_incidence.rs`'s `a_wrong_radius_circle_on_each_class_arm_ends_as_its_incidence_read`;
- `crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs`'s `a_lane_choices_envelope_refusal_ends_not_yet` and `a_stored_rows_envelope_refusal_keeps_its_term`;
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz/projected.rs`'s `a_spline_net_the_projected_lane_cannot_partition_ends_not_yet`;
- `crates/sweep/tests/a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest.rs`'s `..._at_interval`.

Fork 259 item 2 kept "a check's own lever" unchanged past a build for the same reason (`crates/geom-brep/src/recourse.rs` `Unsized::recourse`, which routes only decisions with no lever of their own).

The row now asserts each finding's own ending, so a new ending fails there.

The same review asked that the STEP import door's undecided last resort name the file, not a kernel bug. That was tried and reverted too. `crates/topo/src/test_support_samples.rs`'s `every_certify_refusal_ends_alike_at_rest_and_at_the_import_door` and `crates/geom-brep/src/certify.rs`'s `the_import_door_reads_at_rest_with_its_eps_in_words` pin that the import door ends as rest does, apart from ε_in's words and the defect ending's culprit (D4 ¶1). Naming the file in the last resort changes that rule, which is a ruling, not a re-wording.

## What would close it

A finding that knows it was read over a parameter box, not one that infers it from the margin's kind. For example, the session box's presence carried on the reading, or the box scalars' reading door distinct from `Interval` at a point. Then the box recourse can replace a check's lever exactly where a box is the cause. For the import door, a ruling on whether its last resort names the file.
