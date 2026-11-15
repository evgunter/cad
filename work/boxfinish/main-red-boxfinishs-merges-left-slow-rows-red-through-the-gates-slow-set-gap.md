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
(stored `ends: Some(..)` against derived `ends: None`). Measured by the
PR's lane: red after the merge and green on its first parent at three ε.
The split row is re-stated (a child states its end at the cut on
purpose) and the mint row goes back to its first contract (the lane's
own mechanical edit wrote `ends: None` into it).

Not this PR's, measured: `demos/tour`
`k_probe_brackets::k_probe_brackets_the_lofted_leaf_at_eps_1e_9` and
`..._at_eps_1e_6` are red on the PR's first parent too. Filed on GREEN:
`work/green/the-tours-k-probe-reads-lily-wall-8-at-the-probe-scalar-where-it-refuses-nomarch.md`.

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


## The thread-count line, pinned and still to confirm (GREEN `green/nightly-1115`, PR 6985, 2026-11-15)

The "PR 6946, to confirm" row above is one line of
`crates/sweep/tests/thread-count-digest/eps-1e-12.txt`:
`arc_loft_1e11eps`' pads. On main `3e7a8c93b` the one-thread and the
four-thread walk read the same line, with the volume and area bits
unmoved and both pads wider: `vpad` `3db30319374e81b6 -> 3db303198d26e97a`
(1.7291407e-11 to 1.7291412e-11, 2.7 parts in 10⁷), `apad`
`…d630 -> …d690` (2 parts in 10¹⁴). GREEN pinned the line at the serial
reading and named this row in `expected`'s doc.

**PR 6946 is NOT confirmed.** GREEN's two-commit check (the walk at
`980a97ec9^1` and at `980a97ec9`, ε 1e-12, `opt-level = 2`) did not
complete: the first build was killed for memory on the lane's box. The
attribution stands as BOXFINISH's inference.

Reproduce:
`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep -E 'test(the_walk_matches_the_serial_golden_at_one_thread)'`
at those two commits; the assertion prints the line.
