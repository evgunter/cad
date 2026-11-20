---
id: a-refusal-is-told-in-the-readers-notation
kind: unit
title: A told quantity renders in the reader's notation; the viewer's readout grid retires onto it
status: dispatched
opened: 2026-11-20
priority: P1
cost: H
parent: patherror-display-renders-float-noise
branch: scalar/a-refusal-is-told-in-the-readers-notation
---

Unit 2 of `patherror-display-renders-float-noise`, the build of the
last paragraph of `crates/geom-core/README.md`, "A number told to a
person": **a refusal is said in the notation of whoever reads it.**

**Letter M, tier dual.** The door a told quantity renders through is
what every per-crate sweep and every GUI surface will call, so its
shape is hard to change later; the arithmetic is unit 1's and is
already reviewed, which is why the letter is M and not H.

## What is built

- `Told` (and `Compared`, and a position) render given a notation;
  `Display` is that rendering in metres and radians and its text does
  not change. The grid, the sidedness of an offer and the order of a
  compared pair hold **in the notation rendered**, not in metres then
  converted: an offered bound typed back in millimetres is on the side
  that works, a given number reads exactly what was received in the
  unit it is read in, and ε/10 is a length whatever the unit.
- The tolerance is set in metres, and a recourse that names it says it
  in metres, in every notation.
- The notation type has one home that `geom-core`'s rendering and the
  viewer both use. Today it is `viewer::props::Notation` over
  `quantity`'s units; `quantity` is a zero-dependency leaf and
  `geom-core` does not depend on it. Where the type lives and what
  `geom-core` sees of it is this unit's choice; say what was chosen
  and what it cost.
- The viewer passes its working notation wherever it shows a kernel
  refusal, and `viewer::readout::number`'s second grid
  (`crates/viewer/src/readout.rs`, `tolerance`, marked there as the
  deletable spelling) retires onto the told quantity. That file names
  the two places the two differ on purpose, the chosen notation and
  the character bound; the bound is the one a merge cannot drop, so
  say how it is kept.
- A whole refusal renders in a notation, not only one number: a
  `profile` refusal's `Display` composes told quantities, and the
  viewer needs the sentence. How a sentence is asked for in a notation
  without each refusal type writing its text twice is the design
  centre of the unit.

## Measure first, and report

- Who shows a kernel refusal to a person, by surface (viewer panes,
  the editor's messages, the Python door, the CLI), and which of them
  has a notation to pass.
- For `told-is-boxed-per-number-where-boxing-the-large-variants-would-keep-copy`:
  how many error types in the crates the sweeps will reach would cross
  `clippy::result_large_err`'s line holding an inline `Told`. Report
  the count; do not change the boxing.

## Not this unit

Positions held as `Point2<T>`
(`a-refusals-corner-point-is-read-as-a-number-by-the-viewer-and-python`),
the per-crate sweeps (unit 3 on), and a fourth dimension
(`three-dimensions-do-not-cover-the-sentences-already-in-the-tree`).

