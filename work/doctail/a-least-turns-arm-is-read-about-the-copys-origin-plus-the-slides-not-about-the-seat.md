---
id: a-least-turns-arm-is-read-about-the-copys-origin-plus-the-slides-not-about-the-seat
kind: issue
title: A least turn's arm is read as the reach about the copy's origin plus the written slides, an upper bound; the turn pivots about the seat, and its pole verdict (a definite-sign claim) reads that one upper arm
status: open
opened: 2026-11-22
priority: P2
cost: H
refs: [a-face-charts-turn-is-levered-over-none-of-what-it-carries]
---


Found by the `tracecarry` designers (2026-11-22). Both agree it is correct, and that it is a design question for Ev, separate from `[ev]` PR 7353.

## What

- **The seat.** A plane chart is F(q) = S·O·(Z·q + s) + foot, with S the least turn, O the orientation, Z the spin and s the slides (checked against `mate/value.rs` `face_chart_slides` and `settle`). Rounding in S moves a point by δS times its distance from the seat's preimage q* = −Zᵀs. The arm today (`mate/solve.rs` `arm_of`: reach about the copy's origin, plus the written slides, plus the datum) is only the triangle-inequality bound of that. Worked example: slides of 300 and a profile of radius 50 at the seat give an arm of about 650, where about 50 is true.
- **One arm for two verdicts.** `least_turn` reads one arm for the pole verdict, a definite-sign claim, and for the conditioning. geom-core's ratified lever sentence ("a verdict that claims a definite sign reads the extent from below …"; for a verdict acting on every sign, read both bounds and act where they agree, `LeverBounds`) is not followed: every carriage passes an upper read.
- **The datum term.** Whether `arm_of`'s datum belongs in a plane chart's arm at all: both designers are unsure.

## Done when

Ev rules on the seat rule and the two-arm least turn. A change reaches every placement's verdicts and `[placement.values]`'s "written slides included" (fork 270). It is then built, with the boss-against-placed-cube rows re-derived.
