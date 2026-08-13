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

**Reviewer suites are a SEAM TO MINE, not cargo to carry (Evan,
2026-08-13 — amends the PR #17 reading).** After each PR's fix pass the
reviewer's consumer suite is a resource: go through it, take the rows
worth keeping as permanent gates, and RETIRE THE REST. Promotion is a
per-row act of selection, not the default fate of the file. The rows
that are kept are independent derivations and that independence is
their regression value — so do not "simplify" a KEPT row to match
shipped fixtures. That protection covers rows deliberately promoted; it
was never a prohibition on retiring the others, and reading it as one
is what let the suites accumulate.

A row that ASSERTS NOTHING is never promotable. A `println!` probe, a
census, a truth-table dump, a latency table — these are evidence for a
reviewer at the time, and they cannot fail, so they cannot gate. They
are exactly the thing to mine and drop.

Retiring a row means naming the gate that now owns its claim (a
stronger permanent row, or a new one written for it). "It is not an
exact duplicate" is not a reason to keep something — an assertion-free
probe is never an exact duplicate of anything, which is precisely how
`step-export`'s `rev_probe` rows survived as five-ε-row gates while
their own file header said "Not in the `all` aggregator".

**Why this needs saying explicitly:** each crate's
`every_suite_file_is_aggregated` guard plus `autotests = false` means
any file dropped into `tests/` is forced into the `all` binary and runs
on every ε row forever. So "review artifact" and "permanent gate" are
the SAME THING by default, and deletion is the only retirement lever.
The selection has to happen at the fix pass, deliberately, or it never
happens. Measured 2026-08-13: 55% of all workspace test time sat in
modules named after a specific past review or PR.

Suites hit by later API changes (e.g. PR 5's raw-builder demotion)
migrate or get pruned at that PR like any other test.

**Dependencies: install freely, with supply-chain sanity.** Installing
tools/crates as needed is fine, as long as it isn't genuinely risky
supply-chain-wise; put roughly a **2-week minimum age** on dependency
versions (avoid brand-new releases). Combine with the existing
crate-landscape vetting in DESIGN.md ([[cad-project-state]]).
