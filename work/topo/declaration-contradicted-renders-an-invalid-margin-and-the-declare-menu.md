---
id: declaration-contradicted-renders-an-invalid-margin-and-the-declare-menu
kind: issue
title: topo: a contradicted declaration renders its definite verdict as an invalid margin followed by the declare menu, beside its own recourse
status: open
opened: 2026-09-29
priority: P2
cost: E
---


(CHROME, disclosed in the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`
and filed here so the disclosure has a file. `work.py territory` gives
`crates/topo/src/merge_faces.rs` to TOPO and ZIP. It names no owner for
`crates/topo/src/boolean/mod.rs`, so this row takes both.)

## What

Two arms render the whole `Indeterminate` inside a parenthesis, then a
recourse of their own:

- `BooleanError::DeclarationContradicted`
  (`crates/topo/src/boolean/mod.rs:1750`, the variant at :871): "a
  declared coincidence contradicts the geometry ({diag}) — … fix the
  declaration or the geometry, the op never glues a lie".
- `MergeCoplanarError::DeclarationContradicted`
  (`crates/topo/src/merge_faces.rs:560`, the variant at :417, raised at
  :1868): "merge_coplanar_faces: declared coincidence contradicts the
  geometry ({diag}) — fix the declaration or the geometry, …". The
  function-name label is the shape guard's `stage_prefixes` problem as
  well.

The `diag` is not an escalation. `plane_eq` builds it for a DEFINITE
verdict with `margin: MarginDiag::INVALID`
(`crates/topo/src/boolean/plane_eq.rs:304` and :344, for
`bool_plane_parallel` and `bool_plane_offset`), and so does
`carrier_eq` (`crates/topo/src/boolean/carrier_eq.rs:336`, for
`carrier_kind`). So `{diag}` renders "margin is invalid (NaN or a
poisoned enclosure) against the ambiguity band (…) — check the
operation's inputs upstream, then declare the coincidence, move the
geometry, or lower the tolerance". That text is wrong three ways:

- the margin was decided, not poisoned;
- "check the inputs upstream" points at a defect that is not there;
- "declare the coincidence" advises adding the very declaration the
  geometry has just contradicted.

## What the shape guard counts

`test_utils::refusal::recourse_markers` would count **one**: the bare
`COINCIDENCE_RECOURSE` inside the parenthesis (one of
`BARE_RECOURSES`, in a sentence no `Recourse:` opens). It does not count
the arm's own "fix the declaration or the geometry", because that has no
`Recourse:` label. So the guard would pass a message whose one counted
recourse is the wrong one, and whose right one it cannot see.
`subjectless_escalations` would pass it too: the clause before the
parenthesis is a sentence. Neither arm is rendered by the shape guard's
samples today.

## Repair shape

Drop `{diag}` from both sentences, and do not substitute `payload()`:
for this `INVALID` margin it says "margin is invalid (NaN or a poisoned
enclosure)", which is equally false. Say which fact contradicted the
declaration, in words routed by `diag.predicate` ("the declared faces'
planes are not parallel" for `bool_plane_parallel`, "…are parallel but
apart" for `bool_plane_offset`, "…are different kinds of surface" for
`carrier_kind`). Then label the existing lever:
`Recourse: fix the declaration or move the geometry`. That gives
exactly one marker. If the measured margin is worth showing, the raise
sites have to carry it (`plane_eq` decides the sign, then throws the
margin away); `INVALID` cannot stand in for it.
