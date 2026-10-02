---
id: the-plate-document-never-cuts-its-holes
kind: issue
title: the two-hole plate's document never cuts its holes, so the gallery's plate opens as a blank slab
status: open
opened: 2026-10-02
---


Found by `gallery-writes-every-document-scene`, writing `plate.pncad`.

**The finding.** The two-hole plate's document (`demos/tour/src/plate.rs`,
`plate`) extrudes the blank and extrudes two hole cylinders, and never
subtracts them: the hole extrudes exist only as the web `Measure`'s
references. So the document's product roots are the blank's extrude and
the web assertion, and its product is a six-face slab with no holes —
what `viewer::docio::open` + `DocSession` draws (6 faces, 12 triangles),
and what `demo-tour gallery` writes as `plate.pncad`. The document
carries the study's numbers but does not denote the part the study is
about. Pinned in `demos/tour/src/gallery.rs`,
`each_gallery_document_denotes_its_scene_or_says_why_not` (`plate`, 2
roots).

**Unmeasured.** Whether the natural spelling — a `Boolean(Difference)`
of the two holes from the blank, with the measure reading the bore walls
of the result — survives the study's three lanes (the certified box
drive in `tolerance.rs`, the advisory stackup, the Monte Carlo replay in
`mcplate.rs`). If one refuses, that refusal is the library finding and
goes on its owner's slate; if none does, the plate is re-authored and
the tolerance and MC cells move.
