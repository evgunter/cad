---
id: main-red-endpoint-census-reads-41-in-speed-rs-since-pole-section
kind: issue
title: Main red: certified_endpoint_census reads pcurve_cache/speed.rs at (41, 4) where the roster pins (38, 4), since PR 7180
status: closed
opened: 2026-11-20
priority: P1
cost: E
refs: [7180]
branch: charttail/speed-census
pr: 7509
closed: 2026-11-23
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
- 2026-11-22 — **a third unlisted file** (EMBED, lane strip): on main `b99f71c3d` the census also finds `("crates/step-import/src/on_built.rs", 24, 0)`, a file its roster does not list. The file came with EXCH's PR 7304 (`exch/adopt-by-description`). `speed.rs` still reads (41, 4) and `recognize_curve.rs` (5, 0). Measured in a clean worktree of that main with `cargo nextest run -p geom-core --test all -E 'test(/^certified_endpoint_census::/)'`.
- 2026-11-22 — **a fourth file moved** (EMBED, lane strip, PR 7334's gate at `5eca1351a`, which selects the census): `("crates/geom-brep/src/ssi/enclose.rs", 42, 41)` where the roster pins (39, 38), beside `speed.rs` (41, 4), `on_built.rs` (24, 0) and `recognize_curve.rs` (5, 0). The census's `injective.rs` entry matches that head.


## 2026-11-23 — the three reads are gone, and what the census still reds on

`pole_azimuth_speed` now reads its bound as the magnitude of the certified quotient `σ_max² / (2·det M)` (`Certification::mag`) behind the quotient's own `is_certified()` ask: a `det M` enclosure that holds or touches zero refuses the division, which is the `ThroughSingularSet` refusal the lower-end sign test spelled. No endpoint is read, so `speed.rs` is back at the roster's `(38, 4)` and its line does not move. The det sign is not a funnel decision: it selects no topology and has no band, only whether the quotient's divisor certifies away from zero (PR body).

On main `ea1aca811b` the census still reds on files of other programs, none of them this row's: `pair_section/ruling.rs` (SECTCONE's `main-red-endpoint-census-reads-ruling-rs-at-nine-since-the-cone-ruled-side`), `topo/src/props.rs` (BOXFINISH's `main-red-endpoint-census-reads-topo-props-rs-at-eleven-since-the-volume-premises`), and step-import's `on_built.rs` and `recognize_curve.rs` (EXCH's `main-red-endpoint-census-has-no-pins-for-step-imports-on-built-reads`). The `pair_section.rs` and `interval.rs` lines this row named have since matched.
