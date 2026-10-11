---
id: the-symbolic-door-s-budget-and-the-theorem-cache
kind: ruling
title: The coincidence door's symbolic budget (resident vs full pass) and whether the mitre earns a theorem cache
status: closed
opened: 2026-10-11
priority: P1
closed: 2026-10-11
pr: 4644
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
to say one thing"?

**Two designers (fork-log row 112) both say no.** Their recommendation:

- Delete the resident/full switch, so the door has one proving strength.
- Synchronous callers (`run_checks`, Python, CI) get the full verdict.
- The viewer runs the door at rest, off the draw path, as a
  generation-keyed job that the next edit cancels. Until it lands, the
  rows read as *pending*, which is a state of the report and not a
  finding.

They differ on two points:
- whether the strength is a bare kernel constant (A) or one
  `SymbolicDials` value type that tests may set but no product surface
  offers (B);
- whether "unproven" splits into *not an identity* and *undecided*
  (A), or stays one wording among the residual's five (B).

The orchestrator's two-pass ruling, which PR 4628 built, is not
ratified text. Retiring it changes nothing in D10.

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

## Decision (Ev, 2026-10-11, PR 4644)

**Q1: one strength (designer A's form).**
- The door proves at one strength, a kernel constant with no setting
  and no dials type. The resident/full switch
  (`ChecksConfig.symbolic_pass`, Python `full_symbolic_pass`) goes.
- Synchronous callers get the full verdict.
- The viewer runs the door at rest, off the draw path, as a job the
  next edit cancels. Its rows read *pending* until the job lands.
  *Pending* is a state of the report, not a finding.
- "Unproven" does **not** split. Ev, on an undecided arm for residuals
  that keep an opaque atom: "if there's an opaque numerical
  computation like that it's pretty fair to ask the user to assert
  that the two things are identical rather than making a special
  undecided case for it". On the frozen mitre form: "1 is
  unfortunate but since it'll hopefully be fixed i am not sure if
  it's worth a special case".
- So an unproven row is one finding. Its recourse names an assertion
  where the residual keeps an opaque atom, as D10 already says a
  contact beyond what the construction fixes is "recorded and linted,
  and its recourse is an assertion".

**Q2: SYM first; the cache stays parked.** SYM's
`the-mitres-feet-distance-freezes-at-sym` goes first.
`a-construction-s-identities-are-a-generated-cache` stays parked unless
that work fails. Ev: "since it'll hopefully be fixed".

PR 4628 merges as built. The successor unit is
`the-symbolic-door-proves-at-one-strength-at-rest`.
