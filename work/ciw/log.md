# CIW log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/ciw/plan.md`. A/B band 1500–1599
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose CIW section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `main-latently-red-at-tier-all` from `work/issues/`
- `render-lanes-red-at-missing-merge-ref` from `work/issues/`
- `retire-render-automatic-matplotlib-fallback` from `work/issues/`
- `hosted-renderer-announces-itself-preview-only` from `work/issues/`
- `nightly-pin-reading-idiom-four-copies` from `work/issues/`
- `mirror-parity-never-compares-flags` from `work/issues/`
- `python-suite-zero-test-guard-three-copies` from `work/issues/`
- `committed-conflict-markers-reach-main` from `work/issues/`
- `bounds-tripwire-blind-to-named-alias` from `work/issues/`
- `cache-rendered-cells-on-input-hash` from `work/issues/`
- `d107-release-profile-job-lives-in-nightly` from `work/issues/`
- `rustdoc-gate-disagrees-with-workspace-doc` from `work/issues/`
- `rustdoc-gate-private-intra-doc-links` from `work/issues/`
- `doc-gate-two-unread-axes` from `work/issues/`
- `sccache-trial-verdict-to-read` from `work/issues/`
- `geom-brep-test-unused-edgedescription-import` from `work/verbs/`
- `perf-history-cannot-identify-its-host` from `work/perf/`
- `facade-guards-defer-to-rustdoc-json` from `work/lib/`

LIB's clippy-row item (`the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row`)
landed on main the same day and stays closed in `work/lib/`.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## The opening re-read (2026-09-04)

The slate was built on 2026-09-03 out of items filed between 2026-08-09
and 2026-09-03. Before dispatching any of it, the orchestrator checked
all eighteen against the tree rather than against their own bodies. Six
moved. Ev then ruled on three of them in chat the same day.

**The finding that moved most of them.** `evgunter/cad` went **public**
on 2026-09-03 (`5cc16e81`…`483212ef`). Standard-runner minutes are free
and the runner is 4 vCPU / 16 GB, up from 2 / 7. That kills the premise
of `docs/CI-MINUTES-2026-08.md` — *"the Actions allowance was being
consumed faster than the work justified"* — and with it the stated cost
argument behind F3, this month's demotions to the nightly, and at least
two declines on this slate. Ev directed that CIW open the re-costing as
a unit rather than assume the answer:
`work/ciw/f3-recosting-on-a-public-repo`. Same day, before the
visibility change, the account's Actions spending limit denied job
starts for two and a half hours — the old regime's last data point, and
already closed in `work/issues/`.

**What the tree said about the slate.** Three items were closed as not
live. `main-latently-red-at-tier-all` was the plan's FIRST unit and has
nothing to fix: the pyo3 half was repaired at `5859c8c6` (its own
comment said so), and the viewer bin/lib doc collision turns out to be
a **cargo** diagnostic rather than a rustdoc one, so `-D warnings`
never reaches it — `scripts/doc-gate.sh --pr --scope '--workspace'` is
green on this tree and `cargo doc --bins -p viewer --all-features`
exits 0 with the warning printed. `rustdoc-gate-disagrees-with-workspace-doc`
was answered by running both sides: `SweepStrategy::Idealized` is
`#[cfg(feature = "sweep-testing")]`, the gate documents at
`--all-features` and resolves it, a plain `cargo doc` at default
features does not (exit 101 on both prose sites) — a feature selection,
not a misconfiguration. `sccache-trial-verdict-to-read`'s carrier, PR
1648, had merged.

Two more closed on Ev's call (2026-09-04):
`committed-conflict-markers-reach-main`, because a committed marker is
self-limiting — obvious, repairable later, nothing compounds on it, so
it is a poor subject for an absence detector; and
`python-suite-zero-test-guard-three-copies`, never observed and needing
a developer tool's contract plus a parity seam moved. The orchestrator
recorded the counter-evidence on the first before closing it.

Two were re-homed for being outside this program's fence, and one new
item filed to S-TCOST for the same reason:
`bounds-tripwire-blind-to-named-alias` (the tripwire is now
`scripts/gates/bounds-allowlist.sh`, whose ratified header argues
against the ask as KNOWN GAP 3, with a fixture pinning the gate to pass
on exactly those uses), `d107-release-profile-job-lives-in-nightly`
(the whole fix is an edit to a Track P finding), and
`rust-cache-never-restores-across-branches` (PR 1648's finding (d) —
five of seven build jobs restored nothing; caches are S-TCOST's knob).

`cache-rendered-cells-on-input-hash` is parked rather than dropped: its
staleness-window argument never rested on minutes and survives, but PR
1648 measured the Actions cache budget evicting a ~205 MB entry inside
the hour, and a render-cells cache would both miss and crowd out the
build lanes' entries.

**Two items got sharper rather than weaker.**
`nightly-pin-reading-idiom-four-copies` has a confirmed instance now —
`c5263958`, "the gated-suite re-take's pin-read step had unbalanced
quotes and never ran", the same idiom, found by a person reading a log.
`perf-history-cannot-identify-its-host` is now urgent rather than
tidy: the runner class changed on 2026-09-03, so a step change of
unknown size runs through all three histories at that date and the
`environment` block cannot name it.
`geom-brep-test-unused-edgedescription-import` grew from one unused
import to four, in files it does not name (measured, not assumed).

**Filed new, beyond the two above.**
`nightly-demotions-have-never-run`: TCOST-C1/C2/C3 moved three jobs
into `nightly.yml` on 2026-09-03, and none has executed — the last
completed nightly (run 33741400551) predates all three merges and its
job list does not contain them; the only run since was a cancelled
dispatch. `c5263958` is the class already firing once. Ev, 2026-09-04:
read tonight's scheduled run rather than forcing a dispatch.

Eighteen items to ten units plus one unscheduled reading. No branch
exists yet and no unit is cut. The first dispatch claims its ordinal
from the band and records it in `docs/MODEL-AB-LOG.md` — though on Ev's
direction (2026-09-04) this program runs **no A/B protocol at all**:
one subagent style review per unit, and a second reviewer for
correctness only where a unit earns it, named in its PR with the
reason.

## First three units dispatched (2026-09-04)

Three implementers ran concurrently in isolated worktrees, on
non-overlapping territory: `ciw/render-lane-merge-ref`,
`ciw/pin-reconciler`, `ciw/perf-host-identity`. One style review each,
per the posture Ev set; no A/B row and no dual on any of them. PRs 1724,
1723, 1722.

**PR 1723 merged** (`nightly-pin-reading-idiom-four-copies`): one
`scripts/ci-pin.py` replaces five `sed` sites, anchored to `ci.yml`'s
workflow-level `env:` block and refusing on ambiguity rather than
picking. The lane demonstrated the defect rather than asserting it —
with a job-level `NEXTEST_VERSION: "0.9.999"` planted above the
workflow block in a copy of the real `ci.yml`, the retired idiom
silently answers `0.9.999` and the reader refuses, naming both lines.

**What the reviews were worth.** Both style reviews found the same
species of defect, and it is worth naming because it is not a typo
class: **a true mechanism written up as a claim slightly stronger than
it supports.**

- 1723's `MIRROR_EXEMPT` entry, written to justify the new reader,
  claimed the local half "has no pin to read". It has five, as
  hand-restated literals — including `local-scripts/ci-local.sh:588-589`,
  an executable error message telling a developer to install
  `--version 0.9.140`. The exemption written to justify fixing five
  machine-read copies of a pin walked past five human-read ones, on the
  same PR. Filed as `local-half-restates-ci-pins-as-literals`; no site
  converted, because `ci-local.sh`'s error text may want its literal on
  purpose and that is a different unit's call.
- 1724's sweep receipt, committed into `render.yml`, said these two
  lanes were the only checkouts under `.github/workflows/` naming a
  `ref:`. Thirteen do; all twelve of nightly's name one, eleven spelled
  `${{ inputs.ref || github.sha }}`. The conclusion survived — every
  other one names an *object*, so none could produce the 103 reds — but
  a false receipt in the file is worse than none, because the next
  author trusts it.
- 1724 also claimed "a lane log exists ⇔ this lane had the tree",
  which is false in exactly the case the unit is about: a lane whose
  checkout fails has a log and no tree.
- 1722's `criterion/README.md` turned the PR's own hedge into a rule —
  "two samples whose `cpu_model` differs are not comparable at the ~10%
  resolution below" — with nothing measured behind it. The 21.6%
  excursion has an unknown host, which is why the unit exists.

All four were repaired in fix passes rather than waved through.

**Two lanes independently re-filed a tracked issue.** The `pncad-py`
`TAG_INVENTORY` red (`work/issues/pncad-py-tag-inventory-misses-two-measure-tags`,
filed 2026-09-03) was re-filed by unit 5 and unit 1 as new items;
`work.py lint` catches neither, because each file is individually
well-formed. Both copies were withdrawn and their new evidence folded
into the original, which now carries a triage guard telling the next
lane to append rather than file. The evidence that accumulated is worth
more than the duplicates cost: the red is **shard `2/2` only**, green on
`1/2` in every run, across both compile modes and all three tolerance
rows, on seven branches — and no run on `main` has drawn a point that
executes the test since the two tag values landed, because main's push
runs classify docs-tier. It is a merge-base property, not a draw.

That last fact is F3's accepted residue caught in the wild, and it is
now evidence in `f3-recosting-on-a-public-repo`: the compensating
control works, but it bills the cost to whichever unrelated lane draws
the point, and nothing routes it back.

**The charter was wrong about its own territory** and is corrected
above. `docs/perf-data/*` is PERF's and `crates/*/tests/*` is S-TCOST's;
"files no live program owns" was false for both. Found by unit 5's lane
reading `work.py territory`'s warning on its own diff rather than
ignoring it.

## 2026-09-04 — full configuration runs reinstated (`reinstate-full-configuration-runs`)

Ev authorised it in chat: *"feel free to reinstate full runs instead of
sampling"*, because *"CI is weakened right now because of sampling only
certain configurations to run"*. A hosted run gated one point of
{default, `interval`} x {default eps, 1e-6, 1e-12}, drawn from the head
SHA; it now gates all six, as two archives and twelve `test (…)` jobs.
`_forces_interval` — the `interval-transcendentals/` lane pin — is
deleted with the draw it existed to pre-empt.

**It is not a 6x multiplier and the measurement says so.** The nextest
archive is built per COMPILE MODE and eps is runtime env, so six points
are two builds and twelve test legs. Over 71 code-tier runs in one 3.9-h
window: **+15.4 job-minutes per run** (24.5 → ~40) and **+57 s of
critical path on the half of runs that would have drawn `default`**, +0
on the other half. That agrees with PR 1796's independent +15.6 to
within 0.2 job-minutes; the two lanes' per-HOUR figures differ (+283 vs
+161) only because their windows differ in busyness, and both are right
for their window. The block that supersedes
`docs/CI-MINUTES-2026-08.md`'s 2026-08-22 sampling section carries the
population and the derivation.

**The dispatch inputs and the `CI-Config:` trailer stay, repurposed.**
They no longer buy coverage for the lane or eps — a run has it — so they
NARROW a run instead, which is what a fast re-gate of one axis wants, and
they remain the only way to choose the k-lint row. Their neutral option
was renamed `sample` → `unset`: one input list cannot carry two
vocabularies once `lane: sample` names a draw that does not exist.

**Two residues, filed rather than disclosed in prose.**
`klint-row-still-sampled` — the third dimension is still drawn 1-in-5,
because five unifications are five compiles rather than one archive
replayed, and it was not re-costed. `interval-only-selection-premise-restored`
— the 2026-08-22 reversal of the interval-only selection was forced by
the lane draw, so its original premise holds again; restoring the
subtraction would REDUCE what a run gates and is a cost lever with its
own argument to make.

**`scripts/ci-filter.py` is S-TCOST's and the edit was made anyway**,
under Ev's authorisation and announced in the PR by name. No open
S-TCOST item assumed the draw; one parked one —
`skip-eps-battery-by-observing-oncelock` — has its premise RESTORED,
since "the ε battery runs the whole suite at three ambient ε values" is
true again.

### 2026-09-04, the style review's fix pass

Six findings, all taken. Two are the standing warning firing again on this
lane's own work.

**The wall-clock claim was wrong and it was the number that made the trade
look free.** "The added ε legs cost ~0 wall" reasoned from the legs starting
together; **wall follows their MAXIMUM**, and six legs have a larger maximum
than two. Measured on TIER=all runs: **+20 s** on a run that would have drawn
`interval` (the ε legs' own cost), **+172 s** on one that would have drawn
`default` (mostly the interval archive), **≈ +96 s in expectation** — not the
+28 s published. The last job on the path is
`test (interval, eps = default, 1/2)`, the first ε row's shard 1, at
156/151/135 s across three un-sampled runs, ending at each run's wall exactly.

**The trailer is now additive-only; the dispatch still narrows.** The review
found a cut neither this lane nor the orchestrator had: the two spellings were
one applier only while either could merely substitute one drawn point for
another. Once one can SUBTRACT they are opposites, and `ci-filter.py`'s own
argument decides which — a dispatch is typed by whoever is standing there, a
trailer is copied and rides one push. So a trailer may never gate less than no
trailer at all, and it keeps the k-lint row, which it never subtracted from. A
narrowed run also gets a `::warning::` run-page annotation, keyed on the VALUE
rather than on `CONFIG_SOURCE`.

**Two miscounts of this lane's own.** `ci-local.sh` said "three sampled
dimensions rather than two" where there is one — in a file this lane's own
disposition table listed as fixed, a hundred lines below a hunk it wrote. And
the stated sweep blind spot was wrong about itself: the survivors were not
prose "without any of those words", they were prose the two sweep passes never
crossed — one pass used a narrow phrase list over a broad path set, the other a
broad pattern over ONE file — and `drawn|the draw` never matched `draws` /
`drew` / `drawing`.

**Population re-derived on a closed window: 156 runs, 72 code-tier**, against
155/71 here and 154/72 from the review. Both were snapshots of a window still
open. Per-tier: **+14.9** job-min TIER=closure, **+19.4** TIER=all, **+15.6**
run-weighted — which is what the pooled figure had been measuring silently.

**Filed rather than reported**: `work/issues/fillet-specs-require-a-narrowing-ci-config`
— two live FILLET specs require `CI-Config: lane=interval` under `## Acceptance`,
which now narrows the gate AND, since the trailer became additive-only, reds
the classify step. Filed in `work/issues/` rather than on FILLET's slate,
which is that program's to claim by moving.

## Unit 8 — `f3-recosting-on-a-public-repo`, measured and asked (2026-09-04)

A measurement unit, so the deliverable is numbers and a question rather
than a diff. The full reading is in the item; the four that change how
this program talks:

- **A code-tier run is 7.4 minutes and 24.4 job-minutes** on the public
  4-vCPU runner (n=149 completed post-flip PR runs), against the
  13.75-minute critical path and ~87/62/40 billed minutes this program
  has been quoting. `--workspace` builds are **336 s / 388 s** against
  820 s / 840 s cold. The runner spec is read first-hand: run
  `33830873453`, job `100893490483`, `nproc` = 4, 15 GB.
- **This log's own explanation of the `TAG_INVENTORY` red was wrong.**
  It says *"no run on `main` has drawn a point that executes the test
  since the two tag values landed, because main's push runs classify
  docs-tier"*. The first clause holds; the reason does not. **45 % of
  main's push runs are code-tier** (90 of 200 ran `renders`, which needs
  `RUN_K_LINT=true`, set for every tier but `docs` —
  `scripts/ci-filter.py:1730`), and the test rows are skipped on all of
  them by F3's `github.event_name != 'push'` guard. The correction makes
  the instance *better* evidence about F3, not worse: nothing about the
  tier was involved.
- **The residue's cost is attribution, not latency.** The compensating
  control detected the composition in **11 m 41 s** (run `33788618577`,
  on `tcost/k2-unit`). What it could not do was say whose it was: **42
  red runs across 20 distinct branches** over the 8 h 11 m to the repair.
  The item's "seven branches" was an undercount of the same event.
- **Restoring the job set alone would have caught nothing.** The push
  run for the composing merge (`33787453014`) was cancelled after 246 s
  by the next push; a full run needs ~442 s. The proposal to Ev is
  therefore the job set **and** a per-SHA concurrency group for push
  runs, at +48 job-minutes an hour and $0.

Units 9 and 10 may now quote figures from
`docs/CI-MINUTES-2026-08.md`'s **2026-09-04 section** and only that one.

### Unit 8, fix pass after verification (2026-09-04)

A verification lane re-derived every load-bearing figure from 760
`ci.yml` runs and they reproduce, most to the digit. What did not
survive was prose, in the two shapes this program has been losing lanes
to all night:

- **A number stated backwards.** "PR runs still red first, by about four
  minutes" compared a run *creation* time (11 m 41 s) with a run
  *duration* (7.4 min). Measured: the PR run reds **17 m 29 s** after
  the merge (job `100761051102`, +348 s into run `33788618577`); a push
  run reaches the same offset **5 m 48 s** after it. **The push run reds
  first, by 11 m 41 s.** The error ran *against* the unit's own
  recommendation, which is why self-review missed it — a figure that
  weakens your case does not trigger the reflex that checks it. Worth
  keeping as a rule.
- **A scope stated too small.** The concurrency half was priced as "one
  line". It is three mechanisms: `render.yml:268–275`'s own gate-mode
  group on the caller's ref, which **starts** firing once the run-level
  group goes per-SHA and cancels `renders` on exactly the merges the
  change exists for; the `cache-on-failure: false` argument at
  ci.yml:1830–1840, argued *from* push runs being cancelled; and
  `renders`' `push_to` write to `main` at ci.yml:4203. The unit now says
  it does not price that and names it a second design pass.

Two more corrections, both self-inconsistencies rather than new facts:
"A without the concurrency half is measurably worthless" (it degrades to
burst-level attribution, ~2 merges — *better* than the ~4.4-merge window
the same document rejects option C for), and a population definition
that omitted the exclusion of cancelled runs, so nobody could rebuild
the frame (220/149 with them excluded, 268/195 with them in).

And one confirmation the unit had and did not use: **51 of 90 code-tier
push runs (57 %) are already cancelled**, at a 259 s median job set,
against 15 % of docs-tier pushes at 40 s. The aggregate 34 % was quoted
with its causality backwards.

Nothing in the measurement moved and the recommendation is unchanged.

### Unit 8 answers Ev, and changes its own recommendation (2026-09-04)

Ev's comment on 1796 asked two things, and the answers moved the unit
off the proposal it opened with.

**Q1 — full job set vs test rows only, on the record.** Two identified
composition instances (`TAG_INVENTORY` at `bdfa604b`; `MateFault::
Unleverable` at `50d9ba21`, filed as
`merge-order-semantic-break-reaches-main`). Measured clear air: 161 s
and **95 s** before the next push cancels; measured time-to-red:
`clippy` **+84…96 s**, the build +127…202 s, the test rows +348 s. So
the full set catches **1 of 2** as things stand (instance 2, by a 6–27 s
margin), the narrow variant **0 of 2**, and both catch 2 of 2 once push
runs stop being cancelled. **The row that catches instance 2 at all is
`clippy` — which the narrow variant does not restore.** The abstract
risk the unit named a revision earlier turned up in the record one night
later.

**Q2 — the configuration draw.** Priced: **+15.6 job-minutes per
code-tier run** (24.4 → ~40), **+161 job-min/h**, and **+22 s of wall
clock (~5 %)**, because six points are two builds and twelve 46–58 s
test jobs, not six builds. Filed as
`configuration-sampling-outlives-its-premise`; Ev has authorised the
change and a separate lane owns the edit. Attribution, stated carefully:
of five recorded main-reds, **zero** are the lane/eps draw's — but that
population is biased against exactly that class, so the record cannot
settle it and the argument has to rest on price. Note `k-lint`'s 1-of-5
sampler is a **second** sampler and is not in that price.

**And the unit now recommends against its own opening proposal.** Ev
authorised experimenting with a merge queue; priced against the same
two instances, a queue **prevents** them where the push gate only
**detects** them, at **the same runner cost** (44 vs 48 job-min/h,
simulated over the 200 observed merge arrivals). The throughput
objection does not survive contact: the median 308 s gap is not the
arrival rate — the mean inter-arrival is 826 s and utilisation is
**ρ ≈ 0.27**, so a serial queue is stable, and the cost is merge latency
(median 442 s, p90 676 s, max 857 s at batch ≤5) rather than backlog.

Two things that fell out of that and are worth keeping: ci.yml's
`!= 'push'` spelling means a `merge_group` event runs the full gate with
**no edit**, which is exactly the case its 2026-08-28 note argued for;
and required status checks are named, so **un-sampling is a
precondition** for a queue, which couples Q2 to the queue rather than
leaving them independent.

Nothing was enabled and no repository setting was touched: the queue is
a design put to Ev.

## Unit 10 — the façade-guards ruling, answered and landed (2026-09-04)

Ev ruled in chat: *"ok yeah no rustdoc json, your arguments are
convincing"* — disposition (2), the item's own recommendation. #696's
deferral is **closed permanently**: the three source-text scans in
`crates/pncad/tests/all.rs` are the enforcement, and no rustdoc-JSON
pass is to be built.

Two findings from this lane carried it. The multi-line `pub use` hole —
the strongest argument for the structural check — is **our scanner's
defect, not scanning's ceiling**, since `pub_use_names` in the same file
is already statement-based; and the classes rustdoc JSON uniquely
reaches (an `as`-aliased key, a key exposed as a public field or
associated type) have **zero live instances** and take a coordinated
two-crate edit whose second half already reds the completeness guard.
The nightly-vs-per-PR placement sub-question is **moot rather than
unanswered**: the nightly toolchain requirement follows the check
wherever it runs, so placement never disposed of the format-instability
cost, and per-PR would have put an unstable schema on the merge path.

What landed on PR 1841 beyond the record: all three guard doc comments
rewritten so they describe the permanent mechanism with its limits
named, instead of pointing at a check nobody will build — a sanctioned
drive-by into LIB's file, announced in the PR, doc comments only. The
logic half is filed as `work/lib/lb13-guards-are-line-local` for LIB,
carrying the line-locality hole and the stale `root_declared_pub_names`
module count.

Numbers re-derived rather than inherited, and two of the item's moved:
the façade has **74** `pub use` statements in code (the item's 77 was a
raw grep that counted three comment lines), **33** of them multi-line,
**17** of those naming an `editor_core::` path (the item's 15 counted
only the `editor_core::{` spelling and missed `mc::` and `report::`).
The 32/28 `pub mod` counts in `editor-core`'s root re-derived unchanged.

## 2026-09-04 — the merge queue trial, designed and prepared (`merge-queue-trial`)

Ev ruled for the queue after PR 1796. This unit is the design, the
workflow support and the runbook; **nothing is enabled and no repository
setting was touched.**

**The central fact holds and was re-checked at the site.** Every gated
job's `if:` was read out of the parsed workflow: fifteen read
`github.event_name != 'push' && …`, two carry no `if:` at all, the two
cache primers are `push`-or-`dispatch` only, and `renders` is
`!= 'workflow_dispatch'`. So `merge_group` needs the `on:` key and no
`if:` edit — which is what the 2026-08-28 spelling note was written for.

**Three things that enumeration missed**, none fatal, two of them
corrections to PR 1796: `renders` *would* have run under `merge_group`
(excluded here — no branch to re-baseline to, and the PR run and main's
push run both render the same tree); the change filter's `HEAD^1` basis
is **correct** under `merge_group` rather than the open question it was
left as, because a merge group's parent is the group before it; and the
two test-cost report steps degrade with a **stated skip**
(`scripts/base-test-listing.sh:86-88`), not silently.

**The numbers were re-derived and PR 1796's queue pricing does not
survive.** That document priced "merge queue, batch ≤5" at 44 job-min/h
from a simulation in which batching reduced the number of CI runs.
GitHub's documentation is explicit that *"merge limits do not combine
`merge_group` builds"* — a group is built per queued pull request — so
**batch size is not a CI-cost lever at all** and the queue costs one full
gate per PR. Post-un-sampling that gate is **44.8 job-minutes** (was
24.4) and the merge rate over 23.96 h is **5.76/h, 41 % code-tier**.
**A queue run is not a PR run**, though: it excludes the render lanes,
which are 330 of a code-tier run's 2686 job-seconds, so what a merge
group actually costs is **40.2 job-minutes** (median of the per-run
difference, n = 9) and the queue costs **99 job-min/h = +1.66 mean
concurrent jobs**, against a queue delay today of 3 s. (This entry said
110 and +1.84 when it was written, on the PR-run figure; the 11 job-min/h
of difference is render work no merge group will ever run.) The 528 s
service time needs no such correction, and that was measured rather than
assumed: dropping the render-lane jobs from each of the 9 runs' wall
clock moves the median not at all.

**The lever that matters is build concurrency, not batch size.**
Simulated on the 138 observed arrivals at the measured 528 s / 42 s
service times: concurrency 1 gives a code-tier PR a **1312 s median and a
3774 s worst case**; concurrency 4 and above gives **528 s flat**, which
is one run's wall clock with nothing queued. The observed peak of groups
in flight is **4**. Recommended: build concurrency 5, maximum-to-merge 1,
"only merge non-failing pull requests" on, merge-commit method (which
`CLAUDE.md` requires anyway, and which also keeps `CI-Config:` trailers
out of a group head).

**The required check is one name: `gate ok`**, a job this unit adds. It
runs on every event but `push`, `needs:` all twenty other jobs, and reads
the run's own job list through the Actions API: it reds if any job is
still running (which is how a stale `needs:` announces itself) or
concluded anything but success/skipped/neutral. **One name because two of
the twenty names are COMPUTED** — `test (eps = …)` interpolates
`eps_rows`, and a required-check list is shared with the pull-request
side where a `CI-Config:` trailer or a dispatch can still narrow the
matrix — **and because a hand-kept list goes stale silently.** Requiring
it gates pull requests too, which is a real tightening of `CLAUDE.md`'s
"agents merge their own PRs" and is called out as one.

**A third reason was led with and is withdrawn (2026-09-04, style
review).** It read: most jobs are *skipped* on a docs-tier merge and this
unit did not establish that a skipped check satisfies a required one.
GitHub documents that it does —
`content/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks.md`:
*"A job is skipped by a conditional | The job reports 'Success'"*, and
*"Successful check statuses are `success`, `skipped`, and `neutral`."*
The pending-and-blocking row is the WORKFLOW-level skip (path or branch
filtering, `[skip ci]`), which no job here is. The design stands on the
two reasons above; the withdrawn one was a documented non-problem
presented as an open question, and it was the one all three prose sites
led with. The same page earns the job its `always()` and is now cited for
it. Two more corrections out of the same review: the `neutral`
accept-list entry's reason was "that is how a render lane reports drift"
— false, and `rebaseline-lane/action.yml` says so (*"a workflow JOB
cannot conclude `neutral`"*; the drift signal is a CHECK RUN, which this
job's `/jobs` read cannot see) — the entry stays because those three are
GitHub's definition of a pass; and the reader came **out of the YAML**
into `scripts/check-run-jobs.py` with a `--selftest` over its seven
decision paths, because six of them never execute on a real run and
nothing in the tree re-drove them.

**And requiring `gate ok` makes a failed render lane block a pull
request**, which no render lane could do before. Kept deliberately, on a
re-measured rate: the 103-reds-in-89-runs population is **pre-fix** — PR
1724 landed as `a5d9f41a` — and since that merge there are **8 failed
render-lane jobs in 259 render-bearing runs (777 jobs), none of them at
checkout**, seven of the eight being one real break that also reddened
`main`. Dropping `renders` from `needs:` would not have removed the
blocking anyway: the sweep reads the run's job list, not `needs:`.

**The k-lint finding, resolved to an ordering dependency.** The obvious
version — "a sampled row cannot be a required check" — is **false**:
`k-lint (gate)` is one job with a fixed name and no matrix. The real
problem is that a merge group's head is a new SHA, so a queue run draws
its **own** row: a PR green on row X can be ejected by row Y, the author
cannot reproduce it by re-running, and re-queueing draws again. Ev has
authorised un-sampling k-lint; that lands first
(`klint-row-still-sampled`), then the switch.

## 2026-09-04 — the `CI-Config:` trailer deleted (`delete-config-trailer`)

**Ev, reading PR 1855:** *"i see in 1855 it's still talking about the ci
config trailer; that code should be deleted since it's no longer live"*. He
is right, and the reason is arithmetic rather than taste. `WHOLE_BY_DEFAULT`
gained `klint` in PR 1850, so it carried all three dimensions; `parse_config`'s
`additive_only` arm red any trailer value that was not the whole-dimension
value. That leaves a trailer exactly two possible effects — name the value
that is already the default and change nothing, or name anything else and red
the classify step. **There is no input that makes it useful**, which is
different from an input nobody happens to use.

The entry above (`reinstate-full-configuration-runs`) called the two
spellings *opposites* and kept the trailer as the half that can be read back
off the commit. That reading survived until the k-lint row went. Once every
dimension ran whole, "readable back off the commit" had nothing left to read:
the only thing a trailer could legally record is that its author asked for
what the run was already doing.

**What went.** `config_from_message`, `CONFIG_TRAILER`, `additive_only`, the
`WHOLE_BY_DEFAULT` table (nothing else read it), `--config-from-message` and
its call site; ci.yml's `HEAD_COMMIT` env and the `git log -1 --format=%B`
line that fed it; and every selftest case that exercised the trailer — the
regex near-misses, the case-insensitivity, the precedence pair, the
additive-only refusal loop and the CLI round-trip — **with the `--selftest`
prose that claimed them**, because a coverage sentence outliving its
assertions is the failure this program keeps meeting in other people's files.
Two assertions replaced them: `--config-from-message` reds rather than being
ignored (the `--seed` precedent, one lane over), and the invocation-narrows
loop that already walked every legal value of every dimension.

**`CONFIG_SOURCE` now has two words, not three.** `unsampled` and
`requested`. `commit-trailer` joins `sampled` and `pinned` as a value no run
can print, and both prose sites say so out loud — a source vocabulary that
names a thing that cannot happen is a reader's wrong turn.

**Nothing about what runs changed.** `LANE=both`, `EPS=all`,
`KLINT_ROW=all`, and `workflow_dispatch` still narrows.

**The sweep was most of the work.** `grep -rn "CI-Config"` found the spelling
taught as live in `memories/agent-lane-operations.md` (Ev's message is the
`CLAUDE.md` sign-off for that one bullet, and it is deleted rather than
rewritten — PR 1855 trimmed it once already and Ev's point is that the whole
thing goes), `docs/prompts/implementer-discipline.md`, `docs/K-REPORT.md`,
`local-scripts/ci-local.sh`, this program's merge-queue runbook, and **five
live specs** across FILLET, PCURVE, EXCH and TRIM. `work/issues/`'s
`fillet-specs-require-a-narrowing-ci-config` is closed by the same sweep,
with the correction that its central sentence — a narrowing trailer REDS the
classify step — is now false in the quieter direction: the line is inert, so
an implementer obeying a stale spec gets no error and no interval lane
either. Logs, `docs/CI-MINUTES-2026-08.md` and `docs/MODEL-AB-LOG.md` keep
their mentions: those are dated records of what was true, not instructions.

## 2026-09-05 — the merge queue is not available to this repository, and the unit closes (`merge-queue-trial`)

**The entry above this one is the story of a design that could never
have been enabled.** Ev went to turn the queue on after PR 1845 landed
and found no toggle. It is not a misconfiguration and not a plan tier:
`github/docs@main`, `data/reusables/gated-features/merge-queue.md`, says
in full that *"Pull request merge queues are available in any public
repository owned by an organization, or in private repositories owned by
organizations using GitHub Enterprise Cloud"* — and `evgunter/cad`
(repository id 1302372371) reports `"visibility": "public"` with
`"type": "User"` on its owner. A personal-account repository is outside
both arms of the grant.

**Going public on 2026-09-03 removed the BILLING gate on Actions
minutes. Merge queue is gated on OWNERSHIP.** Two different gates, and
this program conflated them — the whole re-costing arc was built on the
first and the queue needed the second.

**Ev has ruled** (2026-09-05): *"ok i don't plan to move this to an
organization"*. So the avenue is closed rather than blocked, and
`merge-queue-trial` is `closed`, not parked and not deferred: nothing is
waiting on anything.

**The process failure, named, because this program has been filing this
exact shape all day.** The unit costed the queue to two decimals — 99.5
job-min/h, ρ = 0.39, a latency simulation over 138 observed merge
arrivals — argued the required-check design, shipped a nine-setting
runbook, and wrote a section listing six things it could not
demonstrate. **Nobody, at any point — not the unit, not its style
review, not the orchestrator — asked whether the feature was available.**
It is a true mechanism carrying an unchecked premise, arriving at the
program that has been filing that pattern in other people's work.
**The lesson, actionable and one sentence: an availability/entitlement
check belongs BEFORE the costing, not after the design.**

**What survives and must not be swept up as residue.** `gate ok` is
live, green on every pull request run, and has nothing to do with a
queue; requiring it as a status check is still available, because branch
protection with required status checks works fine on a public
personal-account repository — a different feature from a merge queue.
`scripts/check-run-jobs.py` and its `--selftest` (in `discipline` and in
`ci-local.sh`), and the `page_is_whole` paging guard PR 1845 also put
into `scripts/opt-level-calibrate.py`, are all unaffected. Two facts
from the dead design are durable and should be quoted forward: GitHub
builds one merge group **per queued pull request**, so merge limits and
batch size are not a CI-cost lever (the mechanism error under PR 1796's
44 job-min/h figure — do not quote that number again), and the
2026-09-04 post-un-sampling measurements (44.8 job-min and 528 s wall
per code-tier run, 5.76 merges/h, 41 % code-tier) are real and reusable.

**The dead wiring in `ci.yml` is left in place and is not this unit's
call.** `on: merge_group` and the `merge_group` exclusion on `renders`
can never fire here; they cost no run and no minute, and removing them is
a workflow edit on a file several lanes touch. The trade is written up
for the orchestrator rather than acted on.

**The fallback for the defect class the queue was chosen over is open
again** — the full push job set plus the per-SHA concurrency design pass,
48 job-min/h, named in `work/ciw/f3-recosting-on-a-public-repo`. That is
not reopened here; it is a ruling for Ev whenever CIW asks it.

## 2026-09-05 — F3's residue, measured twice, and the mechanism is scope not event

`reader_census`'s tree-wide row
(`crates/test-utils/tests/reader_census.rs:538`) reddened `main` twice on
2026-09-04 — `fde85c50` (PR 1829) held for **3 h 00 m 50 s** until
`5a1317e5` (PR 1859); `2a924eb2` (PR 1871) held for **47 m 26 s** until
`5d711eea` (PR 1884). **5 red `pull_request` runs, 35 failed jobs, 4
innocent branches**, and two CIW lanes spent repairing it. Filed as
`tree-wide-guards-outside-the-change-closure`, `needs_ev`.

**The reading this was opened on is false and the item says so.** PR
1871 did not go green *on the census*: run **33927923370** was green on
all 37 checks and **never built the census**. Its job **101201002362**
logs `Extracting 8 binaries` / `808 tests run` where the red runs log 35
binaries and 3017 tests, and the real classifier
(`scripts/ci-filter.py --files` over `2a924eb2^1...2a924eb2`) returns
`CARGO_SCOPE=-p editor-core -p pncad -p pncad-py -p viewer`.
`test-utils` is scoped out of both breaking merges.

The closure is over **dependents**, so a crate is reachable only from
itself and what it depends on. `test-utils` has zero dependencies by
design (`crates/test-utils/Cargo.toml:10`), which makes it **1 of 18**
members — the repository's most tree-wide guard housed in its least
reachable crate. Four more rows share the shape; `geom-core`'s two are
at **2 of 18**.

**What that does to the F3 question.** `ci.yml:582` bases a push run on
`HEAD^1` and diffs `HEAD^1...HEAD`, so a restored `build` + `test` on
push re-draws the merge's own closure — **it would have caught neither
break**. Measured against that: a `test-utils`-only guard row forces no
workspace build at all (cold from an empty target dir: **2.69 s** to
compile, **4.86 s** end-to-end, 3.65 s to run; 0.477 s inside CI's
archive), because the crate depends on nothing. `main`'s push run today
compiles nothing either — `cache-prime`'s only build step is gated on a
cache miss (`ci.yml:2230`) — so the row is additive at ~40 s, not a new
build.

Three options are put to Ev with their real costs and no thumb on the
scale: add a row to the push run (measured ineffective for this class
without `--force-all`), leave F3 and price the fleet as the detector
(tonight's numbers), or a narrow unscoped guard row — which on the
**pull-request** side would have red both PRs before they merged.
Nothing about F3 was touched.

## 2026-09-05 — the change closure reaches tree-wide guards

Ev, in chat: *"oh yeah the closure should reach tree wide guards"*, and
*"don't hand the unit to tcost; you can take it"*. The measurement it rules
on is `work/ciw/tree-wide-guards-outside-the-change-closure.md` (PR #1889):
`scripts/ci-filter.py`'s TIER=closure scope is the dependent closure, so
`crates/test-utils/tests/reader_census.rs` — whose subject is every `.rs` file
in the repository — is in scope for 1 of 18 members, and reddened `main` twice
on 2026-09-04 from PRs that were fully green because the guard was never built
in them. Unit: `work/ciw/closure-reaches-tree-wide-guards.md`.

**Nothing is listed.** `PKGS` is now the dependent closure plus a READ REACH
derived from what each crate's sources open: a path that lands at the
repository root or at `crates/` pins its crate into every non-docs closure, a
path that lands in another member is a read edge keyed on that member's seeds,
and a `.md` a Rust suite opens joins `_consumed_markdown` and leaves the docs
tier. A hand-maintained roster was the alternative and is the one shape the
ruling exists to remove — the sweep in #1889 found five guards, the scanner
found a sixth (`crates/bvh/tests/aggregator_headers.rs`, whose own header says
*"its subject is workspace-wide, so no crate owns it and any home is
arbitrary"*) and a seventh instance one tier over
(`crates/geom-core/tests/flagged_census.rs` reading
`docs/predicate-dimension-audit.md`, which a docs-only PR could break and
never run).

**Derived here**: `bvh, editor-core, geom-core, pncad-py, test-utils`.

**Measured cost, cold, on a 4 vCPU / 16 GB box — the hosted runner's shape.**
The pins add almost no COMPILE, because a dependent closure already drags the
whole graph: 17 of 18 single-seed closures gain zero new crate builds and only
extra test BINARIES. `cargo nextest archive` on PR 1829's real diff: 145.4 s /
22 binaries -> 164.1 s / 28 binaries (**+18.7 s, +13 %**). The worst case is a
viewer-only change, the one closure that gains a compile (`pncad-py`): 43.1 s /
2 binaries -> 73.9 s / 14 (**+30.8 s**). The classifier itself: 0.99 s ->
1.82 s on a code diff, 0.86 s -> 1.70 s on a docs one. **A docs-tier PR run
still compiles nothing** — the reach is read in the closure branch and in
`_consumed_markdown`, both of which are python over `crates/**/*.rs`, and no
cargo invocation was added to the docs path.

**F3 is untouched**, and so is what a `main` push re-gates. `JOB_ROOTS` is
keyed on the dependent closure with the reach subtracted again, so pinning
`editor-core` does not silently switch four named job rows permanently on.

**Announced cross-fence change.** `scripts/ci-filter.py` is in S-TCOST's
`paths` and in CIW's `keep_out`; S-TCOST is open, no `program.md` was edited,
and the PR names it and invites S-TCOST to own the result. Residue:
`work/ciw/reach-cannot-follow-every-ascent.md`.

## 2026-09-06 — the orchestrator changed hands, and the slate was two days stale

CIW's previous orchestrator stopped after PR 1909 (merged 2026-09-05
04:06Z) without a handoff. Nothing was wrong with the work: **all ten
units of `plan.md`'s order had landed**, and unit 11 was closed on
2026-09-04 as an avenue that does not exist. What stopped was the
bookkeeping, and from outside the program that reads as a closed
program with files left behind — which is how the gap was found.

**Thirteen rows sat at `review` with their PRs merged.** Every one was
checked against `main` rather than against its PR body before being
closed, because a merged PR is evidence the diff landed and not that the
item is discharged:

| item | PR | what carries it on `main` |
| --- | --- | --- |
| `perf-history-cannot-identify-its-host` | 1722 | `criterion-emit.py:139` `cpu_identity()`, and the two copies |
| `nightly-pin-reading-idiom-four-copies` | 1723 | `nightly.yml:659`, `:660`, `:1091` call `scripts/ci-pin.py` |
| `hosted-renderer-announces-itself-preview-only` | 1739 | `hosted-render-guard.sh:56` |
| `retire-render-automatic-matplotlib-fallback` | 1745 | `render.sh:792` refuses and exits 1 |
| `mirror-parity-never-compares-flags` | 1759 | `check-ci-mirror-parity.py:284` `FLAG_EXEMPT` |
| `geom-brep-test-unused-edgedescription-import` | 1795 | `ci.yml:2109` `clippy-all-features` |
| `reinstate-full-configuration-runs` | 1823 | twelve `test (…)` jobs; `--selftest` asserts it |
| `doc-gate-two-unread-axes` | 1847 | `doc-gate.sh:348` and its two self-test arms |
| `klint-row-still-sampled` | 1850 | `ci.yml:460` fans five legs |
| `klint-memory-false-after-unsampling` | 1855 | `memories/agent-lane-operations.md` |
| `delete-config-trailer` | 1868 | no `CI-Config` reader survives |
| `closure-reaches-tree-wide-guards` | 1909 | `ci-filter.py:988` `_read_reach()` |
| `tree-wide-guards-outside-the-change-closure` | — | closed by 1909 under Ev's 2026-09-05 ruling |

`delete-config-trailer` carried no `pr:` at all; it has 1868 now.

**One row does not close, and it is the one the board was hiding.**
`f3-recosting-on-a-public-repo` delivered its measurement and merged
(PR 1796), and the three options it recommends were put to Ev on PR
1889 — which was then **merged, unanswered**. Ev does not scan merged
PRs (`memories/orchestration-model.md`), and the item never set
`needs_ev`, so the question is absent from `STATUS.md`'s needs-Ev queue
as well: asked in a place nobody reads, and invisible in the place that
lists what is waiting. It is `open` with `needs_ev: true` as of this
entry, and the question needs re-asking somewhere it can be answered.

**A finding, in passing.** `rustdoc-d-warnings-breakages-outside-the-doc-gate`
opens by saying `SweepStrategy::Idealized` "is gone or renamed". It is
not: it is at `reduce.rs:84` behind `#[cfg(feature = "sweep-testing")]`,
which makes both of its sites instances of the in-half broken link unit
9 accepted permanently, not documentation rot. The bullet list also
missed `reduce.rs:18`. Corrected in the item; six sites survive there as
a real `--document-private-items` question.

**Review posture is unchanged** (Ev, 2026-09-06, restating 2026-09-04):
no A/B protocol, one subagent style review per unit, and a correctness
reviewer only where a unit earns one — named in its PR with the reason.

## 2026-09-06 — the second slate, re-read against the tree

The 2026-09-04 re-read moved six rows by auditing them against the tree
rather than inheriting them. The same pass over the 21 rows the first
slate's own lanes filed moves four, and the reason is the same one every
time: the units landed **after** most of these were written, and an item
does not notice when its premise is fixed.

**Two are already discharged.**

- `closure-tier-scope-hides-whole-tree-census-tests` asked for exactly
  what PR 1909 built (its option (c)). Measured, not inferred: a
  `crates/profile/src/path/arc_fillet.rs` change now scopes in
  `geom-core` and `test-utils`, so `bounds_census.rs` builds and runs;
  the smallest closure in the tree (`crates/viewer/src/tree.rs` alone)
  still pins `bvh, editor-core, geom-core, pncad-py, test-utils`.
- `probe-interval-lane-has-no-clippy-row` asked for a clippy row over
  `--features probe,interval`. `clippy-all-features` (`ci.yml:2109`) is
  a superset of that point, `geom-brep` declares both features
  (`Cargo.toml:15,22`), and PR 1795 trimmed the same four imports this
  item names. **It and `geom-brep-test-unused-edgedescription-import`
  are one finding filed twice**, by two lanes that could not see each
  other — the duplicate shape `implementer-discipline.md` §6 describes,
  landing inside a single program's own slate. Unit 6 was dispatched
  from one of them and nobody noticed the other was discharged too. It
  cost two days of a phantom row; it could as easily have cost a second
  implementation.

**Two lost half their premise to the un-sampling**, which is the
un-sampling working. `ci-draw-can-hide-a-compile-break-on-main` is named
for a draw that no longer exists — PR 1823 and PR 1850 made both lanes,
all three eps rows and all five k-lint unifications unconditional, so
the instance it was filed on cannot recur on a PR run. What survives is
that a `main` push still compiles nothing, which is F3 and is Ev's.
`detached-demo-workspaces-are-gated-only-by-a-sampled-row` keeps half 1
(a detached root is invisible to the `--workspace` check a lane runs
locally) and loses half 2 (the demos step rides `dev-default`, which now
runs on every code-tier run). Both parked or re-scoped rather than
closed: a stale title is not a discharged finding.

**The nightly reading is taken, and nothing is red.** Ev's direction on
2026-09-04 was to read the scheduled run rather than force a dispatch.
Run `33957138686` (2026-09-05 09:07Z, `success`) executed all three
demoted rows, read at the STEP level because a green job over a skipped
step is this class's whole failure mode: `corrupt input (release
profile)` 09:08:40→09:09:38, `rustdoc (gate, every root)`
09:08:42→09:12:53, `python suite (ungated re-take)` 09:09:55→09:10:21.
No repair jumps the queue. What the item is actually for — a demotion
verified AT the demotion — is untouched and is now better evidenced:
three rows ran unattended two nights late and happened to be correct,
and nothing in the tree would have said so if they had not been.

**One live defect confirmed by measurement rather than by reading.** A
`crates/geom-core/src/lib.rs` change classifies `RUN_PNCAD_PY=false`
while `pncad-py` is in `PKGS` — the crate is built as a cargo target and
the suite that exercises it is skipped. This is not something 1909
changed: `JOB_ROOTS` is keyed on the dependent closure with the reach
subtracted again, deliberately, so the reach never switches a named job
row on. `implementer-discipline.md` §2 tells implementers the python
suite runs on every code-tier run. One of the two is wrong.

The second slate and its order are in `plan.md`. Seven units, the
posture unchanged, and units 2 and 4 are the two that look likely to
earn a correctness reviewer beside the style one.

**Addendum, same day.** `main` moved 74 commits under this re-read: Ev's
tracker-wide cut of 2026-09-06 routed `work/issues/` and opened BLEND,
EVAL, GATES and METER. Two of those touch CIW.

- **GATES takes `scripts/gates/*`**, and its `keep_out` leaves
  `.github/workflows/*` and `local-scripts/*` here, with a new gate's
  wiring row as one announced line. That changes unit 3's shape:
  `opt-level-selftest-runs-nowhere`'s fix is a widening of
  `gate-roster.sh`'s scope, which is now GATES' file and an announced
  seam rather than a CIW edit.
- **METER takes `tools/*` and the two instrument documents**, leaving
  `tess_budget_cut.sh`, `tess_budget_sweep.sh` and `k_probe_sweep.sh`
  here — all three are already in CIW's `paths`.

The cut also delivered **a third instance of today's duplicate class,
and this one crossed programs**: `dirty-pr-gets-no-actions-run` (SEAT,
from PR 1910) and `no-ci-run-on-a-conflicting-pr` (PROPS, from the
riders lane) are one finding, filed on 2026-09-05 by two orchestrators
who could not see each other, and routed onto one slate a day later.
Merged: SEAT's survives because it carries the measurement, PROPS' two
instances fold into it, and the population is now three occurrences in
one day — none of them a code conflict. All three were tail-append
conflicts in a `log.md` or `DOC-LEDGER.md`, which is the shape this
repository generates by construction, which is why the class recurs.

Three duplicate pairs in one day (two within CIW's own slate, one
across three programs) is not three accidents. The board is the only
instrument that can see them and only an orchestrator reads it whole.

## 2026-09-06 — unit 1 dispatched: one answer to what `ci.yml` pins

`local-half-restates-ci-pins-as-literals` and
`ruff-pin-read-shares-the-first-match-shape`, together on
`ciw/pin-reconciler`. (`ciw/one-pin-reader` is PR 1723's branch and is not reused.) They are
the two populations
`nightly-pin-reading-idiom-four-copies` did not reach — a value retyped
where nothing reconciles it, and a second first-match-at-any-indentation
reader — and both end at `scripts/ci-pin.py`, which that unit built.

They ride one branch because they are one question asked twice and
because the ruff item's own text hands the `.claude/hooks/session-start.sh`
sites to the other. Splitting them would make each PR argue half a
population.

**Style review only** (the posture Ev restated on 2026-09-06). Neither
moves kernel logic; the risk is in what a reconciler's population
derivation misses, and that is a reviewer question rather than a
correctness-dual one.

Two things the brief carries that the items do not settle, because they
are the judgements the unit exists to make: whether the reconciler
derives its population or writes a roster (the items argue derive, and
say why a roster is the thing this repo keeps learning not to write),
and whether `ci-local.sh:588-589`'s human-facing literal stays a literal
(the item argues it should, and that the check reconciles it rather than
the text reading the pin).

## 2026-09-06 — unit 1 in review: the pin has one reader and its copies have a check

PR #2070, branch `ciw/pin-reconciler`, code-tier run green (37 jobs, 12
`test (…)`, all five `k-lint (gate, …)`; the three skips are the two cache
primers and the nightly interval oracle).

**Both judgements the brief left open went the way the items argued, and both
cost something worth writing down.**

*Derive, do not enumerate.* Claim 11 in `check-ci-mirror-parity.py` walks
`local-scripts/` for every `x.y.z` and reads `ci.yml`'s block through a new
`ci_pin.read_pins`, which shares `read_pin`'s anchoring rather than
reimplementing it. What is still written by hand is the inverse table:
`PIN_FREE` declares the three literals that are NOT pins, so a version literal
added to that tree is an error until someone says what it is. That direction is
the whole difference — a roster of pin COPIES falls behind the tree silently,
a roster of exceptions fails closed and expires like `MIRROR_EXEMPT`. The
admesh floor the item warned about is its first entry.

*One arm was not enough.* A value-only reconciler is blind to a literal that
drifts onto some OTHER pin's value, and this tree has five pins to drift
between. So there is a second, name-anchored arm: a line naming a pinned tool
and carrying a version must carry that tool's current pin, with the tool token
derived from the pin's key. It has its own hole — `ci-local.sh:620`'s "against
the pinned 0.9.140" names no tool — which is exactly what arm A covers. Neither
arm subsumes the other and both are needed; that is written at `PIN_FREE` with
the other four disclosed holes.

*The human-facing literal stayed a literal*, as the item asked, and the reason
is now written where a bumper meets it: `ci-local.sh`'s prereq note says the
versions are checked and by what.

**The ruff reader** now loads `ci-pin.py` by path and calls `read_pin`. The
fixtures did not have to move — the planted ci.yml was already a column-0
`env:` block — and two cases were added at that caller for the shapes the
shared reader exists for.

**Residues, both filed rather than mentioned.**
`session-start-hook-restates-ci-pins` (the `.claude/` copies, which no hosted
claim can reach, and whose fix has to decide what the hook does when the read
refuses) and `seal-oracle-toolchain-read-first-match` (turned up by the
third-idiom arm of the sweep: `sed … | head -1` against `Cargo.toml`'s
`rust-version`, a different source of truth with the same shape).

**What the sweep could not match** is in the PR body and, for the part the next
author needs, in the two item files: a paraphrased pin, a version written in
another form, and every source of truth that is not `ci.yml`'s `env:` block —
`rust-toolchain.toml`, `Cargo.toml`'s MSRV, the FreeCAD AppImage pin. The
provenance prose in `scripts/` and the workflows ("verified against the pinned
0.9.140") was found and deliberately left alone: those sentences record what
was measured, and making them track a bump would falsify the record. That is
why claim 11's tree is `local-scripts/` and not `scripts/`.

## 2026-09-06 — unit 1's fix pass: the reconciler's own subject was escaping it

PR #2070 head `a08b32462`, code-tier run green (37 jobs, 12 `test (…)`, five
`k-lint (gate, …)`), verified at step level: `pin reader selftest`, `CI half
parity`, and `python lint` all green in the `mirror` job.

**No MAJOR from either review, and four MINORs that were all real.** Two of
them are worth keeping.

*The most literal restatement possible was escaping the check written to catch
restatements.* `NEXTEST_VERSION=0.16.0` in a local script passed arm A (0.16.0
is sccache's pin) and passed arm B (the derived token `nextest` was matched
with `_` as a word CHARACTER, so it did not match inside the key). Both
reviewers reached it independently from different directions. The fix is small
— every underscore-separated part of the key, case-insensitive, `_` a boundary
— and the lesson is not: the shape a check is written for is the shape its
author has stopped looking at.

*A silent truncation under a loud refusal.* `read_pins` was documented as
inheriting every refusal `read_pin` carries. It inherits the per-KEY ones; the
BLOCK's own edge was a flush-left `#`, where `read_pin` refuses and `read_pins`
just returned the entries above it — and claim 11 then issues a confident,
wrong arm-A red for every restatement of the dropped pin. The correctness lane
found five mutations of that anchoring surviving both self-tests, one of them
returning a job-level pin as the workflow's. Every fixture's block happened to
end at a blank line or EOF, so the edge was never exercised. **A fixture set
that agrees with itself about where the interesting line is will agree with the
code about it too.**

**Also done:** the population is git's index rather than a directory walk
(`*.local.*` is in this repo's `.gitignore` — a walk reds a developer's gate
over a file the repo told them was theirs); `PIN_FREE` gained an inversion
guard and now excuses arm B; the loader is one documented idiom with both
callers catching every exception; `check-python-lint.py`'s pin cases run before
`resolve_ruff` (below it they never ran on any box whose ruff differs from the
pin — which is the box this row exists for); `ci.yml:313` stops restating the
pin's value in the comment above it.

**The blind-spot list lost its count.** It said FIVE; a style lane planted ten
shapes and found four it did not contain, plus one it described wrongly. A
stated blind spot is a work order; a COUNTED one reads as completeness and is
worse than silence when it is short. It is now uncounted, corrected, longer,
and says so.

**Residues filed rather than mentioned:** `session-start-hook-restates-ci-pins`,
`seal-oracle-toolchain-read-first-match`, and now
`pinned-version-named-in-present-tense-prose` — the review was right that
"measured against THE PINNED 0.9.140" asserts what is pinned now, which is a
different sentence from a record of what was measured, and only the second is
correct to leave alone.

**One fence to route:** `scripts/ci-pin.py` is in `work/meta/program.md`'s
`paths` (META opened 2026-09-04, after this unit was dispatched). This PR
changes it, because a shared enumerating reader is what the reconciler stands
on. Announced in the PR body and named to the orchestrator; the file is META's.

## 2026-09-06 — unit 2 dispatched: the python suite on a closure run

`closure-tier-skips-python-suite-on-geom-core-changes`, on
`ciw/python-suite-closure`. Measured at the re-read rather than taken
from the filing: `crates/geom-core/src/lib.rs` classifies `TIER=closure`
with `RUN_PNCAD_PY=false` **while `pncad-py` is in `PKGS`**. So the
wheel crate is built as a cargo target and the suite that exercises it
is not run.

That reading matters for the fix, because it rules out the obvious one.
PR 1909's `JOB_ROOTS` subtracts the read reach again, deliberately, so
that pinning a crate into the closure never silently switches a named
job row permanently on. Widening the reach therefore cannot fix this and
should not be attempted: the seam is between the cargo scope and the
job-row signal, and the fix belongs on the signal.

**Style review plus a correctness reviewer** — the second one earned,
and the reason named in the PR: this changes what a hosted run executes,
on the file S-TCOST owns, and the failure mode of getting it wrong is a
gate that runs less than the tree says it does. That is the same class
as the defect.

**Announced cross-fence change**: `scripts/ci-filter.py` is S-TCOST's by
this program's `keep_out`. Precedent for how to do it is PRs 1868 and
1909 — the PR names the owner, says what moved, and invites S-TCOST to
own the result.

The unit may also conclude that the *doc* is what is wrong —
`docs/prompts/implementer-discipline.md` §2 asserts the python suite
runs on every code-tier run — but only with a cost number attached, and
not on the strength of the filter being harder to change than the
sentence.

## 2026-09-06 — unit 2 delivered: the python suite on a closure run (PR 2071)

The defect re-measured at merge base `e6e27e27b`: a
`crates/geom-core/src/lib.rs` change gives `TIER=closure`,
`PKGS` holding `pncad-py`, `RUN_PNCAD_PY=false`.

**The filter was the wrong half**, and the doc was stale beside it. The
axis keeps the seed shape Ev ruled on and its seed SET is widened from
`{pncad-py, pncad, editor-core}` by every workspace member the façade
names at the top of `crates/pncad/src/lib.rs` — derived there, `pub mod`
read as well as `pub use` so the narrowed `pncad::profile` is not lost.
Fails closed: a façade naming no member Bails and the suite runs.

**What decided it was the cost number the doc answer needed.** The
`python suite` job takes 115–125 s, needs only `filter`, and finishes
692–917 s before the run ends — 35 code-tier runs, jobs API, 2026-09-06;
run wall clock 856–1302 s, set by the serial build → test chain. It is
off the critical path, so turning it on adds zero wall clock, and wall
clock is the currency on a public repository. The C3 argument that a
compile break reds the ordinary rows survives — `clippy
(--all-features)` lints `-p pncad-py` at the `python` feature every
code-tier run — but the `crates/pncad-py/tests/*.py` assertions run in
no other job, and those are the suite's subject.

The reach was not widened, as the dispatch required: `_read_reach` and
`JOB_ROOTS` are untouched and the fix sits on the signal.

Two findings outside the fence went into the PR body, not into another
program's slate: ci.yml's `python-suite` clippy step still claims that
half was "linted by NO row" (`clippy (--all-features)` lints it), and
`work.py territory` reports zero cross-fence paths for a branch editing
a file named in its own program's `keep_out`, because it reads `paths`
only and `keep_out` is prose.

### 2026-09-06 — unit 2, fix pass: the seed set comes off the graph, not the façade

Both reviews landed the same MAJOR on the first attempt, and it was
right: deriving the seed set from the members the façade NAMES at its
top level shipped the original defect one re-export deeper. `bvh` is
named nowhere in `crates/pncad/src/lib.rs` and reaches Python anyway —
`crates/editor-core/src/lib.rs`'s `pub use bvh::Ray`,
`crates/pncad/src/select.rs`'s re-export of it, a `#[pyclass] Ray` in
`crates/pncad-py/src/py/pick.rs`, and 37 uses in
`crates/pncad-py/tests/test_picking.py`. Worse, the premise selftest
REQUIRED `bvh` to stay out, so the correction had to pay for the pin.
Top-level naming is sufficient for reach and not necessary; the
docstring promised reach and the code computed spelling.

**The set is now `pncad-py`'s non-dev dependency closure**, from
`cargo metadata`. No regex, no `FACADE_LIB`, no hand-list, and the
`pub use X as Y` / `pub use X::{…}` blind spot the style review found
disappears rather than being patched. `_member_graph` now returns two
edge maps: the change closure walks upward where dev edges count
(`cargo test -p X` builds them), and this walks downward where they do
not (`maturin build` compiles no test target), which is what keeps
`test-utils` out.

**This is C3's closure condition restored**, up to dev edges — `seed
under pncad-py` and `pncad-py in dependents(seed)` are one statement
read from two ends — and every site that describes the axis now says
so. The measurement is what licenses it and it has one home,
`docs/CI-MINUTES-2026-08.md`'s entry of 2026-09-06; the other sites
cite rather than restate.

**The failure arm has a test now.** Both mutations the correctness lane
found green — the arm answering `false`, and dropping the missing-member
`Bail` — red the new case, which sits in the viewer fixture because that
workspace has no `pncad-py` in it. The old blocks asserted only the
other key there, which is why the hole existed.

Verdicts after the fix: `geom-core`, `bvh`, `verbs`, `profile`,
`quantity` true; `viewer`, `test-utils` false.

Premise sweep (the class this repo keeps re-finding): the always-run
verdict step in `ci.yml`, its `python-suite` job header and its
clippy-siting note, two sites in `nightly.yml`, `ci-local.sh`, the
discipline doc and `docs/CI-MINUTES-2026-08.md` all carried the
three-name set or the "façade keeps `bvh` interior" reading. All
rewritten; the minutes ledger gets a new dated entry rather than an
edit to the 2026-09-03 one. `ci.yml`'s "that half was linted by NO row"
was stale in the other direction — `clippy (--all-features)` reaches it
— and now says what the `python`-alone row actually adds.

**Retraction from the first pass of this unit.** The claim that
`scripts/work.py territory` is blind to a `keep_out` fence is withdrawn:
the zero it returned was taken on an UNCOMMITTED tree, and `territory`
diffs `origin/main...HEAD`. On the committed branch it names both
cross-fence paths — `scripts/ci-filter.py` (tcost) and
`docs/prompts/implementer-discipline.md` (meta), the second of which the
first pass never announced because of the same false reading. Run it
after committing.

## 2026-09-06 — unit 2 reported; two findings placed, one held for the fix pass

Unit 2's lane took the filter answer, not the doc-only one, and the cost
number is what decided it: the `python suite` job is `needs: filter`
only, runs 115–125 s, and finishes 692–917 s before a code-tier run ends
(run wall clock 856–1302 s, set by the serial build → test chain), so
the wall clock it adds is **zero** and wall clock is the currency. The
doc moved too, because §2's sentence had stopped being true on
2026-09-03 either way.

Both reviews are out: the style lane, and the correctness lane this unit
earned — named in the PR with its reason, which is that the unit changes
what a hosted run executes, on S-TCOST's file, and a wrong answer here
fails silently in exactly the way the defect did.

**Filed on META's slate**: `work/meta/territory-is-blind-to-keep-out`.
`work.py territory` reported 0 cross-fence paths for a branch whose
subject is an edit to a file CIW's own `keep_out` gives to S-TCOST,
because `territory` reads `paths` globs and `keep_out` is prose. Every
implementer brief tells a lane to run that check and address what it
names; on a declared-fence crossing it names nothing. Filed straight
onto META's slate rather than routed — the owner is unambiguous
(`scripts/work.py` is in META's `paths`) and `work/README.md`'s
2026-09-04 rule says a finding goes where it belongs without the owner's
permission.

**Held for unit 2's own fix pass, not filed**: `ci.yml`'s comment block
above `clippy (pncad-py, python feature)` (`:3749-3757`) says *"on a PR
whose seeds miss {pncad-py, pncad, editor-core} the row does not run"*
and that a change reaching `src/py/` through `quantity` or the
re-exported kernel *"seeds nothing"*. **The lane's own diff falsifies
both sentences** — those crates seed the axis now. That is not a
follow-up item; it is reviewer question Q4 (did this change invalidate a
premise something else cites) landing on the diff that caused it, and it
goes back to the lane with the reviews.

## 2026-09-06 — unit 1 reported; both units under review, and one routing debt recorded

Unit 1 (PR 2070) took the reconciler shape the item argued for rather
than a substitution, and inverted the hand-written part: `PIN_FREE`
declares the three literals that are NOT pins (the admesh floor, a
rustfmt version in a hook, `0.0.0` crate versions), so an undeclared
literal is an error and a declaration whose literal disappears is also
an error. The population is an `os.walk`, not a roster. `ci-local.sh`'s
human-facing install line stays a literal, checked rather than
rewritten, which is what the item asked for.

**Both units are getting a correctness reviewer beside the style one,
and unit 1's is a departure from this program's default worth stating.**
The posture is style-only unless a unit earns more (Ev, 2026-09-06).
Unit 1 earns it not for complexity of logic but for **blast radius**: it
adds a gating claim whose population is derived by walking a directory,
so a false positive reds every PR in the repository and the surface is
every file anyone adds under `local-scripts/` in future. A checker that
is wrong in that direction is worse than the drift it detects.

**A routing debt, recorded now and payable at unit 2's merge.** Two OPEN
items on LIB's slate cite the python suite's three-name seed set as
current:

- `work/lib/pncad-py-python-feature-clippy-lane-is-red.md:104`
- `work/lib/the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row.md:109`

Unit 2 moves that premise under them. One-file-one-item means CIW does
not edit them (META's `keep_out` states the rule: a stale citation in
another program's slate is routed to its owner, never fixed across the
fence). **This orchestrator runs on a remote box with no away-channel
monitor and no `gh`**, as PROPS, SHELL and TOPO record for themselves, so
the route available is the PR body plus this line — and neither is a
slate. Stated as a debt rather than as a discharge: if unit 2 merges and
nobody has told LIB, the citations are stale and the only record is
here.

## 2026-09-06 — two retractions, both this orchestrator's

**`work/meta/territory-is-blind-to-keep-out` is withdrawn and its file is
deleted before it ever reached `main`.** The premise was false.

The finding was that `work.py territory` reported 0 cross-fence paths for
a branch editing `scripts/ci-filter.py`, and my explanation was that
`territory` reads `paths` globs while `keep_out` is prose. The
explanation was plausible and the observation was an artefact: unit 2's
lane had run `territory` on an **uncommitted** tree, and `territory`
diffs `origin/main...HEAD`. The lane found this itself on the fix pass
and retracted it.

Re-taken here on the committed branch, and it is right about everything:

    scripts/ci-filter.py: owned by tcost
    docs/prompts/implementer-discipline.md: owned by meta
    .github/workflows/{ci,nightly}.yml, local-scripts/ci-local.sh: owned by ciw

`scripts/ci-filter.py` **is** in S-TCOST's `paths` — I asserted it was in
no program's `paths` and did not check. The check works; it also caught a
**second** cross-fence edge (`docs/prompts/*` is META's) that the unit's
first pass had missed entirely, which is the opposite of the failure I
filed. META already holds a real and different blindness in
`territory-cannot-see-a-path-two-programs-both-claim`; nothing here adds
to it.

**The mistake worth naming is mine, not the lane's.** I verified the
`bvh` finding hop by hop before acting on it and took the `territory`
finding on trust in the same adjudication, because one looked like a
claim about the kernel and the other looked like a claim about a tool. A
finding filed onto another program's slate is the one that most needs the
check: they cannot see the branch it came from.

**The LIB routing debt recorded above is also withdrawn.** Both items —
`work/lib/pncad-py-python-feature-clippy-lane-is-red` and
`work/lib/the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row` —
are `status: closed`. A dated citation inside a closed item is a record
of what was true when the work was done, not a live thread, so there is
nothing to route and no debt to pay at unit 2's merge.

## 2026-09-07 — units 1 and 2 merged, and closed the same day

PR 2070 (`cd7f03a4`) and PR 2071 (`1cc774c9`). Both verified on `main`
rather than on their branches: `crates/bvh/src/lib.rs` classifies
`RUN_PNCAD_PY=true`, and `check-ci-mirror-parity.py` passes with claim 11
in its summary.

**Unit 2 needed `main` merged in first**, and the conflict was
`work/ciw/log.md` — a tail-append collision, both sides appending
same-day entries, no code involved. That is the third instance today of
the shape `dirty-pr-gets-no-actions-run` is about, and this one is on
this program's own log. Resolved by keeping both threads; the whole tree
was swept for markers and the battery re-run on the MERGED tree rather
than on either diff, which is where `check-ci-mirror-parity`'s claim 11
was seen passing over unit 2's edits to the file it now polices. A run
was confirmed to have STARTED on the resolved head before the merge —
the resolution came out of a conflict, and a conflicting PR gets no
retroactive run, so "no run" and "queued" look identical.

**Closed the same day, deliberately.** This program's slate was found
two days stale on 2026-09-06 because thirteen rows sat at `review` with
their PRs merged; leaving three more there would have been the same
failure by the same orchestrator inside one week. The disposition of
each is in its own file.

Residues the two units filed, all open on this slate and none of them
disclosed-only: `session-start-hook-restates-ci-pins`,
`seal-oracle-toolchain-read-first-match`,
`pinned-version-named-in-present-tense-prose`,
`python-suite-axis-skips-only-two-members`.

## 2026-09-07 — F3 ruled: no post-merge run, and three rows close on it

Ev, in chat: *"i'd be somewhat inclined to skip the post-merge run, just
because i don't think anyone would actually check it"*, confirmed as the
ruling. **F3 stands and the push job set is not restored.**

This closes what the 2026-09-04 ruling left dangling. That one declined
the push gate *in favour of* the merge queue; the queue turned out to be
unavailable to a user-owned repository, so the decline had to be re-taken
on its own terms, and it has been. The reason is a reader rather than a
cost — minutes are free, and the objection is that a detector nobody
reads is not a control.

**The program had already produced the evidence for Ev's argument while
the question was open**, which is worth recording because it is the
strongest thing said on either side: `nightly-demotions-have-never-run`
exists because three rows demoted to the nightly ran unattended for two
nights and nobody looked. Their first reading was taken on 2026-09-06 by
this orchestrator, going deliberately to the jobs API because an item
told it to. A red push run would land in exactly that place.

**Where the options table was arguing the wrong benefit, and this is the
part that survives the ruling.** It prices *detection*. Both 2026-09-04
instances were detected, quickly, by lanes tripping over a red `main` —
F3's stated compensating control working as written. What they cost was
**attribution**: 42 red runs on 20 branches, four innocent branches, and
two agents diagnosing the same one-line break in the same hour, each
paying the diagnosis independently because it is invisible from inside a
single PR. A push run's value was never that someone would watch it; it
was that the red would be attached to the merge that caused it.

Opened as `inherited-red-is-not-attributed-to-its-merge`: not a gate,
nothing on a green run, no watcher — it lands where a person is already
looking, which is why it survives the ruling rather than being closed by
it. It automates a convention that already exists by hand (Ev,
2026-08-31: an inherited red does not block the merge but must be
annotated with its issue, and the causing lane owes the fix), and it is
opened with three unmeasured numbers named — the cost of reproducing on
`main`'s tree, how much of the value the cheap "it is on the tip too"
version buys, and whether the class fires often enough to be worth
having. **Not dispatched until those are taken**; two samples in one day
is not a rate.

Closing three rows on one ruling tripped `work.py lint`'s fired-trigger
error, exactly as designed — two rows parked on a trigger that had just
closed. All three are this program's, so they close in the same PR,
which is the case the rule's accepted cost (Ev, 2026-09-04) is about.

## 2026-09-07 — unit 3 dispatched: a demotion verified at the demotion

`nightly-demotions-have-never-run` and `opt-level-selftest-runs-nowhere`
on `ciw/demotion-verified`. One class from two directions: a row that has
never been shown to execute, and a guard that has never been shown to
fire.

**Pre-dispatch checks, because the slate was written a day ago.**

- Both rows still open, and `scripts/gates/gate-roster.sh` still does not
  reach `scripts/opt-level-calibrate.py`.
- **PR 2077 (`gates/whole-file-skips`) is live on `gate-roster.sh`.**
  That changes the unit's shape: the systemic fix — widening the roster's
  scope so the "a guard that has never been shown to fire is not a
  guard" rule reaches the file it is about — is GATES' file AND is under
  an open PR, so it is not this unit's to take. The instance fix is
  entirely CIW's ground: nothing anywhere invokes `--selftest`, and the
  workflows are ours. The brief says take the instance, announce the
  systemic half to GATES, and do not touch their file.
- **Two more nightlies since the 2026-09-06 reading**, both `success`:
  run 16 (2026-09-06) and run 17 (2026-09-07). Run 16 concluded in
  **10 minutes** against run 17's 27, which is the shape of the `has main
  moved` gate skipping the body — on a day `main` certainly moved. That
  is not what this item was filed about and it may be nothing, but a
  scheduled workflow that skips a night is the same family as a demoted
  row nobody reads, so the brief asks for it to be read rather than
  assumed.

**Style review only.** Neither half moves logic; the deliverable is a
convention plus one invocation, and the risk is in what a parity claim
would over-reach into rather than in a subtle bug.
