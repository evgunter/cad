---
id: split-hands-out-a-body-without-running-tier-3
kind: issue
title: split_direct runs no tier-3 check on its output, so an invalid half is handed out silently (shell.rs's verb validates its own output)
status: closed
opened: 2026-09-28
priority: P2
cost: E
pr: 3797
parent: validate-passes-a-body-with-a-zero-width-slit-face
closed: 2026-10-02
---


Found by CONTACT-6's reviewer (2026-09-28). `splitting/mod.rs`,
`split_direct` (~735), hands its halves out without
`validate_geometric`. At base, a cut through a reversed-sense cavity
wall shipped a half failing `LoopRoleInverted`, and point-in-solid then
answered 1,274 false `Out` inside it (CONTACT-6). `shell.rs` closes its
own verb with `validate_geometric`, which is the precedent. No sentence
in `split`'s docs says that callers validate. Decide whether split
validates its own output, and what it costs.

## Answered, 2026-10-02 (TQUERY, PR 3797)

Moved from `work/hone/` by `git mv`, id unchanged. It asks the same
question as `validate-passes-a-body-with-a-zero-width-slit-face`, whose
measurement answers it. That run covered 573 split halves across topo
and sweep (the table is in that item):
- **Tier 2 runs always, typed.** `split_direct` gates each side with
  `validate_closed` and refuses `SplitFinishError::ResultInvalid`. It
  costs ~20 µs a half (12 ms summed over the suite), and no green row
  refuses.
- **Tier 3 is decided out.** `validate_geometric` costs ~0.5 ms median
  and ~7 ms p95 a half. It refuses 8 halves, every one cut from an
  operand that fails the same door: Euler-op fixtures whose scaffold
  edges (`ScaffoldAtRest`) split accepts today. Gating tier 3 would
  first need split to refuse such operands, which is a contract change.
- **The pseudomanifold door is out.** Designed pinch halves carry
  touching vertex copies with no `ContactRecords`
  (`split-halves-have-no-contact-records-so-no-pseudomanifold-self-check`).
- **This gate does not reach CONTACT-6's case.** That half failed
  `LoopRoleInverted`, a tier-3 finding, so the tier-2 gate would not
  have refused it. That class stays with tier 3's decision above.

## Closed (2026-10-02, PR 3797)

`split` runs tier-2 `validate_closed` on every side it returns and refuses `SplitFinishError::ResultInvalid` (first finding in words). Tier 3 and the pseudomanifold door are deliberately not run, for the measured reasons above; the pinch residue is `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check`.
