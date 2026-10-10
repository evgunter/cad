---
id: a-rod-less-an-inscribed-square-prism-builds-and-the-tessellator-panics
kind: issue
title: A rod less an inscribed square prism builds sound, and mesh::tessellate panics on it
status: open
priority: P2
cost: M
opened: 2026-10-09
refs: [tessellator-panics-on-a-self-slit-face-every-validator-passes]
---

Found by the dual review of the tube-on-a-ball unit (PR 4399, review r2, its N5, family F7), and present on base before that PR.

## What

- **The operands:** a square prism inscribed in a rod. The square has corners `(±1, 0)` and `(0, ±1)`, over `z ∈ [−1, 3]`. The rod is a `z`-axis cylinder of radius 1 over `z ∈ [0, 2]`. The prism's four vertical edges lie in the rod's wall.
- **The result:** `rod ∖ prism` builds SOUND. That covers tiers 2 and 3′, the certificate, a legal operand, and the volume `2π − 4 = 2.283185307`. The same holds turned 0.3 about `z`.
- **The failure:** `mesh::tessellate` then panics at `crates/mesh/src/tessellate.rs:592`, "edge of 4 face triangles": the four pieces meet along lines.

The class is the one of `tessellator-panics-on-a-self-slit-face-every-validator-passes`: a consumer panicking on a body every validator accepts. Either the mesher handles four faces on one line edge, or a validator refuses the body.

## Repro

Review r2's probe file `crates/sweep/tests/r2_probes.rs`, family F7 ("inscribed square in a rod"), on branch `join/tube-ending-on-a-ball-review-r2`. Its lines are in `review-r2/probe-head.txt` (`mesh=PANIC` on `B ∖ A`).
