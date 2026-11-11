---
id: main-red-steeper-tilts-union-stops-by-identity-reads-the-wrong-refusal-shape
kind: issue
title: main red: germ_tilted_rod::the_steeper_tilts_union_stops_by_identity asserts a refusal's Debug shape ('findings') the code no longer produces ('errors')
status: closed
opened: 2026-11-09
priority: P1
cost: E
closed: 2026-11-11
branch: tint/two-slow-reds
pr: 6818
---


BOXES' `cover-app-readers` lane found this running the sweep slow set (at `de25200`, main merged).

`crates/sweep/tests/germ_tilted_rod.rs` `the_steeper_tilts_union_stops_by_identity` fails on a refusal's Debug shape: it expects `findings`, and the refusal now prints `errors`. Neither that branch nor BOXES touches the refusal. The test is in the slow set, so a PR gate that does not spell its name skips it.

The file is TCOST's and TINT's ground; this is filed on TINT as a test-integrity defect. The fix belongs with whoever renamed the refusal's field (`findings` → `errors`): re-pin the row to the new name, or assert the refusal's kind rather than its Debug text, so a rename cannot red it.

Signed: (BOXES orchestrator)

## Dispatch

2026-11-11, TINT orchestrator. One implementer, lane `tint-reds`, branch `tint/two-slow-reds`, which takes both of TINT's open P1 slow-set reds together: the Debug-shape pin in `germ_tilted_rod` and the stale `bounds_census` roster lines. The lane re-measures each on main first. Review tier: the orchestrator's read, since both are test-side re-pins with no kernel change. If a fix has to land in `crates/*/src`, that becomes a style review, announced to the crate's owner.

## Closed (2026-11-11, PR 6818, by TINT)

Measured on main (`f89fd07`, main at `b291655`) at ε 1e-9: red, and the
other way round from the filing. The test pinned `errors: [...]`; the
refusal prints `ResultInvalid { findings: NotFinished { findings: [...] } }`.
The finding inside is the pinned one, unchanged: solid `1v1`, face `3v1`,
`QuadratureUnsupported` with the spiric-wall `what`.

The cause is `3116174492` ("topo: doors take and return the scalar's
finished body"), which landed on main through PR 6371 on 2026-11-08. It
retyped `BooleanError::ResultInvalid { errors: Vec<ValidationError> }` as
`{ findings: NotFinished }`. The pin was written on 2026-11-04
(`dfd7236762`) against the old shape, and the row is in the slow set, so
6371's gate never ran it.

The test now matches the refusal by its variants:
- `BooleanError::ResultInvalid`, then exactly one finding,
  `ValidationError::VolumeUncomputable`;
- that finding's source is `MassPropsError::Face` over
  `PropsError::QuadratureUnsupported`;
- it asserts the solid key, the face key and the whole `what` against
  `STEEP_UNION_STOP`.

A field rename no longer reaches it, and a variant rename fails to
compile instead. The keys are still compared as slotmap's own `Debug`
(`SolidKey(1v1)`), since the result body is not returned. Green at ε
1e-9, 1e-6 and 1e-12.
