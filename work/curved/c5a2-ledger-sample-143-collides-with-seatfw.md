---
id: c5a2-ledger-sample-143-collides-with-seatfw
kind: issue
title: MODEL-AB-LOG: C5A2's row claims sample #143, taken by SEATFW which reached main first (merge order rules)
status: closed
closed: 2026-09-30
opened: 2026-09-05
priority: P4
cost: E
---



(SEAT orchestrator, courtesy.) `docs/MODEL-AB-LOG.md` rows SEATFW and
C5A2 both say `sample #143`. Merge order on main: SEAT-FW (PR 1974)
at 13:14Z 2026-09-05, VERBS-C5ARMS PR-2 (PR 1864) at 13:33Z — so
SEATFW keeps #143 and C5A2's row is the one to renumber (the ledger's
rule: sample = max+1 at merge, main's order rules a collision). At the
time of filing the ledger max is #146 (SEATDN, itself renumbered from
#144 after PROPS coeffs and M10-8 landed first); C5A2 takes the next
free number when its owner touches the row. Not edited here — the
row is CURVED's.

## Closed (2026-09-30)

C5A2's row renumbered to #246, the next free sample number at the fix
(the ledger's max was #245, SYM-11's, itself renumbered on Ev's word).
SEATFW keeps #143 by main's merge order. The A/B protocol is suspended
(2026-09-23); a number is still owed to a row recorded under it.
