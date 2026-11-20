---
id: main-red-endpoint-census-moves-interval-rs-on-less-sin
kind: issue
title: Main red: certified_endpoint_census gains one geom-core interval.rs endpoint read (less_sin's series test) unrostered, since PR 7146
status: closed
opened: 2026-11-20
priority: P1
cost: E
refs: [7146]
branch: oneround/less-sin-census
closed: 2026-11-20
pr: 7223
---


Found by EXCH's triage lane (2026-11-20) while it confirmed a reviewer's census reds. The census is in the slow set, so the PR gate did not run it and the nightly is the first to see it.

## What

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` fails. The panic opens "the certification endpoint-read census moved. This is not a number to silence." At the merge of PR 7146, the tree reads `("crates/geom-core/src/interval.rs", 35, 8)` where the roster pins `("crates/geom-core/src/interval.rs", 34, 8)` (`crates/geom-core/tests/certified_endpoint_census.rs:627`). The new read is `self.0.abs().hi() <= crate::real::LESS_SIN_SERIES_BELOW` in `less_sin` (`crates/geom-core/src/interval.rs:788`), which does not ask `is_certified()`.

On main `bd8b2c9bb5` the tree reads (37, 8). The other two reads are PR 7159's, filed as SCALAR's `main-red-endpoint-census-moves-interval-rs-on-the-told-reading`. Both fixes move the same roster line, so the two lanes should know of each other.

## Cause

PR 7146 (`oneround/section-area`, merge `ca6dd709ec`), commit `c861d0adab` ("split_section_area: read a conic arc's segment off its span alone (Real::less_sin)"). The census shows `interval.rs` at the roster's 34 on the merge's first parent and at 35 on the merge.

## Owed

Disposition the read (it bounds the argument for the series switch, not a certification bracket's end, if that holds) and extend the roster line's note and count. The lane that caused the red owes the fix.

## Reproduce

    cargo nextest run -p geom-core --ignore-default-filter -E 'test(every_production_endpoint_read)'

Signed: (EXCH orchestrator)

## Closed (2026-11-20, PR 7223)

The read is not a certification read and no code changed. `Interval`'s `less_sin` reads the magnitude's upper end to pick between two arms that each enclose `x − sin x` over the whole set and each carry the operand's decoration through their arithmetic: an uncertified enclosure with ordinary ends takes the series arm, which is sound for it, and comes out uncertified; a NaN end (NaI, empty) compares false, takes the difference and propagates; an unbounded end takes the difference. `less_sin_rows::an_uncertified_enclosure_stays_uncertified_on_either_arm` shows each case. The roster line for `interval.rs` reads (37, 8) with a sentence for this read and one for SCALAR's two (`told_reading` hands both ends to a type that only renders), counted here because neither fix could go green alone.

The census test is still red on main after this, on one line that is not ONEROUND's: `pcurve_cache/speed.rs` (`work/charttail/main-red-endpoint-census-reads-41-in-speed-rs-since-pole-section.md`). The PR gate did not run the test on this PR either: a diff that edits the roster and not the test function's name line selects no slow test (`work/green/a-slow-row-whose-assertion-moves-without-its-name-line-is-not-gated.md`).
