---
id: edge-edge-curved-meet-reads-a-kissing-pair-with-no-kappa-test
kind: issue
title: The curved edge–edge confirm reads two coplanar circles' touch with no κ_rel test, so nested circles of nearly equal radii read as meeting at a chord the band cannot place
status: open
opened: 2026-10-13
priority: P2
refs: [point-touch-reading]
---


`census::curved::curved_interiors_meet` (`crates/topo/src/census/curved.rs`)
reads two coplanar circles through `circle_circle`: their radical line,
then `line_circle`'s chord discriminant `pm_census_ee_curved_chord`,
decided on `disc / r`. Nothing reads `κ_rel` (the curvatures' sum or
difference), and for two circles one inside the other with nearly equal
radii it goes to zero: `1/r₂ − 1/r₁`.

For `c₁ = 0, r₁ = 1` and `c₂ = (δ, 0, 0)`, `r₂ = 1 − δ − g` (touching
from inside at `(1, 0, 0)` when `g = 0`) the radical line stands at
`x ≈ 1 + g/δ`, so the discriminant is `≈ −2g/δ`. The gap `g` in metres
is magnified by `1/δ`, so a pair within ε of touching (`|g| ≤ ε`) reads
definitely apart or definitely crossing once `δ` is small, never in
band. The point-touch lanes (`census::point`) test `κ_rel` with
`Margin::sagitta` before they place a touch point; this lane has no
such test.

Found by the review of PR 4956 (its body had claimed `κ_rel` for two
distinct circles is never zero; that is false for this pair). Not
measured on a body: the fixture is two arcs as in
`census::curved::tests::kissing_circles_meet_at_their_touch_point`,
with the inner arc's radius `1 − δ − g` for `δ = 1e-6`, `g = ε/2`.
