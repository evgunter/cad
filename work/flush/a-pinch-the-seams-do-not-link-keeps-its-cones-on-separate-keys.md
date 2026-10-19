---
id: a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys
kind: issue
title: A pinch whose keys no seam ties keeps its cones on separate point keys
status: closed
priority: P2
cost: M
refs: [4207]
opened: 2026-10-07
branch: flush/touch-key
pr: 4823
closed: 2026-10-14
---

Found by PR 4207's review (the FULL lane, M2). PR 4207 puts a pinch's
cone vertices on one point key where the seam correspondence ties their
keys (`zip::point_classes`, `zip::share_points`). This row is the rest:
pinches whose keys no seam record ties.

## What

- **`touch`: a point-touch union ships its pinch on two keys.** Two
  corners touching only at `v` union to one body pinched at `v`. No
  section curve runs through a touch at a point, so no null-pair record
  names the two operands' vertices there, and each keeps its own
  operand's point key. The result is `SOUND` only through the
  `VertexVertex` contact the union returns. Reused as an operand
  without that contact, the result is `dbl`'s shape. That is why
  `dbl`'s operands arrive with two keys.
- **`dbl3`: an operand pinched on three keys stays refused.** Three
  corners joined at `v` by two point-touch unions, then a cube whose
  face holds `v`. Of 72 lines, 40 stay `BAD` on tier 3′
  `UndeclaredContact { VertexVertex }` at `v`, byte-identical to main;
  `share_points` does not reach them. The likely cause, not traced line
  by line: the cube's seams pair only some of the cones' vertices at
  `v`, so a key whose vertex no seam reaches stays outside the class.
  Of the rest, 24 refuse `JoinDesync` and 8 are `SOUND` with one cone.

## Why it waits on D10

Nothing in the records ties these keys. Only two things could:
- a **declared contact**: the touch union's returned `VertexVertex`
  contact, carried with the body as intent;
- a **coincidence read**: comparing the vertices' positions.

The first is D10's question: one way to say intent. The second is
D10's undeclared-coincidence ground. So linking these keys waits on
`d10-one-way-to-say-intent-is-unbuilt`.

## Measured

PR 4207's review probes (branch
`join/pinch-cones-share-a-point-key-review`,
`REVIEW-4207-probes/r1_4139_probes_r4207.rs`, sets `touch` and `dbl3`
with the `R4_KEYS` column), release, main `3e9d1a96` vs PR 4207's head:
- `touch`: 84 lines, byte-identical; 28 of them (both orders) ship
  the pinch on two keys.
- `dbl3`: 72 lines, byte-identical, as above.


## Measured (branch `join/pinch-cones-split-at-insertion`)

`join_pierce_runs_sweep::pinch_runs_battery`'s pinched operand is a
point-touch union of two cubes, and the class reaches it. Read by
`pierce_point_finding` at the notch's corner, release, on PR 4207's head
`fb8c7cbb` with this branch's change: 468 `SOUND` bodies hold their
two vertices at `v` on two point keys. That is `ab U`, `ba U` and
`ba S`, 156 each. None of them is among the 217 lines that branch
builds. Those 217 share one key.

## Measured (PR 4249's fix pass, three pairs at one vertex)

PR 4249's review probes put `notch343`'s corner against three cubes
whose corners touch only there. The pinched operand is a three-cube
point-touch union, so the class reaches it:
- r1's `r1_three_cubes_at_one_vertex`: side-2 cubes, diagonals 120°
  apart (`crates/sweep/tests/join_pierce_runs_sweep/review_r1_4249.rs`
  on `join/pinch-cones-split-at-insertion-review-r1`);
- r2's `r2_shared_vertex_probe`, `R2_PART=tripod`: side-4 cubes, tilted
  (`join/pinch-cones-split-at-insertion-review-r2`).

**Already shipping on main `875e049a`**, release, `OK BAD t3p=false`.
Volume is exact, and there are three vertices for three cones, but on
two keys; tier 3′ gives `CensusUndecidable` "one passes into the other
where they touch". It is the same on PR 4249's head.
- `three`: 8 lines, `ab U` and `ba U` at `i=2,3 j=2,4 k=1`.
- `tripod`: 10 lines, `ab U` and `ba U` at `i=0 j=4 t=2 k=0`,
  `i=0 j=4 t=2 k=1`, `i=0 j=5 t=1 k=0`, `i=1 j=4 t=2 k=0` and
  `i=1 j=5 t=1 k=0`.

**Refused typed by PR 4249.** Where the insertion hangs runs at a
turned run's copy (`insert::hang_in_turned`) and the point's cones then
sit on several keys, the op refuses `PinchConesOnSeparateKeys`
(`zip::refuse_split_hung_points`, read off point keys and the seams'
classes). On main these lines refused `ClassificationInvariant`
(tripod: 3 refused `Euler`). Without the guard they built: 22 + 18
`OK BAD` of the signature above, and 11 `SOUND` bodies on two keys.
- `three`: 33 lines (`ab U` 19, `ba U` 7, `ba S` 7).
- `tripod`: 55 lines (`ab U` 27, `ba U` 14, `ba S` 14).

Pinned by
`join_pierce_runs_sweep::three_pairs_whose_hang_leaves_the_point_on_two_keys_refuse_typed`
(`three i=4 j=3 k=1` and `tripod i=0 j=4 t=1 k=0`). Once the keys link,
these lines should build, and the guard and that row's `ab U` assertion
go.

## Measured (PR 4249's second review, four pairs at one vertex)

The second review's probe sets `notch343`'s corner against four side-4
cubes whose corners touch only there: three tilted round the grid
direction, one along it, 916 poses past the overlap filter. It is
`rf_four_pairs_at_one_vertex` on
`join/pinch-cones-split-at-insertion-review-fix`, ported to PR 4249 as
`join_pierce_runs_sweep::four_pairs_battery`.
- **Already shipping on main `875e049a`**, the reviewer's count,
  unchanged on PR 4249's head: 24 lines `OK BAD t3p=false`, the
  signature above. They are `ab U` and `ba U` at:
  - `i=4 j=2 t=2 k=0`, `i=4 j=6 t=1 k=0`;
  - `i=5 j=1 t=2 k=2`, `i=5 j=6 t=0 k=0`, `i=5 j=6 t=1 k=0`;
  - `i=6 j=1 t=2 k=2`, `i=6 j=6 t=0 k=0`, `i=6 j=6 t=1 k=0`;
  - `i=7 j=6 t=0 k=0`, `i=7 j=6 t=0 k=3`, `i=7 j=6 t=1 k=0`;
  - `i=8 j=6 t=1 k=0`.
- **Refused typed by PR 4249:** 167 lines `PinchConesOnSeparateKeys`.
  Without the guard they are 112 `OK BAD` and 55 two-key `SOUND`
  bodies.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: only a declared VertexVertex contact or a coincidence read can link the keys; stage 4 retires the first and moves the second to the one door. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E does not reach this. Its glue door reads face pairs (`crates/topo/src/boolean/glue.rs:40`). It reads no vertex coincidence and links no point keys, and the touch union's `VertexVertex` contact is still not carried as intent. `PinchConesOnSeparateKeys` still guards the hang (`crates/topo/src/boolean/mod.rs:2416`, raised by `zip::refuse_split_hung_points`, `crates/topo/src/boolean/zip.rs:372`). Under D10 the link is a coincidence read, a vertex pair decided Zero and recorded. That is not built.

## Measured (2026-10-11, branch `flush/pinch-cluster`)

The pinch cluster mints the pierces of one face at one point as one
ring (`vtxfac::mint_ring`). Where a pinched operand on two keys pierces
a face at its pinch (`dbl`'s shape), both of its keys now pair with
the ring's copies, which share one key. So the seams tie all three
(`zip::point_classes`), and `share_points` moves the cones onto one.
`join_pierce_runs_sweep::dbl_battery` (2 160 lines, new) goes from 968
`SOUND` to 2 120.

Nothing else in this row moves. `touch`, `dbl3`, `three`, `tripod` and
`four_pairs` are vertex-vertex pinches, with no pierce. The release
batteries are byte-identical with and without the change (a build
switch that restored the per-pierce rings and the weld):
`four_pairs_battery` (5 334 lines, its `PinchConesOnSeparateKeys`
refusals included) and `pinch_runs_battery` (3 024). Still nothing in
the records ties a point-touch union's two keys, so the row stands on
D10's coincidence read, as its INTENT stage 4 E section says.

## Measured (2026-10-12, branch `flush/pinch-residue`)

On main `300f7cefe` and on the branch alike (the branch's two fixes
move no line of it): `four_pairs_battery` holds 287
`PinchConesOnSeparateKeys` and 24 `OK BAD` lines whose finding is "the
vertices at v do not share one point". `dbl_battery` is 2 160 `SOUND`
on the branch: its two-key operand no longer reaches a refusal.

A plain point-touch union of two cubes at a corner (a lane probe, not
committed) is `BooleanResultKind::Assembly`: **two solids**, their two
vertices at `v` on two point keys, one `VertexVertex` record between
them citing one coincidence row (`DecisionSite::VertexFusion`, margin
0). So ruling B's record half is already there; what is missing is the
one key. The pinched operands of `four_pairs`, `three` and `tripod` are
chains of such unions, and `unite_all` passes on the body without its
records.

## Why the lane stopped

PR 4057's ruling is "a pinch is one vertex per cone: several vertices
on one point key, and no face crosses between cones", and ruling B
(PR 4728) records each value-decided grouping that leaves cones
distinct at one key. Both read the cones of one body. The touch union
here returns two parts of an assembly in contact at a point. Whether
two parts' contacting vertices share a point key is not settled by
PR 4057, ruling B or D10's coincidence paragraph, and each answer
changes what the census reads off keys across solids. The final states:

- **A. One key at the touch door.** Where a boolean decides two
  operands' vertices coincide and returns them as a `VertexVertex`
  contact, it puts them on one point key; the record still cites its
  `VertexFusion` row. Ruling B's shape, extended from one body's cones
  to an assembly's parts. A plain boolean reusing the result reads the
  pinch from its keys, so the 287 typed refusals and 24 wrong bodies
  should build (not measured). The census's same-point rung then reads
  two solids' vertices as one point by structure.
- **B. Keys per part; the carried record ties them.** The touch's
  contact rides into the next op as an operand record
  (`BooleanInputs { records_a, records_b }`), and the door ties the keys
  a carried `VertexVertex` record names (`zip::point_classes` reads it
  as it reads a seam tie). A plain boolean that drops the records keeps
  refusing `PinchConesOnSeparateKeys`, typed. The natural spelling,
  `union(&acc, &b)` on the last result, drops them.
- **C. The later door decides.** Where an op's result brings two
  vertices at one point into one solid (the hang, the split), the door
  decides their coincidence by value (`one_vertex_at`, its margin),
  records it as a `VertexFusion` row with a citing record, and shares
  the key there. Nothing changes at the touch union; positions are read
  at the door that makes the pinch.

## Measured (2026-10-12, branch `flush/touch-key`, main `360cf55e`)

Release, base against head. Re-run after merging main `34097ca6`
(PRs 4777, 4765 and 4398): base `34097ca6` against the merged head, the same
`four_pairs` lines move and the other four batteries are
byte-identical (`dbl_battery` is 2 159 `SOUND` on both: PR 4777 built
its 36 `ZipCorrespondence` lines).

A plain point-touch union of two cubes
returns an assembly whose two vertices at `v` sit on two keys, with one
`VertexVertex` record citing a `VertexFusion` row of margin 0. On main:
`four_pairs_battery` holds 287 `PinchConesOnSeparateKeys`, 24 `OK BAD`
whose finding is "the vertices at v do not share one point", 982
`SharedVertexCrossings` and 3 896 `SOUND`, 477 of those on two keys.
`dbl_battery` holds 36 `ZipCorrespondence` (PR 4777's) and 2 123
`SOUND`; `pinch_runs` 3 023 `SOUND`; `corner_pairs` 16 313 `SOUND`;
`pierce_runs` 4 325 `SOUND`. The ported PR 4207 review probe
(`touch`, `dbl3`, plain booleans): `touch` 56 `SOUND` and 28 `EMPTY`,
28 of the `SOUND` on two keys; `dbl3` (216 lines now, its operand on
three keys) 180 `SOUND` and 36 `OK BAD`, every one tier 3′
`UnrecordedContact { VertexVertex }` at `v` (the row's 40
`UndeclaredContact`, renamed and re-counted).

## Fix (branch `flush/touch-key`)

Ruling A (the FLUSH orchestrator's): a boolean that decides from
values that vertices of its operands coincide puts them on one point
key in its result. The output stage of every path (seamed, assembly,
single-operand fallback; `ops.rs` `share_recorded_points`, after
`carry`) reads the result's vertex-vertex records into classes of
point keys (`zip::record_classes`) and moves each class onto one key
(`zip::share_points`, `Body::share_point`). No position is read; the
records and their `VertexFusion` rows stay. The seam correspondence's
own classes (`zip::point_classes`, rebound before the join) stay a
second source: removing them moves no line of the five batteries, but
main's `records_cite_their_decision::copies_an_operand_already_held_on_one_key_record_no_pinch_row`
(PR 4765, `cdafc97b`) splits an operand's two structural copies onto two keys
without them (`[1, 1, 0] half-lobes intersect`). Those copies carry
no vertex-vertex record, so the record rule cannot reach them, and the
seam tie is not a special case of it. `zip::refuse_split_hung_points`,
`BooleanError::PinchConesOnSeparateKeys` and the insertion's hang
bookkeeping it read (`HungPoint`, `hung_points`, `Hang`) are retired.

Head against base: `four_pairs` 287 `PinchConesOnSeparateKeys` → 115
`SOUND` and 172 `OK BAD`; the 24 `OK BAD` keep their tier 3′ refusal
with the key finding gone; the 477 two-key `SOUND` lines now hold one
key; `SharedVertexCrossings` (982) is identical. `dbl`, `pinch_runs`,
`corner_pairs` and `pierce_runs` are byte-identical. `touch`: the 28
two-key lines hold one key, outcomes unchanged. `dbl3`: its operand
now holds one key, and all 216 lines are `SOUND`; the 36 refusals went
because the cube's union reads the operand's corners off one key.

The output stage's join (`edge_join`) refuses to join a vertex whose
point key another vertex shares: the joined edge would run through the
other vertex, a vertex-on-edge contact no record names. Shared keys
now also mark two lumps touching, and an operand carrying its records
in (`BooleanInputs::records_*`) names that contact. So the boolean's
join frees a vertex its operands' carried vertex-vertex records name
(`ops::carried_vertices`, read as `carry` reads them) where no other
vertex of its shell sits on its key (`edge_join::free_vertices`); a
pinch's cones lie in one shell and stay. Without it, `topo`
`union_flush_onto_edge_contact::joined_folds_certify_and_build_one_body_in_every_order`
kept a joinable vertex in two of its six member orders (17 vertices
against 16). A first cut that freed every vertex shared only across
shells, records or not, moved 195 `dbl_battery` lines `SOUND` →
`OK BAD` (an unrecorded vertex-on-edge at `v`); with the record test
the five batteries are byte-identical to the measurement above.

Tests restated for the new shape: a vertex pair or an edge overlap
between two shared keys now passes tier 3′ with its records withheld
(`m3_pr6_tier3prime`, `surgery_carries_records`,
`union_flush_onto_edge_contact`, `pierce_strut_at_a_pinch`,
`sweep` `blend_carries_records`, `stl` `review_m3_pr6_e2e`), and `boolean_pinch_copies` allows
two vertices on one key where a record ties them. Re-baselined, through
`kiss_carry` alone: the m10-p fences, `intent_s2_c_world::PRE_C` and
`seat4_verb_lowering`'s digests (its body holds one point fewer; no
outcome or coordinate moved).

The 196 `four_pairs` `OK BAD` and the `three` / `tripod` unions refuse
tier 3′ `MixedTouch` at `v` with the oracle's volume, one key and a
clean mesh: filed as
`work/restread/a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap.md`.
A result reused without its records now passes a point touch at rest
on its shared key: filed as
`work/intent/a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys.md`.

## Fix pass (PR 4823's review, 2026-10-13)

- **The offset doors part a corner kiss the offset pulls apart.**
  `replace_face::group_by_point` groups moved corners by point key, so
  the two kissing corners were solved as one six-plane corner and
  `offset_planes_together` refused `TogetherCorner { planes: 6 }` (the
  general door, `VertexDisagreement`). Grouping by solid instead broke
  main's `m3_pr3_split::an_offset_that_moves_both_tip_copies_together_keeps_them_touching`
  (a split's two halves keep one point at a tip, and must move as
  one). So each door (`offset_together`, `offset_axial`,
  `replace_face::solve_corners`) solves a key's corners together first,
  and where a group spanning two solids refuses, solves each solid's
  corner alone and the corners part (`replace_face::split_by_solid`).
  Pinned by `topo` `offset_a_corner_kiss` (planar and general doors,
  five distances, two frames); the axial door takes no planar body, so
  its arm is unpinned.
- **A forged carried record refuses stale.** Main's carry now confirms
  each carried record by the census's read (`records::landings`)
  before it reaches the result, so a carried vertex pair 1 or √3 apart
  refuses `CarryRefused { Stale }` and moves nothing; an honest one
  still rebinds. Pinned by `topo` `carried_vertex_pairs_share_a_key`.
- **The seam classes' second pin.** `records_cite_their_decision::an_operands_copies_on_one_key_stay_on_one_key_at_every_normal`:
  43 of 120 runs split an operand's copies onto two keys without
  `zip::point_classes`.
- **The join's freeing, second pin.** `union_flush_onto_edge_contact::a_pinch_lines_end_joins_where_the_next_union_straightens_it`
  builds 17 vertices against the other order's 16 without it.
- The pins that pass a withheld record name the INTENT row
  `a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys`
  as their open state, and assert the keys.
