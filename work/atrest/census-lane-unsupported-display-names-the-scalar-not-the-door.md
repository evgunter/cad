---
id: census-lane-unsupported-display-names-the-scalar-not-the-door
kind: issue
title: The _structural doors' verdicts moved to the door at every scalar, and CensusLaneUnsupported's Display still names the scalar and the wrong arm
status: closed
priority: P3
cost: E
opened: 2026-09-21
parent: ATREST-8
closed: 2026-09-25
pr: 3185
---

## Finding

**The verdict, first.** Since LANE-2 (PR 3038) the census takes the
chart-region door as `Option<RegionLane<T>>` and
`validate_pseudomanifold_structural` /
`validate_pseudomanifold_certificate_structural`
(`crates/topo/src/validate.rs`) hand it `None` at EVERY scalar. Before
it, `f64`'s `ChartRegionLane` impl reached the census through
`AtRestPolicy`'s supertrait, so the `_structural` doors at `f64`
examined declared patch records and conformal candidates. Measured at
the merge base and the head on the declared straddle seat
(`topo::test_support::straddle_seat`, one patch record):
`validate_pseudomanifold_structural` at `f64` answered `Ok(())` at the
base and answers
`Err([UndeclaredContact{EdgeEdgeCross} ×2, CensusLaneUnsupported{FacePair}])`
at the head — the two crossings the declaration backs are reported as
undeclared contacts, on a body whose contact IS declared, with the
declaration in hand. `validate_pseudomanifold` at `f64` is `Ok` at
both. This is the ruling's letter (`work/scalar/H5.md` §RATIFIED
ruling 3: the `_structural` twin is the door that holds no certified
lane, at any scalar, and a runtime branch on the scalar is what the
cut forbids), it matches what `nurbs_lane` and `quad_lane` already do
at that twin, and it is pinned deliberately
(`mate9_crossing_rung::the_structural_door_at_a_dual_refuses_the_declared_seat_typed_and_backs_no_crossing`,
`lane2_r2_probes::the_declared_seat_through_the_four_doors_at_f64`,
`sweep::cert_m2r1_passes::m2r1_declared_seat_f64`'s dump line). What
it leaves is a public `topo` door whose refusal names the wrong
recourse, below.

**Reach.** No production caller and no user route: the at-rest gate at
a certifying scalar runs `validate_pseudomanifold`
(`crates/topo/src/props.rs`, `AtRestPolicy`'s arms;
`crates/editor-core/src/assembly.rs`, the gather), `pncad::prelude`
exports only `validate_pseudomanifold` and `pncad-py` binds only that
(`crates/pncad-py/src/py/value.rs`). The observers are `topo`'s public
API and the test corpora that call the `_structural` doors at `f64`:
`sweep/tests/cert_m2r1_passes.rs`, `editor-core/tests/cert_m2r1_corpus.rs`,
`topo/tests/cert_m3r1_probes.rs`, `sweep/tests/r1_lane0_e2e.rs`.

**The `Display`, second.** `ValidationError::CensusLaneUnsupported`'s
`Display` (`crates/topo/src/validate.rs`, the `Self::CensusLaneUnsupported`
arm) says *"this scalar has no certified chart-overlap lane, so the
conformal face-pair arm could not examine the candidate … Replay the
body at f64, the telemetry probe or the interval scalar"*, and the
variant's doc says the same ("the SCALAR has no certified chart-overlap
lane"). Two things are wrong with it now:

- it names the scalar where the fact is the door's: an `f64` caller of
  the `_structural` door is told to replay at `f64`. The recourse is
  the certified `validate_pseudomanifold` family at a certifying
  scalar, and the variant's doc should say so with it;
- it names "the conformal face-pair arm" where the declared-record
  confirm arm (`confirm_curve_and_patch_records`' Door 2) raises the
  same variant — which is exactly the arm the straddle-seat rows
  above exercise, so the tree now pins a sentence naming the wrong arm
  (`census::tests::lane_unsupported_sentence` restates it verbatim
  and moves with this row).

The same class, outside `topo`: `crates/editor-core/src/assembly.rs`'s
classification comment on `CensusLaneUnsupported` ("a fact about the
RUN's scalar … replay the document at a certifying scalar") — true
for the product gather, which reaches only the certified door, and
the same scalar-keyed reading; re-word with the `Display`.

The sentence and the payload were left byte-identical by LANE-2 and
its fix pass on purpose (its spec: "the same payloads and `Display`";
D9's dumps compare the bytes).

## Priority

P3, re-argued from P4: a public library door whose verdict at `f64`
is now the dual's and whose refusal text sends the caller to the
scalar it is already at is a library-usability defect
(`work/README.md`'s P3 band), not a code improvement. It is capped
there by the reach — no production or Python route, refusing rather
than blessing — and by the verdict itself being ruled correct; what
is wrong is the sentence and the doc, not the answer.
