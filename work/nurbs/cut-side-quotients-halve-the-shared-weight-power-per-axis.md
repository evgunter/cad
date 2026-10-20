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

**Also in this PR** (from #5433's review, https://github.com/evgunter/cad/pull/5433#issuecomment-6282863390, and #5424's review):
- Done here: the three test-only f64 binomial folds (`carrier_net.rs`, `derivative.rs`'s oracle row, `analytic_rung3_certificate.rs`) read `pieces::binomial`.
- Done here: `polynomial_area_pieces` reads `halving_counts`.
- Done here: `shared_pieces` fills each numerator's list or refuses the whole reading, so no slot is left at a default and no guard is unreachable.
- **Moved to build A's PR 3** of `binomial-bernstein-product-and-blossom-have-two-homes-each`, which touches `split_restrict`. Two items move: the "[0,1]" comment (`s.hi` reaches 1+2⁻⁵² when t is tiny), and the thin-step row drawing wide end weights and a tiny t. This PR does not touch `split_restrict`.
