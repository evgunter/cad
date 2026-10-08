# PR 4289 review 3 probes (scratch, not for merge)

- `r3_bound.py`: `in_sector` levers only at the read point's reach. Cases
  2k+1, 2k+2 move a 1 mm bound's far point by 3e-10 / 6e-10 m (inside
  the 1e-9 zero band); head decides the original `Out` (or `In`) where
  the moved cone's exact class is the opposite.
  `python3 r3_bound.py && ./run.sh <bin> head bnd 1e-9`.
- `mut.py` + `evalmut.sh`: mutants Xsign, Xnoabs, Lreach, Larm0, ElseS,
  EscS, NoRound, Aon, BndLever against the unit rows, `cone_fuzz` and
  the review fuzz families.
- `errsample.py`: new cs refusals (head Err, 2f67fe0d decided) sampled
  and read under band-sized moves of every far point.
- `offplane.py`: cones whose bound far points lie off their own plane.
- `cmp_scaled.py`: review 1's `cmp.py` with its in-band allowance at 4ε.
- The germ oracle row takes review 2's crown scene (`RV2_ONLY=1`) and
  `R3_FORCE_FIN` / `R3_FORCE_DENT` to build the fin / dented crown at
  any ε.
