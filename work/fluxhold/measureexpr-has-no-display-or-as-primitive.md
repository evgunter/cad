---
id: measureexpr-has-no-display-or-as-primitive
kind: issue
title: MeasureExpr has no Display and no as_primitive, so a reader outside the crate cannot say what a measure measures
status: closed
opened: 2026-09-30
priority: P1
cost: M
closed: 2026-10-10
---


Found by AUTH-7 (PR 3528) and its review. The feature tree now shows a
measure's value (`Measure »  0.0125 m`). The parent row's example,
`Measure  distance  12.500 mm`, also names what was measured, and the
viewer cannot produce that word honestly:

* `MeasurePrimitive::verb()` is public, but
  `MeasureExpr::kind()` is `pub(crate)`
  (`crates/editor-core/src/measure.rs`, `MeasureKind`). A reader
  outside the crate cannot tell a bare `distance` apart from
  `distance - 5 mm`. `primitives()` lists the leaves and says nothing
  about the arithmetic above them.
* `MeasureExpr` has no `Display`, so a composed measure has no kernel
  spelling at all. A viewer that wrote one would be minting a second
  unparser.

Either door closes it: an `as_primitive() -> Option<MeasurePrimitive>`,
which lets the row name the verb only when it is the whole expression,
or a `Display` that `unparse` shares. The consumer is the tree row in
`crates/viewer/src/tree.rs` (`Measured`), and the viewer half is one
line once the door exists.

## Re-homed from FLUX to FLUXHOLD (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXHOLD holds FLUX's rows on the D10 hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Each waits on the INTENT unit that rebuilds its ground, named in `blocked_on`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Closed (2026-10-10): superseded by INTENT stage 2 D (PR 4355)

(FLUX orchestrator) Checked at FLUX's cut: `MeasureExpr` no longer exists
anywhere in `crates/` (`rg MeasureExpr crates/` is empty). Stage 2 D,
`measure-is-an-operation`, made one `Measure` one primitive defining one
observed scalar, and moved its arithmetic into a `Defined` variable. So
a measure's primitive is the node itself, and the composite this row
asked to render is gone. If a reader still cannot say what a `Measure`
measures, that is a new finding against the new type.
