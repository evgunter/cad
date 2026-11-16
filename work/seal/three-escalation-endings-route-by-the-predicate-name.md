---
id: three-escalation-endings-route-by-the-predicate-name
kind: issue
title: topo: escalation endings pick their decision by the Indeterminate's predicate name (join's cylinder axes, spline_region's carrier, chord_join's order, is_span_precision, the chart-singularity prefix match)
status: open
opened: 2026-11-09
priority: P3
cost: E
---



Found by the sweep of `a-tangent-reading-returns-a-verdict-and-only-a-record-is-contradicted` (branch `seal/tangent-verdict`) for predicate-string dispatch in `topo`, after the census's `same_solid_tangent_pair` stopped matching `diag.predicate`.

## What

D4 ¶1 (ii): "the decision is a closed type at its site, so its recourse is an exhaustive match, never a lookup by predicate name." Three sites outside test code pick a decision, and so an ending, by comparing an escalation's `Indeterminate::predicate` string:

- `crates/topo/src/boolean/join.rs`, the `Uncarried::Section(SectionError::Escalated(diag)) if diag.predicate == Some(geom_brep::CcDecision::Parallel.name())` arm: the cylinder pose's parallelism becomes `BooleanDecision::CylinderAxes`.
- `crates/topo/src/boolean/spline_region.rs`, `(diag.predicate == Some(CARRIER)).then_some(ContainDecision::Carrier)`: the carrier rows' escalation becomes the containment's carrier decision.
- `crates/topo/src/chord_join.rs` `order_ending`: `match diag.predicate { Some(FRAME_AXIS | FRAME_ARM) => lever_ending, _ => ending }`.

Each is an in-band escalation, not a verdict, so it ends in its lever either way; what the string picks is which decision's lever. A renamed row silently moves the refusal to the other ending.

## Shape

Have the deciding call hand back which decision escalated (a closed type beside the `Indeterminate`, as `geom_brep::CcDecision` already is for the cylinder pose), so each site matches it.

## Blind spot, and the second pass

The first pass matched `predicate == Some(`, `predicate != Some(`, `match <x>.predicate` and a `Some("…") =>` arm. It cannot see a name compared through a helper or a closure. A second pass for `is_span_precision(`, `predicate.is_some_and(` and `predicate.map_or(` found the same shape at three more sites, each a decision or a sentence picked by the name:

- `census::is_span_precision` (`crates/topo/src/census.rs`, a `matches!` over three row names), read by `boolean::ops::collinear_escalated` (`BooleanDecision::LineSpanPrecision` or `LineEdges`), by `ValidationError`'s `Display` (`validate.rs`, the `CensusEscalated` arm) and by the census itself;
- `validate.rs`, the chart-singularity pass's `Err(diag)` arm: `diag.predicate.is_some_and(|p| p.starts_with("curve_carrier_") || p.starts_with("curve_pair_span_"))` picks `Undecided::Boundary` or `Undecided::Side`, a prefix match on row names.

The class is wider than this row's title; the shape above fixes each the same way. Not searched: a name compared after being copied into a `String`, or one read out of `Debug`.
