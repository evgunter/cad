---
id: a-solids-charts-as-a-move-set-is-spelled-per-sweep-test-file
kind: issue
title: a solid's charts as a ChartMove set is spelled per sweep test file
status: open
opened: 2026-09-24
priority: P4
cost: D
---


## Finding

- **Where**: `crates/sweep/tests/*` (S-TCOST's and S-TINT's ground;
  this program announces by seam, as for every row on its slate).
- **Confidence**: sure that the function-shaped members below are
  copies — several are byte-identical modulo a parameter name. The
  inline members are a **candidate list**, not dispositioned.
- **Raised by**: the X4 re-read of
  `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md`'s
  fold, 2026-09-24. That row's `moves_of` left `topo` with the fold;
  the same function lives three times in `sweep/tests`, and it is one
  member of a larger class there.

**The thing spelled n times**: group a solid's (or a body's) faces by
surface key, in arena order, and turn each group into a
`topo::ChartMove { faces, distance }`. The offset doors take exactly
this shape, so every suite that drives them builds it.

### Candidate list, merge base `6db5b87f2`

Function-shaped (`git grep -n "fn .*-> .*ChartMove"`, no path
argument — 10 hits, all in `sweep/tests`):

| function | files |
| --- | --- |
| `moves_of(body, solid, d)` over `charts_of` | `shell10_r1_probes.rs`, `shell10_r2_cost.rs`, `shell10_r2_dump.rs` |
| `chart_moves(body, d)` — the grouping written inline | `common/cone_nappe.rs`, `shell6_r1_probes.rs`, `shell6_r2_probes.rs` |
| `hollow_moves(body, t)` | `shell7_common.rs`, `shell7_dump.rs`, `spiric_rim.rs` (generic over `T`) |
| `inward_moves(body, solid, t)` | `shell8_r2_probes.rs` |

The grouping itself, `fn charts_of(body, solid) -> Vec<Vec<FaceKey>>`
(`git grep -n "fn charts_of"`): `shell8_common.rs` (the `pub(crate)`
one), and private copies in `shell8_r1_probes.rs` and
`shell8_r2_probes.rs`.

Inline (`git grep -n "ChartMove {"`, 36 hits outside
`topo/src/offset_together.rs`, read with four lines of leading
context): the ones mapping a `charts_of` result straight to moves are
`shell10_r2_probes.rs` ×2, `shell10_scoped_walks.rs`,
`shell8_multi_solid.rs` and `shell8_r1_probes.rs` ×2. `verbs_shell.rs`
(~:1744) is a **closure-shaped** member the function grep cannot see:
a `charts` closure that re-spells the grouping and a `move_all`
closure over it. The other hits (`sf2a_*`, `sf2b_*`, `torax_*`,
`shell7_*`, `spiric_rim.rs`) were not read past that window; some
build moves with per-group offsets or senses and may not be members.

**What the instruments cannot see**: a closure (one found, above, by
reading rather than by pattern), a move set built through a helper
whose return type is not spelled `ChartMove`, and a grouping keyed on
something other than `.surface` that is still the same partition. `topo/src/shell.rs`
builds moves in `src` twice (~:1149, ~:1448); whether either is a door
the suites could call instead of copying is the first question for a
unit here, and was not checked.

`topo`'s own `offset_together::scope_walks::moves_of` is a member too,
and cannot share a home with these — `topo`'s in-crate tests cannot
reach `sweep`'s test tree — unless the home is a `topo` door.
