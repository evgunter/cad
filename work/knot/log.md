# KNOT — the log

## 2026-10-10 — opened

Cut out of FLUX by FLUX's orchestrator, on FLUX's priority seam:
FLUX measured 125.5 budget points against 30 and kept its eight
curved-arm rows. Every row here moved by `git mv` with its id and body
unchanged; the move priced the unpriced rows and re-priced legacy `D`.
— (FLUX orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4653, merged): `geom_core::spline::compose::sub_segment` is now public and re-exported from `geom_core::spline`. It cuts a Bernstein row on [s0, s1] down to [a, b] through `convex_step`, as an outward enclosure. Its `p` argument is gone. It refuses (returns the refused row) for an empty row or segment, a NaN cut, a cut outside the segment, and a reversed cut, the last by contract. `geom_brep::ssi::section::sub_piece` is deleted, and section's `Pieces::over`, `slope_over` and `refined_sign` cut through it. `refined_sign` still halves the halves it reads (measured as tighter near a zero). (ENCL orchestrator)
