---
id: the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files
kind: issue
title: Ev withdrew the reviewer-suite no-simplify rule; nineteen files under crates/ still state it, in two sentences that need different dispositions
status: closed
opened: 2026-09-19
closed: 2026-09-19
pr: 2886
---


## Finding

- **Where**: **19** files under `crates/`, in **two different sentences
  with two different objects** — a distinction that decides the
  disposition and that this row missed when it was opened:
  - **16 say "…to match shipped fixtures"**, which is verbatim the
    phrase the memory names as withdrawn: `topo/src/review_m0_pr7.rs`,
    `topo/src/review_m1_pr1.rs`, all six `topo/src/review_m1_pr2/*`,
    five `geom-core/tests/review_m0_pr*.rs`, two
    `geom/tests/surfaces/review_m2_pr1*.rs`, and
    `profile/tests/review_m2_pr2.rs`.
  - **3 say "…to match the implementation"** and scope it to *the
    derivations* (ledgers, anchor rules, orbit orders,
    slot/generation semantics), not to fixtures:
    `topo/src/review_m1_pr3.rs`, `topo/src/review_m1_pr4.rs` and
    `topo/tests/review_m1_pr5.rs`.
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

  **A line-shaped instrument then undercounted the SPLIT too**, twice: a
  regex for the object after *"to match"* put `review_m1_pr4.rs` in the
  wrong group, because there *"the"* ends one `//!` line and
  *"implementation's comments"* begins the next. The counts above were
  taken by **joining each file's `//!` block into one string and reading
  the object** — not by matching a pattern against lines. Three separate
  undercounts in one row, all the same cause: **this class of text wraps,
  and a line is not its unit.**
- **Importance**: high — it is binding-shaped text, it is withdrawn, and
  it is the exact instruction that decides whether a duplication row may
  fold a reviewer-written fixture
- **Confidence**: sure. The withdrawal is quoted below from the memory
  itself; the 19 carriers are a multiline scan at `origin/main`
- **Raised by**: the S-DUP orchestrator, 2026-09-19, out of the
  cube-sequence fold's lane asking for a second reader on exactly one of
  the 19

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
  **But its header carries a different withdrawn citation**, found by
  PR #2843's style review: it opens *"promoted into the shipped suite per
  the standing convention (`memories/review-and-dependency-policy.md`) …
  and the programs are kept."* The memory no longer says the programs are
  kept; it says the useful ones enter as normal rows and are *"trimmed,
  gated or retired under the same rules as every other row."* So the file
  cites a named memory for a rule that memory withdrew — **a carrier shape
  this row's grep structurally cannot see, because it paraphrases the
  withdrawn rule instead of quoting it.** How many of the 19, and how many
  files outside them, cite that memory for something it no longer says, is
  unmeasured, and is the sharper question this row should be asking.
- **`review_m1_pr2/cube_independent.rs`** — plausibly still keeps its
  own code, but **on a different ground than the one this program has
  been citing**. Its claim *is* independence: it is a cross-check that
  would catch a bug in the shared builder, so its row's claim needs its
  own derivation. That is the surviving clause, not the withdrawn one.
  The orchestrator has twice written that this file is exempt *"per Ev's
  request (PR #17 thread)"* — sourced from the file's own header and
  never checked against anything Ev ratified. The exemption should be
  restated on the ground that survives.
- The other sixteen are unexamined here.

## What this row owes

**Not a mass edit.** The remedy is per file and it is a reading of each
row's claim, which is the whole point of the clause that replaced the
blanket rule. What this row owes first is the census: for each of the 19,
does that file's claim need its own derivation, or is it sharing-eligible?
The two groups start from different places — the 16 state a withdrawn
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

## Closed 2026-09-19 — the denominator was the citation, and three of this row's numbers were wrong

### The sentence census, re-taken at `5b4979ef2`

Instrument: every tracked file (`git ls-files`, **no path argument**),
each file's comment markers stripped and the whole file collapsed to one
whitespace-normalised string before matching — so a sentence that wraps
across `//!` lines is one token, which is the defect that beat this row
three times. `do not\s*"?simplify"?` returns **six** files: the three
`topo` carriers of the SURVIVING rule
(`src/review_m1_pr3.rs`, `src/review_m1_pr4.rs`,
`tests/review_m1_pr5.rs`) and three tracker files. **The sixteen
group-A carriers no longer contain the sentence**: it was removed by the
carrier PR that `the-pr-17-promotion-attribution-is-half-checked`
records, and `work/comb/S36.md` already logs the removal. So this row's
title — *nineteen files under `crates/`* — was stale when this unit
opened, and so was its claim that `review_m1_pr3.rs`'s header still
says *"the programs are kept"*: that header now reads *"the useful ones
enter the permanent suite as ORDINARY rows … nothing here is a
protected class"*, which is the corrected text.

Two false positives the instrument returns and a reader must discard:
`local-scripts/monitors/github-away-channel.sh` (*"Do not 'simplify'
this back to one feed"*) and `tools/tess-lint/src/main.rs` (*"do NOT
simplify a demo's geometry"*). Same words, unrelated subject — which is
the answer to this row's second unmeasured item: **the withdrawn rule
has no carrier outside `crates/`.**

### The citation census — 35 files, each dispositioned

Same instrument, needle `review-and-dependency-policy`. Blind spot: a
file that states the withdrawn ground **without naming the memory** is
invisible to it (`review_gui0_r1`/`_r2` state their own grounds and do
not cite; both were read by hand and neither grounds itself in what the
file is).

**All three counts below are FILES, not citation sites.** Two files
carry more than one site — `crates/viewer/Cargo.toml` (four) and
`work/tcost/plan.md` (two) — and neither changes a file count. 35 = 28
+ 7.

- **Cites a SURVIVING clause — 28 files, no action.** Named in full,
  because a number and a list that are not the same object is where a
  census loses things:
  1. `.github/workflows/ci.yml` — dependency age
  2. `Cargo.toml` — dependency age
  3. `benches/Cargo.toml` — dependency age
  4. `crates/pncad-py/Cargo.toml` — dependency age
  5. `crates/viewer/Cargo.toml` — dependency age, four sites
  6. `crates/editor-core/tests/m10_2_r1_probes.rs` — retirement licence
  7. `crates/step-export/tests/rev_probe.rs` — retirement licence
     (accurate in substance; it attributes *"a row that asserts nothing
     is never a gate"* to this memory when that sentence is
     `test-suite-cost`'s — mis-shelved, not withdrawn)
  8. `crates/topo/src/review_m1_pr3.rs` — the surviving rule, scoped to
     the derivations
  9. `crates/topo/src/review_m1_pr4.rs` — same
  10. `crates/topo/tests/review_m1_pr5.rs` — same
  11. `memories/MEMORY.md` — the index line, corrected text
  12. `memories/orchestration-model.md` — ordinary-rows rule
  13. `memories/review-and-dependency-policy.md` — the memory itself
  14. `memories/test-suite-cost.md` — retirement, cross-reference
  15. `docs/REVIEW-STYLE-DISPATCH.md` — the causal-story rule
  16. `work/view/log.md` — the causal-story rule
  17. `work/sym/logs/sym-4-review-brief-r1.md` — bare pointer
  18. `work/sym/logs/sym-4-review-brief-r2.md` — bare pointer
  19. `work/sym/logs/sym-5-review-brief-r1.md` — bare pointer
  20. `work/sym/logs/sym-5-review-brief-r2.md` — bare pointer
  21. `work/comb/S36.md` — records the correction
  22. `work/dup/log.md` — records the correction
  23. `work/dup/the-cube-sequence-is-written-five-times-and-twice-inside-src.md`
      — cites the surviving clause
  24. `work/dup/the-pr-17-promotion-attribution-is-half-checked.md` —
      quotes the withdrawn clause as withdrawn
  25. `work/dup/the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files.md`
      — this row
  26. `work/dup/viewer-review-suites-cite-the-withdrawn-independence-reading.md`
      — the viewer row
  27. `work/tcost/plan-states-the-withdrawn-reviewer-independence-rule.md`
      — the sibling row, which quotes its carrier
  28. `work/tint/tint-plan-states-the-withdrawn-reviewer-independence-rule.md`
      — the sibling row, which quotes its carrier
- **Cites a WITHDRAWN clause — 7 files.**
  - `crates/viewer/tests/common/mod.rs`, `review_gui2_r1.rs`,
    `review_gui2_r2.rs`, `review_gui3_r1.rs` — the viewer row's four.
    **Fixed in this PR**, per file, on the surviving ground.
  - `crates/sweep/examples/p1b_r2_ab.rs` — cites *"`memories/
    review-and-dependency-policy`'s promotion rule"*, a named clause
    the memory no longer has. **Fixed in this PR**: the
    assertion-free-never-gates claim now cites `test-suite-cost`, which
    is where it lives, and the policy memory is cited for the
    ordinary-row rule, which is what it says.
  - `work/tcost/plan.md` (two sites) and `work/tint/plan.md` — already
    owned by `work/tcost/plan-states-the-withdrawn-reviewer-independence-rule`
    and `work/tint/tint-plan-states-the-withdrawn-reviewer-independence-rule`.
    Not touched: one file, one item.

### The three surviving `topo` carriers keep their code, on the surviving ground

`review_m1_pr3.rs`, `review_m1_pr4.rs` and `tests/review_m1_pr5.rs`
scope *do not "simplify" them to match the implementation('s comments)*
to the **derivations** — ledgers, anchor rules, orbit orders,
slot/generation semantics, the splice taxonomy, the separating-curve
argument — and each names what the derivation was computed from
instead (Mäntylä ch. 9/11, the pinned PR 1/2/3 surgeries). That is the
surviving clause's test answered in the file: the thing the row would
otherwise read is the implementation's own commentary, and a wrong
convention there would be invisible to a row that read it. No edit.
Their headers already carry the corrected citation.

### The first unmeasured item: it cannot be settled from git here

*Was any group-A header ratified by Ev, file by file?* The check does
not reach a conclusion in this checkout, and the reason is not only the
shallow grafts that
`the-ratification-check-claude-md-prescribes-is-unreliable-here`
measured. Reading the oldest commits on those paths oldest-first, the
promotions (`d3d61be50`, `633d98c68`, `eff70cfc6`, all 2026-07-16/18)
are authored **`Evan Ryan Gunter <evgunter@gmail.com>`** and their
messages are plainly lane work (*"tests: promote salvaged M0
adversarial-review demos into CI"*, *"PR 2 fix pass: …"*). `CLAUDE.md`
says that address *"signs every commit already"*; across the carrier
paths, 160 commits are authored `Claude` and 69 carry Ev's name or
address. **So the author field is not a discriminator**, and
`git log`'s answer to "did Ev write this" is the same account-conflation
trap `the-pr-17-promotion-attribution-is-half-checked` names on the
GitHub side. This is a firmer result than *unmeasured*: the prescribed
check cannot answer the question in this repo at all.

### The third unmeasured item: the cited issue existed and was deleted

`work/issues/reviewer-pair-rebuilds-two-trees-two-rules.md` is not in
the tree, was deleted by `499122b10` (*"work: code-quality leaves the
tracker (DOC-LEDGER sweep 11)"*), and is readable at `81b5a3cfb`:
`kind: issue`, opened 2026-09-03, **closed 2026-09-04** — the day of
the withdrawal — raised by TCOST-10's style review over
`crates/geom-brep/tests/shared/mod.rs` and
`crates/sweep/tests/common/cavity.rs`. The memory still cites the dead
path; that is `the-policy-memory-cites-a-tracker-file-that-left-the-tree`,
filed for Ev because `memories/` is his.

## The third instrument, run on the style review's instruction (2026-09-19)

The citation census's declared blind spot — *a file stating the
withdrawn ground without naming the memory* — was closed by hand-reading
two files, which is not an instrument. Method item 8 says a disclosed
blind spot is an instruction to run another one, so one was run:
enumerate the **class-shaped ground** itself over every tracked file,
markers stripped and text joined — a generalisation over what a file IS
(*"a/the/every review suite"* plus a property, *"reviewer suites
are/keep/derive"*, *"derives what it needs independently"*, *"the
independence is/worth…"*, *"protected class"*, *"promoted as-is"*,
*"keep verbatim"*, *"like every other fixture here"*). **28 files.**

Discarding the homonyms — six files where *"independence"* is a
numeric quantity (a `|det|`, an order-independence claim) — the live
carriers the citation census could not see were **two**, and both are
real:

- `crates/viewer/tests/review_gui3_r2.rs` — *"this suite's own, like
  every other fixture here (a review suite derives what it needs
  independently)"*, over a **fourth** copy of the `xy_frame` body, plus
  private `len`/`scl`/`tempdir`. Fixed: ground restated on what its
  rows assert, sugar shared.
- `crates/viewer/src/camera.rs` — in **`src/`**, and it lets the
  withdrawn reading decide a public error vocabulary rather than a test
  fixture. Filed as
  `a-viewer-error-arm-is-not-split-because-a-review-suite-pins-it`.

`review_gui4_r1` and `review_gui4_r2` were read too and are **not**
carriers: both ground their own derivation on a named self-consistency
defect in the shipped oracle (*"the solve derives the placement FROM
the authored alignment, so that assertion holds for any alignment the
tool mints"*), and `gui4_r1` carries a measured mutation for it. That
is the surviving clause, not the withdrawn one.

**Blind spot of the third instrument**: it matches a ground phrased as
a generalisation. A file declining a change for the same bad reason,
phrased entirely about one named suite, would not match — and nothing
tells that apart from a legitimate instance-specific argument except
reading it.
