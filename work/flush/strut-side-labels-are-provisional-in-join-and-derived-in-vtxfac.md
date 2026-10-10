---
id: strut-side-labels-are-provisional-in-join-and-derived-in-vtxfac
kind: issue
title: join.rs calls pierce-ring strut labels provisional; vtxfac mints them as derived sense data
status: open
opened: 2026-10-08
priority: P4
cost: E
---


Found by PR 4300's review (s6), 2026-10-08.

## What

`boolean/join.rs` `resolve_roles_geometric`'s doc says "Strut side
labels are never consulted: pierce-ring struts carry provisional
labels". `boolean/vtxfac.rs` step 3 mints each ring strut's attribute
through `Body::mev_null_run` as derived sense data (the join module's
sense theorem: the half facing the run's start germ is DOWN), and the
join's own module docs call the attributes "the discipline; nothing
rebinds later". One of the two statements is stale. The tree ring (PR
4300) does not settle it: every strut, at the ring vertex or at a
parent's far end, is minted by the same rule.

## Owed

Read whether `resolve_roles_geometric` still needs to avoid the labels.
If it does not, make its doc say the labels are derived; if it does,
say what makes a ring strut's label untrustworthy.
