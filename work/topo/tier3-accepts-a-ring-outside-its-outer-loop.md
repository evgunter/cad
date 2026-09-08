---
id: tier3-accepts-a-ring-outside-its-outer-loop
kind: issue
title: tier 3 accepts a face whose ring lies OUTSIDE its outer loop (check 9 tests contact, not nesting): a shell_open glue with the host/guest roles inverted validates with the correct volume
status: open
opened: 2026-09-08
---


Found by SHELL-5's R1 review (PR 2159), by a scratch mutant:
`shell_open`'s rim loop (`crates/topo/src/shell.rs`, the
`(host, guest)` assignment) with a void designation treated as an
outer-shell one — `kfmrh(designated, counterpart)` where the lifted
counterpart's boundary ENCLOSES the designated face's. The glue's
precondition `validate::ring_outer_contact` reports `Disjoint` (it
decides contact, not which loop is inside), `kfmrh` makes the larger
loop a ring of the smaller face, and the verb's closing
`validate_geometric` accepts the body: tier 3 green, the flux volume
correct (the face set and windings are the same either way). Only
`verbs_shell::opening_the_hollow_boxs_void_ceiling_cups_the_inner_wall_only`'s
structural assertion "the designated void face dies" caught it; the
R1 e2e row (`shell5_r1_probes::r1_e2e_hollow_twice_then_open_the_inner_wall`)
passed on the mutant.

Consequences: (1) check 9 (`RingMeetsOuter`) is a contact check and
no tier-3 check states ring-inside-outer, so a face with a ring
surrounding its outer loop is a body every structural tier blesses;
(2) `shell.rs`'s comment at the role assignment — "the disjointness
check below and tier 3's windings are what verify it" — overclaims:
the role is verified by nothing at rest. The hole path is better off:
`pair_rings` decides nesting by mean radius and refuses the inverted
case typed (`"the designated face's hole does not sit inside the
cavity counterpart's"`, measured on the same mutant). The fix is a
nesting decide (the `encloses` shape) either in check 9 or as a second
precondition of the glue, and the comment corrected either way.
Placed by the SHELL orchestrator (2026-09-08).
