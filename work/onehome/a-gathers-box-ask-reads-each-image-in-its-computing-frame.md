---
id: a-gathers-box-ask-reads-each-image-in-its-computing-frame
kind: issue
title: A gather asks the session box of each image read into its computing frame, and refuses a valid document
status: open
opened: 2026-11-18
---


A gather (`⊔`, the product gather, a distribute) builds one body out of
its operands, each read into the gather's computing frame whole
(`Framed::read_in`, `crates/editor-core/src/eval/framed.rs:470`). That read
is `Held::moved` through the relative motion, and the move asks the session
box of the image *in the computing frame*. It does not ask where the image
stands in the world.

The computing frame is the first operand's where conditioning does not
decide, so the box is asked about the wrong origin. PR 7088's delta-3
review reports the case: two copies of a block, one at +900 m and one at
−900 m, under `assemble_world_copies`. Each copy stands inside the 1 km box
in the world, and each placement admits it. The `⊔` reads the second
copy 1800 m off the first copy's origin and refuses with margin −800. A
valid document is refused by a frame artefact.

It is the gathers' path, so it is #7099's (`onehome/one-aggregate-door`).
There, a gather asks each operand of its world placement, through the one
home `Framed::asked_where_it_stands` (on the composed motion, or the
result's world reach), and not of its image relative to the computing
frame. A row there pins the ±900 m document building, and a pair past the
box still refusing at the placements.

## The twist has the same shape (delta-4 review of PR 7088)

Under `EvalOptions::computing_twist` only, `twisted` (`crates/editor-core/src/eval/wire.rs:646`) moves a construction's body by its node's twist. The move (`moved(&twist, tol)`, `wire.rs:661`) asks the session box of the image in twisted coordinates. It then gives the value the twist's inverse as its frame (`wire.rs:664`), so the body stands where it did. The box is asked of where the twist put the body, not of where it stands in its space. That is the gathers' frame-artefact shape: a body inside the box can refuse under a twist that carries it out, and the twist is a test lever that should change no verdict. The fix belongs with the gathers' in #7099: ask where the value stands through `asked_where_it_stands`, not the image in the computing frame.
