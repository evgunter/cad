---
id: interior-loop-cut-in
kind: issue
title: A face pair with a certified interior loop (the section certificate's R-loop) refuses where the loop could be cut into both faces and answered
status: open
opened: 2026-09-28
priority: P3
cost: H
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

The section certificate (`crates/topo/src/boolean/section_cert.rs`,
PR 3372) refuses R-loop when a face pair's section has a component
certified strictly inside both faces with no event on the pair: the
half-donut bracket, the dome bracket, the cylinder P0 saddle and its
pin-less control. Each is a definite fact, with a closed-form witness
point on the loop, and the refusal is the honest answer while nothing
can cut the loop in.

The fork's second half (the spec's Q2): insert the loop as a ring edge
in both faces (`euler_ring`), so the join sees it and the op answers.
Rows to flip: `germ_interior_oval.rs`' half-donut and dome rows,
`germ_interior_saddle.rs`' saddle rows, with closed-form volumes
(the half-donut lens is `oval_lens_volume`; the P0 ∩ is `0.0900944`).

## Evidence (2026-10-05, `reach/trimmed-sphere-escape`)

For a sphere face's R-loop against a plane face, the cut that answers
is not a ring: `boolean::ops::apply_cut_ins` cuts the sphere face along
its own chart's meridian through the circle, boundary to boundary, so
the crossing layer meets the circle at two pierces and no face carries
a ring. A ring edge cut into the sphere face (measured first on that
branch) reached the join's sphere ring lane
(`a-ring-on-a-sphere-face-has-no-island-winding`) and then props and
point classification, neither of which reads a ringed sphere face
(`sphere-face-with-a-hole-has-no-closed-form`). The meridian's pieces
stay in the result (the cosurface merge records a period closure and
keeps them), so the hole a kept region leaves runs into the face's
outer loop instead of standing as a ring.
