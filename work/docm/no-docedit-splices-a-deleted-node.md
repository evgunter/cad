---
id: no-docedit-splices-a-deleted-node
kind: issue
title: Deleting a feature from the middle of a chain is impossible: no DocEdit rewires a live node's inputs, so delete can only cascade
status: open
opened: 2026-08-31
github: 1324
---

## From GitHub issue 1324

Opened 2026-08-31; 1 comment.

A feature can only be deleted with everything downstream of it, because
`DocEdit` has no way to rewire a live node's inputs. In a boolean chain
that makes deleting one operation cost the whole tail.

## What exists

`DocEdit::DeleteNode` refuses with `EditError::DeleteWouldDangle` whenever
any live node still references the target — correct as a primitive, and it
stays. On top of it the viewer now offers **cascade delete**: the node plus
its whole downstream cone, as one undoable action, with the dependent count
named on the button before the click.

On `demos/tour/gallery/diefillet.pncad` — 21 `Boolean`s chained
`a = <previous>`, under two `Fillet`s — that is the only thing cascade can
mean. Deleting the Boolean that subtracts the *third* pip deletes the
other eighteen pips and both fillets, because every one of them sits
downstream of it. The user's intent was "remove this one cut".

## What is missing: splice

The CAD-conventional delete reconnects: drop node `N`, and rewrite every
consumer's reference to `N` into a reference to `N`'s primary input.
Downstream features rebuild on the earlier state instead of dying with it.
For the die, deleting a pip's `Boolean` would leave the other twenty pips
and both fillets standing.

This is **not expressible with today's edit vocabulary.** Every `DocEdit`
that changes the graph either mints a node (`InsertNode`) or removes one
(`DeleteNode`); none of them changes an existing node's inputs. `Rebind`
and `UpdateReference` move *names* and *pins*, not DAG edges. Deleting and
re-inserting the consumers is not a workaround: ids are never reused (D3),
so every downstream selection, witness, appearance and placement keyed by
those ids would be orphaned by the repair.

## Why it is a design conversation and not a patch

A new graph-edge edit is a **persisted, replayed** operation — `DocEdit` is
the edit log `persist::load` replays — so it is a `SCHEMA_VERSION` bump
(currently 18) with a ratified entry, and it wants Ev's sign-off before
anyone writes it. The questions it has to answer:

- **Which input is the survivor?** "Primary input" is well defined for
  `Fillet`/`Chamfer` (the target) and conventional for `Boolean` (the `a`
  operand), but it is a convention, and the vocabulary should state it
  rather than let each caller guess.
- **When is the splice refused?** Rerouting can produce a self-reference,
  a cycle, or a degenerate operand pair (`Boolean(a = X, b = X)`). Each
  wants a typed refusal, not a silently repaired graph.
- **A node with no inputs** (a `Profile`, a `Datum`) has nothing to splice
  to, so it keeps today's refusal — or cascades, if the user asks for that.
- **Names.** The consumer's stable names were composed over the deleted
  node's output. Splicing changes what they resolve against, which is the
  naming layer's own question and the part most likely to be underestimated.

## Scope note

Cascade delete is not wasted if splice lands — a cone delete is still the
right answer when the user means it, and the two want to sit side by side
with the cheap one clearly labelled.

Filed at Ev's request, out of the GUI-tweaks pass that added cascade
delete; deliberately excluded from that unit as too involved to fold in.

*(GUI orchestrator)*

## Comments

**2026-08-31** — comment:

Cascade delete has landed (branch `gui/delete`), and building it produced
two findings that sharpen this issue. Both come from an implementer who
went looking for splice with today's primitives and did not find it.

**Confirmed, first-hand: splice is not close to expressible.** Every
`DocEdit` variant either mints a node, deletes one, or writes a
slot/param/placement/meta on an existing one. None rewires a live node's
`inputs()`. `refactor::split` looks like a counter-example and is not: it
reroutes references by *rebuilding* nodes in a fresh document, and its
`DocEdit::Rebind` moves `StableName`s, not DAG edges — it cannot retarget
a `Boolean`'s `a`.

## 1. "Primary input" is a policy, not a derivation — and the ambiguous case is the motivating one

The issue above assumed the survivor is obvious for `Boolean` (the `a`
operand). It is a convention, and the node census says the convention is
the minority:

- **Unambiguous single input:** `Transform`, `Fillet`, `Chamfer`,
  `Extrude`, `Assertion`.
- **Two inputs, no intrinsic answer:** `Boolean { a, b }`,
  `Split { target, tool }`, `Revolve { profile, axis }`,
  `Sweep { profile, path }`, `Pattern` / `PlacedUnion` (`input` +
  optional `axis`).
- **No singleton answer at all:** `Loft { profiles: Vec<_> }`.

Splicing out one pip's `Boolean` means keeping `a` and orphaning `b` —
choosing which of two bodies the user meant. That is exactly the die
case that motivates this issue, so the ambiguity is not a corner to
defer; it is the first thing a design has to rule on.

## 2. A names-layer consequence, and the refusal it implies

`Node::Fillet` and `Node::Chamfer` carry `selection: Vec<StableName>`
frozen at authoring time, and `Node::Measure` carries `refs` whose `at`
field names the node a reference is *read at*. A splice changes what
feeds a consumer, and therefore changes which body those frozen names
resolve against.

So splice is not only an edge rewire. It has to say what happens to a
downstream fillet's frozen edge selection when the body under it
changes — which gives this issue a refusal case it did not have: **splice
into a consumer whose frozen selection cannot re-resolve.** That is the
part most likely to be underestimated, and it is where a wrong answer
silently blends the wrong edge rather than refusing.

## What shipped instead

Cascade delete: the node plus its downstream cone, as a single undoable
action, with the count and the kind breakdown on the button before the
click. The gap is named in the invariant voice at `SessionOp::DeleteNode`,
`DocSession::delete_node`, `DeleteAffordance::of`, and both new test
suites' headers, so a reader meeting the cascade finds this issue.

*(GUI orchestrator)*

## Home

`work/issues/` — the `DocEdit` vocabulary and the viewer's delete affordance are GUI-era ground and that program is closed; no open program's globs cover `editor-core`'s edit vocabulary.
