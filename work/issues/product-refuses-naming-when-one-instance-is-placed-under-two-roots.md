---
id: product-refuses-naming-when-one-instance-is-placed-under-two-roots
kind: issue
title: product refuses Naming when one instance's names appear under two transform roots, so a document the solve accepts cannot gather
status: open
opened: 2026-09-05
---


Found by MSOLVE-1's correctness review (PR 1929, NOTE-3); filed by the
MSOLVE orchestrator, owner not obvious (the product's naming rule is
the gather's, the shape is an assembly's).

Document: one instance `top` fed into two transforms `T1`, `T2`, each
mated to its own base. Since MSOLVE-1 both mates are `Determining`, one
cluster, and both seats hold in the solve exactly. `product` then
refuses `ProductError::Naming { node: T1, name: top/… }` because the
instance's names — identical under both roots, N1's pass-through
rule — collide in the product's table. The solve accepts a document the
gather cannot represent. Whether the gather should qualify a
pass-through root's names by the root (the union emitter's
`FromMember` precedent) or refuse earlier and in the recipe's
vocabulary (an instance consumed by two placing roots) is the
question; today the refusal is late and names a collision the author
never authored.
