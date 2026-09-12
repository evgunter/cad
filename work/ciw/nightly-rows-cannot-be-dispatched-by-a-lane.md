---
id: nightly-rows-cannot-be-dispatched-by-a-lane
kind: issue
title: no agent here can workflow_dispatch, so a nightly-only row lands unverified
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

**No agent here can do that.** `POST
/repos/evgunter/cad/actions/workflows/nightly.yml/dispatches` answers
**403 Resource not accessible by integration** — for the implementer
lane's token AND for the CIW orchestrator's, tried independently. It is
the integration-token class, the same refusal
`work/ciw/apt-update-fails-on-the-runner-image-google-chrome-repo.md`
records for `rerun-failed-jobs`, and not a lane limitation something
further up the chain can route around. So the discipline's own remedy is
unavailable to everyone it binds, and every nightly-only row this program
touches reaches its first execution unattended, hours later, with nobody
reading.

The dispatch would be safe if it could be made: all four of `nightly.yml`'s
write paths guard on `inputs.ref == '' && github.ref == 'refs/heads/main'`,
so a dispatch naming a ref runs the rows and commits nothing (confirmed by
the orchestrator, 2026-09-10). Ev has the option; it has been recommended
against for this unit, because `scripts/apt-install.sh admesh` is the same
one-liner as `scripts/apt-install.sh m4`.

## Why it is not just PR 2277's problem

`nightly.yml` is CIW territory and CIW is the program that edits it. The
rule above will bind the next such unit exactly as it bound this one, and
it will fail the same way. The gap is in the token, not in the diff.

## Shapes

Either the integration token gains `actions: write` on this repo (which
also returns `rerun-failed-jobs`, and that is a separate cost/benefit — an
agent that can re-run past a flake can also re-run past a finding), or Ev
dispatches when a nightly row changes, or the discipline gains a sentence
saying what an agent does when it cannot dispatch. The second is a request
on Ev's channel; the third is META's. **What is NOT a shape is "the
orchestrator does it for the lane"** — that was tried and got the same
403.

## What PR 2277 did instead, stated so it is not mistaken for verification

The changed row is `run: scripts/apt-install.sh admesh` plus a
`timeout-minutes:` — the same one-line invocation as four rows that DID
execute on that PR's hosted run (34417694481, all four green at STEP
level). `scripts/check-ci-mirror-parity.py` reads every file under
`.github/workflows/`, `nightly.yml` included, and passed on that run, so
a structural break in the file would have fired. **None of that is the
row running.**
