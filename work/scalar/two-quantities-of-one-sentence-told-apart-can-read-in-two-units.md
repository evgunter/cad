---
id: two-quantities-of-one-sentence-told-apart-can-read-in-two-units
kind: issue
title: Two quantities of one sentence told through separate doors can read in two units
status: closed
opened: 2026-11-20
priority: P2
cost: M
parent: patherror-display-renders-float-noise
rides_with: a-number-in-a-unit-is-rounded-once
closed: 2026-11-21
---


A `geom_core::Compared` pair and a `geom_core::Position` fall back to
the canonical unit **whole**: both members read in one unit
(`crates/geom-core/src/told.rs`, `Compared::form` and `Position`'s
`tell`). Two quantities a sentence tells through separate doors do
not: each falls back alone
(`a-unit-that-cannot-say-a-quantity-falls-back-to-the-canonical-one`).

The measured instance is the viewer's range wording,
`valid from 1e307 mm … up to 1e306 m`
(`crates/viewer/tests/valid_range.rs`,
`a_bound_with_no_millimetre_value_is_not_worded_as_infinity`;
`crates/viewer/src/bounds.rs`, `Bounds::wording`, which tells each
bound through `props::shown_text`). In `profile`, any refusal that
tells two lengths without comparing them can do the same (a radius
and a setback; a given length beside a computed one, where the given
one has no preimage in the unit).

## The question

Is the unit of a sentence a property of the **sentence** or of each
**quantity**? If the sentence's: the door needs a way to tell several
quantities together (a group that falls back whole, as `Compared`
does for two), and the `tell` bodies need to use it. If each
quantity's: the two-unit sentence stands and the unit word carries
it.

## Decided

Ev, PR 7261 (2026-11-21): settled by `a-number-in-a-unit-is-rounded-once`, which this row rides with and which builds the answer.

## Closed

Built by `a-number-in-a-unit-is-rounded-once`: every unit says every quantity, so the unit a quantity is told in depends only on the reader's notation and its dimension, and two quantities of one sentence through separate doors read in one unit. The one quantity told in metres in every notation is the tolerance, as ratified. A compared pair of two dimensions no longer constructs (`Told::beside` panics: a builder's bug), and a position's coordinates must be of one dimension. The viewer's `valid from 1e307 mm … up to 1e306 m` reads `… up to 1e309 mm`; the row there is the viewer's, and its test is re-baselined in the same PR.
