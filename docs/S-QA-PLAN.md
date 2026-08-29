# S-QA — gates that lie (plan)

**STATUS: RATIFIED (Evan's in-chat rulings, 2026-08-29, all three
folded at Rulings below).** Opened on Evan's direction
(in-chat, 2026-08-29: "can you orchestrate its program S-QA") from the
ratified stream cut in `docs/WORK-STREAMS-2026-08.md` (§S-QA, merged
at #1200). The cut is the charter and is cited, not re-litigated.

Branch prefix (the #396 convention): **`qa/`** — unit branches
`qa/<unit>-<slug>`, orchestrator branch `qa/orchestrator`.
Away-channel tag `(S-QA orchestrator)`. A/B ordinal band
**S-QA = 800–899**, claimed in `docs/MODEL-AB-LOG.md`'s banding entry
in this same commit, per that entry's rule; implementer blocks are
named `QA-B1, QA-B2, …` (unit names occupy `QA-<n>`). Live state is
`docs/S-QA-LOG.md`'s tail, never this file.

## Charter (from the cut, verbatim in substance)

The meta-cluster: test and CI infrastructure that reports green
without looking.

- **Red now**: #1102 (eps=1e-12 census row bites the next PR to draw
  it), with #1128 (fail-fast under-reporting) as the amplifier.
- **Silent passes**: #888 (`|| true` masks grep exit 2), #1023
  (filter-skipped lint), #1038 / #746 (tess-lint stops comparing),
  #1122 (filename-substring lane pin).
- **Test integrity**: #882 + #1134 (one panic-hook race, two issues),
  #774 (wrong generator), #651 / #681 (unguarded measured claims;
  #808 stays parked on #763), #470, #466.
- **Operability**: #1051 (workflow_dispatch matrix point), #469,
  #1139.
- **SMELL track J claimed whole** (workflows / `local-scripts/` /
  `scripts/doc-gate.sh` / `scripts/gates/{gate-roster,probe-suite-census}.sh`
  / every `*.py` / root `Cargo.toml`'s `[workspace.lints]`);
  coordinates with the live K/P/W session on `scripts/gates/` and
  stays off K's contested allowlist rows.

## Ratified ground (cited, not re-litigated)

- The stream cut and its keep-outs (`docs/WORK-STREAMS-2026-08.md`
  §S-QA).
- **Configuration sampling is Evan's design**
  (`docs/CI-MINUTES-2026-08.md` §2026-08-22): one point of
  {lane} × {ε} per run, seeded from the head SHA; the premise is that
  disagreements persist in the tree so a later draw finds them, and a
  briefly red main is affordable. This program hardens the design's
  edges; it does not re-litigate the design.
- **The sampling admissibility rule** (same doc): sampling is sound
  for detectors whose subject persists in the tree, unsound for
  detectors of absence — and every future sampling entry is argued
  per-row against absence, never inherited.
- **A scheduled full run on main is DECLINED (Evan, twice:
  2026-08-22 at F3, re-affirmed against the nightly)** — the next
  PR's merge-ref gates the landed tree; the accepted residue is that
  a semantic conflict between two green PRs surfaces on the next,
  innocent PR. No unit here re-proposes it.
- **Hosted CI is the only gate** (`memories/local-battery-scope.md`);
  reviewer suites promote as-is and may be retired
  (`memories/review-and-dependency-policy.md`); every row this
  program writes obeys `memories/test-suite-cost.md` (fuzzer seed
  rules, EFFORT dial, assertion-free rows never gate).
- **A stated coverage gap is a blocker when the untested axis is the
  row's own subject** (ratified out of #1102's postmortem) — binds
  this program's own units doubly, since its subject is the gaps.

## Substrate facts the slate is shaped by (surveyed 2026-08-29)

- **The charter's "red now" is paid down to the CLASS.** #1102's
  instance was fixed at #1108 (main green 2026-08-27); #1190 — the
  second 1e-12 red, M10-DI's product-door row — was filed and closed
  the same day it surfaced. Three instances in three days of a red
  landing on an unsampled matrix point and detonating in an innocent
  PR (#1102, #1029, #1190), each costing a full A/B against main to
  attribute. What persists is the machinery that makes each instance
  expensive: fail-fast truncation (#1128), the invisible lane pin
  (#1122), and the filter-skipped/undrawn-row debt (#1023, D183).
  (#1102 itself is closable on #1108's record — done at opening.)
- **#1051 is LANDED** (2026-08-28: `workflow_dispatch` inputs +
  `CI-Config:` head-commit trailer, `CONFIG_SOURCE` printed per
  dimension; `docs/CI-MINUTES-2026-08.md` §asking-for-a-point, shown
  to fire hosted). The unit is verify-against-the-asks-and-close, not
  build.
- **#746's mechanism is largely closed by the K/P/W session's C15**
  (merged in #1187, 2026-08-29): the join gained a precondition over
  the columns the comparison does not read. Its residues (stable face
  identity D201, missing-measurement-as-lane-fact D202, `CHART_TAGS`
  pin D204) are Track K rows — the K/P/W session's, not this
  program's. #746 closes or narrows on their record.
- **#1038's 146-row instance was handed to VERBS with an audit**
  (the five-scene fold; #1023's thread carries the terms). The
  gate-side class fix — distinguishing "never seen" from "the
  baseline's cut point predates it" — remains, and `tools/tess-lint`
  is Track K's fence: QA-5 dispatches only after coordination with
  the K/P/W session (or its successor schedule state).
- **The probe-suite census's own selftest flakes red over green
  content** (`docs/CI-MINUTES-2026-08.md`, 2026-08-29 note:
  broken-pipe producers under `pipefail`; the padded-`grep` fix does
  not cover every path). `probe-suite-census.sh` is named in J's
  fence, so the owed fix is this program's — QA-1.
- **D183's path-pin ask is #1023's class in miniature**: the row that
  guards `tess-meter`'s constants is drawn 1-in-5, so the merge that
  retunes them is more likely than not ungated. Its note also records
  the debt this program inherits: `docs/K-REPORT.md:219` and `:226`
  still say "unconditional" about rows the sampling made 1-in-5, and
  `docs/CI-MINUTES-2026-08.md:335` records the correction owed.
- **PR #1138 (the smell-scan issues sweep, merged 2026-08-28 18:12
  PT) paid several charter items before this program opened**, and
  the issues are still open because commit-message references do not
  close: the #888 headline fix landed (`lib.sh`'s `gate_grep`,
  adopted across ten gate scripts — but `probe-suite-census.sh`
  still carries four `grep … || true` matcher sites and
  `gate-roster.sh` one documented as load-bearing); #774's
  generator was rewritten (radii drawn directly, no gap-rescale);
  the #882/#1134 race was closed structurally
  (`test_utils::panic_capture::caught` installs the hook once and
  switches per-thread — the downcast route was rejected because the
  bit-identity-punning gate forbids it, the exact caveat #1134
  raised); and #808 was finished. The slate below takes the
  residues and the close-outs, not the already-landed fixes.
- **Main's default-lane clippy red today is LIB's** (#1225 in
  flight) — named so nobody double-fixes it.
- The K/P/W session's bounds-allowlist rows (D102/D103/D106/D109)
  are contested by live branches and stay untouched, per the cut.

## The slate

Ordered by urgency and dependency. Each unit gets its own spec at
dispatch; difficulty logged pre-draw per the protocol. Everything
here lands red-first where a red is constructible: a gate fix is
demonstrated by planting the defect it now refuses.

- **QA-1 — gates that report green without running: the #888
  residue + the census-selftest flake (S/M); dispatchable
  pre-ratification** (fence-named defects with the fix shape on
  record — `lib.sh`'s own `gate_grep` doctrine). The headline fix
  landed in #1138; what this unit takes: (i) the remaining
  `grep … || true` matcher sites — four in `probe-suite-census.sh`,
  one in `gate-roster.sh` whose comments call it load-bearing —
  each converted to `gate_grep`, or its exception argued at the
  site against `lib.sh`'s per-stage rule; a fresh sweep of
  `scripts/gates/*.sh`, `scripts/*.sh` and workflow inline shell
  for the shape, hit list with per-hit disposition in the PR body.
  (ii) A selftest arm proving the malformed-pattern case refuses
  (the issue's own reproduction, planted). (iii) The census
  selftest's broken-pipe race (the 2026-08-29 flake): every
  producer in `probe-suite-census.sh`'s pipelines tolerates a
  closed reader (or the readers drain), replacing the per-`grep`
  padding; red-selftest-over-green-content becomes impossible or
  loud-by-name. (iv) #888 closed on the combined record.
  `bounds-allowlist.sh` is contested ground: any hit there is filed
  on the issue, not edited.
- **QA-2 — the matrix says what it did (#1128 + #1122 + #1051
  close-out) (S); dispatchable pre-ratification.** (i) #1128 by its
  own option 1 + 3: `--no-fail-fast` on both nextest run steps (a
  green run does identical work; only a red run does more), the mode
  printed in the run, and the nextest-default mechanism confirmed
  against the pinned 0.9.140 rather than trusted from the issue.
  (ii) #1122 per the Q2 ruling: the basename half of
  `_forces_interval` is removed, the `interval-transcendentals/`
  half stays, an advisory prints when `*interval*` basenames appear
  in the diff, and `docs/prompts/implementer-discipline.md`'s lane
  paragraph is rewritten to the request convention in the same PR.
  (iii) #1051: verify the landed request-a-point feature against the
  issue's three notes (record-which-point conventions, tier
  handling) and close it with the pointer. (iv) **#1204's minimum**
  (adopted at the PCURVE orchestrator's report on the opening PR —
  the sharpest member of the class: a draft PR's run rewrites every
  `RUN_*` flag to false and still reports success with
  `TIER`/`LANE`/`CARGO_SCOPE` truthful, so three greens on a
  19-kernel-file branch gated nothing): the draft skip prints
  itself in a step with no `if:`, and the `ready_for_review` escape
  is documented beside it; the issue's option 2 (a non-success
  conclusion) is assessed and reported, not taken — the F5
  draft-skip behaviour itself stays. Fence: `ci.yml`,
  `scripts/ci-filter.py`.
- **QA-3 — the debt-charging class (#1023 + D183) (M); Q1 RULED,
  sequenced AFTER QA-2 lands (same files: `ci-filter.py`,
  `ci.yml`).** The ruled shape: a change under `tools/` path-pins
  the k-lint row that compiles it (`demos/` explicitly excluded per
  the ruling), so path-correlated breakage cannot land on an
  undrawn row; plus the three owed sentence corrections
  (`docs/K-REPORT.md:219`, `:226`, the `KLINT_ROWS` header's third
  instance). The residue — breakage uncorrelated with its paths —
  stays covered by the ratified persistence argument and is stated
  at the site, not silently accepted. #1023 closes on this plus
  QA-2's visibility work, with the fail-fast shard-erasure half
  closed by QA-2(i).
- **QA-4 — landed-fix close-outs (#882 + #1134, #774, the #808
  finish) (S).** Verification against each issue's own asks, then
  the close with the record: the panic-hook fix under the issues'
  200-run reproduction loop (structural, not rarer — the install-
  once + per-thread-switch design is the claim to falsify); #774's
  residual asks (docstring states what the strategy guarantees; the
  minimal counterexample pinned as an explicit fixture per
  `test-suite-cost`; the `.proptest-regressions` convention decided
  and recorded); #808 against its checklist. Any ask #1138 did not
  pay is small and lands here; a pure-verification outcome closes
  the issues with no PR beyond the record.
- **QA-5 — the comparison gate that stops comparing (#1038,
  gate side) (M); after K/P/W coordination.** Under the Q3 ruling:
  recommendation is the issue's option 2 (a fresh-sweep scene absent
  from the baseline FAILS the gate, so adding a tour scene forces
  the re-cut in the same PR), with the baseline's cut commit
  recorded in the file so "never seen" and "outgrown" stop being
  indistinguishable. Consumes VERBS' five-scene audit (their fold
  restores coverage; this unit makes the decay impossible).
- **QA-6 — the measured-claim sweep, J-fence legs (#681) (M/L;
  possibly one PR per leg).** The legs on this program's ground:
  manifests (`--marker '#'`), `.github/workflows/` + `scripts/`,
  `tools/` lint-threshold provenance, Python (docstring pass),
  `interval-transcendentals/`'s one real row. Each leg names its
  instrument variant and its own blind spots; every claim classified
  into Q6's four buckets at the claim site. The `crates/*/tests` leg
  is Track W-adjacent and waits for K/P/W's state; `docs/` prose
  legs deferred with the reason (highest over-match, lowest yield).
  #651 stays open as the class home; #808 stays parked on #763.
- **QA-7 — CI reports test cost (#469) (S/M).** Slowest-N into
  `$GITHUB_STEP_SUMMARY`; the this-PR-added diff via the
  `interval-only-selection.py` sibling. Deliberately not a gate, per
  the issue; the two measurement traps (per-leg incomparability,
  head-not-mean) are spec text.
- **QA-8 — what the rustdoc gate cannot see (D180 + Track R's D301,
  landing together) + the false copies (D181, D182) (S/M).** The
  gate gains a `not(feature)` arm (or a recorded refusal naming the
  blind spot), `mesh/src/budget.rs`'s three link errors are fixed in
  the same PR so closing the instance does not hide the class; the
  two-of-three false copies of "what the budget gate reads"
  (`ci.yml:2749`, `ci-local.sh:846`) and the `.py` third copy (D182)
  are corrected against C15's actual join. Rows land per §D's
  conventions (delete the row in the landing PR).
- **QA-9 — the status line that invites wrong action (#1139)
  (XS).** `with-build-slot.sh`'s annotation becomes
  reader-relative, per the issue's own suggestion. Rides whichever
  early lane touches `local-scripts/` or dispatches solo as a
  filler.

**Parked, with reasons**: #470 (its own "why not now" — the prize is
~90 cpu-s until the opt-level story settles; revisit when
`docs/perf-data/opt-level/` has runner samples); #466 (a "consider",
and its own text schedules the uniform-harness PR first); #808 (on
#763, per the cut).

Cross-program interfaces, named so "CI" does not become a bucket:
`scripts/gates/` beyond the two J-named scripts, `tools/` and
`docs/K-REPORT.md` are Track K's (the live K/P/W session);
`crates/test-utils` and `crates/*/tests` mechanisms are Track W's
(same session) — QA-4/QA-5/QA-7's tests leg take exactly their named
issues and nothing adjacent; the bounds-allowlist rows stay
contested; the #1038 five-scene audit is VERBS'; main's clippy red
is LIB's #1225; k-lint distribution semantics (what a fired lint
means) are the K-telemetry ground (`docs/K-REPORT.md`), not this
program's to reinterpret.

## Rulings (Evan, in-chat, 2026-08-29)

1. **Q1 — RULED: the k-lint path pin lands at `tools/` scope.**
   A change under `tools/` forces the k-lint row that compiles it
   (D183's mechanism, the same substitution `ci-filter.py` makes
   for the interval lane); measured over the prior 14 days of main
   that fires on ~7% of code-shaped merges (`tools/tess-meter`
   alone is ~3%). `demos/` is explicitly NOT in the pin (~29% —
   demos churn would make the drawn row deterministic on a third
   of runs and erode the sampling, while the demos failure shape
   that actually bit is caught by any row that runs). No
   unconditional row, no scheduled full run (declined twice). The
   residue — path-uncorrelated breakage waiting for a later draw —
   stays accepted per the sampling design's own argument. QA-3
   executes.
2. **Q2 — RULED: the basename-substring lane pin is DROPPED in
   favour of the manual-request convention.** A change to interval
   semantics requests its lane (`CI-Config: lane=interval`, the
   landed request-a-point door); the filter prints an ADVISORY when
   `*interval*` basenames appear in the diff (fired ~10% of recent
   code merges as a pin, much of it rename noise per #1122's
   measurement). **The load-bearing half of the ruling is the
   doc, not the advisory (Evan)**: the convention lands in
   `docs/prompts/implementer-discipline.md` — the file every lane
   reads — in the same PR that drops the pin, replacing its current
   path-rule paragraph. The `interval-transcendentals/` pin stays
   (0.7% fire rate, exact by construction; the crate's own guard
   jobs complement rather than replace the lane draw). `LANE=both`
   is moot — no substitution remains to widen. QA-2's #1122
   deliverable is re-cut to this ruling.
3. **Q3 — RULED: #1038's option 2.** An uncovered fresh-sweep
   scene FAILS the gate, so corpus growth forces the baseline
   re-cut in the growing PR; the baseline's cut commit is recorded
   in the file. At current churn that fires on ~2-3 scene-adding
   PRs a week, and each firing's cost is folding the PR's own rows
   — what well-behaved scene PRs already do voluntarily. QA-5
   remains gated only on the K/P/W fence coordination.

## Process

Standard, v6: substrate → binding spec → one implementer + the
cross-model dual review + union fix pass; implementer arms drawn per
the current block rule in `docs/MODEL-AB-LOG.md` (read on main at
each dispatch — that document owns every live number); ordinals
claimed on main at review dispatch from band 800–899;
record-at-merge with per-phase tokens/wall-clock; blinding
discipline verbatim (no `Co-Authored-By` in lane commits; no
arm-naming surface reviewers can read). Hosted CI is the only gate;
every new row ε-three-outcome honest; reviewer suites promote as-is
and may be retired per policy. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path; reviewers get
explicit claims to falsify plus `docs/prompts/reviewer-style-lane.md`.

One program-specific addition, because this program edits the
instruments themselves: **a unit that changes `ci.yml`,
`ci-filter.py` or a gate script must show the changed instrument
firing on a planted defect AND passing on clean input, hosted where
the mechanism is hosted-only** (the request-a-point feature's own PR
is the precedent) — a gate fix verified only by reading the script
is the defect class this program exists to close. Mirror parity
(`check-ci-mirror-parity.py`) binds every `ci.yml` edit.

**This orchestrator runs in a remote container** (the S-CERT/M10/GUI
precedent): no persistent `~/.local/share/cad-work`, no script
monitors (PR watching via MCP subscriptions + scheduled self
check-ins; away-channel etiquette by hand under the `(S-QA
orchestrator)` tag), GitHub through MCP. Disk (~29 G free) is the
binding constraint: lanes are worktrees sharing one object store,
each with its own `CARGO_TARGET_DIR`, ≤ ~2 concurrent lane targets,
review targets reclaimed the moment the report is in hand. The
CONFLICTING-means-silent-CI and push-early rules bind unchanged. The
clone arrived shallow and was unshallowed with a blob filter at
opening.

## Exit shape (proposed)

No gate named by the charter can report green without looking: the
allowlist gates fail loud when their matcher fails; the census
selftest cannot red over green content; a red run reports its
failure surface and says which matrix point and mode it ran; a
pinned lane says so and adds coverage rather than substituting it;
the k-lint rows cannot be dodged by the changes they compile;
tess-lint refuses corpus growth it is not comparing; the
`test-utils` harness cannot flake by construction; the ε-battery's
generators satisfy their own premises; the J-fence measured claims
are guarded, scheduled, or excused at the claim site; CI prints what
each PR costs the suite; the rustdoc gate's `not(feature)` blind
spot is closed or named at the gate. Track J is empty in §D. Every
unit merged on its own green hosted head; the walk convention
applies at exit.
