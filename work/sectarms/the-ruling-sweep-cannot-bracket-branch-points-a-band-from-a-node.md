---
id: the-ruling-sweep-cannot-bracket-branch-points-a-band-from-a-node
kind: issue
title: The ruling sweep escalates where an extreme of its discriminant stands within a fraction of a band past the escalate edge: its cuts halve toward the extreme and never bracket the branch points
status: open
opened: 2026-11-04
priority: P3
cost: M
refs: [cylinder-sphere-reads-its-pose-on-the-ruling-sweep]
---



Filed by the cylinder × sphere ruling-sweep lane (branch
`sectarms/cylinder-sphere-ruling-sweep`, 2026-11-04).

## What

`pair_section::ruling`'s discriminant recursion (`Sweeper::discriminant_cell`,
`Sweeper::split`) cuts an undecided cell at the first of `off_a_zero`'s
five angles where `Δ` decides: the middle, then ±⅛ and ±¼ of its width.
A branch cell needs `Δ` decided on both its ends and its slope decided
over it.

Where an extreme of `Δ` stands `x` escalation widths past the escalate
edge, the cell from the extreme's ruling is never monotone (its slope
reads zero there). So the sweep must cut it at a decided angle between
the extreme and `φ_a`, where `Δ` has fallen `x` widths, and `φ_a` is
followed by a stretch in the band out to `φ_b`, where it has fallen
`2 + x`. The ladder spans a ratio of three, so it finds that cut only
where `φ_b / φ_a` is under three.

- For a smooth extreme the ratio is `√((2 + x)/x)`, under three past
  `x = 1/4`.
- For a kink, where the near ruling runs through the centre, the ratio
  is `(2 + x)/x`, under three past `x = 1`.

Short of that the cell halves toward the extreme and ends at the floor,
escalating on `ruling_discriminant`. This happens at either extreme:

- **the far ruling (`φ = π`)**: a figure-eight's lobes just short of
  their node;
- **the near ruling (`φ = 0`)**: a small loop just past the sphere's
  touch. The reviewer's case is cylinder `r = 0.5` against a sphere at
  `(1, 0, 0.2)` of radius `0.5 +` 1.17 escalation widths.

## Witness

**The far ruling.** Cylinder `r = 0.5` about `z` and a sphere at
`(0.25, 0, 0)` of radius `0.75 − 5.5e-9`, at the band
`(4.95e-10, 4.95e-9)`. The far reach is 1.1 widths past the edge.
`pair_section` escalates on `ruling_discriminant`, and the pose decides
one loop. `topo::boolean::offer_rows`'s
`transverse_frame_reach_short_of_tangency` pins this as the reason its
offer stays withdrawn. Before the pair read its count on the sweep, the
same pose refused later, at the march (`TubeStraddles`).

**Measured zone** (2026-11-04, after the fix pass made `ρ²` relatively
precise near the near ruling). Each extreme was swept from the edge to
four widths past it, over 46 614 poses at ε 1e-6, 1e-9 and 1e-12:
general offsets, near-coaxial ones, centres on the wall and a few
widths off it. Every ruling escalation stood within 0.29 of a width of
the edge. `tests/pair_section.rs` exempts one width (`SWEEP_ZONE_WIDTHS`,
argued from the ladder) and samples twice that.

No answer is lost end to end. On the base these poses refused later
(`CellBudget`, `StepRefinementFailed`), and from 2 to 5 widths past the
edge the head now carries loops the base could not (the review's
measurement).

The cylinder pair's sweep shares the recursion, so its extremes are
expected to meet the same limit. That is not measured.

## What the change could be

When the ladder finds no cut that decides, also try cuts geometrically
close to each end (`lo + w·2⁻ᵏ`, `hi − w·2⁻ᵏ`). These find a decided
point between an extreme and its branch point, after which the cell
beyond is a branch cell. Every new cut is still a decided reading, so
this only turns escalations into answers.
