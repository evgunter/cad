---
id: approx-face-has-no-step-printer
kind: issue
title: STEP export refuses an Approx-faced body - the writer has no OFFSET_SURFACE printer for the kind
status: open
opened: 2026-09-04
refs: [1758]
---


Found by both SHELL-2 reviewers' end-to-end consumers (2026-09-04):
after `transform_rigid` moves an `Approx`-faced part,
`step_export::step_string` refuses with "face …'s surface
(approximating surface) has no printer in the analytic subset". The
transform door is not the cause — the writer has no arm for
`Surface::Approx` at all, moved or not. STEP AP214/AP242 carry an
`OFFSET_SURFACE` entity (a basis surface plus a distance), which is
exactly what `SurfaceDescription::Offset { base, d }` stores; the fit
itself would print as its B-spline. Third wall a user meets on an
`Approx`-faced part, beside the cache refusals recorded in
`work/shell/no-approx-faced-body-is-both-movable-and-valid.md`.

Home: the STEP writer is EXCH's (Track U's STEP/STL rows).
