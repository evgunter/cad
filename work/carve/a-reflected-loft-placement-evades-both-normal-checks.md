---
id: a-reflected-loft-placement-evades-both-normal-checks
kind: issue
title: A loft section placed with a reflected (left-handed) frame passes both stacking decides and builds a zero-volume body that tier 3 accepts
status: open
opened: 2026-10-06
priority: P1
cost: M
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
