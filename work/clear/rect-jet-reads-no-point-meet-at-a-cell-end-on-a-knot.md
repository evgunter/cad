---
id: rect-jet-reads-no-point-meet-at-a-cell-end-on-a-knot
kind: issue
title: rect_jet reads a cell ending exactly on a knot as not meeting the span past it, so at a knot of multiplicity >= the degree it misses the derivative the evaluator reads there
status: open
opened: 2026-11-01
priority: P2
cost: E
---


## What was found (SLIVER's fuzzer extension, 2026-11-01)

`NurbsSurface::rect_jet` (`crates/geom/src/surfaces/nurbs.rs`, its
`clip_to_span`) keeps a knot span only where the rectangle meets it with
extent (`a < b`). A cell whose end is exactly a knot meets the span past
that knot at one point, which is dropped. At a knot of multiplicity at
least the degree the first partials jump there, and `NurbsSurface::ders`
at that point reads the span past it. So the enclosure misses a value
the door's contract ("at every point of `u × v`") claims.
`rect_enclosure`'s own clip keeps a point meet (`a <= b`). `rect_jet`
excluded it because the old slope read divided by the meet's width, and
`SpanRow::restrict_derivative` no longer does.

Reproduced on main and on SLIVER's head alike:
`rect_jet_contains_every_sampled_partial_of_its_rectangle`
(`crates/geom/tests/surfaces/rect_enclosure.rs`), with `side`'s
knot-adjacent arm allowed to end ON a knot, at
`CAD_FUZZ_SEED=0x6d81d7e11d955871 CAD_FUZZ_EFFORT=200`. `S_v` at
`(0.1220…, 0.8443801491053619)` reads `x ≈ −2.89`, outside the
enclosure `x ∈ [0.127, 0.646]` of `(0.1220, 0.6684) × (0.84438014787,
0.8443801491053619)`. SLIVER's arm draws ends off every knot, so the
default suite does not hit it.

## What a fix needs

A ruling first. Should a point meet count? Is the monotone arm's
derivative owed at a measure-zero kink? Keeping the meet (`a <= b`) is
sound now. But it widens the jet on every cell that ends on a
`p`-fold knot (closed forms' enclosing nets, the bisection's dyadic
points), so verdicts and receipts on revolved and conic bodies may move.
