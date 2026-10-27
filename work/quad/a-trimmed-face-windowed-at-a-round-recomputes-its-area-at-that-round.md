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

In `props/quad/`, `trimmed_patch_face_rounds` computes the face's area once, at the first round of the window it is called with. A one-round window `RoundWindow::at(r)` (topo's sign walk, `PastTarget::refine`) therefore recomputes the area at round r's chords:
- round 6 costs 66 s windowed against 2.4 s inside a whole-schedule call;
- the width differs: round 1's `width_len` is 4.4466e-3 windowed against 4.4565e-3 in-schedule.

So `SignCertificate::refine_to_target`'s claim to be bit-identical to `mass_properties` does not hold for trimmed faces.

Owed: the area is a function of the face, not of the window. Hold it across windows, or compute it once per face. Then pin windowed-equals-scheduled bits on a trimmed face. The measurement door's unit B (`docs/QUAD-MEASURE-SPEC.md`) owns the face state across windows (R11), so sequence this with it.
