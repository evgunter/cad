---
id: part-unresolved-refusal-draws-the-workspaces-stage-prefix
kind: issue
title: editor-core: a part whose reference does not resolve through a Workspace draws the store's own 'workspace:' stage prefix inside PartFault::Unresolved, which the concision roster's made-up messages hide
status: closed
closed: 2026-09-29
pr: 3492
rides_with: edit-refusals-short-of-the-shape-guard
branch: edit/part-refusal-recourse
opened: 2026-09-29
priority: P3
cost: E
---

## What

`PartFault::Unresolved { fault, message }`
(`crates/editor-core/src/eval/parts.rs`, `impl Display for PartFault`)
renders the resolver's `message` inside its own sentence ("the reference
did not resolve: {message}", "the reference's pin does not hold:
{message}"). The shipped resolver is `pncad`'s `Workspace`
(`crates/pncad/src/workspace.rs`, `impl PartResolver for Workspace`),
whose `message` is `WorkspaceError`'s `Display`, and every arm of that
opens with the stage prefix `workspace:` ("workspace: no document with
id …", "workspace: pin mismatch for document … at `…`: …"). So the
feature tree draws, for an instance whose part the store does not hold:

    node N failed: instantiating the part: the reference did not resolve: workspace: no document with id …

`test_utils::refusal::problems` flags `workspace:` as a stage prefix.
The concision roster does not see it: its `Part/Unresolved(*)` rows
(`crates/editor-core/tests/refusal_concision_chains.rs`,
`document_arms`) build the fault with made-up messages ("no document
with this id is registered"), the blind spot
`part-product-refusal-draws-the-gathers-stage-prefix` closed for
`PartProduct`. The three rows are also admitted as stating no recourse
(`FILED_NO_RECOURSE`, under `edit-refusals-short-of-the-shape-guard`).

Found by `edit/part-product-refusals`' sweep for a carrier that draws a
wrapped refusal's stage word under its own.

## What would close it

The roster rows built from a real `Workspace`'s refusal (red), then the
store's sentence carried without its stage word where the part names
the stage itself — `PartProduct`'s shape (`ProductError::sentence`) is
the precedent. `ResolveFailure` and `WorkspaceError` are `pncad`'s
(LIB's), so the store's half is announced to LIB.

## Ruled (2026-09-29, EDIT orchestrator) — rides with `edit-refusals-short-of-the-shape-guard`

One unit with the feature-tree half; the spec is in that row.

## Built (2026-09-29, PR 3492)

`WorkspaceError`, `PersistError` and `ProductError` render their sentence without the stage word through one shared helper, `editor_core::sentence::Staged` (fix pass). Both shipped resolvers carry the sentence: `Workspace`'s, and the viewer's `DirResolver`. The rows are real text through `DirResolver` (`crates/viewer/tests/instance_authoring.rs`). The details are in the carrier's `## Built`.
