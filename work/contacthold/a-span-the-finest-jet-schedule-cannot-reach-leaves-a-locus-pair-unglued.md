---
id: a-span-the-finest-jet-schedule-cannot-reach-leaves-a-locus-pair-unglued
kind: issue
title: A tangency whose span bound the finest jet schedule cannot reach is left unglued and meets the frontier refusal
status: closed
branch: contacthold/glue-door-escalates
pr: 4918
opened: 2026-10-13
priority: P2
closed: 2026-10-13
---



## What

`contact_verify::tangent_table` reads a span whose only unsettled
reading is the jet schedule's sag again on finer schedules, up to
`REFINE_HALVINGS` (sixty-four spans to each of the nine-sample
schedule's, the sag down by 4096). A span still past the band there
refuses `NotCertifiable` (`SAG_UNREACHED`), which `verify_tangency`
reads as `TangencyFailure::Unsupported`, so the glue door leaves the
pair unglued, and it meets the frontier refusal later
(`CurvedPierceUnsupported`), although the witness lane has a locus for
it. C8 (`crates/topo/README.md`) says such a pair never reaches that
refusal.

## Measured (PR 4918, 2026-10-13)

The G1 tube chain (`sweep::mate7a_torus_rest`, `segment_a` /
`segment_b`): two tori sharing a meridian circle (radius 0.06). Every
sample lies on both carriers (`|r| ≤ 8·10⁻¹⁶`); the circle arm's
residual sag over the whole circle is 1.21 and goes as the step
squared: 4.7·10⁻³ at 16 spans, 1.9·10⁻⁵ at 256, 7.2·10⁻⁸ at 4096
(band `ε = 10⁻⁹`, `K·ε = 10⁻⁸`). Within ε needs about 35 000 spans.
The tangency is exact; the bound is what cannot reach it. This is the
meridian class `geom_brep::tangent_certificate_lane`'s docs name
"in-lane but uncertifiable".

## Ways out

A meridian-aware span bound (one that is exact for a non-coaxial
circle on two surfaces of revolution), or an adaptive schedule with a
budget argued from the bound's own rate. Either is the certificate's
lane, not the door's.

## Resolved (PR 4918, second fix pass)

The door refuses a sag-only `NotCertifiable` typed
(`verify_tangency`: `UnsupportedDeclarationClass { Tangent }`), so the
pair never meets a later stage unglued. Measured first: no fixture in
topo, sweep, editor-core, verbs or pncad reaches the door with a
sag-only refusal at ε = 10⁻⁹, nor topo or sweep at 10⁻⁶ or 10⁻¹². The
G1 tube chain no longer does: read on every sample, it escalates
`contact_tangent_second_order` at its meridian's poles, where the two
tubes osculate (`κ_rel` exactly zero), and that escalation answers at
the door (`Coincidence(Contact)`). No `Ok` answer moved.

