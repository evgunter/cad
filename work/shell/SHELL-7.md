---
id: SHELL-7
kind: unit
title: the axial offset door takes a one-surface corner — the full-period torus shells
status: open
opened: 2026-09-08
branch: shell/7-seam-corner
refs: [axial-door-refuses-a-one-surface-seam-corner, SHELL-5, 1674]
---


`offset_charts_together` refuses a corner where one profile constraint
meets and the vertex is off the axis; a full-period torus's seam
vertex is exactly that, so the solid torus and its hollow twin both
refuse to shell. A vertex all of whose faces lie on one surface of
revolution is a point of that surface and moves by `d` along its own
normal — the concentric move on a profile circle, the perpendicular
foot on a profile line — with the azimuth carried from the old vertex
as every seam's is; the torus's latitude seam takes the standard
latitude rule. Closes `axial-door-refuses-a-one-surface-seam-corner`.
Spec `docs/SHELL-7-SPEC.md`. Pre-draw difficulty **S–M**, task class
**NUMERIC** (one closed-form arm in the corner solve, one carrier arm,
closed-form rows) — logged AFTER block SHELL-B2's byte was drawn (the
block was drawn at SHELL-6's cut), so the covariate is contaminated
for this row and is disclosed as such.
