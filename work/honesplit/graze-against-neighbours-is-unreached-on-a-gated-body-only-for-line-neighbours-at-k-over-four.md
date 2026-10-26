---
id: graze-against-neighbours-is-unreached-on-a-gated-body-only-for-line-neighbours-at-k-over-four
kind: issue
title: split: a grazed wall's material against its neighbours is unreachable on a gated body only for line neighbours at K > 4; the conic-neighbour bound is unproven
status: open
opened: 2026-10-10
priority: P3
cost: M
---


(Filed by PR #4571's fix pass, under the orchestrator's ruling.)

## What

`splitting/rules.rs` `wall_graze` refuses
`SectorReading::GrazeAgainstNeighbours`, with the lever, where a wall
rule (a) read grazed holds its material across from both `S`-ON-`S`
neighbours. K is a runtime parameter (`CAD_AMBIGUITY_K`, any finite
K > 1), so the state can be reached through input and the typed refusal
is right. The open question is whether it is reachable on a gated body
at large K. If it is not, a K-conditional `debug_assert!` (D9 row 5)
belongs there.

## The derivation, as far as it holds

Set-up:

- `v` is the ON vertex: `|d(v)| ≤ ε`, where `d` is the signed distance
  from the split plane `P`.
- `W` is a convex wall that bends into its material. The kinds are a
  cylinder, a cone nappe and a sphere; `sector_face` admits cylinders
  and cones.
- Rule (a) read the sine `sin θ · L ≤ ε` between `n_W(v)` and `n_P`,
  over the face extent `L`.
- Take `n_P` oriented so that `S`, the neighbours' side, is `+n_P`.
  `GrazeAgainstNeighbours` means the wall's material lies on `−S`. So
  `n_W·n_P ≥ cos θ > 0`: the outward normal points toward `S`.

**A line neighbour.** This case holds. A line edge certified on `W`
through `v` is a ruling, and its far vertex `w` is a boundary point of
`W`, so `|w − v| ≤ L`. Its class is the decided side of `w`:
`d(w) ≥ Kε` toward `S`.

- `W` is convex, so it lies on the inner side of its tangent plane at
  `v′`, the foot of `v` on `W`.
- The at-rest gate certifies the vertex-on-face residuals `|v − v′|` and
  `|w − w′|` to be at most ε each.
- So `(w − v)·n ≤ (w − w′)·n + (v′ − v)·n ≤ 2ε`.
  - For a cylinder or a sphere, `n` at the off-surface point `v` is the
    normal at the foot exactly.
  - For a cone it is not, by an angle of about ε over the distance to
    the apex, which adds a term of that order.
- Write `n_P = cos θ · n + sin θ · u`. Then
  `d(w) = d(v) + cos θ (w − v)·n + sin θ (w − v)·u ≤ ε + 2ε + ε = 4ε`.
- A decided `S` needs `d(w) ≥ Kε`. So line neighbours cannot reach this
  arm at K > 4 on a cylinder or a sphere, or at K a little above 4 on a
  cone.

**A conic neighbour.** This case is not proven. Its class is the
first-order departure `t̂·n_P·E` (with `E` the edge's extent), or, where
that decides zero, the second-order bend. Bounding the first order needs
the edge's tangent at `v` to lie within about ε/E of `W`'s tangent plane.
Edge certification bounds the edge's DISTANCE to `W` (≤ ε over its
span), not its tangent. A bound on the tangent follows only by an
argument over the conic's curvature that I have not made.

So the derivation does not hold in general, and no `debug_assert!` is
added.

## To close

Either bound the conic neighbour's departure on a gated body and add the
`debug_assert!` for K above the bound, or record that the arm is
reachable at any K.

## Slate triage (2026-10-26, CLEAVE)

- Band P3: a reachability proof owed before a typed refusal becomes a debug assertion: latent unsoundness, unreached by any known input, P3 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to HONESPLIT (`work/honesplit/`) by `git mv`, id and body unchanged: whose charter is split and carve bookkeeping, unproven walks and the reads nothing reaches. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
