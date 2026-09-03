---
id: quad-face-extent-trusts-caller-perimeter
kind: issue
title: props_quad_face_extent trusts a caller-supplied perimeter that is sometimes not an upper bound - 10 of 333 corpus returns violate it
status: open
opened: 2026-08-31
github: 1368
refs: [1366]
---

## From GitHub issue 1368

opened 2026-08-31, 0 comments.

(S-CERT orchestrator) Filed from CERT-6's calibration corpus (PR 1366), which minted the instrument that makes this checkable.

`props_quad_face_extent` reads `Margin::over_lever(area.lo(), perimeter)` with a **caller-supplied** perimeter that must over-estimate for the lever to be conservative. CERT-6's `boundary_chord_perimeter_lo` is a certified *lower* bound in the same lane, so `perimeter_lo > perimeter` proves the caller's number wrong. **Measured: 10 of 333 certified returns violate it, all in `geom-brep` test probes** — ratios 1.02, 1.09, 1.31, 1.67, **3.66** (a violently-warped chart probe passing the UV rectangle's perimeter 4.0 where the metric image is ≥ 14.65), and **two probes that pass `perimeter = 0.0` outright** and still certify extent. **Zero violations from the production caller in `topo/src/props.rs` across the whole corpus** — so this is probe hygiene plus a missing guard, not a live soundness bug.

The fix shape: a `debug_assert!(perimeter_lo <= perimeter)` at the gate would have teeth immediately — which is exactly why CERT-6 did not ship it: it reds ten existing rows and each needs its own disposition. Whoever takes it inherits that row-by-row pass. S-CERT-adjacent ground (`props/quad.rs`); CERT-10 edits nearby.

Second, smaller finding from the same corpus, recorded here rather than as its own issue: **`nurbs_patch_face`'s composite arm has zero certified-return coverage in the corpus** (all 290 integral-lane returns took the exact per-span arm) outside CERT-5's dedicated probes — harmless for the gauge (the arms share one area enclosure) but the composite flux arm's own convergence behaviour deserves a certified-exit row.

## Home

S-CERT: the gate is `props_quad_face_extent` in `crates/geom-brep/src/props/*`, S-CERT territory, and the issue names its own ground as S-CERT-adjacent with CERT-10 editing nearby.
