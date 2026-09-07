---
id: pcurve-chart-phase-is-atan2-of-the-start-radial
kind: issue
title: "the plate's last identity residual is the chart's phase: pcurve_map_residual carries atan2(0, ‖a_r‖) from the cylinder chart derivation, which no value-free rule folds"
status: open
opened: 2026-09-07
refs: [M10-10, plate-ceiling-is-now-the-scaffold-pushforward]
---

**Found by M10-10**, which built the form-level mechanism (rule D —
`sin`/`cos` of `q · atan(X)` in closed form — with rules A/B per node)
and measured the two-hole plate's four identity residuals one by one.
Three of the four go: `carrier_on_surface_2` and `witness_on_surface_2`
as theorems, `carrier_matches_mapped_source` through the door. The
fourth does not, and this row says exactly why and where.

## The residual, rendered

`pcurve_map_residual` (`crates/geom-brep/src/certify.rs`, the
`Resolved::Chart` arm: `p.distance(surface.eval(q))`, `q =
chart.pcurve.eval(t_i)`) on the plate's four chart-described rim arcs,
at every one of the nine samples. Under the shipped tier its early form
is `sqrt` over three frozen component squares; explained to the
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
defined. The form cannot say so: `atan2(0, X)` is `0` or `π` by the
SIGN of `X`, and `geom_core::sym` folds no atom on a sign it has not
read (the same line that keeps rule C dial-off). Its own comment at
the `Atan2` arm says exactly this.

## What would discharge it, and whose it is

- **A sign read** — rule C's kind: the funnel has already decided
  `pcurve_chart_azimuth_frame` on `‖a_r‖` (Positive, 4 decisions on
  the plate's nominal), so the sign IS certified over the box, and a
  fold `atan2(0, X) → 0` on a certified-positive `X` would be sound
  and counted `sign_gated`. M10-10's ruling was that no rule in it
  reads a value; this is the row that says what the next value-reading
  rule would buy: the plate's whole-certifying box from `1.25e3 · ε`
  to 0.237–0.263 of its REAL study at all three ε rows, bounded by the
  assertion's own margin (the staged walk with this one residual
  passed: `m10_9_r2_probes_interval::r2_evidence_plate_ceiling_with_identities_passed`).
- **A structural phase** — the chart derivation stating `u₀ = 0` when
  the chart's `u_ref` is the edge's own start radial, rather than
  re-deriving it through `atan2`. That is PCURVE's construction
  (edge-description migration is PCURVE's; the derivation is the
  certifier's side), not a form-level rule, and it is the same-object
  shape E12's reserve describes: the constructor that built `u_ref`
  from this rim knows the phase.
- **A syntactic positivity** — `atan2(0, X) → 0` when `X`'s form is
  manifestly non-negative (a quotient of products of `sqrt` atoms,
  even powers and positive constants). Value-free, but a new rule
  family with its own soundness argument (the `X = 0` point, where
  `atan2` is discontinuous, must be one clause 1 already refuses), and
  outside M10-10's scope.

## The numbers

| | with the algebra off (M10-9) | with it on (M10-10) |
| --- | --- | --- |
| plate, whole-certifying ceiling | `[7.811e2, 7.814e2] · ε` | `[1.2497e3, 1.2505e3] · ε` (three ε rows) |
| over the band at ceiling + δ | `carrier_matches_mapped_source` `[0, 1.0001 · ε]` | `pcurve_map_residual` `[0, 1.0004 · ε]` (2/2) |
| `pcurve_map_residual` at the nominal | 0/0/0/36 (theorem/gated/registered/numeric) | 0/0/0/36 |
| with `pcurve_map_residual` passed (staged dial) | — | certifies 0.2368 (ε = 1e-6), 0.2630 (1e-9), 0.2631 (1e-12) of the real study; beyond it `assert_bound` `[1.0e-5, 1.9e-4]`, `[7.29e-9, 2.0e-4]`, `[−4.1e-8, 2.0e-4]` — a real margin, and at `1e-12` a genuine flip |

R1's annulus is bounded by the same residual at `[1.2455e3, 1.2470e3] ·
ε` (1/27 over the band).
