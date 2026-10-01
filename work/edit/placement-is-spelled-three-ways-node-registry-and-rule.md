---
id: placement-is-spelled-three-ways-node-registry-and-rule
kind: issue
title: Placement is spelled three ways — a DAG node, a document registry row, and a pattern rule — and the three disagree on whether a placement can be parametric
status: spec
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

## RULED (2026-09-29, Ev on `[ev]` #3437) — a placement is parametric

"yes!" A cluster's placement in the A11 registry has `Expr`
components, the one placement type `Node::Transform` holds too, so a
document parameter can drive where a cluster sits. A frame the
maintenance mints from a solved pose is written as literals. A11
rule (2) says so (`crates/editor-core/ASSEMBLY.md`).

**What this settles, and what it leaves.** The first unit is one
`Placement` type of `Expr`s, shared by `Node::Transform` and
`Doc::placements`, evaluated at the document's parameters before mates
compose onto it. It reaches persistence (the registry's wire form) and
the solver's composition point (`SolvedPoses::placement`), which is
MSOLVE's ground. Whether `Transform` stops being a node and becomes an
edited slot is not decided. It stays this row's later question, with
the two obstacles recorded above (`roots` as the sink set, and names as
role paths through the node).

## Put to Ev (2026-09-29) — where a placement lives, after two designer pairs

**The first pair.** It weighed the unit's design: one parametric type
for the registry and `Transform`, and maintenance that mints only a
literal (`docs/DESIGN-FORK-LOG.md` row 17). Ev did not choose between
its two sides. Ev asked what the recorded literal is for, found it "a
weird side channel through which to keep the source", accepted that a
part may jump when the thing putting it there is gone, and proposed a
gauge that attaches to several members at once.

**The second pair.** It weighed that widened problem (row 18) and
converged. A11 (2)–(5) now state the result:
- **The gauge is a node.** It holds a parametric placement, a chain of
  rigid `Expr` steps and literal matrices; `Transform` holds the same
  type.
- **An instance's pose** is its gauge's frame composed with an offset.
- **Mates place only within one gauge.** Contact across gauges is
  declared and verified, never placed.
- **Extra statements of where a placed instance sits** are verified,
  never dormant.
- **No edit records a frame.** Maintenance, its logged rows and its
  solve at the edit door all go.
- **The accepted jump.** A part whose placing source is gone sits at
  its gauge's origin.

**Ev's answers so far (2026-09-29, on #3441):**
- Membership: (a) is leaned towards, since a "copy x's gauge to y, then
  mate" shortcut stays possible as one compound edit.
- Deleting a gauge: "definitely not refused".
- Ev asked whether a group nothing places can be shown without a jump,
  with the displayed location "only a convenience" that never enters
  the logic.

**The third pair (row 19).** It weighed what such a group becomes. Both
designers agreed:
- the viewer holds the group where it was last shown, as display state
  (G3's probe, widened to a group);
- placing it "where shown" is one edit whose frame the user supplies.

A11 (2) now states these points and the settled ones.

**What Ev rules on:** the pair split, and crossed in reconciliation, on
one question. When nothing places a group, does the logic:
- **answer at a default pose?** Evaluation stays total and A9's "one
  deterministic body" holds. The jump reaches export and the gate, and
  the GUI annotates it. Deleting a gauge re-gauges its instances to the
  world.
- **or refuse?** "Unplaced" is a state. Export, the gate and cross-group
  queries refuse with a recourse, and nothing unauthored enters the
  logic. Deleted-gauge references are kept, so the refusal names the
  cause.

**Sequencing (orchestrator).** The box and seed lanes still solve mates
at the nominal, a silent class filed on MSOLVE as
`work/msolve/a-mate-through-a-parametric-placer-is-solved-at-the-nominal-in-box-and-seed-runs`.
The placement unit ships no parametric placement into the lanes
without that refusal.

## RULED (2026-09-29, Ev on `[ev]` #3441) — gauges, and an unplaced group lives in its own space

Ev settled the design over the comments on #3441. A11 (2)–(5) and A9 now
state it.

- **Placement lives on a gauge.** A gauge is a document node holding a
  parametric placement: a chain of rigid `Expr` steps and literal proper
  matrices, the type `Node::Transform` shares.
- **Membership is (a).** Each instance names its gauge (the world by
  default) and may carry an offset in it.
  - Mates place parts only within one gauge.
  - Contact across gauges is declared and verified, never placed.
  - "Copy x's gauge to y, then mate" is one compound edit.
- **No edit records a frame.** Maintenance, its logged rows and its solve
  at the edit door all go; replay re-applies edits alone.
- **Deleting a gauge, a placed member or a placing mate is never refused.**
- **A group nothing places lives in its own space.** Ev: "sounds perfect!
  STEP can complain about unplaced parts, that sounds good."
  - It is evaluated in its own frame.
  - Nothing outside the group is compared with it: the at-rest gate and
    cross-group measures do not ask.
  - STEP export refuses unplaced parts, naming how to place them.
  - The viewer draws the group where it was last shown, as display
    state no logic reads (G3's free-move probe, widened to a whole
    group).
  - "Place where shown" is one edit whose frame the user supplies.

The designer pairs are in `docs/DESIGN-FORK-LOG.md` rows 17, 18 and 19.
Their blinding bytes are on
`analysis/design-fork/edit-placement-type-2026-09-29`.

**Next.** The unit's spec is rewritten from this ruling in
`docs/EDIT-PLACEMENT-SPEC.md`, and it is built with a dual review. The
box/seed MSOLVE row still gates shipping any parametric placement into
the lanes.

## Spec'd (2026-09-29, EDIT orchestrator): three units, P1 first

`docs/EDIT-PLACEMENT-SPEC.md` holds the slate:
- **P1**, the `Placement` type held by `Node::Transform`, spec'd in full, dual review;
- **P2**, gauges: the registry and maintenance go, an unplaced group lives in its own space, and STEP refuses unplaced parts; outlined, and spec'd in full when P1 merges, dual review;
- **P3**, the viewer's group-wide probe and "place where shown", filed on the viewer owner's slate when P2 merges.

The row closes when P2 merges.

## Built (P1, 2026-09-29, PR 3497)

`Node::Transform` holds one `Placement` (`crates/editor-core/src/placement.rs`):
- A chain of `Step::Rigid` (the three `Expr` components) and `Step::Matrix(Frame)`, composed left to right as a product (`[a, b]` is `a ∘ b`).
- One evaluator: `Placement::eval`, over the construction the node evaluation and the mate solve both read. A one-step rigid chain moves every corpus body by the bits it did before (51 transforms, pinned on the merge base).
- `Placement::literal` is the bit-exact `Frame` door.
- Addressing: a later rigid step's components are `SlotId::PlacementStep { step, arg }` through `SlotId::rigid`, and step 0 keeps the transform's three slots.
- A matrix step is held to A6 at the edit door and at load.
- An old file refuses `Unreadable`.
- Python: `Placement` and `Node.transform_by`.

Not built here, as P2's: `Doc::placements`, `InstantiatePart`'s gauge and offset, the maintenance's removal, and gauges.

The row stays open for P2 and is set back to `spec` when this merges.

Filed from the sweep: `work/edit/node-bit-eq-compares-a-mates-alignment-by-value.md`.

### Fix pass (2026-09-30, PR 3497)

- The step kinds are `Step::Rigid` and `Step::Literal` (the wire too), and `Placement::compose` (`a.compose(b)` is `a ∘ b`, `Frame::compose`'s order) builds a chain from two, in Rust and Python.
- The chain folds through one composition rule with `Frame::compose` (`placement::Motion`), so an identity literal anywhere moves no bit; the empty chain is the identity (`Placement::IDENTITY`), admitted at both doors. P2's identity offset can be it.
- A literal frame is rigid at every door that admits one — a transform's step, the registry, an explicit rule's listed frame — by the evaluation's own predicate (`topo::check_rigid`), refused `NonRigidPlacement` naming the frame.
- Filed: `work/lib/python-slot-words-stop-short-of-a-step-index.md` (LIB), `work/offer/viewer-free-move-decides-rigidity-by-its-own-predicate.md` (OFFER, for P3).
