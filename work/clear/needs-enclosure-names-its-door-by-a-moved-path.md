---
id: needs-enclosure-names-its-door-by-a-moved-path
kind: issue
title: MeasureUnavailableAt::NeedsEnclosure names its door as clearance::min_separation, which now lives at topo::clearance::min_separation
status: closed
opened: 2026-10-28
priority: P4
cost: E
closed: 2026-10-28
pr: 5928
---

Found by SHELL-3's sweep for readers of the moved symbols. The `f64`
lane's `min_clearance` refusal, `MeasureUnavailableAt::NeedsEnclosure`,
carries `door: "clearance::min_separation"` (`editor-core/src/eval/wire.rs`,
the `min_clearance` arm of the measure evaluation). SHELL-3 moved that
door to `topo::clearance::min_separation` and left the string alone:
it is rendered text, pinned by
`m10_6_clearance_assertion_interval.rs` (three rows),
`r2_m10_6_probes_interval.rs`, `pncad-py/src/tests.rs`'s
`the_fourth_verbs_two_refusals_are_stable` and
`pncad-py/tests/test_measures.py`, and a move with no behaviour change
does not re-pin it.

The suffix still names the door, so the recourse reads true. Whether the
refusal should name the full path, or the measure path's own door once
`MinClearanceLane` goes (`symbolic-tier-and-clearance-engine`), is that
unit's call; it is the natural place to re-pin the five rows together.

