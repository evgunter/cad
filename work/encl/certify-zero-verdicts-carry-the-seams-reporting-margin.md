---
id: certify-zero-verdicts-carry-the-seams-reporting-margin
kind: issue
title: geom-core/geom-brep: every outcome of the classify seam carries its margin for error reporting only, and certify's Zero arms quote it (Ev, [ev] PR 3402)
status: dispatched
branch: encl/zero-verdicts-reporting-margin
priority: P2
cost: M
opened: 2026-09-29
---


(Filed by the ENCL orchestrator on Ev's ruling on `[ev]` PR 3402, 2026-09-29, which took designer A's form.) The ruled text is the `Bounds` scope rule's clause 2 in `crates/geom-core/src/real.rs`, together with D4 ¶1 (i).

## What

Ev's condition, verbatim: "as long as the change in 2 makes it structurally obvious that it's not ok to use this number for anything but error reporting … presuming it can be set up so you have to do something obviously wrong to make a decision based on that number".

1. **geom-core.** `Decide::sign_within` returns `Result<Decided, Indeterminate>` with `Decided { sign, margin }`, and `k_stats::decide` returns `Decided`. The margin is minted where `MarginDiag` is minted today.
2. **The fence.** The margin type has no bare numeric accessor that a comparison could read in passing:
   - the recourse wording (whether it `tightens`, and the `m/K` it quotes) is computed inside geom-core and rendered;
   - any escape to an `f64` is one visibly named, error-text-only door that a gate counts, so that a decision on it takes an obviously wrong step.
   - Apply the same fence to `Indeterminate`'s margin view, so there is one reporting-margin type.
3. **recourse.** `Classified` carries the reporting margin and its band.
   - Delete `RefusedArm::Zero(None)`, `tighten(None)`, `Definite`, and `Refused`'s reliance on a `Bounds::lo` read (`ssi/certify.rs`).
   - The Zero arms of `NotTransverse`, `NotSecondOrderSeparated`, `TubeNotSeparated`, `IntervalNotForward` and the lane's `NotTransverse` carry it. `certify_via` keeps it.
4. **The span.** `ParamSpan`'s Zero arm routes through the table:
   - 0 < m: the lever plus the valued offer;
   - m ≤ 0: the lever plus a reading-aware `at_zero` note ("no kernel construction mints a zero or reversed span": a kernel bug at a build, a kernel-or-file defect in stored geometry).
   - Rewrite the `IntervalNotForward` docs, and retire "a zero span is always a defect, not data". It was co-authored fix-pass text (`e1600790f9`), not an Ev ruling. PR 3392 restored it on a wrong attribution.
5. **Structured verdict for the doors.** `CertifyError` exposes `(CertCheck, RefusedArm)`, and `SizedDecision` exposes its size and lever, so the import door (`adoption-certification-reads-as-at-rest-with-the-eps-in-stopgap`) composes from the verdict rather than from a string.
6. **Stale reason.** Correct `CertifyError`'s doc header "no f64-projection of a generic T exists for every lane". Designer B's open point, the sign-certain residual carrying its miss, is covered by A's every-outcome form.

Re-pin: certify, `validate::certify_escalation_rows`, editor-core `refusal_concision_chains`, and the `tier_gate` poleband cells.

**Pins required:**
- a pin that fails on any unvalued tighten;
- a gate or compile-fail pin showing that a decision on the reporting margin needs the named door.

Design record: `[ev]` PR 3402 and `docs/DESIGN-FORK-LOG.md` row 10.
