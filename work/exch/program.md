---
id: exch
kind: program
title: EXCH — exchange: STEP and STL
status: open
opened: 2026-09-03
area: kernel
prefix: exch/
tag: (EXCH orchestrator)
ab_band: 2100-2199
paths: [crates/step-import/*, crates/step-export/*, crates/stl/*, docs/STEP-BANK.md, scripts/step_import_check.py]
keep_out: [code-quality Track U's STEP and STL rows are claimed — D343 moved into this directory, C13 and C14 closed in work/code-quality before the move; its pncad and pncad-py rows (D341) are LIB's and stay in work/code-quality until LIB claims them, topo/src/pcurves.rs is TRIM's — the ExtrudedPoint rung of nurbs_iso_derive is filed as a TRIM row and EXCH consumes it, geom-core/src/spline/compose* is PROPS' glob since S-CERT's exit (Track N claimed whole by PROPS, DOC-LEDGER sweeps 7 and 11) — the derivative channel and the tensor hull are filed as PROPS rows when their units are cut, geom-brep surface recognition (the M7-6 lane) is PROPS' (Track R claimed whole) — announce, the pncad-py Python side of any option-surface change is LIB's unit, tcost and tint hold crates/*/tests/* — their test-cost and test-integrity rows on the STEP crates' tests stay theirs and EXCH's unit edits to those tests ride unit PRs, dm1's arc-rim MapResidual frontier is TRIM's on both files — EXCH consumes work/issues/arc-rim-mint-frontier-on-foreign-charts and does not build it]
priority: P3
---

The I/O crates, which have had no program since M7 closed on
2026-08-09: certified curve recognition on import (open arcs, ellipse,
helix; degree-1 promotion), an algebraic cylinder-recognition
certificate, and the caller-facing option surface (the ε type, STEP
header fields, STL header names) LIB held as "plan to Ev first" and
never drafted. Class H core with three D→E option items. Claims
code-quality Track U's STEP/STL rows. Charter and unit order:
`work/exch/plan.md`; narrative in `work/exch/log.md`.
