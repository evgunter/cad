---
id: torn-body-refusal-families-beyond-the-six-doors
kind: issue
title: Torn-body refusal families outside the euler, read-back, pcurve, shell, replace_face and boolean/graft doors still answer typed
status: open
opened: 2026-10-04
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
| `PointInSolidError::CorruptFace` | `boolean/solid_contain.rs` | 67 | yes: `point_in_solid_of`'s solid, `SolidFaces::of` | split: stale solid typed, record misses panic; it is also used for non-corruption cases mapped to `Trim(None)`, which need their own variant first |
| `ChartRegionError::Corrupt` | `chart_region.rs` | 42 | yes: `chart_region_overlap`'s faces | split |
| `StepExportError::Corrupt` | `step-export/src/{lib,writer,volume}.rs` | 26 | no (whole body) | panic; the "shell carries no faces" arm is a schema fact and may stay |
| `ContainError::Corrupt` | `boolean/contain.rs` | 24 | yes: `contfp`'s face | three things in one variant: the caller's face, torn reads, and reachable refusals `contain::solid_err` folds in (`WallOutlineUnsupported`, `RayExhausted`); split those out first |
| `SplitFinishError::Corrupt`, `::TornComponent` | `splitting/finish.rs` | 22 | no (driver) | panic |
| `PointInLoopError::CorruptLoop` | `splitting/containment.rs` | 16 | yes: `point_in_loop`'s loop | split; a whole-turn scaffold circle is a legal state and stays typed |
| `MassPropsError::Corrupt`, `LoopEdgesError::Corrupt` | `props.rs` (and `mesh/src/curved.rs`) | 17 | no (whole body) | an empty loop is legal tier-1 scaffolding and stays typed under its own name; key misses panic |
| `SplitReduceError::CorruptOperand` | `splitting/{mod,classify}.rs` | 12 | no (operand records) | panic |
| `RevertError::Corrupt` | `revert.rs` | 12 | no (whole body) | panic; reachable typed today through `VoidInsertError::Revert` on a torn cavity |
| `TouchVerdict::Corrupt`, `Undecided::CorruptInstance` | `census.rs` | 13 | no (the census runs on bodies tier 1 admits) | panic |
| `SplitError::TornGroup` | `splitting/mod.rs` | 11 | no | panic |
| `SplitJoinError::Corrupt` | `chord_join.rs` | 9 | no (driver); `chord_join::vertex_point` still answers a missing vertex `corrupt_vertex` | panic |
| `SectionError::Corrupt`, `NestFault::Torn` | `splitting/{section,section_loops}.rs` | 11 | no (scratch body) | panic; `SenseFault` is left with one variant |
| `Unexaminable::Corrupt` | `coherence.rs` | 8 | check the door's loop argument | split or panic |
| `TransformError::Corrupt` | `transform.rs` | 8 | no (whole body) | panic |
| `BooleanError::TornComponent` | `boolean/` | 7 | no (driver invariant) | panic |

Beside the named variants, the same shape under other names:

- about 30 `BooleanError::ClassificationInvariant` raises over torn
  lookups in `boolean/{reduce,recl,rest,join,sectors,vtxfac,shell_witness,solid_contain}.rs`,
  and `JoinDesync` torn reads in `boolean/finish.rs` (a completed null
  face, its shell, the operand solid) and `ops.rs` `apply_recuts`;
- `describe_edges`' `EdgeDescribeFailure::NotWalkable`, shared by the
  boolean and `merge_faces`;
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
