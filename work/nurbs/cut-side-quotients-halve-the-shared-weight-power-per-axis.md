---
id: cut-side-quotients-halve-the-shared-weight-power-per-axis
kind: issue
title: The cut side's quotients still halve the shared W² once per numerator
status: open
opened: 2026-10-19
---


**What.** #5399 gave a cell's quotients one halving of each shared weight power (`shared_pieces`, `crates/geom-core/src/spline/derivative.rs:912`). The cut side was left out by agreement with the cut lane:
- `CellFirstQuotients::pieces` (`derivative.rs:1347`) still halves the restricted `W²` once per numerator;
- so does the both-axes `cut_pieces` (`derivative.rs:1410`).

That agreement was recorded only in the closed `halving-repeats-the-shared-weight-powers` item and #5399's body. #5399's FULL review raised it (MINOR M1, https://github.com/evgunter/cad/pull/5399#issuecomment-6279030575).

**Done when:**
- both cut-side readers pair one halving of `W²` with `S_u`'s and `S_v`'s numerators through `shared_pieces`, bit for bit;
- a row pins it on the halvings themselves. `WEIGHT_HALVINGS` counts `shared_pieces` calls today (`derivative.rs:918`), and the review's M2 mutant, which re-halves the denominator per numerator inside `shared_pieces`, passes every row. Count where the halving happens instead.

**Also in this PR** (from #5433's review, https://github.com/evgunter/cad/pull/5433#issuecomment-6282863390, and #5424's review):
- Three test-only f64 binomial folds still exist, at `carrier_net.rs:491`, `derivative.rs:2063` and `analytic_rung3_certificate.rs:664`. Point them at `pieces::binomial`.
- `polynomial_area_pieces` (`derivative.rs:1256`) still writes the `halving_counts` pair by hand.
- `shared_pieces`' any-None guard is unreachable today, and the code after it silently returns an empty list. Make it fail loud.
- If `split_restrict` is touched: the "[0,1]" comment is inexact (`s.hi` reaches 1+2⁻⁵² when t is tiny), and the thin-step row should draw wide end weights and a tiny t.
