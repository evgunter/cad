---
id: a-stand-on-an-instance-reads-its-part-by-the-names-shape
kind: issue
title: A stand on an instance reads its part by the name's shape
status: open
opened: 2026-10-18
priority: P3
cost: M
parent: transform-retires-into-a-placement
---

`mate::stand` (and `pin_at`, `place_at`) anchor a copy on a face whose geometry reads no variable, decided from the recipe by `Anchors::fixed` (`crates/editor-core/src/mate/anchor.rs`). An instance's faces are `InPart { of }` names, and what `of` reads is the part's recipe, which `Anchors` cannot see: an `Evaluation` keeps a part's product (`eval/parts.rs::PartValue`), not its document.

In D, `fixed` answers an instance's face by the name's shape (`part_start_cap`): the face is fixed when, under the part's `Placed`/`InPart` copies and the `From`/`SplitFragment` carries, it is a construction's start cap. The row is `intent_s3_d_placements::an_instance_stands_on_its_parts_base`. This is right for a part whose world placements stand their copies by constant values (what `stand` writes). It is wrong in two cases:
- a part placement that slides its copy by a variable: the instance cancels that motion;
- a start cap carried through an operation that moves it.

Before D every instance face counted as fixed. Between D's kernel change and this fix, none did, so `stand` took the part's end cap (the slice reports' `msolve9` and `asm_r2b` repros).

**Done when** `fixed` reads the part's own recipe for an `InPart` name, through the resolver the evaluation crossed or a record the part value carries ("which of my product's faces read no variable"), and the name-shape rule goes.

- 2026-10-18: superseded by the placement pivot (fork-log row 162, PR 5312). `stand` becomes an unmated placement whose values are the datum chain or six zeros, and `Anchors`, the in-plane/out-of-plane split and the moved-landing refusal retire with it. This closes with that unit, whichever PR carries it.
