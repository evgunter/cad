---
id: a-cup-leaning-off-the-spine-under-a-fillet-is-charged-its-boxs-diagonal
kind: issue
title: blend: a cup whose axis leans off the spine under a fillet refuses as a bound, its walls charged their box's diagonal times the lean rather than their run along the ruling
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## Finding

A cup on a plane–plane band's cut-off cap goes with the band when every
face is read inside the prism `M` (`reach::prism_inside`). A face that
is not a plane certainly not parallel to the spine — a cylinder wall,
or a plane along it — is read by its ruling: each point lies on an
`a`-line between two boundary points, so it is charged its run along
that line, bounded today by the face box's **diagonal**, times
`|a × τ|`. Under the convex chamfer no term is owed (the triangle is
convex); under the fillet the term is charged in full.

`band_one_cycle_members::a_leaning_blind_bore_goes_where_its_lean_is_paid`
pins it: a bore of radius `0.006` from `(0.012, 0.012)` at `x = 0` to
its floor at `x = ½`, its axis turned `0.0011` in each corner
coordinate, goes under both verbs; turned `0.0031` (a lean of
`≈ 0.0088` rad), the fillet charges each wall its seam lines' slack
(`0.5 · 0.0088 ≈ 0.0044`) and its box's diagonal times the lean again
(`≈ 0.0044`), against a depth of at most `0.006`, and refuses
`FaceClearance { bounded: true }`, though the bore lies wholly inside
(the chamfer takes it at the closed form). `a_cup_leaning_across_the_fillets_disk_
crosses_and_the_chamfer_takes_it` pins a pin leaning 26° along
`u + v = 0.054`, inside the fillet's fill throughout, refusing the same
way.

## What a fix needs

Charge the run along the ruling actually inside the face — for a
cylinder, the axial extent of its boundary (`sheet_clip` already reads
it, `reach.rs`), not the box diagonal — and once, not again as the
seam lines' slack (the seams are rulings themselves); or read the wall
piecewise along its ruling against the core, as `curve_in_core` reads
an edge. Cost M.
