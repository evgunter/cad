---
id: lever-arm-fold-six-hand-rolled-siblings
kind: issue
title: The two-surface lever-arm fold has one documented home and six hand-rolled siblings; contact_tangent_opposed is classify_material_pairing's undisclosed twin
status: open
opened: 2026-09-01
github: 1439
refs: [1423]
---

## From GitHub issue 1439

opened 2026-09-01, 0 comments.

Filed from the MATE-3 dual review (PR #1423; found bilaterally — one arm from the prose, one from the constants/data sweep). MATE-3 introduced `geom_brep::folded_lever_arm` documented as "one home for the fold" (three consumers named, margins comparable only if levered against the same arm) — but the identical three-way `min` fold survives hand-rolled at six sites: `topo/boolean/contact_verify.rs:351-353` (the fold's own stated origin), `geom-brep/certify.rs:1685-1687` and `geom-brep/ssi.rs:894-896` (the helper's OWN crate), `topo/boolean/ops.rs:1020-1022`, `sweep/revolve/upgrade.rs:233-235`, `sweep/extrude.rs:882-884`. Two of eight sites use the home.

Deeper half: `contact_tangent_opposed` (`contact_verify.rs:349-360`) is arithmetic-for-arithmetic the new `classify_material_pairing` (`normalize(∇F)·sense`, dot, `Margin::levered`) under a second predicate name — the dependency direction admits calling the shared home.

All six sites were legitimately outside MATE-3's fence; scheduling this here rather than half-fixing one crate in passing. The consolidation is mechanical but crosses four crates and re-homes a predicate name, so it wants its own small unit (any program touching those lanes may take it; S-MATE has no claim on the boolean/sweep sites).

Signed: (S-MATE orchestrator)

## Home

`work/code-quality/` — a duplicated-spelling structural finding (one documented home, six hand-rolled twins) crossing four crates, which the issue explicitly declines to route to any one program.
