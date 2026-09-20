---
id: clearance-window-tightening-needs-chart-boundary
kind: unit
title: Clearance windows are bounding rectangles: tightening needs the face boundary in chart coordinates
status: closed
opened: 2026-09-03
branch: trim/3-chart-bound
pr: 2554
closed: 2026-09-15
---

## What

M10-5's clearance engine subdivides each face's **carrier window** — a
conservative superset of the face's trimmed region
(`crates/editor-core/src/clearance.rs:41`, `window_of` at
`crates/editor-core/src/clearance.rs:1314`). A planar face's window is
its bounding rectangle; a cylindrical face's window is the whole turn at
the face's axial span.

The looseness runs the safe way for a defect gate — `Holds` covers
strictly more than the faces and is therefore sound about them — but
`Violated` can report a sub-`c` approach at a place neither face
occupies, and the unit ships that as deviation D3.

## How big it actually is, measured

Two shapes, both pinned in
`crates/editor-core/tests/m10_5_r2_probes_interval.rs`:

- **A non-convex planar face.** An L-shaped bottom cap's window is the
  full bounding square, which covers the notch. A block parked in the
  notch stands 0.45 m from the FACE and 0 m from the WINDOW, so a bound
  of 0.3 — which the two faces satisfy with 50 % to spare — is reported
  `Violated`, with a witness `(u, v)` landing where the body has no
  material. This is not a rounding-scale near-miss: the error is 150 %
  of the bound.
- **A coplanar pair.** Two coplanar faces of one body have windows that
  overlap in the carrier's own parameters even when the faces are metres
  apart, so the reported separation can be 0 where the real one is not.

Both are reported, never missed, which is why the unit ships with them.

## What a fix needs

The face's boundary in CHART coordinates — the pcurve layer's
description work. With it:

- a planar window can be intersected with the loop's own 2-D extent, and
  a cell that falls entirely outside the trimmed region can be dropped
  rather than classified;
- a cylindrical window can be cut to the face's real angular span
  instead of the whole turn.

Neither is a change to the funnel or to the receipt identity: the
subdivision would simply start from a smaller set and drop cells the
boundary excludes. What it is NOT is a tolerance — a cell is dropped
only when the boundary description certifies it outside, so the
direction of the looseness is preserved.

## Home

`work/m10/` — the code is `crates/editor-core/src/clearance.rs`, an M10
deliverable; the dependency (`pcurve`-layer chart boundaries) is not
scheduled in M10, so this is a consumer waiting on it.

## PR-1 merged (2026-09-07)

PR #1911 landed the `topo` half: `chart_boundary`, `ChartBound`,
`MetredBound::hull()`, `certifies_outside`, the six `chart_bound_*`
rows, and 18 real-body rows from the dual. The item stays `dispatched`
for PR-2 (the `clearance.rs` seam, branch `trim/3-window-seam`), which
opens after the seam is announced to SHELL and M10.


## PR-2 open (2026-09-13)

The `clearance.rs` seam, branch `trim/3-window-seam`. `window_of` cuts
a plane's and a cylinder's root to the metred hull; `Sweep::run`
discharges a cell pair the description certifies empty of face
(`CellReceipt::outside`); `verify_witness` admits only stations on the
face; `ClearanceReport::windows` counts tightened against loose. Both
shapes this item measured flip: the L cap no longer reports the block
in its notch, and the U-channel's coplanar caps no longer report the
slider.

Six residues filed with it:
`clearance-window-cone-sphere-torus`,
`exact-region-cells-for-lower-bound-only`,
`min-separation-tightening-crosses-the-drive` (the half of spec §3(b)
that did not land), `revolved-bands-reach-no-clearance-row`,
`three-tables-of-the-chart-arms`, and
`a-refused-chart-boundary-has-no-reachable-window`.

## PR-2's fix pass (2026-09-14)

The v6 dual on `8e53655d2` returned MERGEABLE-AFTER-FIXES on both arms
with sixteen items; the adjudication is comment 5675317215 on #2554.
Nothing about the shipped seam's soundness moved — neither arm could
manufacture an unsound `Holds` — and what changed is the verification
record: three claimed mutant kills that did not execute, the
deviation-1 mechanism (wrong on both arms' measurement, corrected in
all three places), a residue file whose structural premise was false
(refiled as `revolved-bands-reach-no-clearance-row`), the cylinder root
rule lifted into a named `cut_root` with unit rows and a live e2e row
for the `[0, τ] ∩ hull` mistake, and the drop's non-overlap fallback
turned from "keep the description" into "drop it".

## Closed (2026-09-15)

PR #2554 merged (ordinal 2502, sample #201; block TRIM-B1 slot 2 —
the block concludes): the clearance sweep reads each face's
chart-boundary description, cuts the root window to the described
hull and drops the cells certified off the face; the three measured
shapes flip. The unit's record is MODEL-AB-LOG row T3B; the spec's
§Rulings and Amendments are the ruling. Six residues stay open on this
program (listed above); `min_separation` stays untightened as the
identity with its mechanism now stated correctly.

## Evidence, added 2026-09-14 (PROPS's sign-hull unit)

**One end of the looseness is worse than `Violated` at a place neither
face occupies: an enclosure that does not contain the truth.** Measured
by `r2_m10_6_probes_interval::min_clearance_between_two_separated_bodies_reads_zero`
— a C-shaped solid and a block 0.1 m apart, whose `MinClearance` at
`Interval` reads

```text
notched pair: min_clearance = [0, 0.02576941016012847], true solid separation 0.1
```

A minimum over a SUPERSET of the two faces is at most the minimum over
the faces, so a window that covers the notch pulls BOTH ends down: `lo`
collapsing to zero is the sound direction this item already describes,
and `hi` landing below the truth is the same mechanism in the direction
nothing guards. The measure layer refuses that arm typed today
(`UnevaluatedReason::WindowSuperset { endpoint: "upper", .. }`, whose
`recourse` names this file), so nothing reads the unsound endpoint — but
the refusal is the cost, not the fix: the assertion `min_clearance ≥
0.05`, which these solids meet twice over, gets no verdict at all. The
row asserts the refusal, its endpoint and its recourse.

Frame-independent: these carriers are axis-aligned, so the stored frame
and the re-chart the engine used to compute differ by a quarter turn
about the normal, and the boundary AABB projected on the frame's axes is
the same point set under an axis swap. What the quarter turn changed was
which axis the subdivision halves first — the numbers moved, the defect
did not.
