---
id: an-in-band-holding-refusal-lands-by-member-order
kind: issue
title: Where an n-ary union's in-band holdings refusal lands depends on member order: which pieces the fold makes does
status: open
opened: 2026-11-02
priority: P2
cost: M
---



## Finding

The kernel's n-ary holdings (`topo::NaryStep::holdings`, read by
`lift` in `crates/topo/src/boolean/nary.rs`) are the geometric truth
in every member order. But which pieces a fold makes depends on the
order, and so does where an in-band `topo::overlap` reading lands. One
member order can therefore publish `Held::Refused` for a finished face
whose holdings another order reads.

This is the residual the item
`a-union-whose-fold-glues-a-run-no-judgement-opened` asked to be stated
beside `[operations.operators]`. It is stated in the n-ary door's
module docs ("What depends on member order").

For the naming layer (PR 6381), this means a name built on a face's
holdings can refuse in one order and not in another, at the same
finished body. What is open: whether naming should read a refusal as a
typed refusal of the whole union in every order (look for any refusal
across orders, which one run cannot), or accept the order-dependence
beside the carrier-description one DESIGN already allows. That is a
design question for REFS, and through it for Ev.
