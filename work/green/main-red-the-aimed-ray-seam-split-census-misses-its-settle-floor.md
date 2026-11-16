---
id: main-red-the-aimed-ray-seam-split-census-misses-its-settle-floor
kind: issue
title: Main red at 898df16c: an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out panics, its mass-properties settle Unmet{Floor}
status: closed
opened: 2026-11-04
priority: P1
cost: M
parent: nightly-reds-default-eps-behaviour-rows
closed: 2026-11-14
---


Filed by the ONEDOOR orchestrator on 2026-11-04. The ONEDOOR one-surface-record lane met this row and ran it on a clean checkout of main at `898df16c9c`, where it is red. It is not that branch's red.

`sweep` `an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out` panics at `crates/sweep/tests/an_aimed_ray_from_far_reads_a_split_side_right.rs:93:22` (called from `:202:31`):

    the pinned resolution is met: Unmet { stop: Floor }

The row's `read` expects `MassProperties` at its pinned resolution, but `topo::props::settle` stops at its floor.

No bisect has been done. The row is in the slow set, so the per-PR gate runs it only when a diff edits its file, and the nightly is its only other runner. To reproduce:

    cargo nextest run -p sweep -E 'test(the_seam_split_census_reads_no_side_inside_out)'

The other main reds the same lane met at `898df16c` are already filed:
- `docm2_part_interval`, `node_standing`, `m10_sym_profile_interval`, `seat7_sweep_lowering` and the loft row (green rows, and pcert's arc-rim row);
- `kernel_sidecars` (measuredoor);
- `mass_props_are_thread_count_invariant` (green, tilted_cut_upper);
- `m10_p_fence` interval (green).

## Closed

Greened by GREEN's default-eps lane (`green/default-eps`, 2026-11-14),
by a pin. Still red on main `b363f5fa` at ε 1e-9 with the message above;
green at 1e-6 and 1e-12.

- **Bisect.** First-parent, by running the test, from the merge of PR
  6174 (which added the row): first bad `898df16c9`, the merge of PR 6478
  (`spiricarm/rotational-lane-cylinder`); its parent `aef1e5892` is
  green. So `898df16c` above is not just where it was met but where it
  starts.
- **What stops.** Four poses of the 384 (second azimuth, `ψ = 5e-7`,
  both offsets, either normal): both sides build and pass tier 3, and
  the sliver side's measure stops at `Stop::Floor` with a volume bracket
  of ±7.0e-7 m³ about 2.67e-10 m³. The two sides' brackets still hold π.
- **The pin.** `read` returns `Read::Unsettled` for such a pose and the
  census holds the unsettled poses to exactly those four at ε 1e-9 and
  to none at the other rows.
- **The fix row** is SPIRICARM's:
  `work/spiricarm/a-seam-split-slivers-measure-stops-at-its-floor-2600-volumes-wide-since-the-rotational-cylinder-lane.md`.
