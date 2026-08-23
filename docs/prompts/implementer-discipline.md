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
- **A change to interval code always draws the interval lane** — the rule is
  path-shaped (`interval` in the basename, or anything under
  `interval-transcendentals/`) and lives in `scripts/ci-filter.py`. A change to
  interval-gated code inside an ordinarily-named file is NOT matched and falls
  back to the draw, so if your unit is one of those and the lane matters to it,
  say so in the PR rather than assuming the gate saw it.

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
- When the build queue is busy, a blocking foreground wait is the correct state
  — re-issue a timed-out call rather than parking.
- **Use your own `CARGO_TARGET_DIR`, never one shared with another lane.** A
  shared target directory clobbers across git worktrees and will serve you
  another lane's binary — observed twice in one wave, once reporting a test
  count from sources that were not yours, once behind a green claim over ten
  broken assertions. Confirm a `Compiling <crate>` line appears before trusting
  any run.
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
