---
name: review-and-dependency-policy
description: Evan's rules for reviews (hands-on e2e demos, not just diff reading) and for adding dependencies (fine to install; ~2-week minimum release age)
metadata:
  type: feedback
---

Two standing rules from Evan (2026-07-15), applying to any agent working
on this project:

**Reviews must include end-to-end exercise, when applicable.** Code
review alone is not enough: reviewer agents should walk through a few
real demos — write and run actual programs against the functionality
under review — to check not only in-scope correctness but that the thing
*solves the right problems* and isn't missing something that matters in
practice.

**Why:** a diff can be locally correct while the API is unusable or
misses a practically important case; only driving it end-to-end surfaces
that.

**How to apply:** reviewer prompts should require writing/running small
realistic usage programs (not just the crate's own tests) and reporting
what the exercise revealed about scope/ergonomics, alongside the ranked
findings. For pure-scaffolding diffs with no runtime surface, running
the toolchain/CI commands is the e2e equivalent.

**Reviewer local runs = unique signal only (2026-08-01, from
Evan's iteration-speed principle, [[local-battery-scope]]):**
review charters enumerate the runs only the reviewer can do —
their own probes, merge-base differentials, planted corruptions,
and any non-CI rows (e.g. the demo tour's ε battery) — and say
explicitly "existing pinned suites ride the PR gate; verdict
conditional on green." Re-running CI-covered suites in a review
clone is duplication (3 of the session's 4 waiter-parks happened
grinding exactly such runs).

**Never enshrine a causal story you have not checked** — a lane's, a
reviewer's, or a warning's. Fix the facts and write no account of how they came
to be that way. And when you retract one, grep for the claim, not the sentence —
a correction made where you first wrote it leaves every other copy standing.

**Reviewer suites get promoted into CI.** A review charter has the
reviewer write their OWN consumer test suite — an independent
derivation of what the PR claims, not a re-reading of its diff. After
that PR's fix pass, **that suite is committed into the repo** as a
normal test file (the original example: `crates/topo/tests/
review_m1_prN*.rs`), where the aggregation guard picks it up and it
becomes a permanent gate like any other test (Evan, PR #17 thread).
That commit is what "promotion" means below. The suites are
independent derivations and that independence is their regression
value.

**Promotion stays cheap; RETIREMENT IS ALWAYS PERMITTED (Evan,
2026-08-13 — amends how the PR #17 clause has been read).** Three
parts, and note what is deliberately NOT being added:

1. **The conventions bind the reviewer WRITING the suite**, not the
   person promoting it. A reviewer authoring a consumer suite follows
   [[test-suite-cost]] as they write: no fixed seeds, counts on the
   EFFORT dial, and be aware that a `println!` probe, a census, a
   truth-table dump or a latency table cannot fail and therefore
   cannot gate — those are evidence for the review, and should be
   marked as such rather than left to become permanent rows.
2. **Promotion takes the suite AS-IS.** It is not an audit. There is NO
   obligation to comb a suite row by row at the fix pass, and adding
   one would be a recurring tax on every review to pay for a problem
   that is cheap to fix later.
3. **Full license to fix them afterwards.** If a promoted suite becomes
   a problem — slow, redundant, asserting nothing — trim or retire it
   then, freely, without ceremony. Do not "simplify" a row that is
   pulling its weight to match shipped fixtures (that is the
   independence above, and it is worth keeping); but that protection
   was NEVER a prohibition on retiring ones that are not, and reading
   it as one is what let the suites accumulate.

When retiring, name the gate that now owns the claim (a stronger
permanent row, or a new one written for it). "It is not an exact
duplicate" is not a reason to keep something — an assertion-free probe
is never an exact duplicate of anything, which is precisely how
`step-export`'s `rev_probe` rows survived as five-ε-row gates while
their own file header said "Not in the `all` aggregator".

**Why the license needs stating:** each crate's
`every_suite_file_is_aggregated` guard plus `autotests = false` means
any file dropped into `tests/` is forced into the `all` binary and runs
on every ε row. So "review artifact" and "permanent gate" are the SAME
THING by default. That default is fine — cheap promotion is worth it —
*provided* clearing up afterwards is uncontroversial. When it was last
measured, most of the workspace's test time sat in modules named after
a specific past review or PR, and it got there because the clause above
read as forbidding exactly that clear-up.

Three levers, not one — reach for the right one:

- **Delete** it, when a stronger permanent row already owns the claim.
  Name that row.
- **`#[ignore]` it**, when the row REPORTS rather than gates. That
  takes it out of the ε matrix while leaving it runnable. If something
  still wants the report, that runner needs `--ignored` added to it.
  Worked example, `m4_pr8_latency::rebuild_latency_table` (#462): its
  own header says "REPORTING (measured, never gated) … there is no
  threshold gate", and a dedicated `rebuild latency (reporting)` job
  had existed for it since 2026-07-26 — yet the aggregation guard was
  ALSO running it in all five ε rows, where its green-document and
  counted-reuse assertions are a strict subset of `m4_pr8_corpus`'s.
  Six payments for one report. (It has since grown assertions of its
  OWN — the corpus manifest's nodes/cone pins, 2026-08-17 — so it is no
  longer wholly redundant with the corpus row; the ignore still holds,
  because those pins are ε-independent by construction and one run per
  gate covers them.) Note
  the shape: the job already existed, so the fix was one attribute and
  one flag. Reach for this lever when a reporting row is being paid for
  by the matrix as well; it is not an instruction to stand up a new job
  per retired test.
- **Gate it on the change filter**, when it is a randomized sweep — see
  [[test-suite-cost]]. Runs only when the code it tests moved.

What is NOT available is an automatic check: a probe that only prints
still `unwrap`s, so "has no assertions" is not reliably greppable. So
this cannot be enforced at promotion even if we wanted to — another
reason to put the convention on the author and the license on whoever
cleans up later.

Suites hit by later API changes (e.g. PR 5's raw-builder demotion)
migrate or get pruned at that PR like any other test.

**Dependencies: install freely, with supply-chain sanity.** Installing
tools/crates as needed is fine, as long as it isn't genuinely risky
supply-chain-wise; put roughly a **2-week minimum age** on dependency
versions (avoid brand-new releases). Combine with the existing
crate-landscape vetting in DESIGN.md.
