---
id: exch
kind: program
title: EXCH — exchange: STEP and STL
status: ready
opened: 2026-09-03
area: kernel
prefix: exch/
tag: (EXCH orchestrator)
ab_band: 2100-2199
paths: [crates/step-import/*, crates/step-export/*, crates/stl/*, docs/STEP-BANK.md, scripts/step_import_check.py]
keep_out: [code-quality Track U's STEP and STL rows are claimed — D343 moved into this directory, C13 and C14 closed in work/code-quality before the move; its pncad and pncad-py rows (D341) are LIB's and stay in work/code-quality until LIB claims them, topo/src/pcurves.rs is TRIM's — the ExtrudedPoint rung of nurbs_iso_derive is filed as a TRIM row and EXCH consumes it, geom-core/src/spline/compose* is PROPS' glob since S-CERT's exit (Track N claimed whole by PROPS, DOC-LEDGER sweeps 7 and 11) — the derivative channel and the tensor hull are filed as PROPS rows when their units are cut, geom-brep surface recognition (the M7-6 lane) is PROPS' (Track R claimed whole) — announce, the pncad-py Python side of any option-surface change is LIB's unit, tcost and tint hold crates/*/tests/* — their test-cost and test-integrity rows on the STEP crates' tests stay theirs and EXCH's unit edits to those tests ride unit PRs, dm1's arc-rim MapResidual frontier is TRIM's on both files — EXCH consumes work/issues/arc-rim-mint-frontier-on-foreign-charts and does not build it]
priority: P3
---

**STEP import's recognition and certification lanes, after the
2026-09-20 cut**: what the importer can recognise, what it promotes,
and what it certifies about the promotion.

The charter's core is unchanged — certified curve recognition on
import (open arcs, ellipse, helix; degree-1 promotion) and the
algebraic cylinder-recognition certificate. What the cut adds is that
the recognition chain has a measured hole at each rung: `recognize.rs`
normalizes a plane normal with no length decision and cannot mint the
witness; the circle limb certifies locus and closure but not the MAP,
so a re-timed parameterisation passes; and recognition promotes at
`eps_in` while selection and certify run at ambient, which is two dials
on one strand.

`torus-rim-mint-abandons-a-half-applied-split` is the P0 row: it bails
with `Ok(())` after `split_at_midpoint` has already run.

EXCH was cut on 2026-09-20 (Ev, in chat) from 44.5 budget points into
EXPORT (the writing direction and the shared refusal plumbing) and this
remainder. EXCH keeps its band 2100-2199.

Charter and unit order: `work/exch/plan.md`; narrative in `work/exch/log.md`.
