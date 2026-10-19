---
id: via-arrival-start-rustdoc-hides-which-direction-its-director-binds
kind: issue
title: paths: ViaArrivalStart's rustdoc does not say its director binds the arrival tangent at Start
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found while weighing the arrival-constructed arc (`arc-closer-constructed-from-arrival-tangent`).

`ViaArrivalStart` (`crates/profile/src/path/family.rs:845`, "A `Via` CLOSE: anchor at the entry, director pending.") and its `angle_kernel` (`family.rs:867`) bind a director that authors the ARRIVAL arc's end tangent at `Start` — `via_close` takes it as the arc's direction at the entry, and the state that results has no outgoing slot (it is a closed loop). Every other arrival-builder director binds the anchor's OUTGOING direction, so the rustdoc should say which direction this one binds. Comment-only; the fix is one sentence on the struct and one on the kernel. If the `Arrive` ruling lands first, `Via { q, p: Start.arrives_tangent() }` closes at the verb with no director, and this sentence should name that as the tangent form.
