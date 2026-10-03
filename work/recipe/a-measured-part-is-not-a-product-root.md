---
id: a-measured-part-is-not-a-product-root
kind: issue
title: A Measure reading a part's faces consumes it, so a document that asserts a requirement over its own part has no product
status: open
opened: 2026-10-03
priority: P2
cost: M
design: true
needs_ev: true
---



Found by SHOW's `the-plate-document-never-cuts-its-holes`, re-authoring
the tour's two-hole plate in its natural spelling.

## What

`demos/tour/src/plate.rs` `cut_plate` cuts both holes from the blank
and reads the web `Measure` off the CUT PART's bore walls — the part
the requirement is about. `Node::inputs` makes each reference's `at` a
DAG edge (`crates/editor-core/src/node.rs`, the `Node::Measure` arm),
so the part is no longer a sink, and by A10 the root set IS the sink
set (`crates/editor-core/src/roots.rs`): the document's one root is the
`Assertion`, which denotes no body. `product` refuses `NoBodyRoots`;
the viewer, checks, mass properties and export have nothing to read.
`DocEdit::SetRoots` cannot add the part back: ancestor-freedom forbids
a root that is an ancestor of another.

Pinned live: `demos/tour/src/gallery.rs`
`the_cut_plate_has_no_product` (wall 2, `NoBodyRoots`; the roots are
exactly `[assertion]`).

## Why it is a design question

The 2026-09-09 ruling on
`work/lib/a-recipe-cannot-hold-a-narration-body-without-it-becoming-a-root.md`
(Ev, `[ev]` PR 2231, option D) rests on "a body measured IN the graph
by a `Node::Measure` is not a sink and the measure denotes no body" —
measured means NOT modelled. That holds for a narration body and
fails for the commonest requirement there is: a tolerance or clearance
assertion over the product itself. The uncut plate only escapes it by
measuring the hole extrudes rather than the part, which is why its
gallery document is a blank slab
(`work/show/the-plate-document-never-cuts-its-holes.md`).

Shapes a fix could take (not weighed here): a measure reference as an
A12-style READING edge (evaluation order kept, root set untouched), or
a product rule that gathers a body-denoting ancestor of a non-body
root.

## A reference's type says whether it consumes or reads

Ev took typing the edges (PR 3929: "1 is a good idea!") and asked whether the distinction could be baked into the structure more, rather than two somewhat different things with the same form. It can: the tree already has the two forms, and uses them loosely. Today a measure's references and a face frame's `at` are reads written as operands (they drop the body from the roots); a face frame is a sited reference spelled as two loose fields; a gauge reference is a bare id that reads; mates' heads are sited reads kept outside the edge list. Measured on main: a face frame read off a box drops the box from the roots, a measure over a part drops the part, and a measure over two unmated instances fuses A9's partition from 2 components to 1.

- **Two forms, no tag.** A bare `RecipeNodeId` in a payload is an operand: consumed whole. A sited reference (`{ at, name }`) is a read. `FaceFrame` takes a sited reference for its face; a gauge reference becomes a site; measure references and mate heads keep their shape. `Node::operands()` and `Node::sites()` are exhaustive over field types, as `payload_names` is, so a new node kind cannot ship a misclassified reference.
- **Roots.** A10 runs over operands. Reads are node-to-entity, not edges of the node graph, so "the root set is the sink set" holds as written, and the measured part and the sketched-on body stay roots.
- **Evaluation.** Reads order evaluation, key the memo and carry poison, except where the reader's value is the solve's answer (a mate, a gauge, an instance): feeding their sites into keys and poison turns the solve's own typed fault into `Poisoned { through: operand }` (measured by both designers: 13 of 33 and 22 of 2297 rows).
- **A9.** The partition runs over operands and reads but does not pass through a reader whose value lives in no space (a measure, an assertion), as `mate::spaces_with` already says; a face frame's read still couples, because a body sketched on a face moves with it (measured).
- **Rebind.** One rule for every sited reference, the mate head's: an `at` read at its name's own mint follows the name when the name is rebound; an `at` the author placed elsewhere (a downstream transform, for placed geometry) never moves. The measure's "never moves `at`" was that rule without its first half: after a re-mint, an at-mint site that stays reads a name its table never held.
- **Delete.** Deleting an operand is refused (a node missing an operand is not a node). Deleting a read's site strands the read and the edit reports it (DM7, #3437): the reader fails typed `NodeGone`, and one `Rebind` repairs it in place in the at-mint case; an authored site is re-authored, as a mate's already is. Deleting a block a sketch was drawn on keeps the sketch, failing until it is rebound.

Text this moves: A10 (in this PR), A12 (every read is a site; sites never decide roots), A11 (2)'s "never refused" to every read, A9 (not through space-free readers), REFERENCES §0 and DM1 (a bare id consumes, a sited reference reads; `FaceFrame`'s signature), and the `Node::Measure` and `SitedRef` docs (agent-written, M10-2 and MSOLVE-1).

Weighed by two pairs of designers over four rounds (fork-log row 57); the reports are in the PR.
