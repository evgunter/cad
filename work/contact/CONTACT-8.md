---
id: CONTACT-8
kind: unit
title: the merge deletes a seam edge left dangling inside a merged face at any angle, and a boolean refuses a planar declared group it cannot glue (PR 3350, ratified)
status: dispatched
opened: 2026-09-28
priority: P0
cost: M
branch: contact/8-dangling-seam
---


Carries `area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`.
Spec: `docs/CONTACT-8-SPEC.md`. The design was ratified by Ev on PR
3350 (`docs/DESIGN.md`, "Maximal-faces precondition and the merge
stage").

Review tier: **single full.** The class at risk is record carriage: a
contact record citing a deleted vertex must drop as consumed. The
merge's region must also be unchanged by every deletion.
