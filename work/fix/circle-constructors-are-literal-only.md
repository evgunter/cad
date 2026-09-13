---
id: circle-constructors-are-literal-only
kind: issue
title: Circle loop constructors are literal-only - parametric authors get no door
status: closed
opened: 2026-09-04
refs: [no-parametric-loop-constructor, 948]
branch: fix/circle-parametric-door
closed: 2026-09-11
pr: 2374
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

## Disposition: 2, and the doc owes one more sentence than the row asks for (FIX orchestrator, 2026-09-11)

**Taking disposition 2** — add no constructors, say at
`LoopProgram::circle` that the struct literal is the parametric door
and why. The row's reasoning is right and I checked its load-bearing
claim rather than taking it:

`circle` (`crates/editor-core/src/program.rs:1606`) and `circle_split`
(`:1618`) are **pure struct-literal wrappers**. Neither expands:

```rust
pub fn circle(cx: f64, cy: f64, r: f64) -> Result<Self, DimensionError> {
    Ok(LoopProgram::Circle { centre: [len_lit(cx)?, len_lit(cy)?], radius: len_lit(r)? })
}
```

`CircleSplit` is its own variant carrying `n` and `phase`; the split
happens at lowering, not in the constructor. So the polygon precedent
does not transfer — `polygon` expanded into five steps, which is what
made an unreachable builder mint a second expansion and then a third.
There is no expansion here to hide and nothing that can drift. Adding
`circle_expr` would put a third spelling of the circle door in the
tree to save a `lit()` per argument.

**What the row missed, and what the doc actually owes.** The
constructors are not pure sugar. Their whole body is `len_lit(..)?`
and `Expr::literal(phase, Dimension::Angle)?` — they are the
**dimension-checking** door, and they return `DimensionError`. An
author who writes the struct literal instead is not merely choosing a
different spelling; they are taking a door with no dimension check at
the point of construction.

So a doc that says only *"the struct literal is the parametric door"*
is incomplete and mildly misleading. It has to say where the
dimensions of a parametric `centre`/`radius`/`phase` get checked
instead. **Establish that before writing the sentence** — if the
answer is "at evaluation", say so and name the site; if the answer is
"nowhere", this row stops being a documentation unit and becomes a
real gap, and it should be re-filed as one rather than papered over
with the sentence it was about to get.

That check is the unit. The sentence is the easy half.

## Closed: checked at the document door, and the doc names it (2026-09-11)

The check the disposition demanded before any sentence was written:
**a parametric author's dimensions are checked, at the document door,
and the outcome is not "nowhere".**

`apply` walks every slot of an entering node and compares the
expression's dimension against the role's — `check_node_slots`
(`crates/editor-core/src/edit.rs:1314`), the comparison at `:1325`,
reached from the `InsertNode` arm at `:1577`, from the
parameter-redeclaration re-walk at `:1305`, and from `set_slot`
(`:2096`) on every slot write. `SlotId::Profile { arg, .. }`'s
dimension is `StepArg::dimension` (`crates/editor-core/src/node.rs:220`):
`CenterX`/`CenterY`/`Radius` Length, `Phase` Angle. A disagreement is
`EditError::SlotDimensionMismatch` and the program never enters the
document.

Executed, not read. The tree already pins the literal case —
`insert_node_checks_program_dimensions`
(`crates/editor-core/tests/switch_slots.rs:311`) hand-builds a
`LoopProgram::Circle` with an Angle radius and asserts that refusal. A
throwaway probe (not committed) extended it to the two shapes that
row does not cover, and both refused as derived: a `Radius` role
holding a *named parameter* declared Angle, and a `CircleSplit`
`phase` holding a Length.

**One correction to the disposition's reasoning**, which does not
change the disposition. The constructors are NOT "the
dimension-checking door". `len_lit`/`ang_lit` are
`Expr::literal(v, dim)`, whose only refusals are
`LiteralCountIsInteger` — unreachable at a fixed Length or Angle — and
`NonFiniteLiteral`. So the only error `circle` can return is
non-finiteness, which is what its `# Errors` section already said.
Dimension-wise the constructor does not CHECK, it PICKS: it assigns
Length and Angle so the slot walk cannot fail. A struct-literal author
picks instead, and the document checks the pick. Both of this door's
guarantees are therefore re-established elsewhere for the literal
author — finiteness by `Expr::literal`, which is the only way to mint
a literal `Expr` at all, and dimension by the slot walk — so the
struct literal gives up nothing.

The doc at `LoopProgram::circle` says both halves: why there is no
`circle_expr` (the variant has no expansion to keep in step, unlike
`polygon`), and where a parametric author's dimensions are checked
instead, with the fence named — the check is the document's, so a
program built and replayed without entering one (`viewer::sketch::preview`,
`crates/viewer/src/sketch.rs:726`) never reaches it, and that builder
assigns its own dimensions exactly as `circle` does.

No residue. The two struct-literal sites the row cites
(`crates/editor-core/tests/corpus/plate_param.rs:66` and
`viewer::sketch::loop_program`) are the documented door, not
workarounds to sweep.
