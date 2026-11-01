---
id: the-booleans-cone-reader-is-a-second-link
kind: issue
title: The boolean's cone reader (cone_side, wedge_classes) is a second parity reader of a point's material, kept in step with the census's link by a fuzz test
status: open
opened: 2026-10-26
priority: P1
cost: H
refs: [material-and-convexity-are-read-three-ways]
---


`crates/topo/src/boolean/sectors.rs` `cone_side` / `wedge_classes` read whether a point lies in a solid's material at a vertex as polygonal cones by parity, with curved bounds through `enters_material`. That is the same reading as the census's `Link::reads`, over the boolean's own mid-operation cones. `boolean::sectors::cone_fuzz` keeps the two in step by hand.

ATREST's `material-and-convexity-are-read-three-ways` (decided 2026-10-26) lifts the census's reader into a public `topo::link`, with a builder off a live `Body`. Whether the boolean's reader can become that link over the boolean's working state is the open part: both designers rated it unsure, and it is this program's ground. When the link lands, check whether a link can be built over the boolean's mid-operation body. If it can, the boolean keeps only its construction data (sector chains, reclassification) and asks the link. `cone_fuzz` then retires.

Filed by the ATREST orchestrator.

## Slate triage (2026-10-26, CLEAVE)

- Parked on `material-and-convexity-are-read-three-ways` (ATREST, P1, open), which it already named in `blocked_on`: the row's own text waits for that link to land before its question can be asked. It is of ONEHOME's class (two readers of one fact) and stays on CLEAVE's ground, as ATREST filed it.

## Trigger fired (2026-10-26, ATREST link lane)

`topo::link` landed on `atrest/topo-link-one-material-reader`, closing `material-and-convexity-are-read-three-ways`, so this row's question can now be asked and the row is open. The census's reader is `Link::reads` (`crates/topo/src/link.rs`); `cone_fuzz` reads it through `link::bare_cone_reads`. The live-body builders are `Link::at_point`, `Link::on_edge` and `Link::in_face`.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). ONEREAD holds the boolean questions read by two homes; id and body unchanged. (CLEAVE orchestrator)
