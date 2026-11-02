---
id: tess-lint-gate-red-on-main-23-findings
kind: issue
title: the nightly k-lint (release-budget) tessellation-budget gate is red on main with 23 findings against the 2026-10-02 baseline, and nothing has read them
status: closed
opened: 2026-10-15
priority: P1
cost: M
closed: 2026-10-28
---


## What

`k-lint (release-budget)`'s step `tessellation-budget lint (gate — a grown budget fails this row)` is red on nightly 60 (run 38516952143, 2026-10-14) and nightly 61 (run 38687151370, job 116264822560, 2026-10-15, main `1cd489fe0a`):

    tess-lint: gate vs ../../docs/tess-budget-data/tess-budget-baseline.csv (cut at 24f02a376460 (2026-10-02T00:54:03+00:00)): 23 finding(s)
    tess-lint: GATE FAILED — 23 finding(s).

The findings visible in the log tail are re-keyed faces: "a different face … the per-face join is by ORDINAL, so this face and every face above it went uncompared". For example:
- `heatsink5/7/9` face 7: chart sphere -> plane;
- `lily_sepal_b/c` face 2: nv 285 -> 286 and 286 -> 288;
- `klein_loop` face 2: nu 25 -> 22;
- `nonuniform_loft`, `twisted_loft`, `s_duct`, `twisted_tube`, `twisted_duct` (and its two shadows), `teapotspout`, `spring`, `cutaway_spring`.

No row on any slate names these findings (grep over `work/` for the scene names and "GATE FAILED", 2026-10-15). Nobody has checked which merge moved them.

## Owed

The gate's own recourse, item 4: establish for each scene whether the model changed (a face genuinely replaced is a geometry change) or the faces were only renumbered. Then re-cut the baseline with the reason in the commit. Do not re-cut before reading them.

— (CIW orchestrator, nightly triage)

**#5531 adds three findings to this list** (the trimmed lane's CDT in the
lattice metric): `twisted_duct/twisted_duct` triangles 1.07x over the
baseline (12508 → 13438), and `lofts/loft_prism` faces 3 and 5 recoverable
slack (1.1x → 1.1x, flagged as wastefuller). The re-cut folds them with the
23, and #5531's body has the per-scene deltas against `main`.

**30 findings from the 2026-10-22 nightly (INTENT triage, 2026-10-23).**
The gate read 23 findings on every nightly from 2026-10-17 through
2026-10-21, and 30 on 2026-10-22 (run 39728496598) and 2026-10-23 (run
39906156937, main `be9b907694`). The seven new ones: triangle growth on
`lily/lily_leaf_a` (10108 -> 15138), `lily/lily_sepal_a` (4234 -> 8774),
`lily/lily_sepal_b` (4194 -> 8772) and `lily/lily_sepal_c` (4220 -> 8772);
`twisted_duct/twisted_duct` (12508 -> 13438, #5531 above); re-keyed
`lily_leaf_a` and `lily_sepal_a` face 2 (nu 1 -> 21, 2 -> 15); and
`nonuniform_loft` growing again (19736 -> 20384). The lily rows arrived
in the merges between `8c7db2ae37` and `0ffab3663d`; not bisected further.

## 2026-10-24 (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`), `k-lint (release-budget)`:
`tess-lint: GATE FAILED — 30 finding(s)`. These are the same 30
`FINDING` lines as 2026-10-23's run 39906156937, diffed line for line,
so nothing new has entered the window. Not re-derived here.

## Earlier reading, merged from `work/instr/tess-gate-red-beyond-twopeg-since-the-10-02-cut` (2026-10-26, tracker sweep)

ENCL's nightly triage of 2026-10-12 (nightly 38115086131, same cut
`24f02a376460`) read 22 findings and filed them on INSTR. That row is
closed as a duplicate of this one. What it carried that this row does
not:

- **Triangle growth:**
  - `bracket/bracket` 44 → 60 (1.36×);
  - `lofts/nonuniform_loft` 14950 → 19736 (1.32×), which grows again in
    the 10-22 reading above;
  - `twisted_duct_shadow_{y,z}` 7260 → 7660 (1.06×);
  - the `twopeg` rows, filed at
    `work/tess/twopeg-triangles-grew-past-the-tess-gate-on-main`.
- **Re-keys it read**:
  - `nonuniform_loft` face 2 nu 16 → 19;
  - `s_duct` nv 112 → 121;
  - `twisted_tube` 519 → 544;
  - `twisted_duct` 658 → 682;
  - the shadows 465 → 482;
  - `spring`/`cutaway_spring` 3188 → 3102.
- **Its reading of the chart swaps**: the `bracket`, `tiltedcut`,
  `heatsink` and `projectbox` plane ↔ cylinder and sphere ↔ plane swaps
  look like a face-order change rather than a mesh change. Check that
  first, and do not coarsen delta.

## 2026-10-26 (SPLICE nightly triage)

Nightly run 40274924857 (main `38f1a24ed3`), `k-lint (release-budget)`:
`tess-lint: GATE FAILED — 31 finding(s)`. Diffed line for line against
2026-10-25's run 40165703340 (main `9f5b44561d`, 30 findings, the same
30 as 10-23 and 10-24), one is new:

    FINDING lily/lily_lantern: triangles 2556 -> 3038 (1.19x)

with the note `lily/lily_lantern face 8: in this sweep, absent from the
baseline`. The one merge in the window that touches `demos/tour/src/lily.rs`
is #5722 (`22b1a8fc2c`, "roll the lantern's mouth in the scene and retire
wall 6"). It fillets the sphere-zone × pucker-cone rim into a torus band,
which adds a face to the lantern. So this one is a GEOMETRY change by
recourse item 1: a new face, not a sizing regression. That attribution
comes from reading the diff; no sweep was run at the merge. It still
waits on the same re-cut as the other 30, and the re-cut is not taken
until the rest have been read (recourse item 4).

## Closed (2026-10-28)

**The gate is green, and has been since 2026-10-27.** Nightly 74 (run
40637814390, main `a1cfd1cfdb`) reads `0 finding(s)` against a baseline
cut at `c3ff4a85a526` (2026-10-26). That re-cut was taken by #5717
(`pred/mesh-sizing-rule`, commits `64770d2530`, `68b376acec`,
`3c97472e2e`), which re-cut for its own sizing rule and "absorbs the
tour's drift since the 2026-10-02 cut". It did so **without reading the
31 findings**, which is recourse item 4's order reversed. This section
is that reading, taken after the fact. It finds nothing the re-cut
should not have folded, so the baseline is not re-cut again.

**How it was read.** The 31 were reproduced three ways, and all three
agree:
- `tess-lint` over the current baseline, with the pre-#5717 file
  (`64770d2530^`, cut `24f02a376460`) as `--baseline`: 30 findings
  (the lantern is a note there, because #5717 shrank it);
- a local `--sizing-only`-equivalent sweep of `38f1a24e`, the last
  main before #5717, gated against the same file: the 31 `FINDING`
  lines of nightly 72 (run 40274924857), line for line;
- every nightly's `FINDING` lines, 10-03 to 10-26, diffed night to
  night to date each move.

Between `38f1a24e` and the re-cut, **all 68 NURBS rows are identical
in every column but `worst_dev`/`dev_samples`**. #5717 moved only
analytic faces (114 plane, 125 cylinder, 20 cone, 177 sphere, 68
torus). So every sized-row move below is drift, not #5717. No scene
vanished (85 → 85), and every face-count change is in the table.

**No finding was a renumbering of a sized face.** The 19 sized re-keys
are each the same face at the same ordinal and chart, whose
whole-patch divisions (`nu`/`nv`, an identity column) moved because
the face was re-charted or its bound tightened. The pure renumberings
(`heatsink5/7/9`, `projectbox/cutaway_above`, `cutaway_below`: identical
multisets of chart, triangles and trim box) carry no sized face, so
they were notes and never reds. C15's stable identity would not have
cleared any of the 31; the evidence is on `work/instr/C15.md`.

| night it appeared | scenes | what moved | merge | verdict |
|---|---|---|---|---|
| 10-03 | `twopeg` ×3 (+ notes) | plate corners rounded, `CORNER_R` 0.5: four cylinder faces added; 116/204/212 → 164/300/308 | #3657 (`b43aa794e5`, 10-02 02:32) | geometry, intended. #5717 then took them to 76/220/228 |
| 10-04 (note) | `tiltedcut_above` face 3 | cylinder → plane, 2608 → 2600, 27 faces both | a tilted-cut change in 10-02 → 10-03 (likely #3805, REACH's exact ellipse rim); **not bisected** | geometry; no sized face, and rule 1 covered the total, which shrank |
| 10-06 (note) | `heatsink5/7/9` face 7 | renumbered only | #4119 (the plate rounds after the union) | renumbering |
| 10-07 | `nonuniform_loft` (14950 → 19736), `twisted_loft`, `s_duct`, `twisted_tube`, `twisted_duct` + both shadows (7260 → 7660), `spring`/`cutaway_spring`, `teapotspout` nv, lily blades nv, `klein_loop` | every loft and sweep's v re-parameterized: the outer loop's whole section set, not the first strip | #4193 (CARVE `loft-v-is-the-whole-sets`, with #4186/#4188) | geometry (a re-chart), intended. Its body derives the nonuniform loft's new bulge (half-width 1.646 → 1.853) and pins the volume's closed form |
| 10-07 | `bracket` 44 → 60 (+ face 3 note) | blend bands the scene used to wall now build | #4209 (mitre), d26fdc848f (#4121, band end face) | geometry, intended |
| 10-11 (note) | `cutaway_above` face 12, `cutaway_below` face 23 | renumbered only | 10-10 → 10-11 window; not attributed further | renumbering |
| 10-14 | `bracket` 60 → 76 | the chamfer's non-isosceles turn builds its overrun | #4926 | geometry, intended. #5717 then took it to 68 |
| 10-14 | `klein_loop` nu 25 → 22, `lily_leaf_b/c` nu, `teapotspout` nu 50 → 45 | the rational patches' Hessian bound (`muu` etc.) tightened | #4984 (one second-order quotient recipe for rational bounds) | a bound change, not geometry. It moves the counterfactual divisions; each face still certifies |
| 10-22 | `lily_leaf_a`, `lily_sepal_a/b/c` (10 faces → 4; 10108 → 15138, ~4200 → ~8770) | the blades' eight-segment straight "kite" section became the same two-arc lens as `leaf_b`/`c`: two curved NURBS walls instead of eight ruled strips | #5568 (`quad/check7-sign-walk`) | geometry, intended. A doubly curved blade needs the triangles |
| 10-22 | `klein_loop` (10 faces → 6, nu → 44) | the section is `circle`, not `circle_split(4)`: four walls with doubled knot cells, not eight. Wall 5 retired | #5568 | geometry, intended |
| 10-22 | `nonuniform_loft` 19736 → 20384, `twisted_duct` → 13438, shadows → 7700, `loft_prism` faces 3/5 slack | the trimmed lane's CDT runs over the lattice metric, so the column snap went dormant (2573 bands → 0) | #5531 | a sizing change, a documented trade for meshing at every δ. Its body's table gives these exact ratios (1.03–1.04x, 1.005x) |
| 10-26 | `lily_lantern` 2556 → 3038 (face 8 new) | the sphere-zone × pucker-cone rim filleted into a torus band | #5722 | geometry, intended. #5717 then took it to 1900 |

Not attributed to one merge: `teapotspout`'s total fall (71588 →
61242, every wall about −13%). Its `nv` moved on 10-07 (#4193) and its
`nu` on 10-14 (#4984), and nothing in the nightly record moves it
after that. A fall is not a gate finding, and each face still
certifies, so it is left there.

Nothing here is a kernel finding. Each geometry change is a deliberate
model change whose own PR reads its new shape, and the two lane changes
(#4984, #5531) are documented in their PRs.

Closed by the GREEN `tess-lint` lane (PR `green/tess-lint-red`).
