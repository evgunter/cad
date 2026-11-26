---
id: a-sphere-cut-in-refuses-a-seam-lying-in-its-meridian-plane
kind: issue
title: The sphere cut-in refuses a one-band face whose seam lies in the cut's meridian plane on the far half-plane
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-26
pr: 7440
---


Found by the LATHE one-site lane (2026-11-23), measured on its branch
`lathe/one-site-wrap-edges` (stacked on `lathe/full-revolve-one-band`).

## What

`sweep` `snowman::a_slab_tilted_across_the_lens_seams_builds` (the lens
cut by a slab turned ±20° about `z`) passes the join now and refuses
`FallbackExtentUnsupported { operand: A, face, what: "a boundary arc of
the sphere face meets the cut's meridian plane where its roots are not
certified" }` at every ε. That string is `crates/topo/src/boolean/ops.rs`
`apply_cut_ins`, on `CoSurface::InPlane | Touch`.

Hypothesis, not measured past the payload: the slab's circle near the
pole misses the lens's one seam, so the off-face extent scan cuts the
face along the meridian half-plane `h` through the circle's centre
direction, which lies in the seam's plane `z = 0`. The seam, a boundary
arc lying wholly in that plane (on the far half-plane for one tilt),
answers `InPlane`, and the cut-in refuses it as uncertified rather than
reading it as no hit off `h` (or a hit at its pole end). With two
π-bands the circle crossed a seam at either azimuth and the crossing
layer met it without a cut-in.

## Ground

`crates/topo/src/boolean/ops.rs` `apply_cut_ins`; claimed by LATHE as the
one-band consumer gap it is. The sibling row
`a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard`
works the same "circle missing the seam" class in the same file, and may
retire this cut-in's need for the pair.

- 2026-11-26 — Closed by PR 7440 (commit 5a77b64ac2, in the one-band stack), landed with PR 7414: `apply_cut_ins` reads a one-band seam lying in the cut's plane as no hit. Witness `sweep snowman::a_slab_tilted_across_the_lens_seams_builds` fails at 7440's first parent (`FallbackExtentUnsupported`, this row's text) and passes after it, at ε 1e-9, 1e-6 and 1e-12. The port onto ORBIT's closed cut keeps the no-hit.
