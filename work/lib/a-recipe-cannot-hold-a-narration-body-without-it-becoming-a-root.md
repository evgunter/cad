---
id: a-recipe-cannot-hold-a-narration-body-without-it-becoming-a-root
kind: issue
title: a recipe cannot hold a narration or probe body without it becoming a product root
status: open
opened: 2026-09-08
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
