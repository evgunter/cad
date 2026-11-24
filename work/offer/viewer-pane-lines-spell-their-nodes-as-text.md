---
id: viewer-pane-lines-spell-their-nodes-as-text
kind: issue
title: viewer: a pane's tool lines, panel refusals and hover texts spell the nodes they name as text, so none is a link
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [a-node-name-the-viewer-shows-is-not-a-link-to-it, some-spoken-causes-and-reports-are-still-text]
---

Left by `a-node-name-the-viewer-shows-is-not-a-link-to-it`, which made the status line, the Checks window's findings, the feature tree's failure and carried lines, the Properties headings, relation entries and `range?` readings, the picked-entity header, the placement editor's "copy of" and the View pane's names draw a kernel `Sentence` through `widgets::sentence` (which takes a `frame::Tone`), each mention a link. Its sweep (implementer discipline §5) found these sites still flattening a sentence that names nodes to text before it is drawn, so their mentions are words:

**Pane lines drawn through `widgets::message_toned`**, which takes text. Each needs its producer to return the `Sentence` it builds, drawn by `widgets::sentence` with its tone:
- the tools' seat and prompt lines in `crates/viewer/src/pane/create.rs`: `seat_line(seats, by)` and `tool.line(row, by)` (the combine and pattern forms), `seats::pick_said` (`crates/viewer/src/seats.rs`, which spells its node with `sentence!(..).to_string()`), the held-plane and asserted-pick lines `Said(held, by).sentence().to_string()` in `crates/viewer/src/combine.rs` (`HeldPlane::Face`, `AssertedPick::Face`) and `crates/viewer/src/matetool.rs` (the mate tool's picks);
- the panel refusal under a creation form, `crate::shown(refusal)` in `pane/create.rs` (the chooser's refusal), and the profile pane's `why` lines (`crates/viewer/src/pane/profile.rs`);

**Panel refusals as notices**: `tools::PanelRefusal` holds its `why` as a `String`, flattened in `PanelRefusal::new`, so `frame::tool_news` and the mate tool's edit refusals (`pane/features.rs`, `feature_row`) reach the status line as words.

**Viewer faults that spell their nodes in their own `Display`**, which `session::Refusal`'s sentence takes as words (`Refusal::write` spells each before writing it; every payload that has `Say` or is `Staged` is already written as a sentence): `display::DisplayFault` (`Refusal::Display`, `crates/viewer/src/display.rs`), `matetool::MateToolError` (`Refusal::Draft`), `props::SlotUnitFault` (`Refusal::SlotUnit`, `crates/viewer/src/props.rs`), `combine::StepClearFault` (`Refusal::StepClear`, `crates/viewer/src/combine.rs`), `GestureFrameFault` (`Refusal::Frame`) and the sketch faults (`crates/viewer/src/sketch.rs`). Each `write!`s `crate::tree::spelled(node)`; each wants a `Say` impl, and its `Refusal` arm a `Said`. `Refusal::ConstantRefused`'s `EvalError` source has no `Say` either.

**Notices built from text**: `frame::Withdrawal::notice` (`self.to_string()`) and the maintenance row (`row.to_string()` in `frame.rs`, the kernel `Maintenance`, which has no `Say`).

**Badges and rows read as text**, found by `some-spoken-causes-and-reports-are-still-text`'s review (PR #7576). A `Badge` holds its label as a `String`, so these need it to hold a `Sentence` too:
- `frame::product_badge` (`crates/viewer/src/frame.rs`): `fault.spoken(doc).to_string()`, a gather refusal's nodes;
- the undrawn-copies badge in `frame.rs` (`Said(copy, by).sentence().to_string()` per `index.undrawn()` copy), its label and its detail;
- `drafts::latch_said` (`crates/viewer/src/drafts.rs`): a form's latch row, `Said(held, by).sentence().to_string()`;
- `session::OutOfBudget::said` (`crates/viewer/src/session.rs`): the run-out line names its node with `crate::tree::spelled`.

**Hover texts stay text**, by design: a tooltip is gone once the pointer moves toward it, so it cannot hold a link (`on_disabled_hover_text(crate::shown(..))` in `pane/features.rs`, `pane/properties.rs`, `pane/create.rs`). No fix is owed there.

**The fix.** Have each producer above return its `Sentence` and draw it through `widgets::sentence` with its tone, give each viewer fault a `Say`, and hold `PanelRefusal`'s reason as a `Sentence`.
