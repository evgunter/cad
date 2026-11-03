---
id: a-spline-window-restriction-has-four-spellings
kind: issue
title: The spline-window restriction is spelled four times: SpanLocate::restrict_window, spline::pieces::span_row, NurbsCurve3::restricted and (until PR 6399's fix pass) NurbsCurve3::restricted_nets
status: open
opened: 2026-11-03
priority: P1
cost: M
refs: [binomial-bernstein-product-and-blossom-have-two-homes-each]
---


Found by PR 6399's delta review (BOXES `cover-edge-doors`, 2026-11-03).

Restricting a spline to a parameter window (de Boor's blossom) has four homes:
- `geom_core::SpanLocate::restrict_window` (`crates/geom-core/src/spline/locate.rs`): the per-scalar door. It runs the plain recurrence at f64 and the certified `SpanRow::restrict` at Interval.
- `geom_core::spline::pieces::span_row`, generic, read by `section_bounds.rs`.
- `geom::NurbsCurve3::restricted`, Interval-only, which certifies each restricted weight positive.
- `geom::NurbsCurve3::restricted_nets`, written in PR 6399 with no weight check. Its third fix pass retires it onto `restrict_window`.

The closed row `binomial-bernstein-product-and-blossom-have-two-homes-each` folded the span blossom's two homes in `geom_core::spline`. This is the same question one level out: which door restricts a curve to a window, at which scalar, with which weight certificate. P1: two implementations of one logic, and spline readers keep adding one.

Signed: (BOXES orchestrator)
