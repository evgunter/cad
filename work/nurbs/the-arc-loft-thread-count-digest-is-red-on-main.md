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
merging main. Every mover was bisected on builds that compiled from
source.

**`arc_loft_1e9eps`:**
- PR 4553 moved `v`, `a` and the pads. It put the rational patch
  lane's knot refinement on the certification ring, and the new
  enclosures nest inside the old.
- `8054ec236f` (PR 4726, the skin in `T`) moved the pads again, by at
  most 1.1e-9 of themselves, and `a` by an ulp at 1e-6. The arc
  conversion's `libm` sines and cosines move the body by an ulp, so old
  and new enclosures nest, though not always new inside old.

**The two `validate_geometric` sym rows: deterministic, and every
decision class is back where it was before PR 4703.** These were
measured at ε = 1e-9 at four points:
- pre-4703 `8d66dd7e97^1`;
- 4703 `8d66dd7e97`;
- `cb58e63b8b`, which carries 4708 (the 4708 merge itself does not
  compile sweep's tests);
- this branch's head.

At each point the rows were run at 1 and 4 threads, twice each, with
the Sym block alone and after the f64 block. Every reading at one
commit is identical, so the split is deterministic. The two different
readings above (`9baeb0056e` against `090278cdb`) differ by PR 4708,
which only the second carries.

Per decision class (predicate and outcome; only the classes that move):

| row / class | pre-4703 | 4703 | +4708 | head |
|---|---|---|---|---|
| `sym_arc_loft` `pcurve_iso_boundary` Theorem / NumericZero | 20 / 2 | 0 / 22 | 14 / 8 | 20 / 2 |
| `sym_arc_loft` `pcurve_loop_continuity` Theorem / NumericZero | 2 / 30 | 2 / 30 | 0 / 32 | 2 / 30 |
| `sym_thin_strip` `pcurve_iso_boundary` Theorem / NumericZero | 24 / 4 | 0 / 28 | 16 / 12 | 24 / 4 |
| `sym_thin_strip` `pcurve_loop_continuity` Theorem / NumericZero | 4 / 28 | 4 / 28 | 0 / 32 | 4 / 28 |

What each step did:
- **PR 4703's loss was a regression.** An iso pcurve on a patch's
  boundary sits on the knot domain's end by construction, so the
  residual is exactly zero. Once geom-brep read the carriers' held
  knots as unknowns, it could only discharge those decisions
  numerically.
- **PR 4708** recovered most of them. It lost the
  `pcurve_loop_continuity` theorems (2 and 4).
- **`8054ec236f`** restored every class.
- **At head, all 652 and 660 decisions have their pre-4703 outcomes**
  and `sz=48`. Nothing is left to file on INTENT's slate.

What the re-cut changes is the `frozen` column: 1060 and 1177 at head,
against 3686 and 3414 pre-4703 and 1244 and 1268 committed. Fewer
frozen nodes means the walk built more forms.
