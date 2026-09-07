---
id: pcurve-chart-phase-is-atan2-of-the-start-radial
kind: issue
title: "the plate's last identity residual is the chart's phase: pcurve_map_residual carries atan2(0, ‖a_r‖) from the cylinder chart derivation, which no value-free rule folds"
status: closed
opened: 2026-09-07
closed: 2026-09-07
refs: [M10-10, plate-ceiling-is-now-the-scaffold-pushforward]
---

**Closed by M10-10 under spec amendment A1** (`docs/M10-10-SPEC.md`),
which took the third route below — the syntactic positivity — into
rule D: `atan2(Z, N)` with `Z` the zero form and `N` manifestly
non-negative by its syntax (`sqrt`/`abs` atoms, even powers, positive
coefficients, perfect squares, and their products, quotients and sums;
`geom_core::sym::trig::manifestly_nonneg`) is the zero form, and —
what the branch-stabilized azimuth leaves behind on a
definitely-negative frame, `atan2(−y, −x) + π` — `sin`/`cos` at an
exact half-multiple of π is its constant (`trig::fold_at_half_pi`).
The soundness argument the row asked for is at the impl: `N` is `> 0`
at every parameter point where it is defined and not zero, so the
fold is a theorem there; the `N = 0` point, where `atan2` is
discontinuous, is a degenerate geometry clause 1 refuses first, and
IEEE's `atan2(0, 0) = 0` agrees wherever a value exists. Pinned:
`atan2(0, sqrt(X))`, `atan2(0, X²)`, `atan2(0, r²/sqrt(r²))` decide
Zero at every ring width and for a straddling offset with `r > 0`;
`atan2(0, X)` for a plain parameter never; `atan2(Y, N)` with `Y` not
the zero form never; a numeric-only zero never; the `N = 0` box
refuses through clause 1
(`geom-core`'s `sym::tests::rule_d_folds_atan2_of_the_zero_form_over_a_manifestly_nonnegative_form_and_nothing_else`,
`rule_d_folds_trig_at_half_multiples_of_pi_and_nothing_else`,
`tests/m10_10_atan2_interval`).

**What it bought, measured** (`m10_10_pins_interval`): at the plate's
nominal `pcurve_map_residual` 0/0/0/36 → 0/0/36/0 — the phase folds,
and the rim identity the door states closes the rest, so the count is
`registered`. The plate's whole-certifying box, shipped tier, nothing
passed: 0.2368 (ε = 1e-6), 0.2631 (1e-9), 0.2631 (1e-12) of its REAL
study, bounded by `assert_bound` `[9.99e-6, 1.90e-4]`, `[−2.09e-9,
2.00e-4]`, `[−9.12e-9, 2.00e-4]` — exactly the staged walk's end this
row predicted, to the bisection step. R1's annulus: 0.6963 / 0.8416 /
0.8415 of its real study, bounded by `dihedral_wedge` and
`arc_diameter_clearance`, real margins both. The other two routes
(a sign read; a structural phase in PCURVE's derivation) are not
needed for this residual and are not taken. The finding below is
kept as the record of what stood and why.

---

**Found by M10-10**, which built the form-level mechanism (rule D —
`sin`/`cos` of `q · atan(X)` in closed form — with rules A/B per node)
and measured the two-hole plate's four identity residuals one by one.
Three of the four went at the first cut: `carrier_on_surface_2` and
`witness_on_surface_2` as theorems, `carrier_matches_mapped_source`
through the door. The fourth did not, and this row says exactly why
and where.

## The residual, rendered

`pcurve_map_residual` (`crates/geom-brep/src/certify.rs`, the
`Resolved::Chart` arm: `p.distance(surface.eval(q))`, `q =
chart.pcurve.eval(t_i)`) on the plate's four chart-described rim arcs,
at every one of the nine samples. Under the first cut's tier its early
form is `sqrt` over three frozen component squares; explained to the
component (`m10_10_evidence_interval::m10_10_the_four_residuals_rendered_at_the_nominal`
with `CAD_M10_10_EXPLAIN=6`), the x-component is

```
surface.eval(q).x = ( −ρ·cos(atan2(0, ‖a_r‖))·(k + hole_a_r)
                      + ‖a_r‖·(k + hole_a_r) ) / ‖a_r‖  + …
p.x               = −k' − half_spacing − hole_a_r
```

with `ρ = |‖chord‖/2|` the arc's radius and `‖a_r‖ = r²/sqrt(r²)` the
edge start's radial length in the chart frame (`k`, `k'` the 53-bit
nominals). The two sides are equal exactly when `cos(atan2(0, ‖a_r‖))
= 1`, i.e. when `atan2(0, ‖a_r‖) = 0`, i.e. when `‖a_r‖ > 0`.

## Where the atom comes from

The cylinder chart derivation
(`crates/geom-brep/src/pcurve_cache.rs`, `stable_azimuth` — the
branch-stabilized azimuth every chart's derivation shares since M6-3)
computes the phase of the edge start in the chart frame as
`atan2(a_r · v_ref, a_r · u_ref)`. For the extrude's wall the chart's
`u_ref` IS the start's own radial direction (`rim.normalize()`,
`crates/sweep/src/extrude.rs`), so `a_r · v_ref` is the zero form and
`a_r · u_ref = ‖a_r‖`, and the phase is `atan2(0, ‖a_r‖)` — zero for
every real parameter point, because `‖a_r‖ > 0` wherever the arc is
defined. Before A1 the form could not say so: `atan2(0, X)` is `0` or
`π` by the SIGN of `X`, and `geom_core::sym` folds no atom on a sign
it has not read (the same line that keeps rule C dial-off). Two of
the plate's four rims sit on the definitely-negative frame, where the
azimuth adds `π`: with the `atan2` folded their phase is `π` itself,
which is the half-π fold's case.

## What would discharge it, and whose it is

- **A sign read** — rule C's kind: the funnel has already decided
  `pcurve_chart_azimuth_frame` on `‖a_r‖` (Positive, 4 decisions on
  the plate's nominal), so the sign IS certified over the box, and a
  fold `atan2(0, X) → 0` on a certified-positive `X` would be sound
  and counted `sign_gated`. Not taken: a value-free route exists.
- **A structural phase** — the chart derivation stating `u₀ = 0` when
  the chart's `u_ref` is the edge's own start radial, rather than
  re-deriving it through `atan2`. That is PCURVE's construction
  (edge-description migration is PCURVE's; the derivation is the
  certifier's side), not a form-level rule, and it is the same-object
  shape E12's reserve describes: the constructor that built `u_ref`
  from this rim knows the phase. Not needed for this residual.
- **A syntactic positivity** — `atan2(0, X) → 0` when `X`'s form is
  manifestly non-negative (a quotient of products of `sqrt` atoms,
  even powers and positive constants). Value-free; its soundness
  argument (the `X = 0` point, where `atan2` is discontinuous, must be
  one clause 1 already refuses) is stated at the impl. **Taken, by A1.**

## The numbers

| | with the algebra off (M10-9) | with it on, first cut | with A1 (shipped) |
| --- | --- | --- | --- |
| plate, whole-certifying ceiling | `[7.811e2, 7.814e2] · ε` | `[1.2497e3, 1.2505e3] · ε` (three ε rows) | 0.2368 / 0.2631 / 0.2631 of the REAL study (ε = 1e-6 / 1e-9 / 1e-12) |
| over the band at ceiling + δ | `carrier_matches_mapped_source` `[0, 1.0001 · ε]` | `pcurve_map_residual` `[0, 1.0004 · ε]` (2/2) | `assert_bound` `[9.99e-6, 1.90e-4]`, `[−2.09e-9, 2.00e-4]`, `[−9.12e-9, 2.00e-4]` — a real margin |
| `pcurve_map_residual` at the nominal | 0/0/0/36 (theorem/gated/registered/numeric) | 0/0/0/36 | 0/0/36/0 |
| with `pcurve_map_residual` passed (staged dial) | — | certifies 0.2368 (ε = 1e-6), 0.2630 (1e-9), 0.2631 (1e-12) of the real study; beyond it `assert_bound` `[1.0e-5, 1.9e-4]`, `[7.29e-9, 2.0e-4]`, `[−4.1e-8, 2.0e-4]` | the same, with nothing passed |

R1's annulus was bounded by the same residual at `[1.2455e3, 1.2470e3]
· ε` (1/27 over the band); with A1 it certifies 0.6963 / 0.8416 /
0.8415 of its real study.
