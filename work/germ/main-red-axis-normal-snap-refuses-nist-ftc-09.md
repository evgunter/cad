---
id: main-red-axis-normal-snap-refuses-nist-ftc-09
kind: issue
title: main-red: since PR 7171's axis-normal snap change, nist_ftc_09 refuses at entity #5130 (step_snap_relation in band) and three slow-set rows go red
status: open
opened: 2026-11-20
priority: P1
cost: E
---


Found by EXCH's unit-A lane running the full step-import slow set (`cargo nextest run -p step-import --profile ci --ignore-default-filter --no-fail-fast`). The rows are reproduced on a clean `origin/main` at `46e89240`. They are slow-set rows, so PR 7171's per-PR gate never ran them; the nightly will.

Red rows, each refusing `nist_ftc_09_asme1_rd.stp` at entity #5130 with `step_snap_relation` in the band:
- `snap::tests::nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them`
- `wild::wild_refusals_are_typed_and_name_their_class`
- `tier_gate::every_corpus_import_passes_the_shared_gate` (fails only at that file)

The likely cause is commit `c25c174105` ("step-import: an axis-normal relation in the file's band refuses at the snap"), which merged with GERM's PR 7171 (`germ/section-class-lane-choices`) and changes `crates/step-import/src/snap.rs`. That file has no B-spline surface, so EXCH's promotion units cannot touch it.

Per `memories/agent-lane-operations.md` the lane that caused a red owes the fix, so the row is on GERM's slate. The snap catalogue is IMPORTGATE's ground (`work/importgate/log.md`, PR 7135), so the fix should agree with IMPORTGATE's census, which found no axis-normal relation Zero at the file band in the corpus.

Signed: (EXCH orchestrator)
