---
id: one-plane-reads-a-brackets-ends-as-a-point-without-asking
kind: issue
title: injective.rs's one_plane decides two conics share one plane by comparing bracket endpoints, without asking whether the brackets certify
status: open
opened: 2026-11-14
priority: P2
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), reconciling the censuses with main `2c843a2e77`.

## What

`one_plane` (`crates/topo/src/injective.rs`, PR 6908's merge `b9f0193e7f`, `clear/seam-meridians`) decides whether two planar conics' stored frames hold one plane by comparing bracket endpoints exactly: `point(x)` is `x.lo() == x.hi()`, and `is` and `flipped` compare `y`'s ends with `x`'s. That is 10 endpoint reads in a function that never asks `is_certified()`. The certified-endpoint census moved `injective.rs` 144/59 → 154/59 on them.

The census's premise is that a refused bracket carries ordinary endpoints. If either frame's components can be refused, `one_plane` could read a refused point bracket as "one value" and then treat the two conics as coplanar for every member. GREEN did not establish whether a stored frame can carry a refused component. That is this row's question.

## Pinned meanwhile

GREEN pinned the roster at main's `(injective.rs, 154, 59)`, with a note naming this row for the 10 reads.

## Owed

Either argue at the site that a stored conic frame's components are never refused (and say so in the roster note), or ask `is_certified()` before reading the ends.
