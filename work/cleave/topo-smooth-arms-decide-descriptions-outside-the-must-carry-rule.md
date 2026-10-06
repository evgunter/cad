---
id: topo-smooth-arms-decide-descriptions-outside-the-must-carry-rule
kind: issue
title: topo's two smooth-description arms (boolean D6 ladder, split finish) decide outside geom_brep::must_carry_over_edge
status: open
opened: 2026-10-01
priority: P1
cost: M
---


## Finding

Found by BAND's cap-rim smooth-arm sweep (every `DihedralClass::Smooth`
arm that stores a description). `geom_brep::must_carry_over_edge` is the
one home of what description a definitely-smooth join carries, and its
doc (`crates/geom-brep/src/dihedral.rs`) scopes that to "every
smooth-join arm in the sweep verbs". After the cap-rim arm moved onto
it, every sweep arm routes there. Two arms in `topo` still decide on
their own:

1. **`crates/topo/src/boolean/ops.rs`, `describe_minted_edges`' smooth
   arm (the "D6 smooth ladder")** hand-rolls the station walk over
   `tangent_jet` / `curvature_lever_arm` and folds an IN-BAND station
   into the conventional description (`_ => { det = false; break; }`,
   the comment: "a zero-side or in-band sample keeps the CONVENTIONAL
   posture"). `must_carry_over_edge`'s contract says the opposite —
   "An in-band verdict is never silently either side, so no caller may
   fold it into conventional" — and the walk does no per-station
   first-order gate. `dihedral.rs` already names this site as one of the
   two hand-rolled siblings left (issue 1439,
   `work/pred/lever-arm-fold-six-hand-rolled-siblings.md`), but as a fold
   duplicate; the in-band policy difference is the part not recorded.
   The comment cites "tier 3's ratified `SmoothUnderdetermined`
   stance" for the in-band fold — whoever takes this should check
   whether that stance covers in-band or only zero-side.
2. **`crates/topo/src/splitting/finish.rs`, the section-boundary smooth
   arm** reads no second order at all: a definitely-smooth section edge
   keeps a pre-existing `TangentIntersection` or chart image when it is
   coherent, and otherwise restates as `chart(s_self)`. The comment
   argues the case ("a curved wall smooth against the section plane is
   either a π seam or a wedge end") rather than asking the rule; a
   jet-determinate π seam whose existing description is a coherent
   chart image keeps it, where prefer-intrinsic would demand
   `TangentIntersection`. Not measured whether any operand reaches that.

Related, not duplicates: `work/boxes/description-staleness-ladder-three-spellings.md`
(the keep-vs-restate ladder these same two arms spell), and
`work/pred/lever-arm-fold-six-hand-rolled-siblings.md` (the fold).

## Since (FUSE, PR 3889): item 1 done

`describe_minted_edges`' smooth arm now asks the rule through
`topo::boolean::ops::seam_must_carry`: an in-band station refuses typed
(`SeamWedge`/`LeverArm(Seam)` by rung, `BooleanDecision::SeamJet` for
the sagitta), every station is gated first-order, and a seam that is a
corner at a station keeps the conventional description. (The comment's
`SmoothUnderdetermined` citation covered zero-side only: tier 3 refuses
an in-band sample `SliverDihedral`.) Item 2, `splitting/finish.rs`,
stays open.


## Built (branch cleave/smooth-arms)

Item 1 was already done on main (FUSE, PR 3889). Re-checked: the
boolean's `describe_edges` smooth arm decides through
`must_carry_reading` → `geom_brep::must_carry_over_edge`, and an
in-band station refuses typed. The tier-3 stance its old comment cited
is C7 (`crates/geom-brep/README.md`, *Tangency*):
"`SmoothUnderdetermined` when the surfaces under-determine the locus"
and "an in-band second-order tie escalates". That is zero side only,
which agrees with the rule. `git log -S` traces both phrases to the
2026-09-03 move of the design text beside the code (585b3422ff). No
conflict between the contracts.

Item 2: `describe_section_boundary`'s smooth arm
(`crates/topo/src/splitting/finish.rs`) now asks
`must_carry_over_edge(..).description(s_self, s_other, witness)` over
the carrier it restates:

- jet-determinate ⇒ `TangentIntersection { s_self, s_other }`;
- under-determined ⇒ the section-chart image;
- in band ⇒ `DescribeEscalated` for a first-order station, and
  `DescribeBendEscalated` for a second-order one (its own message);
- a transverse station ⇒ `SmoothJoinRefuted`, the split's own typed
  refusal, as `MustCarryRefusal` requires. Only the boolean's seams
  keep such an edge conventional, and the rule's doc now names that
  caller.

A description is kept verbatim only when it is of the demanded kind
and names the current pair. The material-pairing knife-edge refusal
runs first, as before.

Measured with a probe in both arms, over every test in topo, sweep and
editor-core (7370 tests), the demos/tour suite (96 tests) and the
demos/tour binary's full walk:

- Boolean arm: 55 487 (crates) and 13 056 + 2 173 (tour suite, tour
  walk) visits. Every visit is `JetDeterminate` or `UnderDetermined`;
  none is in band or transverse.
- Split arm: 207 visits in the crates and none in the tour. 206 are
  plane–plane `UnderDetermined`, with no change: a coherent chart is
  kept and an `Intersection` is restated. One is `JetDeterminate`:
  `wedge_end_doors::a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam`,
  whose seam previously restated to `chart(s_self)` and now stores
  `TangentIntersection` over its current pair. The test asserts it.
- No visit is in band. The in-band case is not reachable through the
  split's door on the shoulder family: the join's certification of the
  tangent chord refuses the same sagitta first. Filed as
  `work/tang/split-tangent-chord-mints-tangency-without-the-must-carry-rule.md`.
- Unit rows (`splitting::finish::smooth_arm_rows`) pin two cases
  directly on the arm, using a cut cube whose neighbour surface is
  swapped:
  - a corner at a station refuses `SmoothJoinRefuted`;
  - a coherent `TangentIntersection` on an under-determined edge is
    restated in the section chart.
  An in-band station is not pinned: no case reaching it was found.
