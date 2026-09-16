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

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

## Ruled (2026-09-16): stays off, and one premise above was wrong

**The second branch, with the first branch's measurement taken anyway.**
`docs/K-REPORT.md` now carries the ruling beside "Maintenance: this
roster is a RECORD", and `k_lint::EPS_COUPLED_UNRULED` is its
machine-readable half — the list moved out of
`tools/k-lint/tests/predicate_roster.rs`'s `NOT_ROSTERED` and into the
crate, because the CLI reads it too.

**What this file got wrong.** *"it emits zero rows in every committed
baseline"* is true and VACUOUS, and the Finding's argument leans on it
as though it were a measurement. The kernel first minted
`props_quad_last_round` on 2026-09-03 (`33342b11f`, TCOST-K1); the
newest committed era, `m7-eps-*`, was swept on 2026-08-07. **No
committed row could have named it.** The grep in the Confidence line
found an absence it could not distinguish from the predicate not
existing.

**So the sweep was run.** `scripts/k_probe_sweep.sh` at `89c8766`:
**zero rows at all three ε rows**, over 1 263 818 / 1 263 826 /
1 263 838 samples and 277 names, against `props_quad_converged`'s
92 / 104 / 116. The name is minted once per face and only when round 0
fails to certify, and nothing in the Band 4 corpus or the demo scenes
gets there. The Confidence line's open half — *"unsure whether a corpus
that reaches the budget exit exists in the wild set at all"* — is now
closed: it does not.

**And rule (4) would need a second floor even with draws**, which this
file raises and does not settle. `props_quad_converged` meters the
round that STOPPED, bounded above by the target; this one meters a
LOWER BOUND on a round that never runs, and only a definite negative
reading refuses — so its population has a refusal side nothing bounds
below, where rule (4)'s single lower-tail threshold says nothing.

**What keeps the ruling honest** is three rows and one stated residue,
written out in the K-REPORT ruling: the committed-era guard
(`the_unruled_eps_coupled_names_have_no_draw_in_the_era_the_floor_is_cut_from`),
the derivation that rule (3) reds on the first positive row at either
tight ε row
(`an_unruled_eps_coupled_margin_is_loud_under_rule_3_at_the_tight_rows`,
reading `QUAD_TARGET_LEN_FACTOR` out of the kernel rather than writing
1024 down), and the CLI note `Scan::unruled` drives so that red is not
spoken in the metre rules' recourse. The residue is a purely-negative
population.

**Consistent with `k-lint-eps-coupled-criterion-unwritten`, which stays
parked.** That row's account of the cost — the family judged by the
metre rules, below `BASELINE_FLOOR_MARGIN` at the tight rows, with the
CLI's recourse pointing at a baseline re-derivation that will not move
— is exactly right, and the note above is what stops it being the
reader's problem for THIS name. It does not touch the general
criterion, which is still PROPS' to supply.
