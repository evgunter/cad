# Implementer discipline — standing lane obligations

**Read this in full before you start.** It is binding on every implementer
lane, alongside the unit's own spec or brief.

**If you ran anything locally, your report must say what and on which
`CARGO_TARGET_DIR`.** CI speaks for itself on the PR; a local run does not.

---

## 1. Output discipline

≤~150 lines per tool call. Chunked reads. Skeleton-first writes, then fill.
Final report ≤150 lines.

## 2. Verification

**Hosted CI is the verification of record.** Push and let it run. It covers the
full matrix — every eps and feature combination, the python suite, the gates,
the render lanes — on hardware not shared with any other lane, and its result is
a durable artifact.

**Run builds or tests locally only when it is genuinely faster for
development**: a tight edit-compile loop on one failing test, reproducing a
specific failure before you can fix it, or a case where a CI round trip would
cost more than the fix itself. That is an iteration tool, not verification — it
does not replace the CI result and it is not what you report green on. If CI
cannot run at all, say so explicitly rather than substituting a local run
silently.

When you do run locally:

- **Foreground, one at a time**, reading each result before the next. Never arm
  waiters, monitors, or background chains for your own builds and tests. When
  the build queue is busy, a blocking foreground wait is the correct state —
  re-issue a timed-out call rather than parking.
- **Use your own `CARGO_TARGET_DIR`, never one shared with another lane.** A
  shared target directory clobbers across git worktrees and will serve you
  another lane's binary — observed twice in one wave, once reporting a test
  count from sources that were not yours, once behind a green claim over ten
  broken assertions. Confirm a `Compiling <crate>` line appears before trusting
  any run. Delete your target dir when the lane is done; they run to several GB.
- **A build is not a test.** `cargo build` cannot see a broken
  `assert!(msg.contains(…))`. A lane that rewrote text asserted anywhere and ran
  only builds has verified nothing about it.

## 3. k-lint

If the k-lint gate fires, do **not** change geometry to silence it. A fired lint
is distribution evidence: re-derive the baseline per the K-REPORT runbook, or
escalate to the orchestrator.

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
