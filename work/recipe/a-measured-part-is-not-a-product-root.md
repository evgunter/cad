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
