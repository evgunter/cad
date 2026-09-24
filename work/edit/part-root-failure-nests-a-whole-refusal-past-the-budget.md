---
id: part-root-failure-nests-a-whole-refusal-past-the-budget
kind: issue
title: editor-core: a part's root failure renders the part's own node refusal inside an 11-word wrapper, so it can outgrow the viewer's 75-word budget by construction
status: open
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

