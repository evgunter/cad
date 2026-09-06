# CURVED-MERGEDOOR — the surviving cylindrical declared-Rest pair at the planar merge door (spec)

Closes `work/curved/cylindrical-rest-pair-hits-planar-merge.md`. Branch
`curved/mergedoor-skip`. Difficulty pre-logged **S**; task class
**structural** (reasoning under "PR shape"). Survey run 2026-09-04
against main `6c0272a7f` (after MATE-7a, MATE-9, VERBS-1031B, CYLSPH):
every line cite below was re-derived on that head by symbol. **The four
rows were NOT re-run** — the build mutex was held by other lanes'
batteries for the whole of this lane's life (eight bounded attempts,
two detached waiters, one of them alive 2.5 h without the slot). The
table under "The rows" is therefore a PRE-REGISTERED PREDICTION from
the source reads, and re-taking it is the implementer's opening act
(row 0). Read "What the survey found" first: the item's central
sentence — "two surviving co-carrier cylindrical faces that a user
declared coincident and that nothing merges" — is refuted on the
source, and the refutation sizes the unit.

Vocabulary. A *carrier* is the analytic surface a face lies on
(`Surface::{Plane, Cylinder, …}` in the geometry arena); faces share a
carrier *structurally* (one `SurfaceKey`, or one `GeomSource`) or by
*declaration* (a `FacePairDeclaration` on a boolean). A face's *sense*
is the bit saying whether its outward normal agrees with the carrier's
own orientation (S10). The *merge door* is
`Body::merge_coplanar_faces_declared` (`crates/topo/src/merge_faces.rs`),
the boolean's documented F7 output stage; a *skip record* is a
`SkippedMerge` in `MergeCoplanarOutcome::skipped`, relayed verbatim into
`BooleanNaming::merge_skipped` — the ratified visible-skip surface
(review F1/F2, pinned by `crates/topo/tests/merge_skip.rs`).

## The claim

`boolean::ops::declared_surface_pairs` (`ops.rs:1322`) lowers every
surviving declared `Rest` pair to a result surface pair and hands the
list to the merge door at three sites — `ops.rs:559` (seamed lane),
`ops.rs:2241` (combine/assembly lane) and `rest.rs:315` (the
declared-REST zip lane, the one a declared peg-in-bore union exits
through). The door validates the list up front, before any group or
regime exists (`merge_faces.rs:631-645`): a surface that is not a
`Plane` is `Err(MergeCoplanarError::InvalidDeclaration { surface, what:
"declared surface is not a plane" })`, rendered (`:377-380`) as

    merge_coplanar_faces: invalid declared pair at surface SurfaceKey(..): declared surface is not a plane

Raised before `outcome` exists, it is always an `Err` of the call and
never a skip record, so the boolean refuses `Merge(InvalidDeclaration)`
— a door blaming the user's declaration for its caller's choice, and
blaming a declaration the boolean's own door ADMITTED:
`validate_declarations` (`boolean/mod.rs:2350-2367`) names the
declarable inventory as **plane, sphere, cylinder, torus**. Two doors of
one pipeline disagree about what a legal declaration is; the merge door
is the one that is wrong, and this unit makes it agree.

## What the survey found

**Held.**

- P1 — the hand-off is unconditional: a pair is kept iff its class is
  `Rest`, both operand faces resolve, and both lowered surface keys are
  live in the result and distinct (`ops.rs:1339-1351`). Surface
  liveness is a fair proxy for face liveness — every face kill removes
  an orphaned surface (`Body::remove_surface_if_orphaned`, from
  `euler_kill.rs:473` and `:945`) — which is why full engagement offers
  no cylindrical pair and the two-peg demo glues
  (`demos/tour/src/twopeg.rs:55-60, 472-484`).
- P2 — the door's declared-pair rung is planar STRUCTURALLY, not only by
  the gate: `planes_declared_equal` destructures `(Plane, Plane)` and
  `return Ok(None)` otherwise (`merge_faces.rs:1027-1043`). Removing the
  gate alone would make a cylindrical pair license nothing, silently.
  The gate is the only thing that turns "no rung" into "your
  declaration is invalid".
- P3 — the door's own regime doctrine prescribes the answer.
  `GroupRegime::RecordsASkip` (`merge_faces.rs:161-168`): declared-
  licensed runs RECORD an inventory refusal because "the declaration
  served the consuming op's classification even where the glue is
  outside the inventory". That is this case verbatim — the cylinder
  `Rest` served the reduction's covered rung and the zip; the merge has
  no rung for its carrier. The up-front `InvalidDeclaration` arm is the
  one place the door breaks its own doctrine.
- P4 — the visible-skip surface is carried at all three sites with no
  call-site change: `merge_skipped: merged.skipped.clone()`
  (`ops.rs:599`, `:2272`, `rest.rs:347`), documented "the skip is
  visible HERE" (`ops.rs:199-213`), pinned by
  `merge_skip::skipped_declared_merge_is_tier3_green_and_visible`.
  Nothing outside `topo` reads it yet (grep); the naming layer reads
  `merge_groups` only (`editor-core/src/names/emit_topo.rs:532`).
- P5 — the boolean's `Rest` verification accepts BOTH orientations
  (`verify_rest_declaration`, `mod.rs:2017-2047`: `Ok(_)` on
  `SameOriented` and `SameOpposite`), so in a boolean a `Rest` pair
  does double duty — opposed = contact patch (the zip's), same-oriented
  = flush pair (the merge's planar declared rung). The contact DOOR
  contradicts aligned senses (`contact_verify.rs:139-170`); the boolean
  does not consult it for `Rest`.

**Refuted.**

- R1 — **there is no merge work behind the door on any of the four
  rows, and there cannot be for an OPPOSED `Rest` pair on any
  carrier.** A peg wall against a bore wall is
  `CarrierRelation::SameOpposite` (`carrier_eq.rs:38-44`), and the
  merge's every rung has shared sense as a precondition (S10,
  `merge_faces.rs:958-980`): two faces on one carrier with differing
  sense bits are the two sides of a slit, and the declared rung refuses
  them `DeclaredOppositeOrientation`. The faces a peg-in-bore `Rest`
  declaration leaves alive after the zip — the bore wall's uncovered
  remainder (sense in) and the peg wall's proud remainder (sense out) —
  are never a merge candidate, planar or curved; on the four scenes
  they do not even share an edge (each meets a cap or the collar's top
  annulus at its rim). The item's sentence describes a slit, and a
  modeller expects nothing merged across a slit. Prediction, pinned by
  row 2 below.
- R2 — the configuration that DOES want a curved merge — same-sense
  cosurface walls with disjoint extents, the stacked rounded plates of
  `work/bool/cosurface-disjoint-curved-walls-refuse.md` — never reaches
  this door: it refuses `CurvedPierceUnsupported` in the reduction,
  declared or not, and needs a class that is not `Rest` (that item's
  own analysis; `twopeg.rs:130-147`). The curved merge arm's only
  consumer is behind another item's door.
- R3 — "the probe's allow-list should come back down" is one file, not
  four. Only `r1_probes_m9_3::probe_partial_engagement_never_silent`
  names the refusal (`r1_probes_m9_3.rs:341-352`). The two
  `mate2_r2_probes` rows accept any typed refusal (`never_silent`,
  `:24-52`); `mate2_r1_probes::probe_out_by_height_reports_its_outcome`
  asserts nothing — it prints (`mate2_r1_probes.rs:77-101`; per
  `memories/test-suite-cost.md` a row that asserts nothing is not a
  gate and is not counted as one here).

## The rows — PRE-REGISTERED, re-taken by the implementer (row 0)

| Scene | Fixture (`mate2_common` unless noted) | Predicted today | Predicted after this unit |
|---|---|---|---|
| A floating peg | `collar()` ∪ `peg_at(0, 1.5, 1.0)`, 9 wall `Rest`s | `Merge(InvalidDeclaration{not a plane})` | `Ok`, additive to 4 ULP, tier 3/3′ green; 1+ `DeclaredCarrierUnsupported{Cylinder}` records; surviving r=0.5 faces: bore z∈[1,1.5] sense in, peg z∈[2,2.5] sense out, no shared edge |
| B mid-bore | `collar_at(0)` ∪ `peg_at(0, 0.5, 1.0)` | same | same shape: bore z∈[1.5,2] in, peg z∈[0.5,1] out |
| C proud one end | `collar_at(0)` ∪ `peg_at(0, 1.0, 1.5)` | same | `Ok`; bore fully consumed so its surface is orphaned and DROPS at P1 — 0 records predicted; peg z∈[2,2.5] survives alone |
| D partial engagement | `r1_probes_m9_3.rs:292` (plate+peg ∪ plate−bore, 1 planar + 9 wall `Rest`s) | same | `Ok`; planar pair licenses through `eq`, the bore remainder z∈[1.5,2] survives (peg consumed) — 0 or 1 record depending on whether the peg's wall surface is orphaned; record the count |
| E stacked equal pegs, caps only | `peg_at(0,0,1)` ∪ `peg_at(0,1,1)`, cap `Rest` | `CurvedPierceUnsupported` (R2) | unchanged — the successor's opening row |
| F stacked pegs, caps + walls `Rest` | as E plus 9 wall pairs | same (`twopeg.rs:143-147`) | unchanged |

Scene C/D carry a prediction the source cannot settle — whether a
consumed side's surface is orphaned BEFORE `declared_surface_pairs`
runs (the zip's face kills are `euler_kill` kills, so yes; the
curve-reference exception at `body.rs:410` is the hole). If a record
appears with EMPTY `faces`, STOP 3 fires. The scratch probe that takes
this table (six scenes, prints refusal, volume, tiers, records, the
surviving r=0.5 faces' sense/adjacency, and orphan surfaces) is at
`/home/evan/.local/share/cad-work/curved-merge-skip-spec-scratch-probe.rs`
and registers as a module of `crates/sweep/tests/all.rs`; the
implementer runs it red first and pastes the table into the PR body.

## The mechanism: the door records what it has no rung for

Shape 1, AT THE DOOR (the brief's "declared_surface_pairs (or the
door)"). `declared_surface_pairs` is unchanged. In
`merge_coplanar_faces_declared` the up-front validation
(`merge_faces.rs:631-645`) becomes a classification of each declared
pair by carrier kind:

- both surfaces `Plane` → `eq.union(k1, k2)`, exactly as today;
- a key that does not resolve → `InvalidDeclaration { what: "declared
  surface key does not resolve" }`, an `Err` as today (a torn argument
  is a caller bug, not an inventory limit);
- two DIFFERENT kinds → `InvalidDeclaration { what: "declared surfaces
  are not one kind" }`, an `Err`: one carrier cannot be two kinds, the
  boolean's own verification never lets this through, and the public
  door keeps refusing it (the `GroupKindSplit` posture, `:227-241`);
- one NON-PLANAR kind on both sides → a skip record, not an error:

      SkippedMerge {
          faces:  every live face whose surface is k1 or k2, face-arena order,
          reason: MergeCoplanarError::DeclaredCarrierUnsupported {
              pair: (k1, k2),
              kind: geom_brep::SurfaceKind,   // Cylinder | Sphere | Torus | Cone | Nurbs | Approx
          },
      }

  pushed into `outcome.skipped` ahead of the group records. `faces` is
  read off the COMMITTED body (`work`, after the surgery loop), so every
  recorded face is live in the shipped result — the invariant
  `merge_skip.rs:74-77` asserts. The `!any` early return (`:682-684`)
  returns the declined records too, never
  `MergeCoplanarOutcome::default()` when there are any.

`DeclaredCarrierUnsupported` is a new `MergeCoplanarError` variant,
constructed only as a skip reason and never as an `Err` of the door
(the `GroupNotClosed` precedent, `:212-224`: "scope-specific … tells
the reader which gate spoke"); its docs say so. `is_arena_fault`
(`:447-452`) is untouched — the variant is an inventory statement. The
payload carries the surface pair (a consumer can walk the result for
what the declaration covered) and the kind (`geom_brep::SurfaceKind::of`,
`intersect.rs:116`; `topo` already depends on `geom-brep`).

**Why the door and not the caller.** The alternative — filter in
`declared_surface_pairs`, return the declined pairs, thread them into a
new `BooleanNaming` field at three call sites and four constructors —
surfaces the same record with more S-BOOL ground touched and puts
`ops.rs`-minted values into a field documented as relaying
`MergeCoplanarOutcome::skipped` (`ops.rs:199-213`, the D288 rewrite).
At the door the record is minted where the inventory limit lives,
`merge_skipped` carries it with zero call-site edits, and the public
door becomes consistent with its own regime doctrine for every caller.
The caller-side shape is the fallback only if Open question 1 is ruled
the other way. **Why not filter silently:** an invisible skip is the
escape-hatch shape (`memories/test-suite-cost.md`); a consumer looking
at a body with an unmerged declared pair must be able to see why.

## Shape 2 — what a curved declared rung would be, and why it is not this unit's

The rung would be `planes_declared_equal`'s third branch for
`(Cylinder, Cylinder)` (then sphere, torus): verify the pair through
`carrier_pair_verdict(.., declared: true, ..)` (`rest.rs:629`) — the
`carrier_eq` kind ladder already answers `SameOriented | SameOpposite |
Distinct` for cylinders with the `outward` bit folding sense
(`carrier_eq.rs:33-44`) — license the adjacency on `SameOriented`,
refuse `SameOpposite` as `DeclaredOppositeOrientation`. "Adjacent on
one carrier" means what it means for a plane at this door: the two
faces share an edge (`merge_faces.rs:662-680`); the existing curved-run
machinery then applies (`PeriodClosure` for a full-period close,
`:249-266`; sub-period re-merges commit). No angular-window arithmetic:
the merge glues across shared edges, chart overlap is Door 2's
(`chart_region.rs`) and the census's, not the merge's (`:24-33`).

It is a successor's because it has no reachable consumer: a
peg-in-bore `Rest` pair is `SameOpposite` (R1), and the one
`SameOriented` curved configuration a modeller wants glued (R2) is
refused two doors upstream and needs a class that is not `Rest`
(`cosurface-disjoint-curved-walls-refuse`'s open question, CURVED's
operand-reach lane). The arm banks behind that item; when the
reduction admits a same-sense cosurface pair the merge door is its
next door, and THIS unit's record is what will show it arriving —
`DeclaredCarrierUnsupported { kind: Cylinder }` with a shared edge.
Row 7 pre-registers that. **No `carrier_eq` change** (fence).

## The message

Rendered `Display` of the new reason — the text row 6 asserts:

    merge_coplanar_faces: declared pair (SurfaceKey(3v1), SurfaceKey(9v1)) lies on a cylinder carrier — the declaration is legal and served the op, but this door's declared-pair rung is planar and has no cylinder arm; the pair is left unmerged and recorded

Which pair; that the declaration is not at fault and who consumed it;
which rung is missing and what happened instead. Never "invalid". The
`InvalidDeclaration` text is unchanged for the two cases that keep it.

## Fences

- `crates/topo/src/merge_faces.rs`: the declared-pair validation block
  of `merge_coplanar_faces_declared` (`:631-652`), the `!any` early
  return (`:682-684`), the record push (modelled on `:750-753`, placed
  before the loop), the new variant and its `Display` arm, and the doc
  sentences on `SkippedMerge`, `MergeCoplanarOutcome::skipped` and the
  fn docs that say "planes only". Nothing else in that file:
  `planes_declared_equal`, `group_regime`, `merge_group`, winding and
  strut code untouched.
- `crates/topo/src/boolean/ops.rs` and `rest.rs`: **no edit** under the
  recommended shape. Under the fallback shape: `declared_surface_pairs`,
  the three `merge_skipped:` lines and one `BooleanNaming` field, and
  nothing else. Both files are S-BOOL `paths:` ground
  (`work/bool/program.md`); the orchestrator announces this unit on the
  away channel before it lands.
- No `carrier_eq.rs`, `rest.rs`, `contact_verify.rs` or
  `validate_declarations` change; the declarable inventory neither
  widens nor narrows. No `BooleanError` variant (a record, not a
  refusal). No demo re-authored.

## STOP conditions (pre-registered)

1. A row's post-door `Ok` body is NOT exactly additive, tier-3 valid
   and 3′-clean — the skip has exposed a silently wrong body from the
   ZIP. Stop, file with the payload; never narrow the record.
2. Opening the door lands a row on a DIFFERENT refusal
   (`describe_minted_edges`, `gate`, `volume_backstop`). Measure, file;
   a fifth door is outside this unit.
3. A record with EMPTY `faces` on any row: the surface-liveness proxy
   (P1) has a hole and the record names nothing. Stop; the fix is a
   face-liveness test in `declared_surface_pairs` (fallback ground).
4. Row 0's table contradicts R1 — a `SameOriented` surviving pair or a
   shared edge between the two surviving cylindrical faces on A–D.
   That is the successor's consumer arriving early: stop and re-scope.
5. Any planar fixture's `merge_groups`/`merge_skipped` moves.

## Acceptance rows (red-first; each names its mutant)

New module `crates/sweep/tests/curved_mergedoor.rs` under the `all`
harness, scenes from `mate2_common` (+ the two `r1_probes_m9_3`
builders, which the module copies rather than imports — they are
private there).

0. **Opening measurement**: the six-scene table above, re-taken on the
   lane's base and pasted into the PR body before any mechanism lands.
1. **`floating_peg_declared_walls_union_records_the_cylinder_skip`**
   (A): `Ok`; `assert_additive`; tier 3 and 3′ green; `merge_skipped`
   holds ≥1 record with `reason` matching
   `DeclaredCarrierUnsupported { kind: Cylinder, .. }`; every recorded
   face live and on a `Cylinder`; no cylinder face in `merge_groups`.
   Kills **M1** the door still refuses; **M2** the skip is silent;
   **M3** the `!any` early return drops the records (A has no planar
   declared pair, so `any` is false).
2. **`opened_door_leaves_a_slit_not_a_merge_candidate`** (A, B): the
   surviving r=0.5 faces have OPPOSED senses and share no edge — R1
   pinned. Kills **M4**: a future curved rung gluing `SameOpposite`.
3. **`partial_engagement_records_beside_the_planar_pair`** (C, D): the
   planar pair unions through `eq`, the cylindrical ones record (or
   drop at P1 — assert the count row 0 measured), both visible. Kills
   **M5** the classification keys off the first pair's kind; **M6**
   `faces` read off `self` before surgery (pin by liveness).
4. **`mixed_kind_declared_pair_still_refuses_typed`** (public door,
   `m3_pr1_surgery.rs` style): `(Plane, Cylinder)` →
   `Err(InvalidDeclaration { what: "declared surfaces are not one kind" })`,
   body untouched. Kills **M7** the mixed case swallowed as a record.
5. **`unresolved_declared_key_still_refuses`**: kills **M8** the
   resolve check moved behind the kind check.
6. **`rendered_skip_names_the_door_not_the_declaration`**: the text
   contains "declaration is legal" and "no cylinder arm", not
   "invalid". Kills **M9** the old message re-used.
7. **`stacked_equal_pegs_same_sense_walls`** (E, F): REPORTS the door
   they stop at today (`CurvedPierceUnsupported`, with and without
   wall declarations) and asserts only that it is typed. Kills nothing
   here; it turns red when `cosurface-disjoint-curved-walls-refuse`
   lands, and its comment names the curved rung as the door after.

**Allow-lists that come down.** `r1_probes_m9_3.rs:341-352`: the
`Merge(MergeCoplanarError::InvalidDeclaration { .. })` arm is DELETED
from the match, with the comment block `:331-340`; the row's `Ok`
branch (exact additivity, tier 3) now runs. The other three rows change
nothing (R3): `mate2_r2_probes`' two rows fall into `never_silent`'s
`Ok` branch, whose additivity/tier-3/3′ assertions run on these scenes
for the first time — STOP 1 is for them; `probe_out_by_height_…` keeps
printing.

## PR shape, difficulty, task class

One PR. **S**: the geometry is a `match` on a closed kind enum and a
record push; the work is four "planes only" doc sentences, a rendered
message, and seven red-first rows, five of which are one assertion on
four scenes. Easier than the plan's "easy half", because the door's
own doctrine prescribes the answer and no call site moves. It is not L
only because rows 0 and 2 are claims about the ZIP's output nobody has
asserted, and a lane that finds them false has found a zip defect
(STOP 1/4). **Structural**: no margin, band or numeric decision is
added or moved.

## Open questions (rulings needed before dispatch)

1. May the PUBLIC door's posture change — a non-planar declared pair
   records instead of refusing, for every caller? Recommended yes (it
   is the door's own regime doctrine, P3); the only out-of-boolean
   caller passes planes (`m3_pr1_surgery.rs:470`). If no, the fallback
   shape applies with the fence above.
2. Should the record carry `faces` when a `Rest` pair's faces are a
   slit's two sides? Recommended yes — `SkippedMerge`'s contract is
   faces + typed reason (F2/F3) — with STOP 3 guarding the empty case.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. Hosted CI is the
verification of record (twelve test jobs on a code-tier run; no
`CI-Config` trailer). Own `CARGO_TARGET_DIR` outside the worktree;
lane-private drafts. Row 0 first, then rows red, then the mechanism.
Merge `origin/main` before opening; announce on S-BOOL's away channel.
Report: the row-0 table on base and head, the rendered message, the
allow-list diff. Do not merge.

---

## Rulings at ratification (CURVED orchestrator, 2026-09-05)

Ratified as written. Answers to §Open questions:

1. **Yes — the public door's posture changes for every caller**: a
   non-planar declared pair is RECORDED as a skip, never refused as an
   invalid declaration. That is the door's own regime doctrine
   (`RecordsASkip`) applied to the one arm that broke it; the sole
   out-of-boolean caller passes planes.
2. **Yes — the record carries `faces`** (the F2/F3 contract), STOP 3
   guarding the empty case.
3. **Announcement**: `merge_faces.rs` is on no program's `paths:` glob
   but sits beside S-BOOL's boolean ground; the orchestrator announces
   the unit on the away channel at dispatch (the S-BOOL handover thread
   on #1835). The implementer does not wait on an acknowledgement — the
   fence is one validation block and one variant.

**Row 0 is the opening act**: re-take the six-scene probe
(`/home/evan/.local/share/cad-work/curved-merge-skip-spec-scratch-probe.rs`,
lane-private) on the head before any edit and quote the refusals,
records, survivor senses and adjacency in the PR body; if any scene
lands off the pre-registered prediction, STOP and report.

**Branch** `curved/merge-door`. **Pre-log stands: S / STRUCTURAL.**
