# S-QA log — gates that lie

Narrative record; the plan is `docs/S-QA-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-QA. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-29)

Opened on Evan's direction (in-chat: "can you orchestrate its program
S-QA", pointing at the merged work-streams cut), by a fresh
orchestrator on a remote container. The plan is a DRAFT design
conversation for its **Rulings sought** section; QA-1 and QA-2 are
dispatchable pre-ratification as charter-named defect fixes whose
shapes are the issues' own recommendations (recorded below as a
unilateral decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `qa/`** — unit branches
  `qa/<unit>-<slug>`, orchestrator branch `qa/orchestrator` (the
  harness-designated session branch
  `claude/program-sqa-orchestration-sllggq` carries the opening PR
  and is otherwise unused, per the S-CERT precedent).
- **A/B ordinal band: S-QA = 800–899**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same commit, per
  that entry's rule (S-CERT holds 700–799; 800 was the next free
  band at claim time). Implementer blocks are named `QA-B1, QA-B2, …`
  (`QA-<n>` are unit names).
- **This session runs in a remote container** (the S-CERT/M10/GUI
  precedent): no persistent `~/.local/share/cad-work`, no script
  monitors (PR watching via MCP subscriptions + scheduled self
  check-ins; away-channel etiquette by hand under the `(S-QA
  orchestrator)` tag), GitHub through MCP rather than `gh`. Disk
  ~29 G free is the binding constraint: lanes are worktrees sharing
  one object store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent
  lane targets, review targets reclaimed at report time. The clone
  arrived SHALLOW; unshallowed with a blob filter at opening.

**Sweep at opening** (beyond the charter, what the slate is grounded
in): the charter's "red now" is already paid down to the class —
#1102's instance fixed at #1108 (2026-08-27), #1190 filed and closed
same-day (2026-08-29), three innocent-PR detonations in three days
total; #1051's ask LANDED 2026-08-28 (request-a-point:
workflow_dispatch + `CI-Config:` trailer + `CONFIG_SOURCE`), so that
unit is verify-and-close; #746's mechanism largely closed by the
K/P/W session's C15 (#1187, merged 2026-08-29), residues on Track K;
#1038's 146-row instance handed to VERBS with an audit, the
gate-side class fix remaining; the probe-suite-census selftest
flaked red over green content on 2026-08-29 (broken-pipe producers —
J-fence, ours); main's default-lane clippy red is LIB's #1225;
the K/P/W session is live on tracks K/P/W and its
bounds-allowlist rows are contested by live branches — untouched
here.

**Post-opening substrate correction (same day, pre-dispatch):** PR
#1138 (the smell-scan issues sweep, merged 2026-08-28 18:12 PT —
after the cut's survey inputs) had already paid the #888 headline
(`gate_grep`), #774's generator, the #882/#1134 race
(`panic_capture`, install-once) and the #808 finish, with all four
issues still open because commit references do not close. The plan's
substrate section and slate were amended before any dispatch: QA-1
narrowed to the residue (the census script's four `|| true` matcher
sites, `gate-roster.sh`'s one, the selftest arm, the broken-pipe
flake), QA-4 became the landed-fix close-out unit, the old QA-5
absorbed into it, later units renumbered up by one. The lesson is
the program's own charter one level up: a cut surveyed at T is
stale by T+1 in this repo, so every spec re-verifies its issue's
premise against main at dispatch.

**Unilateral decisions at opening** (per the orchestration memory's
log rule):

1. QA-1 and QA-2 dispatch pre-ratification. Ground: every item is
   charter-named; the fix shapes are the issues' own recommendations
   (#888's exit-status check; #1128's option 1+3, whose cost
   argument — a green run does identical work — is in the issue;
   #1122's option 3, the say-it-pinned half only, with option 2 held
   for Q2's ruling; #1051 is verification of a landed feature). The
   one Evan-flavored piece in QA-2 (LANE=both on pin) is excluded
   from the dispatch until ruled.
2. The rulings split: Q1 and Q2 amend what a PR run gates — that has
   been Evan's call at every precedent (sampling, k-lint sampling,
   F3, the declined scheduled run) — so both wait even though a
   no-spend reading exists; Q3 changes what a tour-scene PR owes at
   merge, which is other programs' workflow. Everything else in the
   slate is faithful elaboration of ratified ground and proceeds.
3. #1102 closed at opening on #1108's record (instance fixed, main
   green since 2026-08-27; the class is this program's charter and
   is tracked by QA-2/QA-3, not by an open instance issue).
4. The opening PR rides the harness session branch rather than
   `qa/orchestrator`, to respect the harness branch designation for
   this session's own pushes; unit lanes use `qa/` per the cut.

## Seam: first dispatches (2026-08-29)

QA-1 and QA-2 dispatched per the plan's pre-ratification decision:
specs `docs/QA-1-SPEC.md` / `docs/QA-2-SPEC.md` on
`qa/orchestrator` (6829ca82), lanes on `qa/1-silent-green` and
`qa/2-matrix-speaks`, block **QA-B1 slots 1 and 2** (difficulty
pre-logged S/M and S; the draw byte and arms are recorded at merge
per the blinding rule). Both lanes are shell/python-shaped — no
kernel builds expected, which is why two run concurrently within
the disk budget. The opening PR (#1228) carries the Q1–Q3 rulings
conversation; QA-3 waits on Q1, QA-2's `LANE=both` half on Q2,
QA-5 on K/P/W coordination. #1102 closed at opening per unilateral
decision 3.

## Seam: #1204 adopted into QA-2 (2026-08-29, dispatch+1)

The PCURVE orchestrator, on the opening PR: #1204 (a draft PR's run
reports success with every `RUN_*` flag rewritten false and the
classification lines left truthful — three greens on a
19-kernel-file branch gated nothing) is the class's sharpest member
and read as unowned. Adopted: it is J-fence (`ci.yml` +
`ci-filter.py`) and the same say-what-you-did shape QA-2 is mid-way
through, so its minimum (option 1, the skip prints itself, plus the
`ready_for_review` escape documented) went into QA-2's spec
(`f3455f15` on `qa/orchestrator`) and the running lane was messaged
with the delta; option 2 (a non-success conclusion) is assessed and
reported by the lane, not taken. Unilateral ground: no new spend,
the issue's own author calls option 1 the minimum regardless, and
the fence is already this program's.

## Seam: all three rulings in; plan RATIFIED (2026-08-29)

Evan, in-chat. **Q1 RULED**: the k-lint path pin at `tools/` scope
(~7% of code merges, measured over 14 days; `demos/` at ~29%
explicitly excluded — determinism there would erode the sampling).
**Q2 RULED**: the basename-substring lane pin drops in favour of the
manual-request convention (`CI-Config: lane=interval`), advisory
print kept ("sure why not"), with Evan's emphasis that the
load-bearing half is the DOC — the convention goes into
`docs/prompts/implementer-discipline.md`, the file every lane
reads, in the same PR that drops the pin. The
`interval-transcendentals/` pin stays (0.7%, exact; the crate's own
guard jobs complement it). `LANE=both` moot. **Q3 RULED**: #1038's
option 2 (uncovered scene fails the gate; ~2-3 firings/week at
current churn, each just the growing PR folding its own rows).

Consequences: the plan is RATIFIED; the opening PR merges; QA-2's
#1122 deliverable re-cut to the Q2 shape (spec amended, lane
messaged mid-flight); QA-3 unblocked but sequenced AFTER QA-2 lands
— both edit `ci-filter.py`/`ci.yml` and two lanes on one file is a
conflict by construction. QA-5's shape is settled; its dispatch
still waits on the K/P/W fence coordination.

## QA-1 merged (2026-08-30)

The issue-888 residue and the census broken-pipe class, landed on a
green hosted head (run 33275703002, all 21 jobs) after the v6 dual
and a nine-for-nine fix pass. The dual's headline is the program's
own charter enacted on itself: the unit's `gate-roster.sh` conversion
could print a matcher-death diagnosis and still exit 0 — the one gate
never calling `gate_ok` — found by a reviewer's shim, not by the
author or the selftest; and the "too expensive" deterministic
regression arm was built independently by both reviewers in ~3 s.
Both are fixed and armed. Issue 888 closes on the combined record;
`check_step.sh:92` and the `find`-in-process-substitution class have
durable homes on that issue. Recorded for a future unit: the census
script is now 1,100+ lines carrying five modes in one `gate()` (a
reviewer's Q8 finding — a split candidate, not this unit's).

Operational note at merge: the first gating run of the merged head
(33332490436) went red with both default-eps test shards "failing"
in one second each, runner_id 0, no logs — the Actions budget had
run out, so the jobs never executed. A ghost failure wearing a red
run's clothes is this program's charter one level down the stack;
re-rolled with this commit once the budget was restored, pinning
the same point by trailer so the ghost-failed configuration is the
one re-proven.

Process corrections this unit earned: lane branches now carry ONLY
their own spec (QA-2's spec rode this PR from the shared dispatch
commit — harmless here, sloppy in general); and the fable usage
limit can kill lanes mid-flight — resumed with zero loss because the
lane had pushed first, which is the push-early rule doing exactly
what it is for.

## QA-2 merged (2026-08-30)

The matrix now says what it did: a red run reports its whole failure
surface and names its mode (nextest's fail-fast default measured, not
assumed, on the pinned version); a pinned lane says so in
`CONFIG_SOURCE` and the basename pin itself is gone per the Q2 ruling
— the request convention lives in the discipline doc, the
`interval-transcendentals/` pin and the fail-closed arm stay; a draft
run prints that it gated nothing, with the `ready_for_review` escape
beside it; and the request-a-point door is verified end-to-end, so
issue 1051 closes with 1128, 1122 and 1204. The dual found no MAJORs
and one §D-rule-5 instance (the skip step asserting a false cause —
fixed by deriving one cause in producer order, all eight combinations
executed). The fix pass beat its instructions once: instead of gating
four hand-synced advisory spellings it made the filter compose its
notices into a file ci.yml relays, so the copies ceased to exist.
Residues with homes: issue 1295 (the parity checker compares checks,
never flags), and the not-hosted-verified tripwire disclosed at the
site. The Actions budget outage that ghost-failed three branches'
runs mid-unit is recorded in both units' PRs — a red run whose jobs
never executed is this program's charter one layer down, and worth a
future thought about whether the filter can notice a zero-step
"failure".

Next: QA-3 (the tools-scope k-lint path pin, Q1's ruling) into the
now-freed `ci-filter.py`/`ci.yml`; QA-4 close-out verification; QA-5
awaits the K/P/W coordination.

## QA-4 closed out (2026-08-30)

The landed-fix verification unit (block QA-B2 slot 1; verification-
shaped, so it produced no A/B row — the one residue it landed is the
nine doc-comment lines on this PR recording the ratified
proptest-regressions decision: a written-out fixture over a seed,
because a `cc` seed re-derives its input only through the strategy
that drew it, and that strategy is what the rewrite replaced). All
three verifications PAID: the panic-hook race structurally
unrepresentable and 0-in-200 empirically (plus a load-shaped leg,
against ~15-in-200 pre-fix); the generator's docstring and pinned
counterexample in place; the issue-808 finish complete with the
guard the issue said was missing. Issues 882, 1134, 774 and 808
close on this record. Class noted for a future sweep, not acted on:
three test files still do per-call panic-hook take/set swaps
(topo/src/review_m1_pr2/release_corruption.rs:290,
mesh/tests/profile_overrides.rs:140,
sweep/tests/review_d2_adv_probes.rs:435) — each single-binary-safe
today, all issue-882's shape; and `caught`'s `unwrap_or_default()`
still folds a lost message to the empty string, unreachable now but
the reintroduction path if a second set_hook caller ever lands in
that binary.

## QA-3 merged (2026-08-30); block QA-B1 complete

Evan's Q1 ruling is live: a `tools/` change pins the k-lint row that
runs that crate's own suite, announced in `CONFIG_SOURCE` and the
notices relay, with the draw untouched everywhere else — and the pin
caught D183's failure shape on its own PR three times (each pinned
head would have drawn a row that builds the changed crate without
running an assertion about it). Issue 1023 closes on this plus
QA-2's visibility work. The dual's headline is a lesson this
program should keep: the filter's BEHAVIOR survived fourteen
mutations across two arms, and both MAJORs were in the WRITTEN
record — a derivation that said "compiles" and meant "runs the
suite", and a debt-resolution pointer that vouched for a still-false
comment. In a program whose deliverables are instruments, the record
is part of the instrument, and it now has its own guard
(`_selftest_klint_workflow` parses ci.yml and reds when the mapping,
the row roster, or the fallback's premise drifts). Also earned here:
a debt whose locator is a phrase-grep will point at true sentences
and miss false ones — re-derive the claim set before trusting the
citations (the unit did, found one cited line TRUE and two uncited
lines false, and then committed the same class itself one site over
— caught by the dual). Track Q's declared count (18) disagrees with
its table (16), pre-existing, verified at merge base by both
reviewers independently — flagged for whoever owns the schedule's
bookkeeping next.

Block QA-B1 is complete: three units, three duals, ordinals 800-802,
samples 53/54/56.

## QA-9 merged (2026-08-30)

The slot status line now states only what it can verify: which slots
THIS request polls (one width-home consulted by loop and status
alike), which it just tried, what a dead record over a still-busy
slot actually means (the inherited-fd leak, named), and when the
blocker has no record at all. The dual earned its keep in miniature:
both arms independently reproduced the fix's own new line predicting
"can take it" about a slot the request had just failed to take — the
issue-1139 class re-minted inside its fix, §D rule 5 for the fourth
S-QA unit out of four reviewed. Every prediction is gone; the
transcripts in the PR body are the verification of record, since CI
deletes local-scripts by design. Issue 1139 closes on this record.
Two operational notes: a reviewer probe misfired into the REAL lock
dir once (one stale holder file, orchestrator-cleaned — synthetic
lock dirs are mandatory and both briefs said so; the classifier
blocked the reviewer's own cleanup, which is the right failure), and
the fix pass ran one pattern-match `pkill -x sleep` during cleanup —
the kill-by-recorded-PIDs rule's exact violation, disclosed, with
possible clipped sleeps in sibling lanes' poll loops (recoverable;
none reported damage).
