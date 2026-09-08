---
id: payload-rung-re-sweep-finds-five-more-uncurated-discriminants
kind: issue
title: five more uncurated discriminants of curated types, from LIB-CUR5's re-sweep
status: open
opened: 2026-09-08
refs: [LIB-CUR5]
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
