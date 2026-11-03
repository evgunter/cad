---
id: creating-a-thing-and-editing-it-are-two-implementations
kind: issue
title: viewer: every node kind with a create form and an edit panel has two implementations; create should commit and hand off to the edit panel
status: closed
opened: 2026-11-01
priority: P1
cost: H
refs: [the-profile-create-form-is-a-second-profile-editor, the-extrude-and-revolve-forms-take-no-variables, a-variable-is-edited-in-a-separate-widget-not-on-its-own-row]
closed: 2026-11-03
pr: 6431
---
Reported by Ev (chat, 2026-11-01): "if there are any other things that
have separate creating and editing widgets, those should also be
removed in favor of either (a) creating and then editing the thing or
(b) creating the thing with certain values but it uses the exact same
code as the editing code there. and the choice between (a) and (b)
should be the same everywhere. but (a) seems nice and simple and it
has the desirable property that edits to a profile while you're
originally making it still count as edits so you can undo them."
Filed P1 at Ev's request (two implementations of one logic).

## The rule

One rule for the whole viewer, with (a) preferred: a create gesture
commits the node with default values, selects it, and leaves the rest
to the node's edit surface (its Properties slot rows, or the profile
editor). Every later change is an edit in the history, so undo reaches
changes made while the thing was being created.

## Where the pairs are

Create forms and seated tools in `crates/viewer/src/pane/create.rs`,
each with its own drafts (`crates/viewer/src/drafts.rs`) and fields,
next to the Properties pane's slot rows for the node they create
(`crates/viewer/src/pane/properties.rs:671` `slot_group_ui`,
`:750` `slot_value_ui`):

- add profile (`create.rs:749`), filed on its own as
  `the-profile-create-form-is-a-second-profile-editor`;
- extrude (`create.rs:1083`) and revolve (`create.rs:1133`), whose
  literal-only fields are filed as
  `the-extrude-and-revolve-forms-take-no-variables`;
- boolean, split, mirror, transform, pattern, projection, duplicate,
  blend (`create.rs:1212-1652`);
- the mate tool (`create.rs:546`) against the placement's constraint
  rows. PLACING's `mate-tool-commits-one-mate` holds a design question
  on the same ground. Reconcile with it, don't pre-empt it. A
  specific instance (Ev, 2026-11-01): the placement editor in PR
  #6300 (`placing/bundle-editor`, `crates/viewer/src/matetool.rs`)
  has a Create door that inserts a placement together with its first
  mates in one Apply (`SessionOp::Place` carrying the whole bundle),
  beside an Edit door that writes an existing placement's bundle
  (`SessionOp::SetBundle`). Under (a), Create commits a placement
  with no mates where the body stands and opens the Edit door on it,
  and every mate is added by the Edit door;
- add variable (`properties.rs:240`) against the variable's own
  editor, filed as
  `a-variable-is-edited-in-a-separate-widget-not-on-its-own-row`.

## Open points

- Seated tools whose input is a *pick* (boolean operands, blend edges,
  mate faces) need the pick before a node can exist. Under (a) the
  gesture still collects picks, then commits with default numbers. Say
  which inputs count as "what the node is" (asked at create) and which
  are "its values" (edited after).
- A node committed with defaults may refuse to evaluate. Its row must
  read as a node awaiting values, not as a failure.
- This touches several programs' files (FORMS, DOORS, HELD, PLACING,
  VSEAM). Probably a designer pass first, then cut into per-tool
  units.

## Closed

2026-11-03, FORMS orchestrator. Every pair on FORMS' ground in the
census is now one implementation, under the rule Ev ratified in
`crates/viewer/README.md`, "A create gesture commits the node". The
landings:
- profile: #6403;
- extrude and revolve: #6401;
- blend, mirror, clearance and the pattern count: #6414;
- split and pattern: #6431.

#6388 and #6393 were the plan's earlier steps.

The sweep in #6431 found two fields still on the create forms, and
both stay:
- the projection's `instance` (`pane/create.rs`
  `part_selector_rows`);
- the traced path's "to crossing" (`TraceGesture::crossing.nth`).

Neither is a value; each says *which* thing the gesture reads. The
ratified rule asks a gesture for "the kind and the reads", so they are
reads.

The census's other pairs are owned elsewhere:
- PLACING's mate tool and Create door: `mate-tool-commits-one-mate`,
  closed;
- CHROME's variable editor:
  `a-variable-is-edited-in-a-separate-widget-not-on-its-own-row`,
  still open on CHROME's slate.

The follow-ups FORMS filed are on its slate:
- `a-clearance-bound-shape-is-a-row`;
- `a-plane-other-than-a-face-cannot-be-authored-in-the-viewer`;
- `no-viewer-row-re-points-a-live-nodes-read`;
- `an-anonymous-index-cannot-be-written-into-a-value-row`.
