---
id: the-rectangle-lanes-re-derive-their-area-pass-on-every-window-entry
kind: issue
title: the rectangle lanes re-derive their area pass and hull blocks on every window entry
status: open
opened: 2026-10-30
---


(QUAD per-span lane, `quad/per-span-round-zero`.)

## What

`rational_patch_face` (`crates/geom-brep/src/props/quad/rational.rs`) builds its refined nets, its measure quotients (`MeasureCells::build`), its hull blocks (`MeasureCells::block_hulls`), its area blocks (`MeasureCells::area_bounds`) and the whole area pass (`area_midpoint_taylor`) on every call, before its first round. `nurbs_patch_face_rounds` (`patch.rs`) does the same for its area pass and block hulls. The module docs say so ("each entry re-derives the face's setup"). A sign walk enters a face at round 0 (`topo::props::SignCertificate::at_round_zero`) and the measurement re-enters it at round 1, so every face pays its set-up at least twice, and once more per window a caller opens.

## Measured

On the conic quarter torus (`crates/step-import/tests/a_conic_quarter_torus_measures_a_bracket_at_the_wall.rs`, two 9 × 9-span walls), release, after the per-span schedule: set-up 0.18–0.20 s and area pass 0.47 s per wall per entry, at 16 area cells a span (about 1.4 s per entry at 32). Two entries per wall. The rounds themselves: round 0 0.25 s, round 4 11 s.

## Owed

Either carry the set-up across a face's windows (the round state is already per face in `topo::props`' `FaceRun`), or say why it cannot be carried. The area is a function of the face alone and does not change between windows.
