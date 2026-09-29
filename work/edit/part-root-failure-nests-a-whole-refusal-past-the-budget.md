---
id: part-root-failure-nests-a-whole-refusal-past-the-budget
kind: issue
title: editor-core: a part's root failure renders the part's own node refusal inside an 11-word wrapper, so it can outgrow the viewer's 75-word budget by construction
status: spec
branch: edit/part-root-carried-refusal
priority: P3
cost: M
opened: 2026-09-23
refs: [error-and-check-text-overflows-its-region]
---

## What

`NodeErrorKind::Part { fault: PartFault::PartRootFailed { message, .. } }`
renders the referenced document's own failed-node refusal (`message`
is that node's `NodeErrorKind` text, `eval/parts.rs`) inside two
wrappers: `NodeErrorKind::Part`'s "instantiating {doc_ref}:"
(`eval/mod.rs`) and `PartFault::PartRootFailed`'s "the referenced
document's product root {node} failed:" (`eval/parts.rs`). On the
feature tree's fault line that is eleven words in front of the inner
refusal.

Measured by `editor-core/tests/refusal_concision_chains.rs`
(`Part/PartRootFailed(message)`, CHROME concision-chains): 42 words on
an extrude refusal. Every other refusal the feature tree draws is held
to 75 words with its `node N failed:` included, so the inner kind text
can be 72 words, and the part's line can then be 83; the longest
refusals on the tree today (`BlendError::UnsupportedChain` on its
longest raise-site detail, 75 words) reach that. A part inside a part
adds the wrapper again.

## The standard

The standard is stated once, in
`work/chrome/error-and-check-text-overflows-its-region.md` (section
"The standard a refusal is rewritten to").

## What would close it

This is the one case the concision pass found where a forwarded
refusal cannot be held to the budget by rewriting prose at one site:
the inner sentence is already within budget, and the wrapper is what
carries which part and which node failed. Ev's ruling on the concision
half keeps a viewer-side summary as the fallback for exactly this
("if it is IMPOSSIBLE to include all IMPORTANT information within a
reasonable amount of space"), so the choice is the edit program's to
put to Ev: a shorter wrapper that still names the part (the document
id and pin prefix are 2 words of the eleven), or a summary of the
inner refusal for the nested case only.


## Put to Ev (2026-09-29) — after the designer pair

This went to a designer pair (`docs/DESIGN-FORK-PROTOCOL.md`, row 15 of
`docs/DESIGN-FORK-LOG.md`) and then to Ev on the `[ev]` PR from
`edit/ev-part-root-refusal`. Neither designer took either of the two
closings offered above, a shorter wrapper or a summary. The inner
refusal travelling as text is where the problem starts, so the remedy
changes the value.

**What the designers converged on.**
- **The value.** `PartFault::PartRootFailed { node, refusal: NodeRefusal }`:
  - the inner refusal is typed;
  - `message` and `cause` go;
  - a part inside a part is a nested typed chain that keeps every
    level's `doc_ref`.
- **The sentence.** The instance gets its own bounded sentence, which
  never quotes the inner one and gives this document's recourse
  ("open the part and repair node N").
- **The part's name.** The tree names the part by file name, and the hex
  `DocRef` leaves the drawn text.
- **No summary.**
- **The class.** The same rule applies to
  `MateFault::PlacerRefused` and `LeverRefusal::PartUnresolved`, which
  quote another node's whole refusal the same way.
- **The F6 forwarding comment** over `NodeErrorKind`'s `Display` gains
  one exception: a carried refusal is drawn as its own line.

**The split Ev rules on:** how the tree draws a nested part failure.
- **(a) One line per level.** Each line names that level's part's failed
  node.
- **(b) Two lines at any depth.** The instance sentence names the
  deepest failing node and its depth, and the deepest refusal is drawn
  under it.
- **(c) Point only.** The reason is read by opening the part.

Seams the unit will carry:
- **The viewer has no door for `DocEdit::UpdateReference`.** So "accept
  the updated version" is not a recourse the GUI can offer; the
  `PinMismatch` refusal states it when it applies.
- **The refusal shape check has two gaps.** Its arena-key detector misses
  hex document ids, and it never flags a refusal with no recourse
  (`NoResolver`, `DepthExceeded`, `ReferenceCycle`, `PartProduct`).
- **`product_fault`'s placeholder** "records no cause" becomes a typed
  kernel-bug arm.

## RULED (2026-09-29, Ev on `[ev]` #3444) — (a), the full traceback

"i think go for the full traceback in (a); if that turns out annoying we
can change it later." The tree draws a nested part failure as one line
per document level under the instance row.
- Each level's line is that level's bounded instance sentence, naming the
  failed node in that level's part and giving its recourse.
- The last line is the failing node's own refusal, drawn exactly as its
  own tree draws it.

Everything under "What the designers converged on" stands. If the block
proves too long in use, the value supports drawing fewer lines without a
kernel change.

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — branch `edit/part-root-carried-refusal`

**Tier: single review** (one Opus FULL). The reason: the unit changes a
payload type the kernel, the mate vocabulary, the viewer and the binding
all read, so correctness is at real risk, but it is one idea applied
at three sites. This section is the spec. Treat every premise below as
a hypothesis: verify it against the tree and report any correction.

**The kernel value** (`eval/parts.rs`, `eval/mod.rs`):
1. **The fault.** It becomes `PartFault::PartRootFailed { node: RecipeNodeId, refusal: NodeRefusal }`.
   - `message` and `cause` go.
   - A part inside a part is `refusal.kind() == NodeErrorKind::Part { doc_ref, fault: PartRootFailed { .. } }`, so every level keeps its `doc_ref`.
   - `product_fault`'s "records no cause" placeholder becomes a typed kernel-bug arm, or a restructure that cannot reach it.
   - `PartFault` drops `Eq` unless `NodeRefusal` honestly gains it.
2. **Its `Display`.**
   - It is the instance's own bounded sentence. It never renders the inner refusal and never prints the hex `DocRef`; the payload and `Debug` keep both.
   - It names the part's failed node and gives this document's recourse (open the part and repair node N).
   - It adds no "accept the updated version": the viewer has no door for `DocEdit::UpdateReference`, and `PinMismatch` says it when it applies.
3. **The same rule for the class.**
   - `MateFault::PlacerRefused` names the placer node and points to it. Its row already draws the refusal, and the tree draws "see feature N".
   - `LeverRefusal::PartUnresolved` does not re-render `PartFault`'s carried refusal.
   - The F6 forwarding comment over `NodeErrorKind`'s `Display` gains the one exception, in one place: a carried refusal is another node's, with its own recourse, and is drawn as its own line, never inside this one.

**The viewer** (`tree.rs`, `pane/features.rs`, announced crossing):
4. **The carried lines.** `RowStatus::Failed` carries the carried refusals, one per document level. `failure_lines` draws them under the row's own line, indented, each exactly as that node's own tree would draw it (Ev's (a), the full traceback).
5. **File names.** The instance row names its part by file name, read from the session's directory store (id → path). Each carried line's subject names its level's file. Where no file is known, say so plainly and never print hex.

**Python** (announced crossing):
6. **The binding.**
   - `str(exc)` is the bounded sentence.
   - The carried refusal is exposed typed as `__cause__`.
   - The `part_root_failed` tag does not change.
   - The census, the tag inventory and the stubs move with it.

**Rows.** Each is red on `origin/main`, then green.
- The concision chain's `Part/PartRootFailed` rows are re-baselined to the bounded sentence.
- A depth-3 nested row, and a row whose inner refusal is the longest on the roster, are each within the budget at every drawn line.
- `PlacerRefused` on the longest inner refusal is within the budget.
- A viewer test builds a depth-2 workspace, takes the tree rows, and holds every drawn line to `test_utils::refusal::problems`, including each level's file name.
- Python: `str` is bounded and `__cause__` is the typed refusal.
- `test_utils::refusal::arena_key` also flags a hex document id. It is red on today's text.

**File as rows** (not this unit):
- The shape check never flags a refusal with no recourse (`NoResolver`, `DepthExceeded`, `ReferenceCycle`, `PartProduct`).
- The viewer has no door for `DocEdit::UpdateReference`.
- The kernel says "node N" where the tree says "feature N".
