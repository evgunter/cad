# MSOLVE-13-SPEC.md

MSOLVE-13, a mate reads its face where it says: operand-first resolution, `MovedAbove`, union descent, placement-keyed members (#3969)

Deleted at the unit's merge, 2026-10-03.
Recover with `git show 9d63f664299f8c9f3212d1cb7f7326ed2573fe03:docs/MSOLVE-13-SPEC.md`.
`work/msolve/log.md` records where the build departed from the spec:
- Fillet, chamfer and shell were classified as carried (`FromTarget`).
- Merge and fragment are read as "absent from the consumer's table"
  rather than through `look_through_fold`.
- The review's fix pass worded the recourse "re-pick the face on `by`"
  and added `MovedAbove { copies }`.
