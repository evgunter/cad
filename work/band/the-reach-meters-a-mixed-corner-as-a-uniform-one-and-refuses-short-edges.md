---
id: the-reach-meters-a-mixed-corner-as-a-uniform-one-and-refuses-short-edges
kind: issue
title: The face clearance reach meters an all-three mixed corner as a uniform one, so short edges and close faces refuse FaceClearance where the corner clears
status: closed
opened: 2026-11-13
closed: 2026-11-17
priority: P1
cost: M
pr: 7078
branch: band/mixed-corner-reach
---


Found by the bracket step-3 fix pass (PR 6873), from both reviewers' probes of the all-three mixed corner (`work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`).

**Witnesses** (each builds as a fillet, or would by the geometry; each refuses `FaceClearance { bounded: true }`):
- the flush L of `crates/sweep/tests/band_mixed_corner.rs` (`flush`) with its odd edge 0.25 tall, chamfered at `s = 0.1`: margin exactly `0`, sign Zero, at the top face. It builds from 0.26; a uniform cube corner chamfers at 0.25.
- the bore-less bracket (`band_mixed_corner.rs`, `bracket(false)`) chamfered all round at `s = 0.14`: margin `0`, at the plate's bottom band (edge along `x = 0`, `z = 0`).
- the bracket filleted all round at `r = 0.149`: margin `−3.6e-5`, at a bottom-edge band. The flush and inset links along the 0.5 base edge do not overlap until `r = 1/6`.
- the reviewers' further chamfer witnesses at `s = 0.1`: a pair edge ≤ 0.2–0.25, an arm 0.15 thick, an inset boss 0.25 tall.

All are pinned as typed refusals in `band_mixed_corner.rs` (`a_mixed_corner_reach_metered_as_a_uniform_corner_refuses_typed`).

**Cause, traced.** `reach::corner_reach` (`crates/sweep/src/blend/reach.rs`) builds every fully-requested trivalent corner's reach as a UNIFORM corner's, and an all-three mixed corner is fully requested, so it is read that way too:
- **Fillet**: the reach is `corner_ball` of the first link's convexity, a sphere octant the mixed corner does not carve. Its box is `2·|c − p|` = `2√3·r` about the vertex, so at `r ≥ 0.145` it reaches the plate's bottom bands, and the ball bound reads them as not clear.
- **Chamfer**: each face's foot is where that face's two trimlines cross, so on `E` the reach takes the two PAIR strips' crossing, not the host feet, and the patch is the plane through those three points. That plane leans the wrong way for the real patch and bounds nothing near the top face within the box (`2√2·s` about the vertex: the region reaches `z = (2s + 2·2√2·s)/3`, so `0.255` at `s = 0.1`, the 0.25/0.26 threshold). Its side bounds are read at `q`, the centroid of the vertex and the feet, which at a mixed corner lies ON the walls' planes (`(0, 0, s/2)` on the flush L): `side` then gets a zero normal and reads `0` everywhere, which floors the meter at exactly `0`. That is the margin that does not move with length.

**What the fix needs.** A corner reach of the mixed corner's own: its real anchors (the feet on the odd edge's faces and the two host feet on `E`, as `open::planar::mixed_corner_plan` derives them), a bound on the torus piece or the mitred strip, and side bounds that hold at a reflex corner (the region is not the intersection of the three faces' half-spaces there). Its `replaces` must stay exact, since other chains' supports are cleared through it. That is a reach of a new shape, not a local fix, so the fix pass filed it.

P1: the bracket builds at its natural sizes (`r = 0.05` pinned), and the refusal is typed, but a chamfer of `0.14` on a 0.5 plate is an ordinary request.

**Closed** (lane `band-reach`). The reach meters a mixed corner by its own patch (`reach::mixed_corner_reach`, from `open::planar::mixed_corner_geometry`, which the plan now shares): the fillet's torus section between `E` and the odd band over the wedge between the pair's rests, the chamfer's mitred strip between its two mitres, each exactly the material the patch changes, `E` the one support side. Each link's window and core at the corner end where its band meets the patch, so a pair's strip is no longer claimed replaced on `E` past its rest. Every witness above builds at its closed form, f64 and Interval (`band_mixed_corner_reach`). Two corrections to the row: the bracket's fillet and chamfer build up to `0.249` at their closed forms, which hold only while no two bands overlap; the real limit is `1/4`, where the plate's top and bottom bands cross on its `0.5` sides, not `1/6`. And the uniform reading was unsound as well as tight: a void inside the fillet's real patch passed the reach, and the carve's body then failed to finish (`ShellWinding`); it now refuses `FaceClearance`.

The odd-plus-one chamfer's turn was read as a uniform turn: the pair strip's reach and core ran to the third face's plane, past the odd strip's plane where its material ends, so a void between the two read as crossing the band's material. The near band of a mixed turn now ends in the far strip's plane (`reach::MixedCut`).
