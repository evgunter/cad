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

**It no longer covers the full matrix, and you are expected to know that**
(2026-08-22). A run gates ONE point of {default features, `interval`} x
{default eps, 1e-6, 1e-12}, drawn deterministically from your head SHA. The
python suite, the gates, the discipline and parity rows and the render lanes
are unchanged — every one of them still runs on every code-tier run. What is
sampled is the compile mode and the tolerance row, and three things follow for
you:

- **A green run means green at the point it drew**, which the job names carry
  (`test (eps = 1e-6, 1/2)`). It is not a claim about the other five.
- **A re-run of the same commit draws the same point.** Re-running a red leg
  will not turn it green, and if you find yourself hoping it might, that is the
  bug talking. Push a fix.
- **If the lane matters to your change, ASK for it — do not wait for the
  draw.** Put `CI-Config: lane=interval` (or `lane=default`) in your HEAD
  commit's message, or dispatch the workflow with the `lane` input. The
  request beats the draw for that dimension and leaves the others drawn, and
  the run records it as `lane:requested` / `lane:commit-trailer` in
  `CONFIG_SOURCE` so a reader can tell an asked-for point from a sampled one.
  The trailer is read off the head commit and only that one, so it lasts
  exactly one push: a later commit — a merge of main included — is sampled
  again unless it carries the trailer too.

  **Then say in the PR which lane gated**, and whether it was drawn or asked
  for. Nobody else can reconstruct that later, and a PR that does not say it
  is asking its reviewer to assume the gate saw the axis the change was about.

  A filename does not do this for you (Ev's ruling, 2026-08-29, on #1122).
  `scripts/ci-filter.py` used to pin `LANE=interval` whenever any changed
  file's basename contained `interval`; that arm is gone, because it could not
  tell a rename from a semantic edit and gated a whole branch on the wrong
  axis for its entire life after a type migration touched
  `extrude_interval.rs`. What survives is exact and narrow: a change under
  `interval-transcendentals/`, or a changed-file list the filter could not
  resolve at all, still pins interval and says so. A diff that merely touches
  `*interval*` files now DRAWS its lane and the run prints an advisory telling
  you to ask if the semantics moved. **That advisory is a reminder of this
  paragraph, not a substitute for it** — you are the only party who knows
  whether your edit changed interval behaviour or just its spelling.

**When one point of six is not enough**, run `local-scripts/ci-local.sh`: it is
now the only lane that runs every point on one tree. Reach for it before a
merge that would be expensive to get wrong, not routinely.

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
an unverified claim, not a negative result. Note also that a sweep is accurate
as of your merge base, not your merge: a long-running lane owes a re-sweep
before it lands.

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
