---
id: a-cut-cell-nets-quotients-cost-the-walk-eight-times-its-pre-4954-time
kind: issue
title: a cut cell net builds both axes' Bernstein quotients per read, so the SSI walks run 8-15x their pre-4954 time
status: open
opened: 2026-10-13
---


## Measured

The quotient bound of #4954 (`geom_core::spline::derivative::cell_quotients`)
made the SSI enclosures' derivative readings per-piece Bernstein
quotients. #4954 measured them below; the whole-cell memo in
`geom_brep::ssi::enclose`'s `NurbsBoxes` (`CellNets`) took back the
part that was the same cell rebuilt. Local, 4 cores, ci profile, one
test thread:

| commit | `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit` | `r1_lane0_e2e::the_f64_seam_answers_every_public_door` |
|---|---|---|
| 092b3022^1 (before #4954) | 35.3 s | 12.6 s |
| 092b3022 (#4954) | 1692 s | 608 s |
| 092b3022 + the whole-cell memo | 284 s | 193 s |

What is left is the **cut** nets: `CellNet::cut` (enclose.rs) runs
`quotients_of` → `cell_quotients` on every read of a window that cuts a
span. gdb samples of the memo'd r1 row put 13 of 17 under that call,
reached from `boundary::stretch_within` / `side_stretch` →
`NurbsBoxes::deriv_box`. The four-variable walk's
`PairBoxes::jacobian_read(cut = true)` reaches it too.
- Over the r1 row, 80,000 cut builds hit 59,306 distinct
  `(net, su, sv, cut window)` keys, so a cut-net memo saves at most
  about a quarter.
- Each build computes both axes' `N/W²` on 2×2 pieces, but
  `deriv_box` reads one axis.
- The pre-#4954 cut reading was the pair-term hull, O(n) interval
  operations.

## Levers, none taken here

- **An axis selector on `cell_quotients`.** At most 2× on the cut path.
  It changes no reading.
- **Constant factors in `derivative::product`.** It recomputes
  `pieces::binomial` (a u128 loop) four times per term and accumulates
  through `Option`. A binomial table yields the same `f64` values, so it
  is bit-identical. Expect 1.5–2× at most.
- **Read the cheap pair-term hull first and build the quotients only
  where it does not decide.** This changes which reading a meet
  returns, so it is a design call on #4954's statement.

None of them alone reaches the pre-#4954 cost. Which trade to make,
and whether the 8–15× is accepted as the price of the tighter bound, is
the call of NURBS, the program that owns the statement.
