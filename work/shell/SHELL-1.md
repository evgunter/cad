---
id: SHELL-1
kind: unit
title: the ShellNaming birth channel — shell and shell_open return Shelled<T>
status: spec
opened: 2026-09-04
branch: shell/1-naming
refs: [shell-needs-shellnaming-birth-channel, LIB-G17]
---


The `ShellNaming` birth record, written by `shell` / `shell_open`'s
own steps and returned beside the body as `Shelled<T>` (the
`Extruded` / `BooleanBody` shape): outer wall per source face, inner
twin per source face/edge/vertex through the void graft map, one
`RimNaming` per designated chart (rim face, ring, ring edges keyed to
the source boundary edges, promoted hole rims), and the retired keys.
Closes `shell-needs-shellnaming-birth-channel`; unparks LIB-G17; the
shape SEAT's `VerbRecord::Shell` arm consumes. Spec
`docs/SHELL-1-SPEC.md`. Pre-draw difficulty M, task class STRUCTURAL.
