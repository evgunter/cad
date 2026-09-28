---
id: sweep-revolve-about-y-helper-and-its-fixtures-spelled-per-suite
kind: issue
title: sweep tests: the revolve-about-y helper, the rectangle meridian and the klein elbow are spelled per suite beside test_support::revolved_about_y
status: open
opened: 2026-09-28
priority: P4
cost: M
---


## Finding

Found by the `dup/b9-b` lane at merge base `2adacbc0e` while it closed
`work/dup/sweep-suites-import-fixtures-from-other-suites.md`: moving
the vessel, the tube and the torus walls into
`crates/sweep/tests/common` meant sweeping for their copies, and the
copies stand on three wider classes that are not that row's (they are
spelled per suite, never imported across one).

**1. The revolve-about-`y` helper.** `sweep::test_support::revolved_about_y`
(and `_at` for other scalars) is the kernel-side door — a loop's
vertices on `SketchPlane::xy()`, revolved about the sketch `y` axis. A
private `revolved`/`revolved_about` in each of these suites does the same
job over its own argument shape (`ProfileLoop`, `&[(f64, f64)]`,
`&[(f64, f64, f64)]`, `Vec<(Point2, f64)>`, with or without a turn):
`common/cone_nappe.rs`, `offd_r1_probes`, `p1b_r1_probes`,
`pcurve_p1b_r2_probes`, `pis_arc_capped_poses`,
`review_verbs_rim_lever_probes`, `sf2b_axial`, `sf2b_head`,
`sf2b_interval_probe` (`Interval`), `sf2b_r2_probes`,
`shell10_r2_dump`, `shell5_r1_dump`, `shell6_r1_probes`,
`shell6_r2_probes`, `shell7_common`, `shell7_dump` (two),
`shell9_r1_probes`, `torax_interval` (`Interval`), `verbs_1031b_arcwind`,
`verbs_arms1_annulus`, `verbs_arms1_r1_probes`, `verbs_offd`,
`verbs_rim_closed_lever`, `verbs_rim_r1_probes`,
`verbs_shell_r2_probes`, `verbs_shell_r2b` — 27 definitions
(`git grep -nE '^\s*(pub(\(crate\))? )?fn revolved(_about)?\b' -- crates/sweep/tests`).

**2. The rectangle meridian revolved a full turn** — the body
`common::shell_operands::vessel(r, h)` builds — spelled inline. A
structural scan (four zero-bulge `Point2` corners followed by a revolve
within 600 bytes) names these candidates; each needs its turn and scalar
read before it folds: `fillet_h5_r2_probes` (~:97),
`review_ring_clearance_r1_probes` (~:380),
`review_ring_clearance_r2_probes` (~:53), `sf2b_head` (~:104, ~:128),
`sf2b_r1_probes` (~:59, ~:297), `sf2b_r2_probes` (~:418),
`shell7_r2_probes` (~:205), `topo_ring_nesting` (~:353). The lane that
filed this folded the ones in files it was already editing
(`sf2b_axial`'s drum row, `shell5_r1_dump`, `shell10_r2_dump`'s `drum`,
`shell7_common::drum`). `shell5_r2_probes::can(r, z0, z1)` is the same
shape lifted to `z0` and is a superset, not a copy.

**3. The klein elbow**: `verbs_shell::klein_elbow` and
`shell7_dump::klein_elbow` build one body (a disc `R = 1.2` off the
axis, a quarter turn back) and differ only in naming the constants;
`spiric_rim::klein_elbow(r)` is the same elbow parameterised on the
disc radius.

**Blind spots.** Class 1 is name-shaped: a helper doing the job under a
third name is unmatched. Class 2's scan requires the four corners
literally as `(Point2::new(..), 0.0)` pairs; a loop written through
`corners`, `polyline` or `v(..)` escapes it.

## Why filed rather than folded

Size and variety: class 1 alone is 27 definitions over five argument
shapes, and each fold owes a read of the scalar and the turn. The value
is the program's usual one — one place for a body to be.
