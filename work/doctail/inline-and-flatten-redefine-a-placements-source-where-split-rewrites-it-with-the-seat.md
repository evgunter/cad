---
id: inline-and-flatten-redefine-a-placements-source-where-split-rewrites-it-with-the-seat
kind: issue
title: inline and flatten redefine a node seat's anonymous source (a placement's, or a kept ⊔'s member) with DefineVar, which refuses a source over a pick of the copy as a kernel defect; split and the product slot rewrite it with the seat
status: open
opened: 2026-11-17
priority: P2
cost: M
---


## What

Found in the delta review of PR 7087. `inline` and `flatten` (`crates/editor-core/src/refactor.rs`) re-point a node seat's anonymous source by redefining it in place: the `ReadSite::Definition` arm emits `DocEdit::DefineVar { def: VarDecl::Source(written_body(body, host_body(read))) }`, with `in_source` in flatten. Where the new read is a pick, because the source read a piece of the instance's copy, the declare door refuses it (`lower_source_decl`, `SourceFault::WrittenInPlace`: "only a seat writes" a pick in place). Inline then reports that refusal as a kernel defect.

The product slot's own anonymous source (`own_source` / `reread`) and split's crossing seats take the other rule: rewrite the source with the seat, so the seat's door mints a fresh source over a fresh pick. Two rules for one thing.

The scene below was run on PR 7087's branch and is not committed:

```rust
let (sub, first) = two_pinned(&p, "zz-place-sub");          // two bases, product their ⊔
let sub = crate::fixture::assemble_world_copies(sub);
let piece = sub.output(first, 0).unwrap();
// host: an instance `h` of `sub` at the identity, and a Place of a piece of its copy
let read = Operand::Source(Box::new(BodyRead::plain(
    Operand::pick(Operand::Var(copies), vec![piece]))));
let (doc, placed) = insert(doc, fixture::stood(BodyRead::plain(read)));
editor_core::inline(&doc, h, &r, tol)
// Err: "inline: an edit refused: #28:… is a source whose read is written in
//  place, which only a seat writes. There is no way through: this is a
//  kernel defect; report it"
```

## Why this was not fixed in PR 7087

Rewriting a placement's source with its seat (`SetParam` on `OperandSlot::Source`) leaves the placement's own `on`-side selections reading the old source by id. Those selections are exempt from the reader count only while they select the placement's own source (`Doc::own_source_selections`). Re-pointed alone, they would keep the old source alive, now as a body no placement copies, and they would read the instance's copy that inline deletes. They have to be re-authored over the new source in the same action, with their names landed (`to_host` / `shape_of`).

The other way out is a source definition that may read a pick. That is the same VR4 fork as `a-named-source-over-a-cut-copy-whose-heir-is-a-piece-is-stranded`, so decide the two together.

## Owed

One rule for an anonymous source across split, inline, flatten and the product slot, and the scene above as a row: inlined and flattened, it delivers the piece (an `extent` equality).

## Scope (the third delta review of PR 7087)

A kept `⊔` whose member seat reads a source of a pick hits this too. The review's scenes `d3-n1-*-union` and `d3-n1-*-place` show it: split a document whose kept `⊔` or kept placement reads `Source(pick(pick(copy, [g1]), [a]))`, and inline of the remainder's instance refuses with the same "source whose read is written in place … kernel defect". The `⊔` case has no `on`-side selections, so a rewrite with the seat would serve it alone. The placement case owes the re-authoring described above.
