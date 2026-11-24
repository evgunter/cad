---
id: an-indeterminates-sentence-is-composed-as-a-string-in-metres
kind: issue
title: geom-core's Indeterminate and recourse sentences are composed as Strings, so they read in metres in every notation and a margin carries no unit word
status: open
opened: 2026-11-20
priority: P2
cost: H
parent: patherror-display-renders-float-noise
---


`geom_core::predicate` composes an undecided reading's sentence and
its recourse as `String`s (`MarginDiag::sized_recourse`,
`sized_recourse_in_file`, `miss_recourse_in_file`, `Indeterminate`'s
`Display`, `UndecidedRefusal`), with `format!` over told quantities.
So the sentence is in metres whoever reads it, and a refusal that is
told in a notation (`profile::PathError::Escalated`,
`PathError::Band`, `PathError::Structure`) reads in the reader's
units up to the point it forwards to one of these and in metres after.

Two of the numbers are the tolerance's and are metres by rule (the
offered tolerance of `Band::offer_told`, now built
`Told::of_the_tolerance`, and ε_in). The rest are margins:
`MarginDiag::readable` writes a margin's number with no unit word
(`Told::number`), and a margin is a length for some predicates and a
sine, an area or a parameter for others, which the type does not say
(`three-dimensions-do-not-cover-the-sentences-already-in-the-tree`).
A number with no unit word cannot be converted without becoming
ambiguous, so these wait on the margin's dimension being typed.

Found by the notation unit (`a-refusal-is-told-in-the-readers-notation`),
which left `predicate.rs`'s sentences on `Display`.
