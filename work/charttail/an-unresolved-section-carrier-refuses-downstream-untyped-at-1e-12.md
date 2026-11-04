---
id: an-unresolved-section-carrier-refuses-downstream-untyped-at-1e-12
kind: issue
title: A section ellipse the join reads unresolved refuses downstream, untyped, where its end pins happen to land
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Found by ONECOPY's root cause of `main-red-a-section-ends-own-rounding-escalates-at-1e-12-after-angle-reads`.

## What

A plane nearly along a cylinder's axis, `θ` off it, cuts an ellipse of semi-major `r/θ` whose centre lies about `r/θ` from the face. At `θ = 1e-5` the split's own carrier `C(t) = O + a·cos t·u + b·sin t·v`, with `|O| ≈ 3e4`, rounds by `ulp(3e4) ≈ 3.6e-12` where its arc is used. That is past the `ε/K` share at ε = 1e-12. The join types this carrier as a kernel limitation, but only where an **end pin** is the check that meets the rounding: `topo::chord_join`'s `section_pin_refusal` reads `geom_brep::carrier_resolution::resolution` against `validate::resolved` and returns `SplitJoinError::SectionUnresolved`.

Whether an end pin meets it depends on the end parameter's last bit. Where both ends pin (a miss of about `2.5e-16`), the same rounding is met later, by checks that do not read the carrier's resolution:

- **the pcurve cache's map residual** (`geom_brep::pcurve_cache`'s `schedule_residuals`, `pcurve_map_residual`). The harmonic image's `v = c + b·sin t` with `c ≈ 3e4` and the carrier's `z` each round at `ulp(3e4)`. The split then refuses as `SplitError::Pcurves(Certify { Escalated { MapResidual } })`, which renders "the pcurve of half-edge … failed certification: … escalated". In other poses it refuses as `ResidualExceeded`. Neither refusal names the kernel limitation or `KERNEL_LIMIT_RECOURSE`;
- **the side's volume** (`REFUSED finish … ResultNotAtRest { VolumeUncomputable }`), also untyped.

## Evidence

`NT_FAMILY=axial CAD_TOLERANCE_EPS=1e-12 cargo run -p sweep --release --example section_end_census_probe`, 704 poses:

| outcome | `e9eaae65` | main `e931c7d3` |
|---|---|---|
| `UNRESOLVED` (end pin, typed) | 83 | 82 |
| `UNRESOLVED-PAST` (typed) | 119 | 120 |
| `REFUSED pcurves` (untyped; `MapResidual`) | 17 | 20 |
| `REFUSED finish` (untyped) | 149 | 146 |
| `ok` | 336 | 336 |

PR 6482's angle read moved last bits and so moved poses between these buckets. For example, `w0.3 phi0.4 th1e-5 z0.3` went from `UNRESOLVED EndpointEnd 3.64e-12` to `REFUSED pcurves`. But the untyped buckets were already populated at `e9eaae65`. `section_end_poses::a_section_ends_own_rounding_is_a_kernel_limitation` pins only witnesses whose end pin is the first check to meet the rounding.

## Owed

Decide where an unresolved section carrier's rounding is typed so the result does not depend on which check meets it first. Candidates:

- carry the join's resolution reading on the edge, so the pcurve mint and the at-rest volume can read it;
- or read `carrier_resolution` at the pcurve mint's refusal, the way `section_pin_refusal` does.

The sibling for the square-to-axis reading is HONEPOSE's `a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12`.
