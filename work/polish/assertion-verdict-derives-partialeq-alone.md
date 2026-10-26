---
id: assertion-verdict-derives-partialeq-alone
kind: issue
title: The viewer's hand-written Eq for Asserted rests on an Eq reason that UnevaluatedReason no longer is, and the kernel derive this row asked for cannot supply one
status: open
priority: P4
cost: E
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

## Re-homed from FLUX to FLUXHOLD (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXHOLD holds FLUX's rows on the D10 hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Each waits on the INTENT unit that rebuilds its ground, named in `blocked_on`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Re-read (2026-10-26, tracker sweep): the remedy is gone, the question is not

The fix above (derive `Eq` on `AssertionVerdict<T>`, bounded `T: Eq`)
no longer gives anything: its premise, "`UnevaluatedReason` is `Eq`
already", is false on main. `UnevaluatedReason`
(`crates/editor-core/src/measure.rs:530-533`) derives `PartialEq` only,
with the comment "`Eq` is not derived: the undecided arm keeps the
escalation, whose reporting margin is floating point" — its
`Indeterminate` arm carries a `geom_core::Indeterminate`
(`crates/geom-core/src/predicate.rs:2177`, `PartialEq` only). So no
`AssertionVerdict<T>` is `Eq` for any `T`, and the derive would retire
nothing.

What stands is the viewer's `impl Eq for Asserted {}`
(`crates/viewer/src/tree.rs:440-442`), whose comment still argues it
from "an `Eq` reason". Owed: decide whether `Asserted`'s `PartialEq` is
an equivalence over an `Unevaluated { Indeterminate }` verdict (whether
any stored margin can compare unequal to itself), then either state that
argument in the comment or compare that arm by something that is `Eq`.
Not D10-held any more: the ground is the viewer's tree row, so this row
belongs on a viewer program's slate (CHROME or POLISH claim
`crates/viewer/src/*`).

## Moved (2026-10-26, tracker sweep)

From FLUXHOLD, which closed (`docs/doc-ledger/fluxhold-leaves-the-tracker.md`); this program claims the file the row is about.
