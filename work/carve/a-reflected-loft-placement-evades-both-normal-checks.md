---
id: a-reflected-loft-placement-evades-both-normal-checks
kind: issue
title: A loft section placed with a reflected (left-handed) frame passes both stacking decides and builds a zero-volume body that tier 3 accepts
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-10-31
pr: 6120
---


Found by the review of PR 4188 (`carve/fold-reads-the-far-normal`),
re-measured by its implementer at that branch's head.

`loft_body`'s doc calls `places[i]` "that section's rigid placement",
and nothing enforces it. Over two copies of the 2×2 loft-prism square
(`sweep::test_support::loft_prism_sections`), section 0 at the
identity and section 1 at `z = 1` with linear columns `(x, −y, z)`
(a reflection: `c0 × c1 = −c2`) builds. `topo::validate_geometric`
returns `Ok`, and `topo::mass_properties` gives a signed volume of
±4e-16 (the review measured −4.2e-16, the implementer +4.2e-16): a
zero-volume body, the walls of a loft between a square and its
mirror image.

Both stacking decides in `crates/sweep/src/loft.rs` `stacking_fold`
read only `linear.c2`, which is `+z` in both placements, so both are
`Positive`. But the canonical loop sense (counterclockwise about the
section's normal, `skin.rs` "The correspondence is the author's") is
read in the section's sketch frame, and a reflected frame turns it
clockwise in the world, so the two rings wind oppositely.

The fix is a decide at the loft and sweep doors that each placement's
linear part is a rotation (`det = +1`, orthonormal at tolerance),
refusing typed with the section named, or a stated reason a reflected
placement is legal authoring (and then the fold has to read the
frame's handedness). The same question applies wherever a door takes
an `Affine3` documented as rigid; that sweep is part of this row.

## Closed (2026-10-31, PR 6120)

**Already true on main; now pinned.** Since 2026-10-14 a `SketchPlane` wraps one `geom_core::RigidMotion`. Its fields are private, its constructors are right-handed by construction, and its one door from a raw matrix is `check_rigid`, whose `transform_rigid_det_plus_one` refuses a mirror ("it mirrors"). So the row's `(x, −y, z)` placement refuses where the caller mints the frame, and never reaches the loft or sweep doors. A second rigidity check at the doors would re-check what the type already guarantees, so none was added.

- `crates/sweep/tests/a_mirrored_loft_placement_is_no_sketch_plane.rs` pins the mirror, scale and shear refusals at f64 and Interval, and a proper quarter-turn loft at its exact volume of 8/3.
- `loft_body`'s doc says the placement is rigid by its type.
- The sweep for other doors that take an `Affine3` documented as rigid covered 43 signature hits and 21 field hits; PR 6120's body lists them. One finding was filed: `work/issues/a-mirrored-bodys-mapped-places-are-called-rigid.md`.

**Review: the orchestrator's read.** Dispatch planned a single FULL review for a new decide. The unit came back as tests, a doc line and a filed row, with no kernel logic, so the tier dropped. I checked the lane's key claim against the tree: `RigidMotion`'s fields are private, `from_linear` is private, and `SketchPlane { frame: RigidMotion }`.
