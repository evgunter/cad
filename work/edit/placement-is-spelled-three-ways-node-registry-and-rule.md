---
id: placement-is-spelled-three-ways-node-registry-and-rule
kind: issue
title: Placement is spelled three ways — a DAG node, a document registry row, and a pattern rule — and the three disagree on whether a placement can be parametric
status: open
opened: 2026-09-21
priority: P0
cost: H
needs_ev: true
---

Filed by the AUTHOR orchestrator at Ev's direction (in chat,
2026-09-21), from a reading done while pricing
`work/author/viewer-cannot-author-a-duplicate-node`. **Ev's idea, in
his words**: a uniform way of placing the same geometry that would
unify normal placement, transform and pattern — *"if there were a
'placement' arg and instead of transform being a separate node, the
placement arg would just be edited"*.

Filed here rather than on AUTHOR because the ground is the document
vocabulary (`node.rs`, `doc.rs`, `edit.rs`) and because EDIT's own
`keep_out` says a design question here is an `[ev]` PR before it is an
implementation. `needs_ev` is set for the same reason: it changes
ratified design (D3's recipe DAG, ASSEMBLY-DESIGN A11, and the N1
naming substrate).

## The finding, which is real regardless of the remedy

Three things in this tree place geometry, and they are three
different mechanisms:

1. **A DAG node.** `Node::Transform { input, translation,
   rotation_axis, rotation_angle }` (`node.rs:2100`). The components
   are **`Expr`** — a placement here is parametric and can be driven
   by a document parameter.
2. **A document registry row.** `Doc::placements: BTreeMap<RecipeNodeId,
   placement::Frame>` (`doc.rs:741`), the A11 cluster placement
   registry, written only by the recorded `SetPlacement` edit, keyed
   by the `InstantiatePart` node whose singleton cluster it places. A
   **missing entry IS the identity frame**. The value is a concrete
   `Frame` — **not** an `Expr`.
3. **A rule.** `Node::Pattern { input, count, kind }` (`node.rs:2119`)
   — the master is "placed WHOLE at every placement", so the
   placements are derived from `PatternKind` rather than authored.

So the tree **already has Ev's idea**, for instances: placement as a
slot with a missing-means-identity default, edited rather than
inserted. It coexists with placement-as-a-node for bodies. The
comment at `doc.rs:1227` treats the registry as one of the
document's "two maps keyed by node id" without remarking that a third
spelling of the same concept sits in the node enum.

## Why this is not the cheap refactor it looks like

Three obstacles, each on ratified ground. Recording them so that
whoever prices this starts from the tree rather than from the idea.

**1. The three disagree on whether a placement is parametric.**
`Transform` holds `Expr`s; the A11 registry holds a concrete `Frame`.
Unifying them is not a syntax change — it forces a decision. Make the
slot parametric and the registry becomes expression-valued, which
reaches persistence, evaluation and the mate solver. Make it concrete
and `Transform` loses parametric placement, which is a capability
regression, not a simplification.

**2. `roots` is exactly the sink set, and `Transform` is what makes a
placed body a product.** `roots::on_insert` (`roots.rs:174-181`)
removes a new node's inputs from `Doc::roots` and puts the new node in
the earliest consumed root's slot, and `doc.rs:730` states the
invariant: *"the root SET is exactly the DAG's sink set"*. The viewer
draws roots (`viewer/src/display.rs:480`, `drawn_targets`). **That —
not anything in `Transform` — is why moving a body appears to consume
the original.** If placement stops being a node, nothing consumes the
input, so the original stays a root and every existing document's
product structure changes. That is a persisted-format question, not
only an in-memory one.

**3. Face names are role paths through the DAG.** `RecipeNodeId`'s
doc calls stability "a contract, pinned by test", and N1 names embed
the minting node. Removing a node kind from the DAG changes the role
path of every face downstream of a placement, which is the names
layer (`names/role.rs`, `IDENTITY.md`) and the reason `StableName`
exists.

And a fourth, for whoever scopes it: **mates already compute
placements**. `SolvedPoses::placement` composes the A11 cluster frame
with a solved relative pose (`doc.rs:952-960`). A unified slot has to
say how an authored placement and a solved one compose, or which one
wins — that is MSOLVE's ground as much as EDIT's.

## What this row is NOT

It is not a blocker for
`work/author/viewer-cannot-author-a-duplicate-node`. Ev ruled in the
same conversation that duplicate goes out as a `Pattern` of count 2,
which needs no new document node and no part of this row. If this row
ever lands, duplicate may be re-spelled against it; it does not wait.

## Suggested first step, if it is taken

Not an implementation. The honest first move is an `[ev]` PR that
answers ONE question — **is a placement parametric?** — because the
answer decides whether this is a unification at all. If placements are
concrete, `Transform` cannot fold into the registry and the row
collapses to "the registry and the node should share a type". If they
are parametric, the registry grows expressions and the scope reaches
the solver and the wire format, and the row is a program rather than a
unit.
