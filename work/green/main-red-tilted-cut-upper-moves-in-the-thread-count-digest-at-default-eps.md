---
id: main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps
kind: issue
title: main red: tilted_cut_upper's volume, area and pads move by a few ulps in the thread-count digest at the default eps, since 692f50942c
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-28
---


## Finding

Found by PCERT E1b's suite round (PR #5797), and red on main itself.
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`'s
`the_walk_matches_the_serial_golden_at_four_threads` fails at the
default ε on one row only, `tilted_cut_upper`:

| field | got | golden |
|---|---|---|
| v | `3fd921fb54442d1b` | `3fd921fb54442d18` |
| a | `40096d2e7648c6cb` | `40096d2e7648c6c9` |
| vpad | `3e81270c3c2aaaac` | `3e812c48b2000000` |
| apad | `3ea9ba925a200000` | `3ea9c26d0ae00000` |

The verdict hash (`n=10 h=c5311dd0c395a321`) and every other row,
including the sym-session counts (`sym_thin_strip` frozen 168), match
the golden. Volume and area move by 3 and 2 ulps, and the pads shrink
slightly.

Red on main `be2b7a139c` and on `2f9e6f9e39`, with output identical to
the PR's tree. The digest held at all three ε in E1b's earlier round
on a tree carrying main `692f50942c`, so the move landed between
`692f50942c` and `be2b7a139c`; not bisected. The 1e-12 sibling is
`tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12` (closed).

## Closed (2026-10-28)

Already fixed on main before this row was dispatched; nothing to change.

- **The merge.** PR 5734 (`flux/cylinder-sliver-exact-bracket`, merge
  `ee4a094e`), which sits between `692f50942c` and `be2b7a139c` on
  first-parent main. QUAD bisected it on
  `work/quad/tilted-cut-upper-digest-moved-by-5734` (closed): `ee4a094e~1`
  passes, `ee4a094e` fails with exactly the bits in the table above.
  5734 reads the cylinder lane's `(cos, sin)` at each trim end from
  `quad::trig_at_limit` at the exact limit instead of from the endpoint
  vertex padded by `ε / radius`, so the trig brackets narrow, the pads
  narrow and their midpoints move by 1–3 ulps.
- **The decision.** A deliberate kernel change whose pads only shrank
  and whose new brackets still hold the closed form (`V = π/8`,
  `A = π/4 + π/2 + π/(4 cos 0.3)`) at every ε: a re-bless. PR 5818
  (merge `f78c6479`) re-cut the `tilted_cut_upper` line at all three ε
  (1e-6: pads only; 1e-9: v, a and pads; 1e-12: a and pads).
- **Evidence it holds.** The 1e-9 golden now carries the "got" bits of
  this row. Locally on main `9809965b`, `cargo nextest run -p sweep
  --test all -E 'test(mass_props_are_thread_count_invariant)'` passes
  4/4 at the default ε (the walk row in 242 s). Nightly 74 (run
  40637814390, main `a1cfd1cfdb`, after 5818) lists no
  `mass_props_are_thread_count_invariant` row among its 141 failures at
  any of the three ε.
