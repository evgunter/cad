# Implementer discipline — standing lane obligations

**Read this in full before you start.** It is binding on every implementer
lane, alongside the unit's own spec or brief.

---

## 1. Output discipline

≤~150 lines per tool call. Chunked reads. Skeleton-first writes, then fill.
Final report ≤150 lines.

## 2. Verification

**Hosted CI is the verification of record.** Push and let it run. It runs on
hardware not shared with any other lane and its result is a durable artifact.

**It covers the full configuration matrix again, and you are expected to know
that** (2026-09-04, Ev's two authorisations). A code-tier run gates EVERY point
of {default features, `interval`} x {default eps, 1e-6, 1e-12} — twelve
`test (…)` jobs, each naming its lane, its eps row and its shard — and all five
`k-lint (gate, <row>)` feature unifications. **Nothing is sampled any more.**
The gates, the discipline and parity rows and the render lanes are unchanged and
still run on every code-tier run. **The python suite runs whenever a seed is a
crate a build of the wheel compiles** — `pncad-py`'s non-dev dependency closure,
which on this tree is every workspace member except two: `viewer`, which sits
above the wheel, and `test-utils`, which reaches the bindings along a
dev-dependency edge `maturin build` does not follow. A closure seeded only in
one of those two skips it; everything else buys it. The `change filter` job's
log prints both the seed set and `RUN_PNCAD_PY`, so a run says which way it
went. Three things follow for you:

- **A green run means green at all six lane/eps points and all five k-lint
  unifications.** That is what the job list shows: if you cannot see twelve
  test jobs and five `k-lint (gate, …)` jobs on a code-tier run, something
  narrowed it and you should find out what.
- **A commit trailer cannot configure a run, and nothing in CI reads one.**
  Between 2026-08-22 and 2026-09-04 the run drew one point per dimension and a
  `CI-Config:` trailer on the head commit was how you ASKED for the one your
  change was about. Nothing is drawn now, so that spelling was deleted on
  2026-09-04 — the flag, the workflow plumbing and the parser are gone, and a
  trailer line in a commit message is inert text. Several specs and older
  briefs still instruct it; the run is the authority, not the spec, and the fix
  is to delete the line and, if the spec really wanted one configuration
  proved, dispatch the workflow instead. **To narrow deliberately, dispatch the
  workflow** with the `lane` / `eps` / `klint` inputs — and say in the PR that
  you narrowed it, because a reader counting six test jobs where there should
  be twelve cannot tell a narrowing from a broken matrix.
- **The k-lint row is not drawn either, since 2026-09-04.** `k-lint (gate)`'s
  five feature unifications run as five jobs — `k-lint (gate, dev-default)`,
  `(release-default)`, `(release-budget)`, `(dev-budget)`, `(dev-probe)` — on
  every code-tier run, so **a green k-lint means green at all five** and a
  green over a skipped step is no longer the thing to check for there. Until
  that day one row was drawn from your head SHA and the other four did not
  execute under a single green `k-lint (gate)`; `#1756` -> `#1775` is what that
  cost, and any brief telling you to name one row on the head commit predates
  the change and names a spelling that no longer exists.

  A filename decides nothing (Ev's ruling, 2026-08-29, on #1122).
  `scripts/ci-filter.py` used to pin `LANE=interval` whenever any changed
  file's basename contained `interval`; that arm was removed because it could
  not tell a rename from a semantic edit and gated a whole branch on the wrong
  axis for its entire life after a type migration touched
  `extrude_interval.rs`. The exact pin that survived it — a change under
  `interval-transcendentals/` — is gone too, with the draw it pre-empted:
  nothing needs to pin a lane a run already gates. What is left is an advisory
  on one case, a run YOU narrowed to `lane=default` over a diff of
  interval-named files.

**When the hosted gate is not enough**, run `local-scripts/ci-local.sh`. It is
no longer the only lane that runs every lane, eps row and k-lint unification on
one tree — hosted does all three now — and what it still adds is its opt-in
`--nightly` row. Reach for it before a merge that would be expensive to get
wrong, not routinely.

**A row you DEMOTE to the nightly is verified AT the demotion.** Moving a
check out of the per-PR gate into `.github/workflows/nightly.yml` costs it the
thing that made it trustworthy: every PR ran it, so a mistake in the move
surfaced in minutes on the branch that made it. In the nightly it surfaces at
the next fire, to nobody, and **a row that fails to run at all reports the same
green as a row that ran and passed**. So before the per-PR copy is deleted,
`workflow_dispatch` the demoted job on the demoting PR's head, read the STEP
that does the work rather than the job name, and name the run id in the PR
body. Three rows demoted on 2026-09-03 first executed two nights later,
unattended, and happened to be correct; a fourth (`c5263958`) had unbalanced
quotes and never ran at all, and was caught only because a person read a log.

**The same holds one level in, for a `--selftest`.** A guard sited only in a
scheduled workflow is exercised only on a schedule, which is the same defect
with a smaller subject. A script's inputs are `scripts/*.py` or `scripts/*.sh`
— not a file class `scripts/ci-filter.py` reads as TIER=docs — so the change
set that can break its selftest is exactly the change set the per-PR gate runs
on, and that is where the row belongs. `scripts/check-ci-mirror-parity.py`'s
claim 4 has a second arm that refuses a `--selftest` mode NO WORKFLOW invokes
— a row in `local-scripts/` does not count, because every hosted job deletes
that tree. That is the floor, not this rule: it cannot tell a per-PR row from
a nightly one, and you can.

**Draft PRs do not run the gate at all.** Mark the PR ready for review when you
want it gated; undrafting triggers a full run on the same head.

**Run builds or tests locally only when it is genuinely faster for
development**: a tight edit-compile loop on one failing test, reproducing a
specific failure before you can fix it, or a case where a CI round trip would
cost more than the fix itself. That is an iteration tool, not verification — it
does not replace the CI result and it is not what you report green on. If CI
cannot run at all, say so explicitly rather than substituting a local run
silently.

When you do run locally:

- **Prefer foreground, one at a time**, reading each result before the next.
  Backgrounding a build or test is not forbidden, but treat it as risky:
  harness bugs mean the completion notification often does not arrive, so a
  backgrounded row can finish with nothing waking you.
- **Never end your turn with background work still active.** That is the case
  where a lost notification costs you everything — nobody is waiting, nothing
  wakes, and the lane stalls completely rather than failing visibly. Finish or
  abandon the background row first.
  **A hosted CI wait is the same case, not an exception** — three lanes in one
  day parked on "the CI watcher will wake me" and had to be nudged awake.
  "Push and let CI run" means CI is the verification of record, not that you
  may sleep on it: poll the run's jobs API in the foreground (an until-loop
  inside one call) until it concludes, then report in the same turn.
- When the build queue is busy, a blocking foreground wait is the correct state
  — re-issue a timed-out call rather than parking.
- **Use your own `CARGO_TARGET_DIR`, never one shared with another lane.** A
  shared target directory clobbers across git worktrees and will serve you
  another lane's binary — observed twice in one wave, once reporting a test
  count from sources that were not yours, once behind a green claim over ten
  broken assertions. Confirm a `Compiling <crate>` line appears before trusting
  any run.
- **Keep that target directory OUTSIDE the worktree** (`/home/user/<lane>-target`,
  not `.lane-target/` inside the checkout): the repo's `.gitignore` covers
  `/target` and a few named roots, not an arbitrary in-tree name, and a lane
  that `git add -A`'d its build directory pushed 114 files of incremental
  artefacts into its branch history — unfixable under merge-only rules except
  by abandoning the branch and re-landing the diff (CERT-M2, 2026-09-02). Read
  `git status` before every `git add`; never add with `-A` unattended.
- **A build is not a test.** `cargo build` cannot see a broken
  `assert!(msg.contains(…))`. A lane that rewrote text asserted anywhere and ran
  only builds has verified nothing about it.

**A test is not a test either, until you have seen it go red.** Three greens in
one program measured the harness rather than the tree. Two were mutation drills
whose mutation never reached executing code — one script failed its own
assertion BEFORE writing the file, so the suite ran on an unmutated tree; the
other landed on a doc comment 110 lines above the literal that executes.
The third was a verification chain spelled
`cargo test … | grep -E "^test result|FAILED" && …`, where **`grep` exits 0 on a
match**: a run containing `FAILED` satisfied the `&&`, and three real failures
went into a commit. All three were believed until someone asked what would have
made them red. So:

- **Earn the green with a prior red.** Break the thing on purpose, watch it go
  red, then fix it and watch it go green. That drill is an iteration tool like
  any other local run — it establishes that the check CAN red; CI still says
  whether the tree is green.
- **Read what a red would have to travel through.** A drill that stays green is
  a result about the drill until you have confirmed that the file on disk
  changed and that the change sits in code which executes. A shell chain
  reports its own exit status, not the suite's: check what each stage returns
  before you trust an `&&`, and prefer the runner's status to a match on its
  output.

**An assertion that cannot fail is the same defect written down**, and it is
common enough to look for by shape: *a predicate over things that cannot vary
at runtime, standing beside the assertions that already subsume it*. One
program found nine — `noted.len() == 60` where 60 is `72 - 12` by construction,
`constant.len() + discriminating.len() == COLUMNS.len()` over two filters that
partition by construction, a coverage predicate over a const table. When you
write an assertion, name the runtime value that would make it false; if there
is none it is documentation, and the repair is usually deletion in favour of
the rows that assert the thing BY NAME. Ask also which way its red points: one
that goes red when the corpus IMPROVES is a baseline pinned as a target (§3),
not a guard.

## 3. Baselines, demos, and the status quo

**No baseline is a target to preserve.** A lint threshold, a committed render, a
golden file, a test expectation — each exists to report what the kernel actually
does. When one moves, the only question is whether the new behaviour is
correct. "How do I get the old number back" is never the question, and a change
whose justification is that output stayed identical has not been justified at
all (`memories/output-stability-as-justification.md`).

**k-lint.** If the gate fires, do **not** change geometry to silence it. A fired
lint is distribution evidence: re-derive the baseline per the K-REPORT runbook,
or escalate to the orchestrator.

**Demos.** The tour and the wild corpus render what the kernel produces through
the public API, from an outside consumer's seat — they are evidence, not
decoration. **Write them the way a real user would**: the natural spelling of
the task through the public doors, to the greatest extent possible. A demo that
reaches past the API, hand-builds what a door should produce, or leans on a
private path stops being evidence about the library and becomes evidence about
itself — and it stops showing the friction a user would actually hit. A frame
that changed is telling you the kernel changed. Never adjust
a scene, tolerance, or camera to restore a frame. Decide whether the new output
is right: if it is wrong, fix the kernel; if it is right, re-baseline and say in
the PR what moved and why.

## 4. Comment style

Comments state the **invariant**, not the history. No retired-type archaeology,
no unit tags, no milestone or PR archaeology. An argument about how the code
used to work belongs in the PR description, which is where this repo documents
the logic of a change.

## 5. Sweeps

If your unit fixes an instance of a class, say what pattern you swept with and
**what that pattern could not match**. A sweep whose blind spot is unstated is
an unverified claim, not a negative result — and stating one is not discharging
it: *"my pattern could not match X"* is a work order to find X another way,
never a boundary. Three PRs in one day disclosed a blind spot in their own
sweep while claiming or implying it was empty. Note also that a sweep is
accurate as of your merge base, not your merge: a long-running lane owes a
re-sweep before it lands.

**Assume it is a class.** The trigger above is your own judgement that the
defect has siblings, and that judgement is where this rule misses. Before you
write the scope sentence, grep for the **shape** — not the symbol — and put
**the hit list and its disposition** in the PR description, one line per hit:
fixed, or not-this-unit and why. A pattern with no hits recorded is a claim; a
hit list is a receipt.

**A uniqueness claim is a sweep result.** *"the only", "nowhere else", "the
last one", "this is the sole caller"* — each is a claim about everywhere, and
none is writable before the sweep that could falsify it has been run. Three
reviews in one afternoon returned that same MAJOR, in different words, against
three different units.

**A sweep result is only as wide as the command actually typed.** The pattern
is one axis and the paths are another: a re-sweep of `work/ docs/` under a
claim about the tree returned 9 hits against the root's 10, and missed a site
the same merge had created. The trap generalises to any figure you report —
`cargo test` fail-fasts at the first failing binary, so a count taken without
`--no-fail-fast` is a lower bound, and one drill reported 1 where the
crate-wide truth was 5. **A figure's scope comes from the claim it is offered
for, not from the command that produced it**; where the number depends on how
you tokenized or filtered, the tokenization is part of the claim.

Scope sentences read as completeness even when the claim above them does not
share their scope. One euler-operator header asserts the universal — *"a
mutation phase announces a failed lookup rather than discarding it, at every
write"* — while its evidence is *"these modules"*; the same diff left three
silent discards in a sibling file, ten lines below two `unreachable!`
conversions it had just added.

## 6. Filing what you find outside your fence

A sweep that works turns up defects that are not yours. **They go in your
report and your PR description — not into another program's tracker
directory.**

`work/<program>/` is that program's slate. Filing there from a unit branch is
a cross-program handoff made by diff, and `work/README.md`'s one-file-one-item
rule makes two programs editing one item a merge conflict *by design*. Your own
program's slate is yours to file on; someone else's is the orchestrator's, on
the away channel.

There is a second reason, and it is the one that actually bites: **you cannot
tell whether the item already exists.** Two lanes in one session filed the same
inherited CI red into two different programs' directories, on the same day the
issue was filed and routed by a third — each lane re-derived the provenance
correctly and neither could see the others. The orchestrator could. Report it;
let the party with the whole board place it.

Reporting it is not a lesser outcome. A finding with a named file and line in a
PR body warns every reader of that PR; a duplicate item on the wrong slate
warns nobody and costs a merge.

**This says where a finding goes, never whether it gets a file**, and the two
questions read as one until they come apart. `work/README.md` is equally
binding the other way: *"Disclosing a residue is therefore not scheduling it —
give it its own file at the moment you disclose it."* Both hold at once,
because they are about different slates. **Inside your own program's fence a
disclosed residue owes a file in the same PR that discloses it**, and a
sentence in a merged PR body is not one. **Outside it, reporting IS the
filing act** — you hand it over and the orchestrator writes the file, in
`work/issues/` when no program obviously owns it. What neither document
permits is the third thing, which is what actually happens: disclosed in a PR
body, filed nowhere, by a lane that read this section as an exemption from
`work/README.md`'s. When a program's directory is deleted at close, the PR
body is not a slate and the finding is gone. (Read as a conflict by the T-2
style review, 2026-09-04; it is not one, and this paragraph exists because it
reads like one.)

## 7. Claims about the tree

Every error one program made in a day — four units' MAJORs and the
orchestrator's own six — was an inference from the SHAPE of a thing in place of
a READING of it: which function a doc list implied was gated, which spelling
produced a measured movement, which file a quoted figure lived in, how many
tests a drill reds. None needed cleverness to catch; each needed one `grep`,
one control run, or one header read. Before a claim about the tree goes into a
report, a PR body or a comment, know which of those produced it.

**Commit every edit first, then take citations from the committed tree, then
write them up.** Any other order re-creates the defect by construction: one
citation defect recurred inside the pass that was fixing it, because a gloss
edit inserted lines above the two assertions it had just cited. That ordering
is stronger than "check your citations" because it removes the window rather
than asking for vigilance inside it. And **a test name survives an edit where a
line number does not** — cite the enclosing symbol, with the number as a hint.

**Say what you measured, not what you concluded from it.** A sound measurement
with an invented attribution arrives wearing evidence. One unit's movement in a
measured figure was real and survives; what was invented was that only one of
two spellings produced it — the other was never implemented, so nothing had
been measured about it at all. An unmeasured claim is caught by asking *"did
you measure this?"*, and this one answers yes. The question that catches it is
**"what did you measure, exactly, and what else would have produced the same
number?"** Ask it of your own claims and of every one you pass on, because
**evidence-shaped claims propagate faster than bare ones**: each reader takes
the evidence as having been checked by the last. That one crossed a doc
comment, a lane report, a PR body, a reviewer brief and a summary to Ev before
a reviewer stopped it by implementing both versions.
