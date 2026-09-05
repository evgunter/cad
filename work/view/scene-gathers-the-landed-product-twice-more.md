---
id: scene-gathers-the-landed-product-twice-more
kind: issue
title: scene_of_evaluation and fit_delta each gather the landed evaluation's product again after the landing already did
status: closed
opened: 2026-09-04
closed: 2026-09-05
refs: [refused-a5-gate-eats-the-body-the-fit-then-regathers]
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

## Closed

Both doors stopped gathering. `scene::fit_delta` and the door that was
`scene::scene_of_evaluation` (now `scene::scene_of_body`) take the
gathered `Body` and no longer take the pair that would produce one, so
neither can gather at all; `scene::scene_of` composes the new core
after its own `product_body`, which gives the shared core a production
caller and ends the test-only door the finding named.

The body they are handed is the LANDING's. `LandedRun` carries
`body: Option<Arc<Body<f64>>>`, filled from `land`'s one gather on
both paths where the landing still owns it — a document with no A5
gate to run never gave it away, and a certified gate hands it back on
`Assembly` — and `DocSession::landed_body` is the door. It is `&mut`
because one landing shape kept no body (a REFUSED A5 gate consumes the
product), and there it gathers once and memoizes, so no landing
gathers twice however many consumers ask. That residue is
`refused-a5-gate-eats-the-body-the-fit-then-regathers`.

Measured on this lane rather than inherited: 87 ms to gather a
165-root, 990-face document against 2.4 ms to clone the body it
produced (dev profile). The 248 ms / 8 ms figure in the finding above
is DOCM's, on their corpus, and was not re-taken.

`crates/viewer/tests/landing_gathers.rs` counts what the change is
about: asking for the landed body costs zero gathers on a part
document, zero on a certified assembly, and zero on a refused gather
(there is no product to hand out and asking does not re-run the
refusal).
