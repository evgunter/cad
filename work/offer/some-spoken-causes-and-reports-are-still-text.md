---
id: some-spoken-causes-and-reports-are-still-text
kind: issue
title: kernel: a few spoken causes are stored as text and the text reports flatten, so their mentions cannot be links
status: review
opened: 2026-11-22
priority: P2
cost: M
refs: [the-viewer-spells-a-node-as-its-kind-before-its-name, a-node-name-the-viewer-shows-is-not-a-link-to-it, a-witness-rebuilds-refusal-spells-its-node-as-text]
pr: 7576
branch: offer/spoken-causes-are-sentences
---


Left by `the-viewer-spells-a-node-as-its-kind-before-its-name` (PR #7375), which made every refusal a `Sentence` of words and mentions. A few places still hold a spoken sentence as a `String`, flattened to text where the value is built. Each mention is spelled correctly but reaches the viewer as words, so the link unit (`a-node-name-the-viewer-shows-is-not-a-link-to-it`) cannot draw it as a link.

**Causes stored as text** (the first four built with `NodeError::kind_spoken(doc).to_string()`):
- `DriveRefusal::WitnessDoesNotBuild { cause }` (`crates/editor-core/src/drive.rs`, `drive`);
- `McRefusal::NominalDoesNotBuild { cause }` (`crates/editor-core/src/mc.rs`, `monte_carlo`);
- the stackup measure-refusal causes `StackupRefusal::{MeasureRefusedAtNominal, LeafDiverged} { cause }`, through `Evaluation::reading_spoken` (`crates/editor-core/src/eval/mod.rs`);
- `Unsettled::Failed { cause }`, a node that fails and reads no variable of the drive's box (`drive.rs`, the leaf verdict);
- `ClearanceOutcome`'s and `ClearanceAnswer`'s `sentence: String` (`crates/pncad/src/analysis.rs`: an unevaluated assertion's outcome, a refused clearance);
- `ResolveFailure.message: String`, which `WorkspaceError::resolve_failure` (`crates/pncad/src/workspace.rs`) writes from the store's refusal and `PartFault::Unresolved` (`crates/editor-core/src/eval/parts.rs`) says as words. A part's load refusal therefore reaches the viewer as words, its nodes among them.

**Text reports that flatten at their end:**
- `McReport::render` (`mc.rs`);
- `Stackup::render` and `render_sensitivity` (`stackup.rs`);
- `ParamBoxVerdict::render` (`drive.rs`), whose lines under "reads no variable" name each node as text;
- `MassBudget::render` and the leaf histogram's `render` (`report.rs`);
- `ClearanceReport::render` (`clearance.rs`).

Each builds a `Sentence` with the document's speaker and returns its text.

**The fix.** Store each cause as a `Sentence`; have each report return a `Sentence` (or a list of them, one per line); let `pncad-py` take the text where it hands one to Python. This belongs to the unit that holds every message as a sentence.
