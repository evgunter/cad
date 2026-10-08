---
id: a-flat-chain-of-more-than-128-terms-refuses
kind: issue
title: editor-core: a flat chain of more than 128 terms refuses, because an operator nests one level per term
status: open
priority: P4
cost: H
design: true
opened: 2026-09-30
refs: [3510]
---

(EDIT, filed by the fix pass on PR 3510.)

## What

An expression nests at most `expr::MAX_NESTING` = 128 levels, and
`Add`, `Sub`, `Mul` and `Div` are binary nodes that the parser and
every caller chain to the left, so `a + b + … ` of 129 terms nests 129
levels and refuses `DimensionError::NestedTooDeep`, at the text door
(`parse::parse_expr`), at the smart constructors and at the load door.
The same terms grouped (`(a + b) + (c + d)`) nest less: a balanced sum
of 10⁵ terms nests 18 levels. The docs of `parse_expr`, `Expr`,
`MeasureExpr`, `Doc.parse_expr` and both Python classes say so.

Nothing in ordinary use meets it today: the deepest expression in the
committed documents nests 6 levels, Python builds expressions only
from text, and on `main` before PR 3510 a flat sum of about 62 terms
already saved into a file that would not load (serde_json's recursion
limit counts two JSON levels per operator). A caller that joins terms
by machine (a generated sum over a pattern's instances, say) is the
one that meets it.

## Why the bound is there

The save, load and content-pin doors are serde_json's own recursion
over the expression wire (`persist::wire::WireExpr`), as are the
derived `Clone`, `PartialEq` and `Debug`, the content key and
`unparse`. Each costs the thread's stack one frame or more per level,
and the smallest stack a door runs on is the wasm32 build's 1 MiB. The
bound is what keeps those walks inside it (PR 3510's measurements).

## The two routes that would lift it

1. **An n-ary `Add`/`Mul`** (`design: true`): a sum or product of any
   number of terms as one node, so a flat chain nests one level
   however long it is. It changes the ratified F7 AST and the
   expression wire, and every walk and `ExprPath` over them, so it is
   a fork for Ev (designers first).
2. **An iterative writer and reader for the expression subtree**: a
   hand-written `Serialize` and `Deserialize` for the expression wire
   that keeps its own stack, with the same bytes on the wire, so no
   wire change; the derived `Clone`, `PartialEq`, `Debug`, the content
   key and `unparse` would each need the same treatment, and the load
   door's scan (`persist::nesting::BODY_NESTING`) would need a new
   argument for how deep a body may nest. Large, and no design
   question, but it moves every recursive walk the bound now covers.

Either would let `MAX_NESTING` grow or go; neither is needed until a
real document meets 128.
