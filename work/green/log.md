# GREEN — the log

## 2026-10-28 — opened

Cut across the infra group on the user's request: most infra tracks
carried one or two rows that matter beside a long P3/P4 tail, so the
cut follows the priority seam rather than a semantic one (the user, in
chat: *"i know we don't normally split on a priority seam but there
are a bunch of infra programs that have like 1 actually important
thing"*). Tracks with a reasonable P0/P1 count of their own could stay;
none had more than two.

Nightly 74 (run 40637814390, main `a1cfd1cfdb`) set the slate: red on
`full suite`, `rustdoc (gate, every root)`, `k-lint (dev-probe)` and
`PR CI latency (alarm)` (median 23.1 min, n=183).

**Moved here by `git mv`, ids and bodies unchanged:** from CIW
`pr-ci-wall-clock-root-cause-measured`,
`the-demos-job-is-pr-cis-long-pole-on-its-release-build`,
`a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar`,
`tests-red-under-all-features-never-run-by-ci`; from S-TCOST
`main-reds-editor-core-slow-set-and-the-k-probe-sweep`,
`main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps`;
from BUDGET `tess-lint-gate-red-on-main-23-findings`; from S-TINT
`pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300`; from VACUITY
`pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1`,
`random-integer-rays-search-trips-at-eps-1e-6-on-one-run`,
`cert10-strict-gap-floor-gates-on-a-varying-seed`; from GUARD
`gate-selftests-run-in-no-ci-job`. Filed new:
`nightly-rustdoc-gate-red-on-main`.

**Re-banded.** The latency rows to P0 (the user, in chat: *"the
latency alarm is definitely P0"*): every track's work passes through
the gate. The main reds to P1: a red nightly is a red main, and a new
regression lands unseen among old ones. The two pick-guard rows fold
into one fix (two become passengers, `cost: C`); the eps-1e-6 one
drops P1 → P3, since every recorded red names the guard and none the
oracle. Off this slate: `test-utils-is-production-source-to-every-narrowing-gate`
(TINT) and `mass-properties-bit-comparison-has-thirty-spellings` (HELPER)
P1 → P3, as test-side duplication rather than production architecture.
Closed `two-bisector-node-rows-break-the-fast-set-one-second-bar` (CIW),
already fixed on main.

**Left where they are, on purpose:** COMB's `S19` and `S43` (kernel
refusal and "can only be a bug" architecture, P1, design-heavy roll-ups)
and PERF's `fnv-digest-and-memo-machinery-copies` (production
duplication of the memo machinery, P1). Each is correctly P1, and none
is a green-main or gate-latency row.

## 2026-10-28 — five lanes dispatched

All five at once (`plan.md`, Lanes). The three that build heavily run
as their own cloud sessions, since this box has 4 cores and one build
slot: **slow-set reds** (`green/slow-set-reds*`), **tess-lint**
(`green/tess-lint-red`), **small reds** (`green/tilted-cut-upper`,
`green/rustdoc-red`). The two that are mostly workflow and data run as
subagents here: **latency** (`green/latency-*`) and **gate trust**
(`green/<row>`).

Review tiers, chosen at dispatch:
- latency: single FULL review per lever PR — a workflow change that
  takes a row off the gate can hide a red, which reading alone does not
  rule out;
- slow-set reds and tess-lint: single FULL review — every re-bless has
  to be explained, and the review falsifies the explanation;
- small reds: single STYLE review (an explained re-bless and a doc fix);
- gate trust: orchestrator's read for the seed and pick-guard fixes
  (mechanical, with witness seeds), STYLE review for the two new
  nightly rows.

## 2026-10-28 — two lanes landed; a coupling with OFFSET

- **tess-lint** (#5942, orchestrator's read: the diff was tracker-only).
  The gate was already green: #5717 re-cut the baseline on 10-26 and
  folded the 31 findings unread. The lane read all 31 after the fact
  and attributed each to a deliberate model change (#3657, #4193,
  #4209/#4121/#4926, #5568, #5722), the bound change #4984 or #5531's
  documented sizing trade; no kernel finding. Two moves stay
  unattributed (`teapotspout`'s total fall, `tiltedcut_above` face 3);
  neither is a finding. Also closed TESSSIZE's twopeg row.
- **small reds** (#5943, orchestrator's read: one doc-comment fix).
  Nightly 74's rustdoc red was already fixed by #5917; the lane found a
  fourth link that #5894 added after the nightly and fixed it.
  `tilted_cut_upper` was already re-blessed by #5818 (cause #5734).
  Class worth keeping in mind: with no per-PR rustdoc pass (the latency
  cut's choice), every broken doc link reaches the nightly first. That
  is a deliberate trade, and the latency lane must not undo it.
- **Coupling with OFFSET's #5935** (raised by the user): it bisects some
  editor-core M10 and 1e-12 KernelLimit reds to #5728 and re-pins them.
  `main-reds-editor-core-slow-set-and-the-k-probe-sweep` names several of
  the same tests (`m10_3_driver_interval`, `m10_9_pins_interval`,
  `sym11`'s `two_hole_plate`). The slow-set lane was told to leave
  #5728's reds to #5935 and to say which tests both PRs touch. A note
  is on OFFSET's log. Band difference noted, not resolved here: #5935
  bands its main reds P2 (interval ground), GREEN's slow-set row is P1
  (a week-long red nightly hides new regressions).

## 2026-10-28 — a red is greened, not fixed, here

The user set GREEN's posture on main reds (`program.md`): get them
green without going into the weeds on other programs, by re-blessing an explained move, or by pinning the failure
(or `#[ignore]`ing it with the row it waits on) and filing the fix on
the owning program. No kernel work in a GREEN lane. The slow-set lane
and the gate-trust lane (for the `--all-features` reds) were told the
same day. This also settles the band question above: the slow-set row
stays P1, since greening main is this track's central job.

## 2026-10-28/29 — the lanes land

- **latency** (P0): #5941 split the per-PR `test` job into `app` and
  `kernel` shards (full review; fixes before merge). At TIER=all it
  takes `test` from 27.6 to about 19 min. Sharing the kernel libraries
  between shards was measured and declined (#5957): the shards compile
  them under different features, and the duplicate costs about 200
  unit-seconds. Next: rebalance the shards and move the roster into
  `scripts/ci-filter.py`, after a day of closure-tier data.
- **gate trust**: #5939 (the pick guard is planted, and the eps-1e-6
  red was the guard), #5944 (cert10 split onto a pinned draw), and
  #5962 (nightly gate self-tests through a shared `scripts/run-gates.sh`,
  plus a nightly `--all-features` suite, cacheless so it cannot evict the
  PR gate's caches). All four rows closed.
- **full-suite pins**: #5958 greened nightly 74's reds outside the
  slow-set row: 20 pinned through the new `test_utils::pinned_red`, 2
  disabled, 5 re-blessed, 2 rows filed (one a P1 wrong build on
  SPLICESEAM).
- **slow-set reds**: #5952 fixed #5542's two `segment_gap` defects
  (full review), before the pin-not-fix rule reached the lane. The user
  agreed to merging it rather than handing it off: the effort was
  spent, and review covers the semantics. Its remaining reds are pinned
  in #5966.

## 2026-10-29 — nightly 76, the rebalance, and the triage overlap

- Nightly 76: the latency alarm read a median of 17.6 min (n=229), so
  `pr-ci-wall-clock-root-cause-measured` closed. The gate self-tests and
  the `--all-features` suite were green on their first night.
- #5984 and #5989 merged after a full review and a fix pass. #6019
  (mesh as app-shard ballast) merged on the orchestrator's read: a
  one-tuple change with a selftest that reds a stale ballast entry. The
  closure-tier balance waits on a day of data.
- The nightly-76 lane duplicated the daily nightly triage's rustdoc
  fixes (#6022 closed, superseded by #6018). `program.md` now says GREEN
  acts on the triage's table.
- The r2_turn_probes fuzz red unrelated PRs twice. The user agreed to
  leave it running. The row moved to GERMCERT at P2, because the bound
  is the defect.

## 2026-10-29 (evening) — nightly 76 greened; the tour compiles once

- Nightly-76 lane (cloud). It merged on top of the triage's #6018, citing
  its rows:
  - #6031 (gui cells re-rendered after G's rename) and #6033 (two more
    doc links), both on the orchestrator's read.
  - #6025: the slow seam-region row is left out of the nightly's 1e-12
    command only, citing SECT's row.
  - #6027: the dev-probe pins (key splits at G's ordinals, the lily's
    1e-12 stem stop, a census floor of 19). The 24,045 in-band
    curve-walk margins are filed on CPAIR. One STYLE review covered
    these three; it caught a citation to a closed CLEAVE row.
  - #6030: tracker only; #6023 had already re-pinned `review_pick_r2`.
  - #6044: the large-K lint is a pinned red on its verdict, not on its
    counts. An exact pin on about 52k margin counts would have made
    every kernel merge re-pin it.
- Class filed: `a-pin-or-disabled-red-can-cite-a-closed-or-missing-row`.
- Latency lane: #6039 splits `demo-tour` into a lib and a thin bin, and
  moves the 47 unit tests into `tests/` (FULL review, then a fix pass
  that retired seven `at_rest` copies and the copied bud and teapot
  constants). Same-runner A/B: the tour build phase went from 648 s to
  453 s. `demo-tour-unit-tests-compile-the-bin-twice` closed.
- Open for nightly 77 (09:00 UTC read): close the main-red rows the
  nightly confirms; check the closure-tier shard balance after #6019;
  check whether `demos` is still the p90 tail. The remaining P0 lever is
  sweep's test runtime in the kernel shard.
- 2026-10-30 (INTENT tail retriage) Received from INTENT by `git mv`, ids and bodies unchanged, priority and cost set (PR body has each row's reason): `g-moves-four-main-reds-messages` (P1). INTENT's charter is D10 and these are follow-ups found while building it.

## 2026-10-30 — latency closed; nightly 77

- The latency alarm read a median of 14.7 min on nightly 77. Over the
  116 PR runs after #6019 and #6039 the run median is 14.3 min. The
  closure tier is at median 14.6 / p90 19.4 and the all tier at 24.1,
  inside its 25-min deep-kernel bar. The three open latency rows close
  with those numbers. The shard row misses its own 1-min balance
  criterion (1.2 min paired) and closes on the orchestrator's call; its
  closing section says why.
- Nightly 77: rustdoc and the gui montage are green. The triage (#6082)
  fixed the census re-syncs and the large-K step's `bash -e` bug, and
  filed or annotated every other red. A GREEN lane
  (`green/nightly-77-*`, cloud) pins what it left.
- Duplicate effort: GREEN's #6084 (the reader_census line) re-did the
  triage's same fix half an hour before the triage landed, against this
  program's own rule. GREEN now waits for the triage table before
  touching a nightly red.

## 2026-10-30 — handed back

The P0 and P1 rows are closed, so the sitting ends (the sitting rule in
`memories/orchestration-model.md`) and the track goes back `ready` with
one P3 row.
- #6093 pinned what nightly 77's triage left red. Its review caught a
  re-bless with no basis: R2's rounded pad loses two `symbolic`
  decisions outside G's bracket-only DECIDE-3 trade. It is pinned as
  unexplained, and the row moved to INTENT at P1 (#6094).
- #6094 closed the four main-red rows.
- #6095 added `scripts/gates/pin-cites-an-open-row.sh`. Every
  pinned or disabled red must cite an existing, open `work/` row, so a
  row's close takes its pins with it. It fixed two stale citations, and
  filed the wider class (1,114 code citations of `work/` rows, 432 of
  them stale) on META.

What nobody does now. The daily triage files and fixes trivial reds,
but it pins nothing. While GREEN has no orchestrator, a red that needs a
pin stays red until someone picks the track up. `plan.md` says how it
runs.

Review tiers over the sitting: every latency lever got a FULL review,
and every pin PR got a STYLE review. Docs-only and one-line changes
went on the orchestrator's read. Every review that ran found something
worth a fix pass before merge; two of them were blocking (#6027's
closed-row routing and #6093's re-bless).
- 2026-10-31 — (SECTARMS orchestrator) **On `main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps`:** SECTARMS is bisecting its own suspect, PR 6139 (cylinder × cylinder stage 1), against the 18 rows. Each row is run at 6139's first parent and at its merge `f2f34e16`. Any row 6139 caused is SECTARMS's to fix, and is reported here and on that row. Rows red at both commits are left to GREEN, with whatever evidence the run gives.

## 2026-11-01 — the per-PR tour builds at opt-level 1

- Ev asked why PR 6179's `demos` job took 23.9 min. It is GREEN's closing
  number, not a regression (eight jobs 10-30 to 11-01: 23.3–24.3 min). The
  viewer PR ran demos because its `Cargo.toml` edit is TIER=all.
- `the-pr-tour-suite-builds-at-opt-level-1`: the PR row builds the tour in
  a `ci-tour` profile, release at opt 1 with assertions armed. One 4-core
  box: 17.5 → 10.0 min, build plus tests. The build's critical path was
  editor-core, then the `all` binary's opt-3 re-instantiation of generics.
  `docs/PERF-SCAN-2026-08.md`'s claim that rustc shares generics only at
  opt 0 is corrected: it does at 0, 1, `s` and `z`.
- Not done, for a later row: a member-manifest edit that only adds a
  target (`[[example]]`, `[[bin]]`, `[[test]]`) still bails to TIER=all.

## 2026-11-02 — picked up; four lanes

- No `triage/nightly-*` PR exists for 10-31 or 11-01, so the charter's
  wait-for-the-triage rule has nothing to wait on. The slate's own rows
  were filed from lane evidence and are worked from it. Nightly 11-01's
  full suite (run 41276693129) fit in 130 min and was red at 1e-12 on
  rows beyond the slate (`refusal_concision_chains` ×3,
  `seat7_sweep_lowering` digest, `certified_endpoint_census`,
  `sym_rule_f_rows` copysign census, `m5_pr12_die_body`, `m6_surgery`,
  `spline_spiric_operands` pin passing). Those wait for a triage table.
- Lanes, each on its own `green/` branch:
  - `green/full-suite-split` (P1 E): the nightly's full suite as one
    archive build and one job per ε row. FULL review: a workflow change
    whose failure mode is a row that silently stops running.
  - `green/main-red-sweep` (P0) and `green/main-red-editor-core` (P1 and
    the unbanded coincidence-door row), cloud sessions: reproduce on main,
    attribute, then re-bless, pin or `#[ignore]` with the owner's row. STYLE
    review per PR, as in nightly 77's pins.
  - `green/pr-latency` (the user, in chat, 2026-11-02: improve test-suite
    latency without increasing queueing). The alarm reads median 14.3 /
    p90 24.6 min, n=193 (nightly 11-01). The lane measures the p90 tail
    and proposes levers that cut wall clock without adding runner-minutes
    or jobs. FULL review per lever.
- A fifth lane, `green/slow-test-census` (the user, in chat, 2026-11-02:
  the full suite's runtime suggests the kernel code is "pretty unusable
  unless there's an insane amount of debug_assert going on"): a
  per-test, per-ε time census from nightly 11-01's log, debug assertions
  on vs off for the costliest tests, and speed rows filed on the programs
  that own the slow code. Tracker-only PR; orchestrator's read.
- #6323 merged (full-suite split): FULL review, no blocker, every claim
  held. Not dispatched on its branch: a dispatch runs all ~27 nightly jobs,
  which is the queueing the user asked not to add. Nightly 11-03 is its
  first run; check there that each leg's `full-suite-junit-<eps>` artifact
  is populated (a wrong junit path is hidden by the copy's `|| true` and
  `if-no-files-found: warn`) and the build runner's disk while archiving.
  Class from the review, logged here: `render.yml` keeps a hand-written
  census of `nightly.yml`'s checkout count in a comment, already one
  behind before #6323, and nothing guards it.
## 2026-11-02 — editor-core interval and coincidence-door reds (lane `green/main-red-editor-core`)

- Six interval rows bisect to PR 6105, CLEAR's `cylinder_rim_axial` decision. The four that are that decision's own symbolic count are re-blessed. m10_3's two drives refuse every leaf; they are pinned and filed on PCERT.
- The coincidence door bisects to PR 6047 (SECT's rehome by path). It is pinned and filed on SECT at P0.
## 2026-11-02 — latency lane: the gates run one per CPU, and a script read by pncad-py stops forcing TIER=all

The user asked for lower test-suite latency without more queueing.
Window: the 170 completed `ci.yml` PR runs from 11-01 18:33Z to 11-02
06:25Z, after the opt-1 tour. Median 12.8 min, p90 19.7.

- **Queue time is not the tail.** Run created → first job: median 0.05
  min, p90 1.05. Per job: p90 0.9, p99 5.2. The p90 is execution:
  `test (kernel)` is the last job in 28 of the top-20% runs, and its
  build step is most of that job.
- **Two levers taken** (PR 6324):
  - `scripts/run-gates.sh` runs the gates in parallel. Gates step
    164 → 85 s on one runner, about 330 s → about 150 s on the
    window's runners. Every run gets it, and docs-tier runs end with it.
  - `scripts/ci-filter.py`'s `SCRIPT_READERS`: an edit to
    `scripts/payload-rung-sweep.py` seeds pncad-py instead of forcing
    TIER=all. That edit drove 16 of the window's 23 TIER=all runs.
- **Filed, not taken:**
  - `kernel-shard-long-pole-is-one-test-binary` (P2): the tail lever,
    and a change to `test-aggregation`'s invariant.
  - `test-shards-link-examples-nextest-never-runs` (P3): measured at
    43 s on one runner.
  - `test-shards-rebuild-dependencies-their-cache-holds` (P3).
- **Method.** Hosted build times are bimodal by runner: the TIER=all
  kernel build is 540–663 s on some runners and 821–868 s on others.
  An A/B across runs measures the runner, so the lane's numbers come
  from one runner running both sides.
- Correction to the entry above: "about 330 s → about 150 s on the
  window's runners" was an extrapolation, not a measurement. The one
  paired hosted measurement is 164 s → 85 s (one runner), and the one
  hosted run of PR 6324's head took 189–192 s for the parallel gates step
  on a slower runner. And `SCRIPT_READERS` did not land as a table: the
  filter derives a script's readers on every run (`_script_reader_members`).
- 2026-11-02 — Slow-test census of the `full-suite` job (runs 41276693129 and 41172334943) appended to `the-full-suite-jobs-three-eps-rows-outgrow-its-180-minutes`: rows of 17.2, 13.4 and 40.3 minutes on 11-01, each saturating four threads; 1e-12 is 57 % of row time and sweep is 64 % of that; the 10-31 runner was 1.75× slower per test. Debug assertions cost under 2 % on seven runs of six of the slowest tests; the cost is the curved boolean pipeline, filed on SECTRUNG (P1). (GREEN lane)
- #6343 (sweep main reds) STYLE review: merge, with a fix pass (stale
  docs and rows, test names that now contradict their pins, per-pin
  "what lands next" clauses, owner split of the conic-trim refusal).
  Class, logged here: the sweep slow tests spell one door vocabulary
  three ways (`Door` in `cylinder_sphere_frame`, `Stop` in
  `torus_doors_join`, `stop_word` in `germ_tilted_rod`, plus
  `germ_plane_torus_frame::TorusDoor`), each matching the same refusal
  prose by `starts_with`; a refusal re-worded upstream moves them apart.
- #6324 merged (parallel gates; script readers derived per PR so a
  `payload-rung-sweep.py` edit seeds pncad-py). FULL review, then three
  fix passes. Each pass's re-review found compiled code the filter read
  as prose (a Rust `#[doc]` attribute, an indented `///` doctest, a
  hidden `# ` rustdoc line in a consumed `.md`). The class: deciding
  "comment or code" by a line's leading characters fails open wherever
  rustdoc compiles text. The final rule bails on any mention inside a
  fence or an indented block, reads `#` as a comment only in yml, toml
  and sh, and scans `git ls-files`. Sibling gap filed:
  `read-reach-blanks-doctests`.
- #6343 merged (sweep main reds, P0): 7 of 18 rows already green; 11
  moved at PR 6013 (FOREIGN's crease rule, Ev's PR 5913 ruling) past the
  spline join to SPIRICARM's volume gaps, pinned per owner. STYLE review,
  one fix pass (test names now say where they stop).
- #6355 merged (editor-core main reds, P1 and the coincidence-door row):
  four interval counts re-blessed as PR 6105's new
  `pcurve_cylinder_chart_axial` decision, proved by dropping only that
  `decide` call; the m10_3 drive pinned on a new PCERT row (P2); the
  coincidence door pinned on a new SECT row (P0, bisected to PR 6047).
  STYLE review, one fix pass.
- #6361 (integration-test binaries at opt 0 in the PR test job, kernel
  build 882 → 539 s paired): FULL review, fix pass in flight.
- `read-reach-blanks-doctests` banded P3 E (a guard that cannot see a
  read edge: tooling that prevents a silent bug).

## 2026-11-02 — handed back

- #6361 merged after its fix pass: the shipped wrapper re-measured
  paired (kernel 814 → 537 s on the slow runner, app 502 → 420 s on a
  fast one), 14 tests moved to the slow set, a step reds the build if
  the wrapper stops applying, `test-fast.sh` uses the same wrapper. The
  0.5–0.8 s cohort is a P4 row.
- Not taken: `test-shards-rebuild-dependencies-their-cache-holds`. Its
  wall payoff is small and per-shard priming adds runner-minutes on
  every main push, against the user's no-more-queueing condition.
- P0 and P1 are closed, so the sitting ends and the track goes back
  `ready`. Review tiers: FULL on every workflow/filter lever (#6323,
  #6324, #6361), STYLE on each pin PR (#6343, #6355), the census
  tracker-only on the orchestrator's read. Every review that ran found
  something worth a fix pass; #6324 took three.

## 2026-11-14 — picked up (Ev, in chat: orchestrate GREEN)

- Track set `active`. Four main-red rows arrive from tcost by `git mv`, ids unchanged, at P1: the far-off-origin union (rewritten: it now fails at 1e-12 only, with a different refusal), the round keel at 1e-6, the filleted die, `revalue_boss_union`.
- Nightly 43371638371 (main `e36675c1`, 2026-11-13) read whole from its junit artifacts: 22 failures at the default ε, 35 at 1e-6, 38 at 1e-12; `suite under --all-features` did not finish its build in 60 minutes; `k-lint (dev-probe)`, `viewer app feature`, the first-refusal mutant job and `gate self-tests` are red. The 1e-12 leg ran 92 minutes, and 118 the night before, against a 120-minute cap.
- No `triage/nightly-*` PR is open, so the wait-for-the-triage rule has nothing to wait on, as on 10-31; this sitting reads the nightly itself.
- Review tier for this PR: orchestrator's read (tracker headers and moves only).
- Slate reconciled against that nightly. Closed, every named test green at all three ε: the partners-along-the-face row (the slate's one P0), the near-node cap, m10_p_fence's digest, and the PCERT E1b list. The 53 failing tests and five failing jobs are cut into six units, `nightly-reds-*` (five, by kind and ε row) and `nightly-jobs-that-do-not-finish-or-read-nothing`; the open main-red rows are parented to the unit that greens them. Letters and tiers are in each unit's file: all M; STYLE single review on the five pin units, FULL on the jobs unit, as on the 11-02 sitting.
- Six implementer lanes dispatched 2026-11-14 02:32Z as cloud sessions, one per unit, each on `green/<slug>`; briefs on `green/briefs-2026-11-14` (`briefs/*.txt`, one shared text for the five pin units). Draws are on `analysis/model-compare/green-2026-11-14`; this log stays arm-free until the rows land. Sessions: censuses `session_018u3gVJ9E1ukgusAjY7sDER`, goldens `session_018u3gVKd7LYdue6rCC2nj1p`, default-eps `session_018u3gVGaK5achAnJYngUfrT`, eps-1e-6 `session_018u3gVKVCb1kFJDiBuP9AEn`, eps-1e-12 `session_018u3gVFCsDPPq9zvP8AhiZk`, nightly-jobs `session_018u3gVJix3wBi11szBtVrjt`.
- GREEN's load reads 42/30 while the old main-red rows stand beside the units that green them; they close with their units.
- 2026-11-14 03:15Z — censuses lane reported: PR 6921, head `af65121e97`, `gate ok` green; no live wrong answer. Review (STYLE, single) dispatched 03:16Z on that frozen head, `session_018u3gVHk4V7kPMYDaZ62W3z`; reviewer briefs are `briefs/review-<slug>.txt` on the briefs branch. PR 6918 (the tracker cut) merged.
- 2026-11-14 03:38Z — censuses review in: APPROVE-WITH-FIXES, two MAJORs, both demonstrated (the branch was already stale against main: PR 6907 reds two of its six censuses on the merged tree; a second gate, `recourse-ending-by-hand.sh`, has the SIGPIPE shape the lane fixed and fails open). Fix pass sent to the implementer's session. Class, logged here: a census re-bless is true only of the main it was run on, and main moved under it within the hour; the lane re-merges main and re-runs immediately before reporting.
- 2026-11-14 04:24Z — nightly-jobs lane reported: PR 6932, head `8434fac66f`. Its `gate ok` is red through `test (kernel)` on three tests the lane reads as main's own reds (the reviewer has that as a claim to falsify). FULL review dispatched on that frozen head, `session_018u3gVHuu…` successor id in the session list under tag `green-lane-2026-11-14`. The goldens lane reported mid-run that PR 6856 (after the nightly's sha) moved every id-keyed digest again and reds two `m10_p_fence` rows on no list; it takes them.
- Correction to the entry above: the nightly-jobs reviewer is `session_018u3gVFzqSWMNAzmp4warST`.
- 2026-11-14 04:35Z — nightly-jobs review in (10 minutes): APPROVE-WITH-FIXES, no MAJOR; the inherited red gate, the budget cut-off and the stated causes hold on the reviewer's probes. Taken: the coverage job compares legs with each other and not with an independent listing (a test no leg runs passes); a lint step reads a partial CSV as a pass; the all-features row was closed on where the hang was, not why. Fix pass sent. Nothing in the PR has run on a hosted runner: the first nightly after merge is the check.
- 2026-11-14 04:40Z — default-eps lane reported: PR 6931, head `6ae8d4c99`, gate green; review (STYLE) dispatched, `session_018u3gVHXgMYVfUxs4m1x3X4`. Class, with its row: a PR that pins a slow test by editing its body gets a green gate that ran none of them, since the slow-set step selects by names on the diff's added lines (`work/ciw/an-edit-inside-a-slow-test-that-does-not-respell-its-name-runs-it-nowhere-on-the-pr`, evidence added). Every lane was told to read its gate's step log for which tests ran. Lanes also fill the box's disk on bisects; later briefs say `CARGO_INCREMENTAL=0` and one target directory.
- 2026-11-14 16:30Z — the account's usage ran out at about 04:41Z and stopped six sessions mid-turn (the censuses and nightly-jobs fix passes; the goldens, 1e-6 and 1e-12 lanes before their first report; the default-eps reviewer at its start). Each was resumed in its own session twelve hours later and told to treat results from the interrupted window as suspect, merge main and re-check on the merged tree. The 11-14 nightly ran in the gap without any of this sitting's changes.
- 2026-11-14 16:48Z — default-eps review in: APPROVE-WITH-FIXES, no MAJOR. Taken: the gate ran the six at the default ε only (1e-6 and 1e-12 rest on local runs; the viewer app job never ran the door pair); one pin compares against the kernel's own pad and so eases as the bracket degrades; the needle-cavity pin holds the tail of one rounding spread whose unpinned rows sit 0.18 points inside the bar. Fix pass sent. Class, logged: a pin lane leaves hand-kept, name-keyed pin lists behind it (three in this PR); the lane files the row on this slate.
- 2026-11-14 17:25Z — goldens lane reported: PR 6956, head `0cbea1468`, gate green. Four body moves are pinned unjudged with fix rows on ONECOPY, CLEAVETAIL and CLEAR. STYLE review dispatched 17:27Z on that head.
- 2026-11-14 17:28Z — censuses fix pass reported (head `d895b63552`): every finding taken; main moved under it twice more (PRs 6908, 6943, 6938 re-blessed or pinned with rows on EMBED, TESSLANE, DOORS; PR 6930 landed first and sanctioned what this PR pinned). Its gate is red only on three sweep reds main carries, which run because the diff touches test-utils. It merges on that, annotated, once its run concludes.
- 2026-11-14 17:40Z — 1e-6 lane reported: PR 6934, head `5a50b4459`. A P0 row on ONEDOOR: four rows build bodies that pass every tier while their volume misses the closed form by 1e-7 to 1e-6 at ε 1e-6, since PR 6505. STYLE review dispatched 17:42Z with that as a claim to check. Class, logged: the pin PRs are red on each other's reds (a sweep-touching diff runs sweep's fast set at 1e-6 and 1e-12, and main's reds there are other units'), so each merges on an inherited red, annotated, rather than waiting for the others.
- 2026-11-14 18:30Z — PR 6931 (default-ε rows) closes: fix pass accepted on a green gate. Row `docs/model-compare-log/rows/PR-6931.md`. The lane also moved one sweep test at 1.04 s to the slow set after it reddened two of its runs on the fast-set time bar (an unrelated PR's gate going red on runner variance; evidence on `fast-set-tests-half-a-second-to-0-8-at-opt-0-on-the-slow-runner`).
- 2026-11-14 18:10Z — PR 6921 (censuses) closes: fix pass accepted, last full run red only on the three sweep reds main carries, re-run clean on main `77cf43f7b2`. Row `docs/model-compare-log/rows/PR-6921.md`. Main moved a census under this PR four times in a day (PRs 6907, 6908, 6938, 6943), which is the evidence for raising CIW's census-on-the-PR row: until a census runs on the PR that moves its site, greening them is a daily job.
- 2026-11-14 19:02Z — PRs 6921 (censuses) and 6931 (default-ε) merged. 6921 merged on its three inherited sweep reds, named in its body. 6931's closing head was green; it then conflicted with 6921 in `work/green/log.md` only (GitHub ignores the union driver), resolved by a local merge of main and merged on that docs-only resolution.
- 2026-11-14 18:30Z — 1e-6 review in: APPROVE-WITH-FIXES, no MAJOR. It re-ran the PR 6505 attribution by build for all four wrong-volume rows and read each miss against its printed enclosure: outside it every time, so the P0 on ONEDOOR stands. Taken: two pins on that ground check only that their known bad lines are present, three omit the pad that makes their line bad, and `pinned_red::row`'s containment-only match is the class (34 call sites; the lane files the row). Fix pass sent.
- 2026-11-14 18:00Z — goldens review in: APPROVE-WITH-FIXES, one MAJOR by inspection (boss_union's unjudged pin on an unbanded, mis-titled row parented to the closing unit). The reviewer judged `tilted_cut_upper` on its closed form (re-bless on merits) and kept a one-thread row. Fix pass sent.
- 2026-11-14 19:16Z — 1e-12 lane reported: PR 6967, head `1742ce38e`, gate green; no live wrong answer; six re-blessed (two of them rows that asserted a refusal and now require a build), ten pinned with rows on NEARPOINT, SECTARMS, ONEHOME, BOXFINISH, PCERT. STYLE review dispatched 19:18Z. No PR run can execute a slow test at 1e-12, so the 1e-12 evidence for this unit is local runs and the next nightly.
- 2026-11-14 19:51Z — 1e-12 review in: APPROVE-WITH-FIXES, two MAJORs, both demonstrated and both prose: a pin dated to PR 6505 on a row already red before it, on GERM's stop, which the pin did not cite; and three tracker amendments at the slanted bores that describe a main that does not exist (all six cells refuse their volume; one amendment invited an owner to close a row whose finding is only hidden behind that refusal). Fix pass sent. Class, logged: a lane's "unmeasured" in an owner's row was decidable by a twenty-line probe three times out of three; briefs from here ask for the probe before the word.
- 2026-11-14 19:58Z — PR 6934 (1e-6 rows) closes: fix pass accepted. Row `docs/model-compare-log/rows/PR-6934.md`. It merges on the inherited bead red (the 1e-12 unit's). The lane also read the 11-14 nightly: seven 1e-6 reds that 11-13 did not have and no list carries (`edit_placement_corpus_bits`, `m10_p_fence` ×2, `m4_pr3_names_ci`, `name_words_corpus`, `seat8_split_lowering`, `reader_census`, the viewer's chess-rook story). Several were re-pinned on main during the day by PLACING; the 11-15 nightly says which stand.
- 2026-11-14 19:50Z — PR 6932 (nightly jobs) closes: fix pass, a delta re-review by the same reviewer (APPROVE; its coverage mutant now goes red), and a small second pass. Row `docs/model-compare-log/rows/PR-6932.md`. The 11-15 nightly is its first run on a runner; read it against the PR body's checklist. The 11-14 nightly, without the change: the 1e-12 leg 114 of 120 minutes, the shared build step 98 minutes of a 111-minute job.
- 2026-11-14 20:35Z — PRs 6932 (nightly jobs) and 6934 (1e-6 rows) merged. 6932's closing head: exactly main's three inherited sweep reds. 6934's: the inherited bead at 1e-12 only (the cups and tilted-sphere reds are its own pins, now green); merged after a local merge of main for the `log.md` conflict GitHub cannot resolve. Four of the six units are on main; goldens (PR 6956) and 1e-12 (PR 6967) are in their fix passes.
- 2026-11-14 20:50Z — PR 6956 (goldens) closes: fix pass accepted on a green gate. Row `docs/model-compare-log/rows/PR-6956.md`. The lane moved six sweep tests at 1.00–1.10 s to the slow set after three consecutive runs failed on the fast-set bar alone with every test passing; with PRs 6931 and 6932 that is the fast-set time bar reddening an unrelated gate in four lanes in one day, and `fast-set-tests-half-a-second-to-0-8-at-opt-0-on-the-slow-runner` carries each sighting. All six units have now closed their PRs.
- 2026-11-14 20:45Z — PR 6967 (1e-12 rows) closes: fix pass accepted on a green gate. Row `docs/model-compare-log/rows/PR-6967.md`. With it the bead at 1e-12, the last sweep red every other PR's gate inherited today, is pinned.
