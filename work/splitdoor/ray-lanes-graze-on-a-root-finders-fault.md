---
id: ray-lanes-graze-on-a-root-finders-fault
kind: issue
title: point-in-solid and region rays read a root finder's fault as a graze and cast again
status: open
opened: 2026-10-29
priority: P3
cost: E
---


Found by the §5 sweep of `splitdoor/roots-unsettled-split`.

A ray the kernel casts is its own probe, so a touch of the ray within the
band (`RootRefusal::Touch`) and a limit (`Unsettled`) rightly graze: the
walk sets the ray aside and casts another. Two readers fold a root
finder's fault (`geom_brep::roots::RootFault`, today only
`ImpossibleCount`, which no legal input reaches) into that graze:

- `crates/topo/src/boolean/region.rs` `found`: `RootRefusal::Fault(_)`
  grazes with `Touch` and `Unsettled`.
- `crates/topo/src/boolean/solid_contain.rs` `carrier_roots` and
  `ray_crossings`: `RootRefusal::Fault(_)` grazes with `Touch`.

Both kept the behaviour they had before the split (the fault was then
an `Unsettled` variant that grazed). A fault is the kernel's; it grazing
means another ray may answer over a defect nobody sees. The repair is a
kernel-defect ending at the three sites (`RayFault` has none today), and
a row that drives one through each.
