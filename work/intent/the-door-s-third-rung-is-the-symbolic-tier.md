---
id: the-door-s-third-rung-is-the-symbolic-tier
kind: issue
title: D10 stage 4 PR D: rung 3 at the door: an unproven record is re-decided in the Sym lane with every variable a symbol; the box mitre measured
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [coincidences-are-recorded-at-one-door]
---

INTENT stage 4, PR D. Spec: `docs/INTENT-STAGE4-SPEC.md` §5.

`var_env_symbolic` binds every continuous variable as `Sym::param` whatever its tolerance; an unproven row is re-decided by a `Sym` replay of its node, matched by (node, site, ordinal, cell names), and is `Structural(PolynomialIdentity)` when its normal form is identically zero. Measure every box mitre (Ev, 2026-10-06: output definitions plus rung 3). Check `Ratio::eval` is exact at `Sym`. Closes `unproven-coincidence-lint-binds-every-variable-as-a-symbol` and `isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`.
