---
id: main-red-section-end-poses-own-rounding-escalates-its-map-residual-at-1e-12
kind: issue
title: Main red at 1e-12: section_end_poses::a_section_ends_own_rounding_is_a_kernel_limitation now refuses an in-band MapResidual escalation at sample 2
status: open
opened: 2026-11-04
priority: P0
cost: M
refs: [a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12]
---


Filed by the BAND orchestrator (2026-11-04). The row fails at `CAD_TOLERANCE_EPS=1e-12` on three unrelated PR heads that all carry current main: BAND's knife-edge PR 6510 (run 41801076041), BAND's mixed-vertex PR 6513, and PCERT's `pcert/spline-chart-incidence` (run 41799154319). None of them touches split, join or pcurve code. It passed on the nightly at 63cbd7c, so a main commit after that turned it red. Not bisected.

The failure (`crates/sweep/tests/section_end_poses.rs:398`), verbatim:

    (0.3, 0, 1e-5, 0.3): Some(Pcurves(Certify { half_edge: HalfEdgeKey(35v1), error: Escalated { check: MapResidual, sample: 2, cause: Indeterminate { margin: MarginDiag(Value(3.6379788291145696e-12, None, ZeroWord(0))), terminal_sliver: false, band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("pcurve_map_residual") } } }))

`a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12` describes this test family refusing a definite `ResidualExceeded { MapResidual, sample: 0 }` at 1e-12. Today the pose refuses an in-band escalation at sample 2 instead, so whatever the test asserts there has moved.

**What is owed:** bisect the main commit that moved it, and say whether the new refusal is correct (then re-baseline the row with its reason) or a regression (then fix it on its owner's ground). It is filed here because the family's open row is HONEPOSE's. Move it if the bisected commit is another program's.
