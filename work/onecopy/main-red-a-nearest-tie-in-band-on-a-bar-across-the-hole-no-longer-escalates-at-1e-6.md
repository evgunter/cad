---
id: main-red-a-nearest-tie-in-band-on-a-bar-across-the-hole-no-longer-escalates-at-1e-6
kind: issue
title: Main red at eps 1e-6: since PR 6752, the plane-torus bar across the hole whose germ ends tie in band no longer escalates bool_join_nearest
status: open
opened: 2026-11-11
priority: P1
cost: M
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`). Red at ε 1e-6 only, where the row's tie lies in band. It was green in the nightly before (42761779270, `50cfcf499d`).

**What fails.** `sweep::all germ_plane_torus_frame::a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name` (`crates/sweep/tests/germ_plane_torus_frame.rs`): `T ∪ P: the tie escalates the ranking: None`. At 1e-6 the row expects every op to escalate `bool_join_nearest` as a join coincidence, because the two germ candidates' 3.2e-6 m tie sits in the band. The union now does not escalate it.

**Bisected, first-parent, at ε 1e-6:**
- green at `2459d705`, `eeb308be` (PR 6764);
- red at `cc58938c`, the merge of PR 6752 (`onecopy/first-pair-all-mutual`);
- also red at `8c65db4d`, `dd2dd683`, `b9b97c8d`, `2e4c44de` and main `019bed88`.

Decide whether the union's new outcome is right: a join that no longer reads the in-band tie, or a pairing that no longer reaches the ranking. If it is right, re-pin the row with the argument. If not, the first-pair change lost an escalation.
