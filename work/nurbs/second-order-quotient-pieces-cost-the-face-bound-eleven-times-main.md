---
id: second-order-quotient-pieces-cost-the-face-bound-eleven-times-main
kind: issue
title: The second-order quotient pieces cost the rational face bound 11-15x main's: per-piece restriction, half of it interval-by-f64 multiplies
status: closed
opened: 2026-10-13
closed: 2026-10-14
branch: nurbs/quotient-products-once
---


## Measured

The second-order quotient statement of PR 4984
(`geom_core::spline::derivative`, "The second order") reads every
rational `patch_bound` cell as exact dyadic pieces of `M/W³` at degree
`3p`. `patch_cells` is the shipped tessellation sizing input
(`mesh::nurbs_cert::nurbs_cell_bounds`), so every rational face pays it
once per tessellation. Local, opt-level 1, `--test-threads 1`, three
runs each, after 4984's per-cell table cache, per-`u`-piece cut and
shared weight powers (`Cut`, `cut_both`):

| row | main (`6059d794`, `c16eaf57` for the last) | 4984 (`1c58397c`) |
|---|---|---|
| `nurbs_cert::tests::rational_face_bound_dominates_sampled_hessian` | 0.073–0.086 s | 0.88 s |
| `cert10r2_probes::probe1_signed_reading_encloses_random_rational_sweep` | 0.66–0.78 s | 9.9 s |
| `nurbs_cert::tests::every_cell_bound_dominates_its_own_sampled_hessian` | 0.53–0.59 s | 1.18–1.38 s |

Callgrind on the face-bound row (before the per-`u`-piece cut and the
shared weight powers, which took it from 1.6 s to 0.88 s): 86% of
instructions under the per-piece restriction (`Grid::restrict` →
`pieces::combine`), and about half of all instructions in
`DInterval`'s `Mul`. `combine` multiplies each interval coefficient by
an exact, nonnegative `f64` piece weight lifted to a point interval,
so each step pays a general interval product (four products, outward
rounding) where two rounded products would do.

## Levers

- **A scale-by-`f64` fast path** in the interval arithmetic: an
  interval times a nonnegative exact `f64` is `[↓(lo·w), ↑(hi·w)]`.
  `interval-transcendentals` and `crates/geom-core/src/interval.rs` sit
  in no program's territory today (`work.py territory`), so this needs
  an owner. `pieces::combine` is generic over `Real`, so the fast path
  needs a door it can reach without a new `Real` bound
  (`scripts/gates/no-extra-real-bounds.sh`).
- **Shared restriction with build C's cut-net levers.**
  `a-cut-cell-nets-quotients-cost-the-walk-eight-times-its-pre-4954-time`
  is the same cause one order down: unit 1's `Homogeneous::restrict`
  re-forms `piece_weights` per grid (`Grid::on_piece`) and restricts
  every grid separately. 4984 moved the second-order path to a per-cell
  `Cut` table and a per-`u`-piece cut; one restriction door, cached by
  a pure key (degree, count), would serve both, and any multiply fast
  path lands in both at once.
- **Fewer pieces.** `RATIONAL_CERT_SPLITS` (16) was priced against the
  retired refinement's hull width. The quotient's ratio spread shrinks
  faster, so a smaller count may meet the same sizing, but the count
  moves cell granularity, the census and the offset goldens, so it is
  a measurement before it is a change.

## The integral arm (2026-10-13)

PR 4984 also gave the integral arm an area floor, `A_u × A_v` over one
per span cell (`polynomial_area_pieces`), which only `offset_meters`
reads. Against `a71403e3` (main before 4984), `--test-threads 1`,
opt-level 1:

| row | before 4984 | after |
|---|---|---|
| `offset_fit::the_bumpy_patch_certifies_a_micron_offset_below_a_nanometre` | 2.16–2.33 s | 2.89–3.09 s |
| `offset_fit::a_round_whose_bound_rose_takes_the_both_directions_step` | 11.9–12.2 s | 13.7 s |

That is with `nurbs/remaining-rational-readings`' one-piece shortcut,
which skips the identity cut. Callgrind on the bumpy row puts the floor
at 24% of the run: the `A_u × A_v` product (12.7%) and the per-cell
B-spline-to-Bernstein rows (`Axis::row` → `span_row`, about 7%). Both
are `derivative.rs` internals (build C's ground). The tessellation path
(`patch_cells` in `nurbs_cert`) forms the floor too, on raw knot spans,
though it never reads it.

## Closed (2026-10-14)

`nurbs/quotient-products-once` forms every quotient's products once
over the cell or span and halves them into pieces by de Casteljau at
½: sums unscaled, one exact `2⁻ʳ` scale per output. `Cut`,
`cut_both`, `piece_weights`, `pieces_exact`, `piece_count`, `combine`
and `restrict` are gone, the curve meters (`speed_pieces`,
`accel_pieces`) included, and with them the exactness ceiling: every
degree reads the count its schedule asks for. Local, opt-level 1,
`--test-threads 1`, against main `72ad4910` (5047 in):

| row | main | branch |
|---|---|---|
| `rational_face_bound_dominates_sampled_hessian` | 0.87 s | 0.45–0.50 s |
| `probe1_signed_reading_encloses_random_rational_sweep` | 10.1 s | 6.8–7.0 s |
| `every_cell_bound_dominates_its_own_sampled_hessian` | 1.38 s | 0.93–0.99 s |

The face bound stays about 6× `c16eaf57`'s. Its remaining levers (an
exact-scale door in the interval arithmetic, fewer pieces) are
`interval-scale-by-an-exact-f64-pays-a-general-product`
(`work/issues/`).
