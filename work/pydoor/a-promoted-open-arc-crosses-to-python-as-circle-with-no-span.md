---
id: a-promoted-open-arc-crosses-to-python-as-circle-with-no-span
kind: issue
title: a curve promotion crosses to Python as kind 'circle' with no span, so an open arc reads as a full circle
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by EXCH unit B's review (PR 7170). Step import now promotes an
OPEN rational-quadratic arc to its circle (`step_import::PromotedCurveKind::Circle`),
and the Rust side says which piece the carrier covers: an edge on it
trims inside the covered range, and a wireframe curve carries it
(`step_import::WireframeCurve::span`). The Python crossing of a
promotion (`pncad-py/src/tags.rs`, `promoted_curve_kind_tag`, read as
`CurvePromotion.kind` on an `ImportReport` row) says `circle` and
carries no span, so a Python reader of the ledger cannot tell an arc's
promotion from a full circle's.

What would close it: the row carries the covered range (or an `arc`
word beside `circle`), or the census says why a ledger reader needs
neither. The record is the curve's (one promotion per curve entity,
`CurvePromotion::curve`); an edge's own trim is the body's.
