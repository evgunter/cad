---
id: main-red-the-copysign-mint-census-misses-the-plane-cone-graze-site
kind: issue
title: Main red since PR 5111: the copysign mint census misses the plane × cone graze site in geom-brep/src/intersect.rs
status: open
opened: 2026-10-15
priority: P1
cost: E
---

Found by FLUSH's PR 5119, whose `test` job went red on it.

## What

PR 5111 (TANG U1, `tang/pinch-node-ring`, merged 2026-10-14) added a
`copysign` mint site to `crates/geom-brep/src/intersect.rs` (the plane ×
cone graze). `geom-core`'s
`sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these`
pins the tree's copysign sites, so it goes red on main. The per-PR gate
did not run it on 5111, which touched no `geom-core` path. The test
lives in TCOST/TINT's territory.

Message: "the copysign mint sites moved: a site appeared or went.
Re-ask the census (`m10_10_evidence_interval`'s
`sym12_the_copysign_census_at_the_nominal`) on the measured documents,
then move this table and `manifest.rs`'s header list together." It is
red on `origin/main` `9abb5196f` with that message.

## Ported

PR 5119 carries the fix as its own commit (`ec130c108`, on
`flush/shell-role-reader`). It re-asks sym12's census on the seven
documents with rule F on and off (0 copysign atoms in decided residuals
on every one, and on the bracket and link), and adds the site to the
table and the header list. The commit no-ops once main carries it.

## Owed

Close this when PR 5119 merges, or land the same change first if 5119
waits. TANG: check that the header's claim about the site's
justification matches what U1's graze intends.
