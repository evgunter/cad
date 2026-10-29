---
id: ray-lanes-graze-on-a-root-finders-fault
kind: issue
title: point-in-solid and region rays read a root finder's fault, other than a torus count disagreement, as a graze and cast again
status: open
opened: 2026-10-29
---


Found by the §5 sweep of `splitdoor/roots-unsettled-split`.

A ray the kernel casts is its own probe, so a touch of the ray within the
band (`RootRefusal::Touch`) and a limit (`Unsettled`) rightly graze: the
walk sets the ray aside and casts another. Two readers fold a root
finder's fault into that graze:

- `crates/topo/src/boolean/region.rs` `found`: `RootRefusal::Fault(_)`
  grazes with `Touch` and `Unsettled`.
- `crates/topo/src/boolean/solid_contain.rs` `torus_crossings`: a
  `RootFault::CountDisagrees` blocks the ray
  (`PointInSolidError::RootCountDisagrees`), but `RootFault::Resolvent`
  and `RootFault::ImpossibleCount` graze.

Both kept the behaviour they had before the split (the fault variants
were then `Unsettled` variants that grazed). A fault is the kernel's;
it grazing means another ray may answer over a defect nobody sees. The
repair is that every fault blocks, as `CountDisagrees` does, with the
measured caveat that the quartic's `Resolvent` rung is reached by a
legitimately small resolvent root
(`work/inside/torus-split-lead-escalates-a-legitimately-small-resolvent-root.md`),
so a block there may need that row first.
