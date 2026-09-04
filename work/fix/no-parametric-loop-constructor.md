---
id: no-parametric-loop-constructor
kind: issue
title: No parametric loop constructor - LoopProgram::polygon is literals-only
status: open
opened: 2026-08-23
github: 948
refs: [938]
---

## From GitHub issue 948

Opened 2026-08-23; 0 comments.

Small authoring gap, met by the ASM-DEMO exit walk (#938).

`LoopProgram::polygon` takes `impl IntoIterator<Item = (f64, f64)>` — literal coordinates. Its own doc comment names the gap:

> A literal polygon … (corpus/fixture authoring; **parametric authors write the steps with their own Exprs**).

So a document whose rectangle is driven by named parameters — which is the ordinary parametric part, and what both of the demo's part documents are — cannot use the builder. It writes the five steps by hand:

```rust
LoopProgram::Chain(vec[
    ProgramStep::At([zero.clone(), zero.clone()]),
    ProgramStep::LineTo(ProgramTarget::Point([w.clone(), zero.clone()])),
    ProgramStep::LineTo(ProgramTarget::Point([w.clone(), h.clone()])),
    ProgramStep::LineTo(ProgramTarget::Point([zero.clone(), h.clone()])),
    ProgramStep::LineTo(ProgramTarget::Start),
])
```

`demos/tour/src/assembly.rs::rect` is that, and every parametric consumer will write it again. The natural door is the same constructor over `Expr` — `LoopProgram::polygon_expr(points: impl IntoIterator<Item = [Expr; 2]>)` — with the literal one delegating to it through the literal Expr constructor, so there is one expansion rather than two that can drift.

Note the shape is not rectangle-specific: the gap is the whole chain vocabulary's literal/Expr split at the convenience layer, and a rectangle is just where it shows up first.

— Claude (ASM-DEMO lane)

## Home

`LoopProgram`'s constructors live in `crates/editor-core/src/program.rs`, which sits in no open program's territory (LIB's paths are the `pncad` façade and bindings), so it lands unowned under `work/issues/`.
