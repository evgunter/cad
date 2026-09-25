---
id: the-solve-accepts-a-body-placed-under-two-roots
kind: issue
title: the mate solve accepts a document whose gather refuses PlacedUnderTwoRoots, so the two layers still disagree about one instance mated through two transform roots
status: open
opened: 2026-09-24
priority: P1
cost: D
refs: [product-refuses-naming-when-one-instance-is-placed-under-two-roots]
---

Filed by GATHER's two-roots lane (branch `gather/two-roots-refusal`),
on MSOLVE's slate because the solve is MSOLVE's ground
(`crates/editor-core/src/mate/solve.rs`, `solve_document`).

**The finding.** One instance `top` fed into two transforms `t1`, `t2`,
each mated to its own base, is a document the solve accepts: no fault
on any instance or mate, and both mates `Determining`. The product
gather now refuses the same document from the recipe, as
`ProductError::PlacedUnderTwoRoots { placed: top, select: None, first:
t1, second: t2 }` (`product.rs`, `placed_under_two_roots`), before any
root is read. The row that pins both halves is
`crates/editor-core/tests/gather_placed_under_two_roots.rs`,
`one_instance_mated_through_two_transforms_solves_and_refuses_at_the_gather`.

So the refusal is now early and in the recipe's words, but the two
layers still disagree: the solve returns poses for a document whose
product does not exist.

**The question for MSOLVE.** Should the solve refuse too? The case
against: the solve's answer is well-posed on this document (both seats
hold), and a second copy of the gather's rule inside the solve is a
second truth about what a document's product is. The case for: a
viewer or assembly door that solves and draws before it gathers shows
the author a green solve over a document with no product. If the solve
does refuse, it should call the gather's predicate rather than restate
it: `placed_under_two_roots` is private to `product.rs` today and would
need to become `pub(crate)`, with a typed `MateFault` arm that carries
the same four fields.

