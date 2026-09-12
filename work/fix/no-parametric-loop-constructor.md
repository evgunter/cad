---
id: no-parametric-loop-constructor
kind: issue
title: No parametric loop constructor - LoopProgram::polygon is literals-only
status: closed
opened: 2026-08-23
github: 948
refs: [938]
branch: fix/loop-polygon-expr
pr: 1765
closed: 2026-09-04
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


## Closed

**Landed.** `LoopProgram::polygon_expr(points: impl IntoIterator<Item
= [Expr; 2]>)` in `crates/editor-core/src/program.rs` is the polygon
expansion (`At(p0)`, `LineTo(p1)`, …, `LineTo(Start)`), infallible
because every coordinate arrives as an `Expr`. `LoopProgram::polygon`
lifts each `(f64, f64)` through `len_lit` and delegates.

**Three copies existed, not one, and all three now route through the
builder.** The first revision of this unit unified `impl LoopProgram`
and asserted the tree; a style review falsified that by execution.
The two live copies in shipped `src` were:

- `crates/viewer/src/sketch.rs`, the `Rectangle` arm of
  `loop_program` — character-for-character the same loop, over corners
  that were ALREADY `[Expr; 2]`, so it was `polygon_expr`'s natural
  second consumer;
- `crates/pncad-py/src/py/doc.rs`, `Node.polygon` — the same expansion
  unrolled, with a comment narrating it in prose.

Both fold. `demos/tour/src/assembly.rs` — the measured consumer this
issue was filed from — no longer has a `rect` helper at all: the loop
is authored at its one call site from the document's own named
extents, which is what a user writes fresh against the door. The
helper had been carrying a `zero: &Expr` argument that existed only
because the hand expansion used it four times.

**The stale justification is corrected, precisely.**
`viewer::sketch::loop_program`'s doc declined the builder because
"those constructors take `f64` and mint CANONICAL literals". That is
still true of `LoopProgram::circle` and is now false of the polygon
door, since `polygon_expr` takes `Expr` corners and mints nothing.
Only the false half was rewritten.

**What the pin actually pins.** The agreement rows show the two doors
AGREE, which catches DRIFT between two expansions. They cannot catch
the EXISTENCE of one: any correct duplicate satisfies
`polygon(c) == polygon_expr(lift(c))` by definition. Measured, not
assumed — the reviewer planted the pre-fold body back inside `polygon`
and the agreement rows went green, and a second plant closing the loop
only at arity >= 2 also went green. So
`crates/editor-core/tests/fix_loop_polygon_expr.rs` now carries four
rows: agreement at four corners, agreement at BOTH degenerate arities
(zero and one corner, where the builder is deliberately total because
the edit door refuses degeneracy typed at `insert`), the parametric
shape, and a source census asserting the polygon close is pushed in
exactly one place in `crates/*/src`. The census is the only row that
detects a second expansion arriving, and it was verified to red on the
reviewer's own plant while the three agreement rows still passed.

**The census red this unit caused, and its repair.** `geom-core`'s
`bounds_census` walks the tree's source and panicked on the new
signature: its `angle_end` closed a generic list at the first `{` or
`;` regardless of bracket nesting, so the `;` inside
`<Item = [Expr; 2]>` read as the item's body. `[Expr; 2]` was kept —
distorting the door to suit a census parser would be the wrong repair.
The reading now lives in the SHARED home,
`test_utils::source::angle_end`, beside `balanced_end` and
`top_level_split`, whose own docs state the rule that a copy of a
lexer's postcondition at a call site is how this tree grew its
readers; `bounds_census` calls it and keeps its fail-loud panic.
`top_level_params` stays a private copy and now says why at the site
(the shared split clamps depth where it does not, so swapping it is a
behaviour change to what the census reads, and its own unit).
`angle_end` also no longer treats an arrow's `>` as a closer, and its
half-true disclosure is corrected: refusing to close is loud and
panics, but closing at the WRONG `>` answers a too-short list
silently, which is the undercount direction the file claims to refuse.
Direct rows for all of it live in `test-utils`' own `mod tests`, so a
revert of the repair reds on its own evidence rather than incidentally
through whichever door happens to be spelled with an array that day.

**The class is DEFERRED, not closed** — the first revision claimed
closed and the claim did not survive review. The reasoning that
`circle`/`circle_split` cannot drift is sound about DRIFT and does not
discharge what this issue filed, which is an AUTHORING gap: a
parametric author cannot use those builders either. That is severed
into `work/fix/circle-constructors-are-literal-only.md` rather than
disclosed in prose here, per `work/README.md`. The scope claim is also
restated: the sweep's scope was `impl LoopProgram`, NOT "the authoring
layer" — `viewer::sketch::loop_program` is itself a convenience door
with an expansion behind it, in the layer `CLAUDE.md` calls a thin
client over the API, and the original sentence read as universal over
ground the sweep never covered.

**Swept for.** Pattern 1 (hand-rolled polygon expansions: a
`ProgramStep::At(` read forward to a `LineTo(ProgramTarget::Start)`
close) found 18 hits across the tree; one was the demo consumer,
taken. Five carry non-literal corners and are test fixtures that spell
their `Chain` out as direct coverage of the document vocabulary
(`m10_4_stackup_interval.rs:1264`, `m10_4_seed.rs:173`,
`m10_4_r2_probes_interval.rs:230` and `:409`, `switch_naming.rs:47`) —
not taken, because routing them through the builder would test the
builder instead. The rest are literal; `plate_param.rs:79` says at the
site that it is hand-built on purpose. Pattern 2 read
`ProgramTarget::Start` across `crates/*/src` and is what found the two
live copies. Both patterns were re-run on each merged tree as
`origin/main` moved.

**What the sweep could NOT match.** Pattern 1 reads at most 13 lines
forward from an `At(`, over lines that are themselves `LineTo` steps.
**That blind spot is where both live copies were**: steps pushed onto
a `Vec` in a loop, which is exactly the shape the sentence named. It
is also blind to a chain closing through `ContinueTo`, and to macros.
The Python surface was named as the likely miss and was the wrong
guess twice over — the bigger miss was Rust, and `pncad-py` IS Rust,
so the pattern could have read it and did not. Pattern 2 matched one
literal spelling of a push; an expansion that appends by `extend`, by
collecting an iterator, or by building the `Vec` literally is still
invisible, and the census row inherits exactly that limit and says so.
The census-repair sweep matched one spelling of the break rule and one
helper name; a scanner holding its terminator in a variable would not
appear.
