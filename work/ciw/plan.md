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
for the case a unit moves kernel logic, and no unit of any slate so far
has — it has never been drawn from.

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

The one question that survived that slate is **answered**. Ev ruled on
2026-09-07 that F3 stands and the `push: main` job set is not restored,
because a detector nobody reads is not a control — an argument this
program had already evidenced from the other end
(`nightly-demotions-have-never-run`: three demoted rows ran unattended
for two nights and their first reading was taken by an orchestrator
going deliberately to the jobs API). `f3-recosting-on-a-public-repo`,
`ci-draw-can-hide-a-compile-break-on-main` and
`merge-order-semantic-break-reaches-main` all close on it; the
composition-defect class is **accepted with its cost on the record**
rather than left open as work nobody is doing.

What the ruling does not answer, because the options table never priced
it, is what the two recorded instances actually cost: **attribution**,
not detection. Both breaks were found quickly; what they cost was 42 red
runs on 20 branches and two agents diagnosing one line in the same hour.
That is `inherited-red-is-not-attributed-to-its-merge`, opened with its
three unmeasured numbers named and deliberately not dispatched until
they are taken.

## The second slate landed (2026-09-10)

All seven units merged. What each landed, and the three-pass fight unit
6 took to get a selftest that could fail, is the run of entries from
2026-09-09 in `log.md`; the item files are the record. The pattern the
slate closed on is worth carrying and is stated there: **the artifact
was written against the instance rather than the property**, in four of
seven units and from both ends, and what caught every one of them was
injecting the failure and watching the row stay green — never reading.

## The third slate landed (2026-09-11)

All seven units merged — 1 (#2326), 2 (#2324), 3 (#2330), 4 (#2329),
5 (#2345), 6 (#2325), 7 (#2327). What each landed is the run of entries
from 2026-09-11 in `log.md`; the item files are the record.

**Twelve rows closed, thirty-one filed.** That is the number the next
slate has to answer, and the log's close-out entry says why it is a
receipt rather than a backlog — every one of the 31 carries a
measurement — and why it is still a problem: a program that opens 2.6
rows per row closed does not converge.

**Nineteen of the 31 are four subjects**, and that is what makes a fourth
slate tractable rather than a pile:

- **The parity checker (7 rows).** `mirror-parity-checker-growth`
  (2983 → 4908 lines in three days, +892 in one PR, claim 10's block
  ~63% of a docstring serving twelve claims), `mirror-three-copy-reader-
  preamble`, `population-layer-duplicated-across-two-checkers`,
  `three-shell-splitters-nothing-compares`,
  `mirror-readers-blind-through-bash-c` (live: an allowlisted flag OR
  variable inside a `bash -c` string passes silently — confirmed on
  `main`), `semantic-env-is-fail-open-where-pin-free-is-fail-closed`,
  `mirror-step-keys-still-discarded`.
- **Selftests that cannot see their own failure (4 rows).**
  `criterion-selftest-fixture-is-one-scalar-in-five-fields`,
  `calibrator-cpuinfo-parser-selftest-cannot-see-a-broken-parse` (the
  parity obligation is broken on two of three hand-kept copies, and both
  blind ones are in the merge gate), `calibrator-record-writes-without-
  its-selftest`, `perf-history-writers-are-guarded-three-different-ways`.
- **The provisioning surface no gate reads (4 rows).**
  `session-start-hook-is-exercised-by-nothing` — the largest of them:
  the file provisions every container and every hosted job deletes it —
  with `python-lint-row-is-locally-unverifiable-on-this-image`,
  `tool-versions-outside-the-env-block-have-no-source-of-truth`, and
  `two-anchored-pin-readers-two-homes`.
- **One argument, three to five prose homes (4 rows).**
  `prose-digits-are-records-nothing-reconciles`,
  `tier-blind-rationale-has-five-prose-spellings`,
  `eps-klint-and-shard-counts-are-prose`,
  `criterion-lane-asymmetry-argued-in-four-prose-homes`.

**The fourth slate's shape follows from that**, and is not yet ordered:
the parity checker's cluster is one unit or an `[ev]` design question
about splitting a 4900-line gate, not seven rows; the selftest cluster is
one unit over one shared fixture discipline; the provisioning cluster
needs the `[ev]` question of whether anything may gate `.claude/`; and
the prose cluster is cheap and should ride along rather than lead.
`shellcheck-is-not-run` and `doc-gate-error-sites-outside-the-gate-
population` are still waiting on the decisions named below.

**Two rows arrived from outside** and are not this program's findings:
`no-ci-row-runs-the-suite-at-a-non-default-k` and
`view-made-the-skip-mode-viewer-doc-pass-lint-inert`. Read them against
the tree before dispatching either; this slate's re-read moved two
premises and every unit corrected at least one dispatch fact.

**The review posture earned its keep and should not change.** No A/B, no
A/B protocol, style review per unit, a correctness lane where the unit
earns one — units 1, 4 and 5 at dispatch, and **unit 6 in flight**, when
what it delivered stopped being what was dispatched. Nine blockers were
found across five units and **every one came from injecting the failure
and watching the row stay green**, not from reading. Two more came from
the orchestrator re-injecting mutants a lane had reported dead without
ever applying them: **assert the mutation changed the file**, because a
no-op edit and a surviving mutant look identical.

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
- `shellcheck-is-not-run` — a unit of its own and not a residue, which
  is what its 496-finding measurement bought: two codes are 87% of the
  findings and each is one decision, and the seven errors are all false
  positives in GATES' files. What it needs first is the severity
  selection, and that is a sitting decision rather than a lane's.
- `doc-gate-error-sites-outside-the-gate-population` — the reading it
  asks for walks `scripts/gates/*.sh`, which is GATES' population. Both
  of its shapes need GATES to agree to something (widen the
  instrumentation, or take `doc-gate.sh` into its fence), so it is an
  announcement before it is a unit.
- `python-suite-axis-skips-only-two-members` — its own text names the
  number that settles it (how many code-tier runs seed only `viewer` or
  only `test-utils`) and says to take it first. Unmeasured, and the
  edit is across S-TCOST's fence.
- `inherited-red-is-not-attributed-to-its-merge` — unchanged from the
  first slate: three numbers named in the item, none taken, and the
  item forbids designing before they are.
- `guard-size-was-never-argued` — filed by unit 5 against itself, and
  the one row here that is about this program rather than the tree. The
  item said "a few lines beside its existing invocation scan"; what
  landed is 1770. The unit argued the deviation's SITING at length and
  never its SIZE, which is the whole cost of the choice. It is not a
  defect to fix; it is a question to answer before the next guard, and
  the parity-checker cluster above is the same question with a longer
  history.

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

- **`scripts/gates/*` is code-quality Track K's and `tools/*` is
  INSTR's**, and this clause said otherwise until 2026-09-11. Track K was
  claimed whole by two programs: GATES took the gates half and INSTR
  (`work/instr/program.md`, opened 2026-09-08) took `tools/*`. GATES then
  closed on 2026-09-08 (`docs/DOC-LEDGER.md` sweep 7; `work/gates/` is
  gone) and its half **reverted to code-quality**, which is `status:
  open` under `tag: (SMELL orchestrator)`. So the `tools/*` half of the
  old sentence was wrong and the gates half was right — the opposite of
  what this program's own `keep_out` was corrected FROM, and the reason
  that correction took two attempts. INSTR cedes the other direction
  explicitly: its `keep_out` names `scripts/tess_budget_cut.sh` and its
  siblings as CIW's. The `clippy-panic-gate-blind-in-macros` /
  `gated-marker-*` items are code-quality's and S-TCOST's and stay in
  `work/issues/` for them.
- **A cross-fence edit that a file INVITES by name is still announced.**
  Unit 2 anchored `scripts/tess_budget_cut.sh`'s `CUT_RE`, which reds a
  row in `tools/tess-lint/tests/cut_line_pin.rs` whose own alarm names
  the item and the two edits that close it. Taking that invitation is
  right; taking it silently is not, and the PR said so with the alarm
  quoted. Anything past the invitation — a NEW row in that table — is
  the owner's, and went to `work/instr/` as an item instead.
- S-TCOST keeps its three scripts and the CI build knobs — profile,
  cache and sharding — measured in-unit or not at all. Unit 8 cites
  S-TCOST's cache measurement; it does not fix it.
- Absorbing Track K's remaining gate rows when its live lane finishes
  is an option the proposal names and this plan does not take.

## Exit shape

The ten units above land and unit 10's ruling is answered; the walk
convention applies. Residue re-homes before the sweep.
