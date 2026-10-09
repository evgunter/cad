---
id: rest-zip-drops-the-euler-operators-refusal
kind: issue
title: zip: the declared-REST zip discards the Euler operators' own refusals (map_err(|_| …)) and reports only a sub-frontier or a desync
status: closed
opened: 2026-09-30
priority: P3
cost: M
closed: 2026-10-08
---


(TOPO, the second fix pass of PR 3513; predates it.)

## What

`crates/topo/src/boolean/rest.rs` wraps each Euler operator the
declared-REST zip calls with `map_err(|_| …)`, so the operator's own
`EulerOpError` (which names the site and the invariant it refused on)
never reaches the refusal:

- the seam chord's `mef_chord` and `mekr_chord` (`seam_chord`, near
  :1223, :1234, :1242 and :1249), which PR 3513 types as
  `RestZipFrontier::{ChordMefRefused, ChordMekrRefused,
  PierceRingMekrRefused}` and ends in `geom_core::NOT_YET_ENDING`;
- fourteen `desync("REST lane: … refused")` wrappers, which render as a
  `BooleanError::JoinDesync` kernel-bug report: `single_solid` (near
  :267), the strut undo `kev` (:1104), ring promotion `mfkrh` in
  `glue_pair` (:1622, :1629), the run `kef`/`kev` (:1814, :1840), the
  band run's `kev`, `kemr`, `mfkrh`, `kef` (:1891, :1895, :1915,
  :1917), the band ring `mfkrh` (:1940), the final slit `kef` (:2016)
  and the slit fuse `kev`/pair `kef` (:2070, :2074).

A refusal that drops its cause cannot say which of the operator's
preconditions failed, so the three typed sub-frontiers read "no way
through yet" where the operator may have named a defect, and the
desyncs report a kernel bug with nothing to report.

## Receipt

`map_err(|_|` in `rest.rs`: 18 hits, listed above. Not searched: other
zip files (`zip.rs`, `slit_zip`'s helpers outside `rest.rs`) and
`map_err(|_e|` spellings.

## Repair shape

Carry the operator's `EulerOpError` as the refusal's source (a
`source` field on `RestZipUnsupported`'s three operator arms and on the
desync, or a typed arm per operator), so the rendered refusal can end
as the operator's own decision does.

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/join/log.md`, the 2026-10-03 hold). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: all 17 map_err wrappers are in boolean/rest.rs's surgery, which stage 4 deletes with RestZipFrontier. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed (2026-10-08, INTENT stage 4 A (`intent/s4-a-join`))

`boolean/rest.rs` is deleted, and the join builds every union it built (125 of the door's 185 openings across the topo, sweep and editor-core suites; the other 60 were its declines and refusals, which the join now builds sound or refuses with its own answer). The seventeen `map_err` wrappers were all in it.
