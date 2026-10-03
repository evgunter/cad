---
id: DECIDE-1
kind: unit
title: the self-dot straddle: is any certification-path square still a product at Interval? (a census and a measurement)
status: closed
opened: 2026-09-21
priority: P1
cost: D
branch: decide/1-self-dot-census
refs: [interval-self-dot-straddles-before-rule-a]
closed: 2026-09-21
---

## What

M10-8's R1 NOTE-9 asserted that `v·v` on the certification path is an
interval PRODUCT (`Vec::dot` with one vector twice), so a straddling
component makes `sqrt(v·v)` a domain violation and clause 1 refuses
rule A before it is asked. The tree already spells every norm through
the tight square (`norm_squared`, M2 PR 4) and `Sym::powi` is the value
channel's, so the mechanism reaches a measured document only through a
hand-spelled self-product. Phase 1 is a static census of such sites at
`Interval` and a dynamic count of clause-1 `Invalid` refusals on the six
documents with their mechanism rendered; Phase 2 (the tight square at a
site, bit-identity pinned) only if Phase 1 finds one, else the row
closes on the measurement. Outside protocol v7 (triaged OUT: a census
and a measurement, the fix class ratified) — opus implementer, opus
reviewer, no draw, no row. Spec: `docs/DECIDE-1-SPEC.md`.

## Closed

**Phase 1 closed the row; Phase 2 was empty.** No self-product at
`Interval`/`Sym<Interval>` on a certification path a measured document
takes, and zero clause-1 `Invalid` over the decisions seen on all six
documents at ε = default, `1e-6` and `1e-12`, at the nominal, at
ceiling + δ and (R1's rows, adopted) out to twice the real study. The
two tables are PR #3001's; the verdict, what it rests on and the five
deviations are in
`work/decide/interval-self-dot-straddles-before-rule-a`'s `## CLOSED`
section and in the DECIDE log. `Vec::dot`, `powi` and
`scripts/gates/interval-square-allowlist.sh`'s logic are untouched.

**Residues, each on the slate it belongs to** (none left in prose):

- `work/shell/check-rigid-squares-a-column-by-multiplying-two-copies-of-it`
  — the one production self-dot at `Sym<Interval>` the census found,
  off every measured path.
- `work/guard/self-dot-has-no-gate-the-interval-square-one-cannot-see-it`
  — the interval-square gate cannot see `x.dot(x)`, so the static
  census is a one-shot nothing re-takes.
- `work/helper/per-predicate-fold-over-decisionshape-has-four-spellings`
  — R1's S1, the class behind this unit's own near-fifth copy.

Review: one Opus STYLE review, MERGEABLE-AFTER-FIXES, 0 MAJOR / 5 MINOR
/ 9 style, both hypotheses confirmed by execution; fix pass A–M, S9
declined. The spec is deleted at this merge with its
`docs/DOC-LEDGER.md` entry.
