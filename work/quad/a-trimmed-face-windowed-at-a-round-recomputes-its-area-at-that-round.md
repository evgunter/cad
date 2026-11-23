---
id: a-trimmed-face-windowed-at-a-round-recomputes-its-area-at-that-round
kind: issue
title: The trimmed lane takes its area once at a call's first round, so a one-round window recomputes it at that round's chords: sign walks pay 66 s where the schedule pays 2.4 s, and the width differs from the schedule's
status: open
opened: 2026-10-22
priority: P2
cost: M
---


Measured by the measurement-door unit A lane on main `8513607169` (release, ε = 1e-9), 2026-10-22.

In `props/quad/trimmed.rs`, `trimmed_patch_face_rounds` computes the face's area once, at the first round of the window it is called with. A one-round window `RoundWindow::at(r)` (topo's sign walk, `PastTarget::refine`) therefore recomputes the area at round r's chords:
- round 6 costs 66 s windowed against 2.4 s inside a whole-schedule call;
- the width differs: round 1's `width_len` is 4.4466e-3 windowed against 4.4565e-3 in-schedule.

So `SignCertificate::refine_to_target`'s claim to be bit-identical to `mass_properties` does not hold for trimmed faces.

Owed: the area is a function of the face, not of the window. Hold it across windows, or compute it once per face. Then pin windowed-equals-scheduled bits on a trimmed face. The measurement door's unit B (`docs/QUAD-MEASURE-SPEC.md`) owns the face state across windows (R11), so sequence this with it.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. `trimmed_patch_face_rounds`
(`crates/geom-brep/src/props/quad/trimmed.rs:1480`) still takes the area
at the entering round (`area_at_first`, and the comment at `:1580`).
Windowed callers still enter one round at a time: `walk_set` in
`crates/topo/src/props.rs:877` resumes every refinable face with
`RoundWindow::at(round)`. `SignCertificate::refine_to_target` is gone
(PR 5741), and PR 5628 already gave up the claim that a continuation is
bit-identical to `mass_properties`. So the owed pin is now "a sign walk's
trimmed face reads the schedule's area". The cost half (66 s against
2.4 s at round 6) was not re-measured. FaceRun is one state now (R11,
PR 5628), but it holds no area across windows.
