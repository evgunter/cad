---
id: order-along-qualifier-records-no-partner-so-its-pruned-pair-vanish-cannot-be-recovered
kind: issue
title: Qualifier::OrderAlong records rank and of but no partner name, so a pruned-pair vanish of an OrderAlong fragment name cannot be shadow-executed and falls to the evidence-free RecipeEdit fallback
status: closed
opened: 2026-09-16
refs: [2755, 134]
priority: P1
cost: D
branch: emit/group-resized
pr: 3115
closed: 2026-09-23
---

Found by BOOL-7 (PR 2755) and filed by the S-BOOL orchestrator on
WIRE's slate (`crates/editor-core/src/names/` is WIRE's territory per
`scripts/work.py territory`). Issue 134's recovery rung re-executes a
vanished fragment name's own discriminator pair from the prior and
current evaluations and reports the flip. That is possible for
`Qualifier::SideOf` because the pair is written in the name — one
`(partner StableName, SideVerdict)` entry per seam partner — so both
sides can be re-probed from the two contexts alone. It is NOT
possible for `Qualifier::OrderAlong { rank, of }`: the qualifier
records a rank and a group size and no partner, so the pair the
fragment was ranked against cannot be read back out of the name, and
the pruned run has no sibling left to rank against either (a
single-member group runs zero `name_frag_order_along` pairs). The
corpus's own pruned-pair row (`fixture::pr4::diagnosis_corpus`
"flip-vanish": zero flips, 46 divergences, `name_frag_order_along`
8→0) therefore still diagnoses `RecipeEdit { NodeChanged }` — the
evidence-free fallback — and BOOL-7 pins that limit as a row
(`the_orderalong_half_of_the_issue_is_not_recovered`) and records it
at the rung's docs, the N5 paragraph, `m4_pr4_ci`'s pin comment and
`fixture::pr4`'s header. Closing this needs the naming vocabulary to
carry the partner (or an equivalent recoverable witness) for
`OrderAlong`, which is a names-lane design surface, not the rung's.
Measured, not acted on; difficulty M.
