---
id: a-boolean-reports-one-undeclared-contact-per-refusal
kind: issue
title: A boolean reports one undeclared contact per refusal, so n flush contacts cost the author n declare round trips
status: open
opened: 2026-09-30
priority: P3
cost: M
---


Found by both designers on AUTHOR's boolean-judge fork (#3587).

`topo::BooleanError::UndeclaredCoincidence` carries one pair. It is raised at the first classification site that meets an undeclared coincident contact (`reduce`, `rest`, `vtxfac`, `recl`), not by a pass that collects them all. So a union with n undeclared flush contacts refuses n times. The author declares one pair, re-evaluates, meets the next refusal, and so on: n declare actions and n+1 evaluations.

Measured by AUTH-9 (`two_contact` row in `crates/viewer/tests/combine_ops.rs`, a channelled block). Its viewer gesture accumulates the pairs across refusals so that nothing commits in between, but the author still sees them one at a time.

**Worth asking:** can the boolean report every undeclared pair it will meet in one refusal? Only the boolean knows which pairs it actually refuses. `find_flush_candidates` over-reports: two blocks apart on one ground plane give four pairs, and their union builds undeclared. So the collection has to be the boolean's own.

Owner: `crates/topo/src/boolean/` (TANG/ZIP).
