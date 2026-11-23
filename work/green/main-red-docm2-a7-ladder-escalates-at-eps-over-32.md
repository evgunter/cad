---
id: main-red-docm2-a7-ladder-escalates-at-eps-over-32
kind: issue
title: main is red (slow set): docm2_part_interval::a7_the_width_ladder_of_the_split_of_a_widened_box escalates at ε/32, where the row pins green
status: closed
opened: 2026-11-08
closed: 2026-11-09
---


Found by the ONEHOME finished-body lane (PR 6371) while merging main,
and measured on `origin/main` at `40e3f0f7f` in a clean worktree: the
row fails there exactly as on the merged branch.

`crates/editor-core/tests/docm2_part_interval.rs:178`
(`a7_the_width_ladder_of_the_split_of_a_widened_box`) pins the ε/32
rung and every narrower one `Green`. At `40e3f0f7f` the ε/32 rung
fails: the union of the split's halves (`MintId` ordinal 31) refuses
`BooleanError::Escalated` on `Coincidence(Carriers, Settles(Rest,
Carriers))`, `bool_plane_offset` reading the enclosure
`[−1.50e−9, 1.50e−9]` (band zero 1e−9), with a second escalation on
`bool_sector_coplanar`; ordinals 55 and 78 are poisoned through it.
The ε/10 and ε/16 rungs read as pinned. So the floor the row
describes moved up a rung: what was green at ε/32 now reads the cut
faces' plane offset in band there too.

The row is in the slow set (`.config/nextest.toml`), so main's PR runs
did not run it. What moved it is unread: bisect main from the last
green run of the slow set to `40e3f0f7f`, then either restore the
floor or re-pin the ladder with its cause.

## Closed (BOXES, `boxes/nightly-reds`, 2026-11-09)

Bisected: green at `c036d7ae88^1`, red at `c036d7ae88` (BOXES' PR 6399, `boxes/cover-edge-doors`). That PR made every edge's box its cover: the carrier's cover over its span, hulled with its ends. Before it, a line edge's box was the chord of its two end vertices. At the `Interval` lane the carrier's points are as wide as its stored terms read at the span. The pair extent's centre, where `bool_plane_offset` reads both planes' offsets, inherits that width, and the offset's difference reads it twice. The enclosure is ±3.0e−9 at ε/16 and ±1.5e−9 at ε/32 (band zero 1e−9): it scales with the width, so the floor moved one rung, to ε/64. The cover is a sound bound of the edge, and the escalation is the right reading of that enclosure. So the ladder is re-pinned at ε/32 (`Escalates`, the union, `bool_plane_offset`), and the assertion row's divisor moves to 256 to stay two rungs under the floor. The tighter line reading is filed as `work/boxes/a-line-edges-cover-is-as-wide-as-its-carrier-not-its-ends.md`.
