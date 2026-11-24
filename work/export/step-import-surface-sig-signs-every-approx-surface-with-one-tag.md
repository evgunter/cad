---
id: step-import-surface-sig-signs-every-approx-surface-with-one-tag
kind: issue
title: step-import's surface_sig signs every approximating surface with a bare tag, so a second Approx face in one solid would be glued onto the first's surface
status: closed
opened: 2026-11-17
priority: P3
cost: E
refs: [approx-face-has-no-step-printer]
rides_with: step-import-discards-the-entity-ids-that-are-its-identity-channel
branch: export/import-records
pr: 7361
closed: 2026-11-22
---


Found by a designer weighing `step-import-discards-the-entity-ids-that-are-its-identity-channel` (EXPORT, 2026-11-17), off the question; checked by the orchestrator at `362cd72fd`.

`crates/step-import/src/adopt.rs`, `surface_sig`: the `SurfaceData::Approx(_)` arm returns `vec![tag]`. `attach_surfaces` uses the signature as the dedup key that puts bit-identical surfaces on one `SurfaceKey`, so two different approximating surfaces in one solid would share a key: the second face would be attached to the first face's surface. That is silent wrong geometry. The arm's own comment says it: "a tag alone would alias every approximating surface to every other".

It is dormant because no import path mints an `Approx` surface: `OFFSET_SURFACE` is not read. It becomes live the day one does, for example as the import half of `approx-face-has-no-step-printer`. Fail-loud says the arm should not alias. It should refuse typed, be made statically unreachable, or sign the fit's and base's own payloads as the NURBS arm does.

## Done when

No `Approx` surface can reach `attach_surfaces` and share a key it does not structurally equal.

## Closed (2026-11-22)

PR 7361 (`export/import-records`): surface records follow the file per shell; `surface_sig` and its readers are deleted; the writer emits one surface entity per key; per-pattern pins hold imports to their native twins. Dual review, fix pass, merged.
