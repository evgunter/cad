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
