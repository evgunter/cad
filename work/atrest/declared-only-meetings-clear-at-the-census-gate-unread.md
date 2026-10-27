---
id: declared-only-meetings-clear-at-the-census-gate-unread
kind: issue
title: A pair whose only meetings are records has some taken on their word at the census gate; after stage 4, re-read whether any op can mint records that hide an overlap
status: open
opened: 2026-09-26
priority: P2
cost: M
refs: [CONTACT-13, intent-stage4-is-built, the-at-rest-census-is-a-check, the-touch-analysis-reads-curved-cones-tighter-than-the-reach-box]
---

## Where it stands (2026-10-27): one row, re-read after stage 4

This row now carries `CONTACT-13` as well (closed into it). Both moved
here from CONTACTHOLD by `git mv`, because what is left is the census's
records (ATREST's ground). They were P0s parked on
`intent-stage4-is-built`, which closed on 2026-10-24. Nobody did the
re-read that row asks for when it closes, so this row owes it.

**What stage 4 removed.** Every wrong clear below was built by
declaration: a test pushes `VfContact` and `PatchContact` records into a
`ContactRecords` by hand and hands them to `validate_pseudomanifold`.
The red rows are on branch `contact/13-meeting-ledger` (`3a1c22add`),
in `crates/topo/tests/contact13_declared_meetings.rs` and
`crates/sweep/tests/contact13_curved_declared.rs`. On main that cannot
be written:
- `ContactRecords`' rows are private and `push` is `pub(crate)`
  (`crates/topo/src/boolean/records.rs:317-342`);
- records are minted only by the kernel's operations, each citing the
  decision behind it;
- `ContactClass` and `MintedDeclaration` are gone.

So the rows do not compile against main, and no reachable pose is known.
That is why the row is P2, not P0.

**What stayed.** The census still reads a record-only pair's records with
`records_on_their_word` (`crates/topo/src/census.rs`, `blocks`, and the
gate comment that cites this row). What it now takes on its word is a
record an operation decided, not a user's declaration. PR #5494 (each
operator decides the meetings between its operands) is the ruling that
makes that the intended design.

**The re-read, which is this row's work:**
1. Try to build the three committed poses on main through operations,
   not hand-built records:
   - the I-profile through a slab;
   - the log dipping into a wall;
   - each with its true-rest control.

   The question is whether any op, or a carry through a later op, can
   leave a record that states a side the geometry contradicts while no
   vertex sits strictly inside. Rows on that ground include
   CONTACTHOLD's
   `a-cut-through-a-recorded-touch-mints-a-pair-no-record-backs` and
   `a-carried-line-contact-an-op-cuts-is-dropped`.
2. If no op can mint such records, close this row as retired by stage 4.
   Rewrite the census module doc and the gate comment that cite it so
   they say the record-only reading rests on #5494. Re-read Ev's
   decision 1 on PR 3422 ("a declaration licenses a coincidence, never
   a side") against #5494, and say in the PR how the two fit.
3. If one can, commit that pose as a red row and re-band this row on
   it. Under stage 5 C (`the-at-rest-census-is-a-check`) an overlap
   between copies is an `AtRest` interference finding, and that is the
   acceptance `CONTACT-13` carried ("an interference that no
   declaration can excuse must be a finding there").

The history below is from before stage 4 and is kept as it was written.
Read "declared" there as "recorded by declaration".

Filed by CONTACT-5 and re-scoped by its fix pass. Arm 2 of
`sweep_cross_solid_backstop` (`crates/topo/src/census.rs`) now counts a
declared record naming one entity of each solid — v-on-f, v-v, or a
curve or patch face pair (`meets_declared`) — as a meeting, and a pair
that meets has every vertex probed against the other's material in
both orderings. That closes both demonstrated wrong clears:

- a prism standing on a wall with a point dipping 0.1 into it, its
  four resting corners declared v-on-f
  (`contact5_gate_and_beam::a_declared_rest_with_a_dipping_point_is_probed`);
- a seat declared as two patch records with a V keel hanging 0.5 into
  the block (`contact5_gate_and_beam::a_declared_seat_with_a_keel_is_probed`).

Both passed `validate_pseudomanifold` with `Ok(())` before the fix and
now refuse as `InstanceInterference`.

**What stays open.** A pair whose only meetings are declared is probed
whether or not it reaches, and `blocks` reads its records with
`records_on_their_word`: a declared v-on-f or v-v touch whose cones
decidedly cross refuses as `MixedTouch`. Two kinds of meeting are still
taken on the records' word:

- a declared touch the analysis cannot read, such as a curved corner;
- the vertex events, collinear edge overlaps and edge-on-face bounds a
  confirmed face pair answers for (`census.rs` `Declared::confine` and
  `Declared::holds`, PR 5605): confined to where the touch lies in the
  closure of the pair's overlap (`in_overlap_closure`), but still on the
  pair's word. The touch analysis (`TouchSite::verdict`) does not read
  them.

An overlap those meetings hide with no vertex strictly inside therefore
clears. No such pose has been built. The half-overlap shape cannot be
declared free of findings with v-on-f and v-v records alone, because a
vertex on an edge has no record type.

Running `blocks` over declared-only pairs is not the repair. It
refuses ratified acceptance rows:

- `sweep` `m5_pr9_boss_union::a_touching_curved_assembly_validates_declared_and_refuses_undeclared`,
  the M9-2 declared curved boss, refuses as `TouchUnreadable`;
- nineteen declared planar seats refuse as `DeclaredFacePair`, among
  them `m9_c1_rest_face_rung::the_flush_seat_certifies_in_both_argument_orders`
  and `mate9_crossing_rung::the_declared_crossing_seat_certifies_both_ways`.

The repair is to read each face-pair-backed event through
`TouchSite::verdict` where the sweeps back it, and to decide what a
curved declared rest owes until the analysis reads curved cones.
Cost H.

## Note from CONTACT-7's designers (2026-09-28)

On the declared-only path, `blocks` with `records_on_their_word`
refuses a record only on `MixedTouch`. So a lenient NON-Crossing
reading (`Unanalysed` or in band), not just a lenient Rest, clears
there. Whether the Crossing verdict is complete, never missing a real
crossing, is therefore load-bearing on this path, and this row's repair
has to answer it.

Under CONTACT-7's piece reading, a Rest speaks only for the ball
around the touch point that its pieces cover. Coverage beyond that ball
rests on the vertex probe and on completeness (every meeting stands as
a finding). That coverage is exactly what this path takes on the
records' word.

## A built wrong clear (2026-09-29, a designer lane on `contact/land-7`, now main)

"No such pose has been built" is no longer true.

The pose:
- **The part:** an I-profile, `[(0,-1),(2,-1),(2,0),(1.5,0),(1.5,1),(2,1),(2,2),(0,2),(0,1),(0.5,1),(0.5,0),(0,0)]`, `prism_z(.., 0, 2)`.
- **The slab:** un-holed, `prism_z([(-5,0),(5,0),(5,1),(-5,1)], -1, 3)`.
- The waist `x∈[0.5,1.5], y∈[0,1]` passes through the slab. The head is seated on the slab's `y=1` face and the tail on `y=0`.
- No vertex is strictly inside either solid.

Results:
- **Undeclared:** it refuses, with 32 `UndeclaredContact` and a decided crossing at an edge-in-face site.
- **Declared as four `PatchContact`s** (the seat faces and the tail tops; each verifies): `validate_pseudomanifold` returns `Ok(())`.
- **Declared as sixteen `VfContact`s:** `Ok(())`. The vertex stars are saddles, so they read `Unanalysed`, and the declared-only path refuses only on `MixedTouch`.

This confirms CONTACT-7's designers' note: the Crossing verdict is incomplete, so "refuse only a decided crossing" is unsound even for vertex records. The repair lane commits the three poses as rows.

A second designer lane built a curved wrong clear on the same branch.
- **The pose:** a wall, `brick((0,4),(0,1),(0,4))`, and a horizontal
  log, `extruded(sketch_at(1.0), [bulge_loop([(1.6,1),(2.4,1),(2,1.8)])
  with bulge(·,·,(2,1.3)) on each arc], 2.0)`. The log's lower arc lies
  0.2 m inside the wall.
- **Declared** with four v-on-f records at (1.6|2.4, 1, 1|3), it returns
  `Ok(())`. A true rest (the same log with a flat bottom) also returns
  `Ok(())`, and the records cannot tell the two apart.
- **Arm 1 skips the log's curved wall** against the wall face because a
  v-on-f record names that face, so one point's record clears a whole
  curved face.

The same lane measured the nineteen planar seats: every event a face
pair backs, read strictly through `TouchSite::verdict`, reads Rest
(topo, sweep and editor-core unchanged).

The design fork is on its `[ev]` PR.

## Ev's ruling (PR 3422, 2026-09-29)

"For 1, nice find! Sounds good. For 2, the recommendation makes
sense."

- **Decision 1:** a declaration licenses a coincidence, never a side.
  Every meeting is read by the one touch analysis, and only a Rest
  clears.
- **Decision 2:** C1. A curved face in a touch's star is read through
  its certified reach box, which can certify a Rest or refuse but never
  decides a crossing. This keeps the M9-2 boss.
- **Later:** reading curved cones would be tighter than the box, so it
  is filed as a row:
  `the-touch-analysis-reads-curved-cones-tighter-than-the-reach-box`.
