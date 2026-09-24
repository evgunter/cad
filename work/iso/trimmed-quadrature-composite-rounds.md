---
id: trimmed-quadrature-composite-rounds
kind: issue
title: The trimmed quadrature fences the Newton-Cotes window at p_u + p_v <= 4; the composite fallback is unbuilt
status: open
opened: 2026-09-13
priority: P1
cost: H
---


## What

`geom_brep::props::quad::trimmed_patch_face_rounds` integrates the trim
loop's chord polygon EXACTLY: Green's theorem on
`G_f(u, v) = ∫_{v₀}^{v} f`, with a nested closed Newton–Cotes rule whose
outer order is `3p_u + 3p_v − 1` in `u` and inner order `3p_v − 1` in
`v`. `newton_cotes_weights` builds its nodes as exact `i128` fractions and
tops out at **12 INTERVALS — 13 nodes** (the `i128` headroom, and the
`< 2^53` exactness check on the reduced fraction): `newton_cotes_weights(m)`
returns `m + 1` weights and refuses `m > 12`. The fence arithmetic below
counts intervals and is right; an earlier wording of this file and of
the PR said "12 nodes", which is off by one in the reader's favour and
would make the window look one degree tighter than it is. So the exact
rule runs while

```text
3·(p_u + p_v) − 1 ≤ 12   ⟺   p_u + p_v ≤ 4
```

and a richer chart refuses typed at the site named
`trimmed exact lane's Newton–Cotes window` — the `TRIM_NC_WINDOW`
constant in `quad.rs`, pinned by row `q6_a_chart_past_the_newton_cotes_window_refuses_typed`.

## Why it was fenced rather than built (TRIM-2 §7 Q2, ruled §8.2)

**No fixture reaches it.** Every chart a shipped construction mints and
a `General` image can land on is inside the window: the loft/sweep walls
are `(1, 2)` and `(2, 2)`, and TRIM-2's own fixture — the P-2 body's
degree-2 re-widening — is `(2, 2)`. The composite fallback doubles the
engine (a second rounds schedule with its own convergence story) for a
class nothing produces, and an unexercised second lane is a claim.

## What the fallback would be

The composite trapezoid rounds the spec sketches: `h·f(m) + F₂·h³/24`
per sub-chord, `F₂ ⊇ sup |d²/du² G_f(u, ℓ(u))|` over the sub-chord, by
the chain rule through the chart's own derivative grids (`PatchGrid`'s
ladder already carries `S_uu`, `S_uv`, `S_vv` and one level above for
the flux integrand). Unlike the exact rule it converges rather than
terminating, so it needs its own lever in the round schedule and its
own `props_quad_*` metering — which is the half that makes it a unit
rather than an arm.

## Who reaches it

A STEP-imported trimmed NURBS with a degree ≥ 3 chart in either
direction and a non-iso boundary, or any future construction that mints
a `General` image on such a chart. Both are outside TRIM-2's fence
(§4 *Out*).

## Home

TRIM's, filed by TRIM-2 PR-1 at the moment the fence landed
(`docs/prompts/implementer-discipline.md` §6: disclosing a residue is
not scheduling it). The file `geom-brep/src/props/quad.rs` is PROPS's
ground; the fence is TRIM's because the lane is.
