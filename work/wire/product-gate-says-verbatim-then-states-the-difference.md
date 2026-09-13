---
id: product-gate-says-verbatim-then-states-the-difference
kind: issue
title: the per-part at-rest gate's trigger is one decided policy with a home in neither of the two crates that implement it (product.rs called its copy verbatim)
status: open
pr: 2499
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

## Read against the code (2026-09-12, PR carrying this row)

**The two really are one rule with two spellings, and that makes this a
duplication row rather than a prose one.**

The import loop's `model.instances.len() > 1` counts instances, but an
instance is one solid there and nothing else can be: `build_one_solid`
assembles one `SolidSpec` into a body of its own, `transform_rigid`
maps it without changing its solid count, and `graft_disjoint` appends
one solid per call — which is what
`the_assembly_record_indexes_the_shipped_solids` pins, since the A7
record's per-instance `index` IS the shipped body's `solids()` order.
So `instances.len()` is the shipped body's solid count, and the import
loop's predicate is **"the shipped body holds more than one solid"** —
the same predicate `product.rs` spells `total_solids > 1`.

So the comment's second sentence is not "the explanation of how they
differ", as this row read it. It is the transcription hazard AT THIS
SITE: here a source may carry several solids, so counting sources would
answer a different question. Both sentences are true.

What was actually wrong was one word. "Verbatim" claims word-for-word
identity of an expression that is deliberately spelled differently at
each site, and it is the self-declared-copy vocabulary
`reviewer-style-lane.md` Q1 greps for — a claim no test can read. The
gate no longer makes it; it points at the module doc, which is where
this file states the shared shape once.

## What remains open

The policy itself — **the per-part at-rest gate is asked only when the
aggregate holds more than one solid, because with one the per-part and
aggregate subjects are the same body** — has no home. It is stated at
five sites across three crates, each pointing at another by prose:

| site | what it says |
| --- | --- |
| `step-import`'s materialization loop | the rule, as `instances.len() > 1`, with the identity-not-exemption reason |
| `step-import`'s shared-gate comment | the two subjects |
| `product.rs`'s module doc | "the same F8/D7 shape as the import loop" |
| `product.rs`'s `ProductError::SolidInvalid` doc | the trigger again, for the refusal |
| `product.rs`'s pass-2 gate | the rule, as `total_solids > 1` |
| `topo::graft_disjoint_all`'s doc | "the step-import loop's per-solid-then-aggregate shape" |

Nothing resolves any of those pointers, and the predicate is decided
independently in two crates. **What would settle it**: the F8/D7
policy stated once where both callers can cite it — `topo`'s at-rest
door is the only place both depend on — with the two call sites citing
that instead of each other. That is a cross-crate design move on a
ratified decision, not a comment edit, which is why this row stays open
after the word came out.
