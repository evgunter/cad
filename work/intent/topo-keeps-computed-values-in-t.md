---
id: topo-keeps-computed-values-in-t
kind: unit
title: from_f64 audit, topo: every listed launders site keeps its value in T or enters it through from_computed
status: parked
opened: 2026-10-10
priority: P0
cost: M
blocked_on: [a-computed-value-re-enters-as-a-constant]
refs: [carriers-compare-in-canonical-form]
---


Unit 2–5 of the `from_f64` audit (`a-computed-value-re-enters-as-a-constant`
is unit 1, which landed the gate, the census and the re-valuation row).

Every `launders(topo-keeps-computed-values-in-t)` entry in
`scripts/gates/from-f64-sites.tsv` is a value this crate computed and
re-enters through `Real::from_f64`, so at `Sym` it is a constant where
the computation had a function of the variables. For each, either keep
the value in `T` (the site read it out of `T` and back), or, where a
numerical routine produced it in `f64` (a projection foot, a root, a
Newton iterate, a certified hull bound, f64 box arithmetic), enter it
through `Real::from_computed`, one unknown per call. Each fix deletes
its entry; the gate's STALE rule holds the list to the tree.

Pins: `crates/editor-core/tests/revalue_corpus.rs` lists the corpus
decisions it sees laundered and the row each is owed to; a fix here
that clears one moves that pin, which is the evidence the fix landed.
A move to `from_computed` can only lose symbolic `Zero`s, never
geometry: say which `symbolic_zero` counts moved.

**A candidate the census found.** With the tier off, `REVALUE_DOC=die` in release, reading the decisions before the two builds part (`revalue_corpus.rs`'s `revalue_census`, `parted`), the last decision compared is `volume_backstop` (`boolean/ops.rs`). It re-values to −3.1e-17, where `p0` decided 0 and `p1` decided −2.3e-16. Its margin is built from listed sites of this unit: `props.rs`'s `ac`, `fc` and `self.volume_pad`, and `boolean/ops.rs`'s `margin.lo()`. The loop it sits in refines a data-dependent number of rounds with no decision between them, so the prefix reading cannot attribute it on its own. Fixing those sites should make it re-value to the `p1` bits, or show that the difference is the round count.
