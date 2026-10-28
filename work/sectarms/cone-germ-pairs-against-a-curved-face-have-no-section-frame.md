---
id: cone-germ-pairs-against-a-curved-face-have-no-section-frame
kind: issue
title: A cone wall crossing an oblique cylinder or another cone reaches the join and refuses GermFrameUnsupported: no frame names its section
status: open
opened: 2026-10-10
priority: P1
cost: H
design: true
refs: [torus-germ-pairs-have-no-section-frame, cone-pairs-in-general-pose-have-no-section-arm]
---

Found by GERM's general-pose cone lane (`cone-pairs-in-general-pose-have-no-section-arm`,
branch `germ/cone-pairs-general-pose`), which gave the section certificate
its cone × oblique cylinder and cone × cone arms (`section_cert/ruling.rs`).
Those arms answer the pair's section on both paths, so a pose with no
crossing now builds (`cone_operand_rows::an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms`).
A pose WITH crossings never reaches them: it stops first at the join's
frame.

## Measured

`crates/sweep/tests/cone_operand_rows.rs`,
`the_bite_refuses_at_the_germ_frame`: the preview cone (apex `(0, 1, 0)`,
half-angle `π/4`, base radius 1) against P1, a `5π/3` sector of the rod
of radius `0.3` along `x` about `(y, z) = (0.45, 0.8)`, and P2, the same
bite with a brick beside it, refuse every op `GermFrameUnsupported`
naming `(Cone, Cylinder)` (`join.rs` `pair_section_frame`, the
`FrameError::NoArm` arm). The section certificate now has an arm for
that cone × cylinder pair (the rod's axis stands outside the cone, the
pose of `section_cert_cone_pair_rows::a_rod_biting_the_side_cuts_one_null_loop`),
but no frame names it. A cone × cone germ pair reaches the same arm.

## What a fix has to supply

A frame for the cone × cylinder and cone × cone germ pairs, or a proof
that none exists for a section shape. The ruling charts describe the
section's components (every component a graph over an arc of one
carrier's rulings, or over the whole turn), which is the shape a frame
reads, but a general-pose cone section is a quartic: it can have several
loops and unbounded branches, and an apex the frame would have to keep
off every loop. Past a frame, each pair meets the lane door, as the torus
pairs do (`torus-germ-pairs-have-no-section-frame`).
