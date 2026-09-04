---
id: nested-pattern-mate-heads-refuse
kind: issue
title: Nested patterns and pattern-of-transform mate heads refuse DanglingHead — narrower than the A11 rider's literal text
status: open
opened: 2026-08-31
github: 1411
refs: [1400]
needs_ev: true
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
