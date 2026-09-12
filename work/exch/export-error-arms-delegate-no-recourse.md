---
id: export-error-arms-delegate-no-recourse
kind: issue
title: two ExportError arms state a condition and stop — Class B of the census recourse sweep, in step-export
status: open
opened: 2026-09-12
refs: [2403]
---



Found by the FIX lane closing the `VolumeUncomputable` half of
`work/fix/validation-arms-delegate-a-recourse-their-carriers-do-not-give`
(PR 2403), under its sweep obligation. **Reported rather than filed by
that lane** — `crates/step-export/*` is EXCH's `paths` glob, and a lane
does not file on another program's slate. Placed here by the FIX
orchestrator, who verified both arms first.

## The class

An error arm whose `Display` names the CONDITION and stops, leaving the
reader without the repair. FIX's PR 2403 fixed nine such arms across
`BandError`, `MassPropsError` and `PropsError`, and made the claim
mechanical with three `every_*_arm_names_a_recourse` rows. These two
are the same shape on EXCH's ground.

Verified at `0f011d4`:

- **`crates/step-export/src/lib.rs`, `Corrupt { what }`** —
  `"step export: corrupt body ({what})"`. The whole message. A user is
  told their body is corrupt and nothing about what to do; the parallel
  arm FIX repaired (`MassPropsError::Corrupt`) now points at the
  tier-1/tier-2 report that names the actual violation.
- **`crates/step-export/src/lib.rs`, `NullScaffoldEdge { edge }`** —
  `"step export: edge {edge:?} is null-edge scaffolding — the body is
  mid-surgery (tier 2 refuses null entities at rest)"`. Better than the
  first: it names the condition AND its cause. It still does not say
  what to do, and here the repair is knowable — the body is mid-surgery,
  so the export is being attempted at a moment when the body is not at
  rest.

## Worth weighing before writing anything

The sibling arms in the same impl are **not** uniformly defective, and
that is the argument for reading rather than sweeping: the
no-printer arm ends *"(every described carrier prints; a placeholder
does not)"*, which tells a reader what to change. PR 2403's lane found
the same thing one crate over — `OffsetFitError`'s `BudgetExhausted`
and `SampleCapReached` already name their levers, and the item's
verb-match had called them defective. **A verb-vocabulary match over
string literals is a signal to read the arm, not a verdict on it.**

## The durable half

PR 2403's enforcement rows are the part worth copying, and one of them
is worth copying in its stronger form: `every_mass_props_error_arm_names_a_recourse`
is **transitive** — its delegating arms carry no prose of their own, so
it passes only while its carriers name recourses, and a defect two
crates away reddens it. That is a wrapper's assumption made to fail
loudly rather than documented.

Check first whether anything pins these two message texts; FIX found
that for two of its three carriers nothing did, and recorded the
missing pin as part of the defect rather than as a baseline.
