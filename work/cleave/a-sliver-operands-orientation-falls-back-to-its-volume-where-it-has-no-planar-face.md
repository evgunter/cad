---
id: a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face
kind: issue
title: A sliver operand's orientation probe falls back to its V/A where it has no planar face, and skips a solid whose faces do not read
status: closed
closed: 2026-10-25
opened: 2026-10-12
priority: P2
cost: M
---


Found by PR 4783's review (FLUSH, `flush/thinness-by-pairs`). Filed
here because the code is CLEAVE's (PR 4725).

**Two findings in `boolean/reduce.rs` `gate_sliver_orientation`**, the
operand gate that reads the orientation of each solid check 7 passed as
a sliver (`AtRestBody::slivers`):

1. **No planar face, no aimed ray.** Where the selection's volume does
   not decide the side at infinity, `solid_contain.rs`
   `point_in_solid_faces` reads it off rays aimed at points inside the
   selection's faces. `aim_points` offers a point only for a planar
   face (`stands::planar_interior_point`). A sliver with no planar face
   has no aimed ray, and its side stays undecided: the probe refuses
   `Containment` on the in-band `V/A`. That is a thinness gate in
   disguise for such shapes, the ruler Ev retired on PR 4690 (D10
   Booleans: thinness is judged by pairs alone). An orientation read
   that does not depend on a planar face would close it (an aim point
   on a curved face's carrier, or a crossing ray chosen per face).
2. **A silent skip.** `let Ok(sel) = super::SolidFaces::of(body, solid)
   else { continue };` drops a solid whose faces do not select, so its
   orientation goes unread with no refusal. It should refuse (as the
   `face_reach` miss just below it does) or say why the skip is sound.

**Evidence.** R1's review measured `rc_wide_battery` at ε 1e-6 on
PR 4783: 34 lines move from `ShellRole` to built (`OK BAD …
operand=false`, census clean), and 9 of those results refuse
`Containment` when this operand probe reads them (`review-sliver-probe` feature,
`flush/thinness-by-pairs-review` @ a79ba445). Whether those 9 are
finding 1 is not established here: measure first.

## Measured (FLUSH `flush/shell-role-reader`, 2026-10-14)

The operand gate `gate_sliver_orientation` is retired on that branch:
check 7 reads every solid's orientation at rest with the reader of a
shell's role, so no inside-out sliver finishes at rest, and a dual's
operand gate reads the same check 7. Finding 2 (the silent skip) goes
with the gate. Finding 1 (no planar face, no aimed ray) is now the
reader's own limit, narrowed: the reader aims at cylinders, cones, tori
and closed spheres too, and what is left (trimmed spheres, untrimmable
cones, splines) is on FLUSH's slate as
`the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline`.
This row closes with that PR; CLEAVE closes it.

## Closed (FLUSH `flush/sphere-spline-aims`, 2026-10-25)

Finding 2 went with the operand gate (above). Finding 1, narrowed to
the reader's own limit, closes with FLUSH's
`the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline`:
a trimmed sphere face now offers an aim that reads. What is left, a
selection with a spline face, casts no ray because no ray can cross the
face, and is INSIDE's `point-containments-ray-test-has-no-spline-arm`.
An untrimmable cone or wall has no window to aim in and keeps the
volume.
