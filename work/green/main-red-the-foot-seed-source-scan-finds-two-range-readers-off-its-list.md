---
id: main-red-the-foot-seed-source-scan-finds-two-range-readers-off-its-list
kind: issue
title: Main red: geom's a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound finds a seed() read in the viewer's and the Python binding's range readers, off its list
status: open
opened: 2026-11-23
priority: P1
cost: E
---


Reported by three lanes of ONEROUND's PR 7510 on 2026-11-23, each from a run of clean main: the implementer on `53e08fc62`, and its two reviewers on `c9cf004f9` (`analysis/oneround/conic-readers/review-report-delta.md` on `oneround/conic-readers-review-r1` and `-r2`). Not run by me.

`cargo nextest run -p geom` on main is red in one test at every ε: `curves::on_carrier::rows::a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound` (`crates/geom/src/curves/on_carrier/rows.rs`). It is a source scan for reads of a foot's `seed()` outside the places that seed a search or a bound, and it finds one read each in `crates/viewer/src/session/range.rs` and `crates/pncad-py/src/py/range.rs`, neither on its list.

It reds every PR whose diff seeds `geom` (the fast set runs it). Owed: the two reads judged (a seed used as a search's start, or a seed read as an answer, which the scan exists to refuse), then listed with their reason or re-posed; and the PR that added them named here, so the lane that caused the red owns the fix.
