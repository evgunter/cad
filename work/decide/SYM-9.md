---
id: SYM-9
kind: unit
title: what a refused decision may retry: the wider ring and the kept atom, measured first on the six documents
status: closed
opened: 2026-09-14
branch: sym/9-retry-ladder
refs: [coefficient-ring-width-is-not-monotone-in-reach]
priority: P1
cost: H
closed: 2026-09-24
---



## What

The ring's width is not monotone in reach (SYM-3) and a rule can cost bits (SYM-5 PR-2's width row), and the ladder makes one attempt per rung. Phase 1 measures, per refused decision on the six documents, what a second attempt at a wider ring or with the opening rule off would recover and at what cost; Phase 2 adds those retries to the early and door rungs behind budget dials, counted in the receipt. Block SYM-B2 slot 2 (H / NUMERIC, pre-draw); the full v6 dual. Spec: `docs/SYM-9-SPEC.md`.

## Dispatched (2026-09-15, ~01:35Z)

Block SYM-B2 slot 2, arm FABLE per the block's draw (byte 56 ⇒ fable
at slot 2); the v6 dual at the PR. Inside the program's paths;
`drive.rs` only for a default dial, by the announced seam.


## Displaced (2026-09-19)

The 2026-09-15 dispatch never began: the lane died at its first API
call in the credit outage, no branch, no commit. Ev's ruling on #2728
makes the decision-door fold the program's priority, so SYM-10 takes
block SYM-B2's slot 2 (the same pre-draw fields, H / NUMERIC); SYM-9
opens the next block. Back to `status: spec`.

## Dispatched (2026-09-22, block DECIDE-B1 slot 1)

Arm OPUS per the block's draw (byte 248 ⇒ fable at slot 2); v7 IN, the
full v6 dual; spec `docs/SYM-9-SPEC.md` with its A1 amendment (the
branch cut from `props/sign-hull`'s head `cd14d4fd9` where DECIDE-3
lives; the PR targets `props/sign-hull`; the link row's sixteen as
Phase 1's first table and the kept-atom retry's measured instance).
Lane `/home/user/lanes/sym-9`, branch `sym/9-retry-ladder`.


## In review (2026-09-22), fix pass (2026-09-24)

PR #3083 against `props/sign-hull`, branch `sym/9-retry-ladder`. Both
reviews MERGEABLE-AFTER-FIXES (R1 2/7/7, R2 1/8/5); the fix pass takes
the union, items A–U of the brief, every one listed in the PR body.

Phase 1's tables stand, on five of the six documents at the nominal;
the pad is measured on the release leaf instrument, where the ladder
recovers nothing (`work/sym/the-pads-nominal-replay-is-not-takeable-on-a-four-core-box`).

What changed at the fix pass: the rules differentials run with no
ladder on both sides (the first cut had the ladder inside all of them),
so DECIDE-3's link pin is rule G's trade again; an attempt identical to
the first is not walked; the leaf instrument was taken with and without
the ladder in release. The fix pass then set the drive's default to NO
ladder on the leaf line (the ladder acts only on documents already over
1.6 s and adds 12.5–36 % there); R1's delta was MERGEABLE and recommended
reversing that, and the orchestrator ruled the reversal — a line the base
already fails is not a test of the ladder, and SYM-5 shipped rule E across
it as a disclosed trade. So `SymRetry::kept_atom` ships ON: twelve
theorems on R2's link (the ten rule G costs `carrier_on_surface_2`, and
two), six registrations on R2's bracket, nothing on the other four.

`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`
stays open at P2 for shape 2 (the render). Filed:
`work/sym/a-retrys-zero-has-no-cross-check-against-the-numeric-channel`.

## Closed (2026-09-24)

Merged as #3083 into `props/sign-hull` (head `32efb3abc`, run
36026935338). A refused decision may retry on the early, top-residual
and door rungs, and nothing about the first attempt moves: the drive
ships `SymRetry::kept_atom()` — rule G shut, then rule A shut — ON by
default across the 1.6 s line as a disclosed trade, as rule E did (the
link `[541,0,96,465] → [553,0,96,453]`, twelve theorems, among them the
ten rule G's default costs its `carrier_on_surface_2`; the bracket six
registrations; +1.0 s, +2.4 s and +16 s on the three documents already
over the line). Every rules differential runs at `SymRetry::none()` on
both sides, so DECIDE-3's gate still records rule G's own trade. The
dual on `7b3924cf1`: R1 (OPUS) MERGEABLE-AFTER-FIXES 2/7/7, R2 (FABLE)
MERGEABLE-AFTER-FIXES 1/8/5 — both found, by execution, the ladder
riding every rules differential; the union fix pass A–U; the delta by
R1 MERGEABLE with the default reversed (the fix pass had set it to none
on the orchestrator's leaf-line ruling, a test the base already fails)
and D1–D4, taken at `32efb3abc`. The spec's sentences that did not
survive: a retry "on the early and door rungs only" (the top-residual
rung is retried too); `retry_without` as one mask (two, not composable);
the dials on `SymBudget` (their own session door); six documents at the
nominal (five; the pad on the leaf instrument only); "the cost on the
leaf instrument" read as a gate on the default (it is a disclosure; the
line was failed by the shipped rules before the ladder). Spec deleted
with its note (`docs/doc-ledger/sym-9-spec.md`). Rows left open:
`rule-g-trades-sixteen…` (P2, shape 2's render),
`the-pads-nominal-replay-is-not-takeable-on-a-four-core-box`,
`a-retrys-zero-has-no-cross-check-against-the-numeric-channel`,
`decision-read-triples-the-plate-pin-suites-wall-time` (P2).
