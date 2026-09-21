---
id: census-lane-unsupported-display-names-the-scalar-not-the-door
kind: issue
title: CensusLaneUnsupported's Display says 'this scalar has no certified chart-overlap lane' where the fact is now about the door
status: open
priority: P4
cost: E
opened: 2026-09-21
---

## Finding

`ValidationError::CensusLaneUnsupported`'s `Display`
(`crates/topo/src/validate.rs`, the `Self::CensusLaneUnsupported`
arm) says *"this scalar has no certified chart-overlap lane … Replay
the body at f64, the telemetry probe or the interval scalar to get
the candidate examined"*, and the variant's doc says the same
("the SCALAR has no certified chart-overlap lane"). Since LANE-2 the
census takes the chart-region door as `Option<RegionLane<T>>` and the
refusal is raised whenever the PASS holds `None` — which is
`validate_pseudomanifold_structural` / `_certificate_structural` at
EVERY scalar, `f64` included (pinned:
`census::tests::a_pass_holding_no_region_door_refuses_the_declared_pair_typed_and_backs_no_crossing`
and `mate9_crossing_rung::the_structural_door_at_a_dual_refuses_the_declared_seat_typed_and_backs_no_crossing`,
which shows the same sentence at `f64` and at `Dual64`). So an `f64`
caller of the `_structural` door is told to replay at `f64`. The
sentence and the payload were deliberately left byte-identical by
LANE-2 (its spec: "the same payloads and `Display`"); the recourse
should now name the door — the certified `validate_pseudomanifold`
family at a certifying scalar — rather than the scalar, and the
variant's doc with it. The `pncad-py` projection
(`crates/pncad-py/src/tags.rs`, `validation.rs`) carries only the
tag and the subject, so it is unaffected.
