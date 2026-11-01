---
id: the-swept-elbow-step-fixture-did-not-move-with-libm-atan2
kind: issue
title: PR 5238 moved sweep_places to libm's atan2 without regenerating step-export's swept_elbow fixture and sidecar, leaving main red
status: closed
opened: 2026-10-17
priority: P1
cost: E
closed: 2026-10-17
---


Found by the stage 3 B lane (PR 5232) in a full workspace run, which filed it as `work/issues/the-swept-elbow-step-fixture-and-sidecar-did-not-move-with-libm-atan2.md`. This is INLANE's copy.

PR 5238 switched `sweep_places` to `<f64 as Real>::atan2`. On the swept elbow, station 6's turn angle is one ulp apart between glibc and `libm` (1.19383730525368592 against 1.19383730525368614). PR 5238 re-baselined mesh's `d9_mesh_goldens` and the FreeCAD render, but not step-export's committed fixture:
- `crates/step-export/tests/fixtures/swept_elbow.step` is byte-golden (`committed_fixtures_are_byte_golden`);
- `swept_elbow.expect`'s `KERNEL_VOLUME_PAD_MM3` is checked by `kernel_sidecars::kernel_sidecar_fields_match_live_kernel`.

**The blind spot.** The PR gate runs only the slow and step rows of the crates a diff touches. PR 5238 touched sweep, profile and mesh but not step-export, so these rows never ran on it. The nightly or a full workspace run is the first to see a fixture that another crate's change moves.

## Closed

Regenerated with the repo's own generator, `cargo run -p step-export --example export_fixtures -- crates/step-export/tests/fixtures`. It rewrites all 17 fixtures, and only `swept_elbow.step` moved:
- 92 of its entities changed, all `CARTESIAN_POINT`s of the walls' control nets past the turn, each in its low bits.
- In the sidecar, `KERNEL_VOLUME_PAD_MM3` went from 1.1379786002407855E-5 to 1.0454600148553558E-5. `KERNEL_VOLUME_MM3` and the census are unchanged, and the `EXPECT_*` (OCC) fields are untouched.

step-export and step-import pass in full at 1e-9, 1e-6 and 1e-12 (355 rows each). The slow rows that read the committed elbow also pass on the new bytes: `perf12_census_goldens::the_import_census_verdicts_are_goldened_bit_exact`, `corpus_fold` and `recognize_pins`.

**Sweep for other committed artefacts holding the elbow's bits.** Only two calls reach `sweep_places`: `sweep_body` and `skin`'s sweep geometry. Their consumers in committed artefacts:
- mesh `d9_mesh_goldens`: re-baselined in PR 5238.
- The step-export fixtures: this item. `lily_lantern.step` was regenerated too and is unchanged.
- step-import's perf12 census goldens (three ε) and `corpus_fold` / `recognize_pins`: they read the committed elbow and pass unchanged.
- Renders: PR 5238's `[render]` run re-baselined every lane, and only FreeCAD's `klein.png` and its montage moved. The `demos/renders` kernel frames of `klein`, `lily` and `hollowelbow` did not move.
- editor-core's `hollow_tube_elbow` (`lib_g16` name digests, `intent_s2_c_world`, the perf12 census) is a `tube_along_arc` body and does not reach `sweep_places`.
- The tour's `klein` and `lily` sweep through `sweep_body`, but the tour suite holds no bits of theirs beyond the renders. It ran 93/94 after PR 5238; the one red is the filed tang row.
