---
id: chart-bound-outer-span-decides-a-poisoned-margin
kind: issue
title: chart_bound_outer_span poses a check that is structurally unanswerable at a point scalar, and the K sweep records the poison (9 rule-1 flags per eps row)
status: open
opened: 2026-09-29
priority: P1
cost: M
---

## What fires

`k-lint (dev-probe)` flags nine rows at **every** eps row — identical
at 1e-6, 1e-9 and 1e-12 — with `outcome=invalid` and `margin=NaN`:

| shape | predicate | rows/eps | recorded margin |
| --- | --- | --: | --- |
| `corpus/boss_union` | `chart_bound_outer_span` | 3 | `NaN` |
| `demo/bossplate` | `chart_bound_outer_span` | 3 | `NaN` |
| `demo/lily_walls` | `chart_bound_outer_span` | 3 | `NaN` |

Readings: nightly 36561506133 (job 109383464088, head `aae5716bf`) and
a dispatch at a later main, run 36691873111 (job 109810896182), whose
sweep is 145,464 samples larger and flags the same nine to the digit.

## The mechanism, which is established and is NOT a geometry regression

The margin is `u_arm.meter(hull.u_max - hull.u_min - p)` at
`ChartBound::assembled` (`crates/topo/src/chart_bound.rs`, the `SPAN`
const), where `hull` comes from the outer loop's `window()`. The NaN
enters upstream of that, and it is put there on purpose:

1. `chart_edge` (`crates/topo/src/pcurves.rs`) mints an
   `ChartEdge::Envelope`'s `image` by calling
   `SpanLocate::enclosure_hull` **directly** — on the
   `Fitted | General` arm (`hull.u_min.enclosure_hull(hull.u_max)` and
   the `v` twin) and on the closed-form arm
   (`walked.pcurve.eval(walked.t0.enclosure_hull(walked.t1))`).
2. At a point scalar that impl returns `f64::NAN` deliberately —
   "poison, never a fabricated point value" — because a hull of two
   distinct point results is not a point.
3. `Real::min`/`Real::max` propagate NaN by policy: `real.rs` says
   NaN-dropping "would silently launder a poisoned value, defeating the
   module-level NaN policy", pinned by `min_max_nan_propagation`.
4. `ChartEdge::window()` hulls `image` into the chord box, so the
   loop's `u_min`/`u_max` are NaN, and so is the span margin.

The K sweep runs at `Probe`, which is an `f64` with a recorder. So a
NaN in the CSV means exactly this, and it follows for **any** face
whose outer loop carries one non-straight edge on a chart with a
period.

**Read that consequence before grepping `chart_bound.rs`:** these nine
are structural at the recording scalar, not geometry that moved. They
will reappear on every future sweep of these three shapes, and nothing
in `chart_bound.rs` is wrong on its own terms.

## Which merge, and it is a grep rather than a bisect

The row was green at run 36449836735 (job 109022798206, executed
2026-09-28T16:23:27Z–16:30:32Z), which flagged rule 1 zero times over
3,789,703 samples. The question is therefore not "what broke the
arithmetic" but **which merge gave one of these three shapes a curved
outer-loop edge on a periodic chart** — one grep over the window's ~85
merges, not a sweep per step.

A candidate for three of the nine is already in hand: PR #3375
(`38588fede`) rewrote `demos/tour/src/lily.rs`, and `demo/lily_walls`
carries three of the NaNs as well as all eight of the run's rule-2
flags (`work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md`).
`corpus/boss_union` and `demo/bossplate` are unattributed.

## Three readings, and the third is the likely one

1. The margin must never be poison here — the arm or the hull that
   feeds it is the thing to fix.
2. `docs/predicate-dimension-audit.md`'s `chart_bound_outer_span` row
   is right that "`Zero`/in-band/poison let it stand", and k-lint is
   wrong to call `invalid` a defect at this site.
3. **The audit is correct at the site and the defect is elsewhere: a
   check that is structurally unanswerable at the recording scalar is
   being posed there at all, and recorded as a K decision.** The
   closed-form arm of `chart_edge` says as much in its own words — "At
   a POINT scalar the span hull is poison, the box is poison, and the
   outside test certifies nothing against it — which is the safe
   direction". If poison is the intended and safe answer, the row that
   should not exist is the CSV row, not the refusal.

Reading 3 moves the question off CHART and onto the probe's scope —
what the recording scalar is entitled to be asked — and may be a design
fork rather than a fix. **This row does not decide between the three.**

## Done here

`SpanLocate::enclosure_hull`'s doc claimed "Point scalars (`f64`,
`Probe`, `Dual<f64>`) never reach it by construction (single span)".
That is **false on this tree** — `chart_edge` reaches it at a point
scalar on both arms — and it is a premise other code may be citing.
**Fixed**, in the PROPS k-lint baseline PR: the sentence now says the
claim holds through the evaluators and explicitly not in general, and
names the direct caller. `crates/geom-core/src/spline/locate.rs` is
double-claimed by `nurbs` and `props`, so this is not a crossing.

## Home

CHART — `crates/topo/src/chart_bound.rs` is chart's territory
(`scripts/work.py territory`). Found by the PROPS k-lint baseline unit,
which measured but does not fix kernel geometry, and does not own
reading 3's question either.

## 2026-10-03 — more rows with tier 3 at the boolean door (REACH)

With the boolean door gating its result at tier 3 and callers finishing
operands (`boolean-door-adopts-the-finished-body-type`), the sweep
records 43 `chart_bound_outer_span` rule-1 flags per ε row (1e-6, 1e-9):
projectbox_cutaway 16, tiltedcut 12, lily_walls 6, lily 3, bossplate 3,
boss_union 3. Tier 3 asks the predicate on every body it validates.
`work/reach/k-lint-reads-the-boolean-doors-tier-3-at-probe.md`.
