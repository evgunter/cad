---
id: a-split-side-whose-census-escalates-in-band-refuses-as-not-at-rest
kind: issue
title: A split side whose census escalates in band would refuse as the kernel's ResultNotAtRest, not as an escalation
status: open
opened: 2026-10-28
priority: P2
cost: E
---


Found by the tier-3′ gate measurement (`splitdoor/split-gate-3prime`, 2026-10-28), cause D. It is a blocker of `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime` before that gate flips.

With the split gated at tier 3′, a side whose census returns `CensusEscalated Indeterminate` refuses as `SplitFinishError::ResultNotAtRest`. These are the predicates `bool_contact_vertex` and `curve_pair_pose_extreme`/`offset`, with margins inside the band. An in-band reading is the user's lever, an escalation, not a kernel invariant. Map census escalations to the split's escalated refusal at the gate.

Tests:
- `a_vertex_on_a_grazing_plane::the_split_reads_a_grazing_vertex_against_its_section`, all ε;
- `section_end_poses::along_the_miss_…`, 1e-9;
- `a_rigid_turn_keeps_a_slivers_volume`, 1e-9 and 1e-12;
- `split_seam_vanishing_tilt`, 1e-12.

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).
