---
id: d107-release-profile-job-lives-in-nightly
kind: issue
title: D107's --nocapture question is now a nightly.yml question, not a ci.yml one
status: open
opened: 2026-09-03
---


`work/code-quality/D107.md:33` says of the `kemr: 0` exposure line:

> It is **not** on the CI board: libtest captures a passing test's stdout
> and the release-profile job passes no `--nocapture`. That is a `ci.yml`
> question and `ci.yml` is Track F's F8 and Track G's G-a, so it is named
> here rather than changed.

The `corrupt input (release profile)` job moved to
`.github/workflows/nightly.yml` on 2026-09-03 (S-TCOST unit C1; the
tombstone in `.github/workflows/ci.yml` carries the argument). Two
things in that sentence are now false, and they pull in opposite
directions:

- **The file it names is wrong.** The job is nightly.yml's, so the
  `--nocapture` question belongs to that workflow, and the routing to
  Track F's F8 / Track G's G-a — which are about `ci.yml` — no longer
  follows from it.
- **The reason for leaving it alone got weaker, not stronger.** The
  argument against `--nocapture` on a per-PR row is log volume on every
  code-tier run. The row now runs once a night, where nobody is blocked
  on it and its log is read by whoever is asking this question. If the
  exposure line is worth having on a board at all, the nightly is the
  cheapest place it has ever been.

**Not this issue's to decide, and deliberately not C1's file to own.**
C1 moved a job; it did not take a position on what D107 owes, and
editing a Track P finding's disposition from a CI-posture unit would be
exactly the drive-by this repo's review split exists to prevent. What is
recorded here is that the sentence's *file reference* is stale and its
*cost argument* has changed, so whoever next reads D107 does not inherit
a routing that no longer resolves.

Owner: whoever holds D107 (Track P), or the track that owns nightly.yml's
row set.

## Re-homed to code-quality (2026-09-04, by the CIW orchestrator)

CIW took this onto its opening slate because the job that moved is a
workflow row. On reading it, the whole of the fix is an edit to
`work/code-quality/D107.md:33` — a Track P finding's own disposition —
and this item's body already says, correctly, that editing that from a
CI-posture unit is the drive-by the review split exists to prevent. So
it comes back here, where the file's owner can take it.

Re-verified 2026-09-04, both halves still hold:

- the job is `.github/workflows/nightly.yml:492`, and `ci.yml:2443`
  carries the tombstone that says where it went, so D107's *"That is a
  `ci.yml` question"* names the wrong file and its routing to Track F's
  F8 / Track G's G-a no longer follows;
- the cost argument has weakened further than this item recorded. It was
  written against per-PR log volume; the row now runs nightly, AND the
  repository went public on 2026-09-03, so the billed-minute half of
  every CI cost argument in `docs/CI-MINUTES-2026-08.md` is gone too
  (see `work/ciw/f3-recosting-on-a-public-repo`).

What CIW would owe if the answer is "yes, print it": `--nocapture` on
`nightly.yml`'s row, one line. That is CIW's to land on request. The
decision — whether D107's `kemr: 0` exposure is worth a board at all —
is Track P's and is not CIW's to take.
