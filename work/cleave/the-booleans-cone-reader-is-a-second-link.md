---
id: the-booleans-cone-reader-is-a-second-link
kind: issue
title: The boolean's cone reader (cone_side, wedge_classes) is a second parity reader of a point's material, kept in step with the census's link by a fuzz test
status: parked
opened: 2026-10-26
priority: P1
cost: H
blocked_on: [material-and-convexity-are-read-three-ways]
---


`crates/topo/src/boolean/sectors.rs` `cone_side` / `wedge_classes` read whether a point lies in a solid's material at a vertex as polygonal cones by parity, with curved bounds through `enters_material`. That is the same reading as the census's `Link::reads`, over the boolean's own mid-operation cones. `boolean::sectors::cone_fuzz` keeps the two in step by hand.

ATREST's `material-and-convexity-are-read-three-ways` (decided 2026-10-26) lifts the census's reader into a public `topo::link`, with a builder off a live `Body`. Whether the boolean's reader can become that link over the boolean's working state is the open part: both designers rated it unsure, and it is this program's ground. When the link lands, check whether a link can be built over the boolean's mid-operation body. If it can, the boolean keeps only its construction data (sector chains, reclassification) and asks the link. `cone_fuzz` then retires.

Filed by the ATREST orchestrator.

## Slate triage (2026-10-26, CLEAVE)

- Parked on `material-and-convexity-are-read-three-ways` (ATREST, P1, open), which it already named in `blocked_on`: the row's own text waits for that link to land before its question can be asked. It is of ONEHOME's class (two readers of one fact) and stays on CLEAVE's ground, as ATREST filed it.
