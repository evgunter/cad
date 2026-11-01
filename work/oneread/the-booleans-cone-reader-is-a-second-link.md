---
id: the-booleans-cone-reader-is-a-second-link
kind: issue
title: The boolean's cone reader (cone_side, wedge_classes) is a second parity reader of a point's material, kept in step with the census's link by a fuzz test
status: spec
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

## Decided (2026-11-01, ONEREAD orchestrator, on a converged three-designer weighing)

Decided without Ev: the designers converged after one reconciliation round, no ratified text changes, and the one alternative that would have changed a ratified clause (naming reading the cone itself, against `names/README.md`'s "A boolean reads the sense at the step that mints the vertex") was not taken by any of them. Draw, first reports and rounds: `analysis/design-fork/oneread-cone-reader`.

Premise corrections all three made: `cone_fuzz` checks each reader against an exact oracle, never the two against each other; the cone reader reads curved faces and bounds at first order (`plane_side_code` with `NO_CURVATURE`), not through `pierce_side`; its answer feeds only the published `EdgePieceClass` rows and `UnreadPartner` (naming), never the boolean's construction. The duplicate is wider than `cone_side`: the boolean reads one partner vertex at a time and `vtxfac` re-nests the per-partner cones (`met`/`pointed`, `layered`, `layered_alone`, `interior`, `held`, `NestingUnread`), where one parity reading over every face of the other operand at the point answers directly. The convex/hollow shortcut in `wedge_read` is a `[one-way.readings]` fallback, and refuses bore rims (`NoBisector`).

The final state:

1. **`topo::link` splits into a `Cone` and a `Link`.** A `Cone` is a point and its sectors (a plane and a fan of rays, each ray fixed by a point at its reach): `closes`, `reads`, the apex `tie`. A `Link` is a `Cone` whose wedges are real face points, plus the ball, `offset`, `lowest_on`, `rest::judge`, `Round`, `Sector::cleared`. A cone of tangent sectors cannot be asked `offset` or `judge`: they are not on its type.
2. **One meter: the link's door.** Every sign is a point's signed distance from a plane, through `link::metric`. The boolean's levers (`least_lever`) go. Measured by two designers on `cone_fuzz` (seed `0xa92fdc75aec72e5a`, effort 1): the link's extra refusals against `cone_side` are not a metering gap but two refusals, both fixable inside the door:
   - a direction decided on a face's plane and on a bound two sectors share reads `On` (today `Place::Bound` → `None`): ~70% of the gap;
   - a direction in band of a face's plane but decidedly outside its sector is read as not crossing it (the wide parity, item 2 of ATREST's `the-census-reads-its-link-without-the-wide-parity`): ~25%.
   With both, the link refuses ~114 against `cone_side`'s 107 at ε 1e-9, ~61 against 42 at 1e-6, 2 against 5 at 1e-12, with zero wrong. **Not** `Reading::own` wholesale: reading at the probe's own distance decides 86 probes in band at 1e-9.
3. **The boolean builds one `Cone` of the other operand at each point it classifies**, gathered across keys as the census gathers: every partner vertex's faces, a touched face, an edge the vertex rests on. A curved face's sector is its tangent plane between its bounds' departure tangents (first order, as `cone_side` reads it today). The builder is sign-free, so `the_link_decides_only_through_its_doors` covers it. The orbit walk it reads (`sector_face`/`sector_shape`) moves to a home neutral between `boolean` and `link`.
4. **Deleted:** in `sectors.rs`, `wedge_read`/`wedge_classes`' shortcut and polygon-cone path, `cone_read`, `cone_side`, `WedgeRead::{met, pointed}`, `link_mean`, `in_sector`, `apart`, `arc_side`, `crossing`, `least_lever`, `GreatArc`, the `bool_cone_*` and `bool_corner_one_plane` K names; in `vtxfac`, the layering and `NestingUnread`; `UnreadNeighbourhood::{OneFace, NoBisector}`. `partner_side`'s construction checks stay. `cone_fuzz`'s exact-rational oracle and families move under `link` as the `Cone`'s test. `census_touch_*` K names become `link_*`.

## Lanes

- **Lane 1** (`oneread/link-cone`): items 1 and 2, in `link.rs` and the census. Dual review (an architectural split of a public type four consumers read).
- **Lane 2**: items 3 and 4, after lane 1 merges. Dual review (it moves every published vertex-vertex `EdgePieceClass` row).
