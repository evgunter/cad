---
id: a-line-arc-near-tangent-crossing-resolves-coarser-than-eps
kind: issue
title: A line×arc crossing near tangency resolves coarser than ε at large radius
status: open
opened: 2026-11-09
priority: P3
cost: H
---


`crates/profile/src/seg.rs` `line_arc` places a secant line×arc pair's two crossings at `Line2::circle_heights`, the centre's height `∓ √(r² − h²)`, `h` the centre's side of the line. Near tangency the root is ill-conditioned: an error `δh` in `h` moves a crossing by about `r·δh/half`, `half` the half-chord. At radius 1.36e4 with a 2.7 m chord (`crates/profile/tests/tangent_arc_near_a_full_turn.rs`, `the_rows_three_radii_keep_their_end_vertex`), `h` carries a few ulps of `r` (≈3e-12 m), so the crossing is placed only to ≈3e-8 m, coarser than ε. The crossings then fall on the arc's end vertex inside the band and validation reads the loop as a `Touch`.

The PR 6594 review measured this against 50-digit truth. `Line2::side` gives an `h` error of +7.6e-13, and the half-chord moves by −7.7e-9. The unit perp-dot read the row used before gives −2.9e-12 and +2.9e-8. `√((r − |h|)(r + |h|))` fails the same way, so the cause is conditioning, not cancellation. The row passed on the coarser read only because its rounding happened to land outward. Its expectation now pins validation only where `r·(64·r·ULP)/half ≤ ε`.

Fix: read the crossing so that its error does not scale with `r/half`. For example, take it from the arc's own parameterisation near its end vertex, or certify it with the radicand's error charged. Either way, a clear loop at this scale validates instead of reading a touch.
