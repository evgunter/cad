---
id: boolean-result-kind-assembly-names-a-type-the-kernel-does-not-have
kind: issue
title: BooleanResultKind::Assembly names an assembly the kernel says it has no type for
status: open
opened: 2026-11-10
priority: P4
cost: E
---


Filed by the BAND orchestrator from two designers' independent findings (the kiss-of-two-solids fork, 2026-11-10).

`topo::BooleanResultKind::Assembly` (`crates/topo/src/boolean/ops.rs`) is the provenance tag for a boolean result of several unseamed pieces. DESIGN `[one-way]` and `[assemblies.document]` say "the kernel has no assembly type", and the name collides with that: a design brief read the union of two touching tubes as "an assembly". Rename it to say what it is (a multi-piece or unseamed result), and sweep its uses. The same word in `FILLET3_ASSEMBLY_RECOURSE` means the blend's own assembly stage; check whether that one reads ambiguously too.
