---
id: the-slow-ssi-rows-stay-over-twice-their-pre-4954-time
kind: issue
title: The slow SSI rows stay 3.3-4.8x their pre-4954 time after the quotient unit, against a 2x target
status: open
opened: 2026-10-14
---


## Measured

After the quotient unit (#5047, #5081, #5123 with #5059's midpoint), at
opt-level 1, `--test-threads 1`, sequential:

| row | pre-4954 | main `72ad4910` | unit |
|---|---|---|---|
| `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit` | 35.3 s | 237 s | 117 s (3.3×) |
| `r1_lane0_e2e::the_f64_seam_answers_every_public_door` | 12.6 s | 167 s | 60 s (4.8×) |
| `nurbs_cert::tests::rational_face_bound_dominates_sampled_hessian` | 0.073–0.086 s (`c16eaf57`) | 0.87 s | 0.51 s (about 6×) |

The target is about 2× pre-4954 (the CI wall-time lane's). No cache is
added: the quotient fork ruled caches out.

## Levers, in order

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
