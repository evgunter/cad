---
id: tess-lint-recourse-quote-half-pinned
kind: issue
title: the recourse quote is pinned from the binary's side only, and docs/TESS-BUDGET.md is the unpinned half
status: open
opened: 2026-09-08
refs: [tess-lint-twinned-csv-fixture, 2179]
---


## What

`tools/tess-lint`'s recourse names `docs/TESS-BUDGET.md` and quotes one
sentence from it verbatim. Three copies exist and only two of them are
held against each other:

- `tools/tess-lint/src/main.rs:170` EMITS it, inside the fold recourse:
  *"the values it blesses are current-state, not verified-optimal —
  docs/TESS-BUDGET.md, `restores coverage, it does not verify it`."*;
- `tools/tess-lint/tests/cli_contract.rs:191` ASSERTS the binary emits
  it, so the binary's own spelling is pinned;
- `docs/TESS-BUDGET.md:579` CARRIES it, and **nothing in the tree
  asserts that.**

So a reflow, a rewording or a deletion of that sentence in the document
leaves the binary quoting a sentence the document no longer contains,
and every gate stays green. The recourse is what a reader follows when
the lint reds; a quote that does not resolve sends them to a page that
does not say it.

## Provenance

Disclosed in `cli_contract.rs`'s own module header since `d829ddfee`
(2026-09-05) — *"**Only half of that pin is here**: this file asserts
the string the binary emits; nothing in the tree asserts the document
still contains it ... said plainly rather than left looking pinned"* —
and never given a file. Filed by METER unit 12 (PR 2179), whose
cross-root literal sweep turned it up and dispositioned it as *"a pin,
not a twin"*: correct about the twinning class this unit closes, and
silent about the residue, which `work/README.md` says owes a file at
the moment it is disclosed.

Checked at the time of filing: the sentence IS present at
`docs/TESS-BUDGET.md:579`, so this is a live gap and not a live break.

## The shape of the cure

The `D204` machinery is already in this crate's neighbourhood and this
is a plainer case than the ones it serves — the needle is prose in a
markdown file, not a Rust declaration, so no lexer is wanted:
`include_str!("../../../docs/TESS-BUDGET.md")` from a test in
`tools/tess-lint/tests/`, and an assertion that the file contains the
quoted sentence exactly once. `cut_line_pin.rs` already reads across
that root boundary the same way, and `src/lib.rs`'s `RULE_PAGE` pins
`tools/README.md`'s clause ids by the same means.

The one design question is which side owns the constant. Today the
sentence is typed out in `main.rs` and again in `cli_contract.rs`; a
cure that adds a third copy in the new test has widened the very thing
it was filed against, so the constant wants ONE home that all three
sites read — which is the same shape as this program's standing rule
that a census has one executable home.

## Fence

`tools/tess-lint/*` (METER's) plus a READ of `docs/TESS-BUDGET.md`.
`main.rs` is in the diff if the constant moves there.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
