---
id: area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step
kind: issue
title: A declared area-overlap cap contact is admitted without a merge, and the F7 gate refuses the two coplanar rows at the next boolean
status: open
opened: 2026-09-07
refs: [DOCM-8, 2073]
---

## What

R2's star fixtures (`r2_p2`, `r2_p2b`, `r2_p2c` on `docm/8-review-r2`:
`a` = x∈(0,1), `c` = x∈(0.5,1.5), `d` = x∈(1.2,2.2), all y∈(0,1);
`f` = x∈(0.5,1.5), y∈(0.5,1.5); all z∈(0,1)) report
`UndeclaredContact` for `(f.cap, a.cap)` or `(f.cap, d.cap)` in ten of
24 orders although those pairs ARE declared (`r2_p2c`). DOCM-8's fix
pass traced one refusing order, `[a, f, c, d]`, from bucket to
resolver to census (R-C):

- Step 0 (`a` ∪ `f`): the bucket holds `(a.cap, f.cap)` for both caps,
  the rewrite leaves both as rows, the resolver hands the kernel two
  `coincident_faces` declarations. Nothing is dropped. The step
  succeeds — and publishes NO `Merged` row: `a.cap` and `f.cap` stay
  two coplanar rows of the accumulation (`a` ∪ `f` alone, caps
  declared, lists twelve member faces and no merge).
- Step 1 (`c` joins): the pair boolean's F7 gate finds two coplanar
  adjacent faces of operand A — `a.cap` and `f.cap` — with no
  declaration covering them at this step, and refuses
  `UndeclaredCoincidence` on that same-operand pair, which
  `union_refusal` reports as `(f.cap, a.cap)`.

The same document as a pair chain — `Boolean(a, f, caps declared)`
then `Boolean(that, c)` — refuses the identical `UndeclaredContact`
`(FromB(f.cap), FromA(a.cap))` at the second boolean. So the door is
the kernel's, not the union's routing: a declared cap contact whose
faces overlap in AREA (a corner square) rather than meeting flush is
admitted by the rest door without a merge, and the operand that
results carries two coplanar adjacent faces that no later boolean
accepts and no declaration vocabulary can cover (a same-operand face
pair is `DeclareUnsupportedPair`).

## Why

Which faces the kernel merges is `topo`'s (`merge_faces.rs`, the
`rest` door in `boolean/rest.rs`) and out of DOCM-8's scope. Either
the admitted area-overlap contact should merge the two caps into one
`Merged` row (the flush case does), or the F7 gate should accept an
operand pair a prior step admitted, or the refusal should name the
construction rather than an undeclared contact.

## Where it stands

Open, for `topo`'s placement. Reproducer: R2's `r2_p2d` (fuses alone)
followed by any third member touching either block.

