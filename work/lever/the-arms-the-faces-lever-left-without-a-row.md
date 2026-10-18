---
id: the-arms-the-faces-lever-left-without-a-row
kind: issue
title: Arms the faces' lever left without a reaching row: the extrude's first-order sliver and cap-rim smooth arms, a split's transversality, a fillet corner arc's in-band and pair verdicts
status: open
opened: 2026-10-16
priority: P2
cost: M
---

Since TANG's U-lever (PR 5204) every reader of the wedge between an edge's two faces levers it at the faces' reach (`topo::wedge_reach`), never at the edge's own length. Every row below reached its arm by posing a short edge between large faces; the faces' lever reads each of those poses as a definite corner or tangency, so the row now pins the build and cites this item. Each arm is still in the code, with the argument below for why no point input reaches it. For each, either a row reaches it again (the at-rest, box-widening or hand-built pose named), or it is retired to `unreachable!` with the argument.

**The extrude's first-order wall join, `ExtrudeError::SliverJoin` at a strut** (`crates/sweep/src/extrude.rs`), with its canonical `vertex_index`. The strut's two walls reach at least the two profile segments' lengths, so the wedge `sin α · reach` is at least the profile's own separation at the turn, which the profile door decides first (`SegmentPair`). **Reached at rest through a box**: `editor-core`'s `m10_3_driver_interval::the_certification_width_is_no_longer_bounded_by_epsilon` names the one-leaf refusal past its ceiling, `Extrude(SliverJoin { loop_index: 0, vertex_index: 0, reading: Lever(Reading), .. })`, an enclosure straddling the band at every ε row. What lost a row is the CANONICAL INDEX in both extrusion directions (`sweep/tests/review_m2_pr4.rs`, `survives_shallow_corner_strut_keeps_its_canonical_index_both_directions`, now pinning the strut's `Intersection` from canonical vertex 1) and the θ ladder's `OnPair` and `SliverJoin` rungs (`survives_dihedral_band_sweep_at_the_strut_arm`). A row driving a reversed slab past its ceiling would pin the index again.

**The extrude strut's conventional-pair and in-band must-carry arms** (`sweep/tests/must_carry_rule.rs`, the extrude trio's zero and in-band rows). At the walls' extent the strut's sagitta is the profile's own arc-to-line separation, which the profile door decides first: the rows now end there (`pairs_among == (4, 0)` at zero, `Escalated SegmentPair(0, 1)` in band). The revolve twin still walks all three verdicts.

**The extrude's cap-rim arms: `ExtrudeError::SliverRim` and the `Smooth` arm** (`crates/sweep/src/extrude.rs`, "no admitted input is known to reach it"). The wall reaches at least its distance to the far rim, and that distance times the cap–wall sine is the extrusion height, which the gate decides past `K·ε`. The cap rim's `MaterialPairing` at-rest refusal went with them (`sweep/tests/review_fillet_h6_r1_probes.rs`, now `worst_admitted_obliquity_on_a_tight_rim_is_the_faces_corner`). `SliverRim` with an INVALID margin is reached through a box whose lever form reads below zero; none does on main or here.

**A split's `Certification { Escalated { Transversality } }` at the cut** (`sweep/tests/join_lever_is_the_joined_edge.rs`, now `a_station_on_the_line_edge_builds_either_side_of_k_eps_and_joins_back`). The child is read against its parent's faces, whose vertices hold its parent's ends, so the child's reach is at least half its parent's and its margin at least half its parent's certified one: the arm needs a parent certified within a factor of two of `K·ε`, which the D ∩ brick fixture reaches only where the boolean's two section lines are within the band of each other.

**A fillet's corner arc in band, and its under-determined `OnPair`** (`sweep/tests/contact_edge_must_carry.rs`, `review_contact_edge_must_carry_r1_probes.rs`, now `r1_a_slim_wedges_corner_arcs_are_the_intrinsic_tangency`). At the faces' extent the arc's second-order margin is `κ·arm²/2` with the arm capped at the ball's radius `r`, so `r/2`: in band only where the fillet radius itself is, which the fillet door refuses first.

**The at-rest gate's `SliverDihedral` through import** (`step-import/tests/review_r1_tier_gate_probes.rs`, `r1_in_band_dihedral_wedge_never_ships_silently`). For a PLANAR wedge, two planes in band over their whole extent leave their far ends in band too, so the attachment gate refuses the file's sliver far face before the at-rest gate is asked: the row now pins that refusal. A curved pose (faces whose curvature arm, not their extent, sets the lever) might reach the at-rest arm through import; none was searched for. `topo`'s `review_m2_pr3` lamina reaches the arm natively.

Found by the U-lever review (PR 5204, findings M2 and m1; the import arm, the second review's F4).

