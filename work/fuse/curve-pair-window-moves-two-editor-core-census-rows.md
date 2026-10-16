---
id: curve-pair-window-moves-two-editor-core-census-rows
kind: issue
title: Since PR 5227's curve-pair window, boss_union's census disagrees across scalars and cut_cylinder's census gains two edge overlaps
status: open
opened: 2026-10-16
---


Found on PCERT's PR 5225 (`pcert/azimuth-placement`), whose CI tests its
merge with main. Two editor-core slow-set rows went red on that merge,
and both are main's own.

## The rows

- `crates/editor-core/tests/m10_di_dual_corpus.rs`'s
  `assemble_census_verdicts_match_f64_at_interval`: `boss_union`'s
  census door disagrees across certifying scalars. At `f64`, `assemble`
  accepts (`None`). At `Interval` it refuses with six findings, each
  "no declaration: whether two parts of the body touch is undecided.
  There is no way through: this is a kernel defect or a damaged file;
  report it".
- `crates/editor-core/tests/perf12_census_goldens.rs`'s
  `the_at_rest_census_verdicts_are_goldened_bit_exact`: it drifts at
  line 199 of `tests/golden/perf12_census_1e-9.txt`. `cut_cylinder`'s
  census goes from 12 findings to 14, adding two `UnrecordedContact`s:
  - `EdgeEdgeOverlap { a: EdgeKey(5v1), b: EdgeKey(11v1) }`, witness
    `(0.0, -0.5, 0.5)`;
  - `EdgeEdgeOverlap { a: EdgeKey(6v1), b: EdgeKey(12v1) }`, witness
    `(0.0, 0.5, 0.5)`.

  Both read margin `Value(0.0)`. They are the two halves' coincident
  arcs of the cut, which already meet at the `VertexVertex` contacts the
  golden lists, so the new findings look like a true contact the census
  used to miss. Whether to re-bless is FUSE's call.

## Evidence

- Both rows fail on `origin/main` at `042e4e4fe9` alone. PCERT's delta
  verifier ran them there.
- Both pass at PR 5225's `d9676b667b` without main, and on its earlier
  merge with main from before PR 5227.
- They arrive with PR 5227's merge `37edb540a9`
  (`fuse/curve-pair-reader-c3`: tier 3′ reads every curved edge pair
  through `curve_pair::meet`, and the window counts its turns from the
  span).
- First seen in CI run 38940569883 (PR 5225's head `d9676b667b`, merged
  with main).

Neither row is in the default set, so main's own CI on PR 5227 did not
run them unless the change filter seeded editor-core's slow set.
