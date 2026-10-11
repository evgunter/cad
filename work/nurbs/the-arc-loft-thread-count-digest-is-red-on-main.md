---
id: the-arc-loft-thread-count-digest-is-red-on-main
kind: issue
title: mass_props_are_thread_count_invariant's arc_loft_1e9eps line no longer matches its committed digest on main, at every eps
status: open
opened: 2026-10-11
priority: P1
cost: E
---


## Measured

Found by FLUX's PR 4573, whose diff seeds the sweep slow set on CI;
reproduced on `origin/main` alone at `90d8dba1d` (a main-only worktree):

```text
cargo test -p sweep --test all the_walk_matches_the_serial_golden_at_four_threads
```

fails at `crates/sweep/tests/mass_props_are_thread_count_invariant.rs:471`.
Only the `arc_loft_1e9eps` line moves, at every ε; its verdict count and
hash do not:

| ε | field | committed | main |
|---|---|---|---|
| 1e-9 | v | `4022487ed6a9da05` | `4022487ed6a9d899` |
| 1e-9 | a | `403995a3b53c3c54` | `403995a3b53c3c57` |
| 1e-9 | vpad | `3eb0783c2ee00000` | `3eb0783423f55555` |
| 1e-9 | apad | `3fb5fb0bfc379cc0` | `3fb5fb0bfc12a5e0` |
| 1e-6 | v / a / vpad / apad | `…975e9` / `…e600` / `408ead6b69e00000` / `40f4f65eba153ce0` | `…97631` / `…e5fc` / `408ead5cbab55555` / `40f4f65eb9f13460` |
| 1e-12 | v / a / vpad / apad | `…c656a` / `…920a` / `3cd1af218b000000` / `3e770c62e0defda0` | `…c6487` / `…920e` / `3cd1af1985caaaab` / `3e770c62e0b7ef40` |

The pads shrink (both `vpad` and `apad`, at every ε) and the midpoints
move a few hundred ulps inside them. The line is a rational NURBS loft
measured by the certified quadrature lane, and main's recent NURBS work
(knots and weights in `T`, PR 4608 and the `nurbs/knot-algebra-off-the-ring`
merges) is the likely mover. The table's own rule (at `expected`): the
branch whose change moved it re-cuts it at all three ε, if the new
reading is right, and names what moved.

## Two sym-session rows moved too (INTENT, 2026-10-11)

Reproduced on `origin/main` alone at `9baeb0056e` (ε = 1e-9, four
threads): besides `arc_loft_1e9eps`, the two `validate_geometric`
sym-session rows no longer match the committed digest. Their decision
counts and verdicts hold. Only the split between symbolic zeros,
numeric discharges and frozen nodes moves:

| row | committed | main |
|---|---|---|
| `sym_arc_loft` | `sz=48 num=604 frozen=1244` | `sz=28 num=624 frozen=3706` |
| `sym_thin_strip` | `sz=48 num=612 frozen=1268` | `sz=24 num=636 frozen=3430` |

PR 4684's re-cut covers the `arc_loft_1e9eps` line only, so the row
stays red after it. 20 symbolic zeros now discharge numerically, which
a re-cut alone should not paper over. The likely mover is PR 4703
(NURBS weights and knots in `T`, which reclassed the arc rim's
decisions). Found on INTENT's PR 4669 (the slow set runs there because
its diff seeds sweep).
