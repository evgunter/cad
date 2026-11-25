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
