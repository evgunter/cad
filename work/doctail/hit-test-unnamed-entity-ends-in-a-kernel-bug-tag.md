---
id: hit-test-unnamed-entity-ends-in-a-kernel-bug-tag
kind: issue
title: "resolve/hit.rs: HitTestError::UnnamedEntity ends in 'this is a kernel bug', not the shared kernel-defect ending"
status: open
opened: 2026-10-17
priority: P4
cost: E
---

Found by the `wire-endings` lane's sweep for refusals that end in a
hand-written kernel-bug tag (the class of
`work/wire/eval-kernel-bug-refusals-end-in-a-tag`).

## Finding

`crates/editor-core/src/resolve/hit.rs`, the `UnnamedEntity` refusal's
sentence ends

```
"… naming emission is total, so this is a kernel bug",
```

It carries no `There is no way through` marker and no report. The
ending has one home, `geom_core::KERNEL_DEFECT_ENDING` ("There is no way
through: this is a kernel defect; report it").
`crates/editor-core/tests/m4_pr4_hit.rs` and
`crates/editor-core/tests/display_contract.rs` assert the words
`"kernel bug"` in its rendering (the `HitTestError` cases) and move with
it.

## Repair shape

**Superseded in direction (ENCL, 2026-10-20): read "Evidence: the repair is the shape D2 retires" below before acting on this section.** Ending these arms in `KERNEL_DEFECT_ENDING` is the shape the parent census retires.


End the sentence with ". {KERNEL_DEFECT_ENDING}" and re-pin the two
tests to the constant.

## Evidence: the repair is the shape D2 retires (ENCL, 2026-10-20)

`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` holds that a typed refusal ending in `KERNEL_DEFECT_ENDING` is wrong
in both of `[failure]`'s classes. `UnnamedEntity` rests on "naming
emission is total". If that holds, the state is class 2 and panics, with
its two tests moved to `#[should_panic]`. If it does not, the refusal
names its lever. Either way it does not take the constant.
`part-and-refactor-kernel-defect-endings-split-by-class`, on this
slate, carries the document model's sites that already append it.
