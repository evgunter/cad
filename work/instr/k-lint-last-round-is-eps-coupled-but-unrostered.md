---
id: k-lint-last-round-is-eps-coupled-but-unrostered
kind: issue
title: props_quad_last_round is eps-coupled by the criterion the roster pin now applies, and is not on the roster
status: closed
opened: 2026-09-07
closed: 2026-09-16
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
**zero `props_quad_last_round` rows in all three legs** — the gated
corpus+demo files (1 263 818 / 1 263 826 / 1 263 838 samples, 277
names), the M2 dump and the E6 driver dump — at every ε row, against
`props_quad_converged`'s 92 / 104 / 116.

**Read that as structure, not as coverage.** The mint is reached only
from the two patch lanes and only after round 0 fails to certify;
`cylinder_cut_face_rounds` has no budget exit, and in the committed
era the shapes failing round 0 sit in the lane WITHOUT the mint. So
the zero is determined by which lanes this corpus exercises, and the
sample counts are the run's scale rather than evidence of reach. The
informative numbers — faces entering a patch lane, faces failing round
0 — are not reported by the sweep. The Confidence line's open half
— *"unsure whether a corpus that reaches the budget exit exists in the
wild set at all"* — is closed in the direction it asked: none does.

**A second-floor argument this unit first shipped was FALSE, and the
correction is worth keeping** because the file's own "What a unit here
does" raises it. The first version said the two families differ in
that `props_quad_last_round`'s refusal side is unbounded below where
the rostered family's headroom is bounded by the target. It is not:
`props_quad_converged` carries 24 negative draws at the 1e-9 row of
the committed era and 48 at 1e-12, reaching `|m| = 1.83e-4` ≈
`1.8e8·ε`, and the P0 rule (4)'s floor is cut from is itself a
negative row. Both families record a signed headroom with an unbounded
refusal side. What actually differs is the SAMPLE — once per round,
recording the round that stopped, versus once per FACE, recording a
lower bound on a round that never ran — and whether those two lower
tails are the same shape is precisely what no draw has been taken on.
The ruling rests on the absence of draws, which is measured and
sufficient.

**What keeps the ruling honest** is two rows, written out in the
K-REPORT ruling. A row of an `EPS_COUPLED_UNRULED` name is a FINDING
in `tools/k-lint` — it fails the run in a voice that names this ruling
and says the baseline re-derivation is not the recourse, on rules
(2)/(3)'s demotable side — and
`the_unruled_eps_coupled_names_have_no_draw_in_the_era_the_floor_is_cut_from`
reds if the era the floor is cut from ever carries one.

**The gate is on the NAME, not on a margin, and that is the point.**
The metre rules see nothing on the refusal side, and a budget refusal
does not surface elsewhere either: `crates/topo/src/props.rs`'s
`sign_certified` keeps the enclosure and reports the refusal only
through `last_word`, which is asked exactly when `settle` never
accepted — so when `settle` accepts the sign off the enclosure, these
readings accumulate on a wholly green suite. An earlier version of
this section claimed such a corpus would already be a red suite; it
would not.

**Consistent with `k-lint-eps-coupled-criterion-unwritten`, which stays
parked.** That row's account of the cost — the family judged by the
metre rules, below `BASELINE_FLOOR_MARGIN` at the tight rows, with the
CLI's recourse pointing at a baseline re-derivation that will not move
— is exactly right, and the note above is what stops it being the
reader's problem for THIS name. It does not touch the general
criterion, which is still PROPS' to supply.
