(iso-run-density implementer) The ε 1e-6 skip: the cells CAN be re-posed. The change is held locally as cfacd1584f on top of the frozen a79bf304d5, NOT pushed.

Two corrections first:
- At the head, FIVE cells skip at 1e-6, not four. The millimetre h 1.9e-3 cell skips too.
- Moving the hole, or changing the stub height, does not help. I swept the centre over 8.5–12.5 columns (phases 0.25/0.5/0.75) and dz over 0.02–0.05. Every pose refuses at exactly the same margin.

What the refusals actually are:
- **Unit pipe:** the pierce margin is the hole's sagitta, w²/4r (1.58e-6 at w = 0.04 col; 3.55e-6, 6.32e-6 and 9.87e-6 at 0.06, 0.08 and 0.1). It clears the band from w ≈ 0.12 col.
- **Millimetre pipe:** what refuses is the column-reading probe (a hole 0.01·r wide on the mm pipe), not the two cuts. The cuts themselves build at 1e-6.

The fix:
1. Read the column off the unit pipe at δ/r. This is scale-invariant: n is identical to the old probe on every cell where both build (16 and 50).
2. Make the stub half-width a fixed share of the radius, w = 0.008·r, instead of 0.04·column·r. At δ/r = 1e-2 that is the same hole as before (0.041 col); on the unit pipe at δ 1e-3 it is 0.127 col.

The diff:
- `two_holes` returns `Body` again, and `try_less` is gone.
- The row has no skip branch.
- The doc gives the two margins as the reason for the width.

Result: all 10 cells build, mesh and assert at ε 1e-9, 1e-6 and 1e-12. `inserted` is unchanged (4 on the 8 refusing cells, 0 on r 10), and every Euler identity holds.

Checks: the 3 rowless rows pass at all three ε, the crossing-rods row passes at 1e-9 and 1e-6, and mesh clippy, fmt and run-gates are clean.

Waiting for the reviewers' findings. Push nothing until then.
