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

**A code-tier run gates the whole configuration matrix** (Ev, 2026-09-04):
every point of {default features, `interval`} x {default eps, 1e-6, 1e-12},
and every `k-lint (gate, <row>)` feature unification.
The gates, the discipline and parity rows and the render lanes run on every
code-tier run too. **The python suite runs whenever a seed is a crate a build
of the wheel compiles** — `pncad-py`'s non-dev dependency closure, which on this
tree is every workspace member except two: `viewer`, which sits above the wheel,
and `test-utils`, which reaches the bindings along a dev-dependency edge
`maturin build` does not follow. A closure seeded only in one of those two skips
it; everything else buys it. The `change filter` job's log prints both the seed
set and `RUN_PNCAD_PY`, so a run says which way it went. Three things follow for
you:

- **A green run means green at every lane/eps point and every k-lint
  unification — and you establish that from the `change filter` log, not by
  counting job names.** That job prints `LANE`, `EPS` and `KLINT_ROW`, which
  answers narrowed-or-not directly. **A job's NAME is CI's to change**: a lane
  that moves into a called workflow has its jobs prefixed with the caller's key,
  so a reader matching the start of a name sees a fraction of a full matrix and
  reads it as a narrowing. The roster is declared in `scripts/ci-filter.py`
  (`EPS_ROWS`, `KLINT_ROWS`) and its `--selftest` re-derives `ci.yml`'s matrix
  literals against them, so the count is held executably and does not need
  restating here.
- **Nothing in CI reads a commit trailer.** A `CI-Config:` line in a commit
  message is inert text: the flag, the workflow plumbing and the parser do not
  exist. Some specs and older briefs still instruct one — the run is the
  authority, not the spec, so delete the line, and if the spec wanted one
  configuration proved, dispatch the workflow instead. **To narrow
  deliberately, dispatch the workflow** with the `lane` / `eps` / `klint`
  inputs — and say in the PR that you narrowed it: a reader cannot tell a
  deliberate narrowing from a broken matrix except by being told.
- **A green `k-lint` means green at every row `KLINT_ROWS` declares**, each
  running as its own job, so a green sitting over a skipped step is not the
  thing to check for there. A brief telling you to name one k-lint row on your
  head commit names a spelling that does not exist; delete it.

  **A filename decides nothing** (Ev's ruling, 2026-08-29, on #1122). Nothing
  pins a lane from a path — not a basename containing `interval`, not a change
  under `interval-transcendentals/` — because nothing needs to pin a lane the
  run already gates. The one case left to think about is a run YOU narrowed to
  `lane=default` over a diff of interval-named files.

**A missing run is not a slow queue — it is what a merge conflict looks like
from the runs API.** GitHub builds a `pull_request` run from
`refs/pull/N/merge`, and for a head that conflicts with its base that ref does
not exist, so nothing is queued, nothing is red, and the checks never appear at
all. **Read the PR's `mergeable_state` before you conclude anything from a run
that has not shown up**: `dirty` means no run was ever going to be created, and
the remedy is to merge the base out and push. Not to wait, and not to push an
empty commit to re-trigger — measured, that does nothing, and merging the base
out produced a run immediately. The commit-status API cannot tell you either,
because this repo reports through check runs: `get_status` answers `pending`
with `total_count 0` on a healthy PR and an ungated one alike, so the run
itself is the only record. **A foreground poll that loops until a run concludes
will loop forever here** — the rule below, that a hosted CI wait is polled in
the foreground rather than slept on, assumes a run exists to poll. The base
that conflicts is not always `main`: a stacked PR goes runless whenever the
branch it is based on advances, which is the ordinary life of a lane rather
than an accident. It is the same failure as the demoted nightly row below — a
check that never ran reports the same green as one that ran and passed — and it
has cost lanes roughly forty minutes an occurrence, repeatedly, across several
programs. `work/ciw/dirty-pr-gets-no-actions-run` and
`work/ciw/an-unmergeable-pr-is-silently-ungated-not-visibly-red` carry the
measurements.

**When the hosted gate is not enough**, run `local-scripts/ci-local.sh`. What it
adds over hosted is its opt-in `--nightly` row. Reach for it before a merge that
would be expensive to get wrong, not routinely.

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
- **`--workspace` is not every cargo root, and the roots outside it are
  not covered uniformly.** `Cargo.toml` `exclude`s `benches`, `demos`,
  `tools` and `interval-transcendentals`, so
  `cargo clippy --workspace --all-targets` — the natural check before a
  push — compiles nothing under them. **Do not carry a count in your
  head**, this bullet's included: `scripts/doc-gate.sh --print-roots`
  derives the list, and a root has landed before with every prose count
  in the repo left saying the old number. Two of those roots,
  `demos/tour` and `demos/wild`, are ordinary consumers of the public
  API, so a signature change breaks them the way it breaks a user: two
  lanes in one hour changed a return type, re-spelled every caller
  `--workspace` could see, and learned from CI that `demos/tour/tests/`
  was still red. Those two and the `tools/*` roots are fmt+clippy'd on
  every code-tier run and by `local-scripts/ci-local.sh`, so the gate
  catches you even when your own check does not. **The other two are
  weaker than that**: `benches` gets rustfmt in the PR gate and its only
  clippy is a `nightly.yml` row with no local mirror, and
  `interval-transcendentals`' clippy runs only when the change filter
  buys that job. A green PR is not a claim about either. The cheap
  version when you are not running the local gate is
  `(cd demos/tour && cargo clippy --all-targets -- -D warnings)` and the
  same in `demos/wild`.
- **A build is not a test.** `cargo build` cannot see a broken
  `assert!(msg.contains(…))`. A lane that rewrote text asserted anywhere and ran
  only builds has verified nothing about it.

**Write assertions a bug could break.** Name the runtime value that would make
one false; where there is none — a predicate over things fixed at compile time,
or one its neighbours already subsume — it is documentation, and deleting it is
the repair.

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
**what that pattern could not match** — then try to check that blind spot,
with a second pass shaped at the gap you just named, and report that one too.
A sweep whose blind spot is unstated is an unverified claim, not a negative
result; one stated and never looked into is where the next instance tends to
be. Where a gap genuinely cannot be searched, say that and say why. Note also
that a sweep is accurate as of your merge base, not your merge: a long-running
lane owes a re-sweep before it lands.

**Assume it is a class.** The trigger above is your own judgement that the
defect has siblings, and that judgement is where this rule misses. Before you
write the scope sentence, grep for the **shape** — not the symbol — and put
**the hit list and its disposition** in the PR description, one line per hit:
fixed, or not-this-unit and why. A pattern with no hits recorded is a claim; a
hit list is a receipt.

Scope sentences read as completeness even when the claim above them does not
share their scope. One euler-operator header asserts the universal — *"a
mutation phase announces a failed lookup rather than discarding it, at every
write"* — while its evidence is *"these modules"*; the same diff left three
silent discards in a sibling file, ten lines below two `unreachable!`
conversions it had just added.

## 6. Filing what you find outside your fence

A sweep that works turns up defects that are not yours. **File them, on the
slate of the program whose ground they land on, in the same PR that found
them** — no permission, no routing through anyone. `work/README.md` settles
it: *"a finding goes straight onto the slate of the program whose ground it
lands on"* (`:117`), and *"a lane does not need the owner's permission to put
a finding where it belongs"* (`:146`). `work/issues/` is the last resort it
has always been, for a finding with no obvious owner.

`python3 scripts/work.py territory --files -` says who owns a path. Grep that
program's directory first: if a row already covers your finding, add your
evidence to it rather than opening a second — one file per item, so a
duplicate costs someone a merge conflict.

**Filing is not optional, and a PR body is not a slate.** *"Disclosing a
residue is therefore not scheduling it — give it its own file at the moment
you disclose it"* holds on every slate, not just your own. A finding left in
a PR body is gone once the owning program closes and its directory is
deleted.

Say in your report which rows you filed and where.

## 7. Citations

**Cite by name; line numbers rot.** A number may ride along beside the
name and is allowed to go stale; a bare `file.rs:NNN` is not a citation.
