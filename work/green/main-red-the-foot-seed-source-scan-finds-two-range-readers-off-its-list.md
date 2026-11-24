---
id: main-red-the-foot-seed-source-scan-finds-two-range-readers-off-its-list
kind: issue
title: Main red: geom's a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound finds a seed() read in the viewer's and the Python binding's range readers, off its list
status: closed
opened: 2026-11-23
priority: P1
cost: E
closed: 2026-11-23
pr: 7670
---


Reported by three lanes of ONEROUND's PR 7510 on 2026-11-23, each from a run of clean main: the implementer on `53e08fc62`, and its two reviewers on `c9cf004f9` (`analysis/oneround/conic-readers/review-report-delta.md` on `oneround/conic-readers-review-r1` and `-r2`). Not run by me.

`cargo nextest run -p geom` on main is red in one test at every ε: `curves::on_carrier::rows::a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound` (`crates/geom/src/curves/on_carrier/rows.rs`). It is a source scan for reads of a foot's `seed()` outside the places that seed a search or a bound, and it finds one read each in `crates/viewer/src/session/range.rs` and `crates/pncad-py/src/py/range.rs`, neither on its list.

It reds every PR whose diff seeds `geom` (the fast set runs it). Owed: the two reads judged (a seed used as a search's start, or a seed read as an answer, which the scan exists to refuse), then listed with their reason or re-posed; and the PR that added them named here, so the lane that caused the red owns the fix.

## 2026-11-23 — judged: neither is a foot's seed (PR 7670)

Both reads are `editor_core::CertifiedRange::seed()` (`crates/editor-core/src/range.rs`), "how far the search looked": the window of the certified range search, returned as a `RangeSeed`. Neither reads a `geom::Foot`. The scan counts `.seed()` by text, so it cannot tell the receiver's type, which is how `geom-brep/src/ssi/exhaust.rs` (a cell's own `seed()`) is already on its list.

- `crates/viewer/src/session/range.rs` `said`: renders the range's searched window as the "Checked" line. It came with OFFER's `e17467aada` (PR 7548, `offer/range-is-the-certified-search`).
- `crates/pncad-py/src/py/range.rs` `CertifiedRange::seed`: the Python getter for the same window. It came with OFFER's `7fc414a729` (PR 7369, `offer/the-certified-range-refines-toward-the-current-value`).

So neither read is a seed taken as an answer, and there is no defect to file. Both are listed in `READERS` with that reason. A raw grep of `.seed()` and `Foot…::seed` over every `crates/*/src` turns up no other file off the list. The blind spot is the scan's own text-only shape: it does not see a foot's seed read through any spelling other than `.seed()` or a `Foot…::seed` path, and it counts every other type's `seed()`.
