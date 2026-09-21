---
id: camera-readout-spells-the-millimetre-factor-itself
kind: issue
title: The camera readout spells the metre-to-millimetre factor itself
status: open
opened: 2026-09-21
priority: P2
cost: E
---



## Finding

The view pane's camera readout (`crates/viewer/src/pane/view.rs`,
`ViewPane`'s camera block) converts metres to millimetres with its own
literal:

```rust
let mm = |metres: f64| crate::readout::number(metres * 1000.0);
```

`crates/viewer/src/scene.rs` states that factor once as
`scene::MM_PER_METRE`, and its doc calls it *"the one factor the δ
render and the δ door share"*. `DisplayTolerance::render_mm` is the
same two operations against the named constant —
`readout::number(self.0 * MM_PER_METRE)` — so the readout here is a
second home for a multiply that has a first one, spelled `1000.0`
where the constant is spelled `1.0e3`.

The number is right today. What it costs is that the constant no
longer covers every metre-to-millimetre conversion the viewer makes,
which is what its doc claims; a change to the rendering unit of the
camera band reaches one of the two and the reader of the constant
cannot tell.

**This is not the argued-opposite case.** A row that restates a
factor to check the code applies it (`display_budget.rs`'s
`reads_back_as_a_delta`, which spells `1.0e3` deliberately so that
`render_mm` is checked against something independent of itself) is a
threshold the row chooses. This site is production code with no such
argument: it is not checking `MM_PER_METRE`, it is applying it.

## Home

CHROME. `crates/viewer/src/pane/view.rs`; the constant it should read
is `crates/viewer/src/scene.rs` (`scene::MM_PER_METRE`).
`work.py territory` reports the path as `chrome, vgeom, view` — a
double claim, so CHROME can take it.

## What the sweep that found it could not see

The pattern was the literal spellings `1000.0`, `1_000.0` and `1.0e3`
plus the name `MM_PER_METRE`, across `crates/viewer/`. It cannot see
a conversion that computes the factor (`10f64.powi(3)`), one that
arrives through another crate's helper, or a metre-to-millimetre step
folded into a larger expression with no isolated literal.

## Found by

CHROME-ONE-NUMBER's half-1 sweep, run for hand-written unit
conversions the `factor()` pattern misses because the factor is a
bare literal rather than a `UnitDef` method call.
