---
id: plan-states-the-withdrawn-reviewer-independence-rule
kind: issue
title: tcost's plan states the reviewer-independence rule the policy memory withdrew
status: open
opened: 2026-09-19
---



## Finding

- **Where**: `work/tcost/plan.md`, twice.
  - In the reading list: *"`memories/review-and-dependency-policy.md` —
    **retirement is always permitted** … A promoted reviewer suite's
    **independence is worth keeping** where it pulls its weight; that is
    not a prohibition on retiring the rest."*
  - In the exit criteria: *"Reviewer suites that pull their weight keep
    their **independence from shipped fixtures**
    (`memories/review-and-dependency-policy.md`)."*
- **Why it is wrong**: *"independence worth keeping"* is verbatim one of
  the three phrases the memory names as withdrawn: *"An earlier version
  of this memory made reviewer suites a protected class ('promoted
  as-is', 'independence worth keeping', 'never simplify to match
  shipped fixtures'); that reading was withdrawn"* (Ev, 2026-09-04).
  The surviving clause decides it per row, not per author: *"They share
  helpers where two files build the same thing, keep their own code only
  where a row's claim needs its own derivation (a general test-design
  question, **not a question of who wrote the row**)."*
- **Importance**: high for this program specifically. The retirement
  half of both citations is accurate and still load-bearing, so the
  sentences read as correct; the independence half is the withdrawn
  reading, and it sits in **the plan's exit criteria**, which is what a
  S-TCOST lane checks its unit against before it lands. A source header
  states the rule where a lane may read it; a plan's exit criteria state
  it where a lane must.
- **Confidence**: sure.
- **Raised by**: the S-DUP lane for the withdrawn-no-simplify unit,
  2026-09-19, found by enumerating every tracked citation of
  `memories/review-and-dependency-policy.md` — 30 occurrences in 27
  files — and classifying each by what it claims the memory says.

## What the remedy is, and why S-DUP did not apply it

The retirement clause stays; only the independence clause is at issue.
Rewriting an exit criterion is writing a standing instruction for this
program's lanes, which is S-TCOST's call and not a visiting lane's. The
substitution the memory supports is the per-row test — a reviewer suite
keeps its own code where **that row's claim** needs its own derivation —
but whether S-TCOST wants its exit criteria phrased that way, and what a
lane is then expected to check, is a decision for whoever owns this
plan.

`work/tint/tint-plan-states-the-withdrawn-reviewer-independence-rule.md`
is the same defect in S-TINT's plan; they are filed separately because
one file is one item and neither program may edit the other's.
