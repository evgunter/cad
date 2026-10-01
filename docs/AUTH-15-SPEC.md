# AUTH-15 — accept a part's updated version from the viewer

**Row**: `work/author/viewer-has-no-door-to-accept-a-parts-updated-version`
(P3). Read it in full. The EDIT row it cites
(`part-root-failure-nests-a-whole-refusal-past-the-budget`) is closed.

**Branch** `author/accept-part-version`. **Never merge; I merge.**

## What a person gets

Repairing a part moves its content pin, so every instance of it then
refuses with a pin mismatch. The refusal's recourse
(`pncad::workspace::PIN_MISMATCH_RECOURSE`) says to accept the updated
version: `DocEdit::UpdateReference`, or `workspace::update_to_store`
for every site at once. Nothing in the viewer emits either, so a GUI
author is told to do something the GUI cannot do.

After this unit, an instance refusing on a pin mismatch offers
**accept the updated version**. Accepting records the edit against the
store's current pin, as one action and one undo.

## Check these first

- **How a pin mismatch reaches the viewer.** Trace `ResolveFault::PinMismatch`
  / `WorkspaceError::PinMismatch` from evaluation to the instance's
  tree row, and say what the row shows today. The viewer resolves
  through `docio::DirResolver`, and `docio` can open a `Workspace`
  (`DirResolver::workspace`). `parts.rs` already lists a workspace for
  `Add part…`.
- **Which kernel door the viewer should call.** Use
  `workspace::update_to_store(doc, id, &workspace, tol)`, the
  elaboration that yields one `UpdateReference` per site and refuses
  typed (`WorkspaceError::Update`). Don't build a `DocEdit::UpdateReference`
  by hand from a pin the viewer reads itself. If a per-instance accept
  is also warranted, say why and use the kernel's door for it too.
- **Where the offer lives.** AUTH-9 lesson: the viewer has one home
  for refuse-then-offer (`session::refuse` wording, the `frame` reader
  beside `creation_offer` and `declare_offer`), and OFFER's P1 row
  `work/offer/a-refusal-offers-no-action-in-the-viewer` asks for
  exactly this shape. Put the offer there. Don't mint a fourth home.
  If an offer on a tree row needs a control no row draws today, say so
  and stop.

## Design calls: decide and say

1. **Every site of that part, or the one instance.** The recourse names
   both.
2. **What the author sees before accepting:** which part, and old
   versus new. Use the chrome's existing spellings; don't render pins
   in a new format.
3. **What happens when the store has no newer version,** or the part
   file is gone. The kernel's typed refusal is said as-is.

## Traps

- Fourteen of fourteen AUTHOR units have minted a fresh duplication
  while closing one. Here the likely ones are a second offer mechanism
  (see above), a copied recourse sentence, and a second workspace open
  beside `parts.rs`'s.
- The recourse text itself (`PIN_MISMATCH_RECOURSE`) names a door. Once
  the door exists, check that the sentence is still true of the GUI.
- Drive the real session and pane. A pin mismatch needs a part file
  whose content changes: `crates/viewer/tests/assembly_display.rs` and
  `parts.rs`'s tests build workspaces on disk. Reuse their fixture; don't
  write a third.

## Scope, verification, deliverable

In: `session.rs`, `session/op.rs`, `session/refuse.rs`, `frame.rs`,
`parts.rs`, `docio.rs`, and the pane that draws the instance row
(`tree.rs` / `pane/features.rs`). Out: `crates/editor-core` and
`crates/pncad`. If the kernel door is missing something, file it on
EDIT.

- Rows that go red:
  - a pin-mismatched instance offers the accept;
  - accepting records `update_to_store`'s edits as one action, and the
    instance then evaluates;
  - one undo restores the mismatch;
  - with no newer version, the typed refusal is said;
  - a non-mismatched instance offers nothing.

  Name the mutations, restore each from a byte copy, and **touch**
  afterwards.
- Local:
  - fmt;
  - clippy, both feature sets, `-D warnings`;
  - `doc-gate.sh`;
  - every `scripts/gates/*.sh`;
  - `--lib` and `--test all` as separate runs;
  - `work.py lint`;
  - the `work.py territory` output in the PR.

  If you read evaluation results, run
  `cargo test -p editor-core --test all node_standing::`.
- `CARGO_TARGET_DIR=/root/auth-15-target` on every invocation,
  including excluded roots. Scratch in `/root/auth-15-scratch/`. Wrap
  `cargo` in `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-15: accept a part's updated version from the viewer`.
Report to me; don't merge.
