---
id: strut-side-labels-are-provisional-in-join-and-derived-in-vtxfac
kind: issue
title: join.rs calls pierce-ring strut labels provisional; vtxfac mints them as derived sense data
status: closed
opened: 2026-10-08
priority: P4
cost: E
branch: flush/e-tail
pr: 4714
closed: 2026-10-11
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

## Fix (branch `flush/e-tail`)

The join's doc was the stale one. Every strut's halves, a ring strut's
included, are minted by `Body::mev_null_run` under the sense theorem
(`vtxfac.rs` step 3; the module docs of `join.rs`, item 2), so the
labels are derived, not provisional. `resolve_roles_geometric` still
does not read them, for a reason the doc now gives: the labels fix
which way each loop's chords run, and the zip's
`BooleanError::SeamOrientation` compares that against the roles; read
from the labels, the roles would make that check compare the labels
with themselves. The "single-face seam ring has no in-solid label"
clause went with the stale one: every null half there carries a label
by the same rule.

## Closed (2026-10-11, PR 4714)

Fixed in the E-tail batch, PR 4714 (single FULL review, APPROVE-WITH-FIXES; every fix taken). The PR body has a section for this row.
