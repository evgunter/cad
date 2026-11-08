---
id: the-in-band-wedge-recourse-has-no-witness
kind: issue
title: FILLET3_WEDGE_RECOURSE has no witness once a knife edge builds
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Found in the knife-edge fillet unit (PR 6510); its reviewer R1 asked for it.

`FILLET3_WEDGE_RECOURSE` ("blend an edge whose supports meet at a definite angle or definitely tangent, or leave it out of the request") is now the lever only of `fillet3_convexity_sign` escalating in band. Before PR 6510 its witness was a slit decided Zero. A knife now builds where its ball is defined, so that body no longer reaches the sentence.

`crates/sweep/tests/blend_recourse_followability.rs`, `the_wedge_recourse_has_no_witness_in_this_suite`, pins what the caller meets instead: the slit's own fillet builds. `review_blend1_r1_probes::r1_in_band_convexity_sign_renders_the_tangential_sentence` renders the sentence from a constructed in-band wedge. `r1_a_near_collinear_profile_vertex_and_the_convexity_arm` records how a near-collinear profile vertex meets the arm, depending on ε.

**Wanted:** a body, authored through a public door, whose requested edge's dihedral lands inside `fillet3_convexity_sign`'s band at the default ε. Then a row that follows the sentence to a build on the same body. Until then the sentence is advice no fixture follows.
