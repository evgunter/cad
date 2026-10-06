---
id: slot-id-profile-doc-says-set-program-rebinds-names
kind: issue
title: Six kernel sites still say SetProgram rebinds the names it moves; since #3193 it rewrites none
status: closed
opened: 2026-09-29
priority: P4
cost: E
branch: recipe/set-program-undrawn-names
pr: 3879
closed: 2026-10-02
---

## The finding

Since the step-id ruling (`crates/profile/README.md` V2, "ruled by Ev
... on #3193, 2026-09-25"), `DocEdit::SetProgram` rewrites no name. A
kept step's names keep their spelling, and a dropped step's names are
reported (`DocEdit::SetProgram`'s doc, "The names": "nothing is
rewritten"). There is no `Maintenance::Rebound` arm left. These sites
still carry the older premise, that a reshaping rebinds or moves the
names on it:

1. `crates/editor-core/src/node.rs`, `SlotId::Profile`'s doc (near line
   439): program structure changes only by `SetProgram`, "which reports
   every name its reshaping strands and rebinds every name it moves".
2. `crates/editor-core/src/node.rs`, `SlotId::dimension`'s `Profile`
   arm comment (near line 612): `SetProgram` "rebinds every kept name
   and retires the rest".
3. `crates/editor-core/src/edit.rs`, `DocEdit::moves_the_mate_graph`'s
   `SetProgram` arm (near line 545): "A reshaped program rebinds every
   name on its kept steps in place, a mate's head among them — the same
   motion as `Rebind`". **Stale prose, not a defect**: the arm answers
   `true`, but a cluster comes from `read_mates` → `member::walk`,
   which never reads step ids, so a kept name moves no reading edge.
   The `true` has lost its reason and costs one wasted recipe pass per
   reshape in a mated document. It could answer `false`; that is a
   code change, and EDIT's to decide.
4. `crates/editor-core/src/edit.rs`, `DocEdit::writes_a_mates_datum`'s
   `SetProgram` arm comment (near line 505): "A reshaping rebinds or
   retires the NAMES a mate's heads hold — `Rebind`'s motion over every
   name at once". The arm's answer, `false`, is right; only the reason
   is stale.
5. `crates/editor-core/src/edit.rs`, `MaintenanceNet`'s doc (near line
   2874) gives "a program written as several one-slot writes" as an
   example of a several-edit action. Since #3446 no caller writes a
   program that way: the viewer's profile edit is one `SetProgram`. A
   live example is the viewer's parameter write, where the notation and
   the value land as one action (`DocSession`'s `commit_action` call
   in the parameter door).
6. `crates/editor-core/src/resolve/mod.rs`, `apply_with_names`'s
   `SetProgram` arm comment (near line 2565): "A program and its
   provenance carry no name; the names a reshaping moves are the
   document's own, rewritten at the door." `SetProgram` carries `ids`,
   not a provenance, and the door rewrites no name. The arm's answer
   (no name to check) is right.

The same premise was in AUTHOR's
`the-viewer-keeps-its-profile-lock-and-order-search-after-set-program`
row and in `docs/AUTH-6-SPEC.md`, which asked for a
`LoopProvenance::identity` and a `Rebound { from, to }` that the tree
no longer has. AUTH-6 found it there.

Swept with `rg -n 'rebinds|rebind every|kept name|one-slot writes'`
over `crates/editor-core/src`, `crates/profile/src` and the two crates'
READMEs. That pattern cannot match a paraphrase that says "moves" or
"follows" without "rebind"; a second pass,
`rg -i '(reshap|SetProgram).*(move|follow|re-?aim)'` over the same
paths, found site 6. Neither pattern matches a sentence that wraps
between the edit's name and the verb.

## The fix

Sites 1, 2, 4, 5 and 6 are re-wordings that follow an approved change,
not design changes: "keeps every kept step's names and reports every
name on a step it drops". Site 3 is either the same re-wording with a
new reason for `true`, or `false`.

## Built (2026-10-02, PR 3879)

Sites 1, 2, 4, 5 and 6 are re-worded. Site 3 no longer exists: the
gauges change (1441b5154) deleted `moves_the_mate_graph` whole, so
there is no arm to answer. The sweep found one sibling,
`persist/check.rs`, which said a `SetProgram`'s "provenance is
integers"; it now says step ids.
