---
id: no-parametric-loop-constructor
kind: issue
title: No parametric loop constructor - LoopProgram::polygon is literals-only
status: review
opened: 2026-08-23
github: 948
refs: [938]
branch: fix/loop-polygon-expr
pr: 1765
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
= [Expr; 2]>)` in `crates/editor-core/src/program.rs` is now the ONE
polygon expansion (`At(p0)`, `LineTo(p1)`, …, `LineTo(Start)`), and it
is infallible — every coordinate arrives as an `Expr`, so there is no
literal left to refuse. `LoopProgram::polygon` is that door at literal
corners: it lifts each `(f64, f64)` through `len_lit` and delegates,
so the two spellings cannot drift.

`demos/tour/src/assembly.rs::rect` — the measured parametric consumer,
and the one this issue was filed from — now calls `polygon_expr` and
its five hand-written `ProgramStep`s are gone, along with the GAP
comment that named this issue. `ProgramStep` and `ProgramTarget` left
the demo's import list with them.

`crates/editor-core/tests/fix_loop_polygon_expr.rs` pins both halves:
the literal door and the expression door produce the identical
program at the same corners (which is what makes the delegation
load-bearing rather than incidental), and the expansion keeps its
`At` … `LineTo(Start)` shape at a corner no literal door can take (a
document parameter reference).

**One red the unit caused, and fixed.** `geom-core`'s
`bounds_census::every_sole_bracket_bound_door_is_in_the_roster` walks
the tree's source and panicked on the new signature. Its `angle_end`
helper closed a generic parameter list at the first `{` or `;` with no
regard for bracket nesting, so the `;` inside `<Item = [Expr; 2]>` —
an ordinary fixed-size array, and exactly the type `ProgramStep::At`
holds — read as the item's body and stopped the census. The scanner
now reads that terminator at square/round-bracket depth zero only,
which is the nesting its own sibling `top_level_params` already
respects for commas. `[Expr; 2]` was kept: distorting the door's type
to suit a census parser would be the wrong repair. Swept for the same
shape (`'{' | ';' => break` in a bracket scan, and `angle_end`-like
helpers): one instance, this one. The sibling scanner
`flagged_census::skip_turbofish` counts angle depth alone and has no
`{`/`;` break, so it never had the defect.

**Swept for.** Two patterns, and pattern 1 re-run on the merged tree
after `origin/main` moved: the seventeen untaken hits are unchanged,
main added no new hand-rolled polygon, and it touched neither
`program.rs` nor `assembly.rs`.

1. *The shape, not the symbol*: every `ProgramStep::At(` in the tree,
   read forward for a run of `LineTo` closed by
   `LineTo(ProgramTarget::Start)` — a hand-rolled polygon expansion.
   18 hits. One is the demo consumer, taken. Five more carry NON-
   literal corners, i.e. the same defect in test fixtures:
   `m10_4_stackup_interval.rs:1264`, `m10_4_seed.rs:173`,
   `m10_4_r2_probes_interval.rs:230` and `:409`, `switch_naming.rs:47`.
   They are NOT taken: a fixture that spells its `Chain` out is direct
   coverage of the document vocabulary, and routing it through the
   builder would test the builder instead. The remaining twelve are
   literal and could already have used `polygon`; `plate_param.rs:79`
   says at the site that it is hand-built on purpose, beside the
   carrier forms.
2. *The literal/Expr split at the convenience layer*, which the issue
   body calls the real class. The layer is four constructors:
   `polygon` (fixed here), `from_recorded` (literal BY CONSTRUCTION —
   `profile::Step<f64>` has no expressions and by G1 layering must not
   gain them, so this is a seam, not a gap), and `circle` /
   `circle_split`. The latter two are literal-only and stay that way:
   each is a 1:1 fill of a public struct variant with no expansion
   behind it, so the parametric spelling is the struct literal itself
   (`plate_param::hole_loop` writes it in three lines) and there is no
   second expansion that could drift. That asymmetry is a naming
   inconsistency, not an instance of this defect. Nothing else in the
   authoring layer takes literal coordinates: `Item = (f64, f64)` has
   no other hit under `crates/`, and `Frame` is `f64`-valued by design
   rather than `Expr`-bearing.

**What the sweep could NOT match.** Pattern 1 reads forward at most 13
lines from an `At(`, only over lines that are themselves `LineTo`
steps, so a polygon whose steps are built in a loop, pushed onto a
`Vec` one at a time, split across a helper, or interleaved with
comments or `#[cfg]` lines is invisible to it — and so is any closed
chain that reaches `Start` through `ContinueTo` rather than `LineTo`.
It is also blind to the Python surface entirely: it greps Rust only,
and `Node.polygon` in the bindings was never read. Pattern 2 matched
the exact type spelling `Item = (f64, f64)`, so a convenience door
taking `&[(f64, f64)]`, `Point2<f64>`, or two scalars positionally
would not appear — `circle`/`circle_split` were found by reading
`impl LoopProgram` rather than by that grep, which is the measure of
how narrow the grep is. Neither pattern says anything about doors that
do not exist yet.
