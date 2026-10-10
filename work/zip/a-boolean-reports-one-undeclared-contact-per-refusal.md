---
id: a-boolean-reports-one-undeclared-contact-per-refusal
kind: issue
title: A boolean reports one undeclared contact per refusal, so n flush contacts cost the author n declare round trips
status: closed
closed: 2026-10-09
opened: 2026-09-30
priority: P3
cost: M
---


Found by both designers on AUTHOR's boolean-judge fork (#3587).

`topo::BooleanError::UndeclaredCoincidence` carries one pair. It is raised at the first classification site that meets an undeclared coincident contact (`reduce`, `rest`, `vtxfac`, `recl`), not by a pass that collects them all. So a union with n undeclared flush contacts refuses n times. The author declares one pair, re-evaluates, meets the next refusal, and so on: n declare actions and n+1 evaluations.

Measured by AUTH-9 (`two_contact` row in `crates/viewer/tests/combine_ops.rs`, a channelled block). Its viewer gesture accumulates the pairs across refusals so that nothing commits in between, but the author still sees them one at a time.

**Worth asking:** can the boolean report every undeclared pair it will meet in one refusal? Only the boolean knows which pairs it actually refuses. `find_flush_candidates` over-reports: two blocks apart on one ground plane give four pairs, and their union builds undeclared. So the collection has to be the boolean's own.

Owner: `crates/topo/src/boolean/` (TANG/ZIP).

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/flush/plan.md`, "The intent-refactor hold"). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: UndeclaredCoincidence refusals retire at stage 4 and become unproven-coincidence findings, so the one-pair-per-refusal shape goes with them. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed (2026-10-09, INTENT stage 4 E (`intent/s4-e-glue-on-zero`))

`BooleanError::UndeclaredCoincidence` is deleted. A flush contact decided Zero now glues undeclared: the boolean declares every such pair at its entry (`crates/topo/src/boolean/glue.rs:40`, `:72`–`:78`). A union with n flush contacts therefore builds in one evaluation, and nothing is left to declare. AUTH-9's two-contact row is now `a_union_across_two_flush_contacts_lands_as_one_action` (`crates/viewer/tests/combine_ops.rs:3327`): the union lands as one action, and the refusal it accumulated across is gone.
