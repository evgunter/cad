---
id: step-import-degree-one-line-promotion
kind: issue
title: step-import — promote degree-1 NURBS carriers to Curve3::Line, needs an ExtrudedPoint rung in the NURBS-chart pcurve lane first
status: open
opened: 2026-08-11
github: 388
refs: [327, 389, 390, 391]
---

## From GitHub issue 388

opened 2026-08-11, 1 comment.

Split out of #327 (stage-1 CURVE recognition) by its binding scope rule, with the measured reason.

**The recognition half is done and costs nothing.** A line certificate is available today from the same substrate #327 uses: `geom_core::spline::compose`'s zero-radius `ImplicitSurface::Cylinder` composite is exactly `dist(P, line)²` over the whole domain, so `√sup` is a certified metre residual with no sampling (INV-C2 in the retired draft), and the SEGMENT (rather than infinite-line) obligation follows from convexity: the projection onto the chord direction is affine, so the control values bound it, and a control-point excursion `o` outside `[0, ℓ]` gives `residual ≤ hypot(δ_line, o)` (INV-C4). All 37 of dm1's polyline carriers certify.

**What it costs is downstream, measured on the #327 branch.** A promoted `Curve3::Line` changes the edge's adopted description from `EdgeGeometry::IsoCurve` — whose exact `Pcurve::IsoLine` chart image `topo::pcurves::nurbs_iso_derive` already mints — to `EdgeGeometry::MappedCurve(MappedCurve::ExtrudedPoint { .. })`, which is the arm `adopt::mapped_self_description` takes for a line on a non-`nurbs_rim` edge. `nurbs_iso_derive` has NO derivation for `ExtrudedPoint` at all, so the mint refuses `IsoUnsupported` and dm1 refuses **strictly earlier than it does today** (`HalfEdgeKey(1v1)`, before it ever reaches the rim). That is a pcurve-lane rung, not a recognition question.

**What promotion would flip once the rung lands**: `PlacedSegment`/`ExtrudedPoint` self-descriptions for dm1's 37 polyline carriers, the non-rational pcurve re-mint for them (which re-certifies each rim against its wall rather than leaving it conventional), and exact `LINE(...)` re-export in place of `B_SPLINE_CURVE_WITH_KNOTS`.

Prerequisite: an `ExtrudedPoint` (and `PlacedSegment`-over-`Curve3::Line`) arm in `nurbs_iso_derive`, on top of the imported-chart generalizations #327 landed (chart-own domain, u-direction pick, `chart_u_period`).

## Comments

**2026-08-11** — orchestrator:

Measured addendum from #327/#391, and a second reason to want this rung: retiring edge #685 made dm1's edge **#389** reachable for the first time at any band, and at ambient ε = 1e-6 the D7 ladder offers it **ZERO candidates** — `step import: edge #389: no intensional description certifies —` with an empty attempt list, i.e. a GAP rather than a refusal.

`#389 = EDGE_CURVE('E5', #385, #336, #388, .T.)` over `#388 = QUASI_UNIFORM_CURVE('E5', 1, (#386, #387), .POLYLINE_FORM., .F., .U.)` — a two-point degree-1 polyline, exactly the class this issue is about. It stays NURBS today, and at the coarse band no rung is offered for it at all.

The gap is PRE-EXISTING (nothing in #327 can reach an open degree-1 carrier — the circle estimator refuses an open curve before it estimates anything); it was masked behind #685 at every band. It is pinned as data rather than averaged away: `tier_gate.rs` now carries dm1 as `EpsSensitive` with all nine cells (six at the rational-flux stall of #390, three here), and `r1_dm1_probe` / `review_probes_m7_3`'s V6 / `wild.rs` each state the two-cell fact.

Line promotion plus the `ExtrudedPoint` rung this issue asks for is what would give #389 a candidate.

## Home

`work/issues/`: the rung lives in `topo::pcurves::nurbs_iso_derive` and `step-import`, ground the closed PCURVE program vacated and that no open program's `paths` covers.
