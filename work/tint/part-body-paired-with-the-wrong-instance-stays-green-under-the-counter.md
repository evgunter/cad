---
id: part-body-paired-with-the-wrong-instance-stays-green-under-the-counter
kind: issue
title: A part body paired with another part's instance stays green while every part mints its body under one id
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [part-suites-name-every-parts-body-by-one-constant]
---


## What

`fixture::resolver::in_part(instance, body, cap)`
(`crates/editor-core/tests/fixture/resolver.rs`) takes the body from
the caller. Nothing checks that `body` belongs to the part that
`instance` instantiates. A suite that pairs `top` with `base_body`
still names a real face, because under the counter every one-block
part mints its extrude as the third insert. So the mix-up stays green
on today's tree, and it goes red only under a per-document mint (the
probe in `part-suites-name-every-parts-body-by-one-constant`'s Closed
section) or after unit 2 of
`sibling-branches-mint-one-node-id-for-different-nodes`.

## Fix

Have the store answer for the body. `PartStore::insert_part` records
`body` against the document's id. A checked constructor reads the
instance's `DocRef` off the assembly document, asks the store for that
document's body, and builds the name, for example
`store.in_part(&asm, instance, cap)`. Then a suite cannot pair them
wrongly, because it no longer supplies the body. Moving the ~300 call
sites across 30-odd suites onto it is the cost, which is why it was
not folded into the row that threaded the body.
