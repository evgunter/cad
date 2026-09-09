---
id: nightly-rows-cannot-be-dispatched-by-a-lane
kind: issue
title: a lane token gets 403 on workflow_dispatch, so a nightly-only row lands unverified
status: open
opened: 2026-09-09
---


Disclosed by PR 2277, which changed `nightly.yml`'s `install admesh` row
and could not execute it.

## What happened

`docs/prompts/implementer-discipline.md` requires a row that lands in
`nightly.yml` to be verified AT the change: *"`workflow_dispatch` the
demoted job on the demoting PR's head, read the STEP that does the work
rather than the job name, and name the run id in the PR body."* The
precedent it cites is `c5263958` — unbalanced quotes in a demoted row
that never ran at all, caught only because a person read a log.

A lane cannot do that here. `POST
/repos/evgunter/cad/actions/workflows/nightly.yml/dispatches` answers
**403 Resource not accessible by integration** for a lane's token, the
same class of refusal
`work/ciw/apt-update-fails-on-the-runner-image-google-chrome-repo.md`
records for `rerun-failed-jobs`. So the discipline's own remedy is
unavailable to the lane the discipline binds, and every nightly-only row
this program touches reaches its first execution unattended, hours later,
with nobody reading.

## Why it is not just PR 2277's problem

`nightly.yml` is CIW territory and CIW is the program that edits it. The
rule above will bind the next such unit exactly as it bound this one, and
it will fail the same way. The gap is in the token, not in the diff.

## Shapes

Either the lane token gains `actions: write` on this repo (which also
returns `rerun-failed-jobs`, and that is a separate cost/benefit — a lane
that can re-run past a flake can also re-run past a finding), or the
orchestrator dispatches on the lane's behalf and reports the run id into
the PR, or the discipline gains a sentence saying what a lane does when
it cannot dispatch. The first two are process; the third is META's.

## What PR 2277 did instead, stated so it is not mistaken for verification

The changed row is `run: scripts/apt-install.sh admesh` plus a
`timeout-minutes:` — the same one-line invocation as four rows that DID
execute on that PR's hosted run (34417694481, all four green at STEP
level). `scripts/check-ci-mirror-parity.py` reads every file under
`.github/workflows/`, `nightly.yml` included, and passed on that run, so
a structural break in the file would have fired. **None of that is the
row running.**
