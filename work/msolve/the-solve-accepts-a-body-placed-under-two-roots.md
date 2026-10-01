---
id: the-solve-accepts-a-body-placed-under-two-roots
kind: issue
title: the mate solve accepts a document whose gather refuses PlacedUnderTwoRoots, so the two layers still disagree about one instance mated through two transform roots
status: open
opened: 2026-09-24
priority: P1
cost: M
refs: [3142]
design: true
needs_ev: true
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

## Weighed (2026-10-01)

Two designers weighed this on `msolve/ev-two-roots` and agreed on
their first reports. The solve should not refuse: it answers where
each instance is, and the gather answers whether the roots make one
product. The same shape with no mates fails the gather identically,
and the solve already returns poses for every other product-less
document. `ASSEMBLY.md` A11 (4) gains the sentence that says so.

Both designers also found that `PlacedUnderTwoRoots`'s recourse
("place it under one root, or union the two") is the repair for a
body document. For an instance, the repair is a second instance or a
pattern. One designer traced a union over two transforms of a mated
instance into `RefusedRef::ReadBelowARoot` (read, not run). That
rewording rides the unit that lands the sentence, after a row pins
the trace.

Brief corrections: `work/gather/` left the tracker
(`docs/doc-ledger/gather-leaves-the-tracker.md`). The viewer never
solves before it gathers: it lands through the product and badges
this refusal on the frame. Only Python's `solve_document` exposes the
solve bare.

