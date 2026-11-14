---
id: subtract-complements-b-is-spelled-at-each-boolean-site
kind: issue
title: ∖ complementing B's material is spelled as op == Subtract at each Boolean site that needs it, with no BooleanOp method naming the fact
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by the ONEDOOR orchestrator on 2026-11-14, from the designers' residue on `a-subtract-tokens-chart-agreement-reconciles-revert-by-hand`.

"Subtract keeps B's complement" is the Boolean's own fact and does not depend on the surface kind. It is spelled `op == Subtract` at `crates/topo/src/boolean/finish.rs:289` (`b_kept = b_kept.reverted()`), `holdings.rs:311`, `join.rs:1095` and, once the revert door lands, the token in `ops.rs`'s `OneRecordTokens::of_result`. Each site restates the fact rather than asking it. A `BooleanOp` method that names it (beside `kept_side`, `finish.rs:77`) would give the question one home. P4: the spellings cannot disagree today, since each is the same comparison.
