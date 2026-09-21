---
id: seats-module-header-does-not-name-the-line-it-composes
kind: issue
title: seats' module header lists what the module is the one place for and leaves out seat_line
status: open
opened: 2026-09-20
priority: P3
cost: E
---



Filed from the style review of
`seat-line-spells-the-list-mark-as-a-literal` (#2917).

## The claim and what it leaves out

`crates/viewer/src/seats.rs`'s module header opens with a census of
what the module is for: *"the state every modal tool that consumes
node picks is built out of, and the one place their **pick rule,
survival step and refusal sentence** live."* Three things, named.

**`seat_line` is a fourth and is not in the list.** It is a public
door (`crates/viewer/src/lib.rs` re-exports it), it is what five
panels in `pane::create` actually render, and its own doc comment
makes an argument about why it lives here — *"Composed here rather
than in the widgets because it is the same vocabulary a lost-pick
notice is composed from, and two copies is how the two drift"* — which
is precisely a claim of the form the header's sentence is making, one
paragraph out of reach of the header that should carry it.

## Why it is worth a row

A header that enumerates is a population, and this program's register
treats a population as evidence only as of its filing. A reader
deciding where a new panel line belongs reads the header, finds three
things, and does not find the line — so the argument for composing it
here is invisible at exactly the moment it is needed. That is how the
mate panel came to hand-roll its own
(`mate-panel-hand-rolls-the-seat-line`), and it is a cheaper fix than
that one.

The fix is one clause in the header naming the line, or the header's
sentence re-stated so it does not enumerate. Either way the population
is re-derived from the module's public surface rather than repaired by
addition.

## Home

VNEWS's: `crates/viewer/src/seats.rs`.
