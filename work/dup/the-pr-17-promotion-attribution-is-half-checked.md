---
id: the-pr-17-promotion-attribution-is-half-checked
kind: issue
title: The PR 17 promotion attribution survives in seven headers and only half of it is what Ev asked for
status: open
opened: 2026-09-19
---



## Finding

- **Where**: seven `//!` headers under `crates/topo/src/` end with
  *"Promoted per Ev's request (PR #17 thread)."* —
  `review_m1_pr1.rs` and all six `review_m1_pr2/*.rs`. The same
  attribution, as *"(Evan, PR #17 thread)"*, was written into
  `memories/review-and-dependency-policy.md` by the same commit that
  created the headers: `e9eeace50`, 2026-07-16, *"tests: promote M1 PR 1
  + PR 2 adversarial-review suites into CI"*.
- **What the commit actually attributed to Ev.** The memory clause it
  added reads: *"**Reviewer suites get promoted into CI.** After each
  PR's fix pass, the reviewer's consumer test suite is promoted into the
  repo as `crates/topo/tests/review_m1_prN*.rs` (Evan, PR #17 thread).
  The suites are independent derivations — that independence is their
  regression value, so do not "simplify" them to match shipped
  fixtures…"* The citation is attached to the **promotion**; the
  no-simplify sentence follows it in the same paragraph and carries no
  citation of its own. The headers then copied the attribution onto the
  whole paragraph, so seven files read as though Ev had asked for the
  exemption.
- **What the thread says.** PR #17's comments are all on the `evgunter`
  account, but their contents separate into two kinds. The short ones
  are questions: *"awesome, lgtm! btw, do reveiwer artifacts feed
  acceptance tests? we may want to keep them as an auxiliary source of
  tests even if we don't run them in ci"*, *"how's it going on pr 3?"*,
  *"are there other old reviewer artifacts we should pull into version
  control, or are they gone?"*. The long ones are agent status reports
  posted from the same account, and the **convention — including the
  phrase "independent derivations — do not simplify to match shipped
  fixtures, the independence is the value" — appears first in one of
  those**, as a *recommendation*, answering the first question. Nothing
  in Ev's own words contains it; his question proposed keeping the
  artifacts *"even if we don't run them in ci"*, and the reply went
  further in both directions (into CI, and with an exemption).
- **Importance**: medium. The exemption itself is already retired —
  `memories/review-and-dependency-policy.md` withdrew that reading on
  2026-09-04 and this row's carrier PR removed the sentence from twenty
  headers, so nothing now depends on the attribution being right. What
  is left is a provenance line that reads stronger than its evidence,
  in the place a lane looks when it wants to know whether Ev ratified
  something.
- **Confidence**: sure about the commit, the memory clause and the
  thread's contents. **Not sure who typed which comment**, which is
  exactly what cannot be settled from inside one account.
- **Raised by**: the S-DUP lane for the withdrawn-no-simplify unit,
  2026-09-19.

## Why this is filed rather than fixed

The promotion half of the claim is true — Ev did ask for the artifacts
to be kept, and PR #18 is the promotion he was told about. Deleting the
attribution would lose that; sharpening it to say which half Ev asked
for means **writing a new provenance claim on an authorship question
this checkout cannot answer**. Either move is a judgement about Ev's own
words, and the lane that found it declined to make it.

What would settle it: Ev saying whether *"Promoted per Ev's request (PR
#17 thread)"* should stay as-is, be narrowed to the promotion, or go.
An `[ev]` PR is the channel.

## The general shape, which outlives this instance

`memories/review-and-dependency-policy.md` names the failure mode two
paragraphs above the withdrawal: *"**Never enshrine a causal story you
have not checked** — a lane's, a reviewer's, or a warning's."* Here the
lane enshrined **its own recommendation** and cited the person it had
recommended it to. The check that catches it is not `git log -S` on the
sentence (which lands correctly on `e9eeace50`) but reading the cited
thread and asking which comments are the account's owner speaking.

