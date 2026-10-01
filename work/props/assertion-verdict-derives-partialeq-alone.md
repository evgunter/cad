---
id: assertion-verdict-derives-partialeq-alone
kind: issue
title: AssertionVerdict derives PartialEq alone, so a reader holding one over an Eq scalar hand-writes Eq
status: open
priority: P4
cost: E
refs: [an-assertion-row-shows-no-verdict]
opened: 2026-09-30
---


Found by AUTH-8. `AssertionVerdict<T>` (`crates/editor-core/src/measure.rs`,
~:752) derives `Debug, Clone, PartialEq` and not `Eq`. A derived `Eq`
on a generic enum is bounded `T: Eq`, so adding it costs `f64` readers
nothing and gives `AssertionVerdict<String>` its `Eq`; `UnevaluatedReason`
is `Eq` already.

The viewer's `tree::Asserted` holds an `AssertionVerdict<String>` (the
numbers spelled) inside `TreeRow`, which is `Eq`, and so carries a
hand-written `impl Eq for Asserted {}` (`crates/viewer/src/tree.rs`,
beside `Asserted`). Adding `Eq` to the kernel's derive retires that impl.
