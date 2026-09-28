---
id: CONTACT-8
kind: unit
title: the merge deletes a seam edge left dangling inside a merged face at any angle, and a boolean refuses a planar declared group it cannot glue (PR 3350, ratified)
status: closed
opened: 2026-09-28
priority: P0
cost: M
branch: contact/8-dangling-seam
closed: 2026-09-28
---


Carries `area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`.
Spec: `docs/CONTACT-8-SPEC.md`. The design was ratified by Ev on PR
3350 (`docs/DESIGN.md`, "Maximal-faces precondition and the merge
stage").

Review tier: **single full.** The class at risk is record carriage: a
contact record citing a deleted vertex must drop as consumed. The
merge's region must also be unchanged by every deletion.

## Closed

`merge_coplanar_faces` prunes by topology.
- Every doubled edge with a free end goes with that end (`kev`), at any
  angle and repeatedly, before the ring step.
- An isolated doubled edge goes with both ends, and its empty ring goes
  too (`mekr_chord`, then `kev`).
- Every deleted vertex is recorded in `killed_vertices`.
- The collinearity licence and `redundant_subdivision_vertex` are gone.
  The history check found that the only reason for them was the
  `merge_skip` pin (#1131).

Every planar group takes the refusing regime; only curved groups record
a skip. No reachable planar shape still refuses: the plugged through
hole, which shipped illegal at base, now merges. The reviewer's 22
unions (8 at an oblique pose) glue with the volume unchanged; 10 of
them shipped at base as operands the next boolean refused.

At the document layer, a declared union can now glue faces of two
assembly members into one merged face. The naming emitter's chord
read-through then finds several same-side constituents, and refuses as
the typed missing rule `NamingError::MergedChordConstituents` rather
than an emission bug. The rule itself is filed on WIRE's slate. Every
order of the `docm8_flat_merged` fixtures that fuses is tier-3 green at
the analytic volume.

Review:
- A single full review found no MAJOR and three MINORs. The fix pass
  merges the plugged hole, pins S3 on a real merge's `Descendants`, and
  corrects stale prose.
- The orchestrator's landing check caught four red `editor-core` rows
  the lane's subset runs missed. The second fix pass root-caused and
  re-signed them. The orchestrator read both passes.

Filed:
- `a-curved-merge-group-with-a-dangling-seam-refuses-as-period-closure`
  (P3);
- `work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule`
  (by the lane).

Curved groups are left unpruned: the ratified clause requires only that
their skip is recorded.
