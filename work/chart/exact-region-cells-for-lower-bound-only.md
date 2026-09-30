---
id: exact-region-cells-for-lower-bound-only
kind: issue
title: Certified::LowerBoundOnly retires only with exact-region cells, not with a tightened window
status: open
opened: 2026-09-13
refs: [clearance-window-tightening-needs-chart-boundary]
priority: P1
cost: H
---



## What

`editor_core::measure::WINDOW_TIGHTENING` named this item's parent
(`clearance-window-tightening-needs-chart-boundary`) as the recourse
that RETIRES `UnevaluatedReason::WindowSuperset` — the refusal an
assertion gets when its verdict would be read off the endpoint
`min_clearance` certifies for the carrier rather than for the faces.
The parent does not retire it, and TRIM-3's survey measured why
(spec `docs/TRIM-3-SPEC.md`, refutation 8 — deleted at PR-2's merge, recoverable at the SHA `docs/DOC-LEDGER.md` names). PR-2 corrected the
const's doc to promise only what it delivers; this is the recourse it
stopped promising.

## The measurement

A tightened window is still a WINDOW. `window_of` cuts the root to
the metred hull of the face's chart boundary and the sweep drops
cells the boundary certifies empty of face — but a cell that
STRADDLES the boundary, or sits within `K · eps` of it, is kept, by
construction: `certifies_outside` is a certificate and every rounding
in it keeps the cell. So the set the engine minimises over is the
face's region plus a boundary collar, `hi` is a minimum over that
superset, and `m = M` still does not hold. `Certified::LowerBoundOnly`
therefore still refuses the two unsound arms (`AtLeast`/`Violated`
and `AtMost`/`Holds`), which TRIM-3 PR-2's E9 row pins.

What PR-2 does buy is the SIZE of the looseness, not its existence:
on the M10-6 notch fixture the bracket's `hi` was the window's 0.1
against a true face separation of 0.269.

## Fix shape

Exact-region cells: a cell classification that answers
inside/outside/straddling exactly rather than conservatively, so the
kept set is the face's region and nothing else. That is a different
mechanism from a chord polygon with a funnelled margin — it needs the
boundary's exact algebraic image on the chart, and on a curved
carrier that is the arc's own equation rather than a chord and a box.
Only then is `hi` a bound on the FACES and only then can the refusal
retire.

## Home

TRIM — the description is `chart_bound.rs`, this program's; the
refusal lives in `editor-core/src/measure.rs`, M10's, and retires by
a change to the description's contract rather than to the measure's.
