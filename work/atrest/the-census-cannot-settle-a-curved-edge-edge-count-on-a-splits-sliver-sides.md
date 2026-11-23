---
id: the-census-cannot-settle-a-curved-edge-edge-count-on-a-splits-sliver-sides
kind: issue
title: The census cannot settle the meeting count of two curved edges on a split's sliver, graze and near-seam sides (CensusUnsupported NotCertifiable)
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Filed by SPLITDOOR from the tier-3′ gate measurement (`splitdoor/split-gate-3prime`, 2026-10-28), cause C. It blocks SPLITDOOR's `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`, which is why it is banded P1.

## What

When a split's sides are run through tier 3′, the census reports `CensusUnsupported` (ContactLane, NotCertifiable) with one of two messages:
- "the meeting reading of two curved edges could not settle a count";
- "ran past its budget".

Where it happens: sliver, graze, near-seam and apex poses. Tier 3 passes on these sides, the edges share end keys and nearly coincide, and the sides are not shown unsound.

Tests, by ε:
- **1e-9:**
  - `a_cylinder_split_just_off_its_rim_vertex::a_sliver_just_off_the_rim_vertex_builds…`;
  - `a_rigid_turn_keeps_a_slivers_volume`;
  - `a_vertex_on_a_grazing_plane::an_unresolved_section_reading_keeps_the_offsets_verdict`;
  - `section_end_poses` ×3.
- **1e-6:**
  - the rim sliver;
  - `a_plane_moved_to_a_cone_apex::a_plane_moved_to_the_apex_band_plans_the_ellipse_it_cuts`;
  - `section_end_poses` ×1.
- **1e-12:**
  - `at_1e_12_the_sliver_builds_and_its_wall_stops_at_its_floor`;
  - `m5_pr5_tilted_cut::even_crossing_belly_cut_at_interval`;
  - `section_ellipse_seam_poses` ×2;
  - `section_end_poses` ×3.

PR 5956's review saw the same refusal on a cone near-axial cut at 1e-12.

## Open

Whether the count is decidable at these poses, so the census needs more reach, or whether it is a true in-band case that should refuse as an escalation.
