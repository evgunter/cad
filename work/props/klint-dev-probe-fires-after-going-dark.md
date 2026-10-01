---
id: klint-dev-probe-fires-after-going-dark
kind: issue
title: k-lint (dev-probe) fires 35 flags on its first nightly run after moving off the per-PR gate
status: closed
closed: 2026-09-30
opened: 2026-09-29
priority: P0
cost: M
pr: 3530
---

**A red nightly is a red main, and the orchestrator owns it**
(`docs/prompts/implementer-discipline.md` §2). This is that row.

## What fired

Nightly run 36561506133 (2026-09-29, head `aae5716bf`), job
`k-lint (dev-probe)`, exit 2:

```
k-lint: GATE FAILED — the margin distribution changed: 35 margin(s) crowd a decision
             boundary that the committed baseline says should be empty.
```

Per-row, from the job log:

| eps row | samples | rule 1 (undecided/invalid) | rule 2 (near a threshold) | rule 3 (below a floor) |
| --- | --- | --- | --- | --- |
| `k-eps-1e-6.csv` | 1,260,729 | 9 | 0 | 0 |
| `k-eps-1e-9.csv` | 1,260,737 | 9 | 0 | 0 |
| `k-eps-1e-12.csv` | 1,260,749 | 9 | 8 | 0 |
| TOTAL | 3,782,215 | 27 | 8 | 0 |

Every sample is `classified` (0 `symbolic_zero`, 0 `sign_gated`, 0
`registered`).

**The shape is informative and should steer the measurement.** Rule 1
is **9 at every eps row** — eps-INDEPENDENT, so those nine are not
crowding a threshold, they are margins that came back undecided or
invalid. Rule 2's eight appear **only at 1e-12**, which is the
eps-coupled behaviour rule 2 is for.

## The instrument had gone dark, which is the other half of the story

- `dev-probe` lived in **`ci.yml`** (the per-PR gate) until the CI cut
  `49d5b2aee` ("ci: cut the per-PR gate for latency; move the rest to
  nightly", 2026-09-28) moved it to `nightly.yml`.
- The nightly runs of **2026-09-22 through 2026-09-28 contain ZERO
  `k-lint` jobs**; the 2026-09-29 run has five, of which `dev-probe` is
  the one that failed.
- **No main commit between 2026-09-25 and 2026-09-28 carries a
  `dev-probe` check run at all** (scanned via the commits' check-runs
  API), so it was not running on merges either.

So this is not "the distribution moved last night". It is an instrument
that had not run over main for an unknown window, running again and
firing. The window is the first thing to measure: **when did
`dev-probe` last actually execute and pass**, and what merged since.

The baseline constants themselves are not ancient — `BASELINE_FLOOR_MARGIN`
and friends were last touched at `c3012ed35` (2026-09-16, instr u12).

## What is NOT established, and must not be asserted

A candidate mover is **ENCL's PR 3418** (merged 2026-09-29, seam-noted
on this slate): `Decide::sign_within` now returns `Decided { sign,
margin }` on **every** outcome, `MarginDiag` became opaque, and
`terminal_sliver` is decided at classify time. If more outcomes now
carry a reporting margin, the probe CSV can record margins it never
recorded before — and newly-visible `Invalid` margins would land in
rule 1 exactly as observed, eps-independently, with no geometry having
moved.

**That is a hypothesis with a mechanism, not a finding.** It fits the
eps-independence of rule 1 and nothing else has been checked. It could
equally be that the sample population is unchanged and nine real
undecided margins appeared. Measure it; do not inherit it.

## The one forbidden move

From the lint's own output, and `implementer-discipline` §3: **do not
change geometry to get under the threshold.** A fired lint is evidence
about the margin distribution. The recourses, in the lint's order, are
to re-derive the baseline and thresholds per `docs/K-REPORT.md`'s "M7
addendum (2026-08-07): the large-K lint's floor refresh", or — if
re-derivation is not warranted — to demote the row to advisory with a
recorded justification.

Note the lint's recourse text still names `ci.yml + local-scripts/ci-local.sh`
as the pair that must not drift; after `49d5b2aee` the hosted row lives
in `nightly.yml`, so that sentence is stale and is part of what this
row fixes.

## Refs

Nightly run 36561506133. CI cut `49d5b2aee`. Baseline constants at
`c3012ed35`. Runbook: `docs/K-REPORT.md`.

---

## Measured (PROPS k-lint baseline unit, 2026-09-30)

### 1. The darkness is 19 hours, and two of this item's three premises
are artefacts of where the row lived

**Last green execution: run 36449836735, job `k-lint (gate, dev-probe)`
(job id 109022798206), which RAN 2026-09-28T16:23:27Z–16:30:32Z** (the
run was created at 16:16:28Z; dating an instrument by when it executed
means the job's own timestamps, and it is still the last such job by
either). It read 3,789,703 samples and flagged nothing. The next
execution of the row anywhere is the failing nightly's job
109383464088, 2026-09-29T11:24:53Z–11:32:02Z — a gap of **18 h 54 m**
between executions, one nightly cycle.

The job ran on the merge ref of `vnews/one-seat-line` head `b34a34cbd`
against main at `cc2a4b7ed`, so it is a reading of main plus one PR.
That PR is kernel-neutral: `git diff --name-only cc2a4b7ed...b34a34cbd`
is 17 files under `crates/viewer/**` and `work/**` and nothing else, so
the sweep it produced is main's sweep.

Two premises above are true but do not carry the weight put on them:

- *"the nightlies of 2026-09-22..28 contain zero `k-lint` jobs"* —
  they do, and so does every nightly before them: `k-lint` lived in
  `ci.yml` until `49d5b2aee` and was never a nightly job at all. Run
  36417520955 (09-28) is the last nightly without it; 36561506133
  (09-29) is the first with it.
- *"no main commit between 09-25 and 09-28 carries a `dev-probe` check
  run"* — no main commit has ever carried one. `ci.yml` gates every
  k-lint job on `github.event_name != 'push'` (the 2026-08-20 push-run
  trim), so the row only ever ran on pull-request runs; scanning those
  is what dates it.

So the instrument did not go dark by accident. It ran on every PR up
to 16:16Z on 09-28, the latency cut put it on the nightly ten minutes
later (`b746b6141`, 16:26Z), and it fired on its first nightly. **No
CI row is filed**: the gap is the designed cost of `work/ciw/latency-cut.md`,
which Ev approved on #3340.

### 2. Population, not distribution

| reading | samples 1e-6 / 1e-9 / 1e-12 | total | rule 1 | rule 2 | rule 3 |
| --- | --- | --: | --: | --: | --: |
| last green (36449836735) | 1,263,225 / 1,263,233 / 1,263,245 | 3,789,703 | 0 | 0 | 0 |
| failing nightly (36561506133) | 1,260,729 / 1,260,737 / 1,260,749 | 3,782,215 | 27 | 8 | 0 |
| delta | −2,496 per row | −7,488 | +27 | +8 | 0 |

The population SHRANK by 2,496 samples per row across ~85 merges. It
did not grow, which is the first thing that does not fit the PR 3418
hypothesis.

Rule 1 fires on **every** row whose outcome is `invalid`, with no
threshold in the way (`tools/k-lint/src/lib.rs`, `lint_sample`'s
`"invalid" => reasons.push(Reason::Invalid)`). The last-green sweep
flagged rule 1 zero times over 3,789,703 samples, so **that sweep
contained no `invalid` row anywhere**.

That alone does not finish the question the spec's step 2 asks. It
excludes the nine having been recorded as `invalid` before; it does not
by itself exclude the same nine samples having existed DECIDED and
turned poison since, which would be a distribution change and a
regression. What excludes that is the mechanism, not the counts: the
NaN is `SpanLocate::enclosure_hull`'s deliberate poison, minted by
`chart_edge` at a point scalar and propagated by `Real::min`/`max` per
`real.rs`'s NaN policy, so the margin is NaN for ANY face whose outer
loop has a non-straight edge on a periodic chart, at every eps and on
every sweep. A sample of this predicate on such a face cannot have been
decided; it can only have been absent. So the nine are rows the old
sweep did not record. Full derivation, with the call sites:
`work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md`.

### 3. The nine and the eight, by predicate and site

Rule 1, 9 per eps row, identical at 1e-6 / 1e-9 / 1e-12 (eps-independent
because a poisoned margin has no threshold to be near):

| site | predicate | rows/eps | margin |
| --- | --- | --: | --- |
| `corpus/boss_union` | `chart_bound_outer_span` | 3 | `NaN` |
| `demo/bossplate` | `chart_bound_outer_span` | 3 | `NaN` |
| `demo/lily_walls` | `chart_bound_outer_span` | 3 | `NaN` |

Rule 2, 8 at 1e-12 only, all `demo/lily_walls`:`bool_circle_torus_root_slack`,
zero-classified with `|m| > band_zero / 10^2`:

| \|m\| (m) | count |
| --- | --: |
| 3.14912307142162e-13 | 2 |
| 8.272757261396428e-14 | 2 |
| 6.680730726416532e-14 | 2 |
| 1.9291935961799184e-14 | 2 |

Four distinct margins, each twice; they are eps-INDEPENDENT lengths
that only enter rule 2's window once `band_zero` reaches 1e-12.

### 4. The PR 3418 hypothesis is wrong

It predicted newly-VISIBLE margins, i.e. samples that were always taken
but not recorded. Against it:

- `Probe`'s `Decide::sign_within` records an `Invalid` sample on
  exactly the condition it recorded before — `Err(e) if
  e.margin.is_invalid()` — and the margin it records is still the raw
  `f64` handed to the classifier; PR 3418 changed the Ok arm's type
  (`Sign` to `Decided`) and the spelling `MarginDiag::Invalid` to
  `MarginDiag::INVALID`, and added no recording site;
- PR 3418 does not touch `crates/topo/src/chart_bound.rs` at all, and
  its only two edits to `crates/topo/src/pcurves.rs` are inside
  `mod recourse_tests` (`MarginDiag::value(5e-9)` and a
  `terminal_sliver: false` field);
- `chart_bound_outer_span` has existed since 2026-09-04 (`21184358a`),
  and the M11 reading at `c39a904e` (2026-09-08) found no `invalid`
  row in the whole census.

`bool_circle_torus_root_slack`, by contrast, is a predicate that did
not exist at the comparison point: minted at `987e97789`
(2026-09-28T17:02:22Z), merged as PR #3375 at `38588fede`
(2026-09-29T02:07:58Z), in the same PR that rewrote
`demos/tour/src/lily.rs`.

**The sample count is NOT a ground against the hypothesis, and an
earlier draft of this section used it as one.** "The population fell by
2,496 per row" says nothing about nine additions inside a net change of
that size over 1.26 M rows; the two are entirely consistent. The three
grounds above are what carry.

**What the file sweep behind those grounds could not match.** It asked
which files PR 3418 touches, against the files the SPAN margin is built
from. Reading the mechanism first made the arm a second entry point:
`u_arm` comes from `chart_u_arm`, whose non-azimuth kinds read
`geom_brep::chart_stretch_sup` in
`crates/geom-brep/src/pcurve_cache.rs` — a file PR 3418 DOES touch, and
which a sweep scoped to `crates/topo` would have missed. Checked: its
edits there are `FittedMagnitude`/`PcurveCertifyError` doc prose, the
`certified_clearance` field's type (`f64` to `MarginDiag`), a
`MarginKind::Invalid` link spelling and two test bodies;
`chart_stretch_sup` and `chart_stretch_sup_v` are untouched. The
conclusion stands, now with the entry point that nearly escaped it
named.

### 5. Recourse: NEITHER — stop and report

- **The nine are rule 1, and rule 1 has no recourse.** It carries no
  threshold to re-derive (recourse 1 re-derives
  `BASELINE_FLOOR_MARGIN`, the percentile and
  `EPS_COUPLED_FLOOR_RATIO`, none of which rule 1 reads), and the lint
  records that nothing offers its demotion because demoting it would
  demote the ERROR-DESIGN E6 re-open trigger. A poisoned margin is
  "a defect wherever it appears" in the lint's own words — and here it
  is a STRUCTURAL poison at the recording scalar rather than geometry
  that moved, so it will reappear on every future sweep of these three
  shapes until someone rules on which of the row's three readings
  holds. Filed:
  `work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md`.
- **The eight are a new family's lower tail.** Rule 2's zero-side arm
  compares `|m|` against `band_zero / PROXIMITY_FACTOR` and reads no
  baseline constant, so recourse 1 does not reach it either; the only
  thresholds that could move are `PROXIMITY_FACTOR` (ratified) and
  `EPS_COUPLED_PREDICATES`, whose membership the M7 addendum says is
  explicit and never inferred — and rule 4 does not fit a family whose
  margins are fixed lengths rather than an eps-scaled headroom. Filed:
  `work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md`.

Re-deriving the baseline here would have been the second-worst move
the spec names: it would have papered over a poisoned margin.

### 6. The stale recourse text

The failure message already names `nightly.yml`. The stale pair is one
line of `tools/k-lint/src/main.rs`'s module docs, which said `ci.yml`'s
step carries `--gate-rule-1-only`'s recorded justification; it now says
`nightly.yml`. `docs/K-REPORT.md` names `ci.yml` for this row at five
further places, in instr's territory:
`work/instr/k-report-still-names-ci-yml-for-the-k-lint-row.md`.

One more false sentence, found by chasing the NaN and fixed here rather
than filed: `SpanLocate::enclosure_hull`'s doc
(`crates/geom-core/src/spline/locate.rs`) claimed point scalars "never
reach it by construction (single span)". `chart_edge` reaches it at a
point scalar on both of its arms, so the claim was false on this tree
and is a premise other code may cite. The file is double-claimed by
`nurbs` and `props`, so correcting it is not a crossing.

### 7. What this leaves red

`k-lint (dev-probe)` stays red on the nightly until the chart row is
fixed or the germ row is ruled on. That is the gate working: it is red
because the kernel is, and this unit is not entitled to green it.

### 8. Reproduced hosted at a third population

`nightly.yml` dispatched on `props/klint-baseline` (run **36691873111**,
job `k-lint (dev-probe)`, job id 109810896182) re-cut the sweep at
today's main and read **3,927,679 samples** (1,309,217 / 1,309,225 /
1,309,237) — 145,464 more than the failing nightly. It flags **the same
35**: the same three shapes' `chart_bound_outer_span` NaNs, and the same
four `bool_circle_torus_root_slack` margins to every digit. So the 35
are not a sampling artefact of one population; they survive a corpus that
has moved again underneath them.

A local sweep at today's tip was started twice under
`local-scripts/with-build-slot.sh` and abandoned: the first was reaped at
the harness's background limit two hours in, the second was still queued
behind another lane when the hosted reading above superseded it. No local
sweep was taken at the comparison point either.

**What that costs, stated where the claim is made rather than after
it.** The two hosted job logs carry the sample counts and the per-rule
counts, which is what §2's table is. They do NOT carry per-predicate
populations, so no reading here counts `chart_bound_outer_span`'s rows
at either end. §2 therefore does not rest on the counts for the
population question — it rests on the mechanism, which says a sample of
this predicate on such a face cannot have been decided. The counts
would have made that a second, independent check; they were not taken,
and this is the sentence that says so.
