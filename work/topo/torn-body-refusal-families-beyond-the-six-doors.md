---
id: torn-body-refusal-families-beyond-the-six-doors
kind: issue
title: Torn-body refusal families outside the euler, read-back, pcurve, shell, replace_face and boolean/graft doors still answer typed
status: dispatched
opened: 2026-10-04
priority: P3
cost: M
refs: [stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body]
---


(TOPO implementer, the remainder of
`stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body`'s
second unit, which converted `readback::DanglingRef`, `TornLoop`,
`PcurveMintError::Corrupt`, `ShellError::Corrupt`,
`ReplaceFaceError::Corrupt`, `SectorFaceError::Corrupt`, the boolean's
`corrupt_at` / `CorruptOperand` and the graft's torn source. That unit
stopped at about 7,800 changed lines; this row is what its sweep found
still standing.)

## What

The rule is D2 row 4 (bullet *A torn body is a kernel bug*, Ev on
PR 4006, quoted on `work/pipe/S14.md`): a record miss panics naming the
record; a key the caller passed stays typed. These variants still
answer a torn record typed. Counts are `::Variant` occurrences in
`crates/` (raise sites, matches and tests together), from
`rg -o '\b[A-Z]\w*::(Corrupt|Torn)\w*\b' crates`.

| Variant | Home | Count | Door argument? | Shape of the fix |
|---|---|---|---|---|
| `PointInSolidError::CorruptFace` | `boolean/solid_contain.rs` | 67 | yes: `point_in_solid_of`'s solid, `SolidFaces::of` | split: stale solid typed, record misses panic; it is also used for non-corruption cases mapped to `Trim(None)`, which need their own variant first. `contfp`'s curved doors carry it whole as `ContainError::Curved` |
| `ChartRegionError::Corrupt` | `chart_region.rs` | 42 | yes: `chart_region_overlap`'s faces | split |
| `StepExportError::Corrupt` | `step-export/src/{lib,writer,volume}.rs` | 26 | no (whole body) | panic; the "shell carries no faces" arm is a schema fact and may stay |
| `SplitFinishError::Corrupt`, `::TornComponent` | `splitting/finish.rs` | 22 | no (driver) | panic |
| `PointInLoopError::CorruptLoop` | `splitting/containment.rs` | 16 | yes: `point_in_loop`'s loop | split; a whole-turn scaffold circle is a legal state and stays typed. `contfp` carries it as `ContainError::LoopUnreadable`. `cycle_steps`' and `loop_hull`'s eight record hops past a resolved loop are links now, and panic (PR 4048) |
| `MassPropsError::Corrupt`, `LoopEdgesError::Corrupt` | `props.rs` (and `mesh/src/curved.rs`) | 17 | no (whole body) | an empty loop is legal tier-1 scaffolding and stays typed under its own name; key misses panic |
| `RevertError::Corrupt` | `revert.rs` | 12 | no (whole body) | panic; reachable typed today through `VoidInsertError::Revert` on a torn cavity |
| `TouchVerdict::Corrupt`, `Undecided::CorruptInstance` | `census.rs` | 13 | no (the census runs on bodies tier 1 admits) | panic |
| `SplitError::TornGroup` | `splitting/mod.rs` | 11 | no | panic |
| `SplitJoinError::Corrupt` | `chord_join.rs` | 9 | no (driver) | panic |
| `SectionError::Corrupt`, `NestFault::Torn` | `splitting/{section,section_loops}.rs` | 11 | no (scratch body) | panic; `SenseFault` is left with one variant |
| `Unexaminable::Corrupt` | `coherence.rs` | 8 | check the door's loop argument | split or panic |
| `TransformError::Corrupt` | `transform.rs` | 8 | no (whole body) | panic |
| `BooleanError::TornComponent` | `boolean/` | 7 | no (driver invariant) | panic |

Beside the named variants, the same shape under other names:

- about 20 `BooleanError::ClassificationInvariant` raises over torn
  lookups in `boolean/{reduce,recl,join,vtxfac,shell_witness,solid_contain}.rs`,
  and `JoinDesync` torn reads in `boolean/finish.rs` (a completed null
  face, its shell, the operand solid) and `ops.rs` `apply_recuts`.
  These run on the reduction's body mid-operation, and `vtxfac` kills
  an edge (`kemr`), so each needs its own proven premise before it
  panics; one without stays typed. `sectors.rs`' and `rest.rs`' torn
  reads are converted: `rest.rs` keeps typed every key it carries
  across its own kills, and panics only on a hop past a record it just
  resolved;
- `describe_edges`' `EdgeDescribeFailure::NotWalkable`, shared by the
  boolean and `merge_faces`;
- `boolean/ops.rs` `remap_contacts` / `remap_carried`: a fusion list
  whose row keeps a key an earlier row killed refuses typed
  (`ops::tests::a_corrupt_fusion_list_refuses_where_a_dead_end_drops`);
  the list is the boolean's own bookkeeping, so that is a kernel bug;
- `RevertError::Corrupt`'s rows (`revert::tests::revert_refuses_each_corrupt_link_typed`,
  `review_d18::revert_writes_no_fault_off_a_torn_next_prev_or_start`)
  and `offset_together::tests::an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk`
  assert today that a torn body answers typed or not at all;
- outside topo, each a kernel driver or consumer reading records of a
  body a door built: `mesh`'s `TessellateError::MissingEntity` (about
  30 raises in `chords.rs`, `curved.rs`, `memo.rs`, `planar.rs`,
  `trimmed.rs`, `tessellate.rs`), `sweep`'s `BlendError::BodyNotIntact`
  (`blend/{surgery,admit,build,open}`), and `step-import`'s
  `StepImportError::Topology { what: "internal: …" }` in `adopt.rs`.

## Direction

One unit per family, as the read-back, pcurve, shell and boolean units
did it: decide argument vs record at each raise site from the code; an
argument miss keeps a typed variant stating the fact, checked before
the first write; a record miss goes through `live::linked` /
`live::proven` / `Body::*_linked`; the variant goes when nothing typed
is left in it. Each unit extends
`review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` with its
doors (judged on a clone, unchanged on a premise panic) and proves the
extension can go red with one mutation.

## 2026-10-05 — what `ContainError::Curved` carries into the boolean (measured)

Read off `contain.rs` `curved_face_placement` and its chart arms: the
only `PointInSolidError` payloads that reach `solid_err` uncaught are
`CorruptFace` (the cylinder arm's `full_turn_outline` and
`wall_outline`, both arena misses; the sphere arm's
`sphere_chart_trim`, arena misses and `face_azimuth_window`'s
`Corrupt`/`UnpairedLooseEnds`) and `Loop(CorruptLoop)`
(`wall_outline`'s `loop_reach`: a control-point-less spline edge or a
poisoned extent). `PartialConeFace`, `PartialTorusFace` and
`WallOutlineUnsupported` never get there: the cone and torus arms map
them to `Trim(None)`, and the cylinder arm never hands
`point_on_wall_in_face` an `Unsupported` outline. The arms disagree
about `CorruptFace`: cone, torus and `cylinder_chart_trim` read it as
`Trim(None)`, the honest remainder, while `full_turn_outline`,
`wall_outline` and `sphere_chart_trim` pass it on. The `CorruptFace`
split above is therefore also the decision about which of those
readings is right. The boolean now carries whatever arrives as
`BooleanError::Containment(e)`
(`contain-refusals-on-a-sound-face-reach-the-boolean-as-a-classification-invariant`).
