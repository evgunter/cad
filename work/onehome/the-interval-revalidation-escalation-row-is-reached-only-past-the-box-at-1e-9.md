---
id: the-interval-revalidation-escalation-row-is-reached-only-past-the-box-at-1e-9
kind: issue
title: The wire-loft row whose interval revalidation escalates while f64 decides is reached only past the session box at ε = 1e-9 and 1e-6
status: open
opened: 2026-11-08
---


Filed by PR 6380 (`onehome/run-owns-rounding`), at its merge of main after
PR 6371.

**Finding.** `crates/sweep/tests/wire_loft_end_profile_lift.rs`'s
`separating_pair` searches a ladder of wide-arc pairs for one whose
profile `f64` decides and `Interval` escalates (the
`a_section_whose_interval_revalidation_escalates_is_not_refused_by_the_profile_door`
and `the_wide_arc_section_decides_and_lofts_at_f64` rows). With the session
box at every operation's finished-body gate (PRs 6371, 6380), every rung
that separates at ε = 1e-9 or 1e-6 is a pair whose arcs' circles lie far
past the 1 km box (measured margins −499 km at 1e-9, −5e3 to −5e6 km at
1e-6), and the loft is refused there as `OutsideSessionBox`. Inside the
box the enclosure's width is a few ulps of the box, far below those bands,
so nothing in the box separates. At ε = 1e-12 an in-box pair does
(`(50·s, 1e3)`, its circle about 25 m), and the rows exercise their subject.

**Now.** `separating_pair` stands the two rows down, loudly, where every
separating rung was the box's refusal, and still fails on any other miss.

**Wanted.** Either an in-box configuration that separates the two
arithmetics at the shipped ε (if one exists, the ladder gains it), or a
ruling that this door's escalation case is a 1e-12-only subject, recorded
at the row.
