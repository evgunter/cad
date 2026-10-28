# NURBSWIN — the clearance engine reads an iso-bounded NURBS face

Unit: `work/clear/window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch.md`
(P0). Program: CLEAR. Branch: `clear/nurbswin`. Review: single FULL
review.

## What is true when this lands

1. **`topo::clearance::window_of` admits a `Surface::Nurbs` face whose
   whole boundary is its chart's own isos.**
   - The face has one loop. Each of its edges carries an exact iso
     pcurve on a side of the knot domain.
   - Together those sides cover the domain's boundary, so the face is
     the whole `[u0,u1] × [v0,v1]` patch.
   - Its window is that rectangle. This is the loft and sweep wall the
     row describes, and caps that are planar are not affected.
   - Decide "iso-bounded" through the tree's existing iso machinery
     (`topo::chart_iso`, `topo::iso`, the pcurve records), not a new
     reading of the geometry. If the tree has no door that answers it
     soundly, stop and report what is missing.
2. **Everything else keeps its typed refusal:**
   - a trimmed NURBS face;
   - a face with a non-iso boundary edge;
   - a face whose loop does not cover the domain boundary;
   - `Surface::Approx`.

   Each refusal names what it found, beside today's "a free-form face",
   so a reader can tell trimmed from not-iso.
3. **Refinement narrows.**
   - `refines` and the cell enclosure (`surface.eval` over a
     sub-rectangle at `Interval`) must narrow a NURBS cell under
     halving.
   - Check this, and measure the cost on a loft wall. If the NURBS
     interval evaluation does not narrow (for instance, it encloses by
     the control hull without subdividing the basis), fix it at that
     door or stop and report it. A window that never refines is the
     `chart does not refine` class, not a budget.
4. **Rows, each of which can go red:**
   - two facing loft walls at a known gap read `Holds` above it and
     `Violated` below it, with a verified witness;
   - a sweep wall against a plane;
   - a trimmed NURBS face still refuses, typed;
   - `min_separation` between two NURBS walls brackets the known gap.

   The M10-5/M10-6 goldens do not move.

## Out of scope

The self-intersection redesign (`self-intersection-…`, Ev ruled on
#5866). This unit changes which faces have a window, not which pairs
are examined.

## Ground

- `crates/topo/src/clearance.rs` (CLEAR's, since SHELL-3), and whatever
  NURBS-eval door item 3 needs (`geom`/`geom-brep`). Announce any seam
  with `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full first.
- Never arm a waiter or a background chain. Anything that outlives
  600 s runs `setsid`-detached and is polled in the foreground.
- CI is the verification of record. This diff seeds `topo`, so allow
  25 min.

## Report

≤150 lines: the iso test you used and why it is sound, what refinement
does on a NURBS cell (with the measured cost), the rows, and the PR and
its CI.
