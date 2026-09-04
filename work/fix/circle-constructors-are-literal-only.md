---
id: circle-constructors-are-literal-only
kind: issue
title: Circle loop constructors are literal-only - parametric authors get no door
status: open
opened: 2026-09-04
refs: [no-parametric-loop-constructor, 948]
---


## The gap

`LoopProgram::circle(cx, cy, r)` and
`LoopProgram::circle_split(cx, cy, r, n, phase)`
(`crates/editor-core/src/program.rs:1555`, `:1571`) take `f64` and mint
canonical literals. A document whose radius is a named parameter — the
ordinary parametric part — cannot use either. It writes the struct
variant out:

```rust
LoopProgram::Circle {
    centre: [lit(centre.0), lit(centre.1)],
    radius: hole_radius(),
}
```

`crates/editor-core/tests/corpus/plate_param.rs:66` is exactly that, and
`crates/viewer/src/sketch.rs`'s `loop_program` builds its `Circle` arm
the same way for the same reason, saying so at the site.

This is the SAME asymmetry #948 named — *a parametric author cannot use
the builder* — at the circle doors rather than the polygon one. It was
found while closing `no-parametric-loop-constructor` and is filed
separately because that unit's evidence does not cover it.

## What this is NOT

**Not a drift hazard**, and this is the difference from the polygon
case. `polygon` expanded into five steps, so a parametric author who
could not reach it wrote a SECOND EXPANSION, and the tree ended up with
three. `Circle` is a struct variant with two public fields: the
parametric spelling is a struct literal, there is no expansion behind
the constructor, and no copy of anything can drift. Nothing is
currently wrong in the tree because of this.

So the fix is NOT obviously "add `circle_expr` / `circle_split_expr`".
Four lines of sugar over a struct literal that already reads fine may
be worth less than the vocabulary it adds — that judgement is the unit,
not a foregone conclusion. What is real is the user-facing
inconsistency: a literal circle gets a named constructor, a parametric
circle gets a struct literal, and the next parametric author meets that
step with no doc telling them it is deliberate.

## Dispositions worth weighing

1. Add the `_expr` constructors, for symmetry with `polygon_expr`.
2. Add nothing and SAY so at `LoopProgram::circle`: the struct literal
   is the parametric door, by design, because the variant has no
   expansion to hide. Cheapest, and it closes the "is this an
   oversight?" question a reader currently has to re-derive.
3. Reconsider the convenience layer as a whole once a second
   `Expr`-bearing consumer exists — the polygon door has one consumer
   inside the kernel today (`viewer::sketch::loop_program`) and one
   binding.

## Home

`crates/editor-core/src/program.rs`, which `territory --base main`
reports as **docm**'s. Filed on FIX's slate because FIX owns the
polygon unit this was severed from; re-home by editing the header.
