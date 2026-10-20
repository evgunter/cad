---
id: an-adaptive-block-piece-split-reads-one-piece-where-the-hull-decides
kind: issue
title: BLOCK_PIECE_SPLITS 2 → 1 takes 36% off shelling and 24% off r1 but loses a 1e-9 slope decision; split adaptively: one piece where the hull decides, two where it does not
status: closed
opened: 2026-10-14
closed: 2026-10-19
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

## Closed: measured, not worth a second schedule (2026-10-19)

Measured after #5123 and #5391 (`NurbsBoxes::deriv_box`'s cut is the
restriction, then a 2×2 halving of the restricted quotient). gdb
sampling on r1 puts 25 of 34 samples in the cut: 14 restricting, 11
halving. So reading one piece could still pay, where it is sound.

**Adaptive (one piece, two where it does not decide) costs more than it
saves.** Instrumented at the two hot yes/no doors on r1, shelling and
`m5_pr7_ssi`, with both schedules read at every call:

| door | reads | one piece vs two |
|---|---|---|
| `boundary::stretch_within`'s `clears` | 38,224 | 38,144 false at both, 80 true at both |
| `boundary::beyond_reach` | 7,491 | the same answer at every read |

`clears` is false 99.8% of the time at both schedules, so "one piece
first" would read both nearly always.

**A fixed one-piece schedule is sound only at a door that skips work.**
`beyond_reach` is one: it proves no zero of the side's cover lies within
ε, and the full read it hands on cannot certify the contrary. `clears`
is not: `Reading::Within` is "`|φ| ≤ ε` along it, … and no piece clears"
(`crates/geom-brep/src/ssi/boundary.rs:1021`), and `one_arc` reads it
as the certificate that a side's piece reaches the carrier's end
("a point the boundary pass reads not clear", `one_arc.rs:625`, mapped
at `:663`) or that two stretches link ("reads no piece of the overlap
clear", `one_arc.rs:721`, mapped at `:737`). A wider box that missed a
`Clear` would hand on an `Ok` whose certificate was never established.

**What `beyond_reach` alone buys.** With `clears` at two pieces and
`beyond_reach` at one (`nurbs/adaptive-block-split`, `d8193ba7`, not
merged), every `limb_three` decision on shelling, r1, `m5_pr7_ssi::`
and `nurbs_nurbs_section::` read as main `d989dab1` bit for bit, and
three interleaved rounds on fresh targets measured:

| row | main `d989dab1` | `beyond_reach` one-piece |
|---|---|---|
| shelling | 74.5 / 74.0 / 73.8 s | 71.5 / 72.2 / 71.8 s (−3%) |
| r1 | 17.8 / 17.8 / 17.4 s | 17.6 / 18.2 / 18.3 s (noise) |

Not worth a second piece schedule and a second derivative-box door.

**The remaining r1 lever is `clears`** (38k reads at r1). It needs a
cheaper reading that keeps the two-piece box's resolution: for instance
the step's arithmetic (`Interval::mul_nonneg`, #5163) or a sign test
that does not form the whole box.

**`clears` is not worth a unit (2026-10-20).** A one-piece `clears`
is unsound by definition rather than by cost: `Within` is "no piece
clears" (`boundary.rs:1021`; `one_arc.rs:625/663/721/737` read it that
way), so fewer pieces changes the answer, not only its price. What is
left at r1 is 1.33× pre-4954 (15.4 s against 11.5 s on main 5857a62a
plus #5419), below the bar for a unit of its own.
