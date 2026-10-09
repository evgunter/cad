---
id: the-dihedral-arm-clause-is-seven-literals-held-by-a-source-census
kind: issue
title: The dihedral arm's shared clause is seven literals held together by a source census that false-positives on other arm decisions
status: review
pr: 4401
opened: 2026-10-09
priority: P3
cost: E
---


## What

These come from the delta re-review of PR 4366 (merged at `5e0ed71933`), which approved it.

- **One home for the clause (S1).** The clause "long enough, for how its faces curve, to measure their angle" is spelled as seven production literals:
  - `geom-brep/src/certify.rs`, in the `ArmCollapsed` `Display`;
  - `topo/src/merge_faces.rs`, in `KeptBoundaryUndecided`'s `Display`;
  - `topo/src/boolean/refusal_routes.rs`, in the `LeverArm` subject;
  - `topo/src/validate.rs`, at four sites: `WedgeCheck::Arm`, `NoDihedralArm`, `certify_undecided` and the at-rest lead.

  The pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` holds them together by a source census. A shared `const` that every door composes (`concat!`/`format!`) would make the census unnecessary.
- **The census false-positives (NOTE B).** Its filter ("long enough" + "angle" + "face") also matches other arm decisions worded naturally. Rewording `sector_shape.rs`'s corner arm to "…long enough to measure the angle its faces make" turns the pin red. Its floor `found >= tellings.len()` also counts the test's own literals. The `const` above retires this too.
- **`with_diag` is public (NOTE A).** `enters.rs` `LeverEscalation::with_diag` lets any crate re-quote a gate-refused arm with an arbitrary margin. Both live callers are sound only by data (`dihedral.rs` re-quotes on purpose; `sectors.rs` sits behind `offers_tolerance()`). Options: `pub(crate)` for dihedral, plus a public re-quote for topo that is a no-op once `refused.is_some()`.
- **Two idioms in the at-rest list (S2).** In `validate.rs` `certify_undecided`, the arm now ends "is undecided" (to fit the word budget), while its sibling arms end "is too close to call at this tolerance".
- **The row's parenthetical is stale (NOTE D).** `work/cleave/split-dihedral-readers-drop-the-arm-rung.md` says `BooleanError::of_lever` "already takes the whole escalation", but it delegates to `of_lever_rung` and drops `collapsed_arm` too, at `sectors.rs`'s two `enters_material` callers. Correct that row.
