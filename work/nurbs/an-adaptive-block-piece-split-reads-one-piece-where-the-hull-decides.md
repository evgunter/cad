---
id: an-adaptive-block-piece-split-reads-one-piece-where-the-hull-decides
kind: issue
title: BLOCK_PIECE_SPLITS 2 → 1 takes 36% off shelling and 24% off r1 but loses a 1e-9 slope decision; split adaptively: one piece where the hull decides, two where it does not
status: open
opened: 2026-10-14
---


## Measured

Measured on main `d6396bc0` (NURBS build A, 2026-10-14), with a local edit to
`BLOCK_PIECE_SPLITS` in `crates/geom-brep/src/ssi/enclose.rs`. Setup: opt-level 1,
`--test-threads 1`, a fresh `CARGO_TARGET_DIR`, three interleaved runs, medians.

| config | `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit` | `r1_lane0_e2e::the_f64_seam_answers_every_public_door` |
|---|---|---|
| main, `BLOCK_PIECE_SPLITS = 2` | 137.0 s | 45.2 s |
| `BLOCK_PIECE_SPLITS = 1` | 87.3 s (−36%) | 34.2 s (−24%) |

At 1, `cargo nextest run -p geom-brep` fails two rows (1444/1446):

- `ssi::enclose::tests::a_block_is_read_on_its_pieces`: "the strip's z box
  [-1.6e-8, -5.322666666665176e-9] against the true [-1.6e-8, -7.984e-9]".
- `m5_pr7_ssi::a_flat_wall_whose_chart_path_bends_answers_as_the_plane_it_is`:
  "μ 0.2 at ε 1e-9: the v = 0 side's slope margin 5.333331937376734e-9, expected
  8e-9 inside the band".

The second is a lost decision, not a moved pin. The constant's own doc records it:
one piece reads that wall's slope at two thirds of its least, 5.3e-9 against
8e-9, which falls inside the band at ε = 1e-9.

## Shape of the fix

This is the converged plan's step 5. Read a block on one piece first, and split
to two only where the one-piece hull does not decide what its caller asks of it
(the derivative box's sign or floor). The two rows above are the guard: both
must stay green, along with the rest of `geom-brep` at the three ε. Measure the
saving against `BLOCK_PIECE_SPLITS = 2` on the rows above, after #5123's
restriction cut lands. These numbers are against the step-2 cut.
