---
id: under-the-apart-move-loft-prism-parts-at-an-exactly-parallel-edge-pair
kind: issue
title: Under the apart move, loft_prism parts at an edge-pair census decision whose margin is exactly zero at the nominal
status: open
opened: 2026-11-02
---


**A main red, pinned.** `revalue_corpus::revalue_loft_prism`
(`crates/editor-core/tests/revalue_corpus.rs`) fails on main: run
un-pinned at the merge bases of PR #6341 (`49955c45`, `2bbe06a2`) and on
main after them, it reports the same Found as with that PR — `compared:
1539, moved: 857, flips: [], mode: Some(InStep)` — so the row has
compared in step, not under `Move::Apart` as its `rows!` entry (floor
259) asks, since before #6341. #6341 (a loft section named by a minted
`SectionId`) does not change it; it pinned the row red rather than leave
it failing.

`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`
listed `revalue_loft_prism` among main's slow-set reds and closed on
2026-10-30 while this row was still red.

**What happens.** Under `Apart` the `p0` and `p1` builds part: a prefix
comparison reads 242 decisions in step, 152 moved, and then flips
`pm_census_ee_parallel` at decision 242, whose margin is exactly `0.0`
at the nominal and `2.02e-8` at `p1`; the builds part there (`parted:
(242, 1539, "pm_census_span_precision")`). In step they part nowhere:
1539 compared, 857 moved, no flip. Seen at the witness ε only.

Why an edge pair of the prism reads as exactly parallel at the nominal,
and what tips it under the apart move, is not known.

**Pinned.** `revalue_loft_prism` leaves the `rows!` macro for an
explicit test under `pinned_red::row`, stopping on "compared under
another move than its pin" with `mode: Some(InStep)`, as
`revalue_heat_sink_fins` does.

**What closing it takes.** Find the edge pair the zero-margin decision
reads, and decide whether an exact parallel at the nominal should reach
a sign at all; then the row compares under `Apart` again and its floor
and owed flips are re-measured.
