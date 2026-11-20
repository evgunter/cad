---
id: patherror-display-renders-float-noise
kind: issue
title: A refusal's payload holds a told quantity: build the type, retire the three number rules onto it, and sweep the Display arms
status: open
opened: 2026-08-30
refs: [1267, 7154]
priority: P1
cost: H
---

Restated 2026-11-20 on Ev's ruling (PR 7154: "the rules are fine").
The row's history, from GitHub issue 1282 through DOOR, PROPS, FIX and
FLUX, is in git (`git log --follow` on this file); none of it binds.

## The rule

`crates/geom-core/README.md`, "A number told to a person": a number a
sentence quotes is one opaque told quantity that the refusal's payload
field holds, spelled by its role (given, computed, compared, offered),
in the notation of whoever reads it.

## What the tree holds today

- `profile::path::num` / `num_toward` (`crates/profile/src/path.rs`):
  the grid, and the one-sided rounding of an offered bound. All of
  `crates/profile`'s refusal sentences.
- `viewer::readout::number` (`crates/viewer/src/readout.rs`): a second
  grid with a character bound.
- `geom_core::Readable` (`crates/geom-core/src/readable.rs`): exact
  round-trip, and `MarginDiag::readable` through it. It stays for text
  that must be exact (field text, `Stackup::serialize`, dumps) and
  stops being the sentence rule.
- 150 to 200 error `Display` arms that render a scalar with `{:e}`,
  bare `{}` or `{:?}`, largest in geom-brep, topo and editor-core (an
  explorer's scan, 2026-11-19; re-measure).

## Shape

1. The type and its renderings in `geom-core`, `profile`'s refusals
   moved onto it, `num`/`num_toward` deleted, `MarginDiag`'s sentence
   rendering made the type's, and the gate (no bare `f64` or `T` field
   in a refusal type; no `{:?}` or `{:e}` in a refusal's `Display`).
2. The notation-taking rendering: `Display` in metres and radians, the
   GUI passing the working notation; the notation type moves to
   `quantity`.
3. One sweep per crate over the remaining arms.

Each is a unit; the first is the hard one.

## Read before building

`work/scalar/log.md`, "Fork: a number told to a person" (2026-11-19):
what nobody checked (who reads a refusal payload's scalar field; whether
`PathError<T>` keeps `T`), the signed-zero disagreement pinned both
ways, the content key that hashes a `Readable` rendering, and the rows
this one absorbs.
