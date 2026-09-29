---
id: part-root-failure-nests-a-whole-refusal-past-the-budget
kind: issue
title: editor-core: a part's root failure renders the part's own node refusal inside an 11-word wrapper, so it can outgrow the viewer's 75-word budget by construction
status: open
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
