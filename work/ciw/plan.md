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

## The third slate

The 2026-09-11 re-read, against the tree rather than inherited, as the
last two re-reads were. Twenty-one rows were open at the start of it and
the premises of the seven that became units were re-checked on `main` at
`c04546d` first; the two corrections that re-check produced are in the
unit entries below.

Units, in order:

1. **The wasm row's warning-debt flip.**
   `wasm-row-warning-debt-comment-names-a-closed-item-and-a-deleted-symbol`.
   The comment at `ci.yml:2165-2174` sets its own trigger — *"this row
   becomes `-D warnings` when that item closes"* — and **the trigger has
   fired**: `work/view/viewer-items-unreferenced-at-wasm32.md` is closed
   on PR 2272, and PR 2278 deleted `ViewerApp::deliver_status` outright,
   so one of the two warnings the comment names in the present tense no
   longer exists (`WINDOW_TITLE` survives at `app.rs:129`, `cfg`-ed).
   Take the flip and rewrite the paragraph as the reason the row denies,
   or keep `check` and write *that* reason — the item's two shapes, and
   the item names three places where the obvious edit and the intended
   guard differ. First because it is the only row in this workflow that
   cannot fail on a warning and the written reason for that has expired.
2. **The cut script's anchor, and its false header.**
   `cut-regex-unanchored-admits-a-line-the-lint-refuses` and
   `cut-script-header-claims-no-cross-language-gate-exists` — one file,
   `scripts/tess_budget_cut.sh`, and the second is the header six lines
   above the first's regex. The defect is one-way and has no in-script
   recourse: a trailing-junk stamp line is read as a valid stamp by
   `CUT_RE` (refuse to re-stamp) and as harness breakage by
   `tess_lint::split_cut`, so the repair arm the script has for every
   other malformed shape cannot reach this one. **The fix reds a pin on
   purpose**: `tools/tess-lint/tests/cut_line_pin.rs`'s `TRAILING_CASE`
   row pins the disagreement and its own alarm names this item and the
   two edits that close it. `tools/*` is Track K's by this program's
   `keep_out`, so that edit is announced as invited rather than taken
   quietly.
3. **The criterion selftest's siting.**
   `criterion-selftest-nightly-only`. Re-verified: `--selftest` is
   invoked from exactly one place in the tree, `nightly.yml:1849`, so a
   PR that breaks it merges green and the break surfaces at the next
   fire, to nobody. Its comment cites `opt-level-calibrate.py` as the
   precedent for nightly-only siting and **that precedent moved** when
   the second slate's unit 3 put the calibrator's selftest in the
   per-PR gate. A row in `discipline`, its mirror in `ci-local.sh`, the
   `MIRROR_EXEMPT` entry deleted with its hosted-only reason re-stated
   where it belongs. This is a promotion, not a demotion, so it is
   verified by the gate that now carries it — no dispatch is owed.
4. **The working directory a mirrored pair runs in.**
   `mirror-pairs-context-beyond-env` clause (1). Claim 10 now compares
   cargo flags and environment; `working-directory:` is in `STEP_KEYS`
   and its value is discarded, against a local half that spells the same
   fact as `(cd … && …)`. Five sites on today's `ci.yml` (`:1743`
   `benches`, four under `interval-transcendentals`), and the pairs that
   carry one are the two cargo roots `--workspace` excludes — exactly
   the population where a wrong directory runs a different check under
   the same row name. Clause (2) closed in its own PR; clause (3) is a
   population and not this unit.
5. **The apt preamble recogniser.** `apt-preamble-bypass-is-unguarded`,
   left open by PR 2277 with its own shapes written. The class that unit
   closed can reopen one step at a time and every check in the tree
   passes it. The hazard the item names is the one to settle in the
   unit: the cheap arm widens what a `run:` block means to
   `check-ci-mirror-parity.py`, whose header calls that opacity
   deliberate.
6. **`render-hosted.sh`'s lane roster.**
   `render-hosted-knows-four-lanes-and-there-are-six` (PR 2319, filed by
   the DEMOS lane that found it and declined to half-fix it). `--lane
   mc` has been unreachable since #2284 and `gui` since #2318; the
   header states the stale count three times. The fix is the one
   `render.yml`'s own header took at #2318 — a reading of the lanes
   declared, not a count in prose.
7. **The pin-reading residue.** `session-start-hook-restates-ci-pins`,
   `seal-oracle-toolchain-read-first-match` and
   `pinned-version-named-in-present-tense-prose` — what the first
   slate's unit 1 and this program's `ci-pin.py` did not reach.
   **CIW takes the hook** (Ev's orchestrator, 2026-09-11), so
   `paths` widens to `.claude/hooks/*` in the unit that first edits it;
   the item's real content is what the hook does when a `ci-pin.py` read
   fails, since it provisions the container before any session runs and
   a refusal there ends the session rather than one CI row. The prose
   half is a tense and needs no mechanism; `slowest-tests.py`'s two
   sites are S-TCOST's and stay reported, not edited.
   `debug-only-gate-step-name-understates-its-subjects` rides along —
   one step title in `ci.yml`, and PR 2030 has landed.

**Review posture** (Ev, 2026-09-04 and 2026-09-06; restated by the
orchestrator's dispatcher, 2026-09-11): no A/B row and no A/B protocol,
on any unit of this slate including the ones that earn a correctness
lane. One subagent style review per unit against
`docs/prompts/reviewer-style-lane.md`; a correctness reviewer where the
unit earns one, named in its PR with the reason. **Units 1, 4 and 5
earn one** and are marked so at dispatch — unit 1 changes what a gating
row executes on every branch, and 4 and 5 each widen a gating claim over
a derived population in a checker whose last three units returned a
MAJOR apiece.
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
