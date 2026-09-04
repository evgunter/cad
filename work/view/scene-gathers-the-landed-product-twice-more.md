---
id: scene-gathers-the-landed-product-twice-more
kind: issue
title: scene_of_evaluation and fit_delta each gather the landed evaluation's product again after the landing already did
status: open
opened: 2026-09-04
---


## What

Found by DOCM-5 (PR 1871), which made `DocSession::land` gather the
document's product ONCE and feed its three consumers (the fault, the
check registry, the A5 badge) from it. One layer up, the viewer still
gathers the SAME landed evaluation's product again:

- `crates/viewer/src/scene.rs` `scene_of_evaluation` (~:738) — a
  second gather per landed frame-build;
- `crates/viewer/src/scene.rs` `fit_delta` (~:917) — a third, to size
  the display tolerance.

(`scene.rs` `product_body` (~:717) evaluates and gathers a document for
a caller with no seam; that is its purpose and is not a defect.)

DOCM-5 measured the gather at ~30× the registry's own resident on the
heat-sink corpus document (248 ms against 8 ms, dev profile, 161
solids / 991 faces), so each extra gather per frame is the dominant
term, not a rounding error. The shape is the one `land` just removed:
a consumer deriving its own subject instead of being handed one.

## What it wants

The landing already holds the product's consequences; the scene wants
either the product itself carried from the landing (which now consumes
it in `assemble_gathered` for assembly-shaped documents — the ordering
question DOCM-5's spec item 3 settled for `land` applies here) or the
two derived facts the scene reads (the body to draw, the fit extent)
computed once at the landing and stored beside `landed_checks`. VIEW's
territory (`crates/viewer/src/scene.rs`); filed by DOCM, not built.
