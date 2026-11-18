---
id: a-documents-negative-depth-recourse-names-a-side-it-cannot-set
kind: issue
title: A document extrude's negative-depth refusal tells the author to set a side, and since D10 a recipe extrude has no side to set
status: closed
opened: 2026-11-01
closed: 2026-11-18
---


`sweep::ExtrudeError::NegativeDepth`'s sentence (`crates/sweep/src/extrude.rs`, its `Display` arm) ends *"Recourse: make the depth evaluate positive … and set the side to {}"*, naming `side.flipped().noun()`. That is right for the kernel door, whose `Extrusion::Distance` carries a `side`.

It is not right for a document. INTENT stage 3 D retired the recipe's side (`docs/INTENT-STAGE3-SPEC.md`, unit D: *"`ExtrudeSide` goes: which side of a face the material lies on is the placement's mates' to say"*; DESIGN.md `[placement]`: *"which side of the face its material lies on is the mates' to say"*). `Node::Extrude` is `{ profile, distance }`, `DocEdit::SetExtrudeSide` is gone, and `wire_extrude` (`crates/editor-core/src/eval/wire.rs`) always passes `ExtrudeSide::Along`. So every document extrude with a negative depth refuses with a recourse naming *"against the sketch normal"*, a side no door can set.

`crates/editor-core/tests/m4_pr2_eval.rs`, `a_parameter_driven_negative_depth_refuses_with_a_recourse_that_builds`, pins exactly that sentence on a document (`"set the side to against the sketch normal"`). The recourse that test then applies is only the positive depth, so the test never applies the side half of the recourse.

**Owed:** a document's negative-depth refusal names a recourse the document can take: a depth that evaluates positive, and, if the material should lie the other way, the copy's mate flipped (its sense is `Flip` on one side, DESIGN.md `[placement.mates]`). Where the sentence is restated (in the sweep door with the side made optional, or at the recipe's wiring) is this row's choice. The test above should pin the new words.

**Seam (DOORS):** `work/doors/the-create-pane-has-no-extrude-side.md` was specified against the retired recipe side, including Ev's requirement on #3551 that the form offer the extrude written the other way in one click. What that offer becomes under D10, if anything, follows from the recourse this row settles.

## Closed

Moot for a document since DOCTAIL's signed depth (2026-11-18, `work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face.md`). `Node::Extrude`'s depth is signed: `wire_extrude` (`crates/editor-core/src/eval/wire.rs`) answers the sweep's own `NegativeDepth` verdict by sweeping `|d|` and lifting the body `d` along the profile's `z`, so a document's negative depth builds a pocket's tool and never shows the sentence. The extrude written the other way is its depth written negative. The `m4_pr2_eval` row is now `a_parameter_driven_depth_builds_on_the_side_its_sign_says`, and it pins the two bodies rather than the sentence.

Nothing remains. The sentence's side is right where it can still be read, at the kernel door, whose `Extrusion::Distance` carries a side, as this row says. `refusal_concision_chains` builds the error directly, and pncad-py's `negative_depth` tag names the arm the sweep still has.
