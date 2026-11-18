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
