---
id: no-guard-reds-on-an-unlabelled-figure-in-the-budget-doc
kind: issue
title: Nothing reds when an unlabelled current figure arrives in docs/TESS-BUDGET.md, and the fix that disclosed the hole walked into it
status: open
opened: 2026-09-16
priority: P3
cost: E
---


## Was

Disclosed by INSTR unit 1 (`instr/u1-sizing-figures-second-copy`, PR
2757) and filed in the PR that discloses it, after unit 1's style
review named the disclosure as a Q6 deviation owing a scheduled
followup and the PR filed none.

## Finding

`docs/TESS-BUDGET.md` states its own rule at the top of "The committed
baseline":

> *"**no count over this file is written into this document as a
> CURRENT reading** — what it says today is one command and one test"*

and then enumerates the passages that are exceptions, each frozen and
labelled at its own site. **Nothing executes any part of that.** Three
things are unguarded, in increasing order of how quietly they fail:

1. **The enumeration's own count.** It reads "Three passages do carry
   counts over this file, and all three are FROZEN". Unit 1 moved it
   from two to three. No test reads that word, so a fourth frozen
   passage arriving leaves it saying three, and the sentence that
   promises the list is complete becomes the thing that is wrong.
2. **A new unlabelled absolute arriving in the prose.** The rule
   forbids it; nothing looks. `tools/tess-lint/tests/baseline_census.rs`
   discloses this at the head of the file — *"What a re-cut must not
   produce is a new unlabelled absolute — and no test can see one
   arrive"* — which is honest and is not a schedule.
3. **A labelled passage whose label stops being true.** Each frozen
   passage names the cut it reads; nothing checks that the cut named
   still holds the figures beside it.

**The hole is live, and its first instance was produced by the commit
that disclosed it.** Unit 1's own fix wrote *"the corpus has grown
sized faces since — 64 there, 80 now"* into the passage explaining why
the comparison had to be pinned. `80` is a current reading of the
committed baseline — asserted by the face-identity census, under
`the_committed_baseline_carries_this_many_indistinguishable_pairs` —
written present-tense and cited to nothing, which is exactly what the
rule four hundred lines above forbids. It survived the unit's own re-derived
transcription sweep, because that sweep's blind-spot clause says in as
many words that a two-digit corpus count is too short to grep for. It
was caught by a human reading the diff. That is the same way
`79420738f`'s ungated-column move was caught, one instrument over, and
the same way this class has been caught every time.

## Shape

The Q6 measurement shape: a stated rule with no instrument, no
register entry, and no written reason it can have neither. Three
remedies are available and they are not equivalent:

- **The enumeration's count** is the cheap one and is a real guard: a
  test reading the document for the labelled-passage markers and
  asserting the enumeration's number against how many it found. It
  needs the markers to be machine-findable, which today they are not.
- **The arriving absolute** is the expensive one. A grep for digit
  groups over the document would fire on every frozen passage, the
  pre-fix block and every table, so it needs the frozen regions
  delimited before it can say anything — which is the same
  prerequisite as above, and is the argument for doing that first.
- **A register row** saying "re-read the labelled passages on each
  re-cut" discharges none of the three but is honest, and is what the
  sibling row `baseline-census-transcription-sweep-literals-stale`
  proposes for its own subject.

## Not covered by the neighbours

- `baseline-census-transcription-sweep-literals-stale` is the stale
  SWEEP PATTERN — a pattern derived from figures that have moved. This
  row is about prose arriving that no pattern was ever going to match.
- `tess-lint-ungated-columns-fold-silently` is CSV columns neither
  census reads. This is prose in the document.
- `work/meta/doc-citations-no-gate-checks-rot-silently` is citation
  rot — a pointer that stops resolving. This is a figure that resolves
  to nothing because it was never a pointer.
