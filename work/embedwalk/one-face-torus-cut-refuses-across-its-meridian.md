---
id: one-face-torus-cut-refuses-across-its-meridian
kind: issue
title: A one-face torus cut off its axis refuses Departure across the meridian its two torus faces share, at both scalars
status: open
opened: 2026-10-31
priority: P3
cost: M
---



## Finding

Found by SPIRIC's fix pass (#6159), building a kernel body that reaches
the plain wall presentation.

**Reproducer.** `one_face_torus_cut` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`: the
one-face torus `(2, 0.5)` about `y` (the profile one circle from
profile angle `0.3`), less `x < 0.3`. The cut leaves:
- two torus faces sharing the full meridian circle at `u = 0`
  (`Edge(3v1)`), each closed on itself across a parallel seam;
- one whole spiric oval on each torus face, each closed at a vertex on
  that seam;
- two caps.

**Verdict, at both scalars:** `Uncertified { torus, torus,
Edge(meridian), Departure }`, 13 pairs refused, 148 splits.

Both cap–torus pairs are certified across their ovals (by the plain wall
presentation, `cornered` 0). The refusal is the torus–torus pair across
the meridian, and SPIRIC's arm is not involved in it. On main the body
refuses `Departure` at a spiric first, so whether this refusal predates
SPIRIC is unmeasured.

**Not diagnosed.** The meridian meets each face's parallel seam at the
meridian's own vertex, and refusals that fire at once suggest both
sides are `Structural` there. The pinned row
`a_one_face_torus_cut_certifies_its_caps_by_the_wall_alone` asserts the
refusal typed and goes red when it is fixed.

## Diagnosed (EMBED trace lane, 2026-11-21)

Traced on main `c2a0c41f40` with the at-rest gate and prints at each
`Structural` exit (`embed/trace-cone-torus-measure`). The refusal is at
depth 0: `departure`'s "both stuck" exit, both faces `Structural` in
their whole windows (`3v3` at `u ∈ (7π/4, 2π)`, `4v1` at `u ∈ (0, π/4)`,
both `v ∈ (π, 2π)`, the meridian `Edge(3v1)` shared, vertex `2v1` in
the cell). Each is raised at `present_chart`'s "a line on a periodic
axis whose sheet is unknown has no place" (`placed` returns `None`).
Why the sheet is unknown, on a face closed across its parallel seam, is
not traced; `a-face-with-no-line-on-a-periodic-axis-reads-its-sheet-off-the-stored-lift-unchecked`
is the neighbouring reading.

At the gate this row is 4 tests (torus/torus):
`self_intersection_proves_injectivity::a_one_face_torus_cut_certifies_its_caps_by_the_wall_alone`
(traced), `germ_plane_torus_spiric::the_one_face_and_seamed_tori_s_spiric_poses_answer_as_the_donuts`
(traced, the same pair and exit),
`germ_plane_torus_spiric::every_point_of_the_spiric_box_on_the_one_face_torus_reads_its_side`
and `shell_cavity_embedded::a_cavity_the_engine_cannot_certify_refuses_uncertified`
(by fixture, unchecked). The body is valid, so at the gate's landing
this is P0. Unchecked against PR 7115.
