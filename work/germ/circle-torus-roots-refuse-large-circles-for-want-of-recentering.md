---
id: circle-torus-roots-refuse-large-circles-for-want-of-recentering
kind: issue
title: The circle x torus root door refuses circles much larger than the torus: its degree-2 harmonics cancel ~rho^4 terms
status: closed
opened: 2026-09-29
closed: 2026-10-20
branch: germ/circle-torus-recentering
pr: 5394
priority: P3
cost: M
refs: [circle-crosses-a-torus-face-with-no-root-lane, conic-doors-walk-their-taylor-terms-through-the-harmonics-noise]
---

## What

`crates/topo/src/boolean/circle_torus.rs` `circle_torus_roots` builds
`F` along the carrier as degree-2 harmonics about the circle's own
centre. For a circle much larger than the torus those harmonics are
sums of terms of size `~(|C₀ − c|² + ρ²)² ~ ρ⁴`, which cancel to the
small values `F` takes where the carrier passes the tube. The noise
meter (`half_angle_roots`, `bool_circle_torus_noise` and
`bool_circle_torus_root_slack`) refuses once that rounding reaches the
band. Measured against a torus `R = 1, r = 0.25` at the default band
(`crates/topo/src/boolean/circle_torus.rs` test
`no_wrong_certified_answer_across_circle_radii`): the `f64` answers
taper from `ρ ≈ 10–15`, and at `ρ = 30` every grazing pose refuses. The
threshold scales as `ρ⁴ ≲ 10ε·r·R²/(16u)`.

The refusals are typed. They are not the whole story at large ρ: where
the noise reading lands in the band's gap the ladder passes it, and
certifies roots past the band (below). What the refusals cost is
circles passing NEAR the torus. Distant circles are still cleared earlier, by the
circle rung's enclosure clearance (`reduce.rs` `circle_clearance`).

## The fix

Expand `F` about the carrier point nearest the torus rather than about
the circle's centre. Near the crossings the carrier is then a short
arc through a point at distance `~R + r` from the torus centre, and
the coefficients stay `O((R + r)⁴)` rather than `O(ρ⁴)`. The expansion
is no longer a finite trigonometric polynomial in the shifted
parameter, so this needs its own certificate for the truncation, or an
exact re-expression that keeps the magnitudes small.

## Home

GERM, beside the circle × torus root lane.


## Evidence (2026-10-02, the dual review of PR 3752): large circles on the shared ladder

The ladder is shared with the circle × cylinder door
(`crates/topo/src/boolean/circle_roots.rs`). On it, a circle of radius
1500 tilted to a unit wall gets roots certified 1.09–1.23e-9 m off at
ε = 1e-9, its noise reading in the gap (review 2, `topo_dr2_probe.rs`,
`dr2_gap_noise_root_error`). Recentering would shrink the terms that
put the reading there; the posture is
`circle-torus-meters-accept-an-unreadable-reading`.

## Fixed (branch `germ/circle-torus-recentering`, 2026-10-14)

**Measured first, on main** (`a2d40d45`). The pinned sweep
(`circle_torus::tests::no_wrong_certified_answer_across_circle_radii`)
answered `[32, 32, 8, 0]` of 32 at ρ = 10, 30, 100, 300 — not the
`[16, 0, 0, 0]` above, which predates the subdivision. A random sweep
(poses grazing the tube by ±{3, 30, 3000}·ε, crossing it 0.02 and 0.2 m
deep, clear of it by 0.02 and 0.3 m, at random points and tangents of
the torus `R = 1, r = 0.25`; ρ from 1 to 1000 m; ε = 1e-6, 1e-9, 1e-12)
judged against an mpmath oracle on the exact binary inputs found no
wrong answer and no root past the zero band, and grazing poses refusing
from ρ ≈ 50 at ε = 1e-9 (all from ρ = 200) and from ρ ≈ 500 at 1e-6.

**The refusals were the walk's, not the ladder's.**
`geom_brep::half_angle::certified_subdivision` read `F`, `F′`, `F″`,
`F‴` from the harmonics and charged each `Nʲ·noise`, the noise being
`rounding_charge(h.terms)`, `~u·ρ⁴`. Truncation was never the issue:
the remainder takes the global `|F⁗| ≤ Σ k⁴Aₖ`, which scales with the
arc (`ρ⁴·w⁴ = arc⁴`).

**The fix reads the Taylor terms at the point.**
`geom_brep::conic_torus_jet` gives `F` and its first three derivatives
by the chain rule through `S = |q|²`, `h = q·â` (`q′ = E′`, `q″ = −E`,
`q‴ = −E′` on any frame), each with a running rounding bound
(`geom_core::Rounded`); `SubdivisionFrame::jet` hands it to the walk,
which reads those terms and charges their own bounds instead. The
harmonics give only `M₄`. Every term is at the point's own scale, so
the error grows like `u·ρ` (the point's coordinates), not `u·ρ⁴`. The
item's "expand about the nearest point" is this, at every piece's
midpoint, with no truncation to certify: it is an exact re-expression
read locally.

**After**, same poses: nothing that answered on main changed its answer
(roots bit-identical), every new answer is true, and grazing poses are
answered at every ρ at 1e-9 and 1e-6. The pinned sweep answers every
pose to ρ = 1000 but the 1.6e-8 m dips at ρ = 1000, whose root slack
the point's own rounding (`u·|C|`) puts past the band. At 1e-12 the
shallow-dip refusals are the root slack's at every ρ, main's alike.

**The gap note above is not a live hole.** The ladder's answers are
dropped (`circle-torus-certifies-shallow-roots-off-by-more-than-the-band`,
`circle-torus-meters-accept-an-unreadable-reading`): a noise reading in
the gap only lets the ladder escalate, and the subdivision's root-slack
meter refuses an unreadable slack. Measured on main: of 323 poses whose
noise read in the gap (one seed of 4,800), 220 were answered, none
wrong, none past the zero band.

**Sibling doors** walking through the same noise are filed as
`conic-doors-walk-their-taylor-terms-through-the-harmonics-noise`.
