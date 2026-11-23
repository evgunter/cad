---
id: a-recorded-direction-claims-a-unit-and-is-held-as-two-numbers
kind: unit
title: A profile's recorded direction claims to be a unit and is held as two numbers: the claim is a comment in three places, a lift carries it through from_f64, and a door turns a frame onto it
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [the-axis-mint-asks-for-proof-only-where-the-value-reads-zero, a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided]
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
