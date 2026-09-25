---
id: a-poisoned-curve-carrier-datum-is-refused-at-rest-only-by-accident
kind: issue
title: A curve carrier's poisoned or out-of-convention datum at rest is refused only by check 2's residual, which names the edge and not the datum
status: review
opened: 2026-09-24
priority: P4
cost: D
refs: [ATREST-6]
parent: ATREST-13
---

## What

The sibling of `quadric-datums-unchecked-at-rest`, one dimension
down, found by ATREST-6's sweep and NOT measured.

Check 1 now names an analytic SURFACE whose stored datum describes no
locus. Nothing names a CURVE's: a circle or ellipse radius that is
NaN, zero or negative, or a line `dir` that is zero, reaches tier 3
only through check 2's carrier re-certification, whose refusal is
`ValidationError::EdgeCertification` with a `CertifyError` — and no
`CertifyError` variant names a carrier datum
(`crates/geom-brep/src/certify.rs`, `CertifyError`: the variants are
residual, transversality, winding, escalation and lane refusals).
Attachment certifies a carrier at mint, so the path in is the raw
arena swap `tier3_tests.rs`'s `wrong_cache_at_rest_is_rejected_by_tier3`
already uses.

`validate_geometric`'s not-yet list (`crates/topo/src/validate.rs`,
"Curve conventional-invariant certification") names the unit-frame
half of this and argues it is "partially implied by the residual
checks"; the poison half is the same argument and the same accident.

## What is owed first

The measurement: a pillow edge's carrier swapped for a circle of each
bad radius, and what tier 3 answers — before deciding whether check 2
owes a named datum refusal or a check of its own.

## Fence

Track P. `crates/topo/src/validate.rs` (check 2).

## Answered (ATREST-13)

**The measurement** (the pillow of `tier3_tests.rs`, one chord
re-minted through the public `Body::set_edge_curve` as a half-arc or
line between the chord's two vertices, chart-described; a throwaway
test, CI run 36148485586):

| carrier | mint | at rest |
| --- | --- | --- |
| line, `dir` zero | `ResidualExceeded { EndpointEnd }` | — |
| line, `dir` NaN / `+∞`; `origin` NaN | escalated `carrier_endpoint_start` | — |
| line, `dir = 2·x̂` (`t ∈ 0…½`) / `½·x̂` (`0…2`) | certifies | `Ok(())` |
| circle, `radius` NaN | escalated `interval_span_forward` | — |
| circle, `radius` `+∞`; `axis` NaN; `center` NaN | escalated `carrier_endpoint_start` | — |
| circle, `radius` 0 / −½ | `IntervalNotForward` | — |
| circle, `u_ref` zero | `ResidualExceeded { EndpointStart }` | — |
| **circle, `axis` zero** | **certifies** | **`Ok(())`** |
| circle, `axis = 2·ẑ` (an ellipse) | certifies | nothing names the datum |
| ellipse, `minor` NaN / 0 / −0.3; `major` NaN; `minor` `+∞` | refused (as the circle's) | — |
| **ellipse, `major` −½ with `u_ref` flipped** | **certifies** | **`Ok(())`** |
| **ellipse, `axis` zero** | **certifies** | **`Ok(())`** |
| ellipse, `minor > major` | certifies | `Ok(())` |

So certification is the net under most poisoned carriers — at MINT,
since `EdgeCurve::certify` is the only door into an arena and check 2
re-runs the same derivation — and it lets through exactly the carriers
whose residuals are clean while their datums describe no curve of
their kind.

**Refused by name at check 1**, one variant pair mirroring the surface
pair: `ValidationError::PoisonedCurveDatum` (a datum not a number, or
a zero `dir`/`axis`/`u_ref`) and `UnrepresentableCurveDatum` (a
non-positive circle radius, ellipse semi-axis or spiric tube radius,
through `geom::Curve3::representability_margins`, and — ATREST-13's
D-1 applied to carriers — a circle's, ellipse's or spiric's frame off
unit or off `u_ref ⊥ axis` by more than ε at its largest radius). The
same doors as the surface half (`is_finite_length`, `is_zero_length`,
the one `Bounds::lo` read in `analytic_datum_verdicts`), edge-arena
order, after the face pass. Pinned by
`check_1_names_the_carrier_datum_that_describes_no_curve` (the four
certifying rows: circle zero axis, ellipse zero axis, ellipse
negative major, circle `axis = 2·ẑ`) and
`the_carrier_datum_read_names_every_datum_that_describes_no_curve`
(the rows no mint lets through, and the line and spiric kinds).

Left out and filed: a line's unit `dir` (`unlevered-frame-conventions-are-uncertified-at-rest`)
and the ellipse's `major > minor` ordering
(`an-ellipse-stored-minor-over-major-passes-tier-3`).
