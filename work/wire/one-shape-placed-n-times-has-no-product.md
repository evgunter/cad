---
id: one-shape-placed-n-times-has-no-product
kind: issue
title: one shape placed N times by transforms has no product, so the chain document opens with nothing drawn
status: open
opened: 2026-10-02
---


Filed by SHOW's `gallery-writes-every-document-scene` lane, on WIRE's
slate because the gather is WIRE's ground (`crates/editor-core/src/product.rs`,
`placed_under_two_roots`).

**The finding.** The chain study's document (`demos/tour/src/chain.rs`,
`chain`) authors ONE bar extrude and ONE pin extrude per pin position,
and places them per link through each link's joint stack of
`Node::transform` steps — the natural spelling of "four copies of one
link, each where its joints put it". At `LINKS = 4` that is nine product
roots (four placed bars, five placed pins), and the gather refuses the
document: `ProductError::PlacedUnderTwoRoots` on the bar extrude's body,
"a transform or part selection mints no name, so both would carry its
names. Recourse: place it under one root, or union the two".

The refusal is the ratified one (`work/wire/does-n3-retire-loudly-generalise-to-the-folds-other-compositions.md`,
the "one instance under two roots" row: *Refuse*). What this adds is a
consumer it costs: neither recourse fits a mechanism. Placing the bar
under one root drops three links, and a union welds four links that
turn independently into one solid — the opposite of what the study's
joint parameters mean. So the document saves (`document::save` accepts
it), opens through `viewer::docio::open` with all 35 tree rows clean,
and draws nothing: `DocSession::product_fault` carries the refusal and
`landed_body` is `None`.

**Evidence.** `demos/tour/src/gallery.rs`,
`each_gallery_document_denotes_its_scene_or_says_why_not`, the `chain`
row pins `Report::ProductRefused(ProductErrorKind::PlacedUnderTwoRoots)`.

**The question for the owner.** Whether repeated identical parts placed
by transforms want a door that mints per-placement names (so the gather
can hold N copies of one body), or whether the answer is that such a
document must be an assembly of `InstantiatePart`s — in which case the
refusal's recourse should say so, and the chain scene is re-authored
on that door.
