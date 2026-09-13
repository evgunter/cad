---
id: chart-coherence-ships-off-and-nothing-schedules-turning-it-on
kind: issue
title: the chart-coherence resident defaults Off, which answers the no-consumer row with a check nobody runs — two measurements decide whether it turns on
status: open
opened: 2026-09-12
refs: [2408, coherence-findings-have-no-consumer]
---



(FIX orchestrator, 2026-09-12) Disclosed by the
`coherence-findings-have-no-consumer` lane (PR 2408) as the call it was
least sure of, and filed at the moment of disclosure per
`work/README.md` — a residue named in a PR body and nowhere else is
invisible to the re-homing sweep.

## The tension, stated fairly

`CheckId::ChartCoherence` ships with severity **`Off`**, the only
default on `ChecksConfig` that is not `Warn`. The row it answers
complained that `examine_chart_coherence` had **zero production
callers**. It now has one, wired and pinned — but out of the box
nobody runs it, and `ChecksReport::skipped` advertising the resident on
every default report is a weaker kind of reach than actually running.

**The orchestrator's judgement is that `Off` is right and the lane's
two reasons hold.** This row exists so that "right for now" does not
become a permanent silence by default.

**Reason 1 is a design property, not a preference.** The examination
has no shape door, so `Unexaminable::NonIsoCarrier` puts **two loops
per body** into `unexamined` for an ordinary trimmed cylinder — a lane
boundary, not a defect, and not actionable by the person reading it. At
`Warn` every trimmed cylinder in every document would emit two findings
meaning *"this door does not cover cones"*, which trains a reader to
ignore the check. `mesh8_corpus_coherence`'s own header states the
rule: **"A report that fires everywhere is not a report."**

**Reason 2 is a hard constraint.** This registry's per-resident cost is
MEASURED and PINNED
(`docm5_subject::the_registry_split_is_measured_at_a_pinned_point`),
and nothing has measured this resident, which reads every face of every
rest body on every run. Flipping the default would move a pinned cost
measurement on behalf of a resident nobody has measured. That is not
the baseline-preservation the discipline forbids — it is the opposite:
the pin exists to report what the registry actually costs, so the
number must be taken before the default moves, not after.

## What decides it, and it is two measurements rather than an opinion

1. **Measure this resident's cost** against the pinned point, the way
   the other two residents were measured. If it is cheap, reason 2
   dissolves.
2. **Settle whether `NonIsoCarrier` belongs in `unexamined` at all.**
   The door drops planes outright but reports non-iso carriers, and the
   2408 lane deliberately did not second-guess that taxonomy from the
   consumer side — correctly, since a consumer-side filter would be
   dropping data, which this program is against. **The honest lever is
   in the door, and the door is MESH's** (`crates/topo/src/coherence.rs`).
   If the two-loops-per-cylinder answer is a door defect rather than a
   lane boundary, reason 1 dissolves too and this is MESH's row, not
   FIX's.

Both dissolving makes the flip one field plus a set of re-baselines
(ten pins name `ChecksReport::skipped` and would stop naming this
resident). Either surviving is an answer worth writing down at the
default rather than leaving as an unexamined `Off`.

## What is NOT owed

Not a re-litigation of the resident's shape: `Advisory` (DS6's waiver
`iff`), `CheckKind::Certified`, second in `ALL`, and the
`skipped`/`unexamined` separation are all argued at their sites in
PR 2408 and pinned. This row is about one field.
