---
id: saturated-sphere-span-folds-short
kind: issue
title: "props/curved: a sphere meridian span past 2π folds SHORT — the saturated clamp leaves a sign whose zero set is not empty"
status: open
opened: 2026-09-02
github: 1601
refs: [1599, 723, MESH-12]
---

## From GitHub issue 1601

opened 2026-09-02, 0 comments.

**Found by MESH-11's review (PR [#1599](https://github.com/evgunter/cad/pull/1599), R2, executed on the head AND on the merge base) — pre-existing in the flux lane, outside MESH-11's fence; filed as the durable home.**

`sphere_meridian_span_levels` (and now the shared helper `sphere_meridian_pole_margins`) clamps `c_edge = cos(min(dt/2, π))`. For a span `dt > 2π` that gives `f = ⟨P, M⟩ + 1`, whose zero set is NOT empty (the comment says it is): `P = −M` is an interior point, `δ` past `t0` for a span `2π + 2δ`, and the chord copied onto that sign is ≈ δ·R rather than ≈ 0. Measured:

```
R2-RESIDUAL delta=0.375104 f=-2.22e-16 (chord≈0.3729, 3.73e-3 m at R) short=true
R2-SATURATED 400 spans: fold measured SHORT on 36        (head)
R2-BASE-SATURATED eps=1e-9: fold short on 36 of 400      (merge base, same probe)
```

The area comes out short by `(1 − cos δ)/2`, up to −3.5% at δ = 0.375, at all three ε rows and on `--features interval`. Probe rows: `mesh/11r2-probes` (`r2_a_saturated_span_with_the_pole_antipodal_to_its_midpoint`, `r2_the_saturated_span_sign_is_a_rounding_residual`).

**Reach:** hand-built or uncertified spans only — certification bounds `0 < Δt ≤ τ` and the import door normalises into `(0, τ]`. That is exactly the class CERT-1's `a_multi_wrap_span_covers_both_poles` exists for, and that row passes only because its `f` rounds to `+0.0`. MESH-11's one-chart-branch door is NOT affected (the other pole sits at `M`, `f = 2`, a definite `Positive`).

**Owed:** either refuse a span past `2π` at the parse (the winding invariant certification already enforces, made a decide here) or make the saturated case genuinely fold both poles; correct the two doc sentences MESH-11's fix pass re-homes; a row on both sides of `2π`. Track R / S-CERT ground (the closed form's premise).

Refs #723 (the class), CERT-1, MESH-11.

## Home

`work/mesh/` — although the issue names Track R / S-CERT ground, `work/mesh/MESH-12.md` is the unit that took it ("Issue 1601: a sphere meridian span past the per-edge winding bound refuses typed at the parse"), so it lives on S-MESH's slate.
