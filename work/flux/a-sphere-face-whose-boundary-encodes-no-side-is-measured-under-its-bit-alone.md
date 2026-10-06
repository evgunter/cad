---
id: a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone
kind: issue
title: A sphere face whose boundary encodes no side (the rimless band, a tilted-circle face with a pole on its loop) is measured under its sense bit alone, so a flipped bit measures the complement silently
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [sphere-flux-arm-carries-two-closed-forms-for-one-face-kind]
---


## Measured

Found by the `reach/tilted-sphere-pair` fix pass. The Gauss–Bonnet arm
(`props::curved::sphere_circle_loop`) reads a tilted-circle face's side
off its loop where it can (`sphere_circle_loop_side`: a face holds no
pole, so it is the side of its loop holding neither pole, read on a
meridian through the loop) and refuses a bit that contradicts it
(`PropsError::SenseContradicted`; tier 3 check 6 names the same face
`CurvedSenseInverted`). Where the loop passes through a pole it reads
nothing: the chart walk itself orients a pole junction by the bit, so
no σ-free encoding exists there — the rimless band's residual, shared.

Union of two unit balls 1.4 apart along x (both `y`-poled): flipping
either of A's or B's remnant faces (pole vertices on the seams) gives
`mass_properties` = 3.6798521949048433 against the true 7.868642399691235
(the complement measured), and `validate_geometric` refuses it only as
`LaminaWedge` on the tilted arcs. The lens faces of `A ∩ B` and B's
bitten cap in `A ∖ B` are refused by name
(`crates/sweep/tests/tilted_sphere_pair.rs`,
`a_flipped_tilted_face_is_refused_by_name`).

**How much of the flip population this is** (the delta review of PR
3817, over `47b16394c2..1d29d4bc81`). Over 480 oracle bodies, every face
was flipped alone: 1,646 single-face flips in all. The sense
cross-check refused 274 of them by name. The other 1,372 (83%) are this
item's pose family: the pole-bearing remnants of a sphere cut off its
chart's polar axis, the faces of the union, of `A ∖ B` and of `B ∖ A`
that keep a ball's original pole vertices on their seam. Each of those
flips returns a WRONG volume from `mass_properties`, and
`validate_geometric` sees only `LaminaWedge`. A corrupted sense bit on
the commonest face of a tilted carve is measured silently, which is
why this is P2 and not P3.

## What a fix owes

A second encoding of the side for a face whose loop touches a pole —
the neighbouring faces' sides carried across a shared edge into a
global orientation pass, or the chart walk's pole junction stated
without the bit — or the reason none exists, recorded where
`MaterialSign::Unencoded` is documented.
