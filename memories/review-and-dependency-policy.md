---
name: review-and-dependency-policy
description: Ev's rules for reviews (hands-on e2e exercise, unique-signal-only local runs, when a stated gap blocks, reviewer tests are ordinary tests) and for adding dependencies
metadata:
  type: feedback
---

**Reviews must include end-to-end exercise, when applicable.** Reading
the diff is not enough: a reviewer writes and runs real programs
against the functionality under review, to check not just in-scope
correctness but that the thing solves the right problem and isn't
missing something that matters in practice. Report what the exercise
revealed about scope and ergonomics alongside the ranked findings. For
pure-scaffolding diffs with no runtime surface, running the
toolchain/CI commands is the e2e equivalent.

**Reviewer local runs = unique signal only.** Review charters enumerate
the runs only the reviewer can do — their own probes, merge-base
differentials, planted corruptions, non-CI rows — and say explicitly
"existing pinned suites ride the PR gate; verdict conditional on
green." Re-running CI-covered suites in a review clone is duplication
([[local-battery-scope]]).

**Never enshrine a causal story you have not checked** — a lane's, a
reviewer's, or a warning's. Fix the facts and write no account of how
they came to be that way. When you retract one, grep for the claim, not
the sentence: a correction made where you first wrote it leaves every
other copy standing.

**A verdict conditioned on a gate is only as good as a check that the
gate ran** (2026-08-29, PCURVE P-1b's dual, R2's own post-mortem). A
reviewer wrote "APPROVE WITH FIXES, conditional on that gate being
green", noted that the unit's suites "ride the PR gate, not me" per the
reviewer-local-runs rule, and never verified a green existed. It also
quoted the PR's own citation — "Green in run 33132582293" — into its
report as supporting text for a verdict move. One `gh run view` showed
that run **cancelled, on a different SHA**; the reviewed head had **zero**
runs; and two of the unit's own committed rows were red on it.

**Why:** the reviewer-local-runs rule (unique signal only) presumes the
gate runs. That premise is not self-verifying, and a verdict resting on
it is unsupported rather than wrong — quieter and worse. In the
reviewer's own words, it is [[refusal-text-is-not-cause]] committed by
the reviewer invoking it: taking a cited artifact's DESCRIPTION for the
artifact's STATE.

**How to apply:** if a review's verdict says "conditional on green",
verifying that green exists is part of the review, not the
orchestrator's follow-up. Resolve every run ID a PR cites — status AND
head SHA — before quoting it as support. Excluding the unit's own rows
by policy is fine; assuming something else ran them is not.

**A stated coverage gap is a BLOCKER when the untested axis is the
row's own subject.** Merging a row whose declared purpose is X, on
evidence that never exercised X, buys nothing — the row is decorative
until the axis it names is drawn, and an accepted gap of that shape is
usually a liveness bug in disguise rather than a scoping decision.
Hosted CI draws ONE eps per run from the seed, so a row parameterised
on eps must be run locally at every eps it claims to cover, before
merge. When an implementer names a gap, ask whether the untested axis
is the row's subject or merely adjacent: adjacent is a follow-up, the
subject is a blocker.

**Reviewer tests are ordinary tests (Ev, 2026-09-04).** A reviewer may
write tests while reviewing; the useful ones go into the permanent
suite as normal rows when that makes sense, and nothing about them is
special afterwards. They share helpers where two files build the same
thing, keep their own code only where a row's claim needs its own
derivation (a general test-design question, not a question of who
wrote the row), and are trimmed, gated or retired under the same rules
as every other row ([[test-suite-cost]]). An earlier version of this
memory made reviewer suites a protected class ("promoted as-is",
"independence worth keeping", "never simplify to match shipped
fixtures"); that reading was withdrawn when two test-support trees
were found stating opposite rules for the same class of duplicate
(`work/issues/reviewer-pair-rebuilds-two-trees-two-rules.md`).

**Dependencies: install freely, with supply-chain sanity.** Installing
tools/crates as needed is fine unless genuinely risky; put roughly a
**2-week minimum age** on dependency versions. Combine with the
crate-landscape vetting in DESIGN.md.
