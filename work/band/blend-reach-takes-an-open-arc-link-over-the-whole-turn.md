---
id: blend-reach-takes-an-open-arc-link-over-the-whole-turn
kind: issue
title: blend: an open arc link's reach is taken over the whole turn, so a face on the far side of its circle refuses uncertified
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by the dual review of PR 4143 (predicate 2's reach), R1 m2. The
row that measures it is
`blend_band_reach_rows::an_open_arc_link_s_reach_is_its_whole_turn`
(`crates/sweep/tests/blend_band_reach_rows.rs`).

`circular_reach` in `crates/sweep/src/blend/reach.rs` revolves a link's
cross-section over the whole turn, which is exact for a closed rim and
an enclosure for an open arc link: a face on the arc's circle far from
the arc itself is read as inside the band's reach. Measured at the
meter (the review's rows I and J), each clear of the band:

- a D-boss (`ρ ≤ 1` cut flat at `x = 0.5`), its arc alone: a post at
  `x ∈ [0.85, 1.15]` past the flat, on the arc's circle, refuses at
  `−0.0039`;
- a 120° segment boss, its arc alone: a post across its circle at
  `y ∈ [0.85, 1.15]` refuses at `−0.0345`;
- posts off the circle pass.

The review's row H (a rounded block with a hole across a corner arc's
circle, `−0.035`) now refuses earlier, typed, at the line-to-arc
junction's tangent end face (`UnsupportedRunOut`), which a mixed chain
will need to read as the next link's start rather than an end face.

Latent today: `fillet_edges` refuses all of these chains upstream
(`UnsupportedChain`: an open chain off plane–plane supports, a closed
mixed chain). Fix it when mixed or open arc chains land: bound an open
link's reach by the two half-planes through the axis at its ends (each
widened by the cross-section's run along the end face, as
`straight_reach` does for its window), so the posts on the circle pass
and the row's two refusals flip.
