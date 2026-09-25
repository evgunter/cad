---
id: two-viewer-docs-cite-a-drag-clause-for-a-typed-value
kind: issue
title: widgets.rs and props.rs cite a ratification for a band and a typed write it does not cover
status: open
opened: 2026-09-24
priority: P3
cost: E
---

Found by VNEWS's census,
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
(§The census, population 1), at merge base `f4b178189`. Both members
on VGEOM's ground are PARTIAL.

- `crates/viewer/src/widgets.rs` `number_text` (:292): *"For a
  continuous quantity that band is the ratified render accuracy"*. ε
  is ratified (`docs/DESIGN.md` D4 ¶1, `9f0bb4d91`). The band is not:
  it is `readout::tolerance`, a cap one decade below ε met with a
  relative arm. `crates/viewer/src/readout.rs` itself calls it
  *"legibility choices"* and *"a display choice stated once against
  the ratified default"*.
- `crates/viewer/src/props.rs` module docs (:77): setting a number into
  a driven slot is refused, *"the ratified micro-decision"*.
  `crates/viewer/GUI-DESIGN.md` G4 as Ev wrote it (`5267a9193`) is
  *"Dragging an expression-driven dimension → refuse, with an
  affordance"*. That covers a drag. `session::guard_driven` also refuses
  a typed `set_slot`, which the clause does not reach.

Both are one-sentence repairs: say what is ratified and what is this
crate's own choice. Neither changes a value.
