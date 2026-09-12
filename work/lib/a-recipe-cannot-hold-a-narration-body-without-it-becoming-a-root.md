---
id: a-recipe-cannot-hold-a-narration-body-without-it-becoming-a-root
kind: issue
title: a recipe cannot hold a narration or probe body without it becoming a product root
status: closed
opened: 2026-09-08
closed: 2026-09-09
---


A document's product roots are its SINKS (`editor_core::roots`), and
the viewer draws the product. So any body a recipe builds and does not
feed onward is a body the gallery draws — there is no way to say
"this one is measured, not modelled".

`demos/tour/src/teapot.rs` pays that cost three ways in one scene:

- `gallery_document` DELETES three sinks before handing the document
  over — the sealed hollow (the same operand and wall as the cup, so
  it would render inside it) and the two refusing unions (which hold
  the cup, the spout and the handle out of the root set while they
  stand). The file the GUI opens is therefore not the document the
  scene walks, which is the thing `demos/tour/src/gallery.rs`'s
  module docs say a gallery must not become.
- `wall_one_pot` and `per_rim_answers` build documents OF THEIR OWN
  for bodies the scene only measures — wall 1's re-planted
  torus-bellied pot, and the three per-rim fillet questions plus the
  one-request refusal. Each pays for its own frame and axis nodes,
  and each is a second authoring of the same placement.

`demos/tour/src/diefillet.rs` pays the first way too (`gallery_document`
deletes the blank), which makes this a class of at least two scenes.

What would close it is a way for a node to be evaluated and not
gathered — a flag on the node, a root set the document states rather
than derives (`DocEdit::set_roots` exists for assemblies and is not
reachable for this), or a `Node::Probe`-shaped sink the product skips.
Which of those is right is a design question for the recipe-doors
doc, not a lane's call.

Recorded from LIB-TEAPOT, where both reviewers named it independently.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

A document's product roots are its SINKS, so a body a scene only
MEASURES (the teapot's sealed hollow beside its cup, the wall-1
torus-bellied pot, the per-rim fillet questions, the die's blank)
either gets drawn by the gallery or has to live in a second document.
LIB-TEAPOT paid it three ways and both reviewers named it
independently; `diefillet` pays it too. This is a RECIPE-DOORS design
question and the code is editor-core's, so the unit is another
program's once ruled — the question is asked here because LIB's
scenes are where it bites.

- **(A) A `Node::Probe`-shaped sink the product skips.** A node kind
  that CONSUMES a value (so its input stops being a sink) and is itself
  never gathered; the scene's "measured, not modelled" intent is in
  the graph, the gallery opens the scene's own document, no root
  semantics change. One new node kind (schema bump), one arm in every
  enumerated site — the LIB-G17 shape. Recommended.
- **(B) A DECLARED root set** — generalise `DocEdit::set_roots`
  (assemblies') so a document may state its product roots and a
  derived sink not in the set is evaluated and not gathered. Reuses a
  door; changes what a root IS (declared vs derived) for every
  document, and a stale declaration is a new failure class.
- **(C) A per-node flag** (`gathered: bool`) — cheapest, and a field on
  every node for a property of a few.

Recommendation: **(A)** — additive, and it says the intent where a
reader of the recipe will find it.

### (D), added 2026-09-09 after Ev asked whether every option changes real code for a demo's benefit

It does, and the tree says the seat already exists. Roots are exactly
the DAG's sink set (`crates/editor-core/src/roots.rs`); a
`Node::Measure` reports the bodies it references as real edges
(`Node::inputs`) and "denotes NO body" (`crates/editor-core/src/node.rs`,
ERROR-DESIGN E3) — so a body measured IN the graph is already
"measured, not modelled", with no root semantics involved. The tour's
cost is self-inflicted: it measures OUT of the graph
(`pncad::topo::mass_properties` on values read off the evaluation)
and then deletes the sinks before handing the gallery its document.

The residual gap is narrower than (A)/(B)/(C): the measure lattice
carries four primitives (`Distance`, `Angle`, `MinClearance`, `Gap`;
`crates/editor-core/src/measure.rs`), so a probe whose question is a
mass property or a shell census cannot be asked in-graph. That is a
question about E3's primitive set — whether a document may assert a
mass property — and it is general, not the tour's.

**(D) No change to nodes or roots; this item closes as the demo's own
cost** (the gallery deletions stay disclosed demo tooling), and "mass
properties as a measure primitive" is filed as its own design
question where E3 lives only if Ev wants a document to be able to
assert one. Recommendation revised: **(D)**.

## Ruled (2026-09-09, Ev, `[ev]` PR 2231) — and closed

**(D).** Ev: "sounds good, no need to file that mass properties issue
i think." No change to nodes or roots. The kernel already has the
seat: a body measured IN the graph by a `Node::Measure` is not a sink
and the measure denotes no body. The tour's cost — measuring out of
graph and deleting sinks before the gallery — is the demo's own and
stays disclosed demo tooling. The mass-properties-as-a-measure-
primitive question is NOT filed. Closed as declined.
