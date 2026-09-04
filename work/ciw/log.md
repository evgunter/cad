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
`ciw/one-pin-reader`, `ciw/perf-host-identity`. One style review each,
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
