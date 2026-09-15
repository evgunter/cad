---
id: dead-work-citations-from-shipped-code-and-docs
kind: issue
title: Five shipped files cite work/ items that no longer exist
status: open
opened: 2026-09-13
---


## Finding

Shipped source and design pages cite `work/` items by path. A program is
closed by deleting its tracker directory (`work/README.md`; the walk
stays recoverable at the SHA `docs/DOC-LEDGER.md` names), so every such
citation becomes a dead path the day its owning program closes — and
nothing in the tree checks them. A reader who follows one finds nothing
and cannot tell "this reading was superseded" from "this reading is
still live, somewhere".

Found while sweeping for stale citations in BLEND-12's fix pass (R2
NOTE-3). Five instances, each a path that does not resolve at this SHA:

| citing file | cited path | still there? |
| --- | --- | --- |
| `crates/profile/README.md` (the fillet section's "what stays") | `work/issues/nocornersidecandidate-has-no-producer.md` | no |
| `crates/editor-core/src/clearance.rs` (the `Interval` basis note) | `work/issues/interval-orthonormal-basis-sign-hull.md` | no |
| `crates/pncad-py/src/prose_census.rs` (the step-import row's reason) | `work/issues/debug-in-prose-at-blend-and-step-import.md` | no |
| `crates/topo/src/offset_nappe.rs` (module docs) | `work/issues/cone-nappe-is-decided-in-five-places.md` | no |
| `crates/viewer/src/props.rs` (module docs) | `work/issues/doc-param-unit-edit-has-no-door.md` | no |

The `profile` one is repaired in PR 2508 (BLEND-12), which is where the
class was found — that file was in the unit's fence. The other four are
listed here rather than fixed, because each belongs to a different
crate's owner and none of them is a BLEND path.

**`work/<program>/…` citations are the same class with a shorter fuse**:
they die the moment that program closes rather than when an issue is
resolved, and the tree holds several (`work/curved/…` and `work/fix/…`
citations in `clearance.rs` and `prose_census.rs` resolve today and will
not once those programs close).

## What would actually fix it

Two shapes, and the choice is the point of this item:

1. **A gate.** A row that walks `crates/**/*.rs` and `crates/*/README.md`
   for `work/…\.md` citations and asserts each resolves. Cheap, and it
   inverts the failure: a program that closes discovers, on its own PR,
   which shipped prose was leaning on it. The cost is that closing a
   program then obliges editing other crates' files, which is exactly
   the coupling the tracker's directory-is-the-claim rule avoids.
2. **A convention.** Shipped prose cites the READING, not the path — it
   states the finding in a sentence and, where provenance matters, names
   the SHA rather than a file that moves. `docs/DOC-LEDGER.md` is
   already the done-state of record for exactly this reason.

(2) is the smaller change and the one the ledger's existence argues for;
(1) is the only one that catches the instances already in the tree. They
compose: adopt (2), and gate (1) against new citations only.

No obvious single owner — the five instances span four crates — so this
sits in `work/issues/` per `work/README.md`'s last-resort rule.

## A sixth instance, and a correction to the fifth (2026-09-15, CHROME's `chrome/citation-repoint`)

**Sixth instance.** `crates/viewer/src/pane/properties.rs` — the
comment on the parameter row's unit LABEL, explaining why there is no
picker beside it — cites
`work/issues/doc-param-unit-edit-has-no-door.md`. Same dead path as the
`props.rs` row in the table above, same cited item, a different file.
The two were presumably split apart by
`viewer-session-god-module-split` (#1830) with the citation copied
along.

**Correction to the fifth row.** The table reads *"still there? no"*
for `work/issues/doc-param-unit-edit-has-no-door.md`, which is true of
the PATH and misleading about the item: it was **claimed by EDIT**, not
resolved and not deleted. It is open at
`work/edit/doc-param-unit-edit-has-no-door.md`. That makes these two
instances a different sub-case from the other four, and a cheaper one:
the other four cite items that died with their programs and need shape
(2)'s rewrite-the-reading treatment, while these two need a one-word
path edit — or nothing, if shape (2) is adopted and the sentence is
rewritten to state the reading anyway.

It also sharpens the argument for a gate: this class has a **second**
fuse nobody has named, the `work/README.md` rule that *claiming an
issue MOVES the file*. A citation can rot without any program closing
and without the finding going anywhere, just because its owner was
identified.

Both tracker-side citations of the same item — in
`work/chrome/parameter-row-field-has-no-text-door.md` and
`work/chrome/add-parameter-form-authors-canonical-only.md` — were
re-pointed to `work/edit/…` by that pass. The two source files are
outside its fence and are recorded here instead.
