---
id: signed-zero-module-hand-counts-the-bitwise-readers-it-exists-for
kind: issue
title: step-import — signed_zero.rs says the importer reads bit patterns twice; it is a hand-maintained count and there are more
status: closed
opened: 2026-09-15
priority: P4
cost: E
branch: export/walks-and-zeros
pr: 7067
closed: 2026-11-17
---


## Finding

Found in the S415 style review (2026-09-15), which changed one of the
readers.

`crates/step-import/src/signed_zero.rs` exists so that the argument
for flushing `-0.0` is stated **once**. Its header then makes that
argument rest on a hand-maintained count:

> this importer reads bit patterns twice:

and names two — `adopt::surface_sig`'s bitwise record comparison, and
the writer round-trip's printed sign. **There are more, and the module
has never counted them:**

- `crates/step-import/src/adopt.rs` — a second bitwise comparison
  beside `surface_sig`, over point and knot bit arrays.
- `crates/step-import/src/entities.rs` — three `prev.to_bits() ==
  value.to_bits()` guards on repeated unit/conversion factors.
- `crates/step-import/src/assemble.rs` — `plant`'s scaffold-anchor
  scan WAS a bitwise reader; S415 (#2633) made it value equality, so
  the number moved again without the module noticing either way.

So a module whose whole purpose is to hold one argument in one place
holds, inside it, a count of the sites that argument covers, which
nothing derives and nothing checks. Every site added or changed since
it was written has left it saying the old number, and a reader
weighing whether a new mint owes the flush is weighing it against a
list that is not the list.

## Not a request to delete the argument

The argument is good and belongs here. What is unsupported is the
ENUMERATION: either derive the readers (they are `to_bits` uses in
this crate, greppable and few) or state the property without counting
— "this importer compares floats bitwise in several places, and every
one of them is a reason" is true and stays true.

The same shape is worth checking on the `Where it is called, and where
it is not` paragraph below it, which is a second hand-maintained
inventory in the same header.

## Where to look

- `crates/step-import/src/signed_zero.rs` — the header's two
  enumerations.
- `crates/step-import/src/adopt.rs`, `entities.rs`, `assemble.rs` —
  the readers, found by grepping `to_bits()` in the crate.

## Closed (2026-11-17)

PR 7067 (`export/walks-and-zeros`). It had a single style review and a fix pass. The signed-zero fold has one home, `geom_core::fold_signed_zero`, and the importer's half-edge walks go through `topo::Body::face_of_half_edge` or the local `body_loop`/`loop_face` helpers. The PR body lists each site and what was done.
