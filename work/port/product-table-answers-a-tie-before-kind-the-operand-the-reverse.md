---
id: product-table-answers-a-tie-before-kind-the-operand-the-reverse
kind: issue
title: The gate answers Ambiguous for a tied non-face in the product's own rows but NotAFace for the same tie at the operand
status: closed
opened: 2026-09-06
parent: PORT-DOORS-1
pr: 2635
closed: 2026-09-15
---



Left by MSOLVE-5 (PR 2090), pinned by value in
`crates/editor-core/tests/msolve5_read_below_a_root.rs` (the tied-face
and tied-edge rows assert their controls at the pattern root).

`resolve_face` (`crates/editor-core/src/assembly.rs`) matches the
PRODUCT's own rows tie-first: `Entry::Tied` is `Ambiguous { width }`
whatever the entities' kind, and only a `Unique` non-face is
`NotAFace { kind }`. `operand_answer`, the sibling MSOLVE-5 added for
the operand's table, was ruled kind-first: a non-face entry, unique or
tied, is `NotAFace` because a non-face never mints anywhere, so what
it is precedes where it is rooted. The same tied edge therefore
answers `Ambiguous` read at the root and `NotAFace { kind: Edge }`
read one node below it. Both are refusals and both are true; only the
first word differs. MSOLVE-5's spec put `Ambiguous`/`NotAFace`
semantics out of scope, so the product match was left verbatim.

What is owed: one order for both tables — kind-first in the product
match too (a one-arm change: the kind is the name's, which the table
enforces for every candidate), with the `display_contract` and the
two control assertions moved — or a stated reason the product's rows
answer differently.

## Decided: kind-first in the product match too (PORT orchestrator, 2026-09-15)

Ev was asked whether this call was theirs and answered that if the
orchestrator is confident it is the orchestrator's. It is. The row
offered two ways out — one order for both tables, or a stated reason
the product's rows answer differently — and no reason for the second
survives reading the two functions.

MSOLVE-5's argument for kind-first in `operand_answer` is that **a
non-face never mints anywhere**, so what an entity IS precedes where
it is rooted. That premise is about the *name* being resolved, not
about which table is being read: an edge is not a face at the pattern
root either. A reason for the product's rows to answer differently
would have to be a fact about the root that changes what the name
denotes, and there is none — the root is where the name is *looked
up*, not where its kind is decided.

The shape confirms it. `operand_answer` dispatches kind, then
rootedness, then collapses the tie; `resolve_face` dispatches the tie
first and reaches kind only on a `Unique`. So one function treats the
entry's multiplicity as prior to the name's kind and its sibling
treats it as posterior, in the same file, over the same two facts.
Kind-first makes the dispatch order one rule.

It is also the more answerable refusal, which is this program's
standing question (`work/port/plan.md`, Review posture).
`Ambiguous { width }` tells the reader to disambiguate a reference
that can never resolve however narrow they make it; `NotAFace { kind }`
tells them the reference names the wrong sort of thing. Only the
second is actionable, and today which one you get depends on where you
read from.

**What moves with it**, as the row already says: the `display_contract`
arm, and the two control assertions in
`crates/editor-core/tests/msolve5_read_below_a_root.rs` that pin the
tied-face and tied-edge answers by value at the pattern root. Those
assertions are a baseline, not a target — `docs/prompts/implementer-discipline.md`
§3 — so they are re-taken and the PR says what moved.

**Dispatches as one unit with
`assembly-door-raises-only-the-head-of-each-refusal-list`.**

## Re-homed to PORT (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PORT collects the crate-boundary doors — the Python surface, the
exchange crates and the façade refusals — where the thing a user meets
is a refusal. This row is one of them.

Its class at the cut was **M** — one-arm change, but which order is
right was put out of MSOLVE-5's scope and needs deciding. The class is a
dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.
