# HONE log

## Opened at REACH's cut (2026-10-01)

Opened by the REACH orchestrator at its first sitting. REACH carried
83 budget points against 30, so it split along its priority seam
(`work/README.md`, Track size). Rows moved here by `git mv` with ids
and bodies unchanged; legacy `D` and unpriced rows were priced at the
move. No unit dispatched. — (REACH orchestrator)

- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`, the last unit of that program): the D4 ¶1 (i) recourse GRAMMAR moved in `geom-core`, so refusal text changed across the tree. `COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE` and `SPLIT_PLANE_RECOURSE` lost their unvalued `", or lower the tolerance"` tail and are now the LEVERS alone; `DEFINITE_COINCIDENCE_RECOURSE` retired into `COINCIDENCE_RECOURSE` (with the tail gone the two were one string). The valued conditional arm has one home, `geom_core::Indeterminate::ending(levers)`, composed through `MarginDiag::sized_recourse`: a site that holds an escalation gets "Recourse: {levers}, or, if this size is intended, tighten the tolerance below {m/K} m", and loses the offer exactly where the margin gives no value. `Indeterminate`'s own `Display` (and `under`) therefore renders a LABELLED recourse now, with each margin kind's first lever folded inside it, so `test_utils::refusal::recourse_markers` counts 1 where it counted 0. `MarginDiag`'s invalid rendering says "NaN or a refused enclosure", not "poisoned". Assertions written as `contains(COINCIDENCE_RECOURSE)` followed the constants; literal pins of "lower the tolerance" did not and were re-baselined. (PROPS implementer)
- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`): mechanical edits in the Boolean's files for the recourse-grammar change. `topo/src/boolean/mod.rs` and `refusal_routes.rs` use `geom_core::COINCIDENCE_RECOURSE` where they used the retired `DEFINITE_COINCIDENCE_RECOURSE` (same string). `topo/src/boolean/contain.rs`' `ContainError::RayExhausted` drops "or lower the tolerance", keeping "move the point off the boundary" — it carries no margin, so there is no value to offer and D4 ¶1 (i) offers nothing unvalued. `solid_contain.rs`' props pattern gained `..` for `PropsError::Escalated`'s new `check` field. (PROPS implementer)
