---
id: does-n3-retire-loudly-generalise-to-the-folds-other-compositions
kind: ruling
title: N3 rules what a name means when a MERGE consumes its entity; three open rulings ask the same question for four other compositions - does N3's answer generalise?
status: closed
opened: 2026-09-15
closed: 2026-09-15
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

## RULED (Ev, PR 2677, 2026-09-15)

> *"refuse and offer where a unique best offer exists" is good. for
> context on n3, the decision that was made against was not even
> refusing, just silently taking the merged descendant*

**The rule, as ratified:** a composition that breaks *one name denotes
one entity* **refuses**; it **offers** a replacement where a **unique
best** offer exists, and refuses with no offer where one does not.

**And the second sentence is the load-bearing half.** N3's rejected
alternative was not "refuse without an offer" — it was **silently taking
the merged descendant**. So what the rule protects is *never silently
re-point*; refusing is the floor and the offer is the courtesy above it.
That is what makes the rule decidable for cases where no offer can be
computed: the absence of an offer is not a reason to fall back to
silence.

The mechanism is already shaped for it: `ResolutionFailure::offers` is
`Vec<StableName>`, documented *"Empty when nothing structural offers
itself"*, so zero, one and many are all expressible and "refuse with no
offer" needs nothing built.

## What it settles, case by case

Read against the five compositions this row tabled. **Three are settled,
one is substantially settled, one is not settled at all** — recorded
this way rather than as a blanket "yes" because a ruling over-applied is
the same failure as three inconsistent answers, in the other direction.

| composition | verdict under the rule |
| --- | --- |
| **split by a later member** | **Refuse, no offer.** Which fragment the declaration meant is geometric, and DM4's routing step forbids re-measuring there — so no *unique best* offer exists. Settled, with one residue below. |
| **consumed by containment** | **Refuse, no offer.** No replacement exists at all. Settled outright. |
| **inside a fragmented merged row** | **Refuse, no offer**, on the split's reasoning. Unreachable today because the emitter refuses the shape first — which is `two-emitter-refusals-a-legal-declared-union-reaches`, in flight. |
| **one instance under two roots** | **Refuse** — which `ProductError::Naming` already does, so the refusal's KIND is now correct and was never the question. No unique best offer exists (both roots are equally the author's). What remains is that the refusal is LATE and in the wrong vocabulary, which is a unit, not a decision. |
| **the operand seat (A/B)** | **NOT settled.** Nothing here re-points a name: no name vanishes and nothing resolves to the wrong entity — two valid documents simply mint different names. The rule is about a reference whose entity went away, and this is not that. Stays a ruling. |

### The one residue on the split case

The rule says *unique best*, and N3's own offer is **plural** (*"the
merged name vanishes with its constituents offered"*). So a reading
exists under which a split offers its fragment SET rather than nothing.
This row takes the narrower reading — **refuse with no offer** — on the
ground that N3's plural case is an exact DECOMPOSITION of what the
merged name covered, whereas a split's fragments are CANDIDATES for what
the reference meant, and offering candidates is one step from the silent
pick the rule exists to prevent. A lane that finds that wrong says so and
this paragraph is corrected in the same PR.

## Consequences on this slate, applied in this PR

- `member-space-look-through-…` — re-kinded **`issue`**: the decision is
  made and what remains is applying it.
- `product-refuses-naming-…-two-roots` — re-kinded **`issue`**: refuse
  earlier and in the recipe's vocabulary.
- `the-pair-verbs-declared-merge-is-asymmetric-…` — stays **`ruling`**,
  and is now the only one.
