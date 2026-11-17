---
id: a-named-source-over-a-cut-copy-whose-heir-is-a-piece-is-stranded
kind: issue
title: split leaves a named source over a cut copy reading a copy the remainder deleted where the copy crosses to a piece of the instance's copy
status: open
opened: 2026-11-17
priority: P2
cost: M
design: true
---


## What

Found in the review of PR 7087. `split` (`crates/editor-core/src/refactor.rs`, `split`, `crossing_sources`) defines a named remainder source over a cut copy afresh over the instance's copy, but only where the copy crosses whole: the part's product is that one copy, or the cut `⊔` that is it. Where the copy crosses to a piece (the part has several world copies, so the heir is a pick of the instance's copy), the source would have to read a pick written in place. `DocEdit::DefineVar`'s source lowering refuses that (`lower_source_decl`, `SourceFault::WrittenInPlace`: "only a seat authors" a pick). So the source is left as it was, reading a copy the remainder deletes, and its readers are re-pointed past it (a seat or the product slot to a pick of the instance's copy).

The reviewer's row, three pinned copies `c, a, b`:

```rust
let (doc, _) = step(doc, DocEdit::DeclareVar {
    name: VarName::from_static("s"),
    def: VarDecl::Source(BodyRead::plain(Operand::Node(a))),
});
// read by the product or by nothing:
let (doc, _) = step(doc, product(Operand::Var(doc.var_named("s").unwrap())));
let out = split(&doc, &[a, b], "named-source", &p.opts()).unwrap();
// out.remainder holds `s`, and `s`'s read is a variable the remainder does not hold
```

With one cut world copy (`s = b`, the cut `{b}`) it holds: `p2_split::a_named_source_over_a_cut_copy_reads_the_instances_copy`.

## Owed

A choice, since it touches VR4's "a definition reads a variable by id or name, or a node or its port". Either a source's definition may read a pick of a copy (what inline's and flatten's source re-definitions need too: `host_body` and `in_source` can answer a pick, which `DefineVar` would refuse in the same way), or split refuses a named source it cannot carry. A row: the scene above, round-tripped.
