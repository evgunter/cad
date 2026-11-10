# Escapes by review tier, and what a late fix costs

Off-file analysis, asked for by Ev on 2026-11-10: are the hard-unit dual
reviews worth it, judged by the issues found later; are those issues more
common in single-review units; and what does a late fix cost against
finding and fixing the defect up front? It builds on readout 6
(`analysis/dual-review/readout-6.md`, branch
`analysis/dual-review/readout-6`), whose `ESCAPE_CANDIDATES` it extends.
Every number is printed by `analysis/dual-review/escape-cost-numbers.py`
(stdlib, run from a checkout: `python3 analysis/dual-review/escape-cost-numbers.py .`,
about two minutes).

## Summary (phone)

- **No sign single-review units escape more.** From 10-11 to 11-09: 17% of dual units (50/296) and 19% of H pairs (32/166) had a red or regression on main traced to them. For single-review units the figure is 5–10%, depending on the denominator.
- **Size-matched, the rates are about equal.** Per 100k changed lines it is 4.7 for dual and 6.4 for single. Dual units are 2–3× bigger and harder by selection, so a similar rate is what a working second review would look like. The data cannot show more than that.
- **The escapes are a CI blind spot, not a reviewer blind spot.** 92% are rows the PR gate never ran: the slow set (60%), 1e-6/1e-12 rows, feature-gated rows, nightly jobs and pncad-py. 63% are moved pins or censuses with no wrong answer. Behaviour regressions are a third.
- **Late escapes are found and fixed fast.** The median is 0–1 days from merge to issue and 1 day to close. Fix PRs have a median of 62 lines. But 39% of escapes were filed twice or more, and 29% carried a P0.
- **Late cost, estimated from lane costs:** a pin escape costs about 0.6 of a review (~140k tokens), a behaviour regression about 2.7 reviews (~670k). That is 7–9× what fixing the same defect inside the unit's own fix pass would cost (range 1.2–70×).
- **At H, the second review does not pay for itself in fix cost alone.** It costs 250k and buys about 0.14 blocking defects per unit, so about 1.8M tokens per defect. Fixing those as late reds would cost only about 95k per unit.
- **It pays on correctness.** The tallied findings are wrong answers that no row covers, so they would ship silently rather than as reds. They never appear in the escape record. That is the case for keeping H duals.
- **The cheaper lever is CI selection.** Run a PR's crates' slow set and ε rows before merge. That would remove most of these escapes, which no number of reviewers removes.
- **For Opus/Fable:** use the tallied rate (0.217 per pair) as the endpoint, not escapes. Escape attribution covers only about half of reds, and CI dominates the escapes.

## 1. The population

**Window.** Units merged 2026-10-11..2026-11-09: readout 6's "since
readout 5" window, where the tier records and the `main-red-…` issue
convention are both dense. An early window (2026-09-23..10-10) is printed
too, but its escape filings are sparse (section 2.4).

**Dual units: 296**, from the rows' own dates. 166 are H concurrent, 41 M/S
holdouts, 88 M/S sequential and 1 M without a draw (readout 6's
corrections applied).

**Single-review units: a range, not a count.**
- Merged PRs were found from `Merge pull request #N` commits: 1,922 in
  the window. Of these, 1,234 touch code (crates/, scripts/, demos/,
  bindings, CI, or any .rs/.py/.toml/.yml file).
  - A spot check against GitHub (PRs 4761–4860, all merged) found merge
    commits for 68. The rest reached main by a local merge and have no
    such commit.
  - So this population misses about a third of merged PRs, nearly all of
    them non-dual: 292 of 296 dual rows have merge commits.
- The tier of a non-dual code PR is classified by pattern over every
  program log and item, including the 97 pruned programs' logs recovered
  from history. A paragraph naming the PR or its branch beside
  single/FULL/STYLE words, or beside "orchestrator's read", marks it.
  - **Single: 453** (355 single words alone, plus 98 that also mention an
    orchestrator's read nearby).
  - **Orchestrator's read: 50.**
  - **Unrecorded: 443.** This group is mostly red fixes, re-pins and small
    follow-ups (median 189 lines), plus units recorded only by slug.
- Every non-dual introducer of an escape was read by hand in its
  program's log. All were single FULL or STYLE reviews; none was an
  orchestrator's read.
- **Denominators used:**
  - 453 (recorded single), which gives the highest single rate;
  - 896 (recorded + unrecorded), the lowest;
  - 580 (recorded + unrecorded, at least 200 lines changed), a middle
    figure.

**Size.** The median lines changed per PR:

| tier | median lines changed |
|---|---|
| dual | 1,756 |
| recorded single | 734 |
| unrecorded | 189 |

Dual units are bigger and reach more pinned ledgers. That is part of
what "hard" means.

## 2. Escapes by tier

**What counts.**
- An issue under `work/` (on main or pruned) that reports a red on main
  or a regression and attributes it to the PR that introduced it.
- The attribution must be one of: a first-bad bisect, "since PR N"
  stated as the cause, or the fix naming the PR.
- 314 red/regression issues were opened since 09-23 and were read by
  title and attribution sentences. 261 of them since 10-11.
- 128 of the 261 carry an attribution. The rest name no PR: "unattributed",
  "since 11-03..11-07", varying-seed fuzz reds, hosted-runner flakes. Every
  rate below is therefore a lower bound, for both tiers alike.
- The list (`ESCAPES` in the script) holds 150 issues in 101 events. Each
  event records its kind, where the row runs, its class and the strength
  of the evidence.

**Against readout 6's list.**
- Readout 6 counted 28 units since readout 5, 16 of them H.
- This scan finds 50 dual units, 32 of them H.
- The additions include several bisected behaviour regressions:
  - DR-171 (#4604): touching operands built as disjoint;
  - DR-193 (#4551): cap fits stop at 1e-12;
  - DR-294 (#5333): an escalation at 1e-12;
  - DR-344 (#5768): the teapot join's refusal moved.
- Readout 6's scan read only items on main and missed the pruned ones.

### 2.1 Units with at least one attributed escape, 2026-10-11..11-09

| tier | any escape | behaviour regression | sole cause, not "suspected" |
|---|---|---|---|
| dual, all | 50/296 = 0.169 (0.128–0.217) | 15/296 = 0.051 (0.029–0.082) | 41/296 = 0.139 |
| dual H concurrent | 32/166 = 0.193 (0.136–0.261) | 12/166 = 0.072 (0.038–0.123) | 26/166 = 0.157 |
| dual M holdout | 6/41 = 0.146 (0.056–0.292) | 0/41 (0–0.086) | 3/41 = 0.073 |
| dual M sequential | 12/88 = 0.136 (0.072–0.226) | 3/88 = 0.034 (0.007–0.096) | 12/88 = 0.136 |
| single, recorded tier | 47/453 = 0.104 (0.077–0.136) | 14/453 = 0.031 (0.017–0.051) | 33/453 = 0.073 |
| single, ≥200 lines | 47/580 = 0.081 (0.060–0.106) | 14/580 = 0.024 (0.013–0.040) | 33/580 = 0.057 |
| single, all code PRs | 47/896 = 0.052 (0.039–0.069) | 14/896 = 0.016 (0.009–0.026) | 33/896 = 0.037 |

95% Clopper–Pearson intervals. "Sole cause" drops the events that a
single issue spreads over several PRs (ledger drifts bisected to 3–6
PRs) and the three suspected attributions (DR-170, DR-216, DR-456).

**Per unit of code changed.**
- Dual: 4.7 escapes per 100k changed lines.
- Single: 6.4 per 100k.

**Within size bands.** Units with an escape, out of units in the band:

| lines changed | dual | single |
|---|---|---|
| < 500 | 3/30 = 0.10 | 7/482 = 0.015 |
| 500–1,500 | 10/90 = 0.11 | 19/300 = 0.063 |
| 1,500–4,000 | 20/123 = 0.16 | 13/92 = 0.14 |
| ≥ 4,000 | 14/49 = 0.29 | 3/22 = 0.14 |

**Arms.** The M sequential arm (one review, a second only if the first
raised a MAJOR) escapes at 0.136 against 0.146 for M holdout pairs. This
agrees with readout 6: there is no sign the conditional arm leaks more.

### 2.2 Answer to "any sign those issues are more common in single review units?"

No. Per unit, single-review units escape at half the dual rate or less,
whichever denominator is used. In the one band where the two tiers
overlap in size (1,500–4,000 lines) the rates are equal (0.16 against
0.14). Per changed line, single escapes slightly more (6.4 against 4.7
per 100k), and the intervals overlap.

**The confound runs one way.** Dual units are chosen because they are
tricky or broad:
- they are 2–3× larger;
- they touch more crates;
- their PRs move more of the pinned censuses, goldens and ledgers that
  produce most of these reds.

Absent a second review, they would be expected to escape *more* per
line, not equally. So a size-matched rate that is similar is the pattern
a working second review would produce. It is not evidence that the
second review is useless.

**How far the data can go.** It can go no further than that, for three
reasons:
1. The tier was chosen by difficulty, with no draw, so nothing here
   separates "the second review helped" from "hard units are no harder
   to keep green once reviewed twice".
2. The single denominator is uncertain by a factor of two.
3. Half of the reds are unattributed.

The one randomised comparison is the M holdout against sequential arms
(section 2.1), and it shows nothing either way at n = 41 and 88.

### 2.3 What the escapes are

Units, both tiers, 2026-10-11..11-09 (a unit can carry two tags):

| | dual | single |
|---|---|---|
| **kind:** pin / census / golden moved, no wrong answer | 33 | 32 |
| behaviour: refusal, escalation, wrong value, slowdown | 15 | 14 |
| build: rustdoc, compile, feature build | 2 | 1 |
| **where:** slow set | 30 | 30 |
| 1e-6 / 1e-12 rows | 12 | 6 |
| feature-gated (probe, all-features, per-op-postcondition) | 4 | 5 |
| nightly-only job (rustdoc gate, demos/tour) | 3 | 2 |
| pncad-py suite | 0 | 4 |
| fast set in a crate the change filter skipped | 1 | 0 |
| **class:** CI blind spot (an existing row the PR gate never ran) | 47 | 42 |
| own new row never passed at an unrun ε / feature | 2 | 3 |
| crossed (two PRs each green alone) | 1 | 2 |
| environment / flaky | 0 | 0 |

The last row is zero only by construction: hosted-only flakes and
varying-seed fuzz reds name no PR, so none is in the list.

**Could a review plausibly have caught them?**
- **A reading review: almost never.** The pin and census reds are a
  count or digest moving somewhere the diff does not show. Reading the
  diff does not reveal them; running the census does.
- **A review that runs the slow set and all three ε rows of the crates
  the PR touches: almost all.** Reviews run under the dual protocol often
  do run these, and these still got through two reviews.
  - The five own-row cases (5269, 5704, 5753, 5661/5666) are the clearest
    review-reachable ones. In each, the PR's new row was red at an ε or
    feature from the day it landed.
- **Behaviour regressions** (a third of events) are the class that matters
  for correctness. Even these were all caught by an existing row within
  a day, so they reached main as visible reds, not silent wrong answers.

### 2.4 The early window (2026-09-23..10-10)

- Dual: 8/160 = 0.050.
- H concurrent: 3/103.
- Single, recorded tier: 10/240 = 0.042.
- Per 100k changed lines: dual 1.5, single 1.4.

The rates are lower in both tiers because the `main-red-` filing
convention and nightly triage lanes had not yet started. The same
finding holds: the tiers are indistinguishable.

## 3. What the late escapes cost

**Recorded facts.** 84 escape events in the window, 126 issues:

| | all | pin | behaviour | build |
|---|---|---|---|---|
| events | 84 | 53 | 28 | 3 |
| filed more than once | 33 (39%) | 17 | 14 | 2 |
| with a P0 filing (gate-blocking) | 24 | 14 | 8 | 2 |
| days, merge → first issue, median (IQR) | 0 (0–1) | 0 (0–1) | 1 (0–1) | 1 |
| days, issue → closed, median (IQR) | 1 (0–2.75) | 1 (0–3) | 1 (0–2) | 0 |
| fix PR lines changed, median (IQR) | 62 (23–226), n=41 | 72 (26–385), n=28 | 28 (7–174), n=12 | 135, n=1 |

**What the table means.**
- Escapes are caught by the nightly almost at once and fixed quickly in
  small diffs.
- The waste is duplication: 39% of events were triaged by two or more
  lanes, each filing its own issue.
- The other cost is friction: 64 of 457 dual-review rows record working
  around a main red in their own CI. One example is PR 4597's lane,
  "stood the PR down once" over #4609's red. Some lanes merged over known
  reds.
- An escape's introducer has a median of 1,580 changed lines; the fix,
  62.

**No fix lane's token cost is recorded per escape.** So the cost is
modelled from the 22 lane costs that program logs do record: a median of
233k tokens per lane (IQR 204k–287k), over about 40–90 minutes.
- **A pin escape:** triage at a quarter-lane per filing, plus a
  quarter-lane to re-bless. That comes to **about 141k tokens, or 0.6 of
  a review** (range 94k–349k).
- **A behaviour escape:** triage and bisect at half a lane per filing,
  plus a fix lane, plus a FULL review of the fix. That comes to **about
  669k tokens, or 2.7 reviews** (range 184k–1.04M, 0.6–4.9 reviews).
- **Weighted by the observed mix:** about **324k tokens, or 1.3
  reviews, per escape event.**
- These are estimates from proxies (lanes per filing, the fraction of a
  lane), not measurements.

**The same defect fixed up front**, inside the unit's own fix pass, by a
lane that already holds the context:
- a pin: 5k–40k tokens, a re-bless and a line in the PR body;
- a behaviour fix: 30k–150k tokens.

**The ratio of late to up front is about 9× for pins and 7× for
behaviour.** The honest range is 1.2× to 70×. The ratio is wide because
neither side is measured per event. The direction is not in doubt.

**The catch:** what finds these defects up front is running the rows.
That is CI minutes, not a reviewer. A second reviewer finds them only if
it runs those rows, and the escape rates in section 2 say both reviewers
of a pair usually do not.

**The second review, for comparison** (readout 6):
- about 250k tokens (IQR 212k–292k) and 1–2 hours;
- no latency when it runs concurrently.
- At H it yields 0.283 tallied blocking defects per fair pair, so a lone
  review would ship about 0.14 per unit. That is about **1.8M review
  tokens per blocking defect caught.**

## 4. Guidance

**Is the H-tier dual review worth it?**

- **Not on the token arithmetic of reds.**
  - Suppose each of the 0.14 blocking defects a lone review ships per H
    unit later surfaced as a behaviour escape and was fixed then. That
    costs about 95k tokens per unit (26k–147k), against 250k for the
    second review.
  - Break-even needs a shipped defect to cost about 1.8M tokens, or 2.6
    behaviour escapes.
  - Nor does the second review visibly cut the reds themselves: the H
    escape rate (0.19 per unit, 0.07 for behaviour) is no lower than the
    single tier's, per line.
- **Yes on what the tallied findings are.**
  - Readout 6's tallied findings are demonstrated wrong answers: a wrong
    volume, `inf` served, a false pass at 1e-12, an inverted complement.
  - The silent review "did not exercise the region", which means no row
    covers it.
  - Such defects do not turn main red. Every escape here surfaced
    because an existing row turned red; none is a silent wrong answer
    found later. That fits the tallied class being silent, not its
    being harmless.
  - For a fail-loud kernel, one silent wrong value in a shipped
    operation easily costs more than 1.8M tokens of review. It may never
    be found, or be found by a user, and it is then bisected against a
    month of history.
  - So the H second review buys correctness that nothing else in the
    pipeline buys. It does not buy CI greenness.
- **What would change the answer.**
  1. **Fix the CI blind spot first.** Run the slow set and the 1e-6 and
     1e-12 rows of the crates a PR touches before merge: per-PR, or in a
     merge-queue job. Add the probe/all-features pass and the pncad-py
     binding census when those crates move.
     - That removes about 90% of the escapes in both tiers.
     - At about 1.3 reviews per escape and about 0.10–0.19 escapes per
       unit, it is worth about 0.13–0.25 of a review per unit, in CI
       minutes instead.
     - Then the escape rate becomes a usable measure of review quality.
  2. **If later evidence showed tallied-class defects surfacing as
     cheap reds,** the token case for H duals would weaken. Today there
     is none.
  3. **If the M arms' holdout miss rate** (0.10–0.15 per pair, readout 6)
     **stays low,** the sequential arm remains the right price for M.
     Escapes do not distinguish the arms (0.136 against 0.146).
- **Record escapes at the source.** The orchestrator who files or fixes
  a red that bisects to a logged unit appends the rule-11 line. Only 2 of
  the 50 dual units here have one (DR-200, DR-382). Also deduplicate filings: 33 of 84
  events were triaged twice or more, which is the cheapest cost to cut.

**For the Opus/Fable step (one Opus + one Fable reviewer on dual units):**

- **Keep the tallied-finding rate as the endpoint.** Opus/Opus at H is
  0.217 of fair pairs (0.168–0.272) and 0.283 findings per pair.
  - The question to ask is whether a cross-model pair's unilateral,
    demonstrated, blocking findings run clearly above that.
  - Also ask which model's review is the silent one, by a recorded draw
    of which seat each model takes.
- **Do not use escapes to judge the switch.**
  - Attribution covers about half the reds.
  - About 90% of escapes are rows neither reviewer ran. They track CI
    selection and diff size, not the reviewer's model.
  - With ~0.17 escapes per dual unit and wide intervals, a model effect
    on escapes would need hundreds of units to show.
- **If escapes are to be a secondary endpoint, fix them first.**
  - Fix CI selection first, or it swamps the signal.
  - Record rule-11 lines.
  - Tag each escape with whether either review's report touched the row
    or region that went red. Then "both reviews missed it" and "no
    review ran it" can be told apart.
- **Compare like with like.** Compare cross-model pairs with Opus/Opus
  pairs within the same size band and letter. Escape rates rise
  steeply with diff size (section 2.1: 0.10 → 0.29 for dual from
  <500 to ≥4,000 lines).

## 5. Caveats

- **Attribution is half complete.** 128 of 261 red/regression issues
  since 10-11 name an introducing PR. Unattributed reds (date-range
  "since", varying seeds, hosted-only flakes) are missing from both
  tiers. Nothing suggests they fall unevenly, but nothing shows it
  either.
- **The single denominator is uncertain by about 2×.** About a third of
  merged PRs have no merge commit and are invisible to the population
  scan. Tier records match by PR number or branch for only about half of
  the non-dual code PRs.
- **ESCAPES was read by hand from titles and attribution sentences, not
  adjudicated** under the protocol's rule 6.
  - Multi-PR ledger drifts count against every PR named.
  - Three dual attributions are only "suspected".
  - Pins are counted as escapes. Ev has not ruled on readout 5's DR-144
    question, whether a moved pin counts. Excluding pins roughly divides
    every rate by three and leaves the comparison unchanged.
- **Costs are modelled.** Token costs per fix lane are not recorded; the
  model uses 22 recorded lane costs and stated fractions. Review costs
  are recorded on fewer than 5% of reviews since readout 5. Red-on-main
  blocking time is not measured beyond P0 counts and rows that mention a
  main red.
- **Selection.** Dual tier is chosen by difficulty, and size is only a
  partial control for it. No conclusion here is causal except the M arm
  comparison, which is underpowered for escapes.
- **Silent defects are out of scope.** The escape scan sees reds and
  named regressions. Latent wrong answers found later, by a review of
  another PR, are seldom attributed to their introducer and are not
  counted. That is the class the H second review targets.
