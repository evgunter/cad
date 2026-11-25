---
id: a-two-strut-cone-apex-reports-an-internal-message-a-file-reaches
kind: issue
title: a cone face reaching its apex by two struts reports an `internal:` message, which a file can reach
status: open
opened: 2026-11-25
---


Found by the dual review of `lathe/import-wrap-tip` (PR 7774), both
reviewers' N2. It predates that branch, which does not change the path
(the reviewers' reading). Not reproduced in that lane: the witness file
is not committed.

## What

A cone face that walks two struts out and back to its apex (two seam
generators ending at a valence-2 apex vertex, the face on both sides of
each) imports to a refusal whose text begins `internal:`. By this
crate's convention that prefix marks a broken invariant, a state no
file reaches, yet a file reaches it. A file that states two wrap tips
at one apex is one the kernel does not adopt (DESIGN `[topology]`'s
seam clause: one wrap edge, and only one), so the right outcome is a
typed refusal naming that.

## What would close it

The scan (`normalize/singularity.rs`, `scan`) or the face gate names a
singular vertex reached by more than one edge the face walks out and
back, and refuses it typed, without `internal:`. A row pins it on the
reviewers' two-strut file, committed beside the rim-only caps.

- 2026-11-25 — witness committed: `crates/step-import/tests/fixtures/wrap-tip-review/cone_with_two_struts_to_its_apex.step`
  (apex at the origin, axis +z, half-angle 0.5, rim at z = 10 split at
  azimuths 0 and π, each rim vertex joined to the apex by a generator the
  face walks out and back). Both the one-band stack and `lathe/import-wrap-tip`
  refuse it with "entity #133: internal: a built vertex with no built fan
  anchor", at the build (`build.rs`), before the chart-singularity pass
  reads it; azimuths {0, 0.3} give the same. Pinned red at every ε by
  `crates/step-import/tests/wrap_tip_review_witnesses.rs`,
  `a_cone_with_two_struts_to_its_apex_refuses_typed`.
