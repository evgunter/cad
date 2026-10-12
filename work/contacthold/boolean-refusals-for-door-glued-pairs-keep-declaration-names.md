---
id: boolean-refusals-for-door-glued-pairs-keep-declaration-names
kind: issue
title: Boolean refusals reached by door-glued pairs keep declaration-era names: UnsupportedDeclarationClass, DeclaredContact payloads, the ContactContradicted kind
status: open
opened: 2026-10-12
priority: P3
cost: E
---



## What

Under D10 no declaration reaches a boolean: every glued pair is the glue
door's own decision, or a union step's given verdict. PR 4831's fix pass
reworded the messages a door-glued pair reaches (`ContactContradicted`
on a `Tangent` pair now says the kernel's own readings disagree and ends
on the kernel-defect sentence; `TangentSlitArmUnbuilt` no longer calls
the pair declared) and renamed the `declaration` field of
`RimCuspArmUnbuilt` and `TangentSlitArmUnbuilt` to `pair`. What it left,
because each rename reaches the public kind list and the Python tags
(`crates/pncad-py/src/tags.rs`, `pncad.pyi`):

- `BooleanError::UnsupportedDeclarationClass` and its kind and tag
  `unsupported_declaration_class`: raised only for a glued pair the
  witness lane cannot read, which the door never glues, so it is reached
  from test-built glue alone (`TangencyFailure::refusal`, `route_rim`'s
  unplaced junction);
- the `crate::contact::DeclaredContact` payload type of
  `ContactContradicted`, `RimCuspArmUnbuilt` and `TangentSlitArmUnbuilt`,
  and those variants' docs that still say "declared";
- `ContactContradicted` on a door-glued `Rest` pair keeps the
  "correct or remove the declaration" recourse
  (`crate::contact::CONTRADICTION_RECOURSE`); `vtxfac`'s decided-tilt arm
  reaches it.
