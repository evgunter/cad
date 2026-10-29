---
id: the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch
kind: issue
title: the at-rest volume-sign gate and the measure cost 10-28x more on a conic patch split per span than on the same patch whole
status: open
opened: 2026-10-29
priority: P1
cost: H
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot]
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

## What

The import door now splits a foreign chart at each interior C⁰ knot (Ev's PR 5913 ruling, DESIGN [topology]). A rational conic patch states every span joint as a double knot, so it imports as one face per span. `a_conic_quarter_torus_measures_a_bracket_at_the_wall`'s torus, two 9 × 9-span conic half bands, imports as 162 one-span walls instead of 2. The locus is the same, and so is the total cell count. The cost is not.

## Measured

Release, `the_conic_quarter_torus_answers_a_bracket_holding_pappus`, with the import phases timed (one run each; other lanes were building on the box, so the user-CPU line is the steadier figure):

| | origin/main `ac788c57` (2 walls) | branch head (162 walls) |
|---|---|---|
| `entities::resolve` | 0.016 s | 2.6 s |
| assemble + adopt | 0.34 s | 4.1–4.6 s |
| `gate3` (tier 3, check 7's volume sign) | 4.9 s | 118–138 s |
| whole import | 5.3 s | 125–146 s |
| `measure` at 1.024e-6 m | 13.5 s, stops `Wall`, pad 2.9e-3 | 134 s, stops `Met`, pad 3.8e-4 |
| test, real / user | 19.2 s / 26.5 s | 259 s / 492 s |

Debug: the same test on main is 90 s in total. On the head the import alone is 1044 s.

Stack samples of the head's release run during `gate3` sit under `topo::validate::plus_v_by_sign` → `SignCertificate::at_round_zero`, and during `measure` under `topo::props::quad_lane::cut_face_rounds` → `geom_brep::props::quad::patch::nurbs_patch_face_rounds` (`PatchGrid::collapse_1d`, `area_midpoint_taylor`, blossom `split_restrict`). The time is per-face rounds of the rectangle lane, paid 162 times over: the edge certification is not where it goes.

## Why P1

Every rational conic surface in a foreign file (cylinders, cones, tori stated as rational B-splines with double knots, dm1's walls among them) now pays this at import, through the at-rest gate every imported body passes. The per-face round-zero cost of a one-span patch should be a fraction of the whole patch's, not a multiple of it.

## Owed

Find what in round zero scales with the face count rather than with the cells (a fixed per-face grid, a refinement read relative to the face, a per-face set-up), and make the split patch cost about what the whole patch did.
