---
id: cut-side-quotients-halve-the-shared-weight-power-per-axis
kind: issue
title: The cut side's quotients still halve the shared W² once per numerator
status: closed
opened: 2026-10-19
closed: 2026-10-20
---


**What.** #5399 gave a cell's quotients one halving of each shared weight power (`shared_pieces`, `crates/geom-core/src/spline/derivative.rs:912`). The cut side was left out by agreement with the cut lane:
- `CellFirstQuotients::pieces` (`derivative.rs:1347`) still halves the restricted `W²` once per numerator;
- so does the both-axes `cut_pieces` (`derivative.rs:1410`).

That agreement was recorded only in the closed `halving-repeats-the-shared-weight-powers` item and #5399's body. #5399's FULL review raised it (MINOR M1, https://github.com/evgunter/cad/pull/5399#issuecomment-6279030575).

**Done when:**
- both cut-side readers pair one halving of `W²` with `S_u`'s and `S_v`'s numerators through `shared_pieces`, bit for bit;
- a row pins it on the halvings themselves. `WEIGHT_HALVINGS` counts `shared_pieces` calls today (`derivative.rs:918`), and the review's M2 mutant, which re-halves the denominator per numerator inside `shared_pieces`, passes every row. Count where the halving happens instead.

**Closed** (`nurbs/cut-side-shared-pieces`). `CellFirstQuotients::pieces`
and the both-axes `cut_pieces` pair one halving of `W²` with both
numerators through `shared_pieces`; a one-axis cut keeps its one
halving. `WEIGHT_HALVINGS` now counts in `grid_pieces`, where a
one-channel grid (a weight power) is halved. The row
`a_cut_reader_halves_its_weight_power_once` pins one halving per cut
read and none for a cut asking no axis; M2 (re-halving the denominator
per numerator inside `shared_pieces`) turns it and
`a_cell_halves_each_weight_power_once` red.
