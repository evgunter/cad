---
id: gui-gauge-assembly-shows-three-check-findings
kind: issue
title: the viewer opens the corpus gauge assembly (04c71837…) with 'checks: 3 finding(s)' in red, which the 2026-10-07 render did not show; nobody has read what the three are
status: open
opened: 2026-10-15
priority: P2
cost: E
---


## What

render.yml run 38816108307, lane `viewer gui montage`, re-rendered on PR 5205 (`demos/renders-gui/montage-gui.png`, the gauge cell: a desk with a box on it, the document `04c718374cb49ae558c3d9f6b5fce9f6`). The status bar now reads `at rest: certified (3 declaration(s))` followed by **`checks: 3 finding(s)`** in red. The montage committed from the 2026-10-07 render shows the same document with no checks readout. The geometry is identical between the two renders. The feature list gained `PlaceInWorld` nodes (12 nodes, was 8), so the findings arrived with the placement nodes from INTENT's stage work.

Not read: which checks fire, at what severity, and whether they are right (a placement the corpus document should declare and does not) or wrong (a check misreading `PlaceInWorld`). The tessellation-budget sweep's assembly scene logs `enforce_checks refuses: 1 check finding(s) at Error severity` on the same corpus family (nightly 61, `k-lint (release-budget)`). That may or may not be the same cause.

## Owed

Open the document headless (`run_checks` over it) and name the three findings. Then fix the document or the check, whichever is wrong. If a finding is intended, say so in the corpus so the GUI's red is expected.

— (CIW orchestrator, from the gui montage re-baseline)
