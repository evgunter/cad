---
id: the-symbolic-door-s-budget-and-the-theorem-cache
kind: ruling
title: The coincidence door's symbolic budget (resident vs full pass) and whether the mitre earns a theorem cache
status: open
opened: 2026-10-11
priority: P1
needs_ev: true
---


Two questions from INTENT stage 4 C+D (`carriers-compare-in-canonical-form`,
PR 4628). The code is built as the orchestrator ruled, pending this.

## Q1. One door, two budgets

The coincidence door replays a contact row symbolically to prove it.
Measured cost at the full budget (256 terms plus the kept-atom retry
ladder), in release:

| document | door time per evaluation | full-pass result |
|---|---|---|
| die_composed_tour | 26.2 s | 21 of 42 joint rows proven |
| die | 7.2 s | none of 21 plane-ladder rows proven |
| corner_table | 3.7 s | none of 8 proven |
| kitchen_sink | 2.1 s | none of 4 proven |

Debug builds are about 10× slower. The viewer re-runs the default
checks after every evaluation.

A small resident budget (64 terms, no ladder) proved 0 corpus rows and
added 25 % to the gallery test. As built, the door is therefore:

- **one door** with a budget parameter;
- **resident** (the default after each evaluation): rung 0 only;
- **full pass**: the same door at 256 terms plus the ladder, chosen
  through `ChecksConfig.symbolic_pass` (Python: `full_symbolic_pass`).

A row the resident did not discharge says "unproven at the resident
budget; the full symbolic pass decides". That wording is distinct
from the full pass's own "unproven" and from "not replayed". All three
are pinned in tests.

The question: does a budget parameter on one door fit D10's "one way
to say one thing"? The alternative would be one budget everywhere. At
the full budget that costs the hot-path times above. At the resident
budget nothing past rung 0 is ever proven.

## Q2. The mitre and a per-node theorem cache

Your ruling of 2026-10-06 (on
`isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`): a
per-node theorem is kept "only as a possible cache … added only where
the algebra is measured too slow or undecidable". D10 adds that it is
generated from the symbolic evaluation and checked against it.

The measurement is now in:

- A box mitre's margin closes its angle term at every budget from 64
  to 4096 terms, but freezes on the feet distance (three opaque atoms).
  That is 0 of 8 rows proven, at 53–172 s per box.
- A disc's blend joints prove only from 1024 terms, at 1.6 s per disc.

Rows filed in PR 4628:
- `work/sym/the-mitres-feet-distance-freezes-at-sym.md` (SYM tier work
  that would make the cache unnecessary);
- `the-blend-joints-prove-only-past-the-door-s-budget`;
- the cache itself, parked: `a-construction-s-identities-are-a-generated-cache`;
- `the-full-symbolic-pass-runs-off-the-hot-path`, which would make the
  full pass affordable without a cache.

The question: does the mitre meet your bar now, so that the cache is
built? Or does SYM's tier work on the feet distance go first, with the
cache staying parked unless that fails?
