---
id: a-displays-constant-ending-answers-one-door-at-every-door
kind: issue
title: A Display still writes a constant ending (DEFECT, KERNEL_DEFECT_ENDING) or a helper's fixed-door ending, which answers one door at every door
status: open
opened: 2026-11-03
---


## Finding

Found by E1e's sweep (PR "PCERT E1e: a refusal's Display carries only its payload"). E1e makes every `Display` that *spelled a reading* payload-only, and `scripts/gates/display-payload-only.sh` holds that. Two shapes it cannot see still let a `Display` end one door's way at every door:

1. **A constant ending string.** `geom_core::KERNEL_DEFECT_ENDING` (the build's culprit), `KERNEL_OR_FILE_DEFECT_ENDING` (at rest's), and topo's at-rest table `validate::{DEFECT, NOT_YET, TOLERANCE}` are written into `Display` bodies as text. For example, `ShellError`/`AsShelled` (`crates/topo/src/shell.rs`), `PieceSortError` (`crates/topo/src/pieces.rs`), `ValidationError` (`crates/topo/src/validate.rs`) and `SplitFinishError` (`crates/topo/src/splitting/finish.rs`) do this. The validate table is the same ending `pcurve-certify-error-arms-with-no-ending` (P1) names as a second implementation.
2. **A helper that fixes the door.** A `Display` (or a `write` with `at`) calls a helper whose ending is fixed at one reading inside it. Examples: `BooleanDecision::recourse`/`render` (`crates/topo/src/boolean/refusal_routes.rs`, at a build); `merge_faces::undescribed_ending` (at a build); `validate::{classify_pcurve, classify_offset_fit, classify_mass_props, classify_census_cause}` (at rest); `ContainDecision::lever_ending` (at a build). E1e routes the Boolean decision's ending through `BooleanDecision::written` so a payload omits it, but the ending itself stays a build's.

## The question

Should these endings take the door too, so a payload carries no ending at all and the door's own culprit is named? A constant culprit ending would become `recourse::defect_ending(at)` read at the door, and the helpers would take `at`. The gate would then have to see constant ending text in a `Display`, which needs the constants named in one place.
