---
id: a-recorded-direction-claims-a-unit-and-is-held-as-two-numbers
kind: unit
title: A profile's recorded direction claims to be a unit and is held as two numbers: the claim is a comment in three places, a lift carries it through from_f64, and a door turns a frame onto it
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided]
---


Raised by the signed-radius lane's finding
(`spun-onto-takes-a-bare-vector-for-a-unit`, which lands with that
unit's PR) and weighed by two designers on 2026-11-22, who agreed at
their first reports and found the same two corrections independently
(`analysis/design-fork/spun-onto-takes-a-bare-vector-for-a-unit`).
Not put to Ev: both read it as what DESIGN `[scalars]` already
ratifies (fork 274: a claim is "held as the claim it is (a frame as a
`RigidMotion` or `UnitVec3`, never as its entries)"), applied to the
one claim of that kind still held as entries.

**What is wrong.** A validated segment records unit directions (a
line's director, an arc's radial at each end) as bare `Vec2`s. That
each is a unit is said in three comments (`Dir::from_unit`, "the
invariant is the callers'"; `SegmentKind`'s doc, accepted "by check
1's measure at its lever"; `RigidMotion::spun_onto`'s doc) and held by
no type.

- **The reachable case is the lift, not the table.** `SegmentKind::map`,
  `ProfileLoop::map_scalar`, `ConstructedLoop::lift_computed` and
  `ValidatedProfile::lift_onto` pass a direction's two numbers through
  `from_f64`. An `f64` radial is a unit only to rounding; read at
  `Interval` as a point it contains no unit vector, and a frame turned
  onto it contains no rotation. Nothing shipped is wrong today (the
  loft's lifted sections feed its stacking check and are not seen to
  reach `Placing::frame_onto`: read, not run), and nothing stops the
  first caller that hands a lifted section to that door. The
  `from-f64` gate already marks `lift_onto` "launders".
- **The table door is test-only.** `RawLoop::new_directed` is compiled
  under `test` / `test-support` and called from two test files; two
  rows pin a stretched written direction as accepted
  (`recorded_directions.rs` `square_with_directors`,
  `a_tables_direction_is_a_unit_by_check_ones_measure_at_its_lever`).
- **ONEHOME's PR 7323** (check 1's frame half goes) rests on "no
  production door mints a frame from raw entries". `spun_onto(Vec2)`
  is such a maker until it is typed; in shipped builds no written
  radial exists and no lifted one is seen to reach it, so 7323 does
  not wait for this row.

**The final state.**

1. **`UnitVec2<T>` in `geom-core` beside `UnitVec3`**, private
   contents, a closed list of makers each true at every scalar: the
   exact axes; `of_angle` (`sin_cos`); `normalized(v)` (`v/‖v‖`, which
   does not decide: the length was another gate's); negation, the
   quarter turn, a unit turned by or reflected across a unit; one
   crate-private maker per closed-form identity, beside the formula
   that proves it (`Arc2::from_chord_radials`, whose `sign: T` becomes
   a sign type); and a deciding `new(v, site, band)` for a vector a
   user typed, which divides after deciding. No maker takes two
   numbers on trust and none accepts a vector within a band.
2. **The profile's record holds it** (`SegmentDirections`,
   `SegmentKind`); the path layer's `Dir` and the sweep's `Unit2` are
   the same thing and become it. The sweep's `Unit3` becomes
   `UnitVec3` when ONEHOME's carrier-directions row lands.
3. **Crossing scalars is the witness's own:** within a kind, carried;
   `f64` to `Interval`, the enclosure of `v/|v|`; down to `f64`, the
   point value; into `Dual` or `Sym` there is no read, and a lift
   there mints the direction again from the lifted points.
   `SegmentKind::map(closure)` and `SegmentDirections::map(closure)`
   do not survive as written.
4. **A table cannot write a stretched direction.** Validation's
   reading of a table's direction keeps only its subject (does the
   unit point at the vertex); the "stretch bounded by ε/ℓ" half and
   the sentence equating it with check 1 go. The test-only door stays
   and takes `UnitVec2`.
5. **The door says what it needs:** `spun_onto(UnitVec2)` with no
   arithmetic (the symbolic identity the signed-radius unit built it
   for is kept); `spun_toward` is `normalized` then `spun_onto`.
   Nobody reads the frame afterwards.
6. **Text:** no DESIGN change. The profile README's "A table's are
   minted the same way, once at validate" is re-worded to what tables
   do (carry the directions their construction made, checked against
   the vertices; one that carries none has them minted); `UnitVec3`'s
   note that the 2-D analogue "is left where it is" and
   `SegmentKind`'s "stored as written, never normalised" paragraph go.

**Chosen by the orchestrator where the designers left a choice (both
leaned the same way):** identity makers are crate-private to
`geom-core`, and a verb's formula that is none of the listed makers
moves into `geom-core`'s arc algebra or records `normalized`; the
test-only table door survives.

**Rows that flip:** the two stretched-direction rows above (the state
becomes unwritable: a `compile_fail` doc test, as `UnitVec3` has, and
a misaimed-fixture row replace them);
`the_lift_carries_a_recorded_direction_and_never_rederives_it`
(`f64` to `Interval` becomes an enclosure); the `from-f64-sites.tsv`
row for `profile/src/validate.rs` shrinks.

**Corrected by the signed-radius lane's trace (2026-11-22):** a lifted
radial DID reach the door on that lane's branch.
`ValidatedProfile::lift_onto`'s one production caller is the loft's
enclosure arm, and its lifted sections go to the stacking door and to
`assemble`, both of which build each rim through
`placed_segment_spec`, whose arc arm is `frame_onto`. (The designers
read the stacking check and missed `assemble`.) On main that arm
normalizes; the lane's un-normalized door opened it, and the lane
closes it in its own unit by normalizing an arc's radials in
`ValidatedSegment::lift` at the target scalar. So when this row is
built the lift's crossing is already a normalization for arcs, and
what remains is to make it the witness's own read for every direction.

**Not established:** whether every verb's identity radial (tangent,
via, onto, the fillet paths) is one of the listed makers; what
`lift_computed` (`f64` to `Sym`) callers do at `Sym`; whether a lifted
section reaches the sweep's door on any shipped path (a run, not a
read).

**Parked on three units that rewrite the same ground:** doors-read
(the directions move into the kind's arm), the mint
(`Real::Expression`; the line mint's two branches are two makers), and
signed-radius (which adds the door). Dual tier when it is dispatched:
a witness type every profile and sweep reader holds.

**From PR 7384's second review (2026-11-23), three more places this
unit takes:** `swept::Traversed::half_turn(…, radial: Vec2<T>)` takes a
bare vector for a unit claim (new in that PR); every field of
`ValidatedSegment` and `SegmentKind` is public, so another crate can
write a free-standing segment whose director has length 5 (it cannot
enter a `ValidatedLoop`); and `SegmentKind::map` accepts any closure,
so nothing in its signature keeps a mapped director a unit or the map
upward.

## Evidence from the signed-radius lane (folded in from its row, `spun-onto-takes-a-bare-vector-for-a-unit`)

Found by `boxfinish/signed-radius`, which added the door, on the
orchestrator's question about ONEHOME's frame claim (fork 274 decision 1,
`work/onehome/carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes.md`,
and PR 7323, which removes check 1's frame half).

**The door.** `geom_core::RigidMotion::spun_onto(c: Vec2<T>)` turns a
frame onto `c` with no arithmetic on `c`; `sweep`'s
`Placing::frame_onto(Unit2<T>)` is its one caller, and
`swept::sketch_direction` hands it an arc's recorded start radial
(`profile::SegmentKind::Arc { radials }`, wrapped as `Unit2(start)`). The
extrude's and the loft's arc carriers and cylinder walls are turned so.
`RigidMotion::spun_toward`, which every line still goes through,
normalizes first.

It is un-normalized on purpose: over a box the frame's `u` is then the
recorded radial placed, node for node, so `centre + radius·u` is the
placed vertex as a theorem; through `spun_toward` the radial's own
length is a root the tier holds as an atom (measured on the merge of
main: the arcs' start endpoints stop folding).

**What reaches it, and what makes each a unit.**

| radial | made by | a unit by |
|---|---|---|
| the circle verb's `(±1, 0)` | `path::circle_loop` | literals |
| the split circle's `(cos θ, sin θ)` | `circle_split`, at the lane scalar | `sin_cos` at the scalar: encloses a unit |
| a bulge leg's, a via, tangent or onto arc's | the lowering (`Arc2::from_chord_radials`) and the verbs | a formula whose squared length is one as an identity |
| a half turn's `[r, −r]` in the sweep | `swept` | its source radial |
| one validation minted | `seg::mint_radial`, the offset over its measured norm | a normalization |
| **one a table wrote** | `RawLoop::new_directed`, a test-only door (`raw_door!`, compiled under `test` / `test-support`) | **a decision**: `segment_direction` accepts it where `‖c + r·d − p‖` is within the band, which bounds `(|d| − 1)·r`, and stores it as written |

Every row but the last encloses a unit at every point of its box, so
`spun_onto` of it encloses a rotation, and at `f64` is one to rounding.
The last is a unit only at the radius's lever: a written radial may be
stretched by `ε/r`. `validate.rs`'s `SegmentKind` doc says so and leans
on check 1 ("a radial accepted here is one check 1 accepts as a `u_ref`
at that radius"). PR 7323 deletes that check on the claim that every
frame mint yields a rotation in exact arithmetic and none takes raw
entries; `spun_onto` of a written radial is a mint that does not.

**A radial lifted across scalars** is the shipped row.
`ValidatedProfile::lift_onto` (and `SegmentKind::map`,
`ProfileLoop::map_scalar`, `ConstructedLoop::lift_computed`) passes a
radial's two `f64` numbers through `from_f64`: a unit to rounding, and
at `Interval` a point that holds no unit. Traced: `lift_onto`'s one
production caller, the loft's enclosure arm (`loft.rs`, `enclosure`),
hands the lifted sections to `assemble` and to the stacking door, and
both build each rim through `placed_segment_spec`, whose arc arm is
`frame_onto(sketch_direction(seg))`. So a loft of point sections that
hold arcs, at an enclosure scalar, reached `spun_onto` with a lifted
radial. Closed in this unit: `ValidatedSegment::lift` normalizes an
arc's radials at the target scalar, so what the loft turns onto encloses
the unit the `f64` radial rounds. `map_scalar` and `lift_computed` feed
a profile that is validated again at its scalar, whose radials the
verbs or the mint make there.

The table door being test-only, the hole it leaves is a statement about
types: no shipped build holds a written radial. The row
`sweep`'s `a_tables_stretched_radial_is_stored_in_the_walls_frame_as_written`
pins it as a fixture.

So on main as it stands the written radial is still read by check 1 at
the finish. Once 7323 lands nothing reads it: at an enclosure scalar its
rounding is taken as zero, and at `f64` 7323's measured rounding charges
it only where it is levered.

**Why it is not closed here.** The argument above is a comment: neither
`spun_onto`'s `Vec2` nor the sweep's `Unit2(…)` tells a constructed unit
from a written one, and the validated profile does not either (a
`ProfileLoop` a verb built and a `RawLoop` a table wrote both arrive as
`radials: [Vec2; 2]`). Telling them apart is a type in the profile's
direction record (which constructions record a direction, and what a
table's written one is), which is the direction mint's, under `[ev]`
PR 6993.

**Repair shape.** The validated arc says which its radials are (made by
a construction or a normalization; or written and accepted at a lever),
`frame_onto` takes only the first as a type, and a written radial goes
through `spun_toward`, as every direction does on main today. Or the
table door normalizes what it accepts.

**The tests that say it is closed.**
- A `RawLoop::new_directed` arc whose start radial is stretched by half
  of `ε/r` validates, and the frame its extrude stores is a rotation to
  rounding at `f64` (its `u_ref` squared less one within a few ulps).
- `spun_onto`'s argument is a type no written direction inhabits.

**Measured since:** a written radial is read today by the pcurve certificate's envelope, its `Frame` term, not by check 1. `sweep`'s `a_tables_stretched_radial_is_stored_in_the_walls_frame_as_written`: stretched by `0.1·ε` the slab builds holding a `u_ref` that long; by `0.4·ε` the extrude refuses under `pcurve_envelope` reading `2·ε`.
**From the signed-radius review (2026-11-24): the sweep door's
registered ends rest on the same premise.** `sweep::swept`'s
`register_placed_carrier_ends` states that the placed carrier at 0 and
at its span is the placed sketch point `c + r·radial`. That is an
identity of the stored expressions only where the frame's `u` is the
recorded radial placed, which the frame mint keeps only for a radial
that is a unit. For a table's radial off unit the reviewers' probes
(`boxfinish/review-signed-radius-r1`,
`p6_a_tables_stretched_radial_at_the_enclosure_scalars`) read:

- at `Interval`, a written radial off by `1e-15` (inside the frame
  mint's charge) stores a `u_ref` whose enclosure holds no unit; past
  the charge the frame poisons and the door refuses typed at
  `EndpointStart`;
- at `Sym<f64>`, the frame mint squares the stretched radial, so the
  frame's `u` is no longer the recorded radial placed and the
  registered end is false of the stored carrier.

Every radial a verb or validation's mint records is a unit, so no
shipped door registers a false end; the table door is the one that can,
and removing it (below) removes the premise.

## One point reopened by the line-directions weighing (2026-11-24): the test-only table door goes

This row's weighing kept the test-only table door that writes
directions (the orchestrator's choice between two leanings). Both
designers of the line-directions weighing
(`a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners`)
now lean the other way, one from its first report and the other after
a round: a table carries no directions; the door goes, with the
production check that exists only to serve it (the `segment_direction`
gate, its resolution gate and its two refusals: `[simple.checks]`); and
a loop that gives up its provenance (`ConstructedProfile::into_profile`,
a public and not test-only route that today converts and KEEPS its
directions) gives them up and has them minted.

**Ruled (the orchestrator, logged): the door goes, in this row's unit.**
The one cost to measure first: a verb's exact oblique direction on a
loop converted to a table and then swept. Neither designer found a
production caller that does that; the unit greps the callers of
`From<ClosedLoop<T>> for ProfileLoop<T>`, `into_loop` and
`into_profile` and says what it found. The line-directions units do not
depend on this point and land either way.
