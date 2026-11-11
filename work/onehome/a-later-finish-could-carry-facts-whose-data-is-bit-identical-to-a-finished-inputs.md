---
id: a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs
kind: issue
title: A later finish re-derives facts whose data is bit-identical to a finished input's: the product's aggregate, distribute::fused and a Boolean's untouched facts could carry them
status: open
opened: 2026-11-11
priority: P3
cost: M
---

A cost and box-scalar follow-up from ONEHOME fork 259 (PR 6820).

## What

A move carries a finished body's certificates and verdict
(`topo::FinishedBody::moved`). A later finish does not: it re-derives
every fact from scratch, including those whose data is bit-identical to
a finished input's. Three sites finish such data:
- the product's aggregate (`crates/editor-core/src/product.rs`, the
  gather's one `T::finish`), whose sources are finished bodies, each
  grafted unchanged;
- `distribute::fused` (`crates/editor-core/src/eval/wire/distribute.rs`);
- a Boolean's result, whose faces and edges away from the cut are its
  operands' untouched.

Over a boxed motion, re-deriving a carried fact is also where readings
straddle (`work/shell/transform-rigid-recertifies-images-enclosed-apart.md`),
so carrying them is a box-scalar gain as well as a saving.

## Gate

Carry a fact only behind a from-scratch agreement pin: on the corpus, a
finish that carries agrees with one that re-derives, finding for
finding.

## Measurement ask

Measure the product of boxed-moved members at `Sym` once the cos 2π fix
lands: how many of its findings are carried facts re-derived, and what
carrying them would save.
