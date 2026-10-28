---
id: a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole
kind: issue
title: The spline point-in-face door reads a cell touching a closed chart's seam, or a point near a pole of the chart, as unsettled
status: open
opened: 2026-10-27
priority: P2
cost: E
---


Filed by the SPLICE lane that built `topo::boolean::spline_region`
(PR 5805), from both reviewers of its dual review (r1 NOTE-1, r2).
This is a completeness gap; the reading stays sound.

`Trim::read_window` meters a query's clearance from the trim over a
window round the query, clipped to the chart's knot domain, and first
decides every way out of the window past the band
(`Trim::exits_clear`). On a channel the chart is closed in, the domain's
side is a way out (the cover goes on past it), so:

- **A cell touching a closed channel's seam is never placed.** Its
  window is clipped at the seam with no room, so the exit decides zero
  and the cell reads `Near`. The carrier walk then subdivides it down to
  its floor. A point whose only feet lie in such cells reads unsettled
  (`Trim(None)`) unless a foot is found in a cell clear of the seam. A
  foot's own point placement reads `Near` within about the band, along
  the chart, of the seam, though the seam bounds nothing there.
- **A point within about `du/512` of a pole or a collapsed row reads
  unsettled.** Every window still holds the pole's cell after the six
  halvings, and that cell has no positive arm (`chart_stretch_inf`'s
  inf is zero there), so no window gates.

The fix is to read the window in the cover: wrap it across the seam and
take its arms from the cells on both sides. For the pole, let the
halvings continue past six while the window shrinks toward the query,
or meter by a pole-aware arm. Witness rows would be a face across
`square_tube`'s seam (`spline_region_rows.rs`) read at a point within
`1e-3` of `u = 1`, and a point near a degenerate row of a capped chart.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
