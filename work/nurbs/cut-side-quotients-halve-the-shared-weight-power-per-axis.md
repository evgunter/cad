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
