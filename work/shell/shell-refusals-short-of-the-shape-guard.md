---
id: shell-refusals-short-of-the-shape-guard
kind: issue
title: shell: refusals the viewer draws that state no recourse, by the shape guard's census
status: open
opened: 2026-09-29
priority: P2
cost: M
---

(CHROME `refusal-residue`, from the shape guard's zero-recourse check.)

## What

`test_utils::refusal::problems` now flags a refusal that states no
recourse: no `Recourse:`, no "There is no way through" in either case,
and none of the shared unlabelled repairs (`BARE_RECOURSES`). The
standard (`work/chrome/error-and-check-text-overflows-its-region.md`,
"The standard a refusal is rewritten to") says the recourse is the
part never to drop, and where there is no way through the sentence
says so.

These rows, raised through `ShellError` (`crates/topo/src/shell.rs`) and the replace-face refusals it forwards, render with none. Each is admitted
by exact id, under the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  12 feature-tree rows.

Families: `Shell`.

## Repair shape

Rewrite each arm at its source to the standard: add the recourse the
raise site supports, or say "There is no way through" where none
exists (`geom_core::KERNEL_DEFECT_ENDING` and its siblings for a
kernel defect). Read the raise sites first: a recourse is a claim.
Then drop the row's entry. The lists carry a must-fire check
(`every_admission_admits_a_row_it_is_needed_for`, and the same check
in the edit and at-rest suites), so an entry left behind after the fix
goes red.

## A stage for a subject (CHROME fix pass, PR 3457)

The shape guard now reads a clause whose subject is a stage — a gerund
with a wrapper verb, `<doing something> refused:` — as a label. These
are admitted by exact row and label in
`refusal_concision_chains.rs` `FILED`:

- `Shell/Face` and the `Shell/Face/Fit/` family (`FILED_NAMESPACES`):
  "offsetting a face inward refused:" (`topo/src/shell.rs`).
- `Shell/Lift`: "lifting the rim back onto a designated open face
  refused:".
- `Shell/Insert`: "inserting the cavity refused:".

## Escalations that offer a declaration the door cannot take (CHROME triage)

(From the triage in `work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.) The shell op takes no declaration (`Node::Shell`,
`crates/editor-core/src/node.rs` near :2007, has no `declare`). Yet
these arms render the whole `Indeterminate`, which ends in
`COINCIDENCE_RECOURSE` ("declare the coincidence, …"):

- `ShellError::Escalated` (`crates/topo/src/shell.rs` near :520,
  `Display` near :629): "a classification of the shell is too close to
  call: {source}". It is raised from `RingOuterVerdict::Escalated`
  (near :1594) among others, so its subject is generic too.
- `ReplaceFaceError::Escalated` (`crates/topo/src/replace_face.rs`
  near :525, `Display` near :708): "replace_face_offset escalated:
  {source}", a function name for a subject. It carries
  `TransportError::Escalated` (near :1806), the spiric constructor's
  escalation (`offset_axial.rs` near :2113) and the section router's
  (near :1926).
- `OffsetError::Escalated` (`crates/geom-brep/src/offset.rs` near
  :273, `Display` near :306): "offset_surface escalated: {source}".
  It reaches the shell through `ReplaceFaceError::Offset` (near :566).
  OFFSET claims this file too.

`IsoRowError::Escalated` reaches the same door through
`ReplaceFaceError::IsoRow`, and is filed on TRIM
(`work/trim/trim-escalations-offer-a-declaration-the-door-cannot-take.md`).
The repair is `payload()`, a subject in plain words and a routed
recourse, as in `sweep::blend::BlendError::Escalated`'s `Display`.
