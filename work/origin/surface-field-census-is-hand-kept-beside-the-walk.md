---
id: surface-field-census-is-hand-kept-beside-the-walk
kind: issue
title: param_source.rs's SurfaceField is a hand-kept census of fields the surface walk now enumerates
status: open
opened: 2026-09-29
priority: P4
cost: E
refs: [surface-field-walks-and-source-theorem-checks-have-no-one-home]
---


From PR 3429's sweep. `crates/topo/src/param_source.rs`'s
`SurfaceField` lists the surface fields a parameter can drive, by hand,
beside `Surface::analytic_data` which now enumerates them from one
destructure. Derive it from the walk (or state at the enum why it is a
deliberate subset), so a new field cannot be missed there.
