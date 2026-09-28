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
