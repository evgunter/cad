---
id: nested-pattern-mate-heads-refuse
kind: issue
title: Nested patterns and pattern-of-transform mate heads refuse DanglingHead — narrower than the A11 rider's literal text
status: open
opened: 2026-08-31
github: 1411
refs: [1400]
---

## From GitHub issue 1411

Opened 2026-08-31; 0 comments.

Filed from the MATE-1 dual review (PR #1400, both arms; R1 MINOR-5(a)). The landed member vocabulary accepts `Pattern` + `Instance(i)` over a live `InstantiatePart` input only. A nested pattern's copy (pattern-of-pattern) and a pattern over a transform both refuse `DanglingHead` at the (outer) pattern node — typed and honest, and disclosed in the PR body, but NARROWER than the rider's literal text, which fences the head spelling (`Pattern` node + `Instance(i)` qualifier) and says nothing about the pattern's *input*.

Two dispositions possible, and the choice is a small design call: extend `head_of`'s member vocabulary through nested inputs (the derived offset composes through the chain — the rule-1 conjugation is associative over it), or ratify the single-level fence as intended v1 scope with a sentence in the rider. S-MATE's backlog either way; not scheduled as a unit yet.

Signed: (S-MATE orchestrator)

## Home

`work/mate/` — the issue names S-MATE's backlog explicitly, and mates × patterns (the A11 member vocabulary) is the program's charter ground.

## The ruling asked (FIX orchestrator, 2026-09-04)

Read at `crates/editor-core/src/mate/solve.rs` before asking. Three
things the filing does not say, one of which changes the question.

### 1. Nothing is silently wrong, and the fence is already stated

`head_of`'s own doc comment (`solve.rs:150-155`) already names every
excluded head: *"a pattern whose input is not itself a live instance —
a patterned boolean, a nested pattern — resolves to no member and
refuses."* So the code says what it does; what is narrower than the
rider's literal text is the **scope**, not the honesty. This is a
scope ruling, not a defect report.

### 2. The title names TWO questions, and one of them does not follow
from the other

**(a) Nested pattern** — `Pattern` over `Pattern` over
`InstantiatePart`. Well-posed: `stepped_rule_map`'s maps LEFT-compose
and rigid maps are associative, so copy `(i₂, i₁)` has pose
`M₂(i₂) · M₁(i₁) · placement(I)`, and the cluster-graph vertex is
still the ultimate `InstantiatePart` — exactly what `Member.instance`
is documented to be. The filing's associativity claim is correct for
this case.

**(b) Pattern of transform** — `Pattern` over `Node::Transform` over
an instance. This is a DIFFERENT question and ruling (a) does not
answer it. There is no `Instance(i)` chain to compose; what is open is
whether a `Transform` node is a pose-carrying vertex of the cluster
graph at all, or an opaque geometry node the mate solve is right to
refuse to stand a member on. Answering (a) yes and (b) no is coherent;
so is refusing both.

### 3. "A small PR either way" is false for one branch — measured

Ratifying the fence IS a one-sentence PR. Extending is not.

`Member.copy` is `Option<(RecipeNodeId, u32)>` — **one level, by
type**. A nested member's identity needs the whole chain, because the
reason `copy` is in `Member` at all is sibling-copy distinctness (two
mates to sibling copies are different pairs, which is what makes the
second one close a loop), and that argument holds at every level of
the nest. So extending means `Member` carries a chain: it loses
`Copy`, and it is the `BTreeMap` key for `by_pair` (`solve.rs:763`)
and `edge_of` (`:865`) and the thing the spanning tree selects its
edges by (`:843-861`). The blast radius is confined to
`crates/editor-core/src/mate/solve.rs` — that part is small — but it
is a change to the member-identity type the pair keying and the
spanning tree are built on, not a second match arm.

### The recommendation, if Ev wants one

**Ratify the single-level fence as intended v1 scope** with a sentence
naming what it excludes and why, and re-file the extension as its own
S-MATE unit with its own red-first rows, to be scheduled when a
construction actually wants it. Two reasons: the case arrived from a
review sweep rather than from a construction that needed it, so there
is no measured demand; and a change to the member-identity type wants
a unit that can carry the loop-closure rows for a nested member, which
a one-PR fix program cannot.

Ev's call either way — and if (a) and (b) should be split into two
rulings, that is also an answer.

## RULED (Ev, PR 1731, 2026-09-04)

**Both in.** The member vocabulary extends through identity-transparent
nodes — nested patterns and pattern-of-transform alike. The
single-level fence is NOT ratified.

Ev's words, on the sequencing this orchestrator proposed: *"sounds
good"*, against the plan below.

### One rule, not two

Resolve a mate head through identity-transparent nodes to the minting
instance, composing each level's static map. That covers both cases the
title names. What made this one question rather than two is that
`wire_transform` is an identity-preserving pass-through by spec D2 — a
`Transform` contributes **no `RolePath` segment** — so the head resolves
through it exactly as it resolves through a nested pattern's chain.

The natural case that decided it: **a mate onto a transformed,
unpatterned instance is accepted today.** Add a pattern and it refuses.
Nothing in the user's model changed between those two.

### The gate, and it is not optional

**`work/issues/mate-solve-is-transform-blind.md` must be fixed first or
alongside — never after.**

Measured, through ordinary doors: the solve is transform-blind and
silent. A `Node::Transform` between an instance and mate-named material
gives a **string-identical solved frame** with and without the
transform, `fault = None`, `product = Ok`, and mated faces 10 apart — a
green document with a gap where the author declared contact.

The patterned form of that same shape is **loud**: it refuses
`DanglingHead`. So the refusal this ruling retires is currently **the
only guard on the silent half's patterned twin.** Extending the
vocabulary without composing the transform's map converts one silent
wrong answer into two.

The characterization rows for it are on main (PR 1773) and are written
to go **red when the bug is fixed**; their header says the fix deletes
them rather than updating them.

### The order of work

1. A `derived_offset` sibling that walks the input chain and composes
   **every pose-bearing node's map** — not just the pattern's. This is
   the transform-blindness fix and it is the unit's first task, not a
   follow-on.
2. Then the vocabulary extension rides on it.

The measured cost of (2), unchanged from this file's opening: `Member`
grows from `Option<(RecipeNodeId, u32)>` to a chain, loses `Copy`, and
it is the `BTreeMap` key for `by_pair` and `edge_of` and the thing the
spanning tree selects its edges by. Bounded to `mate/solve.rs`, but it
is the member-identity type the pair keying and spanning tree are built
on, and it owes loop-closure rows for a nested member that nothing in
the suite has today.

### The coupling worth carrying

From the measurement lane, and it reframes the whole question:

> `member_of` decides admission on **naming** transparency; correctness
> needs **pose** transparency. They coincide today by accident.

`Transform` is the unique production node that moves geometry while
contributing no `RolePath` segment. So the vocabulary question was
never really about naming.

### Re-homed

S-MATE closed while this ruling was open, and DOCM inherited the FILES
at that exit rather than this class of question. Ev's steer
(2026-09-04): open a successor. This item and its gate move to
**MSOLVE**. Not a FIX one-PR item under any reading.
