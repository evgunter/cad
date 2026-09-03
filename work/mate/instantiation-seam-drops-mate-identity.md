---
id: instantiation-seam-drops-mate-identity
kind: issue
title: The instantiation seam carries contact records but not mate identity or mint health (the PartValue channel)
status: open
opened: 2026-08-31
github: 1429
refs: [1420]
---

## From GitHub issue 1429

opened 2026-08-31, 0 comments.

Filed from the MATE-6 dual review (PR #1420; the finding is bilateral, and one half narrows the Q1 ruling's letter — **flagging that for Ev explicitly**). Two faces of one gap, both loud today, neither wrong:

1. **A carried declaration's refutation cannot name its mate.** The Q1 ruling's sentence (S-MATE plan §Rulings 1) says a refuted carried declaration lands "`StaleContactDeclaration → Refuted` naming its mate"; what ships is `ContactContradicted` + `Attribution::Unattributed` — the refuting arm is arguably *better* vocabulary (definite counter-evidence, not an absent witness), but the attribution is anonymous because `attribute` matches against what THIS document minted and `MintedDeclaration` rows do not cross the seam (`PartValue` at `eval/parts.rs:342` carries `contacts`, drops `minted`/`unminted`).
2. **Inner mint refusals stop at the seam** (R2's probe P8): an inner document whose only mate is unmintable refuses its own `assemble`, but instantiated into an outer document the outer gate is `Ok` — the outer gate re-verifies carried *records*, not sub-documents' mint health. Identical to pre-MATE-6 behavior by construction; recorded as the bound on "verification runs once at the outermost gate."

Fix shape named by the unit: a `PartValue`/`NodeValue` channel carrying `MintedDeclaration` (and possibly `unminted`) across the seam under the graft's descendant map, letting `attribute` name carried mates and letting the outer gate see inner mint health. A semantics decision rides it (is an inner mint refusal the OUTER document's error?) — likely a short Ev conversation before implementation. S-MATE backlog; not scheduled as a unit yet.

Signed: (S-MATE orchestrator)

## Home

`work/mate/` — the instantiation seam and the mate mint/attribution channel are S-MATE's charter (assembly composition, the instantiation seam), and the issue names S-MATE's backlog.
