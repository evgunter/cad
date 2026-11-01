---
id: a-wide-section-row-escalates-chord-side-at-eps-1e-12-on-main
kind: issue
title: loft_profiles_door's wide-section row escalates the section's own chord_side at eps 1e-12 on main
status: closed
opened: 2026-10-11
priority: P1
cost: E
closed: 2026-10-16
---


Found by the FLUX cone-tail unit (PR 4744), whose CI took sweep's
1e-12 row; reproduced on `origin/main` alone (main at `b48fc81e0`, a
main-only worktree):

```text
CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep \
  -E 'test(at_interval_a_wide_section_refuses_as_section_not_a_point)'
```

fails at `crates/sweep/tests/loft_profiles_door.rs:39`, before the row
reaches its subject — the section's own validation escalates:

```text
the section validates: Escalated { site: SegmentPair(SegmentRef {
loop_index: 0, segment_index: 0 }, SegmentRef { loop_index: 0,
segment_index: 1 }), source: Indeterminate { margin: Enclosure { lo:
-2.0002e-12, hi: 2.0002e-12 }, band: Band { zero: 1e-12, escalate:
1e-11 }, predicate: Some("chord_side") } }
```

The row landed with `43e2c69ee7` (u6 fix pass). Why the section's
`chord_side` enclosure is wider than the band at this ε is unmeasured.

## Closed

Dissolved by `the-skin-at-interval-encloses-the-nominal-skin` (PR 5233),
which deletes the row: a wide section no longer refuses at `Interval`,
it encloses its family. The rows that replace it
(`loft_profiles_door`'s `at_interval_a_wide_section_encloses_its_family`
and its siblings in `a_loft_computes_its_walls_in_t`) take a box
`ε/256` either side of the corner (`common::enclosure::box_about`), and
are run at the 1e-12 and 1e-6 rows as well as the default.
