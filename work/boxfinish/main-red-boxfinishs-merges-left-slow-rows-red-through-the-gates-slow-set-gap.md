---
id: main-red-boxfinishs-merges-left-slow-rows-red-through-the-gates-slow-set-gap
kind: issue
title: Main red: BOXFINISH's merged PRs left slow rows red, which the PR gate never ran: five revalue rows (6963), two iso-image rows (6959), one thread-count golden (6946, to confirm)
status: open
opened: 2026-11-15
priority: P0
cost: M
refs: [6946, 6959, 6963, 6958]
---

Found on 2026-11-15, two ways: a reviewer of PR 6972 ran the sweep slow
set on a merge base that had PR 6959, and the nightly of that morning
(run 43680833019, main `dbb21b4a`) was diffed against the night
before's (run 43561237293, main `ff2b5f5d`). Twelve failing tests are
in the new nightly and not the old; these are the ones in files or
decisions BOXFINISH's four merged PRs own. Attribution is by the
panic's text and the file lists until each lane's measurement confirms
it.

**PR 6963 (the profile's recorded directions).** Five `editor-core::all`
rows, at every ε:
`revalue_corpus::revalue_boss_union_apart_to_its_parting`,
`revalue_plate_param_apart_to_its_parting`
(`crates/editor-core/tests/revalue_corpus.rs:488`),
`revalue_declared_tangency`, `revalue_die_chamfer`,
`revalue_measured_web` (`:429`). Each panic names
`segment_direction_axis` decisions answered with different signs
between two valuations. That reading is PR 6963's axis mint (audit row
F43), which its PR body argued no comparison could see. Its review
asked for the two-lanes case to be driven and it was not built.

**PR 6959 (a chart image stores its ends).** Two `sweep::all` rows, at
every ε: `split_edge_loft_charts::a_split_on_a_spline_chart_carries_the_parents_image`
(a child's `IsoLine` `ends` differ from the parent's) and
`m8_4_intersection_iso::a_boundary_column_intersection_mints_its_iso_image`
(stored `ends: Some(..)` against derived `ends: None`). Possibly also
`demos/tour` `k_probe_brackets::k_probe_brackets_the_lofted_leaf_at_eps_1e_9`
and `..._at_eps_1e_6` (wall 8's refusal moved; `demos/tour/src/walls.rs:33`):
a weak guess by name.

**PR 6946 (edge turns), to confirm.** `sweep::all`
`mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_one_thread`,
ε 1e-12 only. The unit changed what the whole-turn decision reads at
that ε; other PRs moved that golden in the same window.

**PR 6958 (the aimed ray).** Nothing attributed.

**Not BOXFINISH's, by the same diff:** `profile::all`
`raw_door_census` (sites in `crates/mesh/src/trimmed.rs`; GREEN holds
it), `review_chamfer_r1_probes`, `topo` `conic_quadric::cone_rows` and
`section_cert::square_search` at 1e-12 (the last prints a fuzz seed).

**Why the gate was green on all four PRs.** Its slow-set step runs only
slow rows whose names appear on a diff's added lines:
`work/ciw/a-stored-data-change-runs-none-of-the-slow-rows-that-read-it.md`.

**The repair.** Each PR's implementer session measures its rows on main
and on main before its PR, and the whole slow set of the crates it
touched, before proposing anything: branches
`boxfinish/profile-directions-revalue`, `boxfinish/chart-ends-slow-rows`,
`boxfinish/edge-turns-golden`. The revalue rows are not to be re-pinned
green: a re-valuation that flips a decision is what they exist to
catch.

**What closes it.** The nightly lists none of these rows, each fixed by
its cause; and each unit's model-compare row carries the escape
(`docs/MODEL-COMPARE-PROTOCOL.md` rule 10).

