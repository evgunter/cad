---
id: one-shape-placed-n-times-has-no-product
kind: issue
title: one shape placed N times by transforms has no product, so the chain document draws its links but checks, mass properties and export have nothing to read
status: open
priority: P1
cost: M
design: true
opened: 2026-10-02
refs: [the-solve-accepts-a-body-placed-under-two-roots, does-n3-retire-loudly-generalise-to-the-folds-other-compositions]
---

Filed by SHOW's `gallery-writes-every-document-scene` lane, on WIRE's
slate because the gather is WIRE's ground (`crates/editor-core/src/product.rs`,
`placed_under_two_roots`).

**The finding.** The chain study's document (`demos/tour/src/chain.rs`,
`chain`) authors ONE bar extrude and ONE pin extrude per pin position,
and places them per link through each link's joint stack of
`Node::transform` steps. That is the natural spelling of "four copies of
one link, each where its joints put it". At `LINKS = 4` that is nine
product roots (four placed bars, five placed pins), and the gather
refuses the document: `ProductError::PlacedUnderTwoRoots` on the bar
extrude's body, "a transform or part selection mints no name, so both
would carry its names. Recourse: place it under one root, or union the
two".

The refusal is the ratified one (`work/wire/does-n3-retire-loudly-generalise-to-the-folds-other-compositions.md`,
the "one instance under two roots" row: *Refuse*). What this adds is a
consumer it costs: neither recourse fits a mechanism. Placing the bar
under one root drops three links, and a union welds four links that
turn independently into one solid, which is the opposite of what the
study's joint parameters mean.

**What a user sees.** The document saves (`document::save` accepts it)
and opens through `viewer::docio::open` with all 35 tree rows clean. The
canvas draws the nine placed bodies (40 faces, 160 triangles), because
`PickIndex::assemble` (`crates/viewer/src/pickindex.rs`) tessellates each
root's own body and never reads the gather. `badge_site` routes the
fault to the product badge. So the links are on screen under a
product-fault badge, while `DocSession::product_fault` carries the
refusal and `landed_body` is `None`. What is missing is the product,
which the check registry, mass properties and export read.

**The same refusal from the solve side.**
`work/msolve/the-solve-accepts-a-body-placed-under-two-roots.md` records
that the mate solve accepts a document with one instance mated through
two transform roots while the gather refuses it. That item is about the
two layers disagreeing. This one is about a document no mate is
involved in, where the gather's refusal is the only verdict and its
recourse does not fit. Whatever ruling settles one should be read
against the other.

**Evidence.** `demos/tour/src/gallery.rs`,
`each_gallery_document_denotes_its_scene_or_says_why_not`, the `chain`
row pins `Report::ProductRefused(ProductErrorKind::PlacedUnderTwoRoots)`.

**The question for the owner.** Do repeated identical parts placed by
transforms want a door that mints per-placement names, so the gather can
hold N copies of one body? Or must such a document be an assembly of
`InstantiatePart`s? If the latter, the refusal's recourse should say so,
and the chain scene is re-authored on that door.
