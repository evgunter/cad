---
id: payload-rung-re-sweep-finds-five-more-uncurated-discriminants
kind: issue
title: five more uncurated discriminants of curated types, from LIB-CUR5's re-sweep
status: closed
opened: 2026-09-08
refs: [LIB-CUR5]
closed: 2026-09-08
---

LIB-CUR5 closed the five rows CUR3 and CUR4 banked, and re-swept the
class at its own merge base as the discipline requires. The re-sweep
found the banked set empty and five hits that are NEW — none of them
named by CUR3's table, CUR4's, or any of the five items CUR5 closed.

## The pattern, and what it could not match

For every name the three curated façade lists (`document.rs`,
`select.rs`, `prelude.rs`) introduce that resolves to a `pub enum` or
`pub struct` declared in the façade's own path-dependency set, take
every type identifier appearing in the declaration's body and report
those declared in that same set and on no curated list. Crate-aware,
so `viewer::blend::BlendError` cannot shadow `sweep::blend::BlendError`
(CUR4's methodology note). 525 curated names, 796 declared types, 115
raw hits; narrowed to the CUR3/CUR4 shape — an ENUM (a discriminant a
caller branches on) that is not itself an error type and is declared
in the same crate as its carrier — leaves 22, of which 17 are already
argued or are false positives (below).

Blind spots, restated with the two this run adds:

- (a) struct payloads — CLOSED one rung by CUR4 and by this scan,
  which indexes `pub struct` as well.
- (b) a payload named only through a type ALIAS or an associated type.
  This run has a live instance: `CarrierRelation` reads as an
  uncurated payload of `FlushEvidence` and is the SAME TYPE the
  prelude carries as `PlaneRelation`
  (`crates/topo/src/boolean/plane_eq.rs:58`).
- (c) refusals the façade DECLARES rather than re-exports; their
  payloads never enter the compared set.
- (d) a generic parameter instantiated to an uncurated type.
- (e) macro-minted types are invisible to a declaration-level index,
  so every `slotmap` key in a payload position is unscannable.
  Unchanged: this scan reads `pub enum` / `pub struct` lines.
- (f) NEW — the scan reads every field, PUBLIC OR PRIVATE. A private
  field of a curated struct reads as a payload it is not:
  `Provenance`, `NullFacePair` and `CurveGeom` all reported against
  `Body`, whose arenas are private, and `UnitQuantity` against
  `UnitDef`.

## The hit list

| hit | carrier | disposition |
| --- | --- | --- |
| `CensusSubject` | `ValidationError::{CensusUnsupported, CensusLaneUnsupported}` (`crates/topo/src/validate.rs:1013`, `:1034`) | **OPEN, the sharpest of the five.** Exactly CUR4's shape on the refusal CUR4 curated three payloads of: two arms, `Entity(EntityId)` and `FacePair(FaceKey, FaceKey)`, and both of those key types are curated now — so the only thing between a caller and the subject is this discriminant. |
| `MappedCurve` | `EdgeDescription::Scaffold` (`crates/geom-brep/src/description.rs:148`) | **OPEN.** A prelude-carried READ-BACK value whose scaffold arm's payload has no curated name. |
| `RevolvedKind` | `Revolved::kind` (`crates/sweep/src/revolve/mod.rs:208`) | **OPEN.** A pub field of a prelude-carried value, and the case split the module docs call ratified — which is a thing a caller reads. |
| `PromotedKind` | `StepImportError::SurfacePromotion::to` (`crates/step-import/src/lib.rs:356`) | **OPEN.** A pub field of a prelude-carried refusal: which surface kind certified. |
| `ImportContact` | `ImportOptions::declared_contacts` (`crates/step-import/src/lib.rs:459`) | **OPEN, and not a payload at all — an INPUT.** A caller holding the prelude-carried `ImportOptions` cannot push a declaration onto it without naming a type no curated list has. Stronger than the payload shape: an unfillable public field, not an unreadable one. |
| `BandField` | `BandError::InvalidValue` | argued non-carriage, CUR4, in `prelude.rs` with its falsifier. |
| `MarginDiag` | `Indeterminate::margin` | argued non-carriage, LIB-CUR5, in `prelude.rs` with its falsifier. |
| `Diagnosis`, `RecipeEditRef` | `ResolveError` | deliberately interior at LIB-CUR5 (LB17): the telemetry half. |
| `Entry` | `NameTable` | the LB13 seal. Correctly interior, and guarded. |
| `Qualifier` | `RoleSeg` | naming interior; `RoleSeg` is itself the inside of a name, which nothing user-side reads. |
| `Attr`, `AttrKind`, `MetaValue` | `DocEdit`, `EditError` | the appearance and metadata families in `NOT_CARRIED`. |
| `ParamValue` | `ParamEnv` | `NOT_CARRIED`, "types whose curated face is a different shape". |
| `PairingViolation` | `SensitivityRefusal` | `NOT_CARRIED`, the analysis lane's interior. |
| `KProbe` | `DriveConfig::k_probe` | the `interval`-gated `crate::analysis` surface, outside the three lists the census reads — a different list's question. |
| `CarrierRelation` | `FlushEvidence::relation` | **false positive**, blind spot (b): the same type the prelude carries as `PlaneRelation`. |
| `Provenance`, `NullFacePair`, `CurveGeom` | `Body` | **false positives**, blind spot (f): private arena fields. |
| `UnitQuantity` | `UnitDef` | **false positive**, blind spot (f). |
| `ImportContact` (second sighting), `MetaVersionError` etc. | — | error types with their own `Display`, the class #1173 disposed of. |

## What a unit closing it would decide

Whether each of the five is a discriminant a caller BRANCHES on, which
is the CUR3 test and the only one that has ever settled one of these.
`ImportContact` is the one that is not that question at all — an
unfillable input field is a reach defect, not a matchability one, and
it should be judged on whether the import-side declaration channel is
callable through the façade at all.

## Closed

LIB-CUR6 (branch `lib/cur6`). All five settled under the CUR3 test,
with LB17's carrier rule placing each census row. Four carried, one
argued interior at its entry with a falsifier.

- **`CensusSubject` — CARRIED**, prelude group 5 beside the census
  vocabulary it is the subject of. The two arms are two recourses: an
  `Entity` is one carrier outside the certifiable inventory
  (simplify it, or certify it through a supported lane), a `FacePair`
  is a candidate CONTACT (declare the coincidence, or separate the
  faces). Matched exhaustively in `all.rs`
  (`census_subject_is_matchable`), with the unordered-pair half NOT
  pinned and the reason stated: distinguishing `(a, b)` from `(b, a)`
  needs two distinct `FaceKey`s and nothing on the curated lists mints
  one. Census: `INTERIOR` — the validate doors cross their failures as
  joined `Display` prose with a `door` and a `failure_count` and no
  per-arm tag, which is `CensusContact`'s measurement and this type's
  carrier too.
- **`MappedCurve` — NOT carried, argued at the entry** (prelude group
  4, beside `EdgeDescription`) with its falsifier, the
  `BandField`/`MarginDiag` shape and the first of that family whose
  reason is the ARM rather than the payload. Three measurements: the
  `Scaffold` arm is fenced to construction and tier 3 refuses it at
  rest (`ValidationError::ScaffoldAtRest`,
  `crates/topo/src/validate.rs:3324`), while this list carries the
  validation ladder; the authoring form `EdgeDescriptionSpec` is on no
  curated list, so a façade caller can neither be handed a scaffold
  nor write one, and every in-tree consumer of the discriminant is a
  re-MINT rather than a read (`transform.rs:687`,
  `offset_axial.rs:2051`, `replace_face.rs:1811`); and the rung is
  uncarried WHOLE — `EdgeDescription::Chart` carries a `ChartCurve`
  whose `pcurve` field is a four-arm `Pcurve`, neither curated, so
  carrying `MappedCurve` alone would ship the `Convexity`
  inconsistency in reverse. Falsifier: a door that hands a caller the
  description of a TRANSIENT edge, at which point the rung is carried
  whole. No census row, the `MarginDiag` precedent; the
  `EdgeDescription` row points at the argument.
- **`RevolvedKind` — CARRIED**, prelude group 3 beside `Revolved`.
  The same claim one value over: the group's other payloads are what a
  carried REFUSAL says, this is what a carried RESULT says. `Partial`
  and `Full` are two disjoint sets of handles rather than one shape
  with a label — wedge caps and both meridian chains against no caps,
  one seam chain and the wire case's second π-band — so the fields are
  arms, not `Option`s a caller could probe. CONSTRUCTED in `all.rs`
  rather than fabricated: `a_revolve_result_is_matchable_through_the_prelude`
  calls the prelude door twice and reaches both arms. Census:
  `INTERIOR`, the plainest instance of the carrier rule in that file —
  the carrier does not cross at all, since Python speaks the document
  layer and a revolve is a `Node.revolve` whose answer is a body.
- **`PromotedKind` — CARRIED**, prelude group 7. **The slate's carrier
  attribution was wrong and is corrected here**: `SurfacePromotion` is
  an arm of `NormalizationKind`, reached through
  `StructureNormalization` and `StepImport::Solid`, none of which is on
  any curated list — so that chain reaches no curated carrier at all.
  The carrier that does is `StepImportError::RecognitionAmbiguous`,
  whose `kind` field is a `pub` field of a prelude-carried refusal
  (`crates/step-import/src/error.rs:246`). Carried because the two
  kinds have different next moves: `Plane` declining is a flatness
  question at ε_in, `Cylinder` declining is an ill-conditioned axis.
  Python: the carrier PROJECTS a tag, so the payload projects its own
  BESIDE it — `promoted_kind_tag`, exhaustive, with a `TAG_INVENTORY`
  row and a Rust construction pin, surfacing at
  `StepImportError.promoted_kind` (`None` on every other arm). Not
  forwarded, and that is the decision: `recognition_ambiguous` names
  the condition and a caller branching on the import ladder needs it
  to stay put. Census: `BOUND_AS`, `MeshPickError`'s spelling.
- **`ImportContact` — CARRIED as a reach defect**, prelude group 7.
  Judged as the item asked, not on matchability: `ImportOptions` was
  on the list and its `pub declared_contacts: Vec<ImportContact>`
  could not be filled, because filling a public field means spelling
  its element type. So the import-side declaration channel (M9-2, D7
  step 4) was callable and not fillable, and had no prelude caller at
  all — what a caller could not do before is attach a
  position-anchored declaration to an import and have it certified by
  the same tier-3′ gate a native declared-contact body runs. The
  façade row FILLS it:
  `the_import_surface_is_matchable_and_fillable_through_the_prelude`
  builds the options with a contact in them and hands them to
  `import_step`. Census: `different-shape` beside `ImportOptions`,
  which is `import_step`'s absent second argument in Python — the
  defect is a Rust one and does not reproduce there; the entry names
  the door whose binding would flip it.

**What this does NOT cover.** The stanza this item's `PromotedKind`
row named — `NormalizationKind`, `StructureNormalization` and
`StepImport` itself, the SUCCESS side of an import — is untouched and
is not a payload-rung question: `StepImport` is the door's own return
type and no curated list names it, which is a reach question about
what a successful import reports. Not filed as a payload-rung row for
that reason; it has its own file,
`a-successful-step-imports-own-report-is-uncurated`.

The re-sweep at LIB-CUR6's merge base is filed as
`payload-rung-re-sweep-finds-six-uncurated-profile-discriminants`,
with the full table, its numbers, and blind spots (a)–(h): (f) and a
new (g) closed, (e) narrowed, a new (h) open.
