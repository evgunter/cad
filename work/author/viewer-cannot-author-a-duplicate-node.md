---
id: viewer-cannot-author-a-duplicate-node
kind: issue
title: viewer: a 'duplicate' node reachable from the UI (Ev's request)
status: closed
opened: 2026-09-17
priority: P0
cost: D
branch: author/part-and-duplicate
pr: 3052
closed: 2026-09-24
---

**Ev requested this specifically** (in chat, 2026-09-17): a
**duplicate** node, authorable from the viewer's UI.

Ev's expectation, recorded as said rather than investigated: it should
be very close to **transform**, and could even be a **subtype of
transform**. The next step after duplicating is generally to move the
copy away anyway, so duplicate is "transform, with the original kept"
where transform is "transform, with the original consumed".

What a first look found (2026-09-17, not a design pass). The document
is a recipe DAG, and `Node::Transform { input, … }`
(`crates/editor-core/src/node.rs`) is a new node that places its
`input` without consuming it: the input stays in the DAG. So "keep the
original" is mostly a question of whether the input remains a visible,
independent body in the viewer, not a missing kernel capability.
`Node::Pattern` with a linear kind and count 2 already gives "the
original plus a moved copy" as a multi-body value, and `Node::Part`
selects one instance of it. It is worth comparing against before
building anything new.

Whether duplicate is a flag on transform, its own node, or a viewer
gesture over `Pattern`/`Transform` is a design choice to bring to Ev,
not to settle in the lane. If it does need a new document node, that
half is EDIT's (the document vocabulary), not CHROME's.

## Ev ruled: a Pattern of count 2 (2026-09-21, in chat)

Brought to him as this row asked, with the tree's own answer attached.
**Ruling: duplicate goes out as a `Node::Pattern` of count 2 — no new
document node, no flag on transform, and therefore no EDIT half.**

What the investigation found, and why it made the question a good one:
the row's first look said the input "stays in the DAG", which is true
and is not what a person sees. `roots::on_insert`
(`editor-core/src/roots.rs:174-181`) REMOVES a new node's inputs from
`Doc::roots` and puts the new node in the earliest consumed root's
slot; `doc.rs:730` states that the root SET is exactly the DAG's sink
set; and the viewer draws roots
(`viewer/src/display.rs:480`, `drawn_targets`). **So Ev's premise —
that transform consumes the original — is right about the viewport,
but the mechanism is the roots invariant and not anything inside
`Node::Transform`.** That matters, because "duplicate as a subtype of
transform" would have had to fight a ratified invariant, while a
`Pattern` of count 2 already yields the original placed whole plus one
moved copy and needs nothing new.

**One question left for the lane rather than settled here**: whether
`Part`-ing one instance out of the pattern removes the pattern from
`roots` and so stops drawing the other copy. If it does, the gesture
is `Pattern` plus two `Part`s. AUTH-4's spec asks the lane to settle
it by running.

Ev also raised unifying placement across normal placement, transform
and pattern behind one edited `placement` arg, and then ruled it a
bigger change for another program: filed as
`work/edit/placement-is-spelled-three-ways-node-registry-and-rule`
(P0, H, `needs_ev`), which this row does NOT wait on.

Dispatched with its sibling as **AUTH-4**
(`docs/AUTH-4-SPEC.md`, branch `author/part-and-duplicate`), because
`AddPart` is what makes a duplicate usable and both halves live in
`session/op.rs` and the create pane.

## Closed 2026-09-24 — PR 3052 merged (`2273a3a1`)

**Ev's gesture exists.** One button duplicates a picked body: a
`Pattern` of count 2 projected twice as one action and one undo, so
both copies stay drawn and either can be moved on its own. No new
document node, as Ev ruled.

What it took beyond the ruling, all found by running:
- **The copy must be picked as the drawn body, not its feature.** A
  viewport click handed tools the node that MADE the face, so
  duplicating a moved copy duplicated the original extrude onto an
  existing body. Tools now read `Selection::seat_node()`; the tree's
  `Selection::node()` is untouched, since for the tree the feature is
  the right answer. This also changed the boolean, split, transform
  and pattern tools, for the same reason.
- **The offset is measured, not fixed.** 20 mm overlapped the default
  shapes. The step is now the body's own x-width plus at least 25%,
  measured from a current evaluation — refused while the picture is
  older than the document, which a narrow re-review caught.
- **A multi-body input is refused** rather than silently doing nothing.

Residue: `work/vseam/an-action-of-several-edits-becomes-several-undos-after-reopen`
(the one undo becomes two after save and reopen; affects AUTH-3 too).
