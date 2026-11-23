---
id: a-stored-data-change-runs-none-of-the-slow-rows-that-read-it
kind: issue
title: A source change runs none of the slow pins it moves through src; a named canary of about 90 recurrently red pins and censuses would catch about 80% of a month's reds for about a minute per code PR
status: open
opened: 2026-11-15
priority: P2
cost: M
refs: [6959, 6972]
---

**The mechanism.** The PR gate's slow-set step (`.github/workflows/ci.yml:457-475`)
runs `(seeds) & selected`. `selected` (`scripts/slow_set.py:157`, since PR 7359)
picks a slow row when an added line spells its name or the diff edits the
file it lives in. A slow row whose answer moves through `src/` and is not
named by the diff runs on no PR step. That covers a stored-data change (ids,
digests, receipts, exported bytes), a change to a counted ledger, and a
tree-wide source census that reads a site the diff moves. The `(seeds) &`
term also drops any row in a crate the diff only reaches as a dependent.

**What it cost, 2026-10-22 to 11-22.** 157 `main-red-*` rows were opened
across `work/`. About 140 of them are a slow row or a non-default ε row
that the breaking PR's gate did not run. The rest are rustdoc and build
breaks (11), fuzz seeds (3) and a few python rows. Two classes in that
count fit this row:

- **Source censuses and rosters: about 51 rows.** Examples:
  `certified_endpoint_census` (PRs 6478, 6479, 6486, 6808, 6811, 6862,
  7052, …), copysign (5956, 6139, 6345, 6486, 6510), `bounds_census`
  (6532, 6579), `recourse_roster`, `prose_census` (6430, 6448, 6549,
  6579) and `face_normal`'s fold guard.
- **Id, digest, ledger and receipt pins: about 25 incident groups, which
  bisect to about 40 PRs.** The `m10_sym_profile` plate ledger
  (6504, 6527, 6562, 6568, 6588, 6594, 6628, 6631, 6639), the corpus id
  pins (5613, 5637, 6856), seat4/7/8 (6820, 6832, 6856), the step-export
  sidecar (5734, 6478, 6509), `m10_9` (6594), `m10_bulge` (6504),
  `m10_6` receipts (5542), `perf12` (5745), sym11 (6988), and
  BOXFINISH's 6959, 6972 and 6982.

Each of these came from a gate that did not run the row. None of them
reached main any other way. Most were found by the nightly and fixed by a
separate PR, often after a hand bisect at about 6 min a probe
(`work/ciw/log.md`, 2026-11-10). A few were caught in review or by a
lane's local differential, which costs 45 to 70 min per tree (BOXFINISH,
2026-11-15).

**Ruled out on cost.** Running the closure's slow sets costs about 60 min
at opt-1 (`work/ciw/log.md`, 2026-11-10). Running the slow rows whose
covered files the diff touches selects a median 64% of topo's. Ev ruled
both out on PR 6731.

**The canary.** One named group in `.config/nextest.toml` holds two lists.
The slow-set step runs the group whenever the diff changes any
`crates/*/src` file, intersected with the shard's scope rather than with
the seeds:

1. **Pins with a recorded catch:** `m10_sym_profile_interval`'s ledgers,
   `m10_9_pins_interval`, `m10_p_fence`, `seat4`, `seat7` and `seat8`
   lowering, `lib_g16_corpus_name_digests`,
   `perf2_name_keying_differential`, `perf12_census_goldens`, sym11,
   `m10_bulge`, `m10_6` receipts, step-export's
   `kernel_sidecar_fields_match_live_kernel` and
   `committed_fixtures_are_byte_golden`. Each row must cost under 10 s in
   the nightly.
   - Measured on nightly 44870651337's default-ε junit (opt-2): about 35
     to 60 rows, 73 to 121 cpu-s, longest 9 s.
   - At the PR gate's opt-1 that is about ×2.5: 180 to 300 cpu-s, or 45
     to 80 s of wall time on the app shard.
   - Every kernel crate's dependent closure already builds editor-core,
     step-export and sweep, so it adds no build.
2. **Every tree-wide source census:** 34 rows, 54 cpu-s in the nightly,
   longest 11 s (`predicate_name_coverage`).
   - That is about 35 to 45 s of wall time at the PR gate.
   - geom-core's and profile's census binaries are outside a topo PR's
     scope, so either build them (one more test binary) or move those
     censuses into `test-utils`, which every closure already builds.

**Price.** The canary costs 0 on a docs-only or tracker-only PR, which is
about half of PR runs (8 of the 12 successful PR runs sampled on 2026-11-22 built nothing). On a code PR it
adds about 1 to 1.5 min to the shards, and less on the critical path
whenever the lint or demos job is the longer pole.

**Catch.** Replaying the list above, about 20 of the 25 pin groups and
every census red would have gone red on their own PR. Neither list
catches PR 6972's `coincidence_door`, `index_memo` or ε-only rows, PR
6982's shell rows, or PR 7114's `edit_strip_mouth_cups`. Those stay with
the nightly attribution (`dependents-slow-sets-skip-a-kernel-pr`).

**Keeping the list honest.** When the nightly attribution
(`dependents-slow-sets-skip-a-kernel-pr`) traces a red to a PR that did
not run the row, and the row costs under 10 s, the row joins the list.

**Also.** Update `docs/prompts/implementer-discipline.md` §2 (line 20) to
say what the step runs. That edit goes to Ev. Delete the selector-only
name lines PR 6931 and PR 6985 wrote, since PR 7359 selects by file now.

What would close it: the canary group runs on every PR that changes a
`crates/*/src` file, its measured per-PR wall time is recorded, and a
rule keeps it fed from the nightly attribution.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): - About 25 incident groups (about 40 PRs) of slow pins moved through
`src` and about 51 census reds in the month, every one a row the gate
did not run. The table is above.
- The original close-condition (the slow rows of every crate whose `src`
the diff reaches) is about 60 min per PR, which Ev ruled out.
- The reform is a named canary: pins with a recorded catch, each under
10 s, plus every source census. It runs on any `crates/*/src` change
and costs about 1 to 1.5 min of shard time on code PRs and 0 on
docs-only runs (8 of 12 sampled). It would have caught about 80%.
- Reform at `reform/a-stored-data-change-runs-none-of-the-slow-rows-that-read-it.md`.
- 2026-11-22 — **P1 → P2, not built now (Ev, in chat).** Priced against what it saves: about 30 PR-hours of latency and 40 runner-hours a month (600 code PRs a month, about 3 runs each, about 1 min each), against about 80 to 115 lane-hours saved while main reds are bisected by hand, and about 12 to 45 once the nightly attribution (`dependents-slow-sets-skip-a-kernel-pr`, P1) names the breaking merge. So the attribution goes first. If anything here is built later, it is the censuses-only half, about 40 s for 51 of the month's incidents, and only if census reds still reach main a month after the attribution lands. Ev also asked whether these reds catch defects at all or mostly make test-only work; that is being measured, and a family that only ever re-baselines is a candidate for deletion rather than for wider running.

## 2026-11-22: four more reds of the same shape, from one merge (SCALAR, PR 7267)

PR 7159 (the told-quantity type, merge `bd8b2c9bb5`) changed three things: how a refusal spells an offered tolerance (`geom-core` `told.rs`, `predicate.rs`), the `Refused`/`Takes` sentences in `profile` `path/program.rs`, and a new file holding hand-written impls. Its gate ran none of the slow rows that read those, because no added line named them. The nightly would have been the first to see four reds:

- topo `offer_rows::every_offered_tolerance_passes_just_below_it` and two `a_corner_reads_only_its_own_sectors` rows: they read the sentence's offer;
- test-utils `hand_written_impl_census` (two rows): they read the tree's impls;
- pncad-py `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`: it reads `program.rs`'s sentences;
- geom-core `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`: it reads `interval.rs`.

Since PR 7359 the step also selects the rows of a file the diff edits. None of these four is in a file that PR 7159 edited. Each reads production text, a shared helper's output, or the tree. The fix in PR 7267 had to name each row in a doc line before the gate would run it.
