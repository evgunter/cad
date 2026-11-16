---
id: a-torus-verdicts-standing-is-matched-outside-its-one-home
kind: issue
title: A torus verdict's standing is matched at four topo sites rather than read through TorusRefusal::refusal; of_door_refusal's Torus arm is unreachable by convention; ring_floor_of is crate-wide where only tests read it
status: open
opened: 2026-11-10
priority: P3
---

Filed from PR 6777's delta review (the torus floor's ring gate). Line numbers are at 6777's head (`a019d0d8`).

## What

- **(i) `torus_refusal_at` spells the split again.** `crates/topo/src/boolean/mod.rs`, `torus_refusal_at`, matches `refusal.verdict` itself: `Refused` becomes `DegenerateTorus`, `Undecided` becomes `Escalated`. `geom_brep::TorusRefusal::refusal` is the convention's one home for that standing: an undecided half is ranked by its margin, and a torus decided outside it aborts.
- **(ii) Two more topo sites match `TorusVerdict` on their own:**
  - `crates/topo/src/boolean/join.rs:4098-4105`, `section_refusal` (`FrameError::Escalated` / `DegenerateTorus`);
  - `crates/topo/src/boolean/contact_verify.rs:139-140` (`ContactRefusal::Escalated` / `NotCertifiable`).

  One home should decide a torus verdict's standing, and every site should read it, ending it in its own type.
- **(iii) `BooleanError::of_door_refusal`'s `DoorRefusal::Torus(_) => unreachable!` is held by convention.** A conic-quadric door that came to refuse a torus would panic. Either the plain mapper cannot be handed a torus door's refusal, by type (a quadric door's own refusal type without the arm), or it routes the arm to `of_torus_door_refusal`, which needs the face.
- **`ring_floor_of`'s crate-wide escape hatch.** `crates/geom-brep/src/implicit.rs:1673` is `pub(crate)`. It computes the floor's formula on radii no `RingTorus` witness holds, and only tests read it: `circle_lane_rows::a_ring_a_float_from_fat_keeps_its_floor` and `implicit.rs:3710`'s ratio row. A production site in the crate could read the floor without a ring. It should be `#[cfg(test)]` beside a private formula, or the rows should build their radii through the witness.
