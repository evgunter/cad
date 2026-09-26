---
id: a-solids-charts-as-a-move-set-is-spelled-per-sweep-test-file
kind: issue
title: a solid's charts as a ChartMove set is spelled per sweep test file
status: closed
opened: 2026-09-24
closed: 2026-09-26
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

**Fix pass (2026-09-26, PR #3284).** The private `charts_of` copies in `shell8_r1_probes` and
`shell8_r2_probes` are gone; both import `shell8_common::charts_of`.

## Closed (2026-09-26, PR: batch 6)

**One home: `crates/sweep/tests/common/charts.rs`**, by
`tests/common/mod.rs`'s narrowest-home rule — every consumer is a
`sweep` suite, and `cone_nappe` (itself in `common`) was one. It holds
the partition (`charts` over a body, `charts_of` over one solid, one
private `group` under both, generic in the scalar so the `Interval`
suites share it) and the move sets built from it (`moves_by`, a uniform
distance; `moves_inward`, `shell`'s sense rule; `hollow_moves`, the two
composed over a whole body).

**The census, re-taken at the merge base `032999ff2`** (every form `git
grep` with no path argument): 11 function-shaped members, not the 10
this row listed (`torax_axial::hollow_moves` was the one missed), plus
the grouping `shell8_common::charts_of`, plus 11 inline or
closure-shaped members. Every one is folded:

| member | disposition |
| --- | --- |
| `common/cone_nappe::chart_moves` | folded; its three `shell6_nappe_home` calls read `moves_by(charts(..), d)` |
| `shell6_r1_probes::chart_moves`, `shell6_r2_probes::chart_moves` | deleted — both were DEAD copies, called by nothing |
| `shell7_common::hollow_moves` (and its four importers), `shell7_dump`, `torax_axial`, `spiric_rim` (generic) | folded onto `common::charts::hollow_moves` |
| `shell10_r1_probes`, `shell10_r2_cost`, `shell10_r2_dump` `moves_of` | folded: `moves_by(charts_of(..), d)` |
| `shell8_r2_probes::inward_moves` | folded: `moves_inward(body, charts_of(..), t)` |
| `shell8_common::charts_of` | MOVED into the home; its eight users re-pointed |
| inline: `shell10_r2_probes` ×2, `shell10_scoped_walks`, `shell8_r1_probes` ×2 | folded |
| closure: `shell8_multi_solid::moves`, `verbs_shell`'s `charts` + `move_all` | folded |
| inline grouping: `sf2b_head`, `sf2b_r1_probes`, `torax_interval` | folded (`sf2b_head`'s first-face sense is `moves_inward`'s rule; `torax_interval`'s `iv(±0.05)` is `hollow_moves` at `iv(0.05)`, bit-exact since `Interval::from_f64` embeds a point and negation of a point is exact) |
| `sf2a_r1_head::charts` (grouping + each plane) | the grouping folded; the plane read stays, over `charts` |
| `sf2a_r2_probes` ×6, `sf2a_r2_interval_probe` ×2 | **not members**: one move per FACE, sign read off each plane — no grouping |
| `shell10_r2_probes::chart_of` | **not a member**: one chart for one face; kept, with the `NOT \`common::` marker the module's rule asks for |
| `topo::offset_together::scope_walks::moves_of`, `topo/src/shell.rs`'s `chart_groups` / `group_by_chart` | **out of reach** of a `sweep` test home; filed as `the-chart-partition-has-a-topo-src-home-and-a-test-home` |

**The row's first question, answered**: neither of `topo/src/shell.rs`'s
two move-set builders is a door a suite could call — both build over
the private `chart_groups` / `group_by_chart`. Whether that partition
should be a public `topo` door is the filed row's question.

**Instruments and their blind spots.** (1) `git grep -n "fn .*ChartMove"`
(function-shaped; misses closures and inline sets); (2) `git grep -n
"ChartMove {"` read in full, every hit (misses a set built through a
helper not spelling `ChartMove`); (3) the grouping NEEDLE, not the
type — `git grep -nE "\*s == f\.surface|\*k == key|\.surface ==
f\.surface|== data\.surface"`, which found the `sf2a_r1_head` grouping
that builds no `ChartMove` itself; (4) a map-keyed grouping
(`entry(….surface)`, `group_by`, `chunk_by`), which found only
`topo/src/census.rs`'s curved-face `BTreeMap` (keyed order, a different
partition order, `topo`'s ground). What none can see: a partition
keyed on something other than `.surface` that is still the same
partition.

**Measured by plant** (harness restores each file's pre-plant bytes
and checks the tree hash against its pre-plant state; filter = every
suite importing the home, 169 rows):

| plant | direction argued | red |
| --- | --- | --- |
| C2 `panic!` in `group` (reach control) | no answer satisfies it | **52** — every importer's reaching rows |
| P5 `moves_inward` sign flipped | hollowing becomes thickening: a different answer, not a relaxation | 18 |
| P6 `moves_by` distance doubled | a different magnitude | **1** (`shell6_nappe_home::both_doors_mint_the_turned_offset_on_both_nappes`) |
| P7 group order reversed | order only | **0** |
| P8 no grouping (each face its own move) | splits every multi-face chart | 25 |

P7 against C2 is item 19's pair: the partition is reached by 52 rows and
its ORDER is asserted by none, so the home states the order without
claiming anything depends on it. P6's single red is a coverage fact
about the uniform-distance rows — they assert what builds or refuses,
not how far — and not a defect of the fold. 13 of the 52 reached rows
red under none of P5/P6/P8; the cube and chamfered-cube rows among them
wear one plane per face, so P8 is a no-op there by construction.
