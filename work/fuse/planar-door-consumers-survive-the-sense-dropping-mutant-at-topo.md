---
id: planar-door-consumers-survive-the-sense-dropping-mutant-at-topo
kind: issue
title: five consumers of the planar outward-normal door survive a per-site sense-dropping mutant under -p topo — reduce's face_plane, solid_contain's face_plane and face_geo, census's star_face, shell's planar_faces
status: open
opened: 2026-10-02
priority: P3
cost: M
---

## Finding

The per-site form of the mutant in
`declared-opposite-orientation-refusal-is-unreached-by-any-row`: at ONE
caller of the planar door (`face_normal::plane_outward_normal` or the
keyed `face_normal::face_outward_normal`), fold `true` for the sense
and keep every other caller honest. Run as
`cargo nextest run -p topo` (2040 rows, 111 skipped), with that unit's
reversed-face rows in the tree. Sites where the mutant survives (no
row goes red):

- `crates/topo/src/boolean/reduce.rs` `face_plane` (the keyed door).
- `crates/topo/src/boolean/solid_contain.rs` `face_plane` (the
  point-in-solid plane door).
- `crates/topo/src/boolean/solid_contain.rs` `face_geo`, the
  `FaceGeo::Plane` arm.
- `crates/topo/src/census.rs` `star_face` (the keyed door).
- `crates/topo/src/shell.rs` `planar_faces`.
- `crates/topo/src/merge_faces.rs` `merged_outline_ring`: survives
  `-p topo`, but the whole-door mutant's wider run
  (`-p topo -p sweep -p mesh -p editor-core`) reds it through
  sweep's `verbs_1031b_arcwind` suite, so it is guarded outside the
  crate.

The four sites above it were not red in that wider run either (its
red list is check 6's, check 9's and `merged_outline_ring`'s), so the
five survive at least `topo`, `sweep`, `mesh` and `editor-core`. Not
run per site: the other crates, and the nightly's eps rows.

For contrast, the sites a row now reds: `merge_faces.rs`
`planes_declared_equal` (`m3_pr1_surgery`'s two declared-pair rows,
and `m3_pr5_boolean_ops::stacked_union_merges_a_side_wall_…`),
`boolean/rest.rs` `face_carrier`
(`stacked_union_rests_on_a_contact_face_…`), `boolean/join.rs`
`ring_run_ccw` (`pocket_subtract_into_a_top_…`), `sector_face.rs`
(both stacked rows), `validate.rs` check 6, and `face_normal.rs`'s
`face_outward_normal_at` plane arm.

## What is owed

A row per site, or a stated reason a site cannot read a reversed face.
The fixture is cheap now: `m3_pr5_boolean_ops::with_reversed_face`
re-charts one brick face onto its plane's reversal with the sense
flipped (outward side kept), and the boolean doors admit
the result. A point-in-solid query against a brick with one face so
reversed is the likely row for both `solid_contain` sites and
`reduce`'s `face_plane`; `star_face` and `planar_faces` need their
callers traced first.
