---
id: section-cert-ruling-walk-folds-every-refusal-into-its-row
kind: issue
title: section_cert ruling: the walk's touch, limit and fault all refuse as the certificate's row
status: closed
opened: 2026-10-29
closed: 2026-11-24
priority: P3
cost: E
---


Found by the §5 sweep of `splitdoor/roots-unsettled-split`.

`crates/topo/src/boolean/section_cert/ruling.rs` reads its
`geom_brep::half_angle::certified_subdivision` walk as
`Err(_) => return Err(walk.row)`: a touch in band
(`RootRefusal::Touch`), a limit (`Unsettled`) and a fault (`RootFault`)
all refuse as the certificate's row. A refusal of the certificate is
fail-safe (the section is not certified), so no body is wrong, but the
three are three stories: the touch is the geometry's, in band; the
limit is the frontier; the fault is the kernel's defect. The repair
types the three apart at that site.

## Retired (sectcone, PR 7637, 2026-11-24)

`topo::boolean::section_cert::ruling` is deleted: topo's section certificate reads cone × cylinder and cone × cone on the section value (`geom_brep::pair_section`), whose ruling sweep (`pair_section/ruling.rs`) tiles cells over the whole turn and reads no `certified_subdivision` walk. The site this row names is gone with it, and the value's sweep refuses by the row that failed (`SweepRefusal`). Closed as moot.
