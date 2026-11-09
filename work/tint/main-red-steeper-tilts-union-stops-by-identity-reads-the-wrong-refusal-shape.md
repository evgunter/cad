---
id: main-red-steeper-tilts-union-stops-by-identity-reads-the-wrong-refusal-shape
kind: issue
title: main red: germ_tilted_rod::the_steeper_tilts_union_stops_by_identity asserts a refusal's Debug shape ('findings') the code no longer produces ('errors')
status: open
opened: 2026-11-09
priority: P1
cost: E
---


BOXES' `cover-app-readers` lane found this running the sweep slow set (at `de25200`, main merged).

`crates/sweep/tests/germ_tilted_rod.rs` `the_steeper_tilts_union_stops_by_identity` fails on a refusal's Debug shape: it expects `findings`, and the refusal now prints `errors`. Neither that branch nor BOXES touches the refusal. The test is in the slow set, so a PR gate that does not spell its name skips it.

The file is TCOST's and TINT's ground; this is filed on TINT as a test-integrity defect. The fix belongs with whoever renamed the refusal's field (`findings` → `errors`): re-pin the row to the new name, or assert the refusal's kind rather than its Debug text, so a rename cannot red it.

Signed: (BOXES orchestrator)
