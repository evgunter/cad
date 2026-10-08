---
id: the-door-s-third-rung-is-the-symbolic-tier
kind: issue
title: D10 stage 4 PR D: rung 3 at the door: an unproven record is re-decided in the Sym lane with every variable a symbol; the box mitre measured
status: closed
opened: 2026-10-08
priority: P0
cost: M
closed: 2026-10-08
refs: [carriers-compare-in-canonical-form, 4322]
---

INTENT stage 4, PR D. Spec: `docs/INTENT-STAGE4-SPEC.md` §5.

`var_env_symbolic` binds every continuous variable as `Sym::param` whatever its tolerance; an unproven row is re-decided by a `Sym` replay of its node, matched by (node, site, ordinal, cell names), and is `Structural(PolynomialIdentity)` when its normal form is identically zero. Measure every box mitre (Ev, 2026-10-06: output definitions plus rung 3). Check `Ratio::eval` is exact at `Sym`. Closes `unproven-coincidence-lint-binds-every-variable-as-a-symbol` and `isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`.

## Closed — folded into `carriers-compare-in-canonical-form`

Ev approved FORK-S4F in PR 4322 (fork log row 93): the door replays the
document at `Sym` and proves by two rungs, the second of which is this
unit's margin identity, and units C and D merge. Everything above
(`var_env_symbolic`, the cross-lane row match, the `Ratio::eval` check,
the box mitre measurement, and the two issues it closes) is now part of
`carriers-compare-in-canonical-form`, which carries the work. Nothing was
built under this id.
