---
id: an-ellipse-span-is-metered-at-its-minor-axis-and-refuses-a-flank-edge-k-times-longer-than-the-band
kind: issue
title: The certifier meters an ellipse span at its minor semi-axis, so an edge on a steep ellipse's flank refuses IntervalNotForward at up to k times the band
status: open
opened: 2026-10-09
priority: P3
cost: M
refs: [a-steep-ellipse-travel-margin-ties-band-apart-sites-and-falls-back-to-the-chord]
---


Found by JOIN's steep-ellipse travel row
(`crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs`), on
PR 4396's first head. Since that PR's fix pass, the `k = 60` pose
escalates at the join's travel clearance before it reaches the
certifier, so no committed row reaches this check now.
No program claims `crates/geom-brep/src/certify.rs`, hence `issues/`.

## What

`geom_brep::certify`'s interval check (`span_at_floor`, the
`Curve3::Ellipse` arm) decides an edge forward on
`Margin::metered(t₁ − t₀, min(|major|, minor))`: the parameter span
times the smallest speed the ellipse has anywhere. On an ellipse of
aspect `k` the speed at parameter `t` is `√(a² sin² t + b² cos² t)`,
so an edge on the flank reads `b/|P′(t)|` of its length, which is
`≈ √2/k` at `t = π/4` and `1/k` at the minor vertex.

Measured: a `k = 60` cylinder section (semi-axes 60 and 1) crossed by a
block whose top face holds a finger `12ε` wide at `x = k/√2`. The
section edge across the finger is `12ε` long and reads
`2.83e-10 = 12ε·√2/60` at ε = 1e-9, so every op, both orders, at
ε 1e-9, 1e-6 and 1e-12 refuses
`Join(Euler(Certification { IntervalNotForward { Zero } }))`. A `k = 10`
edge of the same length would read `1.67ε`.

The meter is a certified lower bound, so nothing unsound ships. But it
refuses as "shorter than the tolerance" an edge up to `k` times longer
than the band, which is the amplified reading `docs/DESIGN.md` Q1's
conditioning premise rules out.

## Done when

The span reads its edge's length to a bounded factor that does not grow
with the aspect: for example, the speed floor over the span itself, not
over the whole ellipse. Pinned by a flank edge a few bands long on a
`k ≥ 10` ellipse that certifies.

## A note for JOIN

While this meter holds, the join's old travel margin could not have
mattered end to end. A `Zero` tie between two partners of one germ
needs an opposite-sense site between them, and so an in-face section
edge on the same ellipse, no longer than the arc between the two
partners. Its certified length `ℓ·b/|P′|` is at most `ℓ·sin ψ`
(`sin ψ = ab/(|P||P′|)` and `|P| ≤ a`), so at most the old margin, the
whole arc times `sin ψ`. Wherever that edge certifies, the margin was
already definite.
