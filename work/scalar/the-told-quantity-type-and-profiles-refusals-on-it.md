---
id: the-told-quantity-type-and-profiles-refusals-on-it
kind: unit
title: The told quantity in geom-core, profile's refusals moved onto it, and the gate over refusal fields
status: closed
opened: 2026-11-20
priority: P1
cost: H
parent: patherror-display-renders-float-noise
pr: 7159
branch: scalar/told-quantity-type
closed: 2026-11-20
---


Unit 1 of `patherror-display-renders-float-noise`, the build of
`crates/geom-core/README.md`, "A number told to a person" (Ev, PR
7154).

**Letter H, tier dual.** A new public vocabulary type that every
sentence-writing crate will hold in its error payloads: an
architectural choice with broad reach that is hard to change once the
per-crate sweeps build on it, and its rounding rules (one-sided
offers, outward enclosure ends, the compared pair, the float-spacing
limit) are easy to get subtly wrong.

## Scope

1. **The type**, in `geom-core`: reading (value, enclosure,
   unreadable), dimension (length, angle, dimensionless), role (given,
   computed, offered), and the compared-pair form over two of them. It
   renders the number and the unit word; it has no arithmetic, no
   ordering and no accessor to a computed number. Any scalar `T`
   projects into it once, so an `Interval`, `Dual` or `Sym` payload
   reads as a value or an enclosure and never as a `Debug` struct.
2. **`profile` moves onto it**: every scalar payload field of
   `PathError`, `CornerReason`, `CornerRefusal` and `FilletLegCarrier`
   holds the told quantity, and `path::num`, `num_toward`, `grid_tol`
   and `shortest` are deleted, their rule living once in the type.
3. **`MarginDiag`'s sentence rendering** (`readable`,
   `readable_below_zero`, the sized endings in `predicate.rs`) is the
   type's rendering of a computed reading; an offered bound
   ("tighten the tolerance below … m") takes the offered rounding.
4. **The #990 doc** in `real.rs` names the type as the one projection.
5. **The gate** (`scripts/gates/`): in the crates this unit has
   converted, no refusal type declares a bare `f64` or `T` field and no
   refusal `Display` formats with `{:?}` or `{:e}`. It lists the crates
   it does not cover yet, so each later sweep removes a line.

## Not this unit

The notation-taking rendering and `quantity` (unit 2); `viewer`'s
`readout` grid (unit 2); the per-crate sweeps outside `profile` and
`geom-core`'s margins (unit 3 onward). `Readable` stays, for exact
text: field text, `Stackup::serialize` (a content key hashes it),
dumps.

## Measure first, and say what you found

- Who reads a scalar field of a `profile` refusal payload as a number
  (production code, the viewer, pncad-py, tests). A reader of a
  **given** number keeps an accessor; a reader of a **computed** one
  is a finding to report before working round it.
- Whether `PathError<T>` and its siblings still need `T` once their
  payloads are told quantities.
- Signed zero: `num` prints `-0` and `MarginDiag::readable` folds it
  to `0`, each pinned by a test. The rule is fold; say which pins
  moved.

## Decides, where a spec would otherwise assert

- The spacing limit is relative to the value (adjacent floats), never
  an absolute floor: a sub-nanometre margin must not render as `0`
  (the defect PR 2366 removed).
- An offered bound typed back must satisfy the inequality the sentence
  states, and an enclosure's rendered ends must contain its ends. Pin
  both against exact arithmetic over a sweep of magnitudes, not at one
  fixture.
- Two compared numbers that differ must not render alike, and must
  render in their true order.
