---
id: a-valued-anchor-fit-recourse-puts-two-corner-sentences-over-budget
kind: issue
title: The anchor-fit refusal's valued recourse puts a two-corner pair sentence over the 75-word refusal budget
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Found by the refused-corner build unit (PR 6547's ruled sentence).

The anchor-fit refusal's recourse is now valued as Ev ruled it in
PR 6547: "anchor that leg at least S m from the corner, or use a radius
of X m or less", in place of the unvalued "reduce the radius or move
the anchor" (`write_pair_recourse` in `crates/profile/src/path.rs`).
That adds about thirteen words, and a pair sentence that lists a second
corner beside the anchor-fit one goes over the 75-word budget
(`test_utils::refusal::BUDGET`, standard in
`work/chrome/error-and-check-text-overflows-its-region.md`) once a
surface wraps it:

- `ProfileReplay/Path/NoCornerOfPair(two)`: 80 words
  (`crates/editor-core/tests/refusal_concision_chains.rs`, `OVER_BUDGET`);
- `ProfileReplay/Path/NoCornerOfPair(swallow and anchor)`: 83 words
  (same file);
- `Edit/ProfileProgramRefused(Geometry/NoCornerOfPair(two))`: 84 words
  (`crates/viewer/tests/refusal_concision_edits.rs`, `OVER_BUDGET`).

The same unit names the corner a turn refusal is taken at ("its turn
at corner (x, y)", `PathNoCornerReason::TurnWithinTolerance::corner`),
which puts one more row over by two words:

- `ProfileReplay/Path/NoCornerForFillet(turn, leg)`: 77 words
  (`crates/editor-core/tests/refusal_concision_chains.rs`, `OVER_BUDGET`).

Each is admitted at that ceiling, citing this row.

The rows above are synthetic. The kernel's own sentences show the
size of the problem. Measured over grid A's 3 148 anchor-fit refusals
(`crates/profile/tests/common/mod.rs`, `grid_a`) at the default ε,
after the levers were named past the fit gate's band, which changed
their digits but not their word counts:

- 828 sentences are over 75 words, and the longest is 93;
- by word count (five-word bins from the bin's floor): 1 457 at 50,
  155 at 60, 20 at 65, 688 at 70, 10 at 75, 596 at 80, 92 at 85 and
  130 at 90;
- the over-budget ones list a second corner. The longest names both
  legs' setbacks at one corner and a swallowed carrier at the other;
- the line×arc grid's 246 are all within budget, the longest 63
  words;
- a synthetic pair with both legs overrun at both corners renders 107
  words.

What is owed is a
shape that fits: the per-corner clauses are the bulk (a second
corner's reason is fifteen words or more), so the likelier repair is a
shorter per-corner clause, or a pair sentence that names the corner
the recourse is about and summarises the rest, not a shorter recourse,
whose wording is ratified.
