---
id: the-plate-document-never-cuts-its-holes
kind: issue
title: the two-hole plate's document never cuts its holes, so the gallery's plate opens as a blank slab
status: parked
blocked_on: [a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound, a-measured-part-is-not-a-product-root]
priority: P3
cost: M
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
`each_gallery_document_denotes_its_scene_or_says_why_not` (`plate`: 2
roots, a product of 1 solid and 6 faces, so cutting the holes turns
the row red).

**Unmeasured.** Whether the natural spelling — a `Boolean(Difference)`
of the two holes from the blank, with the measure reading the bore walls
of the result — survives the study's three lanes (the certified box
drive in `tolerance.rs`, the advisory stackup, the Monte Carlo replay in
`mcplate.rs`). If one refuses, that refusal is the library finding and
goes on its owner's slate; if none does, the plate is re-authored and
the tolerance and MC cells move.

## Measured (PR 3922): both natural spellings stop short of the study

| spelling | certified drive | advisory stackup | Monte Carlo | gallery product |
|---|---|---|---|---|
| uncut (shipped): web read off the hole extrudes | 193 of 512 leaves | answers | answers | the blank slab |
| **cut**: blank ∖ hole_a ∖ hole_b, flush caps declared (`plate::cut_plate`) | no box certifies, not even 1e-9 of the study whole (the subtract's volume-bound tie, `work/reach/a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound.md`) | `NothingCertified` | answers, the study's web bit for bit | `NoBodyRoots` (`work/recipe/a-measured-part-is-not-a-product-root.md`) |
| **sketched**: one extrude, two inner-loop circles | whole boxes to 1e-2 of the study; 0 of 512 over the real study (`work/paths/inner-loop-circles-bound-the-plate-study-at-arc-span.md`) | refuses with it | answers | `NoBodyRoots` (inferred: the measure reads the extrude) |

The cut is pinned live as two walls: `tolerance::cut_wall` (wall 1, in
`demo-tour certified`) and `gallery::tests::the_cut_plate_has_no_product`
(wall 2). An overshooting tool is not this part's spelling; measured as
evidence, it refuses the 1e-9 box on a second frontier
(`work/chart/an-overshooting-subtract-flips-chart-bound-outer-span-over-a-tiny-box.md`).
The trigger is both blockers closing: the cut then certifies and has a
product, and the plate is re-authored as the cut.

