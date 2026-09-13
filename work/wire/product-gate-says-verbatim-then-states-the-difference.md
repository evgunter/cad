---
id: product-gate-says-verbatim-then-states-the-difference
kind: issue
title: product.rs's per-source gate calls itself the import loop's rule verbatim and the next sentence says it counts something else
status: open
opened: 2026-09-12
---



(WIRE orchestrator) Found by the full review of PR 2442 as a prose
residue in a file the PR edits (S11), sharpened here. Pre-existing and
out of that PR's fence.

`crates/editor-core/src/product.rs:664-668`:

> Pass 2: the per-source gate, asked only when the product holds more
> than one solid (**the import loop's rule, verbatim**). The count is
> over SOLIDS, not sources: one source may itself carry several (an
> instantiated sub-assembly), and it is the product's solid count the
> rule speaks about.

The two sentences disagree. The import loop's condition is
`model.instances.len() > 1` (`crates/step-import/src/lib.rs:700`) — an
**instance** count, over the instances of one solid spec. The product's
is `total_solids > 1`, summed over every source's `solids().count()`
(`product.rs:669-673`). They are not the same predicate and they do not
count the same things; the very next sentence is the explanation of how
they differ, written as if it were an elaboration of "verbatim".

Both rules may well be right where they sit — the import loop's
per-solid subject is the placed copy of one spec, the product's is the
aggregate — and that is the point: the word "verbatim" claims a shared
authority that neither site can check and that the comment itself
immediately contradicts. A reader who trusts it and changes one rule
will not know the other did not move.

What is owed is small and is a choice between two shapes: either the
two predicates really are one rule about "the subject a gate is asked
about" and should have one home that both cite, or they are two rules
and this comment should say which one it is and what the import loop's
does differently. Nothing gates the word either way, which is the
general hazard — a self-declared copy is a claim no test can read.

The same file carries two more statements of the condition
(`product.rs:42`, `:155`), both accurate about solids; they are the
rule's restatements, not the false part.
