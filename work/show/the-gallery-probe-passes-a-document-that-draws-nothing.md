---
id: the-gallery-probe-passes-a-document-that-draws-nothing
kind: issue
title: r1_gallery_probe passes a gallery document whose product the gather refuses, so a file that draws nothing reads OK
status: open
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
`DocSession::landed_body` is `None`: the viewer draws nothing.

The tour's denotation table (`demos/tour/src/gallery.rs`,
`each_gallery_document_denotes_its_scene_or_says_why_not`) pins that
refusal for `chain`, so it is not silent there; the gap is that the
acceptance probe would pass the NEXT document that draws nothing with
no row saying why. The repair is a probe line per document reading
`product_fault`, failing unless the document is one the table already
explains.
