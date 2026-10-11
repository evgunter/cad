---
id: the-arc-loft-thread-count-digest-is-red-on-main
kind: issue
title: mass_props_are_thread_count_invariant's arc_loft_1e9eps line no longer matches its committed digest on main, at every eps
status: closed
opened: 2026-10-11
priority: P1
cost: E
closed: 2026-10-11
pr: 4684
branch: nurbs/main-slow-set-reds-since-4553
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

## Also moved (PR 4732's CI, ε = 1e-9)

Seen on `cleave/chord-end-sample` at `090278cdb`, whose diff changes no
kernel code. CI run 38159733248, job "test (the slow set of the crates
this diff seeds)". Besides `arc_loft_1e9eps`, two `Sym` rows of the same
digest move. Their decisions and shapes do not:

| line | committed | got |
|---|---|---|
| `sym_arc_loft validate_geometric` | `sz=48 num=604 frozen=1244` | `sz=40 num=612 frozen=3732` |
| `sym_thin_strip validate_geometric` | `sz=48 num=612 frozen=1268` | `sz=36 num=624 frozen=2792` |

## Closed (NURBS, PR 4684)

Re-cut at all three ε on `nurbs/main-slow-set-reds-since-4553`, after
merging main at `18555d6eca`. The movers were bisected:
- **PR 4553** moved `v`, `a` and the pads. It put the rational patch
  lane's knot refinement on the certification ring, and the new
  enclosures nest inside the old.
- **`8054ec236f`** (PR 4726, the skin in `T`) moved the pads again and
  the `Sym` `frozen` column. It also undid PR 4708's `sz=40` / `sz=36`
  movement in the table above.

The full account is in
`work/nurbs/main-slow-set-reds-on-two-rows-since-4553.md` and in
`expected()`'s re-cut log.
