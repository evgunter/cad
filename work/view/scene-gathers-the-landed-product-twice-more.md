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
gathers the SAME landed evaluation's product again — corrected after
DOCM-5's review measured the size of the claim:

- `crates/viewer/src/scene.rs` `fit_delta` (~:917) gathers it to size
  the display tolerance, behind `app.rs:~616`'s `fit_delta_on_scene`
  latch — once per OPENED document, not per landing;
- `crates/viewer/src/scene.rs` `scene_of_evaluation` (~:738) gathers
  it too, but has no production caller in-tree (only
  `viewer/tests/display_budget.rs` and `viewer/tests/doc_io.rs` call
  it) — a test-only door that would pay per frame if it were wired.

(`scene.rs` `product_body` (~:717) evaluates and gathers a document for
a caller with no seam; that is its purpose and is not a defect.)

DOCM-5 measured the gather at ~30× the registry's own resident on the
heat-sink corpus document (248 ms against 8 ms, dev profile, 161
solids / 991 faces), so the one live extra gather (per opened document)
is a whole gather's worth, and the test-only door would be the
dominant per-frame term if anything wired it. The shape is the one `land` just removed:
a consumer deriving its own subject instead of being handed one.

## What it wants

The landing already holds the product's consequences; the scene wants
either the product itself carried from the landing (which now consumes
it in `assemble_gathered` for assembly-shaped documents — the ordering
question DOCM-5's spec item 3 settled for `land` applies here) or the
two derived facts the scene reads (the body to draw, the fit extent)
computed once at the landing and stored beside `landed_checks`. VIEW's
territory (`crates/viewer/src/scene.rs`); filed by DOCM, not built.
