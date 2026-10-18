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

End the sentence with ". {KERNEL_DEFECT_ENDING}" and re-pin the two
tests to the constant.
