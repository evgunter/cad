---
id: the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files
kind: issue
title: Ev withdrew the reviewer-suite no-simplify rule; nineteen files under crates/ still state it, in two sentences that need different dispositions
status: open
opened: 2026-09-19
---


## Finding

- **Where**: **19** files under `crates/`, in **two different sentences
  with two different objects** — a distinction that decides the
  disposition and that this row missed when it was opened:
  - **15 say "…to match shipped fixtures"**, which is verbatim the
    phrase the memory names as withdrawn: `topo/src/review_m0_pr7.rs`,
    `topo/src/review_m1_pr1.rs`, all six `topo/src/review_m1_pr2/*`,
    five `geom-core/tests/review_m0_pr*.rs`, two
    `geom/tests/surfaces/review_m2_pr1*.rs`, and
    `profile/tests/review_m2_pr2.rs`.
  - **2 say "…to match the implementation"** and scope it to *the
    derivations* (ledgers, anchor rules, orbit orders,
    slot/generation semantics), not to fixtures:
    `topo/src/review_m1_pr3.rs` and `topo/src/review_m1_pr4.rs`. Plus
    `topo/tests/review_m1_pr5.rs`, same sentence, same scope — **3 in
    this group.**
  **That second sentence is not the withdrawn rule.** It is close to
  what the memory *kept*: *"keep their own code only where a row's claim
  needs its own derivation."*

  **The count in this row's title and id was 17, and that was wrong for
  the reason this program exists to name.** A single-line
  `git grep 'do not "simplify"'` misses every carrier where the phrase
  wraps across two `//!` lines — which is how
  `topo/tests/review_m1_pr5.rs` and `profile/tests/review_m2_pr2.rs`
  were missed. A multiline scan (`rg -U 'do not\s*(//!)?\s*"simplify"'`)
  returns 19. The id keeps "seventeen" because ids are stable
  (`work/README.md`); the title is corrected.
- **Importance**: high — it is binding-shaped text, it is withdrawn, and
  it is the exact instruction that decides whether a duplication row may
  fold a reviewer-written fixture
- **Confidence**: sure. The withdrawal is quoted below from the memory
  itself; the 19 carriers are a multiline scan at `origin/main`
- **Raised by**: the S-DUP orchestrator, 2026-09-19, out of the
  cube-sequence fold's lane asking for a second reader on exactly one of
  the 17

## The withdrawal

`memories/review-and-dependency-policy.md` — a file whose contents are
Ev's call under CLAUDE.md — says:

> **Reviewer tests are ordinary tests (Ev, 2026-09-04).** … They share
> helpers where two files build the same thing, keep their own code only
> where a row's claim needs its own derivation (a general test-design
> question, not a question of who wrote the row) … An earlier version of
> this memory made reviewer suites a protected class ("promoted as-is",
> "independence worth keeping", **"never simplify to match shipped
> fixtures"**); **that reading was withdrawn** when two test-support trees
> were found stating opposite rules for the same class of duplicate.

Authored in **`3a272fefb`, 2026-09-04** — *"Reviewer tests are ordinary
tests: the policy memory rewritten to Ev's ruling; the two test-support
headers and the issue follow"* — matching the memory's own `(Ev,
2026-09-04)` date. Its stat is independent evidence for this row: it
updated the memory and exactly **two** test-support headers
(`geom-brep`, `sweep`), leaving every carrier below untouched.

**This row first cited `ff000b52e`, and that was an artefact of the
tool.** This checkout is a **shallow clone**, and `git log -S` without
`--all` bottoms out at the graft boundary — `ff000b52e` is a
`github-actions[bot]` render commit with no parents. CLAUDE.md's
"check that Ev ever agreed, before you wait for Ev" procedure *is* a
`git log -S`, so **the procedure silently lies here unless `--all` is
passed**, attributing ratified text to a bot. Anyone running that check
in this repo passes `--all` and reads the author.

So the phrase these files state is
the withdrawn reading, quoted back at every future lane from inside the
source tree, where a lane reads it and a memory index does not reach.

## Why this is S-DUP's row and not someone else's

It is one instruction spelled nineteen times, whose canonical spelling
was retracted and whose copies were not. That is this program's subject
with prose as the artifact instead of code. And the same memory names
the failure mode two paragraphs above the withdrawal:

> **Never enshrine a causal story you have not checked** … When you
> retract one, **grep for the claim, not the sentence**: a correction
> made where you first wrote it leaves every other copy standing.

The retraction was made where it was first written. Nineteen copies
stood — and this row's own first count of them was two low, taken with
a line-shaped instrument over a sentence that wraps.

## What the surviving rule actually directs

Not "leave the copies alone". The memory's positive clause is *share
helpers where two files build the same thing; keep your own code only
where a row's claim needs its own derivation*. That is a test-design
test, applied per file:

- **`review_m1_pr3.rs`** — its header enumerates what is independently
  derived (*"ledgers, anchor rules, orbit orders, slot/generation
  semantics"*). A box is not in that list, and the fixture is only what
  the derivations run on. PR #2843 folds its `build_box` onto the shared
  builder and proves the body byte-identical by `deep_snapshot`. That
  fold is what the surviving clause directs, not an exception to it.
- **`review_m1_pr2/cube_independent.rs`** — plausibly still keeps its
  own code, but **on a different ground than the one this program has
  been citing**. Its claim *is* independence: it is a cross-check that
  would catch a bug in the shared builder, so its row's claim needs its
  own derivation. That is the surviving clause, not the withdrawn one.
  The orchestrator has twice written that this file is exempt *"per Ev's
  request (PR #17 thread)"* — sourced from the file's own header and
  never checked against anything Ev ratified. The exemption should be
  restated on the ground that survives.
- The other fifteen are unexamined here.

## What this row owes

**Not a mass edit.** The remedy is per file and it is a reading of each
row's claim, which is the whole point of the clause that replaced the
blanket rule. What this row owes first is the census: for each of the 19,
does that file's claim need its own derivation, or is it sharing-eligible?
The two groups start from different places — the 15 state a withdrawn
rule and have to justify themselves afresh; the 3 state a surviving one
and keep their code unless their claim turns out not to need it.

Do not let a lane delete the sentence wholesale, and do not let one
preserve it wholesale. Either would re-enshrine a blanket rule, which is
the thing Ev withdrew.

## Unmeasured

- Whether any of the 19 headers was itself ever ratified by Ev, file by
  file. `git log -S` on each is the check, and it is the check this
  program keeps skipping.
- Whether the withdrawal has other carriers outside `crates/` (docs,
  `work/`, prompts).
- Whether `work/issues/reviewer-pair-rebuilds-two-trees-two-rules.md`,
  which the memory cites as the cause of the withdrawal, still exists
  anywhere; it is not at that path on `main`.
