---
id: k-lint-eps-coupled-criterion-unwritten
kind: issue
title: What makes a predicate eps-coupled is nowhere written, so k-lint's roster cannot be pinned against the kernel in the ADDED direction
status: open
opened: 2026-09-07
refs: [k-lint-predicate-roster-unpinned]
---


## What

**`k-lint-predicate-roster-unpinned` asks for a pin, and a pin can only
close half of what that item found.** `tools/k-lint/src/lib.rs:283`
holds `EPS_COUPLED_PREDICATES = ["props_quad_converged"]`, a roster of
the kernel's vocabulary held in a workspace-excluded consumer, and the
item names two silent directions:

- a predicate the kernel **renames** leaves the roster naming nothing;
- a predicate the kernel **adds** to that class is absent from the
  roster and is judged by the wrong rule.

An `include_str!` pin over `crates/geom-brep/src/props/quad.rs` — the
`D204` shape, whose machinery already exists at
`tools/tess-meter/tests/derivations.rs:239` — closes the first
direction completely: the name is either still minted at the cited site
or the pin reds. **It cannot close the second at all.** Catching an
ADDED ε-coupled predicate needs a statement of what makes a predicate
ε-coupled that a test can evaluate over the kernel's minted names, and
no such statement exists in the tree. `tools/k-lint/src/lib.rs:30` and
`:126` describe the class as one "whose margin is a headroom against
ε", which is a description of the roster's one member rather than a
criterion anything can apply.

## Finding

**The item's own framing hides this, which is why it gets a file.**
`k-lint-predicate-roster-unpinned` says "nothing pins the two together
in either direction" and then reaches for `D204`'s remedy, which was
adequate there because `CHART_TAGS`' producing half is a closed match
on `Chart::tag` — the roster and the vocabulary are the same finite
set, so pinning one pins both directions. `EPS_COUPLED_PREDICATES` is
not that shape: it is a SUBSET selected from an open and growing
vocabulary by a property, and a subset selected by an unwritten
property cannot be checked for completeness.

So the pinning lane should be dispatched knowing it will close one
direction and disclose the other, rather than discovering at review
that its "pinned in one direction" claim is narrower than the item it
cites. Whether writing the criterion is worth a unit is a separate
question and probably reaches `crates/geom-brep/src/props/*` (PROPS'
territory), which is why this is an issue and not a rider.

**Confidence:** sure that no criterion is stated in `tools/k-lint` or
in the `props/quad.rs` mint sites; unsure whether one exists somewhere
in `docs/` under a name the sweep below could not match.

**Sweep and its blind spot.** Grepped `EPS_COUPLED_PREDICATES` across
the tree and read every hit, and read `tools/k-lint/src/lib.rs`'s
module docs for a definition of the class. The pattern was the
CONSTANT's name and the phrase "ε-coupled"; it cannot match a criterion
written down without either — a rule stated in `docs/K-REPORT.md` about
which margins are headroom, say, in prose that never uses the roster's
spelling.

## Was

Raised by the METER orchestrator's difficulty pass over this program's
slate (2026-09-07), reading `k-lint-predicate-roster-unpinned` against
`tools/k-lint`'s manifest and the `D204` precedent it cites.

## Corrected by the pinning lane (2026-09-07)

The row above dispatched `k-lint-predicate-roster-unpinned` and its two
substantive claims were checked at the code. **One holds, one does not,
and a third direction neither file names is the one that was actually
silent.**

**Holds.** No criterion for ε-coupledness exists that a test can
evaluate over the kernel's minted names, and the "Confidence" line's
open half is now closed: the sweep it could not run was run.
`docs/K-REPORT.md`'s Finding M5-1 (`:935`) is the closest thing in the
tree — *"the first predicate in the project whose margin is ε-coupled
rather than model-scale"* — and it is a description of one family, not a
rule anything can apply to a name. Nothing else in `docs/` states one.
So this item stays open exactly as filed.

**Does not hold, and it is a DIFFERENT file's sentence.**
`k-lint-predicate-roster-unpinned:37` says *"Both directions are
silent"*. Neither is. `tools/k-lint/src/lib.rs:160` states the
allow-list's posture — a NEW ε-coupled predicate is not on the roster,
so it stays under rules (2) and (3) and **flags** —
`tests/review_probes.rs:77` measures it (silent at 1e-6, loud at both
tight rows, which CI always runs), and `docs/K-REPORT.md:645`'s "this
roster is a RECORD, and stays hand-maintained" ruling decides it: *"an
ε-coupled predicate missing from it keeps flagging under the metre
rules until someone rules. A roster omission therefore cannot silently
weaken the gate."* A rename is a roster omission too, so it is loud by
the same mechanism. What both directions cost is not a missed finding
but a **misdirected** one: the gate fires on the family's own margins
with the CLI's recourse pointing at a baseline re-derivation, and the
cause is a name.

**This item's own claim is narrower than that, and survives.** The row
above says a pin cannot close the ADDED direction *at all*, meaning
mechanically, from the k-lint side, over an open vocabulary — and it
concedes two paragraphs earlier that the direction is not a hole in the
gate. That still holds in general; the pin closes a slice of it, below.

**The orchestrator's dispatch brief overstated in the other direction**
and that correction belongs here too. It said *"both the item and the
orchestrator said an unrostered predicate is SILENT"*. This item did
not: `:19-21` says such a predicate is **"judged by the wrong rule"**,
which is the misdirected-diagnosis framing, not silence.
`k-lint-predicate-roster-unpinned` is the file that said silent.

**The direction that WAS silent, and is now pinned.** A predicate that
stays on the roster and stops being ε-coupled — re-meter the quadrature
stopping test against a fixed length instead of `QUAD_TARGET_LEN_FACTOR·ε`
and the name is still minted, the roster still matches, rule (4) still
exempts the family from rules (2) and (3), and a model-scale margin is
judged by a rule that will essentially never fire on it. Nothing flags.
`the_rostered_familys_margin_is_still_eps_scaled_at_its_mint` closes
that, textually and per mint: each rostered mint's margin uses the
WHOLE identifier `target_len`; that identifier is bound exactly once in
the mint's own enclosing `fn`, to `QUAD_TARGET_LEN_FACTOR * eps`; and a
mint whose `target_len` arrives as a PARAMETER is refused rather than
answered, because whose value it is then is the caller's property.

**A slice of the ADDED direction is closed too, and only a slice.**
`every_target_len_mint_is_rostered_or_excused` reds on any mint in
`tools/k-lint/tests/predicate_roster.rs`'s `MINT_SOURCES` whose margin
derives from the ε-scaled `target_len` and is neither on the roster nor
excused by name. That found one:
`k-lint-last-round-is-eps-coupled-but-unrostered`. It is a slice
because `target_len` is this family's SPELLING of ε-coupling, not the
criterion — a family metered against an ε-derived target under any
other name, or minted in a file that table does not list, is still
invisible.

**What that leaves for this item.** The general criterion, and only the
general criterion: the pin above is hand-written against one family's
spelling and does not extend to an entry the kernel has not minted yet.
A unit writing the criterion would need, from PROPS, a statement at the
mint of which `props_*` margins are metered against an ε-derived target
— the thing `QUAD_TARGET_LEN_FACTOR` makes true of this family and
nothing states in general.
`k-lint-roster-wants-a-kernel-side-vocabulary` names the shape that
would carry it: a `const` slice exported from `geom_core::k_stats`,
declared beside which of its names are ε-metered, is one PROPS edit
that closes both this item and the parser.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
