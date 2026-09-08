---
id: k-lint-last-round-is-eps-coupled-but-unrostered
kind: issue
title: props_quad_last_round is eps-coupled by the criterion the roster pin now applies, and is not on the roster
status: open
opened: 2026-09-07
---


## What

`crates/geom-brep/src/props/quad.rs:2767` mints `props_quad_last_round`
with `Margin::of(target_len - last_round_len)` — a headroom against
`QUAD_TARGET_LEN_FACTOR·ε`, which is exactly the property
`tools/k-lint/src/lib.rs:295`'s `EPS_COUPLED_PREDICATES` exists to
name. It is not on that roster, so rules (2) and (3) — the metre rules,
whose floor `BASELINE_FLOOR_MARGIN` is cut from a population of
ε-INDEPENDENT margins — are the ones that judge it.

Nothing fires today: the predicate emits **zero rows in every committed
baseline**, so there is no sample for either rule to act on.

## Finding

The unit that pinned the roster (`meter/klint-roster-pin`) added
`every_target_len_mint_is_rostered_or_excused`, which reds on any mint
in `MINT_SOURCES` whose margin derives from the ε-scaled `target_len`
and is neither rostered nor excused. `props_quad_last_round` is the one
mint that hits it, and this unit **excused** it rather than rostering
it, in `tools/k-lint/tests/predicate_roster.rs`'s `NOT_ROSTERED`.

**Why excused and not rostered.** Rule (4)'s floor,
`EPS_COUPLED_FLOOR_RATIO`, is documented at `tools/k-lint/src/lib.rs:297`
as the minimum of 108 draws of `props_quad_converged`'s |m|/ε
statistic, with 8.9% of headroom and no structural lower bound.
`props_quad_last_round` has contributed no draw to it. Moving the
predicate under that floor would be a distribution ruling made with no
distribution — and the ruling would be invisible for as long as the
predicate keeps emitting nothing, which is the same silence a roster
omission has. Excusing it says the same thing out loud and reds if
anyone changes it.

**What the excuse costs.** If the budget exit starts emitting rows, the
family is judged by the metre rules: its ε-scaled margins land below
`BASELINE_FLOOR_MARGIN` at the tight rows and FLAG, with the CLI's
recourse pointing at a baseline re-derivation that will not move. That
is the misdirected-diagnosis failure `k-lint-eps-coupled-criterion-unwritten`
describes, one predicate over.

## What a unit here does

Either:

- **sample it.** Find a corpus that drives the composite lanes to their
  budget exit, run the `dev-probe` sweep, and read
  `props_quad_last_round`'s |m|/ε population off it. If it looks like
  `props_quad_converged`'s, roster it and delete the `NOT_ROSTERED`
  line; if it does not, rule (4) needs a second floor before it can.
- or **rule that it stays off**, in `docs/K-REPORT.md` beside the "this
  roster is a RECORD" ruling, and leave the `NOT_ROSTERED` line as the
  mechanical form of that ruling.

Not: roster it because the criterion says ε-coupled. The criterion
selects the class; the floor is a measurement, and this family has not
been measured.

**Confidence:** sure that the mint derives from the ε-scaled target and
is unrostered; sure it emits no baseline rows (grepped every
`docs/k-report-data/*.csv.gz` for the name); unsure whether a corpus
that reaches the budget exit exists in the wild set at all.

## Was

Raised by the style review of `meter/klint-roster-pin` (STYLE-5,
`likely`) and disposed by that PR's fix pass, which added the converse
check and the excuse rather than the roster line.
