# S-TCOST log — test-suite cost

Narrative record; the plan is `docs/S-TCOST-PLAN.md`. Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-09-02)

Opened on Evan's direction (in-chat, 2026-09-02) by a fresh
orchestrator on a remote container. Charter and the three rulings
Evan gave on the orchestrator's questions are in the plan.

**Operational facts, recorded once:**

- **Branch prefix `tcost/`**; orchestrator branch `tcost/orchestrator`;
  the harness-designated session branch
  `claude/test-suite-opus-optimization-q8u962` carries the opening PR
  and is otherwise unused.
- **A/B ordinal band: S-TCOST = 1400–1499**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same PR. The band is
  used only by kernel-logic units; test-only units record no row.
- **Three censuses dispatched at opening** (Opus, read-only, reports
  under `~/tcost-work/`): CI red history, CI per-test timing history,
  local build profile. Their findings are summarised here when they
  land; the raw material stays lane-private.

**Decisions taken unilaterally:**

- The program name and prefix (`S-TCOST` / `tcost/`); the band
  1400–1499 as the next free band per the banding entry.
- The gate mechanism's shape (TCOST-1 spec): marker-at-the-suite,
  derived selection, fail open — argued from the same siting rule the
  nightly demotion marker and `scripts/nightly-only-selection.py` use
  (a central roster drifts; a marker at the test cannot).

## Seam: the red-history census landed (2026-09-02)

Read-only census of every hosted-CI failure since the repo opened
(report lane-private; the numbers below are the ones that decide
units). Coverage: 623 failed runs; 43 % of failed-job logs from the
first week are gone from GitHub, and `--no-fail-fast` only landed
2026-08-31, so before that a red job's failure list is a prefix of the
real one. The census is solid from 2026-07-31 on.

- **397 (test, run) reds over 162 distinct tests** — by (test, run):
  219 defect caught, 77 stale pin updated, 46 ε-band/lane sensitivity,
  40 red inherited from main (five tests × one red base tree), 11
  infra, 4 planted.
- **Only 25 % of failed runs failed on a nextest test at all**; the
  rest are clippy, rustfmt/rustdoc, discipline gates, k-lint, compile
  errors and the render lanes' `refs/pull/N/merge` race (~130 jobs,
  the single largest non-test failure cause — outside this program,
  filed as an issue).
- **Cost against return**: of the 145 tests that ever appeared in a
  `Slowest 20` table, 21 have ever gone red. Never-red expensive
  families: `cert5_r2_probes` (11 tests, ~208 cpu-s summed, the single
  most expensive thing in the suite), `m10_3_r1_probes_interval`,
  `cert5_offgrid_knot_rational`, `cert7_r1_probes`,
  `solid_contain::r1_probes`, `r2_cert6_probes`, `m8_3_rational_volume`,
  `offset_fit`, `cert5_r1_patch_probes`, `cert7_r2_probes`,
  `r2_probe_cert8`. Their `*_r1_*` siblings did go red on their own
  PRs. At file level 287 of 322 probe/review suite files (1 790 tests)
  have never produced a red.
- **Genuine wins to protect**: `verbs_pierce_r1/r2_probes` (a silent
  wrong volume), `review_r1_tier_gate_probes` (an in-band wedge that
  shipped), the de-vacuumed `spline::knots` guards, `props::quad`'s
  multiplicity ladder, `review_s2`'s collapsed-offset lever.
- **Class finding (durable home here)**: three probes dump fixtures to
  a path that does not exist on the runner (`Os { code: 2 }`) and cost
  a full red run each — `m4_pr6_eps_diff`, `review_m2_pr4::
  dump_for_cross_profile_diff`, `r2_m10_2_probes`. A content unit
  retires the dump or points it at a runner-valid path.

Reading for the units: "never red" alone retires nothing (a suite can
guard stable code), but a family whose cost is measured and whose
return is zero is where lever 4–6 work starts; the R1/R2 asymmetry
says the second reviewer's suite of a pair is the first candidate to
gate or fold.
