---
id: the-lane-free-sign-guard-has-no-witness-of-a-misrounded-sum
kind: issue
title: The lane-free volume-sign guard has no witness of a sum whose rounding exceeds the volume, now that faces are read at the face
status: open
opened: 2026-11-11
---


Found closing ONEDOOR's face-anchored measure (PR 6505, fix pass 2).
`topo`'s `props::rounding_bound` and the lane-free role read in
`props::shell_volume_role` exist so that a lane-free read never takes a
rounded sum's sign for the volume's. Their witness was
`tier3_tests::far_anchored_slab` (records anchored 5 km off): the
`f64` walk read the +1e-13 m³ slab as −1.2e-12 m³. Each face is now
read at the face (`crates/geom-brep/README.md`
`[geom-brep.face-anchored]`), so the walk reads the slab at its exact
volume and the three rows that leaned on the misread
(`a_lane_free_read_never_takes_the_rounded_sums_sign`,
`a_sign_is_read_off_the_exact_volume_not_its_rounding`,
`the_census_gate_reads_a_shell_role_as_the_classification_does`) now
assert the right roles instead. The guard path itself is unchanged and
untested by a misrounded sum.

Wanted: a body whose face-anchored `f64` sum still rounds past its
volume (a very thin body whose faces are large against the shell
reference's distance, say), pinned as the guard's witness, or a
statement of why no body at rest within the session box can have one.
