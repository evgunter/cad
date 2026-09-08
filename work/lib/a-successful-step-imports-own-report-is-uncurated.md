---
id: a-successful-step-imports-own-report-is-uncurated
kind: issue
title: a successful STEP import's report is uncurated: StepImport and the normalization records
status: open
opened: 2026-09-08
refs: [LIB-CUR6]
needs_ev: true
---


LIB-CUR6 corrected the carrier attribution on one of its five rows and
turned up this beside it. The row said `PromotedKind` was
`StepImportError::SurfacePromotion::to`'s. It is not:
`SurfacePromotion` is an arm of `NormalizationKind`
(`crates/step-import/src/lib.rs:290`), reached through
`StructureNormalization::kind` (`:375`) and
`StepImport::Solid::normalizations` (`:490`). CUR6 carried
`PromotedKind` through the carrier that IS curated —
`StepImportError::RecognitionAmbiguous::kind`
(`crates/step-import/src/error.rs:246`) — and left this chain alone,
because it is a different question.

## The finding

`import_step` is prelude-carried and returns
`Result<StepImport, StepImportError>`. **The refusal half is curated
and the SUCCESS half is not.** No curated list names `StepImport`, so
a caller destructures the answer by inference and can read
`body` — and nothing else on it has a spellable type:

- `StepImport::Solid::normalizations: Vec<StructureNormalization>` —
  what the adoption RE-MINTED, "as data, never silently" in the type's
  own words. `StructureNormalization` (`:375`), its
  `NormalizationKind` (`:290`) and the `FaceCensus` pair it carries
  (`:206`) are all uncurated.
- `StepImport::Solid::curve_promotions: Vec<CurvePromotion>` (`:427`),
  with `PromotedCurveKind` (`:396`).
- `StepImport::Solid::instances: Vec<PlacedInstance>` (`:234`) — the
  assembly record A7 keeps precisely because it is expensive to
  retrofit.
- `StepImport::Wireframe`, the other arm, whose payload types are
  likewise unnamed.

This is not the payload rung's question. There the carrier is curated
and its arms are not matchable; here the CARRIER is uncurated, so the
reach clause is what fails: a prelude consumer can call the door and
cannot state the type of what it is handed, so it cannot store one in
a field, return one from a function, or read the report of what the
importer changed about the file. Everything is one module hop away at
`pncad::step_import::…`, so contract clause 1 holds; what does not
hold is that the door's own answer is spellable from the list that
carries the door.

The `ImportContact` row CUR6 closed is the same defect on the INPUT
side of the same call, which is why this one is worth its own row
rather than a sentence: the import door was curated at neither end.

## What a unit closing it would decide

Whether the success value's report is something a caller READS — and
the type's own documentation argues it is, twice: "Reported, never
silent — this is the one normalization that changes a surface's
DESCRIPTION rather than its tessellation", and the assembly record
"kept whether or not the file states an assembly at all". If it is,
the carriage is `StepImport` plus the record vocabulary its `Solid`
arm names, with each Python row placed by LB17's carrier rule —
noting that Python's `import_step` answers a `Body` and drops the
report entirely, so the Rust and Python halves may land apart.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

`import_step` is curated at neither end of its success half: a prelude
consumer can call it and cannot SPELL the type it is handed
(`StepImport`), nor read what the importer changed about the file
(`StructureNormalization`, `NormalizationKind`, `CurvePromotion`,
`PlacedInstance` — the record the type's own docs say is "reported,
never silent"). Python's `import_step` drops the report entirely and
answers a `Body`. Does the success value's report cross, and where?

- **(A) Carry `StepImport` and the record vocabulary its `Solid` arm
  names at the façade (`pncad::step_import` group, under the reach
  clause: a door's own answer is spellable from the list that carries
  the door), and give Python an `ImportReport` value class beside the
  body** — normalizations, promotions, instances as frozen rows, each
  placed by LB17's carrier rule. Rust and Python halves may land apart.
  Recommended.
- **(B) Carry the carrier alone** (`StepImport`), record vocabulary
  interior: a caller can store the answer and still cannot read the
  report.
- **(C) Leave it**: the report stays a Rust-only, module-hop-away read.

Recommendation: **(A)** — the door's own documentation argues twice
that the report is something a caller reads.
