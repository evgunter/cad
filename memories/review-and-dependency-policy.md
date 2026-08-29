---
name: review-and-dependency-policy
description: Evan's rules for reviews (hands-on e2e exercise, unique-signal-only local runs, when a stated gap blocks) and for adding dependencies
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

**Reviewer suites get promoted into CI.** A charter has the reviewer
write their OWN consumer suite — an independent derivation of what the
PR claims, not a re-reading of its diff — and after the fix pass that
suite is committed as a normal test file, where the aggregation guard
picks it up and it becomes a permanent gate (Evan, PR #17). That
independence is its regression value.

**Promotion stays cheap; RETIREMENT IS ALWAYS PERMITTED (Evan,
2026-08-13).** Three parts:

1. **The conventions bind the reviewer WRITING the suite**, not the
   person promoting it: [[test-suite-cost]] as they write — no fixed
   seeds, counts on the EFFORT dial, and mark evidence-only rows (a
   `println!` probe, a census, a truth-table dump, a latency table
   cannot fail and therefore cannot gate) as such rather than letting
   them become permanent rows.
2. **Promotion takes the suite AS-IS.** It is not an audit; combing a
   suite row by row at every fix pass would be a recurring tax to pay
   for a problem that is cheap to fix later.
3. **Full license to fix them afterwards** — trim or retire a promoted
   suite that turns out slow, redundant or assertion-free, freely and
   without ceremony. Do not "simplify" a row that is pulling its weight
   to match shipped fixtures (that independence is worth keeping), but
   that protection was never a prohibition on retiring the rest.

Each crate's `every_suite_file_is_aggregated` guard plus
`autotests = false` means any file dropped into `tests/` is forced into
the `all` binary and runs on every ε row — so "review artifact" and
"permanent gate" are the same thing by default. That default is fine
*provided* clearing up afterwards is uncontroversial. When retiring,
name the gate that now owns the claim. "It is not an exact duplicate"
is not a reason to keep something: an assertion-free probe is never an
exact duplicate of anything.

Three levers, not one:

- **Delete** it, when a stronger permanent row already owns the claim —
  name that row.
- **`#[ignore]` it**, when the row REPORTS rather than gates: that
  takes it out of the ε matrix while leaving it runnable. If something
  still consumes the report, its runner needs `--ignored`. Reach for
  this when a reporting row is being paid for by the matrix as well;
  it is not an instruction to stand up a job per retired test.
- **Gate it on the change filter**, when it is a randomized sweep
  ([[test-suite-cost]]) — it then runs only when the code it tests
  moved.

No automatic check is available: a probe that only prints still
`unwrap`s, so "has no assertions" is not reliably greppable. Hence the
convention on the author and the license on whoever cleans up later.

**Dependencies: install freely, with supply-chain sanity.** Installing
tools/crates as needed is fine unless genuinely risky; put roughly a
**2-week minimum age** on dependency versions. Combine with the
crate-landscape vetting in DESIGN.md.
