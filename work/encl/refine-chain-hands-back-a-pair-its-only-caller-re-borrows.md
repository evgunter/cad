---
id: refine-chain-hands-back-a-pair-its-only-caller-re-borrows
kind: issue
title: refine_chain hands back a (KnotVector, plans) pair its only caller immediately re-borrows
status: closed
opened: 2026-09-22
priority: P4
cost: E
rides_with: a-third-spelling-of-cut-every-span-into-splits-pieces
closed: 2026-09-26
---


Filed by TESS-2 from a blinded review of its head. Cosmetic; on ENCL's
slate because `crates/geom-brep/src/patch_bound.rs` is ENCL's path.

## What

`patch_bound::refine_chain` returns `(KnotVector, Vec<CurvePlan>)` by
value, and `rational_cells` then writes

```rust
let (kv_u, plans_u) = refine_chain(n.knots_u(), splits)?;
let (kv_v, plans_v) = refine_chain(n.knots_v(), splits)?;
let (kv_u, kv_v) = (&kv_u, &kv_v);
```

— the third line existing only to re-borrow what the first two just
moved, because the rest of the function takes `&KnotVector` (it is the
shape `integral_cells_on` and `DNets::build` want, and the shape
`integral_cells` passes straight through from a surface). It reads as a
lifetime workaround rather than as an intent.

`integral_cells_refined` does not need the line, because it hands the
vectors to `integral_cells_on(&nets, &kv_u, &kv_v)` once.

## Dispositions

Either is fine and neither is worth a PR of its own:

* have `rational_cells` name the borrows at the point of use
  (`&kv_u`, `&kv_v` in the two or three expressions that need them), and
  drop the rebinding line; or
* keep the rebinding and say in a comment that it is a borrow of the
  owned vectors and not a second pair.

TESS-2 left it as it stands rather than touch a working function for a
line of style during a fix pass.
