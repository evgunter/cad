---
id: does-n3-retire-loudly-generalise-to-the-folds-other-compositions
kind: ruling
title: N3 rules what a name means when a MERGE consumes its entity; three open rulings ask the same question for four other compositions - does N3's answer generalise?
status: open
opened: 2026-09-15
needs_ev: true
---


## The question, in one sentence

**N3 already rules what a stable name means when a MERGE consumes the
entity it names. Three open rulings on this slate are the same question
for four other compositions. Does N3's answer generalise, and if not,
where does the line fall?**

This is a FRAMING question, asked before the three rulings are answered
rather than instead of them. If the framing is wrong, answering the three
separately risks three inconsistent answers to one question — which is
how this vocabulary accumulated two spellings of things before.

## What N3 already decides (ratified, `crates/editor-core/src/names/README.md`)

> **N3 — Merge policy: names retire into the merge, loudly.** … The
> constituents retire: **referencing one fails with the merged name
> offered**, and when an edit removes the coincidence the merged name
> vanishes with its constituents offered.

So for one composition the contract is complete and three-part: the old
name **refuses** rather than silently re-pointing; the refusal is
**typed** (N5 `ResolveError::Vanished { name, diagnosis }`); and it
**offers the replacement**. The machinery for all three exists and is
load-bearing.

## The four compositions where it is not decided

Each is a way the fold or the gather breaks *one name denotes one entity*
in the result, and each fails differently today.

| composition | what happens to the named entity | what a document gets today |
| --- | --- | --- |
| **split by a later member** | becomes SEVERAL | `DeclareResolve { Vanished }` in some member orders — refuses, **no offer**, because which fragment is a geometric question |
| **consumed by containment** | becomes NONE | `Vanished` in two orders, `ContactContradicted` in four |
| **inside a fragmented merged row** | is in a set that is no longer a merge | unreachable: the emitter refuses the shape first |
| **the operand seat (A/B)** | denotes a DIFFERENT entity depending on member order | nothing refuses; the names silently differ |
| **one instance under two roots** | is DOUBLED | `ProductError::Naming` — a collision the author never authored |

The rulings: `member-space-look-through-stops-at-splits-containment-and-fragmented-merges`
(the first three), `the-pair-verbs-declared-merge-is-asymmetric-in-its-operands`
(the fourth), `product-refuses-naming-when-one-instance-is-placed-under-two-roots`
(the fifth).

## Why N3 might NOT simply generalise — the three places it strains

Stated because a framing that only works is not worth ratifying.

1. **N3's offer is computable and these mostly are not.** A merge has ONE
   replacement, named by the merged row. A split has several, and which
   one the declaration meant is a geometric question — re-measuring the
   member face against the partners, which DM4's routing step forbids.
   Containment has no replacement at all. So "refuse with an offer" may
   degrade to "refuse", and a refusal with no offer is a weaker promise
   than N3 makes.
2. **The operand-seat case has nothing that refuses.** The other four are
   about which refusal an author gets. This one produces two *valid*
   documents with different names, so generalising N3 to it means
   deciding a union's names should be order-free — which moves `Fragment`
   rows in every existing declared-merge golden and corpus document.
3. **The two-roots case is the mirror image.** Every other row is one
   name whose entity went away; this is one name with two live entities,
   where `Entry::Tied` (N4) is the table's existing shape for exactly
   that. Whether the gather should qualify by root, or refuse in the
   recipe's vocabulary, may be answerable from N4 rather than from N3.

## What is NOT being asked here

- Not the per-case answers. Those stay on their three rulings.
- Not a change to N3 itself.
- Not anything a lane is blocked on: all three rulings are `kind: ruling`
  and nothing is dispatched against them. WIRE has other work and is
  doing it.

## What an answer would look like

Any of these is a complete answer and unblocks the three:

- *"Yes, N3 generalises — refuse and offer where an offer exists, refuse
  without one where it does not"*, which makes the three rulings
  applications rather than decisions.
- *"It generalises to the consumed cases (1–3) but the operand seat and
  the two roots are different questions"*, which splits three rulings
  into two groups and says so.
- *"The framing is wrong, here is the seam you are flattening."*

## How this was found

Reading the three rulings together after the WIRE orchestrator's
2026-09-15 read of DOCM's residue, which established that five of those
seven rows are rulings rather than units. The first framing offered in
chat was that the naming layer HAS no one-name-one-entity contract; that
was wrong, and N3 is the correction — the contract exists, is ratified,
and covers exactly one composition.
