---
id: boolean-refusals-for-door-glued-pairs-keep-declaration-names
kind: issue
title: Boolean refusals reached by door-glued pairs keep declaration-era names: UnsupportedDeclarationClass, DeclaredContact payloads, the ContactContradicted kind
status: closed
opened: 2026-10-12
closed: 2026-10-30
priority: P3
cost: E
pr: 6079
branch: contacthold/glued-pair-reads-its-record
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
  `ContactContradicted` and `RimCuspArmUnbuilt`, and those variants'
  docs that still say "declared" (`TangentSlitArmUnbuilt` is retired,
  `contacthold/face-face-touch-row`, PR 4921);
- `ContactContradicted` on a door-glued `Rest` pair keeps the
  "correct or remove the declaration" recourse
  (`crate::contact::CONTRADICTION_RECOURSE`); `vtxfac`'s decided-tilt arm
  reaches it.

## Closed

Closed by PR 6079: `UnsupportedDeclarationClass` is `TangencyUnsupported`, `DeclaredContact` is `ContactPair`, `ContactContradicted` is tangent-only, and `DeclarationContradicted`/`ContinuationContradicted` are gone. The user-visible strings speak of records, not declarations: `CONTACT_RECOURSE` ("move the geometry so the faces clearly touch or clearly part"), `CONTRADICTION_RECOURSE`, `ContactRefusal`'s Display, the census `ContactContradicted` message, `OUTSIDE_REST_INVENTORY`, and the `Coincide::Contact` subject. `ContactRefusal::Undeclared`, which nothing constructed, is gone. The routing that still reads a declaration that would settle a question is `the-declaration-read-routing-survives-the-declarations`.
