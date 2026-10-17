---
id: the-swept-elbow-step-fixture-and-sidecar-did-not-move-with-libm-atan2
kind: issue
title: step-export's swept_elbow fixture and its KERNEL_VOLUME_PAD sidecar are red on main: #5238's libm atan2 moved the elbow, and the committed files were not regenerated
status: open
opened: 2026-10-17
---


Two `step-export` rows are red on main (`5685e48e02`). They were found
while INTENT stage 3 B ran the workspace on its merge of main; that
branch touches neither `crates/step-export` nor `crates/sweep`.

- `crates/step-export/tests/export.rs:367`
  (`export::committed_fixtures_are_byte_golden`): "swept_elbow.step
  drifted — regenerate deliberately".
- `crates/step-export/tests/kernel_sidecars.rs`
  (`kernel_sidecars::kernel_sidecar_fields_match_live_kernel`):
  "swept_elbow: want KERNEL_VOLUME_PAD_MM3=1.0454600148553558E-5,
  sidecar has 1.1379786002407855E-5".

Cause: #5238 (`sweep_places` turns its stations by `libm`'s `atan2`)
moved the swept elbow; its own fix pass says "the elbow's mesh digests
move". The committed STEP fixture and its `.expect` sidecar still hold
the platform `atan2`'s bits.

Proposed fix: regenerate `tests/fixtures/swept_elbow.step` and its
`swept_elbow.expect` with `crates/step-export/examples/export_fixtures.rs`,
at default ε, as `kernel_sidecars.rs:88` asks. Check that the volume
moved by rounding only.
