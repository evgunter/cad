---
id: tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis
kind: issue
title: Tier 3's frame check refuses a section ellipse on its frame's own rounding, amplified by a semi-axis up to 1e6, as an unrepresentable datum
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [4861, 4812]
---



## What

Tier 3's check 1 reads a carrier's frame through
`geom::convention::frame_margins` (`crates/geom/src/convention.rs`),
reached from `geom::Curve3::representability_margins`. Each margin is
`ε − δ·A`: the frame's deviation `δ` from unit length, or from
`u_ref ⊥ axis`, times the arm `A`, the carrier's largest radius. For an
ellipse the arm is its larger semi-axis.

A frame minted by arithmetic is unit and orthogonal only to its own
rounding, `δ ≈ ulp(1) ≈ 2.2e-16`. A plane `θ` off a cylinder's axis cuts
an ellipse with `A ≈ r/θ`, up to 1e6 in the measured families. Once
`A > ε/ulp`, which is about 4.5e3 at ε = 1e-12, the margin is decided
by whether the frame's norm and dot product round to exactly 1 and 0. A
correctly rounded frame then refuses as
`ValidationError::UnrepresentableCurveDatum { Ellipse, URef, Length | Tilt, .. }`.
The display ends in the defect ending: "does not describe the curve its
kind names".

That is not what Ev's decision on PR 4812 says such a miss is. Its
interim reads a miss within a carrier's own resolution as a kernel
limitation (D4 ¶1(i)'s last resort), and nothing new refuses. A frame
off unit by its own rounding is within its resolution, not an
unrepresentable datum.

## Measured (CLEAVE, PR 4861, ε = 1e-12, unit cylinder)

PR 4861 re-seams each section chord at its nearest principal vertex
and normalizes the quarter-turned seam. On its head, every split side
is validated at rest
(`crates/sweep/tests/section_ellipse_seam_poses.rs`, which states each
pose's outcome):

- **Through-axis planes** (normal `(cos φ, sin φ, θ)`, 96 poses): 32
  refuse at rest on both sides. By θ: 1e-6: 4, 3e-6: 10, 1e-5: 4,
  3e-5: 10, 1e-4: 4, 3e-4: 0. Each refusal is `URef Length` or
  `URef Tilt`, at either end.
- **Coin-flip planes** (through a seam vertex, normal `(0, 1, θ)`, 66
  poses): none refuse.
- **Off-axis planes** (CLEAVE's census family `axial`, 704 poses): 38
  refuse at rest this way.
- Before the turned seam was normalized, PR 4861's review counted 22
  coin-flip, 42 through-axis and 45 off-axis poses refusing this way.
  Main refuses every one of these poses earlier, at the join's endpoint
  pin, so the refusal has moved stage; it has not appeared from nothing.

The ε = 1e-6 and 1e-9 rows show no such refusal: there `ε/A` is above
`ulp` for every `A` here.

`crates/geom/src/convention.rs` is claimed by no program
(`work.py territory`). This row sits with tier 3's check 1, its reader.

## Owed

Separate a frame's own rounding from a datum that is truly off its
convention. For example, charge `δ` against the frame's rounding bound
before levering it by `A`, or read the margin through the arm's
resolution. A frame within its own rounding should pass, or end as a
kernel limitation. It should never end as `UnrepresentableCurveDatum`.
Re-measure with the families above, which pin every pose's outcome, so
the fix re-baselines their tables.

Related: CLEAVE's `the-section-table-admits-a-curve-on-its-resolution`,
whose anchored conic still stores a frame.

## Note from CLEAVE's `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body` (PR 5100, 2026-10-14)

With the split gated at rest (PR 5100), this finding stops at the split
door as `SplitFinishError::ResultNotAtRest`, and the user sees its defect
ending there. PR 5100's review found it is also the first finding at
ε = 1e-9, not only at 1e-12. Its poses are the grazing-plane rows of
`crates/sweep/tests/a_vertex_on_a_grazing_plane.rs`
(`a_reading_through_a_vanishing_sine_offers_no_lever`): the unit cylinder
turned φ = 2.2, cut through its seam vertex at ψ = asin(0.1·K·u/ε), with
the plane's stored origin `L` up to 2.5 m along the rim. The seam-vertex
family `crates/sweep/tests/split_seam_vanishing_tilt.rs` reaches it at
φ = 0.9.
