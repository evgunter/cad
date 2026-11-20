---
id: chart-coherence-reads-a-non-iso-circle-as-an-iso-side
kind: issue
title: the chart-coherence examination reads a non-iso circle as a rim or a meridian, and reports metre-scale findings on a cut sphere
status: open
opened: 2026-11-20
priority: P1
cost: M
refs: [chart-coherence-ships-off-and-nothing-schedules-turning-it-on]
---


(TESSLIE implementer, `tesslie/coherence-default`, 2026-11-20.) Filed
while measuring whether `CheckId::ChartCoherence` can default to
`Warn`. **This row BLOCKS
`chart-coherence-ships-off-and-nothing-schedules-turning-it-on`**: it is
the one reason left for that row's default staying `Off` (stated at
`ChecksConfig::chart_coherence` in `crates/editor-core/src/checks.rs`),
so it carries that row's P1. The flip to `Warn` and the `skipped`
re-baselines land with this row's fix.

## The defect

`topo::chart_iso::classify_kind` classifies a carrier by KIND. A line
reads as a meridian. A circle reads as a rim when
`|axis · chart axis| > 0.5` and as a meridian otherwise, whether or not
the circle is an iso curve of the chart at all. `mesh` only reaches it
behind `props`' iso-rectangle door, which tests iso-ness exactly
(`geom_brep::props::require_iso_rectangle`, `IsoChart::reads`).
`topo::examine_chart_coherence` has no door and reads every
chart-bearing face, so a non-iso circle gets measured against a
premise the face never had.

## The witness (measured, `mesh/tests/tesslie_coherence_nearest_fit.rs`)

The probe builds a unit sphere (axis z) cut along ONE oblique circle,
using `oblique_cap`: two arcs of the circle bound two sphere faces. The
split door refuses a sphere (`CurvedBooleanUnsupported`), so the probe
uses the Euler doors. A STEP sphere trimmed by an oblique plane states
the same carrier.

| tilt of the cut | circle `\|cos\|` | read as | findings at ε = 1e-9 |
|---|---|---|---|
| 0°, 30°, 59° (offset 0.3 and 0) | 1, 0.866, 0.515 | Rim | 0 |
| 61°, offset 0.3 | 0.485 | Meridian | 10: `MeridianClosure` 1.29–1.82 m, `MeridianContinuation` 3.11 m |
| 61°, offset 0 | 0.485 | Meridian | 10 |
| 75°, offset 0.3 / 0 | 0.259 | Meridian | 8 / 10 |
| 90°, offset 0.3 | 0 | Meridian | 8: `MeridianClosure` ~1.27 m |

All of these are spurious, because the body states each coordinate
once, exactly. (At 90° with offset 0 the circle is a true meridian
great circle. That is the known pole-crossing half-turn class, not this
defect.) `mesh::tessellate` refuses every one of these bodies, so the
mesh is unaffected and only the report misreads.

The rim side is quiet in the witness only because both arcs share one
carrier. Two DIFFERENT non-iso circles read as one rim run would
compare two unrelated `rim_v` values. That case is unmeasured.

Cylinder and cone cuts at every tilt tried give ellipse carriers, never
circles (and the cone refuses past its generator angle:
`OpenConicChord`). So the exposure is the sphere, plus the torus with
its Villarceau and other non-iso circles, which is unmeasured.

## The fix's shape

`classify_kind` would answer `None` for a line or circle that is not
an iso curve of the chart. Such a carrier then becomes a run boundary
that is not measured, as a conic already is since this branch. The
exact test exists in `props` (`IsoChart::reads` and the per-kind
boundary parses). It is banded and escalates, and this door has no
band semantics today. `mesh` shares `classify_kind` and only calls it
behind the props door, so the stricter answer cannot change a mesh.
Whether the escalation arm belongs in the report as an `unexamined`
reason, or the exact test is read at the report's ε, is the design
question.

When this lands, re-run the probe above and the
`editor-core/tests/tesslie_coherence_cost.rs` census. If both are
quiet, the default flip in `checks.rs` (FIX/coincide's ground,
announced) and the `skipped` re-baselines are all that remain of
`chart-coherence-ships-off-and-nothing-schedules-turning-it-on`.
