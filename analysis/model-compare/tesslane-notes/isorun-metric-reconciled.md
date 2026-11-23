(iso-run-density implementer) Metric table reconciled. The correction is held locally as d6c35cefd7, on top of 638e764c03. Nothing is pushed.

The reviewer is right: "180 of 180" mixed two outcomes. My probe had 900 cells:
- 5 (r, δ) groups of 180 each: r 1 at δ/r 1e-1, 1e-2 and 1e-3; r 1e-3 and r 10 at δ/r 1e-2.
- Each group is 3 heights × 15 gaps (1.0 to 3.0 columns) × 4 phases.

I re-read the saved runs, so no re-run was needed. Boundaries do not depend on the metric, so extra inserted points per cell = ΔT/2 against the present metric.

Present metric: all 900 mesh, with 0 or 4 points inserted. 945 faces at attempt 0 and 8 at attempt 1.

Alternative k = 1/r (development; "iso"):
- δ 1e-3, r 1: 180 of 180 refuse CertificateExceeded with the budget spent, bound 2.7·δ to 27.6·δ.
- δ/r 1e-2, 540 cells (r 1, 1e-3, 10): all mesh, with 47 to 144 more points (mean 96).
- δ/r 1e-1, r 1, 180 cells: all mesh, with 0 to 27 more (mean 6.5).

Alternative = the old rows' metric ("main"):
- δ 1e-3: the same 180 refuse, same bounds.
- δ/r 1e-2: all mesh, with 47 to 144 more at r 1 and r 1e-3, and 63 to 177 more at r 10 (mean 114).
- δ/r 1e-1: all mesh, with 0 to 33 more (mean 12.6).

Rounds under both alternatives: most corrected faces take 3 to 6 attempts. The rounds log was written by tests running in parallel, so lines are interleaved and I give a range, not a per-cell figure. Mesh and sweep were 16 red rows each, unchanged.

This agrees with the reviewer's "60 to 139" at δ 1e-2.

The `trimmed_metric` doc now says this. The PR body line could read:
"Measured against the two alternatives, which are one alternative (the development k = 1/r, and the metric the lane read when it laid rows): **both lose**. Over 900 two-hole cells, which the present plane meshes with 0 or 4 points inserted, the 180 at δ 1e-3 refuse `CertificateExceeded` with the correction's budget spent, the 540 at δ/r 1e-2 mesh with 47 to 177 points more, and the 180 at δ/r 1e-1 with 0 to 33 more; 16 `mesh` and 16 `sweep` rows are red."

No other "180 of 180" remains in the repo.

Checks: fmt, mesh clippy and run-gates pass. Holding.
