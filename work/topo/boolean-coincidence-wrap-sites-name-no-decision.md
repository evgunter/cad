---
id: boolean-coincidence-wrap-sites-name-no-decision
kind: issue
title: topo: BooleanError::coincidence is a one-word default, so a new escalation gets the declare menu without naming its decision
status: open
opened: 2026-09-30
---


(TOPO, PR 3493's review, NOTE-13.)

## What

`BooleanError::coincidence(diag)` (`crates/topo/src/boolean/mod.rs`,
`impl BooleanError`) wraps an escalation as
`BooleanDecision::Coincidence`, which renders the coincidence sentence
and offers "declare the coincidence". About 55 wrap sites use it
(`reduce.rs` 14, `sectors.rs` 12, `mod.rs` 10, `join.rs` 5, `recl.rs`
4, `insert.rs`, `ops.rs` and `vtxfac.rs` 3 each, `rest.rs` 2, counting
the explicit `BooleanDecision::Coincidence` spellings too). Because it
is one short word, a new escalation site reaches for it by default:
the compiler asks the author for a decision, and the default answers
for them. That is how the Boolean kept routing decisions no face-pair
declaration names (`boolean-coincidence-route-holds-decisions-no-face-pair-names`)
under the declare menu.

## Repair shape

Have each site name the decision it wraps. Either remove the helper
and spell `BooleanError::Escalated { decision, diag }` at every site,
or give it a required argument naming which coincidence the site asks
about (the plane identity rung, the tangent locus, the sector
classification, …), so that choosing `Coincidence` is a decision the
author states rather than one the helper makes. Sites that turn out
not to ask about a coincidence get their own `BooleanDecision` variant
under the sibling row above.
