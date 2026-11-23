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
- 2026-11-14 — **From BOXFINISH: one main red that was not on this slate**, `main-red-the-profile-raw-door-census-names-two-lines-of-mesh-trimmed-rs` (P1): `profile` `raw_door_census::no_production_source_writes_a_vertex_table` names `crates/mesh/src/trimmed.rs:2496` and `:3017` on a clean `origin/main` `2c843a2e`. The same run saw twelve rows this slate already holds red too; the row lists them. (BOXFINISH orchestrator)
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
- 2026-11-14 21:20Z — PRs 6967 (1e-12 rows) and 6956 (goldens) merged, both on green gates. All six units cut from nightly 43371638371 are on main. The 11-15 nightly is the first hosted run of PR 6932's reworked jobs and the only hosted check of the 1e-6 and 1e-12 pins; its read is scheduled for 09:30Z, and the track is not handed back before it.
- 2026-11-14 21:25Z — the slate's last P1, `main-red-slow-rows-uncovered-at-b2337e9e`, closed: every test it names is green (two by PR 6921, the rest at all three ε rows of the 11-13 nightly). The ring-lane time-bar row another program filed here is banded P4, with the fast-set row's class. GREEN has no dispatchable P0 or P1 row; what holds the sitting open is the 11-15 nightly's read.
- 2026-11-14 — (BOXFINISH orchestrator) PR 6963 (`boxfinish/profile-directions`) moves `sym11_exact_channel_rows`' symbolic call counts too, by the same mint readings it re-pins in five other files, and leaves `sym11` un-re-pinned because the row is already red on main. Whoever re-cuts it from main after PR 6963 merges gets both moves at once.
- 2026-11-14 — (BOXFINISH orchestrator) PR 6946 lifts four ε 1e-12 pins `nightly-reds-the-1e-12-rows` lists, because its fix turns the rows green at 1e-12, 1e-9 and 1e-6: `full_turn_wall::both_doors_answer_the_bead_by_its_height_window`, `near_circular_sections`' two turned near-square rows, and `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`.
- 2026-11-15 — (BOXFINISH orchestrator) Nightly 43680833019: eight of its twelve new failing tests are in ground BOXFINISH's merged PRs own (five `revalue_corpus` rows, two sweep iso-image rows, one thread-count golden at 1e-12), and BOXFINISH is repairing them: `work/boxfinish/main-red-boxfinishs-merges-left-slow-rows-red-through-the-gates-slow-set-gap.md`. Please leave those to us; the other four new ones (`raw_door_census`, `review_chamfer_r1_probes`, `conic_quadric::cone_rows`, `section_cert::square_search`) are not ours by that diff.
- 2026-11-15 — (BOXFINISH orchestrator) Filed `the-tours-k-probe-reads-lily-wall-8-at-the-probe-scalar-where-it-refuses-nomarch` here: two `k_probe_brackets` rows and the nightly's dev-probe sweep are red on main at the lily's wall 8, read at the Probe scalar. Measured red before PR 6959's merge, so not one of BOXFINISH's reds; not bisected further.

## 2026-11-15 — the 11-15 nightly, read

- Nightly 43680833019 (main `dbb21b4a`, with all six of yesterday's units): failures 10 / 11 / 15 at the default ε, 1e-6 and 1e-12, from 22 / 35 / 38 on 11-13. Every ε row reported all 19126 listed tests (the new coverage job, green). The 1e-12 shards ran 42, 31 and 29 minutes against 114 of 120 the night before; the shared build job 71 minutes against 111; the mutant job reads its 21 pins and passes; the viewer app job ran all 1592 tests, none red; `gate self-tests` and rustdoc green.
- Of the 53 tests greened yesterday, four are red again: sym11's receipts (moved again by a later merge), one thread-count line at 1e-12 (`arc_loft_1e11eps`, both the four- and the one-thread row), and `coincidence_door`'s coaxial row at 1e-12, which PR 6967 re-stated and which passed locally for its lane and its reviewer. The 1e-6 pins all hold on the runner.
- PR 6932's checklist: items 1, 3, 4, 5, 7, 8, 9, 11 shown; 2 shown (shards under the predicted range); 10 mostly not exercised (nothing red to run past); **6 not shown**: the probe sweep runs past its plain reds and then dies in its dump half on a demo wall whose pinned refusal moved, so neither lint ran. A budget kill and a listing failure did not occur and stay untested on a runner.
- New reds: eleven tests. BOXFINISH holds seven at P0 (its PRs 6963 and 6959, merged the same day through the gate's slow-set gap) and has lanes on them. The rest, with the four above, the probe sweep's dump, and the two render lanes (a matplotlib recursion under Python 3.14, red two nights running), are one unit, `nightly-1115-residue-the-reds-no-lane-holds`: M, STYLE single review.
- The latency alarm is red: median 20.8 minutes over 128 PR runs. Yesterday's six GREEN lanes each ran several tier-`all` gates; the unit reports how much of the load that was.
- 2026-11-15 12:48Z — residue lane reported: PR 6985, head `6935b5368a`, gate green. The render lanes' cause is the runner image moving to an OS whose default Python is 3.14 (between the 11-13 and 11-14 nightlies); fixed by pinning 3.12 in both jobs and verified on a hosted `[render]` run. The raw-door census was misreading a `pub(super) mod tests` as production. One candidate wrong answer: a certified ellipse root 1.39e-6 from the oracle's crossing against a 1e-6 band at ε 1e-12, on a second seed (row on GERM, P2; the reviewer is asked which it is). Latency alarm: GREEN's lanes were 22% of the day's run-minutes; the median without them is 19.6 minutes. STYLE review dispatched 12:50Z.
- 2026-11-15 14:03Z — residue review in: APPROVE-WITH-FIXES, three MAJORs. The one that matters is not about the PR: the cone-rows fuzz red is a **false certificate**. Replayed from the row's printed inputs, the conic–quadric door certifies two ellipse roots that lie 7.8e-7 and 1.4e-6 along the arc from the true crossings against a 1e-6 band, at every ambient ε (the row builds its own band), with the residual resolved nine decades under the miss; likely the first-harmonic arm dropping a second harmonic that sits just inside the zero band. The lane had banded it P2 and written that it might be the test. It goes to P0 on the owner, with the counterexample pinned as a deterministic test. Fix pass sent. Class, logged again: a lane's "not settled whether the certificate or the test" was settled by a reviewer's replay in an hour.
- 2026-11-15 16:40Z — PR 6985 (the 11-15 residue) closes: fix pass accepted on a green gate. Row `docs/model-compare-log/rows/PR-6985.md`. The false certificate is P0 on GERM with its counterexample pinned. Three fuzz rows stay seed-dependent reds in the nightly until their owners fix them; under the rule that a fuzz row takes no fixed seed GREEN cannot green them, only file and pin the counterexample.

## 2026-11-15 — handed back

- PR 6985 merged (16:42Z): the 11-15 residue is on main. The slate holds no dispatchable P0 or P1 row, so the sitting ends and the track goes back `ready`; `plan.md`'s State says where it stands and what the next sitting reads first (the 11-16 nightly).
- Two sittings, seven units, seven model-compare rows (PRs 6921, 6931, 6932, 6934, 6956, 6967, 6985). Review tiers: STYLE single on the six pin units, FULL single with a delta re-review on the nightly-jobs unit, the tracker PRs on the orchestrator's read. Every review found something worth a fix pass.
- What the reviews found that the lanes had not, logged as classes above: a re-bless true only of the main it was run on; pins that cannot see their row getting worse (a containment-only match, a pad left out, a check against the kernel's own pad); "unmeasured" and "not settled" written into owners' rows where a short probe decided it, once on a false certificate the lane had banded P2.
- Left for owners, P0: ONEDOOR's four wrong volumes at ε 1e-6 since PR 6505; GERM's conic–quadric false certificate; BOXFINISH's own seven reds. P1, bodies pinned without a judgement: the filleted die and `boss_union` (ONECOPY), `corner_table` (CLEAVETAIL).

## 2026-11-17 — the 11-16 and 11-17 nightlies, read (Ev, in chat: green these reds; fuzzers may stay red)

- Nightly 43810139969 (11-16, main `1ca97d1d`) ran no full-suite leg: the shared build job was killed at 39 minutes by a runner shutdown (exit 143) while compiling pncad-py. Its other reds: `corrupt input` on a torn-body fuzz seed (CHARTTAIL's row has it), and `k-lint (dev-probe)` as below.
- Nightly 44023697569 (11-17, main `cdee7461`), read whole from every leg's failure list. PR 6985's targets held: `raw_door_census`, `reader_census`, sym11's receipts, the coaxial ball at 1e-12, the chamfer probe row, the render lanes, and the probe sweep now finishes its dumps and runs both lints (PR 6932's checklist item 6, shown).
- What was red, and what became of it (one PR, review tier the orchestrator's read: re-blesses and pins against named merges):
  - `predicate_name_coverage`: a test literal (`chart_region.rs`, ONECOPY's section-error-standing work) spelled as the plane chart's door; merged in PR 7060. BOXFINISH's row closed.
  - `lift_agreement_rows`: greened on main by TESSLANE's PR 7051 after the nightly.
  - `node_end_rows` at 1e-6: red from the merge that added it (SECTARMS' PR 6897), greened by SECTARMS' PR 7057 after the nightly. Bisected by a lane.
  - `certified_endpoint_census`: `quad_lane.rs` re-blessed at (27, 27), every read asking (PR 6982; SPIRICARM's row closed). PR 6897 moved the node machinery into `ssi/node.rs`; six reads there and in `sweep/critical.rs` neither ask nor carry an argument, pinned at main's counts on `work/sectarms/six-endpoint-reads-6897-moved-into-node-and-critical-are-unargued` (P2).
  - `sym_rule_f_rows`' copysign sites: `arms.rs` 3 → 5 (ONECOPY's PR 6789, the coaxial ring radius; BAND's PR 6904, the chamfer frustum's axis). Re-blessed after re-asking `sym12_the_copysign_census_at_the_nominal`: no copysign atom at a decision on any measured document. `manifest.rs` names both.
  - `committed_fixtures_are_byte_golden`: `swept_elbow.step` alone moves (five cap directions' last digits). EXPORT traced it to ONEHOME's PR 6988 (the least turn's W1 spelling), judged it rounding and regenerated the fixture on main the same day; GREEN's pin was dropped for theirs.
  - `spheres_crossing_off_every_edge`'s plain witness at 1e-12: the same refusal, now on the aggregate `Envelope` check since ONEHOME's PR 6988 (bisected); the pin's text follows, ONEHOME's row records it.
  - `k_probe_brackets` at 1e-9 and 1e-6 (the dev-probe job): `lily_leaf_a` reads a number since QUAD's PR 6092 (10-30, bisected over 12 trees), its enclosure narrowed and nested in the old one; hidden behind the wall-8 stop until PR 6985. Re-blessed (no body bracketed), QUAD told.
  - `mass_props_are_thread_count_invariant` (one and four threads, every ε): only the two `Sym` `validate_geometric` lines' `frozen` moved (14 → 16, 12 → 16), at SECTARMS' PR 7028 (bisected; check 6 now winds spline loops). Re-pinned at main's reading; the account is owed on `work/sectarms/check-6s-spline-loop-winding-froze-more-sym-nodes-in-the-thread-count-golden` (P3).
  - Fuzz rows, left red by Ev's ruling: `ellipse_torus` torus rows, `pose::window::oracle`, `square_search`, and `chord_join::wall_ring_rows` at 1e-6, which the nightly's list does not mark as a fuzzer: the lane found its seed red on `dbb21b4a` too; the seed is on SECT's row.
- Class, logged: a red that shows at one ε only is not a flake by that fact. Of the three such on 11-17, two were deterministic regressions (PR 6897; PR 6988) and one a fuzz seed.
- Bisect lanes ran as cloud sessions (tag `green-lane-2026-11-17`); this container OOM-killed sweep's test binary at `-j 4`.
- (DOCTAIL lane) 2026-11-20: closed `main-red-the-collinear-walls-probe-expects-the-edit-door-to-refuse-a-tangent-junction` in PR 7148. The R1 probe now pins the `JunctionTangent` refusal at evaluation, not at the door.
- 2026-11-19 — (SCALAR orchestrator) Filed `the-nightlys-shared-build-job-dies-to-a-runner-shutdown-most-nights` (P1 M): the shared build job was killed by a runner shutdown on 11-16, 11-18 and 11-19, so the full-suite legs ran on one night of the last four.
- 2026-11-20 — (tracker sweep) **TCOST merged in.** It held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size). Its 17 live rows moved here by `git mv`, ids and bodies unchanged, each with a dated Moved note; its charter and plan are the `## From TCOST` sections of `program.md` and `plan.md` (`docs/doc-ledger/tcost-merges-into-green.md`). (tracker sweep)
- 2026-11-22 — (EMBED orchestrator) **Two sweep rows on the 1 s bar moved to the slow set, beside the step-import row PR 7120 moved** (`embed/step-import-bar`): `step-import::all an_edge_is_read_on_its_description_whatever_recognition_says::a_straight_rim_recognition_declines_is_carried_on_its_column` (1.62 to 1.72 s on three gate runs; added by PR 7304 without its slow-set line) and two sweep rows on the bar (`join_pierce_strut_facing::two_out_runs_at_the_corner_build_in_every_op`, `band_turn_overrun_oracles::generic_hexahedra_build_and_fill_at_the_clip_oracle`). They redded `gate ok` on PRs 7115 and 7119 with every test passing; the pole lane timed them the same on main and on its head. The row `a-step-import-row-runs-at-1-65-s-and-two-sweep-rows-at-the-bar-and-red-a-pr-off-their-path` arrives with PR 7115 and is closed there. Why the step-import row costs 1.65 s is not looked at; that is EXCH's if it matters.

## 2026-11-22 — picked up (Opus 5.5 orchestrator)

- Track `active`. The slate: one P0, eight P1, all main reds but the slow-row selection gap and the nightly build job. The build job has run on every nightly since 11-20 (three scheduled or dispatched runs, 113-114 minutes each against its 120), so the shutdown row and the P2 near-limit row are one unit now. `main-red-three-source-censuses-red-since-the-told-quantity-type` is SCALAR's PR 7267 (gate green, open): no lane; it closes when that merges.
- Units, each with its letter and review tier, logged before any draw:
  - **U1 `green/small-reds`** — the ringless-cone 1e-12 pins (fix written in the row), the tour's probe build reading a circle's radius through its accessor, step-import's recourse roster. Letter S. Tier: orchestrator's read — each is written down in its row and mechanical. Not a model-compare row.
  - **U2 `green/rim-ball`** — the P0: bisect the cylinder_sphere_frame r 0.1 rim refusal and say whether the refusal or the row is wrong, then green it. Letter M. Tier: FULL single — whether a refusal is right is a correctness claim a read cannot settle.
  - **U3 `green/editor-1e-12`** — editor-core's four 1e-12 reds (the plates' sym ledger since PR 7183, the e2e slider and two revalue rows since before 11-19): bisect, judge each new reading, re-bless or pin. Letter M. Tier: STYLE single, as the earlier pin units.
  - **U4 `green/slow-row-selection`** — the PR gate selects a slow row whose body, doc or read table a diff touches. Letter M (re-priced from C: the const-table case is more than a written fix). Tier: FULL single — a selection rule that silently selects too little is exactly what a read does not catch.
  - **U5 `green/nightly-build`** — the shared build job: why runners shut it down, and its 113-114 minutes against 120. Letter M. Tier: FULL single — workflow changes whose failures are silent skips.
  - **N `nightly read`** — nightly 44927630239 (`dbc608ff`, dispatched 11-21 17:00Z) read whole from each leg's junit against the slate. Exploration, no PR.
- 02:35Z — Fable usage limit exhausted (resets 16:00Z): U2, U3 and U4 died at dispatch. Ev, in chat: run everything on Opus, no experiment rows, and where a protocol wants two reviewers run both on Opus. U2-U4 re-dispatched on Opus; no model-compare row for any of U1-U5 (rule 12; the note is on `analysis/model-compare/green-2026-11-22`). Running: U1, U4, U5 and the nightly read in this container; U2 and U3 as cloud sessions (sweep's and editor-core's 1e-12 builds do not fit 9 GB).
- 03:00Z — nightly read in. **Run 44927630239 (17:00Z, `dbc608ff`) was dispatched on PR 7264's branch, not main**, as were the 14:50Z and 15:08Z dispatches; the last nightly of main is the scheduled 44870651337 (`f5ef8f2b`). U2 and U3 were redirected to it. Coverage sound (19874 tests per ε row). Held already: most reds (rows on GERM, SCALAR via PR 7267, SECTARMS, SPIRICARM, SSIARITH, DOCTAIL, CHARTTAIL, BAND). The rustdoc gate's red is the tour's private-radius build (U1). The render lane's red is GERM's nist_ftc_09 snap row; that row now names the lane.
- Unheld, filed: `main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17` (P1; plus sweep's sphere offer row, likely PR 7159), dispatched as **U6 `green/eps-1e-6-sweep`**, letter M, STYLE single (cloud). `pr-ci-latency-median-is-32-minutes` (P2: median 27.2 min on 11-21's alarm against the 15-minute bar, no live row). Dispatched anyway under the Friction rule as **U7 `green/pr-latency-2`**, letter M, FULL single (container, run records only).
- 05:15Z — the orchestrator's session process was OOM-killed around 02:40Z (9 GB box shared with other sessions' kernel builds; another program's topo build held ~4 GB at the restart), and every lane stopped with it, the `isolation: remote` lanes included: none of U2, U3 or U6 had pushed. Lanes resumed from their transcripts; the local ones were told to commit their uncommitted work first and to build at `-j 2`. Class, logged: a lane brief says push after every coherent step, and three lanes still held 2.5 hours of work unpushed when the process died.
- 06:10Z — WSL itself crashed on memory with the seven lanes resumed. Cause, read: the three "cloud" lanes (U2, U3, U6) were never remote: `isolation: remote` ran them on this box, and their briefs (written for a cloud session) named no build slot, so up to four kernel builds ran beside other programs' on 9 GB. All seven resumed again from their last pushes, every cargo lane now in a lane clone under `cad-work/`, wrapped in `with-build-slot.sh` at `-j 2`. Class, logged: a brief written for a remote session carries no local-resource rules, so if the lane does not actually go remote it runs unconstrained.
- 07:00Z — U4 reported: PR 7359, head `e8e48aef39`. A slow row is now selected when the diff edits its file, or a golden file it names by a literal. Cost measured over 80 merges: nothing on 58 of them, a median +30 s on the other 22, and +8.7 min on PR 7131, a tree-wide PR. Ruling (orchestrator's, logged): no cap. A cap would restore the blind spot on exactly the PRs that touch the most slow rows, and tree-wide PRs are rare; if the latency alarm (U7) finds this shape driving its median, revisit. The gate's one red, `python suite`'s NameCarrier census, is believed inherited; the review is asked to settle it. FULL review (Opus) dispatched on the frozen head. Residue: `a-slow-row-reaching-its-data-file-by-a-built-name-is-not-gated` (P3) filed; the shared-fixture row annotated.
- 07:50Z — PR 7359's review is in (FULL, Opus): APPROVE-WITH-FIXES, no MAJOR, and every claim confirmed by re-running it.
  - MINORs, demonstrated:
    - a selection past about 1450 entries overflows `-E`'s 128 KiB argv string (E2BIG, exit 126), a cost the no-cap ruling had not priced;
    - `src/tests.rs` mapped to `lib`.
  - The python red is main's, from PR 7346; fix PR 7377 is open.
  - Fix pass sent to the implementer:
    - taken: M1 (a sound compaction of the expression), M2, N1-N4, and the subprocess-helper and stub-rule style findings;
    - argued down: N5 (over-selection is sound) and Q4 (the discipline sentence is still true, and it is Ev's text).
- 08:00Z — Ev, in chat: this machine is very slow, so lean heavily on CI; pushing uncompiled code is fine. Every lane is told to do this (the note is `~/.local/share/cad-work/green-ci-first-note.txt`):
  - CI instead of local builds;
  - a slow row run on CI by putting it on a branch whose diff selects it;
  - bisects as one throwaway branch per candidate merge, all pushed at once (`ci.yml`'s concurrency group is per ref, so they run in parallel), deleted once read.
  - U7 is told to take GREEN's own runs out of the latency median.
- 2026-11-22 — U7 reported: PR 7364, head `be9424e5f7`; its gate is red only on the inherited python census (PR 7346, fixed on main by PR 7377).
  - **Measured: PR CI's latency is queue, not run time.** The wall median rose from 15.7 to 26.7 min between 11-15 and 11-21, while active time went only 15.4 → 17.1. The 20-job concurrency cap was saturated for hours (at 06:30Z on 11-22: 20 running, 76 queued). PR runs went from about 90 a day to 200 or more, and a quarter of the job-minutes are runs cancelled by a newer push.
  - The PR's change moves the python and mesh-budget jobs to the test profile (opt 1, debug assertions on) with cache-prime's key. That saves about 17 job-minutes per run seeding both; it is demand, not the median.
  - Filed:
    - `pr-runs-queue-behind-the-twenty-job-cap` (P2; it holds the alarm question);
    - `the-gates-step-rescans-the-tree-once-per-gate` (P2; 7.6 min on every run, docs-only included);
    - `pr-only-jobs-rust-cache-is-never-saved` (P3).
  - FULL review (Opus) dispatched on the frozen head.
  - Recorded for the CI-first direction: every GREEN bisect branch is more queue for every program. Lanes are to prefer one branch per batch of candidates over one per merge where a run can test several.
- 2026-11-22 — PR 7364's review is in (FULL, Opus): APPROVE-WITH-FIXES. All six claims were confirmed from run records: no detection lost, the cache restores at a full match, python 29.8 → 15.4 min and mesh-budget 12.1 → 7.5 min.
  - MINORs:
    - the branch conflicts with main in three rows PR 7371 closed, so it gets no run;
    - four CI env blocks must stay equal by hand, and a drift is silently slow (the fix minted another copy of the defect it measured);
    - the job-minute share is quoted two ways.
  - Fix pass sent: all taken but Q6 (timings in a PR body) and the scope note (state-sync rows from the orchestrator branch). The env-block equality gets one source or a gate; a sibling cancellation row and the GIL-release row go to BIND.
- 2026-11-22 — U3 reported: PR 7389, head `60c1d63e88`, gate green. The four editor-core reds are bisected and re-blessed; none is pinned red.
  - Plate ledger: PR 7183's ratified conic speed readers add 14 forms; one digest move is left untraced.
  - e2e slider: PR 7088. The receipt is now ε-independent; before, it varied non-monotonically.
  - The revalue rows: PR 7114, whose floors were set from a default-ε run only; the floors are now per ε.
  - The drive-memo row is SCALAR's at every ε.
  - Verified on CI: a throwaway branch with a one-job workflow at all three ε. The bisect pushed 18 branches at once, before the 3-4 rule.
  - STYLE review (Opus) dispatched, with claims on the untraced digest and the folded ε pins. A re-bless that blesses an unexplained move is what the 11-14 reviews kept catching.
  - Found: `isolation: remote` agents run in a harness worktree under `/home/evan/projects/cad/.claude/worktrees/`, and the harness refuses their git elsewhere.
- 2026-11-22 — PR 7389's review is in (Opus): APPROVE-WITH-FIXES, one MAJOR, demonstrated in part. The plate ledger's re-pin blesses a `Plain/Decision` digest move that is not PR 7183's: the old pin reads exactly on PR 7131's 11-19 branch, and reverting 7183's readers on main does not restore it. Untraced; the doc credited it to the readers.
  - The other three re-blesses hold. Folding the e2e pins makes the row stricter, but ε 1e-7 and 1e-11, where the old dependence showed, are gated by no run.
  - Fix pass sent: bisect the digest in 3-4 branch rounds, or name it untraced at the claim site with a row; plus MINOR-1-3, rows on ONEHOME and HELPER, labelled floors, and a trimmed doc. NOTE-1 (count−1 floors) argued down as pre-existing.
  - Class, logged again: a re-pin "every count comes back" can still bless a digest that does not.
- 2026-11-22 — U2 reported: PR 7392, head `a8cd4e366d`, gate green (the pin ran on the slow-set step at default ε; matched against nightly 44870651337's junit at 1e-6 and 1e-12).
  - The P0 is a regression from SECTARMS' PR 7052 (`sectarms/periodic-loop-carrier`): first parent passes, merge fails, both on hosted CI.
  - Probe: the section edge meets the drum's cut plane only at its two rim-vertex ends and clears it elsewhere. `clears_plane` needs every control point strictly off the plane, so it can never clear a clamped carrier that ends on it. Six neighbouring poses refuse alike.
  - Pinned under `pinned_red::row` at every ε, against a new P0 on SECTARMS (`a-rim-ball-refuses-b-minus-a-at-the-slits-unread-carrier-since-the-periodic-loop`). The fix is in the slit's plane reading, outside 7052's hunk.
  - FULL review (Opus) dispatched.
- 2026-11-22 — PR 7392's review is in (FULL, Opus): APPROVE-WITH-FIXES, no MAJOR. The bisect, the probe numbers, the pin's single-run match and the sibling dispositions hold.
  - Corrected: the refusal is a conservative answer from a certificate too weak to clear a clamped carrier ending on the plane, not a wrong answer. The P0 band stands.
  - The pin did not hold the edge and face keys (the containment class again).
  - Fix pass sent: all taken, including `design: true` on SECTARMS' row (strengthen the certificate, or restore the seam split). GREEN's row closes at merge.
- 2026-11-22 — PR 7359 (U4, slow-row selection) closes.
  - Fix pass accepted. A delta re-review by the same reviewer gave APPROVE: 3000 randomized slow sets with no entry dropped, `not default()` read from nextest 0.9.140's source, and the step's shell dry-run at the all-selected case and at 7131's.
  - Tier FULL single, Opus. No model-compare row (Fable outage).
  - The hosted slow-set step has still never run on this PR's own gate, because a scripts-only diff seeds no crate. The first PR that edits a slow row's file runs it: check that run's step log.
- 2026-11-22 — PR 7371 (U1) closes, on the orchestrator's read and a green gate.
  - The ringless cone pins and the tour's private-radius build were already green on main, by TESSLIE's PR 7198 and GERM's PR 7226.
  - step-import's coverage red is held by a `KNOWN_MISSING` entry citing `work/importgate/step-import-runs-no-recourse-roster-over-its-band-funnels.md` (P3). A real roster needs the band layer's funnels renamed, which is IMPORTGATE's call.
- 2026-11-22 — PR 7364 (U7, PR CI latency) closes.
  - The fix pass was taken whole on a green gate (run 45015627267). The env-block equality became a gate, not a single source: anchors went unchecked against the pinned actionlint.
  - The cancel-mid-run sibling went to BIND as a row: an event would make it exercise less.
  - FULL single review, Opus; no model-compare row (Fable outage).
  - The latency row closes, and the queue row holds the remainder.
- 2026-11-22 — PR 7359 (U4) merged on its one red, an inherited varying-seed torn-body fuzz row held by CHARTTAIL (`a-split-reduce-on-a-torn-body-panics-at-pcurves-unreachable-own-loop`). Annotated on the PR.
- 2026-11-22 — PRs 7359 and 7364 merged. Orchestrator's own slip, logged: a merge of main into 7364's branch conflicted in a row, and the same chained call's `git add && git commit` committed the conflict markers and pushed them. It was caught by lint in the same output and repaired by a follow-up commit before the PR merged. Never chain a commit after a merge in one call; grep the tree for markers first (`memories/agent-lane-operations.md` says so).
- 2026-11-22 — U6 reported: PR 7383, head `65b9c2da45`. Its gate is red only on the rim-ball row, which PR 7392 has since pinned on main. All eight 1e-6 reds were bisected on CI in rounds of four.
  - TubeStraddles ×3: PR 7052, SECTARMS' sliver row; pinned at 1e-6.
  - Tipped rod ×2: PR 7112 (ONEHOME). Past-band tips now build bodies whose volume pads reach 2.6e-3. Filed P1 on ONEHOME as no wrong answer; pinned.
  - review_curved ×2: PR 7208 (ONEHOME); the class is the level-root row's; re-blessed at 1e-6.
  - The sphere offer row: greened by SCALAR's PR 7267.
  - STYLE review (Opus) dispatched, with claims on the tipped rod's band (is a body 2.6e-3 loose against a 1e-7 budget a wrong answer?) and on the re-bless's tightness.
- 2026-11-22 — U5 reported: PR 7365, head `c1f4a5f9d3`, gate green.
  - Measured: the nightly build job is memory-bound. Two or three rustc processes run at 5-6 GB, and available memory stays under 3 GB for 15-23 min; disk never drops below 76 GB free. Each kill carries the "starves it for CPU/Memory" annotation, including today's main nightly on the old config.
  - The fix:
    - 12 GB of swap;
    - viewer, pncad and pncad-py built at opt 1 (about 70 unit-minutes for about a minute of tests);
    - the topo scalpel in its own job;
    - a resource sampler in the step log.
  - Hosted nightly 44989234943 on the branch: build 98.3 min against 113-118, and all five ε legs ran.
  - The lane closed both build-job rows on that one night, and filed the CPU-bound margin as P2.
  - FULL review (Opus) dispatched; one of its claims is whether one night closes the shutdown row.
- 2026-11-22 — PR 7383's review is in (Opus): APPROVE-WITH-FIXES, one MAJOR.
  - MAJOR: the review_curved re-bless's filter never reads `v=` against `want=`, so a wrong-volume body with t3p=false passes as known. Three pre-existing filters in that file share the hole (monotone in the wrong direction).
  - The tipped-rod P1 is upheld by probe:
    - no body has both a wide pad and a t3′ failure;
    - every failure is an honest in-band `CensusEscalated { Indeterminate }`;
    - every volume is inside its claim.
  - Also found: 7112's first parent named wrongly; the topo row never bisected; the coarse class wider than ONEHOME's row; a pinned stop hiding the parallel pose's SOUND check.
  - Fix pass sent with all taken but S6.
  - Class, logged: a re-bless filter keyed on a verdict line's prefix inherits whatever that verdict does not check.
- 2026-11-22 — PR 7365's review is in (FULL, Opus): APPROVE-WITH-FIXES, no MAJOR.
  - Nothing a leg reads is lost: 37 binaries in the archive, 19885 tests per ε row, and the scalpel unchanged in its own job. The swap and the sampler are sound.
  - The saving is mis-attributed: opt 1 buys about 13 min and the scalpel move about 10; the probe's gap is mostly runner speed. The sampler's log does not survive a lost-communication kill.
  - Closing the P1 on one dispatched night would remove the only watcher. Ruling: close on the change, with a P2 re-check row for five scheduled main nights.
  - Fix pass sent with all taken but the wasm list.
- 2026-11-22 — PR 7392 (U2, the P0) closes.
  - Fix pass taken whole on a green gate (run 45026090602). The pin now holds the whole one-element refusal list, edge and face keys included.
  - GREEN's row closes. The red's owner is SECTARMS' P0 (`a-rim-ball-refuses-b-minus-a-at-the-slits-unread-carrier-since-the-periodic-loop`, `design: true`: strengthen `clears_plane`, or restore the seam split), so one red is one open P0.
  - FULL single review, Opus; no model-compare row (Fable outage).
- 2026-11-22 — PR 7389 (U3, editor-core's 1e-12 reds) closes.
  - Fix pass taken whole on a green gate. Verified hosted at all three ε (run 45033589672).
  - The review's MAJOR is settled: the plate ledger's untraced `Plain/Decision` digest is PR 7193's (`sect/certificate-by-support`), the same forms built in another order. An order-free sum of the form digests is unchanged across it; only the chain moves.
  - Floors are named by ε, and `(compared, checks)` is pinned across rows.
  - Rows filed: ONEHOME (the e2e receipt's old ε-dependence, P3), FIXTURE (the copied ε-row lookup, P4). Both GREEN rows close.
  - STYLE single review, Opus; no model-compare row (Fable outage).
- 2026-11-22 — PR 7383 (U6, the 1e-6 reds) closes, fix pass accepted on a green gate (run 45042088927).
  - The MAJOR's fix was read by the orchestrator: `at_volume` fails closed (a missing `v=` or `want=` rejects the line) and holds both within `VOLUME_CLAIM` (1e-7), in the `coarse`, `main`, `shared` and `vertex` filters. The coarse class is bounded to 26 named (row, ops) pairs. The topo row is now bisected to PR 7052 too.
  - Residue:
    - ONEHOME's tipped-rod row (P1);
    - SECTARMS' sliver row (P1, three more tests);
    - `review-curved-accepts-an-invariant-refusal-at-any-pose` (P2, GREEN);
    - ONEHOME told to re-weigh the level-root row.
  - The sphere offer row rides SCALAR's PR 7267.
  - STYLE single review, Opus; no model-compare row (Fable outage).
- 2026-11-22 — PR 7383 merged. Three rows landed on GREEN's slate from other programs, and all are dispatched (Opus, CI-first):
  - **U8 `green/seamless-pipe`**: the P0 topo 1e-12 red. Letter M, FULL single.
  - **U9 `green/held-reds`**: the P0 list of reds held by closed rows or none, including the unheld editor-core kite-moats row. Letter M, STYLE single.
  - **U10 `green/fast-set-margin`**: every fast-set test at a hosted p90 of 0.5 s or more moves to the slow set, applying discipline §2's rule as written; only the list changes. Letter S, STYLE single.
- The bar's *policy* is a design question (`design: true`): what it measures, what a breach does on a PR it does not touch, and who owns a test that drifts. Two Opus designers are weighing it (Ev: Fable out, no experiment row). Their reports decide whether this needs an `[ev]` PR.
- 2026-11-22 — The bar designers converged in one round, both Opus (Fable out; no design-fork row, Ev in chat):
  - re-time a flagged test alone ×3;
  - fail a PR at 0.5 s only for a test its diff adds or edits, or at a ceiling (about 2-2.5 s) for any test;
  - drift on main owned by a separately named check over PR-run medians.

  It rewords discipline §2, which Ev ratified in PR 5229, so it waits for Ev: **`[ev]` PR 7397**, armed on its comments.
- Parameters left to the orchestrator and decided: the fastest of the three re-runs, a ceiling starting at 2.5 s and set from measured ratios, and no warning row for other readings.
- U10 is measuring and not moving, since a list cut from contended readings over-selects.
- 2026-11-22 — U10 measured, moved nothing:
  - 76 gate jobs. Run alone on the gate's build, none of 330 candidates reaches 0.5 s (top 0.458).
  - The gate median is 1.73× the solo time, and three runner classes sit at 0.8, 1.0 and 1.33.
  - A contended-p90 cut at 0.5 s would have moved 162 cheap tests.
  - Ruling (orchestrator's, a stopgap until Ev answers 7397): move only the 5 tests at gate p90 ≥ 0.9 s, the band that reds PRs. The data is posted on 7397.
- 2026-11-22 18:50Z — Ev on [ev] PR 7397: "we can increase the hysteresis (maybe 0.25 rather than 0.5?) but i still want it to get caught if an old test becomes dramatically slower". The PR is updated in place:
  - a test added or edited must stay under 0.25 s run alone;
  - drift on main is flagged at 0.5 s, or on a severalfold jump in its median;
  - the ceiling (about 2.5 s run alone) fails the PR that causes a gross slowdown.
  One question is left for Ev at the top of the body: whether a severalfold jump below the ceiling may wait a day.
- U9 reported: PR 7400, gate green. Six of the seven reds are now named by live rows. The kite-moat digest is bisected to PR 7131 and re-blessed as 7131's `Debug` spelling of a plane. STYLE review (Opus) dispatched, with a claim that the values themselves did not move.
- Today's scheduled main nightly (44989179403) was killed on the old build config. Dispatched nightly 44989234943 ran on PR 7365's branch, not main. So main's ε legs have not been read since 11-21; tomorrow's nightly is the first on main with PR 7365, and its read comes first.
- 2026-11-22 — PR 7365 (U5, the nightly build job) closes, fix pass taken on a green gate (run 45042902274).
  - The opt-1 override now fails loudly on a missing package. `build-default-env-agrees.sh` covers nightly.yml's shared keys. The sampler names each rustc by package.
  - The saving is re-attributed: about 13 min from opt 1 and about 10 from the scalpel move.
  - The P1 shutdown row closes on the change, watched by `the-nightly-build-job-holds-over-five-scheduled-main-nights` (P2). The margin row (P2) carries the trend.
  - FULL single review, Opus; no model-compare row (Fable outage).
- 2026-11-22 — U9 (`green/held-reds`): the held-by-closed-rows P0. Re-read on the nightly and on current main at every ε. The tilted rod (SECTARMS, P2), offer rows (SCALAR, P1, PR 7267) and the drive-memo race (SCALAR, P3) are each held by a live row, with per-ε evidence added. The plates ledger and the spline fuzz are green on main. The seamless pipe's enclosure row joins its junction sibling's GREEN P0 (one payload). The kite moat moved twice: spelling only at PR 7131, values at PR 7411 (re-posed `plane_offset`), so it is not re-blessed and a new GREEN P0 holds it (U14, 2026-11-23). The row stays open.
- 2026-11-22 — (CIW orchestrator) **Seams from CIW's triage, for GREEN to weigh.**
  - CIW holds the gates-step latency as its P0 (`work/ciw/the-lint-jobs-gates-step-scans-the-tree-64-times-for-5-to-11-minutes-on-every-pr`, lane `ciw/gates-step-latency`). GREEN's `the-gates-step-rescans-the-tree-once-per-gate` is the same finding; CIW closes it with the P0's PR unless GREEN objects.
  - Three CIW rows closed as duplicates of GREEN's: `a-fast-set-test-two-percent-under-the-one-second-bar-reds-prs-that-do-not-touch-it` and `geom-cores-aggregation-row-runs-past-the-fast-set-bar` (both into `the-one-second-bar-reds-prs-off-their-path-several-times-a-day`; PR 7190 and PR 7174 are sightings for its table), and `a-chart-branch-guard-row-is-red-locally-and-green-on-the-hosted-kernel-job` (into `a-slow-row-reaching-its-data-file-by-a-built-name-is-not-gated`).
  - GREEN's P4 `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster` sits inside CIW's P1 `a-stored-data-change-runs-none-of-the-slow-rows-that-read-it` (now: a named canary of pins and source censuses run on any `crates/*/src` change); close it there or say why not.
- 2026-11-23 — `the-gates-step-rescans-the-tree-once-per-gate` closes as done by CIW's PR 7439: the gates step is 1:06 on a hosted runner, down from 5:22–10:54 (run 45069292031).
- 2026-11-22 19:29Z — Ev on 7397: "if the bar is 0.25 then the drift check should probably flag there too? and would it be necessary to still have it at 2.5s rather than back at 1s with the increased hysteresis?" Yes to both; the text and body are updated.
  - One line, 0.25 s run alone, for new, edited and drifting tests; the drift check times tests alone too.
  - The ceiling for any test is back at 1 s, safe because re-timing alone removes contention and the drift check keeps the fast set under 0.25 s.
  - Cost: about 100 tests leave the fast set on the first drift run. Awaiting Ev's approval.
- U8 reported: PR 7402, gate green. The P0 seamless-pipe 1e-12 red is a `Stalled` refusal under PR 7131's bracket cost; 7131's own mutant F1X alone builds both poses. Re-blessed as a tight pin on the class Ev accepted on 7131, though the margin (2.19e-12) is wider than anything 7131 listed. FULL review (Opus) dispatched, with that as a claim: is it inside Ev's acceptance?
- U10: PR 7399 moves 4 tests. Its gate is red on two causes outside the diff: the bar on `a_knife_edge_fillet…cusp` (1.11 s; left, the class 7397 decides) and `a_cylinder_wall_through_a_prism_corner…` at 1e-12. The latter was not on the 11-21 failure list; filed P1 on TESSLIE. Tier changed to the orchestrator's read: the unit shrank from a sweep to a measured four-line interim list.
- 2026-11-23 — (CIW, `ciw/census-digest-goldens`) **The digest and count goldens on GREEN's ground are narrowed to what an intended change does not move** (row `census-and-digest-pins-cost-a-fix-a-day-and-caught-eight-real-defects-in-a-month`). The sweep thread-count suite compares the 1- and 4-thread walks with a serial walk run beside them (the shape report sends `topo::props` down its serial arm), and its golden files are gone; the rim-Zero row asserts the winding door reads no theorem more often than the build (the 10 → 18 catch still reds, re-planted); the three measured-pad pins are deleted, their ε-scaled ceilings kept; `fixture::digest` and the name digests (`lib_g16`, seat4/7/8, `msolve14`'s pin) read cells by geometry with no arena key or mint id; `m4_pr3`'s die golden is a replay-identity check beside an id-free names digest; `perf2` reads every name back through the reverse map and keeps only the persisted-text digest; editor-core's `perf12_census_goldens` is deleted for the 40- and 160-fin pre-filter differential in `perf12_census_bvh_diff`. Open rows that pin a line of a deleted golden (the thread-count `sym validate` lines, `check-6s-spline-loop-winding...`) lose their pin site; their accounts are still owed if wanted.
- 2026-11-23 — (CIW, `ciw/census-text-pins`) Text pins narrowed to their claims (`work/ciw/census-and-digest-pins-cost-a-fix-a-day-and-caught-eight-real-defects-in-a-month.md`): `m5_pr12_die_body` and `m6_surgery` match `UnbuiltEnd::CurvedEndFace` instead of its text; `germ_tilted_rod`'s steeper union resolves the refusal's faces to the donut's torus wall and the rod's cylinder instead of pinning `FaceKey(3v1)`/`FaceKey(4v1)`; `refusal_concision_chains`' `OVER_BUDGET` ceilings are bounds (a row shortening under one no longer reds); `test_north_star.py`'s teapot spout row no longer pins the stop; `test_utils::source::cfg_test_extents` is public.
- 2026-11-23 — (CIW, `ciw/census-text-pins` fix pass) `name_words_corpus`' `OVER_BUDGET` is a ceiling like `refusal_concision_chains`': a row that shortens no longer asks for its number to be lowered; one that grows still reds. `germ_tilted_rod`'s steeper union also asserts its torus face is the half donut's inner wall.
- 2026-11-23 — **From CIW (PR `ciw: hand-kept census lists derived or deleted`):** the hand-kept census lists in GREEN's slow set are derived or dropped, per `census-and-digest-pins-cost-a-fix-a-day-and-caught-eight-real-defects-in-a-month` (19 reds in a month, no real catch). `reader_census` drops its 89 `Shared` lines (a site reaching the shared lexer is read off its code) and its `Parsed` scope lines; `node_standing`'s `READERS` loses its counts; the deny and hand-written-impl censuses compare their walk with an independent crude count instead of `ATTRIBUTE_SITES_TODAY`/`IMPL_FILES_TODAY`; `flagged_census` drops `LEDGER_FLAGGED_SITES`; the topo and geom-brep rosters drop `INDIRECT`; `predicate_name_coverage` derives its holds and drops `KNOWN_MISSING`. Slow-set names follow in `.config/nextest.toml`.
- 2026-11-23 — **Note from BOXFINISH: the PR queue was about two hours deep at 02:58 UTC, and it looks like fewer than twenty jobs are executing** (106 runs queued, 6 in progress; the three in-progress runs read each had one job executing and none waiting). The reading is appended to `pr-runs-queue-behind-the-twenty-job-cap`. By the tracker's Priority section a PR CI slower than its alarm is P0, and that row is P2: yours to re-band. `[ev]` PR 7508 asks Ev whether reviewed PRs may merge on a locally run gate meanwhile, and to look at the account's concurrency.
- 2026-11-23 — **From EMBED: `main-red-recourse-roster-misses-chain-parity-and-spline-trim-decide-sites` is being fixed on EMBED's landing branch.** The row has been open since 21 Nov and the slow row is still red on main (now four complaints: `Const(NEST)` and `Const(REACH)` match nothing in src, and the two indirect decide sites). It runs only on a PR that edits topo's roster, and EMBED's landing branch `embed/wave3-land` (eight approved units under one hosted run) adds a roster line, so it blocks that landing. A small lane (`embed/roster-main-drift`, cut from main, test file only) re-pins the four with a disposition written from each site and extends the row; it rides the landing branch. If GREEN has a fix in flight, say so on EMBED's log and the lane's commit is dropped in favour of it. (EMBED orchestrator)
- 2026-11-23 — **From EMBED: `main-red-geom-breps-recourse-roster-misses-three-ssi-enclose-cone-sites` reads 14 complaint lines on main `a85cfefa`** (EMBED's landing verifier ran all 24 rosters on its branch merged with that main: 23 green, this one red, and the landing's diff does not select it, so EMBED fixed nothing there). Topo's roster row is fixed on PR 7491 as noted above. The endpoint census's `pair_section/ruling.rs` drift is on CIW's log. (EMBED orchestrator)
- 2026-11-23 — (CIW orchestrator) Carried over from CIW's closed duplicate `a-fast-set-test-two-percent-under-the-one-second-bar-reds-prs-that-do-not-touch-it`, for `the-one-second-bar-reds-prs-off-their-path-several-times-a-day`. There are two sightings on 11-23 (PR 7221 run 45088390543, PR 7203 run 45076908929). There is also a defect in `scripts/check-fast-set-times.py --new-since`: an added module is keyed by its bare name (`pkg:mod:tests`), so one added `mod tests` anywhere in a package holds every `…::tests::…` test of that package to the new-test bar. Keying the token by the module's path would fix it.
- 2026-11-23 — **From EMBED, for `main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9`: PR 7203 moved four more digest pins, and a re-bless for exactly those is on a branch.** Bisected on clean checkouts by an EMBED fix lane (EMBED's landing gate does not select these rows; it was found while checking that): at PR 7203's merge `2b9cd8b24`, `seat7_sweep_lowering` and `seat8_split_lowering` read cut_cylinder `0x99cf4ac091e5776c` (was `0xed031d8ebcfb2ed4`) and kitchen_sink `0x2a79f21a919993e1` (was `0x4080aff5e382f53d`). **Only the `Debug` spelling the digest feeds moved, no geometry**: PR 7203 renamed `Circle`'s and `Ellipse`'s stored frame to `at` and added `Ellipse`'s `held`. Proof: with a hand-written `Debug` in the old shape (asserting every ellipse centre-held; the assertion never fired) both tests read their old pins on main `2130aa45f`. The moved documents are the ones with conics (cut_cylinder 20 circles and 16 ellipses, boss_union 24 circles, kitchen_sink 16 circles); die and corner_table have none and held.
  - **Branch `embed/editor-digests-main-drift` at `d7d679faa1`** (cut from main `2130aa45f`, test files only, no PR): the four pins re-cut, each comment naming PR 7203, and seat8's RE-BLESSED paragraph. On it seat8 passes; fmt and editor-core clippy clean. GREEN's to merge or to take the diff from.
  - **Not re-blessed, still red on main:** `seat7`'s boss_union (`0x7cc355810af7b629` = SECTCONE's unjudged move by PR 7354 plus PR 7203's spelling; its value before 7203 was `0x2ea09f26ff2947dd`) and `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval` (PR 7354's, unchanged by 7203). Blessing either would bless SECTCONE's unjudged move. A correction to this row if it says PR 7500 greened them: PR 7500 left both red in its own words.
  - Not swept: other `Debug`-fed digest rows PR 7203 may have moved. (EMBED orchestrator)
- 2026-11-23 — **From EMBED: nine topo and sweep slow rows red on main `a85cfefaa` (23 Nov about 02:00Z), found by running both crates' whole slow sets on EMBED's landing branch and then on that main alone.** Not re-checked on a later main, and not checked against GREEN's open rows: `topo::all a_corner_reads_only_its_own_sectors::the_corners_offer_executes` (offers 9.999999999999951e-12 against a quoted 9.999e-12) and `::an_unglued_pair_parallel_at_the_arm_refuses_on_the_arms_tilt`; `topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it` (`lever_arm_sector_side_in_the_zero_band`); `sweep offer_rows::every_sphere_offer_passes_just_below_it` (`nested_off_axis_in_the_zero_band_union`); `sweep::all bisector_nodes::the_near_node_poses_at_the_witness_band`; `sweep::all section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds` (`curve_carrier_gap` at 1.0000000272e-9); `sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` (150 s); `topo boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section` (`CAD_FUZZ_SEED=0xf0955626e9ba1237`); `topo face_normal::tests::no_source_file_folds_the_bit_by_hand` (`step-import/src/one_record.rs`, `from_n`). Command: `cargo nextest run --profile ci --ignore-default-filter -p topo -p sweep -E "(package(topo) | package(sweep)) & not default()" --no-fail-fast`, the kernel job's build env. (EMBED orchestrator)
- 2026-11-23 — **From PROFILE:** three reds on main met while landing PR 7264, measured on a probe of `origin/main` alone (run 45164635800) unless marked. I did not check each against your open rows. (1) Lint: the `payload-rung-sweep` gate flags `NodeErrorClass` at `eval/class.rs:41`. (2) The least turn (`7661fad317`) added numeric reads and left slow rows unpinned: `m10_9`'s pad and registry rows, `m10_bulge`'s boss and D-tab rows, the plate's walk ledger, `sym11`, and by inference `sym_9`. (3) Inferred, not probed on main: every `revalue_corpus` row fails "a DAG re-valued at p0 is not the p0 build's own bits" since main's `9eb483e0..704a3eef`. PR 7264 is merged, so a re-pin of (2) measures its counts already in; its share is in `work/profile/log.md`, 2026-11-23.
- 2026-11-23 — **From TESSLANE:** reds met while landing PRs 7498, 7517 and 7526, none of them in the mesh. I did not check each against your open rows, and none was re-run on a main later than the one named. (1) The tour's `wall_jets::the_klein_loops_walls_hold_against_themselves` is red on `origin/main` `02ce465b18` (profile `ci-tour`): a panic at `crates/geom-core/src/real.rs:184`, "the EXACT witness separated a placed arc's ‖q − c‖ from its sketch rim", then `tests/wall_jets.rs:128`. The hosted tour was green with main at `dd8e70198d`. (2) Four `editor-core` rows red on `origin/main` `02ce465b18` (dev profile, ε 1e-9): `corpus_finish_count::the_corpus_runs_its_pinned_count_of_finishes` (398 against a pinned 390); `docm9_range::the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot` (BOXFINISH's `a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies` names this one); `m10_bulge_interval::m10_bulge_the_bosss_split_at_the_nominal` (`m10_8_harness.rs:261`); `m10_p_lift::at_sym_a_fixed_box_is_the_build` (`m10_p_lift.rs:585`). (3) PR 7548's slow-set entry for a viewer test behind the `app` feature reds `test (app)`'s slow-set name check on every pull request (`the-slow-set-name-check-cannot-see-app-feature-tests` is the cause). (4) Hosted lint stops at its gates step on `payload-rung-sweep` (`NodeErrorClass`), so rustfmt, clippy and the doc gate do not run hosted on any pull request while that is red; TESSLANE's lanes run the three locally on the pushed tree.

- 2026-11-23 — U14 (`green/held-reds`, PR 7400's second red pass): filed three main-red P0s met on its gate: the kite moat's PR 7411 value move (not re-blessed), the slow-set name check reading PR 7548's feature-gated viewer entry dead, and the payload-rung sweep's `NodeErrorClass` under `Unsettled` since PR 7458.
- 2026-11-23 — (CIW orchestrator) For `the-slow-set-name-check-cannot-see-app-feature-tests`: a fresh instance. `f712510c37` (11:07) put the `app`-feature viewer test `app::properties_pane_tests::refused_slot_entry::the_range_buttons_grey_from_the_admission_door` in the slow set, and `check-slow-set-names` then failed every PR's `test (app)` (seen on PRs 7570 and 7501's earlier run).
- 2026-11-23 — (OFFER orchestrator) Filed `a-gated-placement-draft-row-flakes-on-an-unsettled-pick-index` (P3, E): a viewer app row seen failing intermittently on two unrelated heads ("the settled app draws from an index"). It passes when run alone.
- 2026-11-23 — (OFFER orchestrator) **Slow rows red on main**, seen by OFFER's #7576 lane running the editor-core slow set at base 52240e427 and at its head. Every row failed identically on both, on pinned receipts, so the PR does not move them:
  - `docm9_range::the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot` (held by BOXFINISH's P1 `a-witness-records-curved-premise-checks-…`);
  - `m10_9_pins_interval`: `m10_9_the_pad_at_both_rule_f_dials` and `m10_9_what_the_registry_buys`;
  - `m10_bulge_interval`: three `_split_at_the_nominal` rows;
  - `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`;
  - `m10_p_lift::at_sym_a_fixed_box_is_the_build`;
  - `m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`;
  - `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`;
  - `sym_9_retry_interval`: two rows, e.g. receipt [1390,1803,0] against pinned [1390,1801,0].
  Several are named in `main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9` and `main-red-rows-two-lanes-found-red-on-their-merge-base-…`. The rest may want an addendum there; GREEN's call.
