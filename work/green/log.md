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
