---
id: the-slow-ssi-rows-stay-over-twice-their-pre-4954-time
kind: issue
title: The slow SSI rows stay over 2x their pre-4954 time (shelling 2.1x, r1 1.5x after #5391), against a 2x target
status: closed
opened: 2026-10-14
closed: 2026-10-19
---


## Measured

After the quotient unit (#5123 with the thin convex step), at opt-level 1,
`--test-threads 1`, sequential, medians of three interleaved runs on one
box against main `85caa597`:

| row | pre-4954 | main `72ad4910` | main `85caa597` (step-2 cut, #5059 in) | #5123 `b3d723f3` |
|---|---|---|---|---|
| `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit` | 35.3 s | 237 s | 143 s (4.1×) | 97 s (2.7×) |
| `r1_lane0_e2e::the_f64_seam_answers_every_public_door` | 12.6 s | 167 s | 49 s (3.9×) | 32 s (2.5×) |
| `nurbs_cert::tests::rational_face_bound_dominates_sampled_hessian` | 0.073–0.086 s (`c16eaf57`) | 0.87 s | 0.54–0.77 s | 0.48–0.58 s (about 6×) |

The target is about 2× pre-4954 (the CI wall-time lane's). No cache is
added: the quotient fork admits a window-keyed cache only if a
measurement asks for one, and its round 2 found window keys never
repeat.

## Re-timed after #5391 (2026-10-19)

Main `36bdaf3b` against pre-4954 (`092b3022^1`, `27c7b42e`), fresh
target directories, opt-level 1, `--test-threads 1`, three interleaved
rounds:

| row | pre-4954 | main `36bdaf3b` | ratio |
|---|---|---|---|
| shelling | 34.3 / 33.9 / 34.2 s | 72.6 / 72.7 / 71.1 s | **2.13×** |
| r1 | 11.7 / 11.8 / 12.3 s | 18.5 / 17.4 / 17.5 s | **1.49×** |
| face bound | 0.076–0.086 s | 0.41–0.61 s | about 5× |

r1 is under the target, but shelling is not, and its runs do not
overlap the 2× line (68.4 s). gdb sampling of shelling on main (66
samples over two runs), each sample in its first bucket:

| bucket | samples |
|---|---|
| whole-cell formation (`chart_speeds` → `fold_cells` → `cell` → `of_spans`, `cell_first_quotients`) | 15 (23%) |
| other `enclose` (whole-cell readings, `fold_cells`) | 12 (18%) |
| `rect_box` (#5059's point reading) | 10 (15%) |
| the cut's restriction (`restrict_axes`, `split_restrict`) | 8 (12%) |
| `transverse_readings` | 6 (9%) |
| offset fit | 6 (9%) |
| the cut's halving | 5 (8%) |
| other | 4 (6%) |

**Each wall is minted once per section, not once per shelling.**
`topo::offset_derive::pair_section` calls `geom_brep::Section::of` (and
`plane_section` calls `PlaneSection::of`), which mints a fresh
`ChartedNurbs` for each wall (`ssi/pair.rs:127`, `ssi/plane.rs:198`). `ChartedNurbs::mint`
reads `chart_speeds` over the whole domain, which forms every cell of
the wall. Instrumented, the shelling row mints 17 times for 4 distinct
walls (53×30 knots each), so 13 of the 17 whole-wall formations repeat
one already done. `ChartedNurbs` is already the value its doc says "a
cell is read once for the operand's life"; the doors just never take
one.

## Levers, in order

After #5391, in the order they would close shelling's gap (it needs
about −6%):

- **Mint each wall once per operation.** A `Section`/`PlaneSection`
  door that takes the walls' `ChartedNurbs`, minted once by the caller
  (`topo::offset_derive`), would drop the 13 repeated whole-wall
  formations: most of the 23% bucket above. This is a lifetime, not a
  window cache (the value already exists and is shared by clone); the
  seam is topo's offset derivation and ssi's section doors.
- **`Interval::mul_nonneg` in the thin step:** the restriction's
  arithmetic, 12% above. **Done in #5424** (8f1edb5f, bit-identical):
  shelling −7.6%, r1 −11% against main 8c6e959e.
- **#5163 itself** in the offset fit (about 9% there).
- **`rect_box`** (15%): not yet split.

Before #5391 (kept for the record):

- **The restriction cut's own steps 2–4**
  (`cheapen-the-restriction-cut-steps-2-to-4`): axis-lazy restriction,
  one window reading per window, batched channels.

- **Exact scale** (#5114, `work/issues/interval-scale-by-an-exact-f64-pays-a-general-product`):
  a halving's one `2⁻ʳ` scale per output pays a general interval
  product. Estimated at most 25% of the face bound.
- **The shared weight powers' halving**
  (`work/nurbs/halving-repeats-the-shared-weight-powers`): `W²` halved
  twice and `W³` four times per cell. Estimated about 8% of the face
  bound.
- **Fewer pieces** (`RATIONAL_CERT_SPLITS`, 16, and `BLOCK_PIECE_SPLITS`):
  priced against the retired refinement. This is a measurement before
  it is a change, because the count moves cell granularity, the census
  and the offset goldens.
- **What the slow rows spend now** is not yet profiled after step 3.
  The next step is a gdb or callgrind split of shelling and r1 on
  #5123's head.

## Closed: both rows under 2× (2026-10-19)

Each wall's chart is minted once per offset (`nurbs/mint-walls-once`:
`topo::offset_derive::WallCharts`, `Section::of_charted`,
`PlaneSection::of_charted`). The shelling row's mints fall from 17 to 5
for its 4 walls: one per wall in the planning frame, and one more where
the written edge is certified (`Body::set_face_surfaces_describing` →
`edge_nurbs::plane_nurbs_limbs`), a separate phase after the planning
frame drops its slots.

Pre-4954 (`092b3022^1`), main `6d53b3d2` and the branch, fresh targets,
opt-level 1, `--test-threads 1`, three interleaved rounds on one box:

| row | pre-4954 | main `6d53b3d2` | mint-once |
|---|---|---|---|
| shelling | 28.5 / 27.7 / 27.7 s | 59.6 / 59.4 / 58.8 s (2.14×) | 51.4 / 51.7 / 51.6 s (**1.86×**) |
| r1 | 9.7 / 9.5 / 9.8 s | 14.3 / 14.5 / 14.2 s (1.47×) | 14.5 / 14.4 / 14.5 s (**1.49×**) |
| face bound | 0.064–0.074 s | 0.32–0.36 s | 0.33–0.38 s |

Every `limb_three` decision on shelling, r1, `m5_pr7_ssi::` and
`nurbs_nurbs_section::` reads as main's, bit for bit. The levers left
for whoever wants more: `Interval::mul_nonneg` in the thin step once
#5163 lands, #5163 in the offset fit, and `clears` at r1
(`an-adaptive-block-piece-split-reads-one-piece-where-the-hull-decides`).

**Measured on the merged levers** (build C, 2026-10-20: main 5857a62a, which carries #5424, merged locally with #5419 at cd38b6f7; fresh targets, opt-1, `--test-threads 1`, three interleaved rounds):

| row | pre-4954 | main 5857a62a | main + #5419 |
|---|---|---|---|
| shelling | 32.7 s | 62.8 s (1.92×) | **54.3 s (1.66×)** |
| r1 | 11.5 s | 15.6 s (1.35×) | **15.4 s (1.33×)** |

Both rows are under the 2× target. The remaining r1 lever, a cheaper `clears`, is not worth a unit at 1.33×, and has no sound one-piece path (see the closed adaptive-split item).
