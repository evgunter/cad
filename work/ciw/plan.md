# CIW — hosted CI, workflows and scripts (plan)

**STATUS: OPEN (2026-09-03).** Opened from
`docs/WORK-TRACKS-2026-09.md` (CIW section); this plan supersedes that
charter. Live state is `work/ciw/log.md`'s tail and the item files
beside this plan, never this file.

Branch prefix (the #396 convention): **`ciw/`** — unit branches
`ciw/<slug>`, orchestrator branch `ciw/orchestrator`. Away-channel tag
`(CIW orchestrator)`. A/B ordinal band **CIW = 1500–1599**, claimed in
`docs/MODEL-AB-LOG.md`.

## Charter

Hosted CI that reports what it ran and runs what it reports. The
territory is retired code-quality Track J's ground plus the render
lanes and the perf emitters.

**Two of those overlap live programs, and the charter said otherwise
until 2026-09-04.** It read "files no live program owns", which is false
and was found false by unit 5's lane when `work.py territory` warned on
its own diff:

- `docs/perf-data/*` is **PERF's** (`work/perf/program.md`, open). PERF
  has *no orchestrator and no units* — it is a register that ranks cost
  centres and keeps `benches/` and `docs/perf-data/` as the measurement
  record. So it holds the record and does not do the work, which is why
  `perf-history-cannot-identify-its-host` was re-homed here at CIW's
  opening. CIW edits those READMEs; PERF owns what they say about
  ranking.
- `crates/*/tests/*` is **S-TCOST's**, and the third perf emitter lives
  at `crates/editor-core/tests/m4_pr8_latency.rs`. Checked at unit 5's
  merge: no open S-TCOST branch touches that file.

Territory warns and does not block (`work/README.md`), and neither of
these is a claim on the other program's work — CIW touches these paths
only where a unit's own item sends it. A unit whose diff widens either
overlap says so in its PR rather than letting the warning stand
unexplained.

## Review posture

**No A/B row and no A/B protocol** (Ev, 2026-09-04). Each unit is one
PR, reviewed by a subagent against `docs/prompts/reviewer-style-lane.md`
— a style review, not a dual. A unit that moves logic subtle enough to
be worth a second opinion on correctness gets one extra reviewer for
that, named in its PR with the reason; that is a judgement the
orchestrator makes per unit and not a default. The band above exists
for the case a unit moves kernel logic, and none on this slate does.

## The 2026-09-04 re-read

The slate was audited against the tree on 2026-09-04 rather than
inherited, and six items moved. The finding that moved most of them:
**`evgunter/cad` went public on 2026-09-03**, so standard-runner minutes
are free and the runner is 4 vCPU / 16 GB (was 2 / 7). Every cost
argument in `docs/CI-MINUTES-2026-08.md` — the document opens *"the
Actions allowance was being consumed faster than the work justified"* —
now has a dead premise, and several items were costed against it. That
re-costing is a unit of its own (10 below); until it reports, no figure
from that document may be quoted forward.

## The first slate landed (2026-09-05)

All ten units of the 2026-09-04 order merged, and unit 11 closed as an
avenue that does not exist: GitHub offers merge queues only to
organization-owned repositories, and Ev has ruled that `evgunter/cad`
stays personal-account-owned (`work/ciw/merge-queue-trial`, which keeps
the design, the measurements and the process failure behind it). What
each unit landed and what carries it on `main` is the table in
`log.md`'s 2026-09-06 entry; the item files are the record.

One question survives that slate and is **on Ev**:
`f3-recosting-on-a-public-repo`. Unit 8's recommendation was the merge
queue, Ev's 2026-09-04 ruling took it, and it is unavailable — so the
composition-defect class is left with one instrument (the full push job
set plus a per-SHA concurrency design pass, 48 job-min/h) whose taking
is a NEW ruling, because 2026-09-04 declined the push gate *in favour
of* the queue. Two rows park on that answer:
`ci-draw-can-hide-a-compile-break-on-main` and
`merge-order-semantic-break-reaches-main`.

## The second slate

The residue the first slate's own lanes filed, re-read against the tree
on 2026-09-06 rather than inherited — the same discipline the
2026-09-04 re-read used, and it moved four rows again. Two closed as
already discharged (`closure-tier-scope-hides-whole-tree-census-tests`
by PR 1909, `probe-interval-lane-has-no-clippy-row` by PR 1795 — the
same finding as unit 6's, filed twice by lanes that could not see each
other), and two lost half their premise to the un-sampling
(`ci-draw-...`, `detached-demo-workspaces-...`).

Units, in order:

1. **One answer to what `ci.yml` pins.**
   `local-half-restates-ci-pins-as-literals` and
   `ruff-pin-read-shares-the-first-match-shape` — the two populations
   `nightly-pin-reading-idiom-four-copies` did not reach: versions
   hand-restated as literals where nothing compares them, and a second
   first-match-at-any-indentation reader. `scripts/ci-pin.py` already
   answers the question; this is wiring the last callers to it. First
   because the class has fired on `main` once and the reader exists.
2. **The python suite on a closure run.**
   `closure-tier-skips-python-suite-on-geom-core-changes`. Measured
   live on 2026-09-06: a `crates/geom-core/src/lib.rs` change gives
   `RUN_PNCAD_PY=false` while `pncad-py` IS in `PKGS`, so the crate is
   built as a cargo target and the suite that exercises it is skipped.
   `docs/prompts/implementer-discipline.md` §2 says the suite runs on
   every code-tier run; the filter disagrees. One of the two is wrong
   and implementers read the doc.
3. **A demotion verified at the demotion.**
   `nightly-demotions-have-never-run` (whose reading is taken — all
   three rows ran green on run `33957138686`, so the convention is what
   is left) with `opt-level-selftest-runs-nowhere`, which is the same
   class one file over: a guard nothing has ever been shown to fire,
   sitting outside `scripts/gates/gate-roster.sh`'s reach. **Fence, new
   as of 2026-09-06:** `scripts/gates/*` is GATES' program now, so
   widening `gate-roster.sh`'s scope is announced to GATES and drawn
   with it — CIW's half is the workflow wiring, per GATES' own
   `keep_out`.
4. **A check that reaches the roots `--workspace` cannot see.**
   `gui-wasm-build-is-not-gated-at-all` and the surviving half of
   `detached-demo-workspaces-are-gated-only-by-a-sampled-row`. Note
   before dispatching: the wasm row's `--exclude viewer` is downstream
   of Ev's viewer-CI-posture ruling, so a fix that makes every code-tier
   run pay the eframe/wgpu graph is an `[ev]` question, not a lane's
   call — the seed-keyed treatment `clippy-all-features` uses is the
   shape that does not need one.
5. **Mirror parity past argv.** `mirror-pairs-env-divergence-unchecked`
   — claim 10 (PR 1759) compares cargo flags and reads nothing about
   the environment the paired commands run under. Direct extension of a
   row that just landed, with a live correct divergence to keep passing.
6. **The `PIPESTATUS` sweep.** `pipestatus-after-assignment-in-ci-yml`
   — one instance is fixed in PR 1725; the sweep is CIW's and the item
   is the citation. Cheap, and the failure mode is a `case` whose
   non-zero arms are all unreachable.
7. **Citations that do not resolve.**
   `gui-log-citations-do-not-resolve` and the six surviving sites of
   `rustdoc-d-warnings-breakages-outside-the-doc-gate` (its first
   bullet is corrected: `SweepStrategy::Idealized` exists behind
   `#[cfg(feature = "sweep-testing")]`, so those two sites are the
   in-half hole unit 9 accepted, not rot).

**Review posture** (Ev, 2026-09-06, restating 2026-09-04): no A/B and
no A/B protocol. One subagent style review per unit against
`docs/prompts/reviewer-style-lane.md`; a correctness reviewer only
where a unit earns one, named in its PR with the reason. On this slate
units 2 and 4 are the candidates — both change what a run executes.

## Not dispatched, and why

- `interval-only-selection-premise-restored` — a cost lever pointing at
  LESS execution, on a runner whose minutes are free. Its own text says
  the boundary with S-TCOST's cost levers should be settled before
  anyone edits the hosted `test-interval` shape. Not this slate's.
- `reach-cannot-follow-every-ascent` — the tree spells every ascent five
  ways and the resolver reads all five; no sixth spelling exists to
  measure against, and `classify` bails to `TIER=all` if the reach finds
  nothing at all. Open, not scheduled.
- `dirty-pr-gets-no-actions-run` — three measured occurrences in one
  day, across three programs, every one of them a tail-append conflict
  in a log or a ledger rather than a code conflict. The cheap half is
  one line in `docs/prompts/implementer-discipline.md`'s verification
  section (a push with no run is a conflict to merge out, not a queue to
  wait on) and rides the next unit that touches that file; the other
  half — a workflow that posts a visible "no merge ref" status so the
  absence becomes a red — is a design question nobody has costed.
  `no-ci-run-on-a-conflicting-pr` was the same finding filed twice and
  is closed into it.
- `green-row-floor-has-no-watcher` — its option 2 is a `DESIGN.md`
  revision and therefore Ev's; raised on PR #1842 and not re-asked here.
- `rustdoc-gate-private-intra-doc-links` — on its stated trigger (a
  public-only doc set, or Q9). The repository is public; nothing
  publishes a doc set yet.
- `cache-rendered-cells-on-input-hash` — parked on
  `work/tcost/rust-cache-never-restores-across-branches`; its design
  needs no revision and should be reused as-is when it unparks.

## Closed at the 2026-09-04 re-read, with the reason in each file

- `main-latently-red-at-tier-all` — neither failure is live. The pyo3
  half was fixed at `5859c8c6`; the viewer bin/lib doc collision is a
  **cargo** diagnostic, not a rustdoc one, so `-D warnings` cannot
  reach it and `scripts/doc-gate.sh --pr --scope '--workspace'` is
  green on this tree (run at closing). Its class half became unit 8.
- `rustdoc-gate-disagrees-with-workspace-doc` — answered by
  measurement: the two halves document different feature selections,
  not different verdicts. Residue folded into unit 9.
- `sccache-trial-verdict-to-read` — PR 1648 merged.
- `committed-conflict-markers-reach-main` — Ev, 2026-09-04: a committed
  marker is self-limiting (obvious, repairable later, nothing compounds
  on it), which makes it a poor subject for an absence detector.
- `python-suite-zero-test-guard-three-copies` — Ev, 2026-09-04: never
  observed, and the fix moves a developer tool's contract and a parity
  seam. The guard itself is present and correct at all three sites.

## Re-homed at the same re-read

- `bounds-tripwire-blind-to-named-alias` → `work/code-quality/`. The
  tripwire moved to `scripts/gates/bounds-allowlist.sh` (Track K's, and
  in this program's `keep_out`), and that gate's ratified header now
  argues against the ask as KNOWN GAP 3, with a fixture pinning it.
- `d107-release-profile-job-lives-in-nightly` → `work/code-quality/`.
  The whole fix is an edit to a Track P finding's disposition.
- `rust-cache-never-restores-across-branches` → `work/tcost/` (filed
  new, from PR 1648's finding (d)). Caches are a build knob and this
  program's `keep_out` gives them to S-TCOST.

## Fences

- Track K keeps `scripts/gates/*` and `tools/*`; the
  `clippy-panic-gate-blind-in-macros` / `gated-marker-*` items are K's
  and S-TCOST's and stay in `work/issues/` for them.
- S-TCOST keeps its three scripts and the CI build knobs — profile,
  cache and sharding — measured in-unit or not at all. Unit 8 cites
  S-TCOST's cache measurement; it does not fix it.
- Absorbing Track K's remaining gate rows when its live lane finishes
  is an option the proposal names and this plan does not take.

## Exit shape

The ten units above land and unit 10's ruling is answered; the walk
convention applies. Residue re-homes before the sweep.
