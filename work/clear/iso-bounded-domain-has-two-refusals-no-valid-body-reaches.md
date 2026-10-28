---
id: iso-bounded-domain-has-two-refusals-no-valid-body-reaches
kind: issue
title: iso_bounded_domain's does-not-close and winding refusals have no row: no valid body reaches either
status: open
opened: 2026-10-28
---


## Finding

`iso_bounded_domain` in `crates/topo/src/clearance.rs` refuses a NURBS
face for five reasons. Three have rows in
`crates/topo/tests/clearance_windows_on_a_spline_chart.rs`: a ring
(trimmed), a non-iso image (the triangle's diagonal), and an iso on an
interior column (a square on a chart twice as wide, which refuses at the
per-image side check with the not-covering text). Two have no row:

- **does not close in its chart**: consecutive lifted images whose
  end and start intervals do not meet. `loop_lift` decides each joint's
  deck element between the two images at their shared vertex, and the
  rows' certificates pin each image's ends to its edge's vertices, so a
  body that validates cannot carry such a pair. Building one needs a
  stored row that disagrees with its own edge, which the mint's
  certification refuses; NURBSWIN found no test door that stores one
  past it.
- **winds other than once**, with every image on a side: a closed
  chain on the domain's boundary that winds 0 or 2 times. Winding 0
  means the chain backtracks along a side, which needs two edges with
  one image, a zero-area sliver face. Winding 2 needs a loop that passes
  each corner twice, which is a loop through repeated vertices. Neither
  is a face the Euler operators or the validator admit.

Both stay as guards: they are what makes the admission a decision and
not an assumption about the body. If a door ever mints such a loop,
its own test is the row.

Filed by NURBSWIN (CLEAR), 2026-10-28.
