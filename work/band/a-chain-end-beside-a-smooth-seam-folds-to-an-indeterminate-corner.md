---
id: a-chain-end-beside-a-smooth-seam-folds-to-an-indeterminate-corner
kind: issue
title: A blend chain ending at a vertex where a smooth seam meets folds the seam into CornerConfig::Indeterminate
status: review
opened: 2026-11-02
priority: P3
cost: E
pr: 6810
branch: band/open-mixed-chain
---

Filed by the REFS lane (`work/refs/select-all-edges-picks-edges-a-fillet-refuses.md`, PR 6344), from its sweep for blend doors that read a Zero dihedral as a refusal; measurements corrected by that PR's review.

At an OPEN chain's end, `battery::corner_at` (`crates/sweep/src/blend/battery.rs`) resolves the vertex's other edges as links to count their convexity, and a neighbour that resolves `Resolved::Smooth` folds into `CornerConfig::Indeterminate` (the `Ok(Resolved::Smooth(..)) | Err(_)` arm). A chain ending where an unrequested smooth seam arrives therefore refuses "the corner's configuration could not be read", whose recourse does not name what is there: a smooth seam has no wedge to count, and the vertex is a corner of the remaining edges alone.

**Not a regression of REFS.** On main before PR 6344 the same neighbour resolved as a `TangentialEdge` refusal, which folded into the same `Err(_)` arm: the refusal is unchanged, only the arm's reason is now a smooth seam rather than a zero-wedge error. (A smooth seam the request itself names is left out of `corner_at`'s request since PR 6344, `a_blend_leaves_a_smooth_edge_as_it_is::a_smooth_strut_requested_beside_a_line_reads_as_the_line_alone`.)

**Measured** (PR 6344's review, at its head):

- One semicircular top arc of a slot (a centre run between two semicircular ends, `a_blend_leaves_a_smooth_edge_as_it_is::slot`) requested alone reaches this arm: `UnsupportedCorner { corner: Indeterminate, policy: Some(RunOutStopAtVertex) }`. Pinned as today's refusal by `crates/sweep/tests/pins_of_todays_blend_refusals.rs::pin_today_a_slot_end_arc_alone_folds_its_smooth_neighbour_to_indeterminate`; this row's PR flips that pin.
- One arc of `sweep::test_support::bored_block_of_arcs(2, ..)`'s rim requested alone does NOT reach it: `battery::is_seam_vertex` reads the valence-4 seam crossing first and refuses `UnsupportedCorner { SeamVertex }`, whose recourse (ask for the rim whole) is served.
- On a rounded rectangle, a straight cap edge requested alone refuses `END_FACE_CURVED` first (its end face is the corner arc's cylinder), before any corner read.

- 2026-11-02 — moved from BANDRULE to BAND by `git mv` when BANDRULE merged into BAND, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and BANDRULE had fallen far under it).
- 2026-11-11 — PR 6810 (band-openmix fix pass, item 9): `corner_at` now reads, before any cut-off or corner reading, whether an unrequested edge at a chain's end meets one of the link's supports smoothly, and refuses `UnsupportedRunOut { PARTIAL_TANGENT_RUN }`. At a trivalent end that is always a tangent run requested in part: the edge between the other support and the face continuing the smooth one runs on tangent to the link. The recourse names the whole run. The slot's end arc alone flips to it (`pins_of_todays_blend_refusals::a_slot_end_arc_alone_refuses_as_a_tangent_run_requested_in_part`). A smooth seam at an end of another valence still reaches `corner_config`.
