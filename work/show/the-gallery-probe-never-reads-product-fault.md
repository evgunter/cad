---
id: the-gallery-probe-never-reads-product-fault
kind: issue
title: r1_gallery_probe never reads product_fault, so a gallery document whose product the gather refuses passes
status: open
priority: P3
cost: E
opened: 2026-10-02
---

Found by `gallery-writes-every-document-scene`.

**The finding.** `crates/viewer/examples/r1_gallery_probe.rs`, the
render lane's "the real gallery opens in the viewer" verdict
(`.github/workflows/render.yml`), checks per document that the open
succeeds, every tree row evaluates `Ok`, and per instance that hide and
a probe take visible effect. It never asks whether the document's
PRODUCT gathered. `chain.pncad`
(`work/wire/one-shape-placed-n-times-has-no-product.md`) opens with 35
rows, 0 not-ok, and the probe prints `gallery probe: OK`, while
`DocSession::product_fault` holds `PlacedUnderTwoRoots` and
`DocSession::landed_body` is `None`. The canvas still draws each root's
own body (`PickIndex::assemble` tessellates per root and never reads the
gather), under the product-fault badge, so the document looks loaded;
what it lacks is the product that checks, mass properties and export
read.

The tour's denotation table (`demos/tour/src/gallery.rs`,
`each_gallery_document_denotes_its_scene_or_says_why_not`) pins that
refusal for `chain`, so it is not silent there; the gap is that the
acceptance probe would pass the NEXT document whose product is refused
with no row saying why. The repair is a probe line per document reading
`product_fault`, failing unless the document is one the table already
explains.
