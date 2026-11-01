---
id: dual-tangent-is-unpinned-for-seven-recipe-verbs
kind: issue
title: Seven recipe verbs evaluate at Dual64 with no row reading their tangent
status: open
opened: 2026-10-02
---


## Finding

`editor-core/tests/m10_di_dual_corpus.rs:98`
(`every_document_evaluates_at_dual64_with_the_f64_value_channel`)
evaluates every corpus document at `Dual64` and compares the VALUE
channel with the `f64` run. A verb whose build drops its tangent (a
size read through its value into an `f64` and lifted back as a
constant) keeps that row green: the value is right and the derivative
is zero. BAND's `S90-impl` pinned the blend's tangent
(`sweep/tests/blend_dual_tangent.rs`,
`editor-core/tests/blend_dual_sensitivity.rs`) and showed that mutant
red there and green on the corpus row.

The same gap stands for the verbs no tangent-reading row names. The
sweep: every `editor-core/tests/*.rs` that reads a tangent
(`Dual::variable`, `sensitivities(`, `stackup(`, `.deriv`), grepped for
each `Node::<Verb>` spelling. No such file names `Sweep`/`Swept`,
`shell`, `chamfer`, `placed_union`/`PlacedUnion`, `Pattern`, `Tube` or
`HollowTube` (`Part` likewise, though it composes other verbs). `chamfer`'s kernel
door is pinned by `blend_dual_tangent.rs`; its recipe node is not. The
pattern cannot see a verb reached through a corpus document loaded by
name rather than spelled `Node::` in the reading file, nor a kernel
row in another crate that reads the verb's tangent directly; both
gaps make this list an upper bound.

## What

Per verb on the list, a row that seeds a size the verb consumes and
asserts the tangent of a quantity it moves against central
differences of the `f64` build, with a mutant shown red.
