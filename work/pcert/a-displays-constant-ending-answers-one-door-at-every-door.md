---
id: a-displays-constant-ending-answers-one-door-at-every-door
kind: issue
title: A Display still writes a constant ending (DEFECT, KERNEL_DEFECT_ENDING) or a helper's fixed-door ending, which answers one door at every door
status: open
opened: 2026-11-03
priority: P3
cost: M
---


## Finding

Found by E1e's sweep (PR "PCERT E1e: a refusal's Display carries only its payload"). E1e makes every `Display` that *spelled a reading* payload-only, and `scripts/gates/display-payload-only.sh` holds that. Two shapes it cannot see still let a `Display` end one door's way at every door:

1. **A constant ending string.** `geom_core::KERNEL_DEFECT_ENDING` (the build's culprit), `KERNEL_OR_FILE_DEFECT_ENDING` (at rest's), and topo's at-rest table `validate::{DEFECT, NOT_YET, TOLERANCE}` are written into `Display` bodies as text. For example, `ShellError`/`AsShelled` (`crates/topo/src/shell.rs`), `PieceSortError` (`crates/topo/src/pieces.rs`), `ValidationError` (`crates/topo/src/validate.rs`) and `SplitFinishError` (`crates/topo/src/splitting/finish.rs`) do this. The validate table is the same ending `pcurve-certify-error-arms-with-no-ending` (P1) names as a second implementation.
2. **A helper that fixes the door.** A `Display` (or a `write` with `at`) calls a helper whose ending is fixed at one reading inside it. Examples: `BooleanDecision::recourse`/`render` (`crates/topo/src/boolean/refusal_routes.rs`, at a build); `merge_faces::undescribed_ending` (at a build); `validate::{classify_pcurve, classify_offset_fit, classify_mass_props, classify_census_cause}` (at rest); `ContainDecision::lever_ending` (at a build). E1e routes the Boolean decision's ending through `BooleanDecision::written` so a payload omits it, but the ending itself stays a build's.

## Every fixed-door helper still left (as of E1e's fix pass)

Each of these spells its ending at one door inside itself, and a refusal's payload `Display` or `write` reaches it. Some are reached unconditionally: their text is in the payload whatever door reads it. Others are reached only when `at` is `Some`, but at a fixed door whatever `at` is.

- `crates/topo/src/boolean/refusal_routes.rs:1271`: `neighbour_ending`, a build's ending, written into `BooleanError::CoplanarNeighbours` and `MergeCoplanarError`'s neighbour arms whatever the door.
- `crates/topo/src/boolean/refusal_routes.rs:1304` and `:1738`: `Ending::recourse` and `BooleanDecision::recourse`, at a build. `BooleanDecision::written` keeps them out of a payload; at a door they are a build's.
- `crates/topo/src/splitting/mod.rs:707`: `SectorReading::ending`, at a build, written into `SplitReduceError`'s sector arms whatever the door.
- `crates/topo/src/validate.rs:3048`: `WedgeCheck::ending`, at rest.
- `crates/topo/src/validate.rs:3888`: `off_face_ending`, at rest.
- `crates/topo/src/validate.rs:4065`: `close_to_boundary`, at rest.
- `crates/topo/src/validate.rs:3279`: `classify_certify`, at rest.
- `crates/topo/src/validate.rs:4188`: `classify_chart_region`, at rest.
- `crates/topo/src/validate.rs:3898`, `:3629`, `:3721`, `:4275`: `classify_pcurve`, `classify_offset_fit`, `classify_mass_props`, `classify_census_cause`, at rest.
- `crates/topo/src/validate.rs:3094`: the at-rest constants `DEFECT`, and `TOLERANCE` beside it.
- `crates/topo/src/merge_faces.rs:801`: `undescribed_ending`, at a build.
- `crates/topo/src/boolean/contain.rs:173`: `ContainDecision::lever_ending`, at a build.
- `crates/topo/src/shell.rs:798`, `:837` and others: `KERNEL_DEFECT_ENDING` written into `ShellError` / `AsShelled` arms, a build's culprit.

The at-rest gate's helpers are read only by `ValidationError`, whose one door is at rest, so they end at its reader's door in practice. The rest answer one door at every door.

## The question

Should these endings take the door too, so a payload carries no ending at all and the door's own culprit is named? A constant culprit ending would become `recourse::defect_ending(at)` read at the door, and the helpers would take `at`. The gate would then have to see constant ending text in a `Display`, which needs the constants named in one place.
