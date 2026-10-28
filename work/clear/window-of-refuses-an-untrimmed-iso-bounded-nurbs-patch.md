---
id: window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch
kind: issue
title: clearance's window_of refuses every NURBS face, including an untrimmed patch bounded by its own chart isos, so the engine cannot answer about any loft or sweep wall
status: closed
opened: 2026-10-06
priority: P0
cost: M
branch: clear/nurbswin
closed: 2026-10-28
pr: 5927
---


## Finding

`window_of` in `crates/editor-core/src/clearance.rs` returns
`unsupported("a free-form face")` for every `Surface::Nurbs`, and also
for `Surface::Approx` at the second site. So the engine cannot answer
any question about a body with a loft or sweep wall. That holds even
after SHELL-3 moves the engine's `Body<Interval>` half into `topo`
(`work/clear/SHELL-3.md`).

The refusal's reason, that there is no certified window, does not apply
to the commonest NURBS face the kernel mints. A loft or sweep wall is an
untrimmed patch whose four boundary edges are its chart's own isos,
with exact iso pcurves. Its window is the whole `[0,1]²` knot domain,
with nothing to prove. The cell evaluation already calls
`surface.eval` generically. A trimmed NURBS face, or one whose boundary
is not iso, keeps the refusal.

## Why it matters now

This is the second prerequisite, beside SHELL-3, for the loft
embedding certificate both CARVE designers recommend
(`work/carve/self-overlapping-spines-build-and-validate.md`, CARVE
log 2026-10-06).

Filed by the CARVE orchestrator, 2026-10-06, from two designer reports.
Not measured.

## Closed (2026-10-28, #5927)

- **What lands.** `topo::clearance::window_of` admits a NURBS face whose one loop lifts to iso images on the exact knot-domain bounds, winding once (`iso_bounded_domain`). Its window is the whole patch, with region `Whole`.
  - Trimmed faces, non-iso faces and faces that do not cover the domain refuse, typed. Each refusal names what it found.
- **A defect found on the way.** NURBS cells did not refine: interval `eval` over a wide cell is unbounded. The fix is the new door `NurbsSurface<Interval>::rect_enclosure` in geom, which restricts the control net per knot cell. It halves the cell width per halving.
- **Review.** A single FULL review read APPROVE-WITH-FIXES: no unsoundness found, and a probe of 12,000 cells found 0 misses.
  - Its MINOR was that no row went red under an under-enclosing door. The fix pass added a containment fuzzer and a curved-pillow row that both go red under that mutation.
  - The orchestrator read and accepted the fix pass.
- **Filed:**
  - `iso-bounded-domain-has-two-refusals-no-valid-body-reaches`;
  - `restricted-net-hull-has-four-spellings`;
  - `boundary-of-evaluates-a-spline-edge-over-its-whole-span`;
  - charttail's `cell-region-reach-reads-a-spline-cell-by-wide-eval`.
