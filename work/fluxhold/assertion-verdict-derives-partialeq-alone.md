---
id: assertion-verdict-derives-partialeq-alone
kind: issue
title: AssertionVerdict derives PartialEq alone, so a reader holding one over an Eq scalar hand-writes Eq
status: parked
priority: P4
cost: E
opened: 2026-09-30
blocked_on: [an-assertion-relates-by-equality]
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

## Re-homed from FLUX to FLUXHOLD (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXHOLD holds FLUX's rows on the D10 hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Each waits on the INTENT unit that rebuilds its ground, named in `blocked_on`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
