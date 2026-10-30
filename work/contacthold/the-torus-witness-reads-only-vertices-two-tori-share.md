---
id: the-torus-witness-reads-only-vertices-two-tori-share
kind: issue
title: The merge's torus witness escalates two distinct tori whose faces share every vertex
status: open
opened: 2026-10-30
priority: P2
cost: E
---


Found on `splitdoor/rods-two-lengths` (PR 6070) after merging main at eb5afd4b and 21a51fda. Those commits made a torus pair the ring reads apart wait on a witness (`boolean::carrier_pair` `torus_pair_witnessed`).

**Replay.** `cargo nextest run -p sweep --test all -E 'test(=two_rims_review_probes::a_lens_inside_the_union_of_its_tori_is_never_read_on_it)' --no-capture`. The fixture is two coaxial tori of tube radius 1 on rings 2 and 2.8: their solid union U, and their lens L (`union_and_lens`). `U ∪ L` and `U ∩ L` refuse `Merge(BooleanMerge(Escalated { decision: NeighbourCoincidence, predicate: "carrier_torus_witness", margin ≈ 1.1e-16 }))`, at ε = 1e-6, 1e-9 and 1e-12. Before those commits, both built at their closed forms (U and L). The row now pins the escalation and names this file.

**Cause.** At the coplanar merge, the result's two torus faces neighbour each other, and the ring reading decides them `Distinct`: the rings differ by 0.8. `torus_pair_witnessed` then asks whether a point of one face stands past the band from the other carrier. Its points are `face_witnesses`, the boundary **vertices** only. Every vertex of both faces lies on the two circles the tori share, so the farthest distance is ≈ 1e-16 and the pair reads undecided. Points interior to the faces' edges are far off the other carrier: the union's seam meridians reach 0.8 from the other torus.

A second gap: `torus_distance` is signed and the fold keeps `max(0, d)`, so a witness inside the other tube never counts as far. The witness should read the distance's magnitude.

**The answer owed.** Two faces on carriers whose rings differ far past the band, meeting only along circles both carriers hold, are two carriers. A witness from the edges' midpoints (`geom_brep::EdgeCurve::mid_point`, as `crate::stands` takes them), read by magnitude, would decide this pair.

