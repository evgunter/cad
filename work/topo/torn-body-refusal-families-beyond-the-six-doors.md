---
id: torn-body-refusal-families-beyond-the-six-doors
kind: issue
title: Torn-body refusal families outside the euler, read-back, pcurve, shell, replace_face and boolean/graft doors still answer typed
status: open
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
| `RevertError::Corrupt` | `revert.rs` | 12 | no (whole body) | **done** (`topo/revert-torn-reads-panic`): every link `revert` follows panics naming its record, `revert` answers `Self`, and `RevertError`, `VoidInsertError::Revert`, `BooleanError::Revert` and the `revert` tag retire. A void cavity is a body some door left, so a torn one panics before `dst` is written |
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
- `offset_together::tests::an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk`
  asserts today that a torn body answers typed or not at all
  (`revert`'s two such rows now assert its premise panic);
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
`BooleanError::PointInFaceRefused { refusal: Curved(e), .. }`
(`contain-refusals-on-a-sound-face-reach-the-boolean-as-a-classification-invariant`).

That moved a user-visible claim for the `CorruptFace` arm. Before, the
reduction's `esc` folded it into `ClassificationInvariant`, which says
"kernel bug". Now it renders as "the Boolean cannot tell what is
inside the solid: one of its faces is broken (it cannot be walked, or
names something that is gone)". Mid-reduction the
face is a working copy of a gated operand, so an arena miss there is
the kernel's own surgery, not the user's broken face: the text went
from "our bug" to "your face is broken" for that arm. Whichever
reading the `CorruptFace` split settles on, it decides this text too.

## 2026-10-05 — typed torn hops the boolean's record-hop unit converted

`torn-hops-read-as-absent-across-the-boolean`'s unit turned these typed
raises over a hop past a resolved record into premise panics, and kept
each one over a key the caller carries:
- `finish.rs` `pinch_site`: `JoinDesync` "a pierce vertex no longer
  resolves" (both callers resolve it first), "a pierce vertex's face",
  "a face's loop no longer resolves", "a face's loop is not walkable".
  `discarded`'s walks keep "a section face no longer resolves" and
  panic past it.
- `zip.rs` `split_across`: `ZipCorrespondence` "a pinch half-edge" /
  "a pinch loop no longer resolves", "a pinch vertex's orbit does not
  close"; the vertex itself keeps "a pinch vertex no longer resolves".
- `carrier_cross.rs` `boundary_crossing`: `ClassificationInvariant`
  "boundary loop lost", "does not close", "half-edge lost"; "face lost"
  stays.
- `reduce.rs` `boundary_meets_circle_only_at`: every hop past the face;
  the face keeps its `ClassificationInvariant`.
- `ops.rs` `describe_edges`: `EdgeDescribeFailure::NotWalkable` now
  means only a worklist edge that does not resolve, or an edge whose
  curve is not certified on the smooth arm.
- `surface_group.rs` `unmated_boundary`: its `Err(face)` now means only
  a member key that does not resolve.

`PointInSolidError::CorruptFace`'s raises in `wall_outline`,
`torus_chart_windows` and `sphere_chart_trim` are untouched: only the
curve reads that answered as a kind were converted there.
The result is that one walk can answer in two ways. In
`sphere_chart_trim`, a torn loop walk, half-edge, vertex or point
answers `CorruptFace`, while a torn edge or curve panics. `wall_outline`
splits the same way, and so does `torus_chart_windows`, where a torn
edge is also typed. Each site carries a comment that hands its
`CorruptFace` raises to this row. Once this row's split ("record misses
panic") lands, each walk answers one way.

## 2026-10-06 — typed torn hops the split / chord-join / reach unit converted

`torn-hops-read-as-absent-in-the-split-the-chord-join-and-the-reach-rules`'
unit turned these typed raises over a hop past a resolved record into
premise panics, and kept each one over a key the caller carries:
- `chord_join.rs` `outer_cycle`: `SplitJoinError::Corrupt` for the
  outer loop and its walk; the face keeps `Corrupt`.
  `run_azimuth_images`' member half-edge and edge, and
  `cone_apex_closure`'s member half-edge (`Corrupt`), now `proven` /
  `linked`. So `face_azimuth_window`, `face_azimuth_images` and
  `cone_apex_closure` answer `Corrupt` only for a face that does not
  resolve, and every caller in `boolean/solid_contain.rs` resolved
  that face first.
- `chord_join.rs` `along_edge_spec`: the circle arm's `Corrupt` for the
  segment edge's `he_plus` and its end; the segment edge itself keeps
  a typed `SectionInvariant` ("the segment's edge no longer resolves").
  A chord end's point is now a link too (`Body::point_of`), where a torn
  point and a stale chord end shared one `SectionInvariant`. The chord
  end itself keeps a typed `SectionInvariant` ("a chord end along the
  segment's edge no longer resolves"), though `segment_curve` read it
  off a half-edge's `start` in `first_chord` / `second_chord` a line
  before, on the same `&Body`. Those functions' own hops past the
  join's halves (`corrupt_he`, `corrupt_loop`, `corrupt_face`) are typed
  `SplitJoinError::Corrupt` raises of this row's, so the chord end
  stays typed with them and moves when they do.
- `splitting/classify.rs` `sphere_zone_reach` checks for a
  `LoopBoundary::Empty` outer loop before it calls `props::loop_edges`,
  because `LoopEdgesError::Corrupt` answers an empty loop and a torn hop
  alike. That pre-check is this row's `LoopEdgesError::Corrupt` split
  done locally at one caller; once the split lands, the empty loop is
  its own variant and the pre-check (and the second read of the outer
  loop) goes.
- `chord_join.rs` `face_azimuth_window_traces` (`sweep-testing`): the
  surface's `Corrupt`.
- `splitting/join.rs` `split_leave`: the face's surface (`Corrupt`).
- `splitting/classify.rs` `sphere_zone_reach` now panics on
  `props::loop_edges`' `LoopEdgesError::Corrupt` past a cycle outer
  loop, and on `sphere_chart_trim`'s `CorruptFace`, both past a face
  the gate resolved. `loop_edges` and `sphere_chart_trim` themselves
  are unchanged and still answer typed for other callers.

**What `solid_contain.rs` now folds.** `cylinder_chart_trim`'s
`face_azimuth_window(..).ok()` and the cone arm's
`face_azimuth_images(..).ok()` / `Err(_)` fold into `CorruptFace` only
the walk's geometric refusals: a run edge with no closed-form chart
image, a fitted image, a vertex off the carrier, `ApexUnlifted`, and,
in `cylinder_chart_trim` and the cone's `face_azimuth_images` fold, an
escalation. Those are not corruption, so the `CorruptFace` label there
is now wrong in every case it fires; this row's split ("needs its own
variant first") is where it gets a name. `wall_outline`'s
`Err(_) => unsupported()` and `sphere_chart_trim`'s `Err(_) => Ok(None)`
fold only geometric refusals, which is what they claim.
