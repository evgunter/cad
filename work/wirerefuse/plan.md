# WIREREFUSE — the plan

WIRE's refusal tail: endings typed by the wrong D9 row, recourse labels spelled by hand or missing, and the eval doors' record and index plumbing.

Opened 2026-10-28 by splitting WIREREST and WIRETAIL. On 2026-11-02, PRs 6291, 6293 and 6297 closed `refusals-forwarding-…`, `a-loft-of-fewer-…` and `list-positions-…` (and WIRE's `loop-coordinates-narrowing-has-no-bound`).

## The slate

**18 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P3 | `eval-wiring-kernel-defect-endings-are-bug-only-typed-refusals` | M |
| P3 | `the-older-missing-rule-naming-variants-lack-the-unsupported-prefix` | M |
| P3 | `a-verb-door-hands-back-any-familys-record-and-the-reader-refuses-the-foreign-one` | M |
| P4 | `editor-core-refusals-spell-the-recourse-label-by-hand` | E |
| P4 | `a-bad-label-in-a-file-refuses-with-the-regenerate-recourse` | M |
| P4 | `node-error-kind-has-no-fieldless-projection` | M |
| P4 | `an-empty-pick-path-refuses-at-evaluation-though-both-doors-hold-it` | E |
| P4 | `an-indexed-reads-index-position-is-clamped-to-u8` (its u8 half fixed by PR 6297) | E |

## Order

- **The D9 row-typing rows come first:** `eval-wiring-kernel-defect-endings-…` and `the-older-missing-rule-…` (`list-positions-…` closed). Each re-types endings against D9's rows. Check each against main before dispatch, as WIREREST did on 2026-10-26.
- **The recourse rows are one unit:** `editor-core-refusals-spell-the-recourse-label-by-hand` and `a-bad-label-in-a-file-…` (`refusals-forwarding-…` closed). Both are about which recourse a refusal names and how it spells it.
- **The verb-door row** takes three `Emission` raises with it (PR 5502's report).

## Review posture

Inherits WIRE's (`work/wire/plan.md`): protocol v7, single reviews.
