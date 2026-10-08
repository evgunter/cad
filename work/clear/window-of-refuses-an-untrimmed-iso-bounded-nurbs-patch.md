---
id: window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch
kind: issue
title: clearance's window_of refuses every NURBS face, including an untrimmed patch bounded by its own chart isos, so the engine cannot answer about any loft or sweep wall
status: open
opened: 2026-10-06
priority: P2
cost: M
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
