---
id: a-split-values-name-table-names-faces-its-output-body-does-not-hold
kind: issue
title: On seat8's split values, 16 of 1,230 name-table entries name a face that output_body(r.body) does not hold
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [7501]
---

Filed by the CIW orchestrator, 2026-11-23, from PR 7501 (CIW's digest
lane). The lane saw this and did not examine it.

**Observed.** PR 7501 rewrites the seat8 lowering digest so it reads
every cell by geometry rather than by arena key (`KeyFree`). That reader
has to resolve each name-table entry's face in `output_body(r.body)`. On
seat8's documents, on split values, 16 of the 1,230 entries name a face
that body does not hold. The new digest records each one as "a face the
body does not hold". The old keyed digest hashed the dangling key
without noticing.

**Why it may matter.** A name that resolves to no face of its body is a
reference that will refuse, or resolve to something else, when a later
feature reads it. It may be a defect in how a split's name table is
mapped onto its two halves. It is not yet shown to be one, and it may
instead be a reader artefact of the new digest.

**First step.** List the 16 entries from PR 7501's reader on its branch
(`ciw/census-digest-goldens`). For each, say which half of the split it
should name and why it does not. Then decide whether this is a defect,
and P1 if a reference can resolve wrongly.

What would close it: every name-table entry of a split value names a
face its output body holds, or the exceptions are explained and pinned.
