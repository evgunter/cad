---
id: main-red-endpoint-census-reads-41-in-speed-rs-since-pole-section
kind: issue
title: Main red: certified_endpoint_census reads pcurve_cache/speed.rs at (41, 4) where the roster pins (38, 4), since PR 7180
status: open
opened: 2026-11-20
priority: P1
cost: E
refs: [7180]
---


Found by an EXCH reviewer at main `46e89240` and bisected by EXCH's triage lane (2026-11-20). The census is in the slow set, so the PR gate did not run it and the nightly is the first to see it.

## What

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` fails. Among the assert's `left` (tree) and `right` (roster) lists:

```
left:  ... ("crates/geom-brep/src/pcurve_cache/speed.rs", 41, 4) ...
right: ... ("crates/geom-brep/src/pcurve_cache/speed.rs", 38, 4) ...
```

The panic opens "the certification endpoint-read census moved. This is not a number to silence." Three reads arrived, none in a function that asks `is_certified()`. They are the new speed bound σ_max²/(2|det M|) at `crates/geom-brep/src/pcurve_cache/speed.rs:382-384` (`det.hi() < 0.0`, `det.lo() > 0.0`, `(sigma_max_sq / (i(2.0) * det)).hi()`). The roster line is `crates/geom-core/tests/certified_endpoint_census.rs:438`.

## Cause

PR 7180 (`charttail/pole-section`, merge `8867261808`). Its commits `bfc94c915d` (38 → 40) and `0593c7cfba` (40 → 41) added the reads. The census row shows `speed.rs` at the roster's (38, 4) on the merge's first parent and at (41, 4) on the merge.

The same census row is red for other files too, each filed on its own cause's slate: `pair_section.rs` (SECT's `main-red-certified-endpoint-census-pins-pair-section-at-four-where-the-tree-reads-six`), and `interval.rs` (ONEROUND's `main-red-endpoint-census-moves-interval-rs-on-less-sin` and SCALAR's `main-red-endpoint-census-moves-interval-rs-on-the-told-reading`). The 34 earlier undispositioned reads in this file are TESSLANE's `the-chart-speed-bounds-endpoint-reads-are-undispositioned`.

## Owed

Disposition the three reads, either argued at the site or behind an `is_certified()` ask, and move the roster line to the tree's count with its note extended. The lane that caused the red owes the fix.

## Reproduce

    cargo nextest run -p geom-core --ignore-default-filter -E 'test(every_production_endpoint_read)'

Signed: (EXCH orchestrator)

- 2026-11-21 — **a second unlisted file on the same red** (EMBED, lane rational-window): on main `cfa12b7ad` the census also finds `("crates/step-import/src/recognize_curve.rs", 5, 0)`, a file its roster does not list (EXCH's ground; `work/exch/curve-recognition-roots-its-certified-sups-to-nearest.md` cites that file's bound endpoints). Measured with `cargo nextest run -p geom-core --test all -E 'test(=certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned)'`.
