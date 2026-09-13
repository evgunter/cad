---
id: boundary-iso-doors-panic-before-they-can-refuse
kind: issue
title: boundary_iso_u/_v panic on a corrupt net instead of refusing, and their # Errors contract promises the opposite
status: open
opened: 2026-09-12
---


Filed by the DOOR orchestrator from the `step-adopt-let-ok-iso-discards`
lane's report (PR #2406). The lane measured it; it is on TRIM's slate
because `nurbs_iso.rs` is Track Q's fence and the root is there, not in
any consumer.

## Finding

`geom_brep::boundary_iso_u` and `boundary_iso_v`
(`crates/geom-brep/src/nurbs_iso.rs:56-64`, `:79-84`) extract a boundary
column from a `NurbsSurface` and re-wrap it with `NurbsCurve3::new`.
Their `# Errors` contract promises a typed refusal — *"unreachable from
this door's own output; surfaced rather than swallowed (D4 ¶2)"*.

**On the input that contract is written about, they panic instead.**
Executed by the lane against a validation-bypassed surface (a 3x4 net
with `net::validate_counts` removed from `NurbsSurface::new`):

| corrupt net | `end = false` | `end = true` |
| --- | --- | --- |
| one control point short | `Ok` | **PANIC** in the slice |
| weights one short | `Ok` | **PANIC** in the slice |
| weight `0.0` at net index 0 | `Err(NonPositiveWeight)` | `Ok` |
| weight `NaN` interior (index 5) | `Ok` | `Ok` |
| weight `inf` at net index 11 | `Ok` | `Err(NonFiniteWeight)` |

The mechanism: `s.control()[base..base + nv]` indexes before anything
validates, so a net whose length disagrees with its knot vector dies in
the slice rather than reaching `NurbsCurve3::new`. Independently
confirmed by the orchestrator from the code path — `validate_counts`
(`crates/geom/src/net.rs:89`) can only be reached with `control.len() ==
weights.len()`, both being the same slice length, so `ControlCountMismatch`
and `WeightCountMismatch` are unreachable through these doors for **any**
`NurbsSurface` value.

## Why it is worth a row rather than a shrug

The state is unreachable through validated construction, so this is a
D9-shaped hole rather than a live bug. Three things make it a row:

1. **The contract is false, not merely optimistic.** A caller reading
   `# Errors` writes a `match` that cannot run, and gets a panic across
   whatever boundary it sits behind. Three consumers already lean on
   that promise: `crates/sweep/src/loft.rs:523`,
   `crates/geom-brep/src/pcurve_cache.rs:3564`, `:3959`, `:4091`.
2. **It crosses to Python.** `crates/step-import` now converts this
   refusal into a public `StepImportError::WallColumnStructure` with a
   Python tag (#2406), so the panic path is one layer under a
   documented public door.
3. **The narrow payload is a consequence of it.** Because the count
   arms cannot be reached, the only `SplineError` these doors can ever
   return is a weight violation. #2406's arm doc had to be corrected to
   say so; that correction is a workaround for this defect, and it
   reverts to the natural wording once the doors refuse.

## Fix shape

Refuse rather than index: check the net's length against
`knots_u`/`knots_v` at the top of each door and return the
`ControlCountMismatch` the contract already advertises. Then #2406's
`WallColumnStructure` doc can say what it originally tried to.

## Not a duplicate of S394, and the two meet

`S394` (closed 2026-09-04) is the adjacent row: consumers **swallowing**
a `boundary_iso_*` refusal with `map_err(|_| …)`, discarding the payload.
This row is the opposite end — the doors **cannot produce** the payload
on the input in question, because they die first. S394 is about what
callers do with the error; this is about the error not existing.

Worth recording: S394 explicitly named `crates/step-import/src/adopt.rs:711`
and `:877` and said *"a taker should decide those separately rather than
converting them by pattern"*. DOOR's `step-adopt-let-ok-iso-discards`
took exactly those two sites, decided them per-site, and converted both
with the reasoning written at each. The two rows agree.
